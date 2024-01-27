use super::*;
use ethers::{
  abi::{Detokenize},
  middleware::{Middleware, contract::FunctionCall},
};
use std::{ error::Error, borrow::Borrow};

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
  }
}

impl OnChainTxHub {
  pub async fn send<B,M,D>(&self, fn_call: FunctionCall<B,M,D>) -> anyhow::Result<OnChainTx> 
    where B: Borrow<M>, M: Middleware, D: Detokenize,
  {
    let unsent = self.insert(InsertOnChainTx{ function_name: fn_call.function.name.clone() }).save().await?;
    // unsent.loginfo("OnChainTxHub::send", "typed_transaction", &fn_call.typed_transaction).await;

    match fn_call.send().await {
      Err(e) => {
        let desc = e.decode_revert::<String>().unwrap_or_else(|| format!("{:?}; {:?}", e, e.source()));
        //unsent.loginfo("OnChainTxHub::send", "contract_error", desc).await;
        return Ok(unsent);
      }
      Ok(pending_tx) => {
        let sent = unsent.update().tx_hash(Some(pending_tx.tx_hash().encode_hex())).save().await?;

        match pending_tx.await {
          Err(e) => {
            //sent.loginfo("OnChainTxHub::send", "provider_error", format!("{:?}", e)).await;
            return Ok(sent);
          }
          Ok(receipt) => {
            let success = receipt.map(|r| r.status.map(|s| s == U64::one()).unwrap_or(true) ).unwrap_or(false);
            //sent.loginfo("OnChainTxHub::send", "receipt", &receipt).await;
            Ok(sent.update().success(success).save().await?)
          }
        }
      }
    }
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
