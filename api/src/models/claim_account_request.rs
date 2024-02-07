use super::*;

model!{
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

impl ClaimAccountRequestHub {
  pub async fn submit_all(&self) -> anyhow::Result<Vec<ClaimAccountRequest>> {
    let mut submitted = vec![];
    let reqs = self.select().status_eq(GenericRequestStatus::Received).all().await?;

    if reqs.is_empty() { return Ok(submitted); }

    let params = reqs.iter().map(|r| r.as_param() ).collect();

    let on_chain_tx = self.state.on_chain_tx()
      .send(self.state.on_chain.contract.claim_accounts(params))
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

impl ClaimAccountRequest {
  pub async fn done(self) -> sqlx::Result<Self> {
    self.update().status(GenericRequestStatus::Done).save().await
  }

  pub fn as_param(&self) -> on_chain::AdminClaimAccountsInput {
    on_chain::AdminClaimAccountsInput {
      account_id: u256(&self.attrs.account_id),
      addr: H160::decode_hex(&self.attrs.addr).unwrap(),
    }
  }
}
