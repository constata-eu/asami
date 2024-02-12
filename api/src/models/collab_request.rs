use super::*;

model!{
  state: App,
  table: collab_requests,
  struct CollabRequest {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(varchar)]
    campaign_id: String,
    #[sqlx_model_hints(varchar)]
    handle_id: String, 
    #[sqlx_model_hints(int4, default)]
    on_chain_tx_id: Option<i32>,
    #[sqlx_model_hints(generic_request_status, default)]
    status: GenericRequestStatus,
  },
  belongs_to {
    Campaign(campaign_id),
    Handle(handle_id),
    OnChainTx(on_chain_tx_id),
  }
}

impl_on_chain_tx_request!{ CollabRequestHub {
  type Model = CollabRequest;
  type Update = UpdateCollabRequestHub;
  type Status = GenericRequestStatus;
  type Param = on_chain::AdminMakeCollabsInput;

  async fn as_param(&self, model: &Self::Model) -> sqlx::Result<Self::Param> {
    Ok(Self::Param {
      campaign_id: u256(&model.attrs.campaign_id),
      handle_id: u256(&model.attrs.handle_id),
    })
  }

  fn fn_call(&self, params: Vec<Self::Param>) -> AsamiFunctionCall {
    self.state.on_chain.contract.admin_make_collabs(params)
  }

  fn pending_status() -> Self::Status { GenericRequestStatus::Received }
  fn submitted_status() -> Self::Status { GenericRequestStatus::Submitted }
  fn done_status() -> Self::Status { GenericRequestStatus::Done }
}}

