use super::*;

model!{
  state: App,
  table: set_price_requests,
  struct SetPriceRequest {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(varchar)]
    account_id: String,
    #[sqlx_model_hints(varchar)]
    handle_id: String,
    #[sqlx_model_hints(varchar)]
    price: String,
    #[sqlx_model_hints(generic_request_status, default)]
    status: GenericRequestStatus,
    #[sqlx_model_hints(int4, default)]
    on_chain_tx_id: Option<i32>,
  },
  belongs_to {
    OnChainTx(on_chain_tx_id),
    Account(account_id),
  }
}

impl_loggable!(SetPriceRequest);

impl_on_chain_tx_request!{SetPriceRequestHub {
  type Model = SetPriceRequest;
  type Update = UpdateSetPriceRequestHub;
  type Status = GenericRequestStatus;
  type Param = on_chain::AdminSetPriceInput;

  async fn as_param(&self, model: &Self::Model) -> sqlx::Result<Self::Param> {
    Ok(on_chain::AdminSetPriceInput{
      handle_id: u256(model.handle_id()),
      price: u256(model.price()),
    })
  }

  fn fn_call(&self, params: Vec<Self::Param>) -> AsamiFunctionCall {
    self.state.on_chain.contract.admin_set_price(params)
  }

  fn pending_status() -> Self::Status { GenericRequestStatus::Received }
  fn submitted_status() -> Self::Status { GenericRequestStatus::Submitted }
  fn done_status() -> Self::Status { GenericRequestStatus::Done }

  async fn validate_before_submit_all(&self) -> sqlx::Result<()> {
    let pending = self.select().status_not_in(vec![
      GenericRequestStatus::Submitted,
      GenericRequestStatus::Failed,
      GenericRequestStatus::Done,
    ]).all().await?;

    for p in pending {
      if p.account().await?.is_claimed_or_claiming().await? {
        p.fail("cannot_submit_for_claimed_account", &p).await?;
        p.update().status(GenericRequestStatus::Failed).save().await?;
      }
    }
    Ok(())
  }
}}

impl SetPriceRequestHub {
  pub async fn create(&self, handle: &Handle, price: U256) -> AsamiResult<SetPriceRequest> {
    if handle.account().await?.is_claimed_or_claiming().await? {
      return Err(Error::validation("handle", "cannot_set_price_on_claimed_account"));
    }

    Ok(self.insert(InsertSetPriceRequest{
      price: price.encode_hex(),
      account_id: handle.account().await?.attrs.id,
      handle_id: handle.attrs.id.clone(),
    }).save().await?)
  }
}
