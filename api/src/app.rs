use std::{
    io::{stdin, Read},
    str::FromStr,
};

use ethers::abi::Address;
use rocket::Config;
pub use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    ConnectOptions,
};
use sqlx_models_orm::Db;
use twitter_v2::authorization::Oauth2Client;

use super::{models::*, on_chain::OnChain, *};

#[derive(Clone)]
pub struct App {
    pub settings: Box<AppConfig>,
    pub db: Db,
    pub on_chain: OnChain,
    pub stripe_client: Box<stripe::Client>,
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
        let stripe_client = stripe::Client::new(&config.stripe.secret_key);

        Ok(Self {
            db,
            on_chain,
            settings: Box::new(config),
            stripe_client: Box::new(stripe_client)
        })
    }

    pub async fn info<S: serde::Serialize>(&self, kind: &str, subkind: &str, context: S) {
        let _ = self.audit_log_entry().info(kind, subkind, context).await;
    }

    pub async fn fail<S: serde::Serialize>(&self, kind: &str, subkind: &str, context: S) {
        let _ = self.audit_log_entry().fail(kind, subkind, context).await;
    }

    pub async fn transactional(&self) -> sqlx::Result<Self> {
        sqlx::Result::Ok(Self {
            db: self.db.transaction().await?,
            settings: self.settings.clone(),
            on_chain: self.on_chain.clone(),
            stripe_client: self.stripe_client.clone(),
        })
    }

    pub async fn commit(&self) -> sqlx::Result<()> {
        self.db.commit().await?;
        sqlx::Result::Ok(())
    }

    pub async fn send_mail(&self, to: &str, subject: &str, content: &str) -> AsamiResult<()> {
        // Create the JSON body
        let body = serde_json::json!({
            "personalizations": [{
                "to": [{ "email": to }]
            }],
            "from": { "email": "asami@asami.club" },
            "subject": subject,
            "content": [{
                "type": "text/html",
                "value": content,
            }]
        });

        // Perform the POST request
        ureq::post("https://api.sendgrid.com/v3/mail/send")
            .set("Authorization", &format!("Bearer {}", self.settings.sendgrid_api_key))
            .set("Content-Type", "application/json")
            .send_json(body)?;

        Ok(())
    }

    pub async fn get_admin_own_account(&self) -> AsamiResult<Account> {
        let addr = self.on_chain.client.address().encode_hex();
        let found = self.account().select().addr_eq(&addr).optional().await?;

        if let Some(account) = found {
            return Ok(account);
        }

        self.on_chain.asami_contract.config_account(self.on_chain.client.address(), u("1"), wei("6000000000000"), u("0")).await?;

        let account = self.account().insert(InsertAccount{
            name: Some("Constata.eu Campaign Manager".to_string()),
            addr: Some(addr)
        }).save().await?;

        Ok(account.update().status(AccountStatus::Claimed).save().await?)

    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub database_uri: String,
    pub recaptcha_threshold: f64,
    pub pwa_host: String,
    pub rsk: Rsk,
    pub x: XConfig,
    pub sendgrid_api_key: String,
    pub internal_alerts_email: String,
    pub stripe: StripeSettings, 
}

impl AppConfig {
    pub fn default_figment() -> AsamiResult<Self> {
        Ok(Config::figment().extract::<Self>()?)
    }

    pub async fn db(&self) -> AsamiResult<Db> {
        let options = PgConnectOptions::from_str(&self.database_uri)?.disable_statement_logging();
        let pool_options = PgPoolOptions::new().max_connections(50);
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
    pub score_cooldown_seconds: u64,
}

impl XConfig {
    pub fn oauth_client(&self, redirect_path: &str) -> AsamiResult<Oauth2Client> {
        let url = format!("{}{}", self.redirect_uri, redirect_path)
            .parse()
            .map_err(|_| Error::precondition("x_redirect_url_cannot_be_parsed"))?;

        Ok(Oauth2Client::new(
            self.client_id.clone(),
            self.client_secret.clone(),
            url,
        ))
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Rsk {
    pub chain_id: u64,
    pub rpc_url: String,
    pub start_sync_from_block: i64,
    pub legacy_contract_address: String,
    pub asami_contract_address: String,
    pub doc_contract_address: String,
    pub wallet_mnemonic: String,
    pub admin_address: Address,
    pub reorg_protection_padding: ethers::types::U64,
    pub blockchain_sync_cooldown: u64,
    pub gasless_rbtc_per_user: U256,
    pub gasless_fee: U256,
    pub admin_claim_trigger: U256,
    pub gas_override: Option<bool>,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct StripeSettings {
  pub secret_key: String,
  pub public_key: String,
  pub events_secret: String,
  pub success_url: String,
  pub cancel_url: String,
}

