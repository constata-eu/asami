use rocket::serde::json;
use twitter_v2::{authorization::Oauth2Token, oauth2::*, TwitterApi};

use super::*;

model! {
  state: App,
  table: handles,
  struct Handle {
    #[sqlx_model_hints(int4, default, op_in)]
    id: i32,
    #[sqlx_model_hints(varchar)]
    account_id: String,
    #[sqlx_model_hints(varchar, op_like, op_ilike)]
    username: String,
    #[sqlx_model_hints(varchar, op_like)]
    user_id: String,
    #[sqlx_model_hints(handle_status, op_in)]
    status: HandleStatus,

    #[sqlx_model_hints(timestamptz, default, op_gt)]
    created_at: DateTime<Utc>,

    // These columns are part of the account activity report
    // they are denormalized and re-hydrated when:
    // - A collab for one of the user handles user is settled.
    #[sqlx_model_hints(boolean, default)]
    force_hydrate: bool,
    #[sqlx_model_hints(int4, default)]
    total_collabs: i32,
    #[sqlx_model_hints(varchar, default)]
    total_collab_rewards: String,

    #[sqlx_model_hints(text, op_is_set)]
    x_refresh_token: Option<String>,
    #[sqlx_model_hints(text, default, op_is_set)]
    invalidated_x_refresh_token: Option<String>,

    #[sqlx_model_hints(timestamptz, default)]
    next_scoring: Option<UtcDateTime>,
    #[sqlx_model_hints(varchar, default)]
    score: Option<String>,
    #[sqlx_model_hints(timestamptz, default)]
    last_scoring: Option<UtcDateTime>,
    #[sqlx_model_hints(int4, default)]
    current_scoring_id: Option<i32>,

    // These are overrides for all the areas of the algorithm
    // where issues may arise.
    #[sqlx_model_hints(engagement_score, default)]
    online_engagement_override: Option<EngagementScore>,
    #[sqlx_model_hints(text, default)]
    online_engagement_override_reason: Option<String>,

    #[sqlx_model_hints(engagement_score, default)]
    offline_engagement_score: EngagementScore,
    #[sqlx_model_hints(text, default)]
    offline_engagement_description: Option<String>,

    #[sqlx_model_hints(varchar, default)]
    poll_id: Option<String>,
    #[sqlx_model_hints(timestamptz, default)]
    poll_ends_at: Option<DateTime<Utc>>,
    #[sqlx_model_hints(poll_score, default)]
    poll_override: Option<PollScore>,
    #[sqlx_model_hints(varchar, default)]
    poll_override_reason: Option<String>,

    #[sqlx_model_hints(operational_status, default)]
    operational_status_override: Option<OperationalStatus>,
    #[sqlx_model_hints(text, default)]
    operational_status_override_reason: Option<String>,

    #[sqlx_model_hints(boolean, default)]
    referrer_score_override: Option<bool>,
    #[sqlx_model_hints(text, default)]
    referrer_score_override_reason: Option<String>,

    #[sqlx_model_hints(boolean, default)]
    holder_score_override: Option<bool>,
    #[sqlx_model_hints(text, default)]
    holder_score_override_reason: Option<String>,

    #[sqlx_model_hints(int4, default)]
    audience_size_override: Option<i32>,
    #[sqlx_model_hints(text, default)]
    audience_size_override_reason: Option<String>,
  },
  queries {
      // We don't expect next_scoring to be null for active handles,
      // but just in case, we force scoring on them.
      // next_scoring should never be null for active handles.
      need_scoring("
          status IN ('connecting', 'reconnecting') OR
              (status = 'active' AND (next_scoring < now() OR next_scoring IS NULL))")
  },
  has_many {
    HandleTopic(handle_id),
    Collab(handle_id),
    HandleScoring(handle_id),
  },
  belongs_to {
    Account(account_id),
    HandleScoring(current_scoring_id),
  }
}

impl_loggable!(Handle);

impl HandleHub {
    pub async fn force_hydrate(&self) -> AsamiResult<()> {
        let ids = self
            .state
            .db
            .fetch_all_scalar(sqlx::query_scalar!(
                "SELECT id FROM handles WHERE force_hydrate = true LIMIT 50"
            ))
            .await?;
        if ids.is_empty() {
            return Ok(());
        }
        self.hydrate_report_columns_for(ids.iter().copied()).await?;
        self.state
            .db
            .execute(sqlx::query!(
                "UPDATE handles SET force_hydrate = false WHERE id = ANY($1)",
                &ids
            ))
            .await?;
        Ok(())
    }

    pub async fn hydrate_report_columns_for(&self, ids: impl Iterator<Item = i32>) -> AsamiResult<()> {
        for id in ids {
            let handle = self.find(id).await?;
            let mut total_collabs = 0;
            let mut total_collab_rewards = u("0");
            for collab in handle.collab_scope().status_eq(CollabStatus::Cleared).all().await? {
                total_collabs += 1;
                total_collab_rewards += collab.reward_u256();
            }
            handle
                .update()
                .total_collabs(total_collabs)
                .total_collab_rewards(total_collab_rewards.encode_hex())
                .save()
                .await?;
        }

        Ok(())
    }

    pub async fn create_or_update_from_refresh_token(
        &self,
        account_id: String,
        raw_auth_code: String,
        raw_verifier: String,
    ) -> AsamiResult<Handle> {
        let client = self.state.settings.x.oauth_client("/x_grant_access")?;

        let auth_code = AuthorizationCode::new(raw_auth_code.clone());
        let verifier = PkceCodeVerifier::new(raw_verifier.clone());

        let token = match client.request_token(auth_code, verifier).await {
            Ok(x) => x,
            Err(e) => {
                self.state
                    .fail(
                        "private",
                        "create_or_update_from_refresh_token",
                        json::json!({
                            "code": "error_requesting_token",
                            "account": &account_id,
                            "raw_auth_code": &raw_auth_code,
                            "raw_verifier": &raw_verifier,
                            "description": format!("{e:?}"),
                        }),
                    )
                    .await;
                return Err(Error::validation("auth_code", "error_requesting_token"));
            }
        };

        let Some(refresh_token) = token.refresh_token().map(|t| t.secret().clone()) else {
            self.state
                .fail(
                    "private",
                    "create_or_update_from_refresh_token",
                    json::json!({
                        "code": "no_refresh_token_found",
                        "account": &account_id,
                        "raw_auth_code": &raw_auth_code,
                        "raw_verifier": &raw_verifier,
                        "description": ""
                    }),
                )
                .await;
            return Err(Error::service("x", "no_refresh_token_found"));
        };

        let twitter = TwitterApi::new(token);

        let me = match twitter.get_users_me().send().await {
            Ok(x) => x,
            Err(e) => {
                self.state
                    .fail(
                        "private",
                        "no_response_for_users_me",
                        json::json!({
                            "code": "no_re",
                            "account": &account_id,
                            "raw_auth_code": &raw_auth_code,
                            "raw_verifier": &raw_verifier,
                            "description": e.to_string(),
                        }),
                    )
                    .await;

                return Err(Error::service("x", "no_response_for_users_me"));
            }
        };

        let Some(user_details) = me.payload().data.as_ref() else {
            return Err(Error::service("x", "no_details_from_x"));
        };

        let tx = self.state.handle().transactional().await?;

        let existing = tx.select().user_id_eq(user_details.id.to_string()).optional().await?;

        let handle = if let Some(h) = existing {
            h.handle_update_refresh_token(refresh_token, account_id).await?
        } else {
            tx.setup_with_refresh_token(
                refresh_token,
                user_details.id.to_string(),
                user_details.username.to_string(),
                account_id,
            )
            .await?
        };

        tx.commit().await?;

        Ok(handle)
    }

    pub async fn setup_with_refresh_token(
        &self,
        refresh_token: String,
        user_id: String,
        username: String,
        account_id: String,
    ) -> AsamiResult<Handle> {
        if self.select().account_id_eq(&account_id).optional().await?.is_some() {
            return Err(Error::validation("account_id", "account_has_another_x_handle"));
        }

        Ok(self
            .insert(InsertHandle {
                account_id,
                username,
                user_id,
                x_refresh_token: Some(refresh_token),
                status: HandleStatus::SettingUp,
            })
            .save()
            .await?)
    }

    pub async fn setup_pending(&self) -> AsamiResult<Vec<Handle>> {
        let result = self.setup_pending_inner().await;
        if let Err(e) = result.as_ref() {
            self.state.fail("setup_pending", "error", &format!("{e:?}")).await;
        }
        result
    }

    pub async fn setup_pending_inner(&self) -> AsamiResult<Vec<Handle>> {
        let scope = self.select().status_eq(HandleStatus::SettingUp);

        if scope.count().await? == 0 {
            self.state.info("setup_pending", "no_handles_to_setup_skipping", ()).await;
        }

        let mut handles = vec![];

        for handle in scope.all().await? {
            if handle.poll_id().is_some() {
                handle.skip_setup().await?;
                continue;
            }

            let Ok(twitter) = handle.clone().x_api_client().await else {
                continue;
            };

            let idx = usize::try_from(*handle.id()).unwrap_or(0) % 20;
            let texts = match handle.account().await?.lang() {
                Lang::Es => &super::poll_texts::POLL_TEXTS_ES[idx],
                Lang::En => &super::poll_texts::POLL_TEXTS_EN[idx],
            };
            let post_result = twitter
                .post_tweet()
                .text(texts.text.to_string())
                .poll(texts.options.to_vec(), std::time::Duration::from_secs(60 * 60 * 24 * 7))
                .send()
                .await;

            match post_result {
                Ok(post) => {
                    self.state.fail("setup_pending", "post_returned", &post).await;
                    if let Some(poll_id) = post.into_data().map(|t| t.id.to_string()) {
                        handles.push(handle.setup_with_poll(poll_id).await?);
                    } else {
                        self.state.fail("setup_pending", "no_poll_id_returned", ()).await;
                    }
                }
                Err(e) => {
                    self.state.fail("setup_pending", "creating_poll", format!("{e:?}")).await;
                }
            }
            // We need to sleep here too.
            tokio::time::sleep(tokio::time::Duration::from_millis(
                self.state.settings.x.score_cooldown_seconds * 1000,
            ))
            .await;
        }

        Ok(handles)
    }

    pub async fn score_pending(&self) -> AsamiResult<Vec<Handle>> {
        let result = self.score_pending_inner().await;
        if let Err(e) = result.as_ref() {
            self.state.fail("score_pending", "error", &format!("{e:?}")).await;
        }
        result
    }

    pub async fn score_pending_inner(&self) -> AsamiResult<Vec<Handle>> {
        let pending = self.need_scoring().all().await?;

        if pending.is_empty() {
            self.state.info("score_pending", "no_handles_pending_scoring", ()).await;
            return Ok(vec![]);
        }

        self.state.info("score_pending", "start_scoring_handles", ()).await;
        for handle in pending.clone() {
            self.state.info("score_pending", "starting_handle", &handle).await;

            handle.state.handle_scoring().create_and_apply(handle).await?;
            self.state.info("score_pending", "scored_handle", ()).await;
            self.state.info("score_pending", "sleeping", ()).await;
            let cooldown = tokio::time::Duration::from_secs(self.state.settings.x.score_cooldown_seconds);
            tokio::time::sleep(cooldown).await;
        }
        self.state.info("score_pending", "done_scoring_handles", ()).await;

        Ok(pending)
    }
}

impl Handle {
    pub async fn x_api_client(self) -> AsamiResult<TwitterApi<Oauth2Token>> {
        use twitter_v2::oauth2::RequestTokenError::*;
        use twitter_v2::Error::*;

        let client = self.state.settings.x.oauth_client("/x_grant_access")?;

        let Some(refresh_token) = self.x_refresh_token() else {
            return Err(Error::validation(
                "x_refresh_token",
                "is_missing_to_create_x_api_client",
            ));
        };

        match client.refresh_token(&RefreshToken::new(refresh_token.clone())).await {
            Ok(token) => {
                self.update().x_refresh_token(token.refresh_token().map(|t| t.secret().clone())).save().await?;
                Ok(TwitterApi::new(token))
            }
            e @ Err(Oauth2TokenError(ServerResponse(_)) | Oauth2RevocationError(ServerResponse(_))) => {
                // These two scenarios seem to indicate we have a bad refresh token
                // So we need to obtain a new one, we force this by deleting the refresh
                // token which will prompt users to do this next time they log in.
                self.handle_expired_refresh_token(e).await?;
                Err(Error::validation("x_refresh_token", "failed_to_obtain_token"))
            }
            Err(e) => {
                let _ = self.fail("error_building_twitter_client", e.to_string()).await;
                Err(Error::validation("twitter_client", "could_not_build_client"))
            }
        }
    }

    pub async fn handle_update_refresh_token(self, refresh_token: String, account_id: String) -> AsamiResult<Handle> {
        if self.account_id() != &account_id {
            return Err(Error::validation("x", "update_rejected_account_id_mismatch"));
        }
        let next_status = match self.status() {
            HandleStatus::NeverConnected => HandleStatus::SettingUp,
            HandleStatus::Disconnected => HandleStatus::Reconnecting,
            _ => return Err(Error::validation("x", "handle_does_not_need_refresh_token")),
        };
        Ok(self.update().status(next_status).x_refresh_token(Some(refresh_token)).save().await?)
    }

    pub async fn handle_expired_refresh_token(self, e: impl std::fmt::Debug) -> AsamiResult<Handle> {
        let _ = self.fail("refresh_token_invalidated", format!("{e:?}")).await;
        let invalidated = self.x_refresh_token().clone();
        let next_status = match self.status() {
            HandleStatus::NeverConnected | HandleStatus::SettingUp | HandleStatus::Connecting => {
                HandleStatus::NeverConnected
            }
            _ => HandleStatus::Disconnected,
        };
        Ok(self
            .update()
            .invalidated_x_refresh_token(invalidated)
            .x_refresh_token(None)
            .status(next_status)
            .save()
            .await?)
    }

    pub async fn skip_setup(self) -> sqlx::Result<Self> {
        self.update().status(HandleStatus::Connecting).save().await
    }
    pub async fn setup_with_poll(self, poll_id: String) -> sqlx::Result<Self> {
        let existing = self
            .state
            .auth_method()
            .select()
            .kind_eq(AuthMethodKind::X)
            .lookup_key_eq(self.user_id())
            .count()
            .await?
            > 0;

        self.account()
            .await?
            .update()
            .name(format!("@{}", self.username()))
            .name_is_locked(true)
            .save()
            .await?;

        if !existing {
            let user = self.state.account_user().select().account_id_eq(self.account_id()).one().await?;
            self.state
                .auth_method()
                .insert(InsertAuthMethod {
                    user_id: *user.user_id(),
                    lookup_key: self.attrs.user_id.clone(),
                    kind: AuthMethodKind::X,
                })
                .save()
                .await?;
        }

        self.update().poll_id(Some(poll_id)).status(HandleStatus::Connecting).save().await
    }

    // NOTE: If collab exists the error should be collab_exists.
    // Do not return any other errors earlier.
    pub async fn validate_collaboration(&self, campaign: &Campaign, reward: U256, trigger: &str) -> AsamiResult<()> {
        // The funds calculation is always made with a fresh copy of the campaign as it could have been updated.
        let available_funds = campaign
            .reloaded()
            .await?
            .available_budget()
            .await
            .map_err(|e| Error::runtime(&format!("campaign available funds calculation: {e:?}")))?;

        let collab_exists = self
            .state
            .collab()
            .select()
            .collab_trigger_unique_id_eq(trigger.to_string())
            .campaign_id_eq(campaign.attrs.id)
            .count()
            .await?
            > 0;

        if collab_exists {
            return Err(Error::validation("all", "collab_exists"));
        }

        let Some(score) = self.score() else {
            return Err(Error::validation("all", "handle_is_not_scored"));
        };

        if u256(score) <= wei("5") {
            return Err(Error::validation("all", "score_is_too_low"));
        }

        if reward < milli("50") {
            return Err(Error::validation("all", "reward_would_bee_too_small"));
        }

        if available_funds < reward {
            return Err(Error::validation("all", "campaign_has_not_enough_funds"));
        }

        if campaign.valid_until().map(|end| end <= Utc::now()).unwrap_or(true) {
            return Err(Error::validation("all", "campaign_has_expired"));
        }

        let handle_topics = self.topic_ids().await?;
        if !campaign.topic_ids().await?.iter().all(|topic| handle_topics.contains(topic)) {
            return Err(Error::validation("topics", "handle_is_missing_topics"));
        }

        let member_rating = self
            .state
            .community_member()
            .select()
            .account_id_eq(campaign.account_id())
            .member_id_eq(self.account_id())
            .optional()
            .await?
            .map(|m| *m.rating())
            .unwrap_or(CommunityMemberRating::Normal);

        if *campaign.thumbs_up_only() && member_rating != CommunityMemberRating::Good {
            return Err(Error::validation("all", "campaign_is_thumbs_up_only"));
        }

        if member_rating == CommunityMemberRating::Bad {
            return Err(Error::validation("all", "member_has_been_excluded"));
        }

        Ok(())
    }

    // Returning None means the account cannot receive rewards.
    // Most likely because it has not been scored yet.
    pub fn reward_for(&self, campaign: &Campaign) -> Option<U256> {
        let high = campaign.max_individual_reward_u256();
        let low = campaign.min_individual_reward_u256();
        let score = u256(self.score().as_ref()?);
        let base = campaign.price_per_point_u256() * score;
        Some(base.clamp(low, high))
    }

    pub async fn add_topic(&self, topic: &Topic) -> anyhow::Result<HandleTopic> {
        Ok(self
            .state
            .handle_topic()
            .insert(InsertHandleTopic {
                handle_id: *self.id(),
                topic_id: *topic.id(),
            })
            .save()
            .await?)
    }

    pub async fn topic_ids(&self) -> sqlx::Result<Vec<i32>> {
        Ok(self.handle_topic_vec().await?.into_iter().map(|t| t.attrs.topic_id).collect())
    }

    pub async fn update_language_topics(&self, corpus: &str) -> anyhow::Result<()> {
        let detector = lingua::LanguageDetectorBuilder::from_languages(&[lingua::Language::Spanish]).build();

        let Some(es) = self.state.topic().select().name_eq("speaks_spanish".to_string()).optional().await? else {
            return Ok(());
        };

        let Some(en) = self.state.topic().select().name_eq("speaks_english".to_string()).optional().await? else {
            return Ok(());
        };

        for topic in self.handle_topic_scope().topic_id_in(vec![*es.id(), *en.id()]).all().await? {
            topic.delete().await?;
        }

        let topic = if detector.detect_language_of(corpus).is_none() {
            es
        } else {
            en
        };

        self.add_topic(&topic).await?;

        Ok(())
    }
}

model! {
  state: App,
  table: handle_topics,
  struct HandleTopic {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(int4)]
    handle_id: i32,
    #[sqlx_model_hints(int4, op_in)]
    topic_id: i32,
  }
}

make_sql_enum![
    "handle_status",
    pub enum HandleStatus {
        NeverConnected, // New account with no valid access token. Setup -> adds refresh token to it.
        SettingUp,      // Has first refresh token. Connect -> Creates poll and moves to connecting..
        Connecting,     // Has token. Score -> Makes it Active or SettingUp.
        Active,         // Account is usable.
        Disconnected,   // Existing account, with no valid access token. Reconnect -> adds new token to it.
        Reconnecting,   // Has renewed token. Score -> Makes it Active or Disconnected.
        Inactive,       // Account was banned or made inactive. Will never be re-scored and cannot be reconnected.
    }
];
