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

  pub async fn validate_collaboration(&self, campaign: &Campaign) -> AsamiResult<()> {
    if self.price_score_ratio() > u256(campaign.price_score_ratio()) {
      return Err(Error::validation("price_score_ratio", "campaign_pays_too_little"));
    }

    if u256(self.price()) > u256(campaign.remaining()) {
      return Err(Error::validation("price", "campaign_funds_too_low"));
    }

    if *campaign.finished() {
      return Err(Error::validation("site", "campaign_was_finished"));
    }


    if self.site() != campaign.site() {
      return Err(Error::validation("site", "campaign_and_handle_sites_dont_match"));
    }

    let request_exists = self.state.collab_request().select()
      .handle_id_eq(self.attrs.id.clone())
      .campaign_id_eq(campaign.attrs.id.clone())
      .count().await? > 0;

    if request_exists {
      return Err(Error::validation("all", "collab_request_exists"));
    }

    let collab_exists = self.state.collab().select()
      .handle_id_eq(self.attrs.id.clone())
      .campaign_id_eq(campaign.attrs.id.clone())
      .count().await? > 0;

    if collab_exists {
      return Err(Error::validation("all", "collab_exists"));
    }

    Ok(())
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

