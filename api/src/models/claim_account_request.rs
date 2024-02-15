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
    OnChainTx(on_chain_tx_id),
  }
}

impl_loggable!(ClaimAccountRequest);

impl_on_chain_tx_request! { ClaimAccountRequestHub {
  type Model = ClaimAccountRequest;
  type Update = UpdateClaimAccountRequestHub;
  type Status = GenericRequestStatus;
  type Param = on_chain::AdminClaimAccountsInput;

  async fn as_param(&self, model: &Self::Model) -> sqlx::Result<Self::Param> {
    Ok(Self::Param {
      account_id: u256(model.account_id()),
      addr: H160::decode_hex(model.addr()).unwrap(),
    })
  }

  fn fn_call(&self, params: Vec<Self::Param>) -> AsamiFunctionCall {
    self.state.on_chain.contract.claim_accounts(params)
  }

  fn pending_status() -> Self::Status { GenericRequestStatus::Received }
  fn submitted_status() -> Self::Status { GenericRequestStatus::Submitted }
  fn done_status() -> Self::Status { GenericRequestStatus::Done }
}}
