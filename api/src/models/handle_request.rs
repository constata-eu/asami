use super::*;

model!{
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
    #[sqlx_model_hints(varchar, default)]
    handle_id: Option<String>,
    #[sqlx_model_hints(varchar, default)]
    tx_hash: Option<String>,
  },
  has_many {
    HandleRequestTopic(handle_request_id),
  },
  belongs_to {
    Account(account_id),
    Handle(handle_id),
  }
}

impl HandleRequestHub {
  pub async fn verify_and_appraise_x(&self) -> AsamiResult<Vec<HandleRequest>> {
    use twitter_v2::{TwitterApi, authorization::BearerToken, query::*, api_result::*};
    use rust_decimal::prelude::*;
    use tokio::time::*;

    let mut handle_requests = vec![];

    let msg_regex = regex::Regex::new(r#"\@asami_club \[(\d*)\]"#)?;
    let conf = &self.state.settings.x;
    let auth = BearerToken::new(&conf.bearer_token);
    let api = TwitterApi::new(auth);
    let indexer_state = self.state.indexer_state().get().await?;

    let mentions = api
      .get_user_mentions(&conf.asami_user_id)
      .since_id(indexer_state.attrs.x_handle_verification_checkpoint.to_u64().unwrap_or(0))
      .max_results(100)
      .user_fields(vec![ UserField::Id, UserField::Username, UserField::PublicMetrics ])
      .expansions(vec![ TweetExpansion::AuthorId, ])
      .send().await?;

    let checkpoint: i64 = mentions.meta()
      .and_then(|m| m.oldest_id.clone() )
      .and_then(|i| i.parse().ok() )
      .unwrap_or(0);

    let mut page = Some(mentions);
    let mut pages = 0;

    while let Some(mentions) = page {
      let payload = mentions.payload();
      let Some(data) = payload.data() else { break };

      for post in data {
        let Some(author_id) = post.author_id else { continue };
        let Some(author) = payload.includes()
          .and_then(|i| i.users.as_ref() )
          .and_then(|i| i.iter().find(|x| x.id == author_id ) )
          else { continue };

        let Some(public_metrics) = author.public_metrics.clone() else { continue };

        if let Some(capture) = msg_regex.captures(&post.text) {
          let Ok(account_id_str) = capture[1].parse::<String>() else { continue };
          let Ok(account_id) = U256::from_dec_str(&account_id_str).map(U256::encode_hex) else { continue };

          let Some(req) = self.state.handle_request().select()
            .status_eq(&HandleRequestStatus::Unverified)
            .site_eq(&Site::X)
            .username_ilike(&author.username)
            .account_id_eq(&account_id)
            .optional().await? else { continue };

          let score = U256::from(public_metrics.followers_count) * wei("85") / wei("100");
          let price = u256(indexer_state.suggested_price_per_point()) * score;
          handle_requests.push(
            req.verify(author_id.to_string()).await?.appraise(price, score).await?
          );
        }
      }

      // We only fetch a backlog of 700 tweets.
      // Older mentions are dropped and should be tried again by the users.
      pages += 1;
      if pages == 5 { break; }
      tokio::time::sleep(Duration::from_millis(3 * 60 * 1000)).await;
      page = mentions.next_page().await?;
    }

    indexer_state.update().x_handle_verification_checkpoint(checkpoint).save().await?;

    Ok(handle_requests)
  }

  pub async fn submit_all(self) -> AsamiResult<()> {
    let rsk = &self.state.on_chain;
    let reqs = self.select().status_eq(HandleRequestStatus::Appraised).all().await?;
    if reqs.is_empty() { return Ok(()); }

    let mut params = vec![];
    for r in &reqs {
      params.push(r.as_param().await?);
    }

    let tx_hash = rsk.contract
      .admin_make_handles(params)
      .send().await?.await?
      .ok_or_else(|| Error::service("rsk_blockchain", "no_tx_recepit_for_admin_make_handles"))?
      .transaction_hash
      .encode_hex();

    for r in reqs {
      r.update().status(HandleRequestStatus::Submitted).tx_hash(Some(tx_hash.clone())).save().await?;
    }

    Ok(())
  }
}

impl HandleRequest {
  pub async fn verify(self, user_id: String) -> sqlx::Result<Self> {
    self.update()
      .user_id(Some(user_id))
      .status(HandleRequestStatus::Verified)
      .save().await
  }

  pub async fn appraise(self, price: U256, score: U256) -> sqlx::Result<Self> {
    self.update()
      .price(Some(price.encode_hex()))
      .score(Some(score.encode_hex()))
      .status(HandleRequestStatus::Appraised)
      .save().await
  }

  pub async fn done(self, handle: &Handle) -> sqlx::Result<Self> {
    self.update().status(HandleRequestStatus::Done)
      .handle_id(Some(handle.attrs.id.clone()))
      .save().await
  }

  pub async fn topic_ids(&self) -> sqlx::Result<Vec<String>> {
    Ok(self.handle_request_topic_vec().await?.into_iter().map(|t| t.attrs.topic_id ).collect())
  }

  pub async fn as_param(&self) -> AsamiResult<on_chain::Handle> {
    let a = &self.attrs;

    let invalid_state = Err(Error::Runtime("Invalid state on appraised handle request".into()));

    let Some(price) = a.price.as_ref().map(u256) else { return invalid_state };
    let Some(score) = a.score.as_ref().map(u256) else { return invalid_state };
    let Some(user_id) = a.user_id.clone() else { return invalid_state };

    let topics = self.topic_ids().await?.iter().map(|i| u256(i) ).collect();

    Ok(on_chain::Handle {
      id: 0.into(),
      account_id: u256(&a.account_id), 
      site: a.site as u8,
      price,
      score,
      topics,
      username: a.username.clone(),
      user_id: user_id
    })
  }
}

model!{
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

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize, sqlx::Type, GraphQLEnum)]
#[sqlx(type_name = "handle_request_status", rename_all = "snake_case")]
pub enum HandleRequestStatus {
  Unverified, // Newly created, never sent on-chain.
  Verified,   // Verified off-chain.
  Appraised,  // Appraised off-chain.
  Submitted,  // Sent, after this we listen for events regarding this handle.
  Done,     // Local DB knows this handle is now on-chain.
}

impl sqlx::postgres::PgHasArrayType for HandleRequestStatus {
  fn array_type_info() -> sqlx::postgres::PgTypeInfo {
    sqlx::postgres::PgTypeInfo::with_name("_handle_request_status")
  }
}
