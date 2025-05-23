/* Campaigns are created locally, then paid in the smart contract.
 * Campaigns could be paid on-chain without being reported in the contract, but it would not
 * have an effect in the front-end. These campaigns will be discarded.
 */
use super::*;

model! {
  state: App,
  table: campaigns,
  struct Campaign {
    #[sqlx_model_hints(int4, default, op_in)]
    id: i32,
    #[sqlx_model_hints(varchar)]
    account_id: String,
    // We may allow an asami account to change their address
    // this should not affect any existing campaigns.
    #[sqlx_model_hints(varchar, op_like)]
    advertiser_addr: String,
    #[sqlx_model_hints(campaign_status, default, op_ne)]
    status: CampaignStatus,
    #[sqlx_model_hints(varchar, op_like)]
    briefing_hash: String,
    #[sqlx_model_hints(varchar, op_like)]
    briefing_json: String,
    #[sqlx_model_hints(varchar, op_lt, op_gt)]
    budget: String,
    #[sqlx_model_hints(varchar)]
    price_per_point: String,
    #[sqlx_model_hints(varchar)]
    max_individual_reward: String,
    #[sqlx_model_hints(varchar)]
    min_individual_reward: String,
    #[sqlx_model_hints(timestamptz, default)]
    valid_until: Option<UtcDateTime>,
    #[sqlx_model_hints(varchar, default)]
    report_hash: Option<String>,
    #[sqlx_model_hints(timestamptz, default)]
    created_at: UtcDateTime,

    // These columns are part of the campaign activity report
    // they are denormalized and re-hydrated when:
    // - A collab for the campaign is settled.
    // - The campaign budget changes.
    #[sqlx_model_hints(boolean, default)]
    force_hydrate: bool,
    #[sqlx_model_hints(int4, default)]
    total_collabs: i32,
    #[sqlx_model_hints(varchar, default)]
    total_spent: String,
    #[sqlx_model_hints(varchar, default)]
    total_budget: String,

    // These fields have X metrics for the campaign
    #[sqlx_model_hints(int4, default)]
    impression_count: i32,
    #[sqlx_model_hints(int4, default)]
    reply_count: i32,
    #[sqlx_model_hints(int4, default)]
    repost_count: i32,
    #[sqlx_model_hints(int4, default)]
    like_count: i32,
  },
  queries {
      needing_report("valid_until IS NOT NULL AND valid_until < now() AND report_hash IS NULL LIMIT 10"),
      needing_reimburse("valid_until IS NOT NULL AND valid_until < now() AND budget > to_u256(0)")
  },
  has_many {
    CampaignTopic(campaign_id),
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
    "campaign_status",
    pub enum CampaignStatus {
        Draft,     // First step of creation, just to provide users with a briefing hash.
        Submitted, // Campaign was apparently submitted on-chain by the user.
        Published, // Campaign has been seen on-chain.
    }
];

impl CampaignHub {
    pub async fn force_hydrate(&self) -> AsamiResult<()> {
        let ids = self
            .state
            .db
            .fetch_all_scalar(sqlx::query_scalar!(
                "SELECT id FROM campaigns WHERE force_hydrate = true LIMIT 50"
            ))
            .await?;
        if ids.is_empty() {
            return Ok(());
        }
        self.hydrate_report_columns_for(ids.iter().copied()).await?;
        self.state
            .db
            .execute(sqlx::query!(
                "UPDATE campaigns SET force_hydrate = false WHERE id = ANY($1)",
                &ids
            ))
            .await?;
        Ok(())
    }

    pub async fn hydrate_report_columns_for(&self, ids: impl Iterator<Item = i32>) -> AsamiResult<()> {
        for id in ids {
            let campaign = self.find(id).await?;
            let mut total_collabs = 0;
            let mut total_spent = u("0");
            for collab in campaign.collab_scope().status_eq(CollabStatus::Cleared).all().await? {
                total_collabs += 1;
                total_spent += collab.reward_u256();
            }

            let total_budget = campaign.budget_u256() + total_spent;
            campaign
                .update()
                .total_collabs(total_collabs)
                .total_spent(total_spent.encode_hex())
                .total_budget(total_budget.encode_hex())
                .save()
                .await?;
        }

        Ok(())
    }

    pub async fn create_from_link(
        &self,
        account: &Account,
        link: &str,
        topics: &[Topic],
        price_per_point: U256,
        max_individual_reward: U256,
        min_individual_reward: U256,
    ) -> AsamiResult<Campaign> {
        use regex::Regex;
        use url::Url;

        let Some(advertiser_addr) = account.addr() else {
            return Err(Error::validation("account", "need_an_address_to_create_campaigns"));
        };

        let u = Url::parse(link).map_err(|_| Error::validation("link", "invalid_url"))?;

        let Some(path) = u.path_segments().map(|c| c.collect::<Vec<&str>>()) else {
            return Err(Error::validation("link", "no_segments"));
        };

        let Some(briefing) = path.last() else {
            return Err(Error::validation("link", "no_content_id"));
        };

        let Some(host) = u.host_str() else {
            return Err(Error::validation("link", "no_host"));
        };

        let x_regex = Regex::new(r#"^\d+$"#).map_err(|_| Error::precondition("invalid_x_regex"))?;

        if !((host.ends_with("twitter.com") || host.ends_with("x.com")) && x_regex.is_match(briefing)) {
            return Err(Error::validation("link", "invalid_domain_or_route"));
        }

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
                budget: weihex("0"),
                briefing_hash: briefing_hash.encode_hex(),
                briefing_json,
                price_per_point: price_per_point.encode_hex(),
                max_individual_reward: max_individual_reward.encode_hex(),
                min_individual_reward: min_individual_reward.encode_hex(),
            })
            .save()
            .await?;

        for t in topics {
            self.state
                .campaign_topic()
                .insert(InsertCampaignTopic {
                    campaign_id: campaign.attrs.id,
                    topic_id: t.attrs.id,
                })
                .save()
                .await?;
        }

        Ok(campaign)
    }

    pub async fn sync_x_collabs(&self) -> anyhow::Result<Vec<Collab>> {
        use twitter_v2::{api_result::*, authorization::BearerToken, TwitterApi};

        let mut reqs = vec![];
        let conf = &self.state.settings.x;
        let auth = BearerToken::new(&conf.bearer_token);
        let api = TwitterApi::new(auth);

        let mut campaigns = self
            .select()
            .budget_gt(milli("50").encode_hex())
            .order_by(CampaignOrderBy::Id)
            .desc(true)
            .all()
            .await?;

        for campaign in campaigns.iter_mut() {
            let post_id = campaign
                .content_id()?
                .parse::<u64>()
                .map_err(|_| Error::Validation("content_id".into(), "was stored in the db not as u64".into()))?;

            self.state.info("sync_x_collabs", "fetching_retweets", &post_id).await;
            if let Some(metrics) =
                api.get_tweet(post_id).send().await?.data.as_ref().and_then(|o| o.public_metrics.as_ref())
            {
                campaign
                    .clone()
                    .update()
                    .reply_count(metrics.reply_count as i32)
                    .repost_count(metrics.retweet_count as i32)
                    .like_count(metrics.like_count as i32)
                    .save()
                    .await?;
            } else {
                self.state.info("sync_x_collabs", "no_organic_metrics_for", &post_id).await;
            }

            let reposts = api.get_tweet_retweeted_by(post_id).send().await?;
            self.state.info("sync_x_collabs", "got_reposts", ()).await;

            let mut page = Some(reposts);

            while let Some(reposts) = page {
                self.state.info("sync_x_collabs", "getting_payload", ()).await;
                let payload = reposts.payload();
                self.state.info("sync_x_collabs", "got_payload", ()).await;
                let Some(data) = payload.data() else {
                    self.state.info("sync_x_collabs", "no_payload_data", ()).await;
                    break;
                };

                for user in data {
                    self.state.info("sync_x_collabs", "processing_user", &user.id).await;
                    if let Some(req) = campaign.make_x_collab_with_user_id(&user.id.to_string()).await? {
                        reqs.push(req);
                    };
                }

                if data.len() < 100 {
                    page = None;
                } else {
                    self.state.info("sync_x_collabs", "fetching_next_page", ()).await;
                    page = reposts.next_page().await?;
                    self.state.info("sync_x_collabs", "got_next_page", ()).await;
                    self.x_cooldown().await; // After fetching a page.
                }
            }

            self.state.info("sync_x_collabs", "next_campaign", ()).await;
            self.x_cooldown().await; // Always between campaigns, even if reposts was None.
        }
        Ok(reqs)
    }

    async fn x_cooldown(&self) {
        // The basic plan allows 5 requests ever 15 minutes, that is, a 3 minute cooldown
        // between api calls.
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

    pub fn max_individual_reward_u256(&self) -> U256 {
        u256(self.max_individual_reward())
    }

    pub fn min_individual_reward_u256(&self) -> U256 {
        u256(self.min_individual_reward())
    }

    pub fn price_per_point_u256(&self) -> U256 {
        u256(self.price_per_point())
    }

    pub async fn available_budget(&self) -> anyhow::Result<U256> {
        let from_registered = self
            .state
            .collab()
            .registered(self.attrs.id)
            .all()
            .await?
            .iter()
            .map(|x| x.reward_u256())
            .fold(U256::zero(), |acc, x| acc + x);

        Ok(self.budget_u256().checked_sub(from_registered).unwrap_or(U256::zero()))
    }

    pub async fn make_collab(&mut self, handle: &Handle, reward: U256, trigger: &str) -> AsamiResult<Collab> {
        handle.validate_collaboration(self, reward, trigger).await?;

        let collab = self
            .state
            .collab()
            .insert(InsertCollab {
                campaign_id: self.attrs.id,
                handle_id: handle.attrs.id,
                advertiser_id: self.attrs.account_id.clone(),
                member_id: handle.account_id().clone(),
                collab_trigger_unique_id: trigger.to_string(),
                reward: reward.encode_hex(),
                fee: None,
                status: CollabStatus::Registered,
                dispute_reason: None,
            })
            .save()
            .await?;

        self.reload().await?;

        Ok(collab)
    }

    pub async fn make_failed_collab(
        &self,
        handle: &Handle,
        reward: U256,
        trigger: &str,
        reason: &str,
    ) -> AsamiResult<Collab> {
        Ok(self
            .state
            .collab()
            .insert(InsertCollab {
                campaign_id: self.attrs.id,
                handle_id: handle.attrs.id,
                advertiser_id: self.attrs.account_id.clone(),
                member_id: handle.account_id().clone(),
                collab_trigger_unique_id: trigger.to_string(),
                reward: reward.encode_hex(),
                fee: None,
                status: CollabStatus::Failed,
                dispute_reason: Some(reason.to_string()),
            })
            .save()
            .await?)
    }

    pub async fn make_x_collab_with_user_id(&mut self, user_id: &str) -> AsamiResult<Option<Collab>> {
        let Some(handle) = self
            .state
            .handle()
            .select()
            .user_id_eq(user_id.to_string())
            .order_by(HandleOrderBy::Id)
            .status_eq(HandleStatus::Active)
            .desc(true)
            .optional()
            .await?
        else {
            self.state.info("sync_x_collabs", "make_x_collab_no_handle", (self.id(), user_id)).await;
            return Ok(None);
        };

        let log_pointer = (self.id(), handle.id());

        if *handle.status() != HandleStatus::Active {
            self.state.info("sync_x_collabs", "make_x_collab_inactive_handle", log_pointer).await;
            return Ok(None);
        }

        let Some(trigger) = handle.user_id().as_ref() else {
            self.state.info("sync_x_collabs", "make_x_collab_no_trigger", log_pointer).await;
            return Ok(None);
        };

        let Some(reward) = handle.reward_for(self) else {
            self.state.info("sync_x_collabs", "make_x_collab_no_reward", log_pointer).await;
            return Ok(None);
        };

        match self.make_collab(&handle, reward, trigger).await {
            Ok(req) => Ok(Some(req)),
            Err(Error::Validation(_, reason)) => {
                self.state
                    .info(
                        "sync_x_collabs",
                        "make_x_collab_invalid",
                        &(self.id(), handle.id(), &reason),
                    )
                    .await;

                if reason != "collab_exists" {
                    let collab = self.make_failed_collab(&handle, reward, trigger, &reason).await?;
                    Ok(Some(collab))
                } else {
                    Ok(None)
                }
            }
            Err(e) => {
                self.state
                    .info(
                        "sync_x_collabs",
                        "make_x_collab_error",
                        &(self.id(), handle.id(), e.to_string()),
                    )
                    .await;
                Err(e)
            }
        }
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
        if let Some(handle) = account.handle_scope().optional().await? {
            Ok(handle.reward_for(self))
        } else {
            Ok(None)
        }
    }
}
