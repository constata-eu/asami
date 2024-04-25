use super::*;

model! {
  state: App,
  table: claim_account_requests,
  struct ClaimAccountRequest {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(varchar)]
    account_id: String,
    #[sqlx_model_hints(varchar)]
    addr: String,
    #[sqlx_model_hints(varchar)]
    signature: String,
    #[sqlx_model_hints(varchar)]
    session_id: String,
    #[sqlx_model_hints(int4, default)]
    on_chain_tx_id: Option<i32>,
    #[sqlx_model_hints(generic_request_status, default)]
    status: GenericRequestStatus,
  },
  belongs_to {
    Account(account_id),
    Session(session_id),
  }
}

impl_loggable!(ClaimAccountRequest);
