/* Campaigns are created locally, then paid in the smart contract.
 * Campaigns could be paid on-chain without being reported in the contract, but it would not
 * have an effect in the front-end. These campaigns will be discarded.
 */
use super::*;

model! {
  state: App,
  table: campaigns,
  struct Campaign {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(varchar)]
    account_id: String,
    #[sqlx_model_hints(campaign_kind)]
    campaign_kind: CampaignKind,
    #[sqlx_model_hints(varchar)]
    briefing_hash: String,
    #[sqlx_model_hints(varchar)]
    briefing_json: String,
    #[sqlx_model_hints(varchar)]
    budget: String,
    #[sqlx_model_hints(timestamptz)]
    valid_until: UtcDateTime,
    #[sqlx_model_hints(varchar, default)]
    report_hash: String,
    #[sqlx_model_hints(timestamptz, default)]
    created_at: UtcDateTime,
  },
  has_many {
    CampaignTopic(campaign_id)
  },
  belongs_to {
    Account(account_id),
  }
}

model! {
  state: App,
  table: campaign_topics,
  struct CampaignTopic {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(int4)]
    campaign_id: i32,
    #[sqlx_model_hints(int4)]
    topic_id: i32,
  }
}

make_sql_enum![
  "campaign_kind",
  pub enum CampaignKind {
    XRepost,        // Members will have to make a post on X.
    IgClonePost,    // Members must clone a post on IG.
  }
];

impl CampaignHub {
  pub async fn create_from_link(&self, account: &Account, link: &str, valid_until: UtcDateTime, topics: &[Topic]) -> AsamiResult<Campaign> {
    use url::{Url, Host, Position};
    use regex::Regex;

    let u = Url::parse(link).map_err(|_| AsamiError::validation("link", "invalid_url"))?;
    let Some(path) = u.path_segments().map(|c| c.collect::<Vec<&str>>()) else {
      return Err(AsamiError::validation("link", "no_segments"));
    }
    let Some(briefing) = path.get(path.len() - 1) else {
      return Err(AsamiError::validation("link", "no_content_id"));
    }

    let x_regex = Regex::new(r#"^\d+$"#).map_err(|_| AsamiError::precondition("invalid_x_regex"))?;
    let ig_regex = Regex::new(r#"^[\d\w\-_]+$"#).map_err(|_| AsamiError::precondition("invalid_ig_regex"))?;

    let kind = if (u.host_str().ends_with("twitter.com") || u.host_str().ends_with("x.com")) && x_regex.match(&briefing) {
      CampaignKind::XRepost
    } else if u.host_str().ends_with("instagram.com") && ig_regex.match() {
      CampaignKind::IgClonePost
    } else {
      return Err(AsamiError::validation("link", "invalid_domain_in_link"));
    }

    /*
    try {
      const u = new URL(url);
      const path = u.pathname.replace(/\/$/, '').split("/");
      const contentId = path[path.length - 1];

      if ( (u.host.match(/\.?x\.com$/) || u.host.match(/\.?twitter\.com$/)) && contentId.match(/^\d+$/) ) {
        result.site = "X";
      } else if (u.host.match(/\.?instagram.com$/) && contentId.match(/^[\d\w\-_]+$/)) {
        result.site = "INSTAGRAM";
      } else {
        result.error = "not_a_post_url";
      }
      result.contentId = contentId;

    } catch {
      result.error = "invalid_url";
    }
    */

    let campaign = self
      .state
      .campaign()
      .insert(InsertCampaign {
        account_id: self.attrs.id.clone(),
        site,
        budget: budget.encode_hex(),
        content_id: content_id.to_string(),
        price_score_ratio: price_score_ratio.encode_hex(),
        valid_until,
      })
      .save()
      .await?;

    for t in topics {
      self
        .state
        .campaign_request_topic()
        .insert(InsertCampaignRequestTopic {
          campaign_request_id: campaign.attrs.id.clone(),
          topic_id: t.attrs.id.clone(),
        })
        .save()
        .await?;
    }

    Ok(campaign)
  }

  pub async fn sync_x_collabs(&self) -> AsamiResult<Vec<CollabRequest>> {
    use twitter_v2::{api_result::*, authorization::BearerToken, TwitterApi};

    let mut reqs = vec![];
    let conf = &self.state.settings.x;
    let auth = BearerToken::new(&conf.bearer_token);
    let api = TwitterApi::new(auth);

    for campaign in self.select().finished_eq(false).site_eq(Site::X).all().await? {
      let post_id = campaign
        .attrs
        .content_id
        .parse::<u64>()
        .map_err(|_| Error::Validation("content_id".into(), "was stored in the db not as u64".into()))?;

      let reposts = api.get_tweet_retweeted_by(post_id).send().await?;

      let mut page = Some(reposts);

      while let Some(reposts) = page {
        let payload = reposts.payload();
        let Some(data) = payload.data() else {
          break;
        };

        for user in data {
          if let Some(req) = self.try_x_collab_for_newest_handle(&campaign, &user.id.to_string()).await? {
            reqs.push(req);
          };
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

  pub async fn try_x_collab_for_newest_handle(
    &self,
    campaign: &Campaign,
    user_id: &String,
  ) -> AsamiResult<Option<CollabRequest>> {
    let Some(handle) = self
      .state
      .handle()
      .select()
      .site_eq(Site::X)
      .user_id_eq(user_id)
      .order_by(HandleOrderBy::Id)
      .desc(true)
      .optional()
      .await?
    else {
      return Ok(None);
    };

    match campaign.make_collab(&handle).await {
      Ok(req) => Ok(Some(req)),
      Err(Error::Validation(_, _)) => Ok(None),
      Err(e) => Err(e),
    }
  }
}

impl Campaign {
  pub async fn topic_ids(&self) -> sqlx::Result<Vec<String>> {
    Ok(self.campaign_topic_vec().await?.into_iter().map(|t| t.attrs.topic_id).collect())
  }

  pub async fn make_collab(&self, handle: &Handle) -> AsamiResult<CollabRequest> {
    handle.validate_collaboration(self).await?;

    Ok(
      self
        .state
        .collab()
        .insert(InsertCollab {
          campaign_id: self.attrs.id.clone(),
          handle_id: handle.attrs.id.clone(),
        })
        .save()
        .await?,
    )
  }

  pub async fn is_missing_ig_rules(&self) -> AsamiResult<bool> {
    Ok(self.ig_campaign_rule_scope().count().await? == 0 && self.attrs.site == Site::Instagram)
  }
}

