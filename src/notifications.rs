use anyhow::Context;
use regex::Regex;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use sqlx::{Postgres, Transaction};

use crate::App;

const EN_OPENINGS: [&str; 10] = [
    "Good morning, Asamians!",
    "Hello, Asamians!",
    "A bright note for the club:",
    "Community update:",
    "A little Rootstock joy:",
    "Good news, Asamians!",
    "A warm club update:",
    "Today brings good news:",
    "Rootstock friends,",
    "With gratitude from Asami:",
];
const EN_THANKS: [&str; 5] = [
    "Thank you for being part of Rootstock.",
    "We appreciate everyone building Rootstock together.",
    "Thank you for making the Rootstock community stronger.",
    "We are grateful to share Rootstock with you.",
    "Thank you for everything you bring to Rootstock.",
];
pub fn sanitize_publisher_name(input: &str) -> Option<String> {
    let controls = Regex::new(r"[\p{Cc}\p{Cf}]+").expect("static regex");
    let links = Regex::new(
        r"(?ix)
        (?:https?://|www\.)\S+
        |
        \b[a-z0-9][a-z0-9-]*(?:\.[a-z0-9-]+)+(?:/\S*)?
        ",
    )
    .expect("static regex");
    let without_controls = controls.replace_all(input, " ");
    let without_links = links.replace_all(&without_controls, " ");
    let compact = without_links.split_whitespace().collect::<Vec<_>>().join(" ");
    let result = compact.chars().take(50).collect::<String>();
    (!result.is_empty()).then_some(result)
}

pub fn render(kind: &str, language: &str, stable_key: &str, payload: &Value) -> anyhow::Result<String> {
    if language != "en" {
        anyhow::bail!("unsupported notification language");
    }
    let index = template_index(kind, language, stable_key);
    let opening_index = index / 5;
    let thanks_index = index % 5;
    let (opening, thanks) = (EN_OPENINGS[opening_index], EN_THANKS[thanks_index]);
    let name = payload.get("sender_name").and_then(Value::as_str).and_then(sanitize_publisher_name);

    let body = match (kind, name) {
        ("disbursement_completed", Some(name)) => {
            format!("{name} was thinking of you! There is a surprise waiting in your wallets.")
        }
        ("disbursement_completed", None) => {
            "Someone was thinking of you! There is a surprise waiting in your wallets.".to_string()
        }
        ("low_gas", _) => "The Asami automator needs RBTC to continue paying network fees.".to_string(),
        ("new_unconfigured_token", _) => {
            "Asami received a token that is not configured. It was not accepted or distributed automatically."
                .to_string()
        }
        _ => anyhow::bail!("unknown notification kind or language"),
    };
    let rendered = format!("{opening} {body} {thanks}");
    if rendered.chars().count() > 280 {
        anyhow::bail!("rendered post exceeds 280 characters");
    }
    Ok(rendered)
}

fn template_index(kind: &str, language: &str, stable_key: &str) -> usize {
    let digest = Sha256::digest(format!("{kind}:{language}:{stable_key}").as_bytes());
    u64::from_be_bytes(digest[0..8].try_into().expect("eight bytes")) as usize % 50
}

pub async fn queue_notification(
    tx: &mut Transaction<'_, Postgres>,
    kind: &str,
    dedupe_key: &str,
    payload: Value,
) -> anyhow::Result<i64> {
    let mut payload = payload;
    payload["dedupe_key"] = json!(dedupe_key);
    insert_queue(tx, kind, "en", "asami_english", &payload).await
}

async fn insert_queue(
    tx: &mut Transaction<'_, Postgres>,
    kind: &str,
    language: &str,
    account_key: &str,
    payload: &Value,
) -> anyhow::Result<i64> {
    let row = sqlx::query(
        "INSERT INTO tweet_queue(kind, language, account_key, payload_json)
         VALUES ($1, $2, $3, $4)
         ON CONFLICT(kind, language, account_key, (payload_json->>'dedupe_key'))
         DO UPDATE SET updated_at = tweet_queue.updated_at
         RETURNING id",
    )
    .bind(kind)
    .bind(language)
    .bind(account_key)
    .bind(payload)
    .fetch_one(&mut **tx)
    .await?;
    use sqlx::Row;
    Ok(row.get(0))
}

pub async fn queue_unconfigured_tokens(app: &App) -> anyhow::Result<usize> {
    use sqlx::Row;
    let rows = sqlx::query(
        "SELECT chain_id, token_address, first_seen_transaction_hash, first_seen_block
         FROM unconfigured_seen_tokens unknown
         WHERE NOT EXISTS (
           SELECT 1 FROM seen_token_alerts alerted
           WHERE alerted.chain_id = unknown.chain_id
             AND alerted.token_address = unknown.token_address
         )
         ORDER BY first_seen_block",
    )
    .fetch_all(&app.db)
    .await?;
    let mut created = 0;
    for row in rows {
        let chain_id: i64 = row.get(0);
        let token: String = row.get(1);
        let transaction: String = row.get(2);
        let block: i64 = row.get(3);
        let mut tx = app.db.begin().await?;
        let dedupe = format!("unconfigured-token:{chain_id}:{token}");
        let en = queue_notification(
            &mut tx,
            "new_unconfigured_token",
            &dedupe,
            json!({
                "chain_id": chain_id,
                "token_address": token,
                "first_seen_transaction_hash": transaction,
                "first_seen_block": block,
            }),
        )
        .await?;
        sqlx::query(
            "INSERT INTO seen_token_alerts
             (chain_id, token_address, english_tweet_queue_id)
             VALUES ($1, $2, $3)
             ON CONFLICT DO NOTHING",
        )
        .bind(chain_id)
        .bind(&token)
        .bind(en)
        .execute(&mut *tx)
        .await?;
        tx.commit().await?;
        created += 1;
    }
    Ok(created)
}

pub async fn render_pending(app: &App) -> anyhow::Result<usize> {
    use sqlx::Row;
    let rows = sqlx::query(
        "SELECT id, kind, language, payload_json
         FROM tweet_queue
         WHERE status = 'pending' AND not_before <= now()
         ORDER BY id
         LIMIT 100",
    )
    .fetch_all(&app.db)
    .await?;
    for row in &rows {
        let id: i64 = row.get(0);
        let kind: String = row.get(1);
        let language: String = row.get(2);
        let payload: Value = row.get(3);
        let key = payload.get("dedupe_key").and_then(Value::as_str).context("tweet payload dedupe_key")?;
        let text = render(&kind, &language, key, &payload)?;
        sqlx::query(
            "UPDATE tweet_queue
             SET rendered_text = $2, status = 'ready', updated_at = now()
             WHERE id = $1 AND status = 'pending'",
        )
        .bind(id)
        .bind(text)
        .execute(&app.db)
        .await?;
    }
    Ok(rows.len())
}

pub async fn queue_low_gas(
    app: &App,
    automator: &str,
    current_balance: &str,
    estimated_cost: &str,
) -> anyhow::Result<bool> {
    let dedupe = format!("low-gas:{}:{}", app.settings.chain_id, automator);
    let mut tx = app.db.begin().await?;
    let acquired = sqlx::query(
        "INSERT INTO notification_deduplications(dedupe_key, expires_at)
         VALUES ($1, now() + make_interval(hours => $2::int))
         ON CONFLICT(dedupe_key) DO UPDATE
           SET expires_at = EXCLUDED.expires_at
           WHERE notification_deduplications.expires_at <= now()
         RETURNING dedupe_key",
    )
    .bind(&dedupe)
    .bind(app.settings.low_gas_tweet_cooldown_hours)
    .fetch_optional(&mut *tx)
    .await?
    .is_some();
    if acquired {
        queue_notification(
            &mut tx,
            "low_gas",
            &format!("{dedupe}:{}", chrono::Utc::now().date_naive()),
            json!({
                "chain_id": app.settings.chain_id,
                "automator_address": automator,
                "current_balance": current_balance,
                "estimated_next_cost": estimated_cost,
            }),
        )
        .await?;
    }
    tx.commit().await?;
    Ok(acquired)
}

pub async fn queue_completion(
    tx: &mut Transaction<'_, Postgres>,
    disbursement_id: i64,
    sender: &str,
    sender_name: Option<String>,
) -> anyhow::Result<()> {
    queue_notification(
        tx,
        "disbursement_completed",
        &format!("disbursement:{disbursement_id}"),
        json!({
            "disbursement_id": disbursement_id,
            "sender": sender,
            "sender_name": sender_name,
        }),
    )
    .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn names_keep_handles_and_remove_links() {
        assert_eq!(
            sanitize_publisher_name("@alice Alice https://evil.example path.com"),
            Some("@alice Alice".to_string())
        );
        assert_eq!(sanitize_publisher_name("https://evil.example"), None);
    }

    #[test]
    fn selection_is_deterministic_and_bounded() {
        let payload = json!({"sender_name": "Alice"});
        let first = render("disbursement_completed", "en", "one", &payload).unwrap();
        let second = render("disbursement_completed", "en", "one", &payload).unwrap();
        assert_eq!(first, second);
        assert!(first.chars().count() <= 280);
    }

    #[test]
    fn all_fifty_variants_are_reachable() {
        let mut found = HashMap::new();
        for index in 0..10_000 {
            let key = index.to_string();
            found.insert(template_index("low_gas", "en", &key), key);
        }
        assert_eq!(found.len(), 50);
    }
}
