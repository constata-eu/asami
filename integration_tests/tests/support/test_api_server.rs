pub struct TestApiServer;

impl TestApiServer {
  pub async fn start(app: api::App) -> tokio::task::JoinHandle<()> {
    std::env::set_var("ROCKET_LOG_LEVEL", "off");

    let server = tokio::spawn( async { api::server(app).launch().await.unwrap(); } );

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
