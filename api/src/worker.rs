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
        ($name:literal, $app:ident, {$($blk:tt)*}) => (
            println!("Running: {}", $name);
            if let Err(err) = { $($blk)* } {
                let _ = $app.send_mail(
                    &$app.settings.internal_alerts_email,
                    &format!("Error in {}", $name),
                    &format!("{:?}", &err)
                ).await;
                $app.fail("worker", $name, &err.to_string()).await;
                println!("Error in {}: {:?}", $name, err);
            }
        )
    }

    // The on chain jobs scheduler has its own internal cooldown and backoff.
    // We just make sure to wake it up once a second here so that it decides what to do.
    every![10000, |s| {
        run!("On chain job scheduler", s, { s.on_chain_job().run_scheduler().await });
    }];
    every![120000, |s| {
        run!("Sync on-chain events", s, {
            s.synced_event().sync_on_chain_events().await
        });
    }];

    every![60000, |s| {
        run!("Create CC campaigns", s, {
            s.campaign().create_managed_on_chain_campaigns().await
        });
    }];

    every![10000, |s| {
        run!("Force account hydrations", s, { s.account().force_hydrate().await });
        run!("Force handle hydrations", s, { s.handle().force_hydrate().await });
        run!("Force campaign hydrations", s, { s.campaign().force_hydrate().await });
        run!("Force community-member hydrations", s, {
            s.community_member().force_hydrate().await
        });
    }];

    every![10000, |s| {
        run!("Attempt account on-chain hydrate", s, {
            s.account().hydrate_on_chain_values_just_in_case().await
        });
    }];

    every![settings.x.crawl_cooldown_minutes * 60 * 1000, |s| {
        run!("verify_pending", s, { s.handle().verify_pending().await });
        run!("score_pending", s, { s.handle().score_pending().await });
        run!("sync_x_collabs", s, { s.campaign().sync_x_collabs().await });
    }];

    every![1000, |s| {
        run!("send_email_one_time_tokens", s, {
            s.one_time_token().send_email_tokens().await
        });
    }];

    futures::future::join_all(handles).await;
}
