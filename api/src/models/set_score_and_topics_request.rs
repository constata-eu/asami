use super::*;

model!{
  state: App,
  table: set_score_and_topics_requests,
  struct SetScoreAndTopicsRequest {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(varchar)]
    account_id: String,
    #[sqlx_model_hints(varchar)]
    handle_id: String,
    #[sqlx_model_hints(varchar)]
    score: String,
    #[sqlx_model_hints(generic_request_status, default)]
    status: GenericRequestStatus,
    #[sqlx_model_hints(int4, default)]
    on_chain_tx_id: Option<i32>,
  },
  has_many {
    SetScoreAndTopicsRequestTopic(set_score_and_topics_request_id),
  },
  belongs_to {
    OnChainTx(on_chain_tx_id),
  }
}

impl SetScoreAndTopicsRequestHub {
  pub async fn create(&self, handle: &Handle, score: U256, topics: &[&Topic]) -> anyhow::Result<SetScoreAndTopicsRequest> {
    let req = self.insert(InsertSetScoreAndTopicsRequest{
      score: score.encode_hex(),
      account_id: handle.account().await?.attrs.id,
      handle_id: handle.attrs.id.clone(),
    }).save().await?;

    for t in topics {
      self.state.set_score_and_topics_request_topic().insert(InsertSetScoreAndTopicsRequestTopic{
        set_score_and_topics_request_id: req.attrs.id,
        topic_id: t.attrs.id.clone(),
      }).save().await?;
    }

    Ok(req)
  }

  pub async fn submit_all(&self) -> anyhow::Result<Vec<SetScoreAndTopicsRequest>> {
    let mut submitted = vec![];
    let reqs = self.select().status_eq(GenericRequestStatus::Received).all().await?;
    if reqs.is_empty() { return Ok(submitted); }

    let mut params = vec![];
    for r in &reqs {
      params.push(r.as_param().await?);
    }

    let on_chain_tx = self.state.on_chain_tx()
      .send(self.state.on_chain.contract.admin_set_score_and_topics(params))
      .await?;

    for req in reqs {
      submitted.push(
        req.update()
          .status(GenericRequestStatus::Submitted)
          .on_chain_tx_id(Some(on_chain_tx.attrs.id))
          .save().await?
      );
    }

    Ok(submitted)
  }
}

impl SetScoreAndTopicsRequest {
  pub async fn as_param(&self) -> sqlx::Result<on_chain::AdminSetScoreAndTopicsInput> {
    let topics = self.set_score_and_topics_request_topic_vec().await?
      .into_iter()
      .map(|t| u256(t.topic_id()) )
      .collect();

    Ok(on_chain::AdminSetScoreAndTopicsInput{
      handle_id: u256(&self.attrs.handle_id),
      score: u256(&self.attrs.score),
      topics,
    })
  }
}

model!{
  state: App,
  table: set_score_and_topics_request_topics,
  struct SetScoreAndTopicsRequestTopic {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(int4)]
    set_score_and_topics_request_id: i32,
    #[sqlx_model_hints(varchar)]
    topic_id: String,
  }
}
