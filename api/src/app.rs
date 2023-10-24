use super::*;
use sqlx::{ ConnectOptions, postgres::{PgPoolOptions, PgConnectOptions}};
pub use serde::{Serialize, Deserialize};
use sqlx_models_orm::Db;
use std::str::FromStr;
use rocket::Config;
use super::on_chain::{AsamiContractEvents, OnChain};
use super::models::*;
use std::io::{stdin, Read};
use ethers::abi::AbiEncode;

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
    let mut options = PgConnectOptions::from_str(&config.database_uri)?;
    options.disable_statement_logging();
    let pool_options = PgPoolOptions::new().max_connections(5);
    let pool = pool_options.connect_with(options).await?;
    let db = Db{ pool, transaction: None };
    let on_chain = OnChain::new(&config, &password).await?;

    Ok(Self{ db, on_chain, settings: Box::new(config) })
  }

  pub async fn sync_on_chain_events(&self) -> AsamiResult<()> {
    use ethers::middleware::Middleware;
    let state = self.indexer_state().get().await?;
    let to_block = i64::try_from(self.on_chain.contract.client().get_block_number().await?.as_u64()).unwrap() - self.settings.rsk.reorg_protection_padding;
    let events = self.on_chain.events(state.attrs.last_synced_block, to_block).await?;

    for (event, meta) in events {
      match event {
        AsamiContractEvents::XhandleAddedFilter(e) => {
          let maybe = self.handle().select()
            .value_eq(e.handle.value)
            .status_eq(HandleStatus::Submitted)
            .tx_hash_eq(meta.transaction_hash.encode_hex())
            .site_eq(Site::X)
            .optional().await?;

          if let Some(h) = maybe {
            h.activate().await?;
          }

        },
        AsamiContractEvents::XcampaignAddedFilter(e) => {
          let req = self.campaign_request().select()
            .content_id_eq(&e.campaign.content_id)
            .status_eq(CampaignRequestStatus::Submitted)
            .tx_hash_eq(meta.transaction_hash.encode_hex())
            .site_eq(Site::X)
            .optional().await?;

          if let Some(r) = req {
            r.activate().await?;
          }

          let not_synced = self.campaign().select()
            .block_number_eq(Decimal::from(meta.block_number.as_u64()))
            .log_index_eq(Decimal::from(meta.log_index.as_u64()))
            .optional().await?.is_none();

          if not_synced {
            self.campaign().insert(InsertCampaign{
              on_chain_id: Decimal::from(e.campaign_id.as_u64()),
              account_id: e.account_id.as_u32().try_into().unwrap(),
              site: Site::X,
              budget: e.campaign.budget.as_u64().into(),
              remaining: e.campaign.remaining.as_u64().into(),
              content_id: e.campaign.content_id,
              block_number: meta.block_number.as_u64().into(),
              log_index: meta.block_number.as_u64().into(),
              tx_hash: meta.transaction_hash.encode_hex(),
            }).save().await?;
          }
        }
        _ => {}
      }

    }

    state.update().last_synced_block(to_block).save().await?;

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
  pub reorg_protection_padding: i64,
}
