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
    // We may allow an asami account to change their address
    // this should not affect any existing campaigns.
    #[sqlx_model_hints(varchar)]
    advertiser_addr: String,
    #[sqlx_model_hints(campaign_kind)]
    campaign_kind: CampaignKind,
    #[sqlx_model_hints(campaign_status, default)]
    status: CampaignStatus,
    #[sqlx_model_hints(varchar)]
    briefing_hash: String,
    #[sqlx_model_hints(varchar)]
    briefing_json: String,
    #[sqlx_model_hints(varchar)]
    budget: String,
    #[sqlx_model_hints(timestamptz, default)]
    valid_until: Option<UtcDateTime>,
    #[sqlx_model_hints(varchar, default)]
    report_hash: Option<String>,
    #[sqlx_model_hints(timestamptz, default)]
    created_at: UtcDateTime,
  },
  queries {
      needing_report("valid_until IS NOT NULL AND valid_until < now() AND report_hash IS NULL"),
      needing_reimburse("valid_until IS NOT NULL AND valid_until < now() AND budget > to_u256(0)")
  },
  has_many {
    CampaignTopic(campaign_id),
    IgCampaignRule(campaign_id),
    Collab(campaign_id),
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
        XRepost,     // Members will have to make a post on X.
        IgClonePost, // Members must clone a post on IG.
    }
];

make_sql_enum![
    "campaign_status",
    pub enum CampaignStatus {
        Draft,     // First step of creation, just to provide users with a briefing hash.
        Submitted, // Campaign was apparently submitted on-chain by the user.
        Published, // Campaign has been seen on-chain.
    }
];

impl CampaignHub {
    pub async fn create_from_link(&self, account: &Account, link: &str, topics: &[Topic]) -> AsamiResult<Campaign> {
        use regex::Regex;
        use url::Url;

        let Some(advertiser_addr) = account.addr() else {
            return Err(Error::validation("account", "need_an_address_to_create_campaigns"));
        };

        let u = Url::parse(link).map_err(|_| Error::validation("link", "invalid_url"))?;

        let Some(path) = u.path_segments().map(|c| c.collect::<Vec<&str>>()) else {
            return Err(Error::validation("link", "no_segments"));
        };

        let Some(briefing) = path.get(path.len() - 1) else {
            return Err(Error::validation("link", "no_content_id"));
        };

        let Some(host) = u.host_str() else {
            return Err(Error::validation("link", "no_host"));
        };

        let x_regex = Regex::new(r#"^\d+$"#).map_err(|_| Error::precondition("invalid_x_regex"))?;
        let ig_regex = Regex::new(r#"^[\d\w\-_]+$"#).map_err(|_| Error::precondition("invalid_ig_regex"))?;

        let campaign_kind = if (host.ends_with("twitter.com") || host.ends_with("x.com")) && x_regex.is_match(&briefing)
        {
            CampaignKind::XRepost
        } else if host.ends_with("instagram.com") && ig_regex.is_match(&briefing) {
            CampaignKind::IgClonePost
        } else {
            return Err(Error::validation("link", "invalid_domain_or_route"));
        };

        let Ok(briefing_hash) = models::hasher::u256digest(briefing.as_bytes()) else {
            return Err(Error::precondition(
                "conversion_from_briefing_hash_to_u256_should_never_fail",
            ));
        };

        let briefing_json =
            serde_json::to_string(&briefing).map_err(|_| Error::precondition("briefing_is_always_json_encodeable"))?;

        let campaign = self
            .state
            .campaign()
            .insert(InsertCampaign {
                account_id: account.attrs.id.clone(),
                advertiser_addr: advertiser_addr.clone(),
                campaign_kind,
                budget: weihex("0"),
                briefing_hash: briefing_hash.encode_hex(),
                briefing_json,
            })
            .save()
            .await?;

        for t in topics {
            self.state
                .campaign_topic()
                .insert(InsertCampaignTopic {
                    campaign_id: campaign.attrs.id.clone(),
                    topic_id: t.attrs.id.clone(),
                })
                .save()
                .await?;
        }

        Ok(campaign)
    }

    pub async fn sync_x_collabs(&self) -> AsamiResult<Vec<Collab>> {
        use twitter_v2::{api_result::*, authorization::BearerToken, TwitterApi};

        let mut reqs = vec![];
        let conf = &self.state.settings.x;
        let auth = BearerToken::new(&conf.bearer_token);
        let api = TwitterApi::new(auth);

        for campaign in self.select().budget_gt(weihex("0")).campaign_kind_eq(CampaignKind::XRepost).all().await? {
            let post_id = campaign
                .attrs
                .briefing_json
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
                    if let Some(req) = campaign.make_x_collab_with_user_id(&user.id.to_string()).await? {
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
}

impl Campaign {
    pub async fn build_report(&self) -> anyhow::Result<serde_json::Value> {
        let collabs = self.collab_vec().await?;
        Ok(serde_json::json![{
            "collabs": collabs
        }])
    }

    pub async fn build_report_hash(&self) -> anyhow::Result<U256> {
        models::hasher::u256digest(&serde_json::to_vec(&self.build_report().await?)?)
    }

    pub async fn topic_ids(&self) -> sqlx::Result<Vec<i32>> {
        Ok(self.campaign_topic_vec().await?.into_iter().map(|t| t.attrs.topic_id).collect())
    }

    pub fn budget_u256(&self) -> U256 {
        u256(self.budget())
    }

    pub async fn available_budget(&self) -> anyhow::Result<U256> {
        let from_registered = self
            .state
            .collab()
            .registered()
            .all()
            .await?
            .iter()
            .map(|x| u256(x.reward()))
            .fold(U256::zero(), |acc, x| acc + x);
        Ok(self.budget_u256() - from_registered)
    }

    pub async fn make_collab(&self, handle: &Handle, reward: U256, trigger: &str) -> AsamiResult<Collab> {
        handle.validate_collaboration(self, reward, trigger).await?;

        Ok(self
            .state
            .collab()
            .insert(InsertCollab {
                campaign_id: self.attrs.id.clone(),
                handle_id: handle.attrs.id.clone(),
                advertiser_id: self.attrs.account_id.clone(),
                member_id: handle.account_id().clone(),
                collab_trigger_unique_id: trigger.to_string(),
                reward: reward.encode_hex(),
                fee: None,
            })
            .save()
            .await?)
    }

    pub async fn make_x_collab_with_user_id(&self, user_id: &str) -> AsamiResult<Option<Collab>> {
        let Some(handle) = self
            .state
            .handle()
            .select()
            .site_eq(Site::X)
            .user_id_eq(user_id.to_string())
            .order_by(HandleOrderBy::Id)
            .status_eq(HandleStatus::Active)
            .desc(true)
            .optional()
            .await?
        else {
            return Ok(None);
        };

        if *handle.status() != HandleStatus::Active {
            return Ok(None);
        }

        let Some(trigger) = handle.user_id().as_ref() else {
            return Ok(None);
        };

        let Some(reward) = handle.reward_for(self) else {
            return Ok(None);
        };

        match self.make_collab(&handle, reward, trigger).await {
            Ok(req) => Ok(Some(req)),
            Err(Error::Validation(_, _)) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub async fn is_missing_ig_rules(&self) -> AsamiResult<bool> {
        Ok(self.ig_campaign_rule_scope().count().await? == 0 && self.attrs.campaign_kind == CampaignKind::IgClonePost)
    }

    pub fn decoded_advertiser_addr(&self) -> AsamiResult<Address> {
        Address::decode_hex(self.advertiser_addr())
            .map_err(|_| Error::validation("invalid_address", self.advertiser_addr()))
    }

    pub async fn trusted_admin_addr(&self) -> AsamiResult<Address> {
        let address = self.decoded_advertiser_addr()?;
        // Position 0 is trusted admin
        Ok(self.state.on_chain.asami_contract.accounts(address).call().await?.0)
    }

    pub async fn we_are_trusted_admin(&self) -> AsamiResult<bool> {
        Ok(self.trusted_admin_addr().await? == self.state.settings.rsk.admin_address)
    }

    pub fn decoded_briefing_hash(&self) -> U256 {
        u256(self.briefing_hash())
    }

    pub fn content_id(&self) -> anyhow::Result<String> {
        serde_json::from_str::<serde_json::Value>(self.briefing_json())?
            .as_str()
            .map(|x| x.to_string())
            .ok_or_else(|| anyhow::anyhow!("no_content_id_in_briefing"))
    }

    pub async fn find_on_chain(&self) -> anyhow::Result<crate::on_chain::asami_contract_code::Campaign> {
        Ok(self
            .state
            .on_chain
            .asami_contract
            .get_campaign(self.decoded_advertiser_addr()?, self.decoded_briefing_hash())
            .call()
            .await?)
    }

    pub async fn mark_submitted(self) -> anyhow::Result<Self> {
        if *self.status() != CampaignStatus::Draft {
            return Ok(self);
        }
        Ok(self.update().status(CampaignStatus::Submitted).save().await?)
    }

    // Find out how much this campaign would pay an account.
    pub async fn reward_for_account(&self, account: &Account) -> AsamiResult<Option<U256>> {
        let site = match self.campaign_kind() {
            CampaignKind::IgClonePost => Site::Instagram,
            CampaignKind::XRepost => Site::X,
        };

        if let Some(handle) = account.handle_scope().site_eq(site).optional().await? {
            Ok(handle.reward_for(self))
        } else {
            Ok(None)
        }
    }
}
