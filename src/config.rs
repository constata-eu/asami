use std::{env, net::SocketAddr, str::FromStr};

use anyhow::{bail, Context};
use ethers::{
    signers::{coins_bip39::English, LocalWallet, MnemonicBuilder, Signer},
    types::{Address, U256},
};
use serde::Serialize;

pub const ZERO_ADDRESS: &str = "0x0000000000000000000000000000000000000000";

#[derive(Clone, Debug)]
pub struct Settings {
    pub database_url: String,
    pub rpc_url: String,
    pub chain_id: u64,
    pub contract_address: Address,
    pub contract_deploy_block: u64,
    pub automator_mnemonic: Option<String>,
    pub automator_address: Option<Address>,
    pub automator_derivation_path: String,
    pub automator_private_key: Option<String>,
    pub listen_address: SocketAddr,
    pub explorer_url: String,
    pub worker_interval_ms: u64,
    pub rpc_poll_interval_ms: u64,
    pub block_chunk_size: u64,
    pub finality_depth: u64,
    pub transfer_reconsideration_limit: i64,
    pub min_automator_gas_reserve: U256,
    pub gas_safety_basis_points: u64,
    pub low_gas_tweet_cooldown_hours: i64,
    pub x_daily_post_cap: i64,
    pub x_client_id: Option<String>,
    pub x_client_secret: Option<String>,
    pub x_redirect_uri: Option<String>,
    pub encryption_key: Option<[u8; 32]>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicConfig {
    pub chain_id: u64,
    pub contract_address: String,
    pub explorer_url: String,
    pub contract_url: String,
    pub sender_registration_url: String,
    pub x_english: &'static str,
}

impl Settings {
    pub fn from_env() -> anyhow::Result<Self> {
        let chain_id = value("ROOTSTOCK_CHAIN_ID", "31337").parse()?;
        let contract_address =
            Address::from_str(&value("ASAMI_CONTRACT_ADDRESS", ZERO_ADDRESS)).context("ASAMI_CONTRACT_ADDRESS")?;
        let encryption_key = match env::var("ASAMI_ENCRYPTION_KEY") {
            Ok(encoded) => {
                use base64::Engine;
                let bytes = base64::engine::general_purpose::STANDARD
                    .decode(encoded)
                    .context("ASAMI_ENCRYPTION_KEY must be base64")?;
                if bytes.len() != 32 {
                    bail!("ASAMI_ENCRYPTION_KEY must decode to 32 bytes");
                }
                let mut key = [0_u8; 32];
                key.copy_from_slice(&bytes);
                Some(key)
            }
            Err(_) => None,
        };

        Ok(Self {
            database_url: required("DATABASE_URL")?,
            rpc_url: value("ROOTSTOCK_RPC_URL", "http://127.0.0.1:8545"),
            chain_id,
            contract_address,
            contract_deploy_block: value("CONTRACT_DEPLOY_BLOCK", "0").parse()?,
            automator_mnemonic: optional("AUTOMATOR_MNEMONIC"),
            automator_address: optional("DISBURSEMENT_AUTOMATOR_ADDRESS")
                .map(|address| Address::from_str(&address).context("DISBURSEMENT_AUTOMATOR_ADDRESS"))
                .transpose()?,
            automator_derivation_path: value("AUTOMATOR_DERIVATION_PATH", "m/44'/60'/0'/0/0"),
            // Retained for local development and tests. Production should use the mnemonic.
            automator_private_key: optional("AUTOMATOR_PRIVATE_KEY"),
            listen_address: value("LISTEN_ADDRESS", "127.0.0.1:8000").parse()?,
            explorer_url: value("ROOTSTOCK_EXPLORER_URL", "https://rootstock.blockscout.com"),
            worker_interval_ms: value("WORKER_INTERVAL_MS", "15000").parse()?,
            rpc_poll_interval_ms: value("RPC_POLL_INTERVAL_MS", "1000").parse()?,
            block_chunk_size: value("BLOCK_CHUNK_SIZE", "1000").parse()?,
            finality_depth: value("FINALITY_DEPTH", "12").parse()?,
            transfer_reconsideration_limit: value("TRANSFER_RECONSIDERATION_LIMIT", "100").parse()?,
            min_automator_gas_reserve: U256::from_dec_str(&value("MIN_AUTOMATOR_GAS_RESERVE", "1000000000000000"))?,
            gas_safety_basis_points: value("GAS_BALANCE_SAFETY_BASIS_POINTS", "12500").parse()?,
            low_gas_tweet_cooldown_hours: value("LOW_GAS_TWEET_COOLDOWN_HOURS", "24").parse()?,
            x_daily_post_cap: value("X_DAILY_POST_CAP", "20").parse()?,
            x_client_id: env::var("X_CLIENT_ID").ok(),
            x_client_secret: env::var("X_CLIENT_SECRET").ok(),
            x_redirect_uri: env::var("X_REDIRECT_URI").ok(),
            encryption_key,
        })
    }

    pub fn public(&self) -> PublicConfig {
        let address = format!("{:#x}", self.contract_address);
        let base = self.explorer_url.trim_end_matches('/');
        PublicConfig {
            chain_id: self.chain_id,
            contract_address: address.clone(),
            explorer_url: self.explorer_url.clone(),
            contract_url: format!("{base}/address/{address}"),
            sender_registration_url: format!("{base}/address/{address}?tab=write_contract"),
            x_english: "@asami_club",
        }
    }

    pub fn safe_summary(&self) -> serde_json::Value {
        serde_json::json!({
            "database_configured": !self.database_url.is_empty(),
            "rpc_configured": !self.rpc_url.is_empty(),
            "chain_id": self.chain_id,
            "contract_address": format!("{:#x}", self.contract_address),
            "contract_deploy_block": self.contract_deploy_block,
            "listen_address": self.listen_address,
            "worker_interval_ms": self.worker_interval_ms,
            "block_chunk_size": self.block_chunk_size,
            "finality_depth": self.finality_depth,
            "transfer_reconsideration_limit": self.transfer_reconsideration_limit,
            "x_daily_post_cap": self.x_daily_post_cap,
            "automator_configured": self.automator_mnemonic.is_some() || self.automator_private_key.is_some(),
            "x_oauth_configured": self.x_client_id.is_some() && self.x_redirect_uri.is_some(),
            "encryption_configured": self.encryption_key.is_some(),
        })
    }

    pub fn unlock_automator(&mut self, passphrase: &str) -> anyhow::Result<()> {
        let Some(mnemonic) = self.automator_mnemonic.as_deref() else {
            return Ok(());
        };
        let expected = self
            .automator_address
            .context("DISBURSEMENT_AUTOMATOR_ADDRESS is required with AUTOMATOR_MNEMONIC")?;
        let wallet: LocalWallet = MnemonicBuilder::<English>::default()
            .phrase(mnemonic)
            .password(passphrase)
            .derivation_path(&self.automator_derivation_path)?
            .build()
            .context("derive automator wallet from AUTOMATOR_MNEMONIC")?;
        if wallet.address() != expected {
            bail!(
                "automator passphrase or derivation path is incorrect: derived {:#x}, expected {:#x}",
                wallet.address(),
                expected
            );
        }
        self.automator_private_key = Some(format!("0x{}", hex::encode(wallet.signer().to_bytes())));
        Ok(())
    }
}

fn required(name: &str) -> anyhow::Result<String> {
    env::var(name).with_context(|| format!("{name} is required"))
}

fn value(name: &str, default: &str) -> String {
    env::var(name).unwrap_or_else(|_| default.to_string())
}

fn optional(name: &str) -> Option<String> {
    env::var(name).ok().filter(|value| !value.trim().is_empty())
}
