use super::{models::*, on_chain::OnChain, *};
use rocket::Config;
pub use serde::{Deserialize, Serialize};
use sqlx::{
  postgres::{PgConnectOptions, PgPoolOptions},
  ConnectOptions,
};
use sqlx_models_orm::Db;
use std::{
  io::{stdin, Read},
  str::FromStr,
};

#[derive(Clone)]
pub struct App {
  pub settings: Box<AppConfig>,
  pub db: Db,
  pub on_chain: OnChain,
}

impl App {
  pub async fn from_stdin_password() -> AsamiResult<Self> {
    let mut password = String::new();
    stdin()
      .read_to_string(&mut password)
      .map_err(|_| Error::Init("Could not get password from stdin".to_string()))?;
    password.pop(); // Remove newline
    let config = AppConfig::default_figment()?;
    Self::new(password, config).await
  }

  pub async fn new(password: String, config: AppConfig) -> AsamiResult<Self> {
    let db = config.db().await?;
    let on_chain = OnChain::new(&config, &password).await?;
    Ok(Self {
      db,
      on_chain,
      settings: Box::new(config),
    })
  }

  pub async fn run_background_tasks(&self) -> anyhow::Result<()> {
    self.on_chain_tx().proclaim_cycle_admin_winner().await?;
    self.on_chain_tx().apply_handle_updates().await?;
    self.on_chain_tx().apply_voted_fee_rate().await?;
    self.on_chain_tx().reimburse_due_campaigns().await?;
    self.on_chain_tx().vest_admin_votes().await?;
    self.on_chain_tx().distribute_fee_pool().await?;

    self.collab_request().submit_all().await?;
    self.handle_request().submit_all().await?;
    self.set_price_request().submit_all().await?;
    self.set_score_and_topics_request().submit_all().await?;
    self.topic_request().submit_all().await?;
    self.claim_account_request().submit_all().await?;
    self.campaign_request().submit_approvals().await?;
    self.campaign_request().submit_all().await?;

    self.synced_event().sync_on_chain_events().await?;
    self.on_chain_tx().sync_tx_result().await?;
    Ok(())
  }

  pub async fn info<S: serde::Serialize>(&self, kind: &str, subkind: &str, context: S) {
    let _ = self.audit_log_entry().info(kind, subkind, context).await;
  }

  pub async fn fail<S: serde::Serialize>(&self, kind: &str, subkind: &str, context: S) {
    let _ = self.audit_log_entry().fail(kind, subkind, context).await;
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
  pub fn default_figment() -> AsamiResult<Self> {
    Ok(Config::figment().extract::<Self>()?)
  }

  pub async fn db(&self) -> AsamiResult<Db> {
    let mut options = PgConnectOptions::from_str(&self.database_uri)?;
    options.disable_statement_logging();
    let pool_options = PgPoolOptions::new().max_connections(5);
    let pool = pool_options.connect_with(options).await?;
    Ok(Db {
      pool,
      transaction: None,
    })
  }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct XConfig {
  pub client_id: String,
  pub client_secret: String,
  pub redirect_uri: String,
  pub bearer_token: String,
  pub asami_user_id: u64,
  pub crawl_cooldown_minutes: u64,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct InstagramConfig {
  pub client_id: String,
  pub client_secret: String,
  pub redirect_uri: String,
  pub crawl_cooldown_minutes: i64,
  pub apify_key: String,
  pub verification_image_url: String,
  pub verification_caption: String,
  pub verification_posts_count: i64,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Rsk {
  pub chain_id: u64,
  pub rpc_url: String,
  pub start_sync_from_block: i64,
  pub contract_address: String,
  pub asami_contract_address: String,
  pub doc_contract_address: String,
  pub wallet_mnemonic: String,
  pub admin_address: String,
  pub reorg_protection_padding: ethers::types::U64,
  pub blockchain_sync_cooldown: u64,
}
