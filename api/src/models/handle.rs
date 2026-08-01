use rocket::serde::json;
use twitter_v2::{
    api_result::PaginableApiResponse,
    authorization::Oauth2Token,
    data::ReferencedTweetKind,
    oauth2::*,
    query::{TweetField, UserField},
    TwitterApi,
};

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

    #[sqlx_model_hints(text)]
    profile_image_url: Option<String>,
  },
  has_many {
    HandleTopic(handle_id),
    Collab(handle_id),
  },
  belongs_to {
    Account(account_id),
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

        let me =
            match twitter.get_users_me().user_fields([UserField::Username, UserField::ProfileImageUrl]).send().await {
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
                user_details.profile_image_url.as_ref().map(|x| x.to_string()),
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
        profile_image_url: Option<String>,
    ) -> AsamiResult<Handle> {
        if self.select().account_id_eq(&account_id).optional().await?.is_some() {
            return Err(Error::validation("account_id", "account_has_another_x_handle"));
        }

        Ok(self
            .insert(InsertHandle {
                account_id,
                username,
                user_id,
                profile_image_url,
                x_refresh_token: Some(refresh_token),
                status: HandleStatus::SettingUp,
            })
            .save()
            .await?)
    }

    pub async fn setup_pending(&self) -> AsamiResult<Vec<Handle>> {
        let result = self.connect_pending_inner().await;
        if let Err(e) = result.as_ref() {
            self.state.fail("setup_pending", "error", &format!("{e:?}")).await;
        }
        result
    }

    pub async fn connect_pending_inner(&self) -> AsamiResult<Vec<Handle>> {
        let pending = self.select().status_in(vec![HandleStatus::SettingUp, HandleStatus::Reconnecting]).all().await?;

        if pending.is_empty() {
            self.state.info("setup_pending", "no_handles_to_setup_skipping", ()).await;
        }

        let mut handles = vec![];

        for handle in pending {
            match handle.clone().x_api_client().await {
                Ok(_) => handles.push(handle.finish_connection().await?),
                Err(_) => continue,
            }
        }

        Ok(handles)
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

    pub async fn finish_connection(self) -> sqlx::Result<Self> {
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

        self.update().status(HandleStatus::Active).save().await
    }

    // NOTE: If collab exists the error should be collab_exists.
    // Do not return any other errors earlier.
    pub async fn validate_collaboration(&self, campaign: &Campaign, trigger: &str) -> AsamiResult<()> {
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

        let user_collaborated = self
            .state
            .collab()
            .select()
            .handle_id_eq(self.id())
            .campaign_id_eq(campaign.attrs.id)
            .count()
            .await?
            > 0;

        if user_collaborated {
            return Err(Error::validation("all", "user_already_collaborated"));
        }

        let Some(score) = self.member_score_for_campaign(campaign).await? else {
            return Err(Error::validation("all", "member_has_no_reputation_score"));
        };

        if score <= 0.0 {
            return Err(Error::validation("all", "member_score_is_too_low"));
        }

        if campaign.valid_until().map(|end| end <= Utc::now()).unwrap_or(true) {
            return Err(Error::validation("all", "campaign_has_expired"));
        }

        if campaign.kind() != &CampaignKind::XRepost {
            return Err(Error::validation("all", "campaign_kind_is_not_member_actionable"));
        }

        if !campaign.is_visible_to_members() {
            return Err(Error::validation("all", "campaign_is_not_open_for_new_reposts"));
        }

        if available_funds.is_zero() {
            return Err(Error::validation("all", "campaign_has_not_enough_funds"));
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

    async fn member_score_for_campaign(&self, campaign: &Campaign) -> AsamiResult<Option<f64>> {
        Ok(self
            .state
            .community_member()
            .creator_scores_for_targets(campaign.account_id(), &[self.account_id().clone()])
            .await?
            .remove(self.account_id())
            .map(|score| score.total_score()))
    }

    pub async fn has_reposted_x_campaign(&self, tweet_id: &str) -> AsamiResult<bool> {
        let user_ctx = self.clone().x_api_client().await?.with_user_ctx().await.map_err(|e| {
            Error::service(
                "x",
                &format!("could_not_build_user_context_for_repost_verification:{e}"),
            )
        })?;

        let mut tweets = Some(
            user_ctx
                .get_my_tweets()
                .tweet_fields([TweetField::ReferencedTweets])
                .max_results(100)
                .send()
                .await
                .map_err(|e| Error::service("x", &format!("could_not_fetch_user_tweets:{e}")))?,
        );

        while let Some(page) = tweets {
            let payload = page.payload();
            if let Some(items) = payload.data() {
                for tweet in items {
                    if tweet.referenced_tweets.as_ref().map(|refs| {
                        refs.iter().any(|reference| {
                            reference.id.to_string() == tweet_id
                                && matches!(
                                    reference.kind,
                                    ReferencedTweetKind::Retweeted | ReferencedTweetKind::Quoted
                                )
                        })
                    }) == Some(true)
                    {
                        return Ok(true);
                    }
                }

                if items.len() < 100 {
                    break;
                }
            } else {
                break;
            }

            tweets = page
                .next_page()
                .await
                .map_err(|e| Error::service("x", &format!("could_not_fetch_next_user_tweets:{e}")))?;
        }

        Ok(false)
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
        SettingUp,      // Has first refresh token. Setup activates it once the token is usable.
        Connecting,     // Legacy transitional state.
        Active,         // Account is usable.
        Disconnected,   // Existing account, with no valid access token. Reconnect -> adds new token to it.
        Reconnecting,   // Has renewed token. Setup activates it again.
        Inactive,       // Account was banned or made inactive and cannot be reconnected.
    }
];
