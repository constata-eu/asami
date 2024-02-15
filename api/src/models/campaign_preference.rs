use super::*;

model! {
  state: App,
  table: campaign_preferences,
  struct CampaignPreference {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(varchar)]
    account_id: String,
    #[sqlx_model_hints(varchar)]
    campaign_id: String,
    #[sqlx_model_hints(timestamptz)]
    not_interested_on: Option<UtcDateTime>,
    #[sqlx_model_hints(timestamptz)]
    attempted_on: Option<UtcDateTime>,
  }
}
