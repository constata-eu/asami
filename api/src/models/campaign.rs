use super::*;

model!{
  state: App,
  table: campaigns,
  struct Campaign {
    #[sqlx_model_hints(varchar)]
    id: String,
    #[sqlx_model_hints(varchar)]
    account_id: String,
    #[sqlx_model_hints(site)]
    site: Site,
    #[sqlx_model_hints(varchar)]
    budget: String,
    #[sqlx_model_hints(varchar)]
    remaining: String,
    #[sqlx_model_hints(varchar)]
    content_id: String,
    #[sqlx_model_hints(varchar)]
    price_score_ratio: String,
    #[sqlx_model_hints(timestamptz)]
    valid_until: UtcDateTime,
    #[sqlx_model_hints(boolean, default)]
    finished: bool,
    #[sqlx_model_hints(boolean)]
    funded_by_admin: bool,
    #[sqlx_model_hints(timestamptz, default)]
    created_at: UtcDateTime,
    #[sqlx_model_hints(timestamptz, default)]
    updated_at: Option<UtcDateTime>,
    #[sqlx_model_hints(varchar)]
    tx_hash: String,
  },
  has_many {
    IgCampaignRule(campaign_id)
  }
}

impl CampaignHub {
  pub async fn sync_x_collabs(&self) -> AsamiResult<Vec<CollabRequest>> {
    use twitter_v2::{TwitterApi, authorization::BearerToken, api_result::*};

    let mut reqs = vec![];
    let conf = &self.state.settings.x;
    let auth = BearerToken::new(&conf.bearer_token);
    let api = TwitterApi::new(auth);
    
    for campaign in self.select().finished_eq(false).site_eq(Site::X).all().await? {
      let post_id = campaign.attrs.content_id.parse::<u64>()
        .map_err(|_| Error::Validation("content_id".into(), "was stored in the db not as u64".into()))?;

      let reposts = api.get_tweet_retweeted_by(post_id).send().await?;

      let mut page = Some(reposts);

      while let Some(reposts) = page {
        let payload = reposts.payload();
        let Some(data) = payload.data() else {
          break;
        };

        for user in data {
          let Some(handle) = self.state.handle()
            .select()
            .user_id_eq(&user.id.to_string())
            .optional().await? else { continue };

          match campaign.make_collab(&handle).await {
            Ok(req) => reqs.push(req),
            Err(Error::Validation(_, _)) => (),
            Err(e) => return Err(e),
          }
        }

        if data.len() < 100 {
          page = None;
        } else {
          page = reposts.next_page().await?;
          self.x_cooldown().await; // After fetching a page.
        }
      }

      self.x_cooldown().await; // Always between campaigns, even if reposts was None.
    }
    Ok(reqs)
  }

  async fn x_cooldown(&self) {
    tokio::time::sleep(tokio::time::Duration::from_millis(3 * 60 * 1000)).await;
  }
}

impl Campaign {
  pub async fn make_collab(&self, handle: &Handle) -> AsamiResult<CollabRequest> {
    handle.validate_collaboration(&self).await?;

    Ok(self.state.collab_request().insert(InsertCollabRequest{
      campaign_id: self.attrs.id.clone(),
      handle_id: handle.attrs.id.clone(),
    }).save().await?)
  }

  pub async fn is_missing_ig_rules(&self) -> AsamiResult<bool> {
    Ok(self.ig_campaign_rule_scope().count().await? == 0 && self.attrs.site == Site::Instagram)
  }
}

model!{
  state: App,
  table: campaign_topics,
  struct CampaignTopic {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(varchar)]
    campaign_id: String,
    #[sqlx_model_hints(varchar)]
    topic_id: String,
  }
}
