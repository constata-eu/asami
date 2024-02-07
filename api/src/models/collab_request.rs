use super::*;
use std::borrow::Borrow;
use ethers::{prelude::FunctionCall, abi::Detokenize};

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
    #[sqlx_model_hints(varchar, default)]
    collab_id: Option<String>,
    #[sqlx_model_hints(generic_request_status, default)]
    status: GenericRequestStatus,
  },
  belongs_to {
    Campaign(campaign_id),
    Handle(handle_id),
    Collab(collab_id),
    OnChainTx(on_chain_tx_id),
  }
}

use async_trait::async_trait;
use std::pin::Pin;
use futures::stream::Stream;


#[rocket::async_trait(?Send)]
impl OnChainTxRequest<CollabRequest, on_chain::AdminMakeCollabsInput> for CollabRequestHub {
  fn select_to_submit(&self) -> SelectCollabRequestHub {
    self.state.collab_request().select().status_eq(GenericRequestStatus::Received)
  }

  async fn as_param(&self, model: &CollabRequest) -> sqlx::Result<on_chain::AdminMakeCollabsInput> {
    Ok(on_chain::AdminMakeCollabsInput{
      campaign_id: u256(&model.attrs.campaign_id),
      handle_id: u256(&model.attrs.handle_id),
    })
  }

  fn fn_call(&self, params: Vec<on_chain::AdminMakeCollabsInput>) -> AsamiFunctionCall {
    self.state.on_chain.contract.admin_make_collabs(params)
  }

  async fn set_submitted(&self, model: CollabRequest, txid: i32) -> sqlx::Result<CollabRequest> {
    model.update().status(GenericRequestStatus::Submitted).on_chain_tx_id(Some(txid)).save().await
  }

  async fn set_done(&self, on_chain_tx_id: i32 ) -> sqlx::Result<()> {
    for x in self.select().on_chain_tx_id_eq(on_chain_tx_id).all().await? {
      x.update().status(GenericRequestStatus::Done).save().await?;
    }
    Ok(())
  }

  fn app(&self) -> &App {
    &self.state
  }
}

/*
impl CollabRequestHub {
  pub async fn submit_all(&self) -> anyhow::Result<Vec<CollabRequest>> {
    let mut submitted = vec![];
    let reqs = self.select().status_eq(GenericRequestStatus::Received).all().await?;
    if reqs.is_empty() { return Ok(submitted); }

    let params = reqs.iter().map(|r| r.as_param() ).collect();

    let on_chain_tx = self.state.on_chain_tx()
      .send(self.state.on_chain.contract.admin_make_collabs(params))
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
*/

impl CollabRequest {
  pub async fn done(self, collab: &Collab) -> sqlx::Result<Self> {
    self.update().status(GenericRequestStatus::Done)
      .collab_id(Some(collab.attrs.id.clone()))
      .save().await
  }

  pub fn as_param(&self) -> on_chain::AdminMakeCollabsInput {
    on_chain::AdminMakeCollabsInput{
      campaign_id: u256(&self.attrs.campaign_id),
      handle_id: u256(&self.attrs.handle_id),
    }
  }
}

/*
impl CollabRequestHub {
  pub async fn submit_all(&self) -> anyhow::Result<Vec<CollabRequest>> {
    let mut submitted = vec![];
    let reqs = self.select().status_eq(GenericRequestStatus::Received).all().await?;
    if reqs.is_empty() { return Ok(submitted); }

    let params = reqs.iter().map(|r| r.as_param() ).collect();

    let on_chain_tx = self.state.on_chain_tx()
      .send(self.state.on_chain.contract.admin_make_collabs(params))
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

impl CollabRequest {
  pub async fn done(self, collab: &Collab) -> sqlx::Result<Self> {
    self.update().status(GenericRequestStatus::Done)
      .collab_id(Some(collab.attrs.id.clone()))
      .save().await
  }

  pub fn as_param(&self) -> on_chain::AdminMakeCollabsInput {
    on_chain::AdminMakeCollabsInput{
      campaign_id: u256(&self.attrs.campaign_id),
      handle_id: u256(&self.attrs.handle_id),
    }
  }
}
*/
