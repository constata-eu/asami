use juniper::GraphQLInputObject;

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

    // The budget is the DOC budget available on-chain
    // for this campaign.
    #[sqlx_model_hints(varchar, op_lt, op_gt)]
    budget: String,
    #[sqlx_model_hints(campaign_kind, op_in)]
    kind: CampaignKind,
    #[sqlx_model_hints(varchar)]
    max_individual_reward: String,
    #[sqlx_model_hints(boolean)]
    thumbs_up_only: bool,
    #[sqlx_model_hints(timestamptz, default)]
    valid_until: Option<UtcDateTime>,
    #[sqlx_model_hints(timestamptz, default, op_is_set)]
    collabs_gathered_at: Option<UtcDateTime>,
    #[sqlx_model_hints(timestamptz, default, op_is_set)]
    rewards_assigned_at: Option<UtcDateTime>,
    #[sqlx_model_hints(varchar, default)]
    report_hash: Option<String>,
    #[sqlx_model_hints(timestamptz, default, op_gt)]
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

    #[sqlx_model_hints(varchar, default, op_is_set)]
    x_announcement_id_es: Option<String>,
    #[sqlx_model_hints(varchar, default, op_is_set)]
    x_announcement_id_en: Option<String>,
  },
  queries {
      needing_report("valid_until IS NOT NULL AND valid_until < now() AND report_hash IS NULL LIMIT 20"),
      needing_reimburse("valid_until IS NOT NULL AND valid_until < now() AND budget > to_u256(0)")
  },
  has_many {
    CampaignTopic(campaign_id),
    Collab(campaign_id),
    CampaignPreference(campaign_id),
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
        Draft,
        Submitted,       // Campaign was apparently submitted on-chain by the user.
        Published,       // Campaign has been seen on-chain.
        Failed,          // Something went wrong, a refund may or may not be needed.
    }
];

make_sql_enum![
    "campaign_kind",
    pub enum CampaignKind {
        XRepost,
        CommunityGiveaway,
    }
];

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum CampaignBriefing {
    XRepost {
        tweet_id: String,
    },
    CommunityGiveaway {
        all_members: bool,
        member_ids: Vec<String>,
    },
}

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

    pub async fn prepare_collabs(&self) -> anyhow::Result<Vec<Collab>> {
        let mut collabs = vec![];
        collabs.extend(self.gather_community_giveaway_collabs().await?);
        collabs.extend(self.gather_x_repost_collabs().await?);
        collabs.extend(self.assign_pending_rewards().await?);
        Ok(collabs)
    }

    async fn gather_community_giveaway_collabs(&self) -> anyhow::Result<Vec<Collab>> {
        let mut created = vec![];
        let mut campaigns = self
            .select()
            .status_eq(CampaignStatus::Published)
            .kind_eq(CampaignKind::CommunityGiveaway)
            .collabs_gathered_at_is_set(false)
            .all()
            .await?;

        for campaign in campaigns.iter_mut() {
            created.extend(campaign.gather_community_giveaway_collabs().await?);
        }

        Ok(created)
    }

    async fn gather_x_repost_collabs(&self) -> anyhow::Result<Vec<Collab>> {
        let mut created = vec![];
        let now = Utc::now();
        let mut campaigns = self
            .select()
            .status_eq(CampaignStatus::Published)
            .kind_eq(CampaignKind::XRepost)
            .collabs_gathered_at_is_set(false)
            .all()
            .await?;

        for campaign in campaigns.iter_mut() {
            let Some(valid_until) = campaign.valid_until().clone() else {
                continue;
            };

            if now < campaign.repost_registration_opens_at()? {
                continue;
            }

            if valid_until <= now {
                campaign
                    .clone()
                    .update()
                    .collabs_gathered_at(Some(now))
                    .save()
                    .await?;
                continue;
            }

            created.extend(campaign.gather_x_repost_collabs().await?);
        }

        Ok(created)
    }

    async fn assign_pending_rewards(&self) -> anyhow::Result<Vec<Collab>> {
        let mut updated = vec![];
        let mut campaigns = self
            .select()
            .status_eq(CampaignStatus::Published)
            .collabs_gathered_at_is_set(true)
            .rewards_assigned_at_is_set(false)
            .all()
            .await?;

        for campaign in campaigns.iter_mut() {
            updated.extend(campaign.assign_rewards_to_pending_collabs().await?);
        }

        Ok(updated)
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

    async fn mark_collabs_gathered_at(&mut self, when: UtcDateTime) -> AsamiResult<()> {
        self.clone()
            .update()
            .collabs_gathered_at(Some(when))
            .save()
            .await?;
        Ok(())
    }

    async fn mark_rewards_assigned_at(&mut self, when: UtcDateTime) -> AsamiResult<()> {
        self.clone()
            .update()
            .rewards_assigned_at(Some(when))
            .save()
            .await?;
        Ok(())
    }

    pub fn max_individual_reward_u256(&self) -> U256 {
        u256(self.max_individual_reward())
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

    async fn create_pending_collab_for_account(
        &mut self,
        account: &Account,
        handle: Option<&Handle>,
        trigger: &str,
    ) -> AsamiResult<Option<Collab>> {
        if self
            .state
            .collab()
            .select()
            .campaign_id_eq(self.attrs.id)
            .collab_trigger_unique_id_eq(trigger.to_string())
            .count()
            .await?
            > 0
        {
            return Ok(None);
        }

        let collab = self
            .state
            .collab()
            .insert(InsertCollab {
                campaign_id: self.attrs.id,
                handle_id: handle.map(|h| *h.id()),
                advertiser_id: self.attrs.account_id.clone(),
                member_id: account.id().clone(),
                collab_trigger_unique_id: trigger.to_string(),
                reward: weihex("0"),
                fee: None,
                status: CollabStatus::Pending,
                dispute_reason: None,
            })
            .save()
            .await?;

        self.state.community_member().create_or_update_from_collab(&collab).await?;
        self.reload().await?;
        Ok(Some(collab))
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

    pub fn briefing(&self) -> AsamiResult<CampaignBriefing> {
        serde_json::from_str::<CampaignBriefing>(self.briefing_json())
            .map_err(|_| Error::precondition(&format!("parse_error_in_briefing_json: {}", self.id())))
    }

    pub fn x_post_id(&self) -> AsamiResult<String> {
        match self.briefing()? {
            CampaignBriefing::XRepost { tweet_id } => Ok(tweet_id),
            CampaignBriefing::CommunityGiveaway { .. } => {
                Err(Error::validation("campaign", "campaign_kind_has_no_x_post"))
            }
        }
    }

    pub fn repost_registration_opens_at(&self) -> AsamiResult<UtcDateTime> {
        self.valid_until()
            .map(|valid_until| valid_until - Duration::days(5))
            .ok_or(Error::precondition("x_repost_campaign_has_no_valid_until"))
    }

    pub fn is_visible_to_members(&self) -> bool {
        self.kind() == &CampaignKind::XRepost
            && self.status() == &CampaignStatus::Published
            && self.collabs_gathered_at().is_none()
            && self
                .valid_until()
                .map(|valid_until| {
                    valid_until > (Utc::now() + Duration::days(5))
                        && valid_until <= (Utc::now() + Duration::days(15))
                })
                .unwrap_or(false)
    }

    pub async fn giveaway_target_account_ids(&self) -> AsamiResult<Vec<String>> {
        let community_member_ids: std::collections::HashSet<String> = self
            .state
            .community_member()
            .select()
            .account_id_eq(self.account_id())
            .all()
            .await?
            .into_iter()
            .map(|member| member.attrs.member_id)
            .collect();

        let mut target_ids = match self.briefing()? {
            CampaignBriefing::CommunityGiveaway { all_members, member_ids: _ } if all_members => {
                community_member_ids.into_iter().collect::<Vec<_>>()
            }
            CampaignBriefing::CommunityGiveaway { member_ids, .. } => member_ids
                .into_iter()
                .filter(|member_id| community_member_ids.contains(member_id))
                .collect::<Vec<_>>(),
            CampaignBriefing::XRepost { .. } => {
                return Err(Error::validation("campaign", "campaign_kind_has_no_giveaway_targets"))
            }
        };

        target_ids.sort();
        target_ids.dedup();

        Ok(self
            .state
            .account()
            .select()
            .id_in(target_ids)
            .acceptance_status_eq(AcceptanceStatus::Active)
            .all()
            .await?
            .into_iter()
            .map(|account| account.attrs.id)
            .collect())
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

    pub async fn mark_submitted(self) -> AsamiResult<Self> {
        if *self.status() != CampaignStatus::Draft {
            return Ok(self);
        }

        Ok(self.update().status(CampaignStatus::Submitted).save().await?)
    }

    // Find out how much this campaign would pay an account.
    pub async fn reward_for_account(&self, account: &Account) -> AsamiResult<Option<U256>> {
        if self.kind() != &CampaignKind::XRepost || !self.is_visible_to_members() {
            return Ok(None);
        }

        let Some(handle) = account.handle_scope().optional().await? else {
            return Ok(None);
        };

        if handle.validate_collaboration(self, handle.user_id()).await.is_ok() {
            Ok(Some(self.max_individual_reward_u256()))
        } else {
            Ok(None)
        }
    }

    pub async fn gather_community_giveaway_collabs(&mut self) -> AsamiResult<Vec<Collab>> {
        let target_ids = self.giveaway_target_account_ids().await?;
        if target_ids.is_empty() {
            self.clone()
                .update()
                .collabs_gathered_at(Some(Utc::now()))
                .save()
                .await?;
            return Ok(vec![]);
        }

        let accounts = self.state.account().select().id_in(target_ids).all().await?;
        let mut collabs = vec![];

        for account in accounts {
            let handle = account.handle_scope().status_eq(HandleStatus::Active).optional().await?;
            let trigger = format!("community_giveaway:{}:{}", self.id(), account.id());
            if let Some(collab) = self
                .create_pending_collab_for_account(&account, handle.as_ref(), &trigger)
                .await?
            {
                collabs.push(collab);
            }
        }

        self.clone()
            .update()
            .collabs_gathered_at(Some(Utc::now()))
            .save()
            .await?;
        Ok(collabs)
    }

    pub async fn gather_x_repost_collabs(&mut self) -> AsamiResult<Vec<Collab>> {
        let tweet_id = self.x_post_id()?;
        let preferences = self
            .campaign_preference_scope()
            .attempted_on_is_set(true)
            .all()
            .await?;

        let mut verified = vec![];
        for preference in preferences {
            let Some(account) = self
                .state
                .account()
                .select()
                .id_eq(preference.account_id())
                .acceptance_status_eq(AcceptanceStatus::Active)
                .optional()
                .await?
            else {
                continue;
            };

            let Some(handle) = account.handle_scope().status_eq(HandleStatus::Active).optional().await? else {
                continue;
            };

            if handle.has_reposted_x_campaign(&tweet_id).await.unwrap_or(false) {
                verified.push((account, handle));
            }
        }

        let mut collabs = vec![];
        for (account, handle) in verified {
            if let Some(collab) = self
                .create_pending_collab_for_account(&account, Some(&handle), handle.user_id())
                .await?
            {
                collabs.push(collab);
            }
        }

        self.mark_collabs_gathered_at(Utc::now()).await?;
        Ok(collabs)
    }

    pub async fn assign_rewards_to_pending_collabs(&mut self) -> AsamiResult<Vec<Collab>> {
        let pending = self
            .collab_scope()
            .status_eq(CollabStatus::Pending)
            .all()
            .await?;

        if pending.is_empty() {
            self.mark_rewards_assigned_at(Utc::now()).await?;
            return Ok(vec![]);
        }

        let member_ids = pending
            .iter()
            .map(|collab| collab.member_id().clone())
            .collect::<Vec<_>>();
        let scores = self
            .state
            .community_member()
            .creator_scores_for_targets(self.account_id(), &member_ids)
            .await?;
        let rewards = prorated_rewards(
            self.budget_u256(),
            pending
                .iter()
                .map(|collab| {
                    let weight = scores
                        .get(collab.member_id())
                        .map(|score| ((score.total_score() * 1000.0).round() as u64).max(1))
                        .unwrap_or(30_000);
                    (collab.member_id().clone(), weight)
                })
                .collect(),
            Some(self.max_individual_reward_u256()),
        );

        let mut updated = vec![];
        for collab in pending {
            let reward = rewards
                .get(collab.member_id())
                .copied()
                .unwrap_or_else(U256::zero);

            let saved = collab
                .update()
                .reward(reward.encode_hex())
                .status(if reward.is_zero() {
                    CollabStatus::Failed
                } else {
                    CollabStatus::Registered
                })
                .save()
                .await?;
            updated.push(saved);
        }

        self.mark_rewards_assigned_at(Utc::now()).await?;
        self.reload().await?;
        Ok(updated)
    }
}

#[derive(Clone, GraphQLInputObject, Serialize, Deserialize)]
#[graphql(description = "The input for creating a new Campaign.")]
#[serde(rename_all = "camelCase")]
pub struct CreateCampaignFromLinkInput {
    pub kind: CampaignKind,
    pub link: Option<String>,
    pub topic_ids: Vec<i32>,
    pub max_individual_reward: String,
    pub thumbs_up_only: bool,
    pub all_members: bool,
    pub member_ids: Vec<i32>,
}

impl CreateCampaignFromLinkInput {
    pub async fn process(self, app: &App, account: &Account) -> AsamiResult<Campaign> {
        let topics = app.topic().select().id_in(&self.topic_ids).all().await?;
        let advertiser_addr =
            account.addr().clone().ok_or(Error::validation("account", "need_an_address_to_create_campaigns"))?;

        let briefing = self.build_briefing()?;

        let briefing_json =
            serde_json::to_string(&briefing).map_err(|_| Error::precondition("briefing_is_always_json_encodeable"))?;

        let Ok(briefing_hash) = models::hasher::u256digest(briefing_json.as_bytes()) else {
            return Err(Error::precondition(
                "conversion_from_briefing_hash_to_u256_should_never_fail",
            ));
        };

        if self.kind == CampaignKind::XRepost
            && app.campaign().select().briefing_hash_eq(briefing_hash.encode_hex()).count().await? > 0
        {
            return Err(Error::validation("campaign", "campaign_already_exists"));
        }

        let campaign = app
            .campaign()
            .insert(InsertCampaign {
                account_id: account.attrs.id.clone(),
                advertiser_addr,
                budget: weihex("0"),
                kind: self.kind,
                briefing_hash: briefing_hash.encode_hex(),
                briefing_json,
                max_individual_reward: parse_u256("max_individual_reward", &self.max_individual_reward)?.encode_hex(),
                thumbs_up_only: self.thumbs_up_only,
            })
            .save()
            .await?;

        for t in topics {
            app.campaign_topic()
                .insert(InsertCampaignTopic {
                    campaign_id: campaign.attrs.id,
                    topic_id: t.attrs.id,
                })
                .save()
                .await?;
        }

        Ok(campaign)
    }

    fn build_briefing(&self) -> AsamiResult<CampaignBriefing> {
        match self.kind {
            CampaignKind::XRepost => Ok(CampaignBriefing::XRepost {
                tweet_id: Self::validate_x_link_and_get_tweet_id(
                    self.link
                        .as_deref()
                        .ok_or(Error::validation("link", "campaign_link_required"))?,
                )?,
            }),
            CampaignKind::CommunityGiveaway => Ok(CampaignBriefing::CommunityGiveaway {
                all_members: self.all_members,
                member_ids: if self.all_members {
                    vec![]
                } else if self.member_ids.is_empty() {
                    return Err(Error::validation("member_ids", "at_least_one_member_required"));
                } else {
                    self.member_ids.iter().map(|id| i32_to_hex(*id)).collect()
                },
            }),
        }
    }

    pub fn validate_x_link_and_get_tweet_id(link: &str) -> AsamiResult<String> {
        use regex::Regex;
        use url::Url;
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

        Ok(briefing.to_string())
    }
}

fn parse_u256(fieldname: &str, value: &str) -> AsamiResult<U256> {
    use ethers::abi::AbiDecode;
    U256::decode_hex(value).map_err(|_e| Error::validation(fieldname, "invalid_hex_encoded_u256_value"))
}

fn prorated_rewards(
    budget: U256,
    weights: Vec<(String, u64)>,
    cap: Option<U256>,
) -> std::collections::HashMap<String, U256> {
    let mut rewards = std::collections::HashMap::new();
    if budget.is_zero() || weights.is_empty() {
        return rewards;
    }

    let total_weight: u64 = weights.iter().map(|(_, weight)| *weight).sum();
    if total_weight == 0 {
        return rewards;
    }

    let mut spent = U256::zero();
    let last_index = weights.len().saturating_sub(1);

    for (index, (account_id, weight)) in weights.into_iter().enumerate() {
        let raw_reward = if index == last_index {
            budget.checked_sub(spent).unwrap_or_else(U256::zero)
        } else {
            (budget * U256::from(weight)) / U256::from(total_weight)
        };

        let reward = cap.map(|max| raw_reward.min(max)).unwrap_or(raw_reward);
        spent += reward;
        rewards.insert(account_id, reward);
    }

    rewards
}
