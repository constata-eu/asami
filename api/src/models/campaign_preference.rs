use super::*;

model! {
  state: App,
  table: campaign_preferences,
  struct CampaignPreference {
    #[sqlx_model_hints(int4, default, op_in)]
    id: i32,
    #[sqlx_model_hints(varchar)]
    account_id: String,
    #[sqlx_model_hints(int4)]
    campaign_id: i32,
    #[sqlx_model_hints(timestamptz, op_is_set)]
    not_interested_on: Option<UtcDateTime>,
    #[sqlx_model_hints(timestamptz)]
    attempted_on: Option<UtcDateTime>,
  },
  belongs_to {
      Account(account_id),
      Campaign(campaign_id),
  }
}
