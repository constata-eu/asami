use super::*;
use sqlx::{ ConnectOptions, postgres::{PgPoolOptions, PgConnectOptions}};
pub use serde::{Serialize, Deserialize};
use sqlx_models_orm::Db;
use std::str::FromStr;
use rocket::Config;
use ethers::{
  signers::{LocalWallet, MnemonicBuilder, coins_bip39::English, Signer},
  prelude::{abigen, Abigen},
  types::Address,
  providers::{Http, Provider},
};
use std::io::{stdin, Read};

#[derive(Clone)]
pub struct App {
  pub settings: Box<AppConfig>,
  pub db: Db,
  pub wallet: LocalWallet,
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

    let wallet = MnemonicBuilder::<English>::default()
      .phrase(config.wallet_mnemonic.as_str())
      .password(&password)
      .build()?;

    let wallet_address: String = serde_json::to_string(&wallet.address())
      .and_then(|s| serde_json::from_str::<String>(&s) )
      .map_err(|_| Error::Init("Could not serialize wallet address to json".to_string()) )?;

    if wallet_address != config.admin_address {
      return Err(Error::Init("Bad wallet password. Address does not match mnemonic.".to_string()));
    }

    Ok(Self{ db, wallet, settings: Box::new(config) })
  }
}

pub mod asami_contract {
  use ethers::{
    signers::{LocalWallet, MnemonicBuilder, coins_bip39::English, Signer},
    prelude::{abigen, Abigen},
    types::Address,
    providers::{Http, Provider},
  };

  abigen!(AsamiContract, "../contract/build/contracts/Asami.json");
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
  pub database_uri: String,
  pub recaptcha_threshold: f64,
  pub pwa_host: String,
  pub rsk: Rsk,
  pub x: XConfig,
  pub instagram: InstagramConfig,
  pub contract_address: String,
  pub wallet_mnemonic: String,
  pub admin_address: String,
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
}
