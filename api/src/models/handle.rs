use super::*;

model!{
  state: App,
  table: handles,
  struct Handle {
    #[sqlx_model_hints(varchar)]
    id: String,
    #[sqlx_model_hints(varchar)]
    account_id: String,
    #[sqlx_model_hints(site)]
    site: Site,
    #[sqlx_model_hints(varchar)]
    username: String,
    #[sqlx_model_hints(varchar)]
    user_id: String,
    #[sqlx_model_hints(varchar)]
    price: String,
    #[sqlx_model_hints(varchar)]
    score: String,
    #[sqlx_model_hints(varchar)]
    nostr_affine_x: String,
    #[sqlx_model_hints(varchar)]
    nostr_affine_y: String,
  },
  has_many {
    HandleTopic(handle_id),
  }
}

impl Handle {
  pub fn price_score_ratio(&self) -> U256 {
    u256(self.price()) / u256(self.score())
  }

  pub fn can_collaborate(&self, campaign: &Campaign) -> bool {
    self.price_score_ratio() <= u256(campaign.price_score_ratio())
      && u256(self.price()) < u256(campaign.remaining())
      && self.site() == campaign.site()
  }
}

model!{
  state: App,
  table: handle_topics,
  struct HandleTopic {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(varchar)]
    handle_id: String,
    #[sqlx_model_hints(varchar)]
    topic_id: String,
  }
}

