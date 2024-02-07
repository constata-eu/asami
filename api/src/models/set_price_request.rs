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

  pub async fn validate_before_submit_all(&self) -> sqlx::Result<()> {
    let pending = self.select().status_not_in(vec![
      GenericRequestStatus::Submitted,
      GenericRequestStatus::Failed,
      GenericRequestStatus::Done,
    ]).all().await?;

    for p in pending {
      if p.account().await?.is_claimed_or_claiming().await? {
        p.mark_failed().await?;
      }
    }
    Ok(())
  }

  pub async fn submit_all(&self) -> sqlx::Result<Vec<SetPriceRequest>> {
    self.validate_before_submit_all().await?;

    let mut submitted = vec![];
    let reqs = self.select().status_eq(GenericRequestStatus::Received).all().await?;
    if reqs.is_empty() { return Ok(submitted); }

    let params = reqs.iter().map(|r| r.as_param() ).collect();

    let on_chain_tx = self.state.on_chain_tx()
      .send(self.state.on_chain.contract.admin_set_price(params))
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

impl SetPriceRequest {
  pub fn as_param(&self) -> on_chain::AdminSetPriceInput {
    on_chain::AdminSetPriceInput{
      handle_id: u256(&self.attrs.handle_id),
      price: u256(&self.attrs.price),
    }
  }

  pub async fn mark_failed(self) -> sqlx::Result<Self> {
    self.fail("cannot_submit_for_claimed_account", &self).await?;
    self.update().status(GenericRequestStatus::Failed).save().await
  }
}
