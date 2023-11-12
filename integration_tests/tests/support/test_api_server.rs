use std::process::{Child, Command, Stdio};

pub struct TestApiServer;

impl TestApiServer {
  pub async fn start(app: api::App) -> tokio::task::JoinHandle<()> {
    let server = tokio::spawn( async { api::server(app).launch().await.unwrap(); } );

    for i in 0..100 {
      let status = ureq::get("http://localhost:8000/graphql/introspect").call();
      if status.is_ok() {
        break;
      }
      std::thread::sleep(std::time::Duration::from_millis(500));
      if i == 99 && std::env::var("CI").is_err() {
        assert!(false, "Public api server is taking too long. Try compiling separately and come back.");
      }
    }

    server
  }
}
