use super::*;
use ethers::{
  abi::{Address, Detokenize},
  middleware::{contract::FunctionCall, Middleware},
  prelude::{SignerMiddleware, Wallet},
  providers::{Http, Provider},
};
use sqlx_models_orm::{SqlxModel, SqlxModelHub, SqlxSelectModelHub};
use std::{borrow::Borrow, error::Error, sync::Arc};

model! {
  state: App,
  table: on_chain_txs,
  struct OnChainTx {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(on_chain_tx_status, default)]
    status: OnChainTxStatus,
    #[sqlx_model_hints(varchar)]
    function_name: String,
    #[sqlx_model_hints(varchar, default)]
    tx_hash: Option<String>,
    #[sqlx_model_hints(timestamptz, default)]
    created_at: UtcDateTime,
  }
}

impl_loggable!(OnChainTx);

impl OnChainTxHub {
  pub async fn send_tx<B, M, D>(&self, fn_call: FunctionCall<B, M, D>) -> sqlx::Result<OnChainTx>
  where
    B: Borrow<M>,
    M: Middleware,
    D: Detokenize,
  {
    let unsent = self
      .insert(InsertOnChainTx {
        function_name: fn_call.function.name.clone(),
      })
      .save()
      .await?;
    unsent.info("typed_transaction", &fn_call.tx).await?;

    match fn_call.send().await {
      Err(e) => {
        let desc = e.decode_revert::<String>().unwrap_or_else(|| format!("{:?}; {:?}", e, e.source()));
        unsent.fail("contract_error", desc).await?;
        let reverted = unsent.update().status(OnChainTxStatus::Reverted).save().await?;
        Ok(reverted)
      }
      Ok(pending_tx) => {
        let tx_hash = pending_tx.tx_hash().encode_hex();
        let submitted = unsent.update().status(OnChainTxStatus::Submitted).tx_hash(Some(tx_hash)).save().await?;
        Ok(submitted)
      }
    }
  }

  pub async fn apply_handle_updates(&self) -> anyhow::Result<Option<OnChainTx>> {
    let c = &self.state.on_chain.contract;
    let this_cycle = c.get_current_cycle().call().await?;

    let applicable: Vec<U256> = c
      .get_handles()
      .call()
      .await?
      .iter()
      .enumerate()
      .filter_map(|(i, u)| {
        if (u.last_updated == this_cycle) || !u.needs_update {
          None
        } else {
          Some(i.into())
        }
      })
      .take(50)
      .collect();

    if applicable.is_empty() {
      return Ok(None);
    }

    Ok(Some(self.send_tx(c.apply_handle_updates(applicable)).await?))
  }

  pub async fn apply_voted_fee_rate(&self) -> anyhow::Result<Option<OnChainTx>> {
    let c = &self.state.on_chain.contract;
    let this_cycle = c.get_current_cycle().call().await?;
    let last_applied = c.last_voted_fee_rate_applied_on().await?;

    if this_cycle == last_applied {
      return Ok(None);
    }

    Ok(Some(self.send_tx(c.apply_voted_fee_rate()).await?))
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
        return Ok(Some(self.send_tx(c.proclaim_cycle_admin_winner(i.candidate)).await?));
      }
    }

    Ok(None)
  }

  pub async fn reimburse_due_campaigns(&self) -> anyhow::Result<Option<OnChainTx>> {
    let c = &self.state.on_chain.contract;
    let block_number = c.client().get_block_number().await?;
    let Some(now) = c.client().get_block(block_number).await?.map(|b| b.timestamp) else {
      return Ok(None);
    };

    let past_due: Vec<U256> = c
      .get_campaigns()
      .call()
      .await?
      .iter()
      .filter_map(|c| {
        if (c.valid_until >= now) || (c.remaining == u("0")) {
          None
        } else {
          Some(c.id)
        }
      })
      .take(50)
      .collect();

    if past_due.is_empty() {
      return Ok(None);
    }

    Ok(Some(self.send_tx(c.reimburse_due_campaigns(past_due)).await?))
  }

  pub async fn vest_admin_votes(&self) -> anyhow::Result<Option<OnChainTx>> {
    let c = &self.state.on_chain.contract;
    let this_cycle = c.get_current_cycle().call().await?;

    let mut vestable: Vec<Address> = vec![];

    for holder in c.get_holders().call().await? {
      let vote = c.submitted_admin_votes(holder).call().await?;
      if vote.2 > u("0") && vote.2 < this_cycle && !vote.3 {
        vestable.push(holder);
      }

      if vestable.len() > 50 {
        break;
      }
    }

    if vestable.is_empty() {
      return Ok(None);
    }

    Ok(Some(self.send_tx(c.vest_admin_votes(vestable)).await?))
  }

  pub async fn distribute_fee_pool(&self) -> anyhow::Result<Option<OnChainTx>> {
    let c = &self.state.on_chain.contract;
    let this_cycle = c.get_current_cycle().call().await?;

    let total_supply = c.total_supply().call().await?;
    let recent_tokens = c.recent_tokens(this_cycle).call().await?;
    let pool = c.get_fee_pool_before_recent_changes().call().await?;

    if (total_supply - recent_tokens) <= u("0") || pool <= u("0") {
      return Ok(None);
    }

    let mut pending = vec![];

    for holder in c.get_holders().call().await? {
      if this_cycle == c.last_fee_pool_share_cycles(holder).call().await? {
        continue;
      }
      if u("0") == c.get_balance_before_recent_changes(holder).call().await? {
        continue;
      }
      pending.push(holder);

      if pending.len() == 50 {
        break;
      }
    }

    if pending.is_empty() {
      return Ok(None);
    }

    Ok(Some(self.send_tx(c.claim_fee_pool_share(pending)).await?))
  }

  pub async fn sync_tx_result(&self) -> anyhow::Result<()> {
    let client = self.state.on_chain.contract.client();
    for tx in self.select().status_eq(OnChainTxStatus::Submitted).all().await? {
      let Some(tx_hash) = tx.tx_hash() else { continue };
      let Some(receipt) = client.get_transaction_receipt(H256::decode_hex(tx_hash)?).await? else {
        continue;
      };
      let status = if receipt.status.unwrap_or(U64::zero()) == U64::one() {
        OnChainTxStatus::Success
      } else {
        OnChainTxStatus::Failure
      };

      let id = tx.attrs.id;

      self.state.claim_account_request().set_done(id).await?;
      self.state.handle_request().set_done(id).await?;
      self.state.set_score_and_topics_request().set_done(id).await?;
      self.state.set_price_request().set_done(id).await?;
      self.state.campaign_request().set_approved(id).await?;
      self.state.campaign_request().set_done(id).await?;
      self.state.collab_request().set_done(id).await?;
      self.state.topic_request().set_done(id).await?;

      tx.update().status(status).save().await?;
    }
    Ok(())
  }
}

impl OnChainTx {
  pub fn success(&self) -> bool {
    self.attrs.status == OnChainTxStatus::Success
  }

  pub fn reverted(&self) -> bool {
    self.attrs.status == OnChainTxStatus::Success
  }

  pub fn submitted(&self) -> bool {
    self.attrs.status == OnChainTxStatus::Submitted
  }

  pub fn created(&self) -> bool {
    self.attrs.status == OnChainTxStatus::Created
  }
}

pub type AsamiSigner = SignerMiddleware<Provider<Http>, Wallet<ethers::core::k256::ecdsa::SigningKey>>;
pub type AsamiFunctionCall = FunctionCall<Arc<AsamiSigner>, AsamiSigner, ()>;

#[rocket::async_trait]
pub trait OnChainTxRequest: SqlxModelHub<Self::Model> + std::marker::Send + std::marker::Sync {
  type Model: SqlxModel<State = App> + std::marker::Send + std::marker::Sync;
  type Update: std::marker::Send + std::marker::Sync;
  type Status: std::marker::Send + std::marker::Sync;
  type Param: std::marker::Send + std::marker::Sync + serde::Serialize;

  async fn as_param(&self, model: &Self::Model) -> sqlx::Result<Self::Param>;

  fn fn_call(&self, params: Vec<Self::Param>) -> AsamiFunctionCall;

  fn app(&self) -> &App;

  fn update(model: Self::Model) -> Self::Update;

  fn set_update_tx_id(update: Self::Update, txid: i32) -> Self::Update;

  fn set_update_status(update: Self::Update, status: Self::Status) -> Self::Update;

  async fn save_update(update: Self::Update) -> sqlx::Result<Self::Model>;

  fn set_select_status(
    select: <Self::Model as SqlxModel>::SelectModelHub,
    status: Self::Status,
  ) -> <Self::Model as SqlxModel>::SelectModelHub;

  fn set_select_tx_id(
    select: <Self::Model as SqlxModel>::SelectModelHub,
    tx_id: i32,
  ) -> <Self::Model as SqlxModel>::SelectModelHub;

  fn submitted_status() -> Self::Status;
  fn pending_status() -> Self::Status;
  fn done_status() -> Self::Status;
  fn log_kind(&self) -> &str;

  async fn validate_before_submit_all(&self) -> sqlx::Result<()> {
    Ok(())
  }

  async fn info<S: serde::Serialize + Send>(&self, subkind: &str, context: S) {
    self.app().info(self.log_kind(), subkind, context).await;
  }

  async fn submit_all(&self) -> sqlx::Result<Vec<Self::Model>> {
    self.info("submit_all_starting", "").await;

    self.validate_before_submit_all().await?;
    let mut submitted = vec![];
    let reqs = Self::set_select_status(self.select(), Self::pending_status()).all().await?;

    if reqs.is_empty() {
      return Ok(submitted);
    }

    let mut params = vec![];
    for p in &reqs {
      params.push(self.as_param(p).await?);
    }
    self.info("submit_all_params", &params).await;

    let on_chain_tx = self.app().on_chain_tx().send_tx(self.fn_call(params)).await?;
    self.info("submit_all_on_chain_tx", on_chain_tx.id()).await;

    if on_chain_tx.reverted() {
      self.info("submit_all_tx_reverted", on_chain_tx.id()).await;
      return Ok(vec![]);
    }

    for req in reqs {
      let mut update = Self::update(req);
      update = Self::set_update_tx_id(update, on_chain_tx.attrs.id);
      update = Self::set_update_status(update, Self::submitted_status());
      let updated = Self::save_update(update).await?;
      submitted.push(updated);
    }

    self.info("submit_all_is_done", "").await;
    Ok(submitted)
  }

  async fn set_done(&self, tx_id: i32) -> sqlx::Result<()> {
    for r in Self::set_select_tx_id(self.select(), tx_id).all().await? {
      let mut update = Self::update(r);
      update = Self::set_update_status(update, Self::done_status());
      Self::save_update(update).await?;
    }
    Ok(())
  }
}

#[macro_export]
macro_rules! impl_on_chain_tx_request {
  ($struct:ident { $($items:tt)* }) => {
    #[rocket::async_trait]
    impl OnChainTxRequest for $struct {
      $($items)*

      fn app(&self) -> &App { &self.state }
      fn update(model: Self::Model) -> Self::Update { model.update() }
      fn set_update_tx_id(update: Self::Update, txid: i32) -> Self::Update { update.on_chain_tx_id(Some(txid)) }
      fn set_update_status(update: Self::Update, status: Self::Status) -> Self::Update { update.status(status) }
      async fn save_update(update: Self::Update) -> sqlx::Result<Self::Model> { update.save().await }
      fn log_kind(&self) -> &str { stringify!($struct) }

      fn set_select_status(
        select: <Self::Model as sqlx_models_orm::SqlxModel>::SelectModelHub,
        status: Self::Status
      )
        -> <Self::Model as sqlx_models_orm::SqlxModel>::SelectModelHub
      {
        select.status_eq(status)
      }

      fn set_select_tx_id(
        select: <Self::Model as sqlx_models_orm::SqlxModel>::SelectModelHub,
        tx_id: i32
      )
        -> <Self::Model as sqlx_models_orm::SqlxModel>::SelectModelHub
      {
        select.on_chain_tx_id_eq(tx_id)
      }
    }
  };
}

make_sql_enum![
  "on_chain_tx_status",
  pub enum OnChainTxStatus {
    Created,
    Reverted,
    Submitted,
    Failure,
    Success,
  }
];
