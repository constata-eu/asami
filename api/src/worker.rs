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

    every![settings.x.crawl_cooldown_minutes * 60 * 1000, |s| {
        run!("sync_x_collabs" { s.campaign().sync_x_collabs().await });
        run!("verify_and_appraise_x_handles" { s.handle().verify_and_score_x().await });
    }];

    every![60 * 1000, |s| {
        run!("ig_crawler do everything" { s.ig_crawl().do_everything().await });
    }];

    futures::future::join_all(handles).await;
}
