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

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize, sqlx::Type, GraphQLEnum)]
#[sqlx(type_name = "site", rename_all = "snake_case")]
pub enum Site {
  X,
  Nostr,
  Instagram,
}

impl Site {
  pub fn from_on_chain(o: u8) -> Self {
    match o {
      0 => Self::X,
      1 => Self::Nostr,
      2 => Self::Instagram,
      _ => panic!("mismatched site on contract")
    }
  }
}

impl sqlx::postgres::PgHasArrayType for Site {
  fn array_type_info() -> sqlx::postgres::PgTypeInfo {
    sqlx::postgres::PgTypeInfo::with_name("_site")
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize, sqlx::Type, GraphQLEnum)]
#[sqlx(type_name = "handle_update_request_status", rename_all = "snake_case")]
pub enum HandleUpdateRequestStatus {
  Received,
  Submitted,
  Done,
}

impl sqlx::postgres::PgHasArrayType for HandleUpdateRequestStatus {
  fn array_type_info() -> sqlx::postgres::PgTypeInfo {
    sqlx::postgres::PgTypeInfo::with_name("_handle_update_request_status")
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize, sqlx::Type, GraphQLEnum)]
#[sqlx(type_name = "claim_account_request_status", rename_all = "snake_case")]
pub enum ClaimAccountRequestStatus {
  Received,
  Submitted,
  Done,
}

impl sqlx::postgres::PgHasArrayType for ClaimAccountRequestStatus {
  fn array_type_info() -> sqlx::postgres::PgTypeInfo {
    sqlx::postgres::PgTypeInfo::with_name("_claim_account_request_status")
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "auth_method_kind", rename_all = "snake_case")]
pub enum AuthMethodKind {
  X,
  Google,
  Instagram,
  Nostr,
  Eip712,
  OneTimeToken,
}

impl AuthMethodKind {
  pub fn from_str(s: &str) -> Option<AuthMethodKind> {
    match s {
      "X" => Some(AuthMethodKind::X),
      "Google" => Some(AuthMethodKind::Google),
      "Instagram" => Some(AuthMethodKind::Instagram),
      "Nostr" => Some(AuthMethodKind::Nostr),
      "Eip712" => Some(AuthMethodKind::Eip712),
      "OneTimeToken" => Some(AuthMethodKind::OneTimeToken),
      _ => None
    }
  }
}

impl sqlx::postgres::PgHasArrayType for AuthMethodKind {
  fn array_type_info() -> sqlx::postgres::PgTypeInfo {
    sqlx::postgres::PgTypeInfo::with_name("_auth_method_kind")
  }
}

model!{
  state: App,
  table: handle_update_requests,
  struct HandleUpdateRequest {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(varchar)]
    account_id: String,
    #[sqlx_model_hints(varchar)]
    handle_id: String,
    #[sqlx_model_hints(varchar)]
    username: Option<String>,
    #[sqlx_model_hints(varchar)]
    price: Option<String>,
    #[sqlx_model_hints(varchar)]
    score: Option<String>,
    #[sqlx_model_hints(boolean)]
    created_by_admin: bool,
    #[sqlx_model_hints(handle_update_request_status, default)]
    status: HandleUpdateRequestStatus,
    #[sqlx_model_hints(varchar, default)]
    tx_hash: Option<String>,
  },
  has_many {
    HandleUpdateTopic(handle_update_request_id),
  }
}

impl HandleUpdateRequest {
  pub async fn done(self) -> sqlx::Result<Self> {
    self.update().status(HandleUpdateRequestStatus::Done).save().await
  }
}

model!{
  state: App,
  table: handle_update_request_topics,
  struct HandleUpdateTopic {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(int4)]
    handle_update_request_id: i32,
    #[sqlx_model_hints(varchar)]
    topic_id: String,
  }
}

model!{
  state: App,
  table: users,
  struct User {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    name: String,
    #[sqlx_model_hints(timestamptz, default)]
    created_at: UtcDateTime,
    #[sqlx_model_hints(timestamptz, default)]
    updated_at: Option<UtcDateTime>,
  },
  has_many {
    AccountUser(user_id)
  }
}

impl User {
  pub async fn account_id(&self) -> AsamiResult<String> {
    let accounts = self.account_user_vec().await?;
    let Some(first) = accounts.first() else {
       return Err(Error::Runtime(format!("user has no account {}", &self.attrs.id)))
    };

    Ok(first.attrs.account_id.clone())
  }
}

model!{
  state: App,
  table: auth_methods,
  struct AuthMethod {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    user_id: i32,
    #[sqlx_model_hints(auth_method_kind)]
    kind: AuthMethodKind,
    #[sqlx_model_hints(Varchar)]
    lookup_key: String,
  },
  belongs_to {
    User(user_id)
  }
}

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

model!{
  state: App,
  table: topics,
  struct Topic {
    #[sqlx_model_hints(varchar)]
    id: String,
    #[sqlx_model_hints(varchar)]
    name: String,
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

model!{
  state: App,
  table: claim_account_requests,
  struct ClaimAccountRequest {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(varchar)]
    account_id: String,
    #[sqlx_model_hints(varchar)]
    addr: String,
    #[sqlx_model_hints(varchar)]
    signature: String,
    #[sqlx_model_hints(varchar)]
    session_id: String,
    #[sqlx_model_hints(varchar, default)]
    tx_hash: Option<String>,
    #[sqlx_model_hints(claim_account_request_status, default)]
    status: ClaimAccountRequestStatus,
  },
  belongs_to {
    Account(account_id),
    Session(account_id),
  }
}

impl ClaimAccountRequestHub {
  pub async fn submit_all(&self) -> AsamiResult<Vec<ClaimAccountRequest>> {
    let mut submitted = vec![];
    let reqs = self.select().status_eq(ClaimAccountRequestStatus::Received).all().await?;

    if reqs.is_empty() { return Ok(submitted); }

    let params = reqs.iter().map(|r| r.as_param() ).collect();

    let tx_hash = self.state.on_chain.contract
      .claim_accounts(params)
      .send().await?.await?
      .ok_or_else(|| Error::service("rsk_blockchain", "no_tx_recepit_for_admin_make_handles"))?
      .transaction_hash
      .encode_hex();

    for req in reqs {
      submitted.push(
        req.update().status(ClaimAccountRequestStatus::Submitted).tx_hash(Some(tx_hash.clone())).save().await?
      );
    }

    Ok(submitted)
  }
}

impl ClaimAccountRequest {
  pub async fn done(self) -> sqlx::Result<Self> {
    self.update().status(ClaimAccountRequestStatus::Done).save().await
  }

  pub fn as_param(&self) -> on_chain::AdminClaimAccountsInput {
    on_chain::AdminClaimAccountsInput {
      account_id: u256(&self.attrs.account_id),
      addr: H160::decode_hex(&self.attrs.addr).unwrap(),
    }
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

fn utc_to_i(d: UtcDateTime) -> U256 {
  d.timestamp().into()
}

fn u64_to_d(u: U64) -> Decimal {
  Decimal::from_u64(u.as_u64()).unwrap_or(Decimal::ZERO)
}

fn d_to_u64(d: Decimal) -> U64 {
  U64::from_dec_str(&d.to_string()).unwrap_or(U64::zero())
}

pub fn eip_712_sig_to_address(chain_id: u64, signature: &str) -> Result<String, String> {
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

  let payload: TypedData = serde_json::from_value(json).map_err(|_| "unexpected_invalid_json".to_string())?;
  let sig = Signature::from_str(signature).map_err(|_| "invalid_auth_data_signature".to_string())?;
  sig.recover_typed_data(&payload)
    .map(|a| a.encode_hex() )
    .map_err(|_| "could_not_recover_typed_data_from_sig".to_string())
}
