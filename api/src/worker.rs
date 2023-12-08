use std::time::Duration;

#[tokio::main]
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

  every![settings.rsk.blockchain_sync_cooldown, |s| {
    run!("blockchain_sync_tasks" { s.run_background_tasks().await });
  }];

  every![settings.x.crawl_cooldown_minutes * 60 * 1000, |s| {
    run!("sync_x_collabs" { s.campaign().sync_x_collabs().await });
    run!("verify_and_appraise_x_handles" { s.handle_request().verify_and_appraise_x().await });
  }];

  every![3 * 60 * 1000, |s| {
    run!("ig_crawler do everything" { s.ig_crawl().do_everything().await });
  }];

  futures::future::join_all(handles).await;
}
