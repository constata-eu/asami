use super::*;
use sqlx::{ ConnectOptions, postgres::{PgPoolOptions, PgConnectOptions}};
pub use serde::{Serialize, Deserialize};
use sqlx_models_orm::Db;
use std::str::FromStr;
use rocket::Config;

#[derive(Clone)]
pub struct App {
  pub settings: Box<AppConfig>,
  pub db: Db,
}

impl App {
  pub async fn default() -> AsamiResult<Self> {
    let config = AppConfig::default()?;
    let mut options = PgConnectOptions::from_str(&config.database_uri)?;
    options.disable_statement_logging();
    let pool_options = PgPoolOptions::new().max_connections(5);
    let pool = pool_options.connect_with(options).await?;
    let db = Db{ pool, transaction: None };

    Ok(Self{ db, settings: Box::new(config) })
  }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
  pub database_uri: String,
  pub recaptcha_threshold: f64,
}

impl AppConfig {
  pub fn default() -> AsamiResult<Self> {
    Ok(Config::figment().extract::<Self>()?)
  }
}

