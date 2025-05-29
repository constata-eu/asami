pub use chrono::{DateTime, Datelike, Duration, TimeZone, Utc};
pub use serde::{Deserialize, Serialize};
pub use sqlx::{self, types::Decimal};
use sqlx_models_orm::model;

use super::{on_chain::LogMeta, *};
pub type UtcDateTime = DateTime<Utc>;
use std::str::FromStr;

pub use ethers::{
    abi::{AbiDecode, AbiEncode},
    middleware::Middleware,
    types::{transaction::eip712::TypedData, Address, Signature, H160, H256, U256, U64},
};
pub use juniper::GraphQLEnum;

pub mod hasher;
pub mod user;
pub use user::*;
pub mod synced_event;
pub use synced_event::*;
pub mod campaign;
pub use campaign::*;
pub mod account;
pub use account::*;
pub mod handle;
pub use handle::*;
pub mod campaign_preference;
pub use campaign_preference::*;
pub mod session;
pub use session::*;
pub mod audit_log_entry;
pub use audit_log_entry::*;
pub mod on_chain_job;
pub use on_chain_job::*;
pub mod site;
pub use site::*;
pub mod auth_method;
pub use auth_method::*;
pub mod topic;
pub use topic::*;
pub mod collab;
pub use collab::*;
pub mod holder;
pub use holder::*;
pub mod one_time_token;
pub use one_time_token::*;
pub mod handle_scoring;
pub use handle_scoring::*;
pub mod community_member;
pub use community_member::*;

#[macro_export]
macro_rules! make_sql_enum {
  ($sql_name:tt, pub enum $name:ident { $($variants:ident),* $(,)?}) => (
    #[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize, sqlx::Type, GraphQLEnum, strum_macros::EnumIter)]
    #[sqlx(type_name = $sql_name, rename_all = "snake_case")]
    pub enum $name { $($variants),* }
  )
}

model! {
  state: App,
  table: account_users,
  struct AccountUser {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(varchar)]
    account_id: String,
    #[sqlx_model_hints(int4)]
    user_id: i32,
  }
}

// This is an account profile when taking the Collaborator role.
model! {
  state: App,
  table: indexer_states,
  struct IndexerState {
    #[sqlx_model_hints(int4)]
    id: i32,
    #[sqlx_model_hints(int8, default)]
    x_handle_verification_checkpoint: i64,
    #[sqlx_model_hints(varchar, default)]
    suggested_price_per_point: String,
    #[sqlx_model_hints(decimal, default)]
    last_synced_block: Decimal,
  }
}

impl IndexerStateHub {
    pub async fn get(&self) -> sqlx::Result<IndexerState> {
        let Some(existing) = self.find_optional(1).await? else {
            return self.insert(InsertIndexerState { id: 1 }).save().await;
        };
        Ok(existing)
    }
}

// Unsafe conversion for values that we know for sure have an U256 hex encoded value.
pub fn u256<T: AsRef<str> + std::fmt::Debug>(u: T) -> U256 {
    U256::decode_hex(u).unwrap_or(U256::zero())
}

// Converts an account id expressed in hex to an i32
pub fn hex_to_i32(hex: &str) -> Result<i32, std::num::TryFromIntError> {
    u256(hex).as_u32().try_into()
}

pub fn i32_to_hex(i: i32) -> String {
    U256::from(i).encode_hex()
}

pub fn wei<T: AsRef<str>>(t: T) -> U256 {
    U256::from_dec_str(t.as_ref()).unwrap_or(U256::zero())
}

pub fn milli<T: AsRef<str>>(t: T) -> U256 {
    wei(t) * U256::from(10).pow(U256::from(15))
}

pub fn weihex<T: AsRef<str>>(t: T) -> String {
    wei(t).encode_hex()
}

pub fn u<T: AsRef<str>>(t: T) -> U256 {
    wei(t) * U256::from(10).pow(U256::from(18))
}

pub fn uhex<T: AsRef<str>>(t: T) -> String {
    u(t).encode_hex()
}

fn i_to_utc(u: U256) -> UtcDateTime {
    // We're absolutely sure this conversion is always possible.
    Utc.timestamp_opt(u.as_u64() as i64, 0).unwrap()
}

pub fn utc_to_i(d: UtcDateTime) -> U256 {
    d.timestamp().into()
}

pub fn u64_to_d(u: U64) -> Decimal {
    Decimal::from_u64(u.as_u64()).unwrap_or(Decimal::ZERO)
}

pub fn d_to_u64(d: Decimal) -> U64 {
    U64::from_dec_str(&d.to_string()).unwrap_or(U64::zero())
}

pub fn make_login_to_asami_typed_data(chain_id: u64) -> Result<TypedData, String> {
    let json = serde_json::json!( {
      "types": {
        "EIP712Domain": [
          { "name": "name", "type": "string" },
          { "name": "version", "type": "string" },
          { "name": "chainId", "type": "uint256" }
        ],
        "Acceptance": [
          { "name": "content", "type": "string" }
        ]
      },
      "primaryType": "Acceptance",
      "domain": { "name": "Asami", "version": "1", "chainId": chain_id.to_string() },
      "message": { "content": "Login to Asami" }
    });

    serde_json::from_value(json).map_err(|_| "unexpected_invalid_json".to_string())
}

pub fn eip_712_sig_to_address(chain_id: u64, signature: &str) -> Result<String, String> {
    let payload = make_login_to_asami_typed_data(chain_id)?;
    let sig = Signature::from_str(signature).map_err(|_| "invalid_auth_data_signature".to_string())?;
    sig.recover_typed_data(&payload)
        .map(|a| a.encode_hex())
        .map_err(|_| "could_not_recover_typed_data_from_sig".to_string())
}
