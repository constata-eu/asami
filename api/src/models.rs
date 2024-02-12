use super::*;
use super::on_chain::{self, AsamiContractSigner, LogMeta};
pub use sqlx::{self, types::Decimal};
pub use serde::{Serialize, Deserialize};
use sqlx_models_orm::model;
pub use chrono::{DateTime, Duration, Utc, Datelike, TimeZone};
pub type UtcDateTime = DateTime<Utc>;
pub use juniper::GraphQLEnum;
pub use ethers::{ types::{ U256, U64, H160, H256, Signature, transaction::eip712::TypedData}, abi::{AbiEncode, AbiDecode}, middleware::Middleware};
use std::str::FromStr;

pub mod hasher;
pub mod user;
pub use user::*;
pub mod claim_account_request;
pub use claim_account_request::*;
pub mod synced_event;
pub use synced_event::*;
pub mod campaign_request;
pub use campaign_request::*;
pub mod handle_request;
pub use handle_request::*;
pub mod collab_request;
pub use collab_request::*;
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
pub mod ig_crawl;
pub use ig_crawl::*;
pub mod audit_log_entry;
pub use audit_log_entry::*;
pub mod on_chain_tx;
pub use on_chain_tx::*;
pub mod site;
pub use site::*;
pub mod set_score_and_topics_request;
pub use set_score_and_topics_request::*;
pub mod set_price_request;
pub use set_price_request::*;
pub mod auth_method;
pub use auth_method::*;
pub mod topic;
pub use topic::*;
pub mod topic_request;
pub use topic_request::*;

#[macro_export]
macro_rules! make_sql_enum {
  ($sql_name:tt, pub enum $name:ident { $($variants:ident),* $(,)?}) => (
    #[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize, sqlx::Type, GraphQLEnum)]
    #[sqlx(type_name = $sql_name, rename_all = "snake_case")]
    pub enum $name { $($variants),* }

    impl sqlx::postgres::PgHasArrayType for $name {
      fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name(concat!("_", $sql_name))
      }
    }
  )
}

make_sql_enum![
  "generic_request_status",
  pub enum GenericRequestStatus {
    Received,
    Submitted,
    Failed,
    Done,
  }
];

model!{
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

/* One time tokens are just single use strings for the one time token authentication method.
 * The authentication strategy will only check that the string exists and has not been used.
 * This token's id is referenced in the lookup key of (at least one) AuthMethod.
 */
model!{
  state: App,
  table: one_time_tokens,
  struct OneTimeToken {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(varchar)]
    value: String,
    #[sqlx_model_hints(boolean, default)]
    used: bool,
  }
}

model!{
  state: App,
  table: collabs,
  struct Collab {
    #[sqlx_model_hints(varchar)]
    id: String,
    #[sqlx_model_hints(varchar)]
    campaign_id: String,
    #[sqlx_model_hints(varchar)]
    advertiser_id: String,
    #[sqlx_model_hints(varchar)]
    handle_id: String, 
    #[sqlx_model_hints(varchar)]
    member_id: String,
    #[sqlx_model_hints(varchar)]
    gross: String,
    #[sqlx_model_hints(varchar)]
    fee: String,
    #[sqlx_model_hints(varchar)]
    created_at: String,
  }
}

// This is an account profile when taking the Collaborator role.
model!{
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
      return self.insert(InsertIndexerState{id: 1}).save().await
    };
    Ok(existing)
  }
}

// Unsafe conversion for values that we know for sure have an U256 hex encoded value.
pub fn u256<T: AsRef<str> + std::fmt::Debug>(u: T) -> U256 {
  U256::decode_hex(u).unwrap_or(U256::zero())
}

pub fn wei<T: AsRef<str>>(t: T) -> U256 {
  U256::from_dec_str(t.as_ref()).unwrap_or(U256::zero())
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
    .map(|a| a.encode_hex() )
    .map_err(|_| "could_not_recover_typed_data_from_sig".to_string())
}
