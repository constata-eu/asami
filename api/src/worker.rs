use std::time::Duration;

#[tokio::main]
async fn main() {
  let app = api::App::from_stdin_password().await.unwrap();
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

  every![5000, |s| {
    run!("blockchain_sync_tasks" { s.run_background_tasks().await });
  }];

  every![600000, |s| {
    run!("verify_handles" { s.handle_request().verify_and_appraise_all().await });
    run!("sync_x_collabs" { s.campaign().sync_x_collabs().await });
  }];

  futures::future::join_all(handles).await;
}
