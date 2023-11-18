use std::time::Duration;

#[tokio::main]
async fn main() {
  let app = api::App::from_stdin_password().await.unwrap();

  loop {
    if let Err(e) = app.run_background_tasks().await {
      println!("Got error running periodic tasks {:?}", e);
      tokio::time::sleep(Duration::from_millis(1000)).await;
    }
  }
}
