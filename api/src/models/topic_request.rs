use super::*;

model! {
  state: App,
  table: topic_requests,
  struct TopicRequest {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(varchar)]
    name: String,
    #[sqlx_model_hints(int4, default)]
    on_chain_tx_id: Option<i32>,
    #[sqlx_model_hints(generic_request_status, default)]
    status: GenericRequestStatus,
  }
}

impl_on_chain_tx_request! { TopicRequestHub {
  type Model = TopicRequest;
  type Update = UpdateTopicRequestHub;
  type Status = GenericRequestStatus;
  type Param = String;

  async fn as_param(&self, model: &Self::Model) -> sqlx::Result<Self::Param> {
    Ok(model.name().clone())
  }

  fn fn_call(&self, params: Vec<Self::Param>) -> AsamiFunctionCall {
    self.state.on_chain.contract.admin_add_topics(params)
  }

  fn pending_status() -> Self::Status { GenericRequestStatus::Received }
  fn submitted_status() -> Self::Status { GenericRequestStatus::Submitted }
  fn done_status() -> Self::Status { GenericRequestStatus::Done }
}}

impl TopicRequestHub {
  pub async fn create(&self, name: &str) -> sqlx::Result<TopicRequest> {
    self
      .insert(InsertTopicRequest {
        name: name.to_string(),
      })
      .save()
      .await
  }
}
