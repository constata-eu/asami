use super::*;

model! {
  state: App,
  table: sessions,
  struct Session {
    #[sqlx_model_hints(varchar, op_in)]
    id: String,
    #[sqlx_model_hints(int4)]
    user_id: i32,
    #[sqlx_model_hints(varchar)]
    account_id: String,
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
    #[sqlx_model_hints(boolean)]
    admin: bool
  },
  belongs_to {
    Account(account_id),
    User(user_id),
  }
}
