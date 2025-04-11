use serde_json::json;

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
    #[sqlx_model_hints(varchar, default, op_like)]
    user_id: Option<String>,
    #[sqlx_model_hints(varchar, default)]
    score: Option<String>,
    #[sqlx_model_hints(handle_status, default, op_in)]
    status: HandleStatus,

    // These columns are part of the account activity report
    // they are denormalized and re-hydrated when:
    // - A collab for one of the user handles user is settled.
    #[sqlx_model_hints(boolean, default)]
    force_hydrate: bool,
    #[sqlx_model_hints(int4, default)]
    total_collabs: i32,
    #[sqlx_model_hints(varchar, default)]
    total_collab_rewards: String,

    // These columns are the raw values updated for the new scoring algorithm
    #[sqlx_model_hints(timestamptz, default)]
    last_scoring: Option<UtcDateTime>,
    #[sqlx_model_hints(int4, default)]
    avg_impression_count: i32,
    #[sqlx_model_hints(int4, default)]
    avg_reply_count: i32,
    #[sqlx_model_hints(int4, default)]
    avg_repost_count: i32,
    #[sqlx_model_hints(int4, default)]
    avg_like_count: i32,
    #[sqlx_model_hints(int4, default)]
    scored_tweet_count: i32,
  },
  queries {
      need_scoring("status IN ('verified', 'active') AND (last_scoring IS NULL OR last_scoring < $1)", date: UtcDateTime)
  },
  has_many {
    HandleTopic(handle_id),
    Collab(handle_id),
  },
  belongs_to {
    Account(account_id),
  }
}

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

    pub async fn verify_pending(&self) -> AsamiResult<Vec<Handle>> {
        use rust_decimal::prelude::*;
        use tokio::time::*;
        use twitter_v2::{api_result::*, authorization::BearerToken, query::*, TwitterApi};

        if self.select().status_eq(HandleStatus::Unverified).count().await? == 0 {
            self.state.info("verify_pending", "no_unverified_handles_found_skipping", ()).await;
            return Ok(vec![]);
        }

        let mut handles = vec![];

        let msg_regex = regex::Regex::new(r#"\@asami_club \[(\d*)\]"#)?;
        let conf = &self.state.settings.x;
        let auth = BearerToken::new(&conf.bearer_token);
        let api = TwitterApi::new(auth);
        let indexer_state = self.state.indexer_state().get().await?;

        let mentions = api
            .get_user_mentions(conf.asami_user_id)
            .since_id(indexer_state.attrs.x_handle_verification_checkpoint.to_u64().unwrap_or(0))
            .max_results(100)
            .user_fields(vec![UserField::Id, UserField::Username, UserField::PublicMetrics])
            .tweet_fields(vec![TweetField::NoteTweet])
            .expansions(vec![TweetExpansion::AuthorId])
            .send()
            .await?;

        let checkpoint: i64 =
            mentions.meta().and_then(|m| m.newest_id.clone()).and_then(|i| i.parse().ok()).unwrap_or(0);

        let mut page = Some(mentions);
        let mut pages = 0;

        while let Some(mentions) = page {
            let payload = mentions.payload();
            let Some(data) = payload.data() else { break };

            for post in data {
                self.state.info("verify_pending", "mention_post", &post).await;
                let Some(author_id) = post.author_id else { continue };
                let Some(author) = payload
                    .includes()
                    .and_then(|i| i.users.as_ref())
                    .and_then(|i| i.iter().find(|x| x.id == author_id))
                else {
                    self.state.info("verify_pending", "skipped_post_no_author", &post.id).await;
                    continue;
                };

                let text: &String = post.note_tweet.as_ref().and_then(|n| n.text.as_ref()).unwrap_or(&post.text);

                let Some(capture) = msg_regex.captures(text) else {
                    self.state.info("verify_pending", "skipped_post_no_regex_capture", &post.text).await;
                    continue;
                };

                let Ok(account_id_str) = capture[1].parse::<String>() else {
                    self.state
                        .info(
                            "verify_pending",
                            "skipped_post_no_account_id_str",
                            format!("{capture:?}"),
                        )
                        .await;
                    continue;
                };

                let Ok(account_id) = U256::from_dec_str(&account_id_str).map(U256::encode_hex) else {
                    self.state.info("verify_pending", "skipped_post_no_account_id_dec", &account_id_str).await;
                    continue;
                };

                let Some(req) = self
                    .state
                    .handle()
                    .select()
                    .status_eq(HandleStatus::Unverified)
                    .username_ilike(&author.username)
                    .account_id_eq(&account_id)
                    .optional()
                    .await?
                else {
                    self.state
                        .info(
                            "verify_pending",
                            "skipped_post_no_pending_request",
                            (&author.username, &account_id),
                        )
                        .await;
                    continue;
                };

                handles.push(req.verify(author_id.to_string()).await?);
            }

            // We only fetch a backlog of 700 tweets.
            // Older mentions are dropped and should be tried again by the users.
            pages += 1;
            if pages == 5 {
                break;
            }
            tokio::time::sleep(Duration::from_millis(3 * 60 * 1000)).await;
            page = mentions.next_page().await?;
        }

        if *indexer_state.x_handle_verification_checkpoint() < checkpoint {
            indexer_state.update().x_handle_verification_checkpoint(checkpoint).save().await?;
            self.state.info("verify_pending", "done_processing_updating_indexer_state", &checkpoint).await;
        }
        Ok(handles)
    }

    pub async fn score_pending(&self) -> AsamiResult<Vec<Handle>> {
        use twitter_v2::{authorization::BearerToken, query::*, TwitterApi};
        let now = Utc::now();

        // The window to score is one month starting a week ago.
        // Se we don't count old tweets, but we also give recent
        // tweets a week to gain impressions.
        let start_time = time::OffsetDateTime::from_unix_timestamp((now - chrono::Duration::days(37)).timestamp())?
            .to_offset(time::UtcOffset::UTC);

        let end_time = time::OffsetDateTime::from_unix_timestamp((now - chrono::Duration::days(7)).timestamp())?
            .to_offset(time::UtcOffset::UTC);

        let pending = self.need_scoring(now - chrono::Duration::days(7)).all().await?;

        if pending.is_empty() {
            self.state.info("score_pending", "no_handles_pending_scoring", ()).await;
            return Ok(vec![]);
        }

        let mut handles = vec![];

        let conf = &self.state.settings.x;
        let auth = BearerToken::new(&conf.bearer_token);
        let api = TwitterApi::new(auth);

        for handle in pending {
            self.state.info("score_pending", "scoring_handle", json![{"handle": &handle}]).await;
            let Some(user_id) = handle.user_id().as_ref().and_then(|x| x.parse::<u64>().ok()) else {
                self.state.info("score_pending", "handle_has_no_user_id", &handle).await;
                continue;
            };

            let response = api
                .get_user_tweets(user_id)
                .max_results(75)
                .start_time(start_time)
                .end_time(end_time)
                .exclude(vec![Exclude::Retweets, Exclude::Replies])
                .tweet_fields(vec![TweetField::AuthorId, TweetField::PublicMetrics])
                .send()
                .await?;

            tokio::time::sleep(tokio::time::Duration::from_millis(3 * 60 * 1000)).await;

            let Some(tweets) = response.data() else {
                self.state
                    .info(
                        "score_pending",
                        "could_not_get_tweets_for",
                        json![{"handle":handle.id(), "response":&response}],
                    )
                    .await;
                handles.push(
                    handle
                        .update()
                        .avg_impression_count(0)
                        .avg_reply_count(0)
                        .avg_repost_count(0)
                        .avg_like_count(0)
                        .scored_tweet_count(0)
                        .last_scoring(Some(now))
                        .status(HandleStatus::Active)
                        .score(Some(u("0").encode_hex()))
                        .save()
                        .await?,
                );
                continue;
            };

            let mut impression_count = 0_i32;
            let mut reply_count = 0_i32;
            let mut repost_count = 0_i32;
            let mut like_count = 0_i32;
            let mut tweet_count = 0_i32;
            for tweet in tweets {
                let Some(m) = tweet.public_metrics.as_ref() else {
                    self.state
                        .info(
                            "score_pending",
                            "no_tweet_metrics_for",
                            json![{"handle":handle.id(), "tweet":tweet.id}],
                        )
                        .await;
                    continue;
                };
                self.state
                    .info(
                        "score_pending",
                        "got_tweet_metrics",
                        json![{"handle":handle.id(), "metrics":m, "tweet":tweet.id}],
                    )
                    .await;
                // We estimate the impression count from public metrics
                // until we change the system to request access to private impression metrics.
                let estimated_impression_count: usize = vec![
                    (m.like_count * 45),
                    (m.reply_count * 300),
                    (m.retweet_count * 30),
                    m.quote_count.map(|q| q * 60).unwrap_or(0),
                ]
                .into_iter()
                .max()
                .unwrap_or(0);

                impression_count += estimated_impression_count as i32;
                reply_count += m.reply_count as i32;
                repost_count += m.retweet_count as i32;
                like_count += m.like_count as i32;
                tweet_count += 1;
            }

            let divisor = std::cmp::max(tweet_count, 10_i32);
            let avg_impression_count = f64::from(impression_count) / f64::from(divisor);
            let score_result = 10_000.0 * (1.0 - (-0.0001 * avg_impression_count).exp());
            let score = Some(U256::from(score_result.floor() as u64).encode_hex());

            handles.push(
                handle
                    .update()
                    .avg_impression_count(impression_count / divisor)
                    .avg_reply_count(reply_count / divisor)
                    .avg_repost_count(repost_count / divisor)
                    .avg_like_count(like_count / divisor)
                    .scored_tweet_count(tweet_count)
                    .last_scoring(Some(now))
                    .score(score)
                    .status(HandleStatus::Active)
                    .save()
                    .await?,
            );
        }

        Ok(handles)
    }
}

impl Handle {
    pub async fn verify(self, user_id: String) -> sqlx::Result<Self> {
        let existing =
            self.state.auth_method().select().kind_eq(AuthMethodKind::X).lookup_key_eq(&user_id).count().await? > 0;

        if !existing {
            let user = self.state.account_user().select().account_id_eq(self.account_id()).one().await?;
            self.state
                .auth_method()
                .insert(InsertAuthMethod {
                    user_id: *user.user_id(),
                    lookup_key: user_id.clone(),
                    kind: AuthMethodKind::X,
                })
                .save()
                .await?;
        }

        self.update().user_id(Some(user_id)).status(HandleStatus::Verified).save().await
    }

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
}

model! {
  state: App,
  table: handle_topics,
  struct HandleTopic {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(int4)]
    handle_id: i32,
    #[sqlx_model_hints(int4)]
    topic_id: i32,
  }
}

make_sql_enum![
    "handle_status",
    pub enum HandleStatus {
        Unverified,
        Verified,
        Active,
        Inactive,
    }
];
