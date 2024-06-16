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
    #[sqlx_model_hints(collab_status)]
    status: CollabStatus,
    #[sqlx_model_hints(varchar)]
    dispute_reason: Option<String>,
    #[sqlx_model_hints(varchar)]
    reward: String,
    #[sqlx_model_hints(varchar)]
    fee: Option<String>,
    #[sqlx_model_hints(timestamptz, default)]
    created_at: UtcDateTime,
  },
  queries {
      registered("status = 'registered' AND campaign_id = $1", campaign_id: i32)
  },
  belongs_to {
    Account(member_id),
    Campaign(campaign_id),
  }
}

impl Collab {
    pub fn reward_u256(&self) -> U256 {
        u256(self.reward())
    }

    pub fn fee_u256(&self) -> Option<U256> {
        self.fee().as_ref().map(u256)
    }
}

impl_loggable!(Collab);

make_sql_enum![
    "collab_status",
    pub enum CollabStatus {
        Registered, // The collab is registered and will be paid out.
        Cleared,    // The collab was paid correctly.
        Failed,     // The collab was registered but will not be paid.
    }
];
