use super::*;

model! {
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
