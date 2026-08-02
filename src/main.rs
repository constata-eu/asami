use std::{
    io::{self, BufRead, IsTerminal},
    time::Duration,
};

use asami::{api, chain, config::Settings, db, notifications, processor, x, App};
use clap::{Parser, Subcommand};
use sqlx::Row;
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[command(name = "asami", about = "Asami club indexer, payout worker, and web server")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    #[command(alias = "serve")]
    Api,
    Worker {
        #[arg(long)]
        once: bool,
    },
    Migrate,
    Logs {
        #[arg(long, default_value_t = 100)]
        limit: i64,
    },
    Config,
    XAuthUrl {
        #[arg(long)]
        account: String,
    },
    XAuthComplete {
        #[arg(long)]
        account: String,
        #[arg(long)]
        redirected_url: String,
    },
    Demo,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "asami=info".into()))
        .init();
    let cli = Cli::parse();
    let mut settings = Settings::from_env()?;
    if matches!(&cli.command, Command::Worker { .. }) && settings.automator_mnemonic.is_some() {
        let passphrase = automator_passphrase()?;
        settings.unlock_automator(&passphrase)?;
    }
    let app = App::new(settings).await?;

    match cli.command {
        Command::Api => api::serve(app).await?,
        Command::Migrate => db::migrate(&app.db).await?,
        Command::Config => println!("{}", serde_json::to_string_pretty(&app.settings.safe_summary())?),
        Command::Logs { limit } => print_logs(&app, limit.clamp(1, 1000)).await?,
        Command::Worker { once } => worker(app, once).await?,
        Command::XAuthUrl { account } => {
            println!("{}", x::authorization_url(&app, &account).await?);
        }
        Command::XAuthComplete {
            account,
            redirected_url,
        } => {
            x::complete_authorization(&app, &account, &redirected_url).await?;
            println!("X account authorization stored.");
        }
        Command::Demo => {
            let status = tokio::process::Command::new("scripts/run-dev").status().await?;
            if !status.success() {
                anyhow::bail!("demo script failed");
            }
        }
    }
    Ok(())
}

fn automator_passphrase() -> anyhow::Result<String> {
    if io::stdin().is_terminal() {
        return Ok(rpassword::prompt_password("Automator wallet passphrase: ")?);
    }

    let mut stdin = io::stdin().lock();
    read_passphrase_line(&mut stdin)
}

fn read_passphrase_line(reader: &mut impl BufRead) -> anyhow::Result<String> {
    let mut passphrase = String::new();
    reader.read_line(&mut passphrase)?;
    while matches!(passphrase.as_bytes().last(), Some(b'\n' | b'\r')) {
        passphrase.pop();
    }
    Ok(passphrase)
}

async fn worker(app: App, once: bool) -> anyhow::Result<()> {
    loop {
        run_cycle(&app).await;
        if once {
            break;
        }
        tokio::time::sleep(Duration::from_millis(app.settings.worker_interval_ms)).await;
    }
    Ok(())
}

async fn run_cycle(app: &App) {
    if let Err(error) = chain::index_once(app).await {
        tracing::warn!("indexer cycle failed: {}", db::sanitize_log_message(&error.to_string()));
        db::log(
            &app.db,
            "fail",
            "worker",
            "indexer",
            "Indexer cycle failed; see process diagnostics.",
            Some(app.settings.chain_id),
            None,
        )
        .await;
        return;
    }
    for result in [
        processor::convert_transfers(app).await.map(|_| ()),
        processor::reconcile_submitted(app).await.map(|_| ()),
        processor::process_one_payout(app).await.map(|_| ()),
        notifications::queue_unconfigured_tokens(app).await.map(|_| ()),
        notifications::render_pending(app).await.map(|_| ()),
        x::post_one(app).await.map(|_| ()),
    ] {
        if let Err(error) = result {
            tracing::warn!("worker step failed: {}", db::sanitize_log_message(&error.to_string()));
            db::log(
                &app.db,
                "fail",
                "worker",
                "step_failed",
                "Worker step failed safely and will be retried.",
                Some(app.settings.chain_id),
                None,
            )
            .await;
            break;
        }
    }
}

async fn print_logs(app: &App, limit: i64) -> anyhow::Result<()> {
    let rows = sqlx::query(
        "SELECT id, created_at::text, severity::text, kind, subkind, message
         FROM audit_log_entries ORDER BY id DESC LIMIT $1",
    )
    .bind(limit)
    .fetch_all(&app.db)
    .await?;
    for row in rows {
        println!(
            "{}\t{}\t{}\t{}\t{}\t{}",
            row.get::<i32, _>(0),
            row.get::<String, _>(1),
            row.get::<String, _>(2),
            row.get::<String, _>(3),
            row.get::<String, _>(4),
            row.get::<String, _>(5),
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::read_passphrase_line;

    #[test]
    fn reads_automator_passphrase_from_stdin_without_line_ending() {
        let mut input = Cursor::new(b"correct horse battery staple\r\n");

        assert_eq!(
            read_passphrase_line(&mut input).unwrap(),
            "correct horse battery staple"
        );
    }

    #[test]
    fn preserves_spaces_and_accepts_an_empty_bip39_passphrase() {
        let mut spaced = Cursor::new(b"  passphrase with spaces  \n");
        let mut empty = Cursor::new(b"\n");

        assert_eq!(read_passphrase_line(&mut spaced).unwrap(), "  passphrase with spaces  ");
        assert_eq!(read_passphrase_line(&mut empty).unwrap(), "");
    }
}
