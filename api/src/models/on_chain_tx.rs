use super::*;
use ethers::{
  abi::{Detokenize},
  prelude::{abigen, Wallet, SignerMiddleware, LogMeta},
  signers::{MnemonicBuilder, LocalWallet, coins_bip39::English, Signer},
  providers::{Http, Provider},
  middleware::{Middleware, contract::FunctionCall},
};
use std::{ error::Error, borrow::Borrow, sync::Arc};
use sqlx_models_orm::{SqlxModel, SqlxModelHub, SqlxSelectModelHub};

model!{
  state: App,
  table: on_chain_txs,
  struct OnChainTx {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(bool, default)]
    success: bool,
    #[sqlx_model_hints(varchar)]
    function_name: String,
    #[sqlx_model_hints(varchar, default)]
    tx_hash: Option<String>,
    #[sqlx_model_hints(timestamptz, default)]
    created_at: UtcDateTime,
  },
  has_many {
    SetScoreAndTopicsRequest(on_chain_tx_id),
    SetPriceRequest(on_chain_tx_id),
    HandleRequest(on_chain_tx_id),
    CollabRequest(on_chain_tx_id),
  }
}

impl_loggable!(OnChainTx);

impl OnChainTxHub {
  pub async fn send<B,M,D>(&self, fn_call: FunctionCall<B,M,D>) -> sqlx::Result<OnChainTx> 
    where B: Borrow<M>, M: Middleware, D: Detokenize,
  {
    let unsent = self.insert(InsertOnChainTx{ function_name: fn_call.function.name.clone() }).save().await?;
    unsent.info("typed_transaction", &fn_call.tx).await?;

    match fn_call.send().await {
      Err(e) => {
        let desc = e.decode_revert::<String>().unwrap_or_else(|| format!("{:?}; {:?}", e, e.source()));
        unsent.fail("contract_error", desc).await?;
        return Ok(unsent);
      }
      Ok(pending_tx) => {
        let sent = unsent.update().tx_hash(Some(pending_tx.tx_hash().encode_hex())).save().await?;

        match pending_tx.await {
          Err(e) => {
            sent.fail("provider_error", format!("{e:?}")).await?;
            return Ok(sent);
          }
          Ok(receipt) => {
            let success = receipt.as_ref().map(|r| r.status.map(|s| s == U64::one()).unwrap_or(true) ).unwrap_or(false);
            sent.info("receipt", &receipt).await?;
            Ok(sent.update().success(success).save().await?)
          }
        }
      }
    }
  }

  pub async fn apply_handle_updates(&self) -> anyhow::Result<Option<OnChainTx>> {
    let c = &self.state.on_chain.contract;
    let this_cycle = c.get_current_cycle().call().await?;

    let applicable: Vec<U256> = c.get_handles().call().await?
      .iter()
      .enumerate()
      .filter_map(|(i, u)| if (u.last_updated == this_cycle) || !u.needs_update { None } else { Some(i.into()) } )
      .take(50)
      .collect();

    if applicable.is_empty() { return Ok(None); }
    
    Ok(Some(self.send(c.apply_handle_updates(applicable)).await?))
  }

  pub async fn proclaim_cycle_admin_winner(&self) -> anyhow::Result<Option<OnChainTx>> {
    let c = &self.state.on_chain.contract;
    let this_cycle = c.get_current_cycle().call().await?;
    let last_admin_election = c.last_admin_election().call().await?;

    if this_cycle == last_admin_election {
      return Ok(None);
    }

    let vested_votes = c.vested_admin_votes_total().call().await?;
    let admin_vote_counts = c.all_admin_vote_counts().call().await?;

    for i in &admin_vote_counts {
      if i.votes > vested_votes / wei("2") {
        return Ok(Some(self.send(c.proclaim_cycle_admin_winner(i.candidate)).await?));
      }
    }

    Ok(None)
  }
}

/*
 * -> Requests are sent in bulk, logging results.
 *
 * -> Bulk "Submit all" is called in an on-chain transaction, and logged.
 *    -> Bulk submission should be pre-validated when building params.
 *    -> Instances that are not valid may be marked as "failed".
 *       -> Here's where we don't even try to send price, campaign, or claim account requests.
 *    -> If the on-chain tx fails, It should be retried later.
 *       SubmitAll is top level. Runs in a db transaction. It returns anyhow::Result,
 *          The only errors allowed are DB errors.
 *
 *       An app level "submit all" calls all submissions, never fails.
 */

pub type AsamiFunctionCall = FunctionCall<Arc<SignerMiddleware<Provider<Http>, Wallet<ethers::core::k256::ecdsa::SigningKey>>>, SignerMiddleware<Provider<Http>, Wallet<ethers::core::k256::ecdsa::SigningKey>>, ()>;

#[rocket::async_trait(?Send)]
pub trait OnChainTxRequest<Model: SqlxModel<State=App>, Param: Send>: SqlxModelHub<Model> + Sized + Send {
  fn select_to_submit(&self) -> Model::SelectModelHub;

  async fn as_param(&self, model: &Model) -> sqlx::Result<Param>;

  fn fn_call(&self, params: Vec<Param>) -> AsamiFunctionCall;

  async fn set_submitted(&self, model: Model, txid: i32) -> sqlx::Result<Model>;

  async fn set_done(&self, on_chain_tx_id: i32) -> sqlx::Result<()>;

  fn app(&self) -> &App;

  async fn submit_all(&self) -> sqlx::Result<Vec<Model>> {
    let mut submitted = vec![];
    let reqs = self.select_to_submit().all().await?;
    if reqs.is_empty() { return Ok(submitted); }

    let mut params = vec![];
    for p in reqs.iter() {
      params.push(self.as_param(&p).await?);
    }

    let on_chain_tx = self.app().on_chain_tx().send(self.fn_call(params)).await?;

    for req in reqs {
      submitted.push(self.set_submitted(req, on_chain_tx.attrs.id).await?);
    }

    Ok(submitted)
  }
}

/*
pub trait OnChainTxRequest {
  // They are all "sqlx_models::Model (or Hub?)"
  // All have a "submit_all" 
  // All have an "As param" method.
  // All have a query to select all applicable items used for submit_all.
  // All have a pre-validation or condition.
  //
  // All have a "set done" which macro implements as a call to the set_done in the model.
  //
  // Impl macro allows for overriding whatever is needed with regular syntax:(the pre-validation condition, the as_params).
  //
  // Excpetions:
  //  CampaignRequest has its own two flows, maybe it's a custom impl of submit_all where it sends both approvals and submissions.
  // 
  submit_all();
}
*/
