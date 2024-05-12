use super::*;

model! {
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
