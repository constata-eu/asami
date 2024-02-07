use super::*;

model!{
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

impl TopicRequestHub {
  pub async fn create(&self, name: &str) -> sqlx::Result<TopicRequest> {
    self.insert(InsertTopicRequest{ name: name.to_string() }).save().await
  }

  pub async fn submit_all(&self) -> anyhow::Result<Vec<TopicRequest>> {
    let mut submitted = vec![];
    let reqs = self.select().status_eq(GenericRequestStatus::Received).all().await?;
    if reqs.is_empty() { return Ok(submitted); }

    let params = reqs.iter().map(|r| r.attrs.name.clone() ).collect();

    let on_chain_tx = self.state.on_chain_tx()
      .send(self.state.on_chain.contract.admin_add_topics(params))
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
