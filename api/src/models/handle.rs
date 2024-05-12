use super::*;

model! {
  state: App,
  table: handles,
  struct Handle {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(varchar)]
    account_id: String,
    #[sqlx_model_hints(site)]
    site: Site,
    #[sqlx_model_hints(varchar)]
    username: String,
    #[sqlx_model_hints(varchar, default)]
    user_id: Option<String>,
    #[sqlx_model_hints(varchar, default)]
    score: Option<String>,
    #[sqlx_model_hints(handle_status, default)]
    status: HandleStatus,
  },
  has_many {
    HandleTopic(handle_id),
  },
  belongs_to {
    Account(account_id),
  }
}

impl HandleHub {
    pub async fn verify_and_score_x(&self) -> AsamiResult<Vec<Handle>> {
        use rust_decimal::prelude::*;
        use tokio::time::*;
        use twitter_v2::{api_result::*, authorization::BearerToken, query::*, TwitterApi};

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
                self.state.info("verify_and_score_x", "mention_post", &post).await;
                let Some(author_id) = post.author_id else { continue };
                let Some(author) = payload
                    .includes()
                    .and_then(|i| i.users.as_ref())
                    .and_then(|i| i.iter().find(|x| x.id == author_id))
                else {
                    self.state.info("verify_and_score_x", "skipped_post_no_author", &post.id).await;
                    continue;
                };

                let Some(public_metrics) = author.public_metrics.clone() else {
                    self.state.info("verify_and_score_x", "skipped_post_no_public_metrics", &post.id).await;
                    continue;
                };

                let text: &String = post.note_tweet.as_ref().and_then(|n| n.text.as_ref()).unwrap_or(&post.text);

                if let Some(capture) = msg_regex.captures(&text) {
                    let Ok(account_id_str) = capture[1].parse::<String>() else {
                        self.state
                            .info(
                                "verify_and_score_x",
                                "skipped_post_no_account_id_str",
                                format!("{capture:?}"),
                            )
                            .await;
                        continue;
                    };
                    let Ok(account_id) = U256::from_dec_str(&account_id_str).map(U256::encode_hex) else {
                        self.state.info("verify_and_score_x", "skipped_post_no_account_id_dec", &account_id_str).await;
                        continue;
                    };

                    let Some(req) = self
                        .state
                        .handle()
                        .select()
                        .status_eq(HandleStatus::Unverified)
                        .site_eq(Site::X)
                        .username_ilike(&author.username)
                        .account_id_eq(&account_id)
                        .optional()
                        .await?
                    else {
                        self.state
                            .info(
                                "verify_and_score_x",
                                "skipped_post_no_pending_request",
                                (&author.username, &account_id),
                            )
                            .await;
                        continue;
                    };

                    let score = U256::from(public_metrics.followers_count) * wei("85") / wei("100");
                    handles.push(req.verify(author_id.to_string()).await?.set_score(score).await?);
                } else {
                    self.state.info("verify_and_score_x", "skipped_post_no_regex_capture", &post.text).await;
                }
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

        indexer_state.update().x_handle_verification_checkpoint(checkpoint).save().await?;
        self.state
            .info(
                "verify_and_score_x",
                "done_processing_updating_indexer_state",
                &checkpoint,
            )
            .await;
        Ok(handles)
    }
}

impl Handle {
    pub async fn verify(self, user_id: String) -> sqlx::Result<Self> {
        self.update().user_id(Some(user_id)).status(HandleStatus::Verified).save().await
    }

    pub async fn set_score(self, score: U256) -> sqlx::Result<Self> {
        self.update().score(Some(score.encode_hex())).status(HandleStatus::Active).save().await
    }

    pub async fn topic_ids(&self) -> sqlx::Result<Vec<i32>> {
        Ok(self.handle_topic_vec().await?.into_iter().map(|t| t.attrs.topic_id).collect())
    }

    pub async fn validate_collaboration(&self, campaign: &Campaign, trigger: &str) -> AsamiResult<()> {
        let available_funds = campaign
            .available_budget()
            .await
            .map_err(|e| Error::runtime(&format!("campaign available funds calculation: {e:?}")))?;

        if available_funds <= wei("0") {
            return Err(Error::validation("site", "campaign_has_no_funds"));
        }

        if campaign.valid_until().map(|end| end <= Utc::now()).unwrap_or(true) {
            return Err(Error::validation("site", "campaign_has_expired"));
        }

        if !self.site().can_do_campaign_kind(campaign.campaign_kind()) {
            return Err(Error::validation("site", "campaign_and_handle_sites_dont_match"));
        }

        let handle_topics = self.topic_ids().await?;
        if !campaign.topic_ids().await?.iter().all(|topic| handle_topics.contains(topic)) {
            return Err(Error::validation("topics", "handle_is_missing_topics"));
        }

        let collab_exists = self
            .state
            .collab()
            .select()
            .collab_trigger_unique_id_eq(trigger.to_string())
            .campaign_id_eq(campaign.attrs.id.clone())
            .count()
            .await?
            > 0;

        if collab_exists {
            return Err(Error::validation("all", "collab_exists"));
        }

        Ok(())
    }

    // Returning None means the account cannot receive rewards.
    // Most likely because it has not been scored yet.
    pub fn reward_for(&self, _campaign: &Campaign) -> Option<U256> {
        let score = u256(self.score().as_ref()?);
        Some(milli("9999").min(milli("1") * score))
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
