use super::*;
pub use sqlx::{self, types::Decimal};
pub use serde::{Serialize, Deserialize};
use sqlx_models_orm::model;
pub use chrono::{DateTime, Duration, Utc, Datelike, TimeZone};
pub type UtcDateTime = DateTime<Utc>;
pub use juniper::GraphQLEnum;

pub mod hasher;

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize, sqlx::Type, GraphQLEnum)]
#[sqlx(type_name = "site", rename_all = "snake_case")]
pub enum Site {
  X,
  Instagram,
  Nostr,
}

impl sqlx::postgres::PgHasArrayType for Site {
  fn array_type_info() -> sqlx::postgres::PgTypeInfo {
    sqlx::postgres::PgTypeInfo::with_name("_site")
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "collab_status", rename_all = "snake_case")]
pub enum CollabStatus {
  Unseen,
  Seen,
  Reverted,
}

impl sqlx::postgres::PgHasArrayType for CollabStatus {
  fn array_type_info() -> sqlx::postgres::PgTypeInfo {
    sqlx::postgres::PgTypeInfo::with_name("_collab_status")
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "auth_method_kind", rename_all = "snake_case")]
pub enum AuthMethodKind {
  X,
  Google,
  Facebook,
  Nostr,
  BitcoinSignedMessage,
  EthereumSignedMessage,
  OneTimeToken,
}

impl AuthMethodKind {
  pub fn from_str(s: &str) -> Option<AuthMethodKind> {
    match s {
      "X" => Some(AuthMethodKind::X),
      "Google" => Some(AuthMethodKind::Google),
      "Facebook" => Some(AuthMethodKind::Facebook),
      "Nostr" => Some(AuthMethodKind::Nostr),
      "BitcoinSignedMessage" => Some(AuthMethodKind::BitcoinSignedMessage),
      "EthereumSignedMessage" => Some(AuthMethodKind::EthereumSignedMessage),
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

// This is an account profile when taking the Collaborator role.
model!{
  state: App,
  table: accounts,
  struct Account {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    name: String,
    #[sqlx_model_hints(timestamptz, default)]
    created_at: UtcDateTime,
    #[sqlx_model_hints(timestamptz, default)]
    updated_at: Option<UtcDateTime>,
  }
}

// This is an account profile when taking the Collaborator role.
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
  pub async fn account_ids(&self) -> sqlx::Result<Vec<i32>> {
    Ok(self.account_user_vec().await?.into_iter().map(|x| x.attrs.account_id).collect())
  }
}

model!{
  state: App,
  table: sessions,
  struct Session {
    #[sqlx_model_hints(varchar)]
    id: String,
    #[sqlx_model_hints(int4)]
    user_id: i32,
    #[sqlx_model_hints(int4)]
    auth_method_id: i32,
    #[sqlx_model_hints(varchar)]
    pubkey: String,
    #[sqlx_model_hints(bigint)]
    nonce: i64,
    #[sqlx_model_hints(timestamptz, default)]
    created_at: UtcDateTime,
    #[sqlx_model_hints(timestamptz, default)]
    updated_at: Option<UtcDateTime>,
    #[sqlx_model_hints(int4, default)]
    deletion_id: Option<i32>,
  },
  belongs_to {
    User(user_id),
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
  }
}

model!{
  state: App,
  table: account_users,
  struct AccountUser {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(int4)]
    account_id: i32,
    #[sqlx_model_hints(int4)]
    user_id: i32,
  }
}

model!{
  state: App,
  table: campaigns,
  struct Campaign {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(int4)]
    account_id: i32,
    #[sqlx_model_hints(decimal)]
    budget: Decimal,
    #[sqlx_model_hints(site)]
    site: Site,
    #[sqlx_model_hints(varchar)]
    content: String,
    #[sqlx_model_hints(timestamptz, default)]
    created_at: UtcDateTime,
    #[sqlx_model_hints(timestamptz, default)]
    updated_at: Option<UtcDateTime>,
    #[sqlx_model_hints(int4, default)]
    deletion_id: Option<i32>,
  }
}

model!{
  state: App,
  table: collabs,
  struct Collab {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    campaign_id: i32,
    handle_id: i32, 
    status: CollabStatus,
  }
}

model!{
  state: App,
  table: campaign_topics,
  struct CampaignTopic {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    campaign_id: i32,
    topic_id: i32,
  }
}

model!{
  state: App,
  table: topics,
  struct Topic {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    name: i32,
  }
}

model!{
  state: App,
  table: handles,
  struct Handle {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    account_id: i32,
    value: String,
    price: Decimal,
    site: Site,
  }
}

model!{
  state: App,
  table: handle_topics,
  struct HandleTopic {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    handle_id: i32,
    topic_id: i32,
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
