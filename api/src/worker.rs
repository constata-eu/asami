use std::time::Duration;

#[tokio::main(worker_threads = 10)]
async fn main() {
    let app = api::App::from_stdin_password().await.unwrap();
    let settings = *app.clone().settings;
    let mut handles = vec![];

    macro_rules! every {
        ($wait:expr, |$app:ident| {$($blk:tt)*}) => (
            let $app = app.clone();
            handles.push(tokio::spawn(async move {
                loop {
                    { $($blk)* }
                    tokio::time::sleep(Duration::from_millis($wait)).await;
                }
            }));
        )
    }

    macro_rules! run {
        ($name:literal {$($blk:tt)*}) => (
            println!("Running: {}", $name);
            if let Err(err) = { $($blk)* } {
                println!("Error in {}: {:?}", $name, err);
            }
        )
    }

    // The on chain jobs scheduler has its own internal cooldown and backoff.
    // We just make sure to wake it up once a second here so that it decides what to do.
    every![10000, |s| {
        run!("On chain job scheduler" { s.on_chain_job().run_scheduler().await });
    }];
    every![60000, |s| {
        run!("Sync on-chain events" { s.synced_event().sync_on_chain_events().await });
    }];

    every![10000, |s| { 
        run!("Force account hydrations" { s.account().force_hydrate().await });
        run!("Force handle hydrations" { s.handle().force_hydrate().await });
        run!("Force campaign hydrations" { s.campaign().force_hydrate().await });
    }];

    every![10000, |s| {
        run!("Attempt account on-chain hydrate" { s.account().hydrate_on_chain_values_just_in_case().await });
    }];

    every![settings.x.crawl_cooldown_minutes * 60 * 1000, |s| {
        run!("sync_x_collabs" { s.campaign().sync_x_collabs().await });
    }];

    every![settings.x.crawl_cooldown_minutes * 60 * 1000, |s| {
        run!("verify_handles" { s.handle().verify_pending().await });
        run!("verify_score_pending" { s.handle().score_pending().await });
    }];

    every![1000, |s| {
        run!("send_email_one_time_tokens" { s.one_time_token().send_email_tokens().await });
    }];

    futures::future::join_all(handles).await;
}
