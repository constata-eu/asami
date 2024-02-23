pub struct TestApiServer;
use rocket::{Config, config::LogLevel};

impl TestApiServer {
  pub async fn start(app: api::App) -> tokio::task::JoinHandle<()> {
    let fig = Config::figment().merge((Config::LOG_LEVEL, LogLevel::Off));
    let server = tokio::spawn( async move { api::custom_server(app, fig).launch().await.unwrap(); } );

    for i in 0..100 {
      let status = ureq::get("http://localhost:8000/graphql/introspect").call();
      if status.is_ok() {
        break;
      }
      std::thread::sleep(std::time::Duration::from_millis(50));
      if i == 99 && std::env::var("CI").is_err() {
        assert!(false, "Public api server is taking too long. Maybe it errored out when launching?.");
      }
    }

    server
  }
}
