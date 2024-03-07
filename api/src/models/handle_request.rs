use super::*;

model! {
  state: App,
  table: handle_requests,
  struct HandleRequest {
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
    price: Option<String>,
    #[sqlx_model_hints(varchar, default)]
    score: Option<String>,
    #[sqlx_model_hints(handle_request_status, default)]
    status: HandleRequestStatus,
    #[sqlx_model_hints(int4, default)]
    on_chain_tx_id: Option<i32>,
  },
  has_many {
    HandleRequestTopic(handle_request_id),
  },
  belongs_to {
    Account(account_id),
    OnChainTx(on_chain_tx_id),
  }
}

impl HandleRequestHub {
  pub async fn verify_and_appraise_x(&self) -> AsamiResult<Vec<HandleRequest>> {
    use rust_decimal::prelude::*;
    use tokio::time::*;
    use twitter_v2::{api_result::*, authorization::BearerToken, query::*, TwitterApi};

    let mut handle_requests = vec![];

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

    let checkpoint: i64 = mentions.meta().and_then(|m| m.oldest_id.clone()).and_then(|i| i.parse().ok()).unwrap_or(0);

    let mut page = Some(mentions);
    let mut pages = 0;

    while let Some(mentions) = page {
      let payload = mentions.payload();
      let Some(data) = payload.data() else { break };

      for post in data {
        self.state.info("verify_and_appraise_x", "mention_post", &post).await;
        let Some(author_id) = post.author_id else { continue };
        let Some(author) =
          payload.includes().and_then(|i| i.users.as_ref()).and_then(|i| i.iter().find(|x| x.id == author_id))
        else {
          self.state.info("verify_and_appraise_x", "skipped_post_no_author", &post.id).await;
          continue;
        };

        let Some(public_metrics) = author.public_metrics.clone() else {
          self.state.info("verify_and_appraise_x", "skipped_post_no_public_metrics", &post.id).await;
          continue;
        };

        let text: &String = post.note_tweet.as_ref().and_then(|n| n.text.as_ref()).unwrap_or(&post.text);

        if let Some(capture) = msg_regex.captures(&text) {
          let Ok(account_id_str) = capture[1].parse::<String>() else {
            self
              .state
              .info(
                "verify_and_appraise_x",
                "skipped_post_no_account_id_str",
                format!("{capture:?}"),
              )
              .await;
            continue;
          };
          let Ok(account_id) = U256::from_dec_str(&account_id_str).map(U256::encode_hex) else {
            self
              .state
              .info(
                "verify_and_appraise_x",
                "skipped_post_no_account_id_dec",
                &account_id_str,
              )
              .await;
            continue;
          };

          let Some(req) = self
            .state
            .handle_request()
            .select()
            .status_eq(HandleRequestStatus::Unverified)
            .site_eq(Site::X)
            .username_ilike(&author.username)
            .account_id_eq(&account_id)
            .optional()
            .await?
          else {
            self
              .state
              .info(
                "verify_and_appraise_x",
                "skipped_post_no_pending_request",
                (&author.username, &account_id),
              )
              .await;
            continue;
          };

          let score = U256::from(public_metrics.followers_count) * wei("85") / wei("100");
          let price = u256(indexer_state.suggested_price_per_point()) * score;
          handle_requests.push(req.verify(author_id.to_string()).await?.appraise(price, score).await?);
        } else {
          self.state.info("verify_and_appraise_x", "skipped_post_no_regex_capture", &post.text).await;
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
    self
      .state
      .info(
        "verify_and_appraise_x",
        "done_processing_updating_indexer_state",
        &checkpoint,
      )
      .await;
    Ok(handle_requests)
  }
}

impl_on_chain_tx_request! {HandleRequestHub {
  type Model = HandleRequest;
  type Update = UpdateHandleRequestHub;
  type Status = HandleRequestStatus;
  type Param = on_chain::Handle;

  async fn as_param(&self, model: &Self::Model) -> sqlx::Result<Self::Param> {
    let a = &model.attrs;

    let price = a.price.as_ref().map(u256).unwrap_or_else(|| u("0"));
    let score = a.score.as_ref().map(u256).unwrap_or_else(|| u("0"));
    let user_id = a.user_id.clone().unwrap_or_default();

    let topics: Vec<U256> = model.topic_ids().await?.iter().map(u256).collect();

    Ok(Self::Param {
      id: 0.into(),
      account_id: u256(&a.account_id),
      site: a.site as u8,
      price,
      score,
      topics: topics.clone(),
      username: a.username.clone(),
      user_id,
      last_updated: 0.into(),
      new_score: score,
      new_price: price,
      new_topics: topics,
      new_username: a.username.clone(),
      needs_update: false,
    })
  }

  fn fn_call(&self, params: Vec<Self::Param>) -> AsamiFunctionCall {
    self.state.on_chain.contract.admin_make_handles(params)
  }

  fn pending_status() -> Self::Status { HandleRequestStatus::Appraised }
  fn submitted_status() -> Self::Status { HandleRequestStatus::Submitted }
  fn done_status() -> Self::Status { HandleRequestStatus::Done }
}}

impl HandleRequest {
  pub async fn verify(self, user_id: String) -> sqlx::Result<Self> {
    self.update().user_id(Some(user_id)).status(HandleRequestStatus::Verified).save().await
  }

  pub async fn appraise(self, price: U256, score: U256) -> sqlx::Result<Self> {
    self
      .update()
      .price(Some(price.encode_hex()))
      .score(Some(score.encode_hex()))
      .status(HandleRequestStatus::Appraised)
      .save()
      .await
  }

  pub async fn topic_ids(&self) -> sqlx::Result<Vec<String>> {
    Ok(self.handle_request_topic_vec().await?.into_iter().map(|t| t.attrs.topic_id).collect())
  }
}

model! {
  state: App,
  table: handle_request_topics,
  struct HandleRequestTopic {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(int4)]
    handle_request_id: i32,
    #[sqlx_model_hints(varchar)]
    topic_id: String,
  }
}

make_sql_enum![
  "handle_request_status",
  pub enum HandleRequestStatus {
    Unverified, // Newly created, never sent on-chain.
    Verified,   // Verified off-chain.
    Appraised,  // Appraised off-chain.
    Submitted,  // Sent, after this we listen for events regarding this handle.
    Done,       // Local DB knows this handle is now on-chain.
  }
];
