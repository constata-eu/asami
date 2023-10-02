use api::{App, AppConfig};
use jwt_simple::algorithms::*;
use std::process::Command;

#[derive(Clone)]
pub struct TestApp {
  pub app: App
}

impl TestApp {
  pub async fn init() -> Self {
    let database_uri = AppConfig::default().expect("config to exist").database_uri;

    Command::new("sqlx")
      .current_dir("../api")
      .env("DATABASE_URL", &database_uri)
      .args(&["database", "reset", "-y"])
      .output()
      .expect("SQLX not available.");

    Self{ app: App::default().await.unwrap() }
  }

  pub fn private_key(&self) -> ES256KeyPair {
    let key = ES256KeyPair::from_pem(
      "-----BEGIN PRIVATE KEY-----\nMIGHAgEAMBMGByqGSM49AgEGCCqGSM49AwEHBG0wawIBAQQg626FUHw6lA0eAlYl\nqT0TI8m/JAWj+H497JAKfoTUrkmhRANCAARJPbG33RdPLOxXXbc390w00YaFAbh8\n0Hv44ScjS0UMB6/ZjjkIs5fV1gRK1IBF1JMnxM6qWjWUBlu/z9ZjvA0b\n-----END PRIVATE KEY-----\n"
    ).unwrap();
    let id = api::models::hasher::hexdigest(&key.public_key().to_pem().unwrap().as_bytes());
    key.with_key_id(&id)
  }
}
