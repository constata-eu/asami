use super::*;
use sqlx::{ ConnectOptions, postgres::{PgPoolOptions, PgConnectOptions}};
pub use serde::{Serialize, Deserialize};
use sqlx_models_orm::Db;
use std::str::FromStr;
use rocket::Config;
use super::on_chain::OnChain;
use super::models::*;
use std::io::{stdin, Read};

#[derive(Clone)]
pub struct App {
  pub settings: Box<AppConfig>,
  pub db: Db,
  pub on_chain: OnChain,
}

impl App {
  pub async fn from_stdin_password() -> AsamiResult<Self> {
    let mut password = String::new();
    stdin().read_to_string(&mut password)
      .map_err(|_| Error::Init("Could not get password from stdin".to_string()) )?;
    password.pop(); // Remove newline
    let config = AppConfig::default()?;
    Self::new(password, config).await
  }

  pub async fn new(password: String, config: AppConfig) -> AsamiResult<Self> {
    let db = config.db().await?;
    let on_chain = OnChain::new(&config, &password).await?;
    Ok(Self{ db, on_chain, settings: Box::new(config) })
  }

  pub async fn run_background_tasks(&self) -> AsamiResult<()> {
    self.campaign_request().submit_approvals().await?;
    self.campaign_request().submit_all().await?;
    self.handle_request().submit_all().await?;
    self.collab_request().submit_all().await?;
    self.claim_account_request().submit_all().await?;
    self.synced_event().sync_on_chain_events().await?;
    Ok(())
  }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
  pub database_uri: String,
  pub recaptcha_threshold: f64,
  pub pwa_host: String,
  pub rsk: Rsk,
  pub x: XConfig,
  pub instagram: InstagramConfig,
}

impl AppConfig {
  pub fn default() -> AsamiResult<Self> {
    Ok(Config::figment().extract::<Self>()?)
  }

  pub async fn db(&self) -> AsamiResult<Db> {
    let mut options = PgConnectOptions::from_str(&self.database_uri)?;
    options.disable_statement_logging();
    let pool_options = PgPoolOptions::new().max_connections(5);
    let pool = pool_options.connect_with(options).await?;
    Ok(Db{ pool, transaction: None })
  }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct XConfig {
  pub client_id: String,
  pub client_secret: String,
  pub redirect_uri: String,
  pub bearer_token: String,
  pub asami_user_id: u64,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct InstagramConfig {
  pub client_id: String,
  pub client_secret: String,
  pub redirect_uri: String,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Rsk {
  pub chain_id: u64,
  pub rpc_url: String,
  pub start_sync_from_block: u64,
  pub contract_address: String,
  pub doc_contract_address: String,
  pub wallet_mnemonic: String,
  pub admin_address: String,
  pub reorg_protection_padding: ethers::types::U64,
}
