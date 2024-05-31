use super::*;

model! {
  state: App,
  table: holders,
  struct Holder {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(varchar)]
    address: String,
    #[sqlx_model_hints(varchar)]
    balance: String,
    #[sqlx_model_hints(varchar, default)]
    last_fee_pool_share: Option<String>,
  }
}
