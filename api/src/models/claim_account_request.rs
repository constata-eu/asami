use super::*;

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize, sqlx::Type, GraphQLEnum)]
#[sqlx(type_name = "claim_account_request_status", rename_all = "snake_case")]
pub enum ClaimAccountRequestStatus {
  Received,
  Submitted,
  Done,
}

impl sqlx::postgres::PgHasArrayType for ClaimAccountRequestStatus {
  fn array_type_info() -> sqlx::postgres::PgTypeInfo {
    sqlx::postgres::PgTypeInfo::with_name("_claim_account_request_status")
  }
}

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
    #[sqlx_model_hints(varchar, default)]
    tx_hash: Option<String>,
    #[sqlx_model_hints(claim_account_request_status, default)]
    status: ClaimAccountRequestStatus,
  },
  belongs_to {
    Account(account_id),
    Session(account_id),
  }
}

impl ClaimAccountRequestHub {
  pub async fn submit_all(&self) -> AsamiResult<Vec<ClaimAccountRequest>> {
    let mut submitted = vec![];
    let reqs = self.select().status_eq(ClaimAccountRequestStatus::Received).all().await?;

    if reqs.is_empty() { return Ok(submitted); }

    let params = reqs.iter().map(|r| r.as_param() ).collect();

    let tx_hash = self.state.on_chain.contract
      .claim_accounts(params)
      .send().await?.await?
      .ok_or_else(|| Error::service("rsk_blockchain", "no_tx_recepit_for_admin_make_handles"))?
      .transaction_hash
      .encode_hex();

    for req in reqs {
      submitted.push(
        req.update().status(ClaimAccountRequestStatus::Submitted).tx_hash(Some(tx_hash.clone())).save().await?
      );
    }

    Ok(submitted)
  }
}

impl ClaimAccountRequest {
  pub async fn done(self) -> sqlx::Result<Self> {
    self.update().status(ClaimAccountRequestStatus::Done).save().await
  }

  pub fn as_param(&self) -> on_chain::AdminClaimAccountsInput {
    on_chain::AdminClaimAccountsInput {
      account_id: u256(&self.attrs.account_id),
      addr: H160::decode_hex(&self.attrs.addr).unwrap(),
    }
  }
}

