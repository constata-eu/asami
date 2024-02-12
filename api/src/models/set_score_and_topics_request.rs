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

impl_on_chain_tx_request!{SetScoreAndTopicsRequestHub {
  type Model = SetScoreAndTopicsRequest;
  type Update = UpdateSetScoreAndTopicsRequestHub;
  type Status = GenericRequestStatus;
  type Param = on_chain::AdminSetScoreAndTopicsInput;

  async fn as_param(&self, model: &Self::Model) -> sqlx::Result<Self::Param> {
    let topics = model.set_score_and_topics_request_topic_vec().await?
      .into_iter()
      .map(|t| u256(t.topic_id()) )
      .collect();

    Ok(Self::Param {
      handle_id: u256(&model.attrs.handle_id),
      score: u256(&model.attrs.score),
      topics,
    })
  }

  fn fn_call(&self, params: Vec<Self::Param>) -> AsamiFunctionCall {
    self.state.on_chain.contract.admin_set_score_and_topics(params)
  }

  fn pending_status() -> Self::Status { GenericRequestStatus::Received }
  fn submitted_status() -> Self::Status { GenericRequestStatus::Submitted }
  fn done_status() -> Self::Status { GenericRequestStatus::Done }
}}

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
