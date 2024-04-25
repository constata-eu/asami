use super::*;

model! {
  state: App,
  table: collabs,
  struct Collab {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(int4)]
    campaign_id: i32,
    #[sqlx_model_hints(varchar)]
    advertiser_id: String,
    #[sqlx_model_hints(varchar)]
    member_id: String,
    #[sqlx_model_hints(varchar)]
    collab_trigger_unique_id: String,
    #[sqlx_model_hints(int4)]
    handle_id: i32,
    #[sqlx_model_hints(collab_status, default)]
    status: CollabStatus,
    #[sqlx_model_hints(varchar, default)]
    dispute_reason: String,
    #[sqlx_model_hints(varchar)]
    reward: String,
    #[sqlx_model_hints(varchar)]
    fee: Option<String>,
    #[sqlx_model_hints(timestamptz, default)]
    created_at: UtcDateTime,
  }
}

make_sql_enum![
  "collab_status",
  pub enum CollabStatus {
    Registered,     // The collab is registered and will be paid out.
    Cleared,        // The collab has been paid.
    Failed,         // The collab was registered but will not be paid.
    Disputed,       // An incomplete intention to collab was observed.
  }
];
