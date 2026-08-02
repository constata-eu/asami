pub mod api;
pub mod chain;
pub mod config;
pub mod db;
pub mod notifications;
pub mod processor;
pub mod x;

use std::sync::Arc;

use anyhow::Context;
use ethers::providers::{Http, Provider};
use sqlx::PgPool;

use crate::config::Settings;

#[derive(Clone)]
pub struct App {
    pub settings: Arc<Settings>,
    pub db: PgPool,
    pub provider: Arc<Provider<Http>>,
    pub http: reqwest::Client,
}

impl App {
    pub async fn new(settings: Settings) -> anyhow::Result<Self> {
        let db = PgPool::connect(&settings.database_url).await.context("connect to PostgreSQL")?;
        let provider = Provider::<Http>::try_from(settings.rpc_url.as_str())
            .context("create Rootstock provider")?
            .interval(std::time::Duration::from_millis(settings.rpc_poll_interval_ms));
        Ok(Self {
            settings: Arc::new(settings),
            db,
            provider: Arc::new(provider),
            http: reqwest::Client::builder().timeout(std::time::Duration::from_secs(20)).build()?,
        })
    }
}
