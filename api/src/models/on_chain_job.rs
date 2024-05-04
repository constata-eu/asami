use super::*;
use strum::IntoEnumIterator;
use ethers::{
  middleware::{contract::FunctionCall, Middleware, MiddlewareError},
  prelude::{SignerMiddleware, Wallet, ContractError},
  signers::LocalWallet,
  providers::{Http, Provider},
};
use std::sync::Arc;

mod promote_sub_accounts;
mod admin_legacy_claim_account;

pub type AsamiSigner = SignerMiddleware<Provider<Http>, Wallet<ethers::core::k256::ecdsa::SigningKey>>;
pub type AsamiFunctionCall = FunctionCall<Arc<AsamiSigner>, AsamiSigner, ()>;
pub type AsamiContractError = ContractError<SignerMiddleware<Provider<Http>, LocalWallet>>;

model! {
  state: App,
  table: on_chain_jobs,
  struct OnChainJob {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(on_chain_job_status, default)]
    status: OnChainJobStatus,
    #[sqlx_model_hints(on_chain_job_kind)]
    kind: OnChainJobKind,
    #[sqlx_model_hints(varchar, default)]
    tx_hash: Option<String>,
    #[sqlx_model_hints(varchar, default)]
    gas_used: Option<String>,
    #[sqlx_model_hints(varchar, default)]
    nonce: Option<String>,
    #[sqlx_model_hints(varchar, default)]
    status_line: Option<String>,
    #[sqlx_model_hints(timestamptz, default)]
    sleep_until: UtcDateTime,
    #[sqlx_model_hints(timestamptz, default)]
    created_at: UtcDateTime,
  },
  queries {
      current("status IN ('scheduled', 'submitted', 'confirmed') ORDER BY id limit 1")
  },
  has_many {
    OnChainJobAccount(job_id),
    OnChainJobCampaign(job_id),
    OnChainJobCollab(job_id),
    OnChainJobHolder(job_id),
  }
}

impl_loggable!(OnChainJob);

impl OnChainJobHub {
  pub async fn run_scheduler(&self) -> anyhow::Result<()> {
    use OnChainJobStatus::*;

    let Some(job) = self.current().optional().await? else {
      for kind in OnChainJobKind::iter() {
        self.insert(InsertOnChainJob{ kind }).save().await?;
      }
      return Ok(());
    };

    if job.sleep_until() > &Utc::now() {
      return Ok(());
    }

    match job.status() {
      Scheduled => { job.execute().await?; },
      Submitted => { job.check_confirmation().await?; },
      Confirmed => { job.check_settlement().await?; },
      _ => {}
    }

    // For now we just schedule all jobs to sleep a fixed period after every action.
    // This could be smarter so we make a job sleep less for an almost immediate retry
    // Or make them sleep longer if we see an issue that may take longer to resolve.
    if let Some(next) = self.current().optional().await? {
      let cooldown = chrono::Duration::milliseconds(
        self.state.settings.rsk.blockchain_sync_cooldown.try_into().unwrap_or(10000)
      );
      next.update().sleep_until(Utc::now() + cooldown).save().await?;
    };

    return Ok(());
  }

}

impl OnChainJob {
  pub async fn execute(self) -> anyhow::Result<Self> {
    use OnChainJobKind::*;
    use OnChainJobStatus::*;

    let maybe_fn_call = match self.kind() {
      PromoteSubAccounts => self.promote_sub_accounts_make_call().await?,
      AdminLegacyClaimAccount => self.admin_legacy_claim_account_make_call().await?,
      /*
      AdminClaimOwnBalances, // The admin claims its own balances, once in a while.
      AdminClaimGaslessBalances, // The admin does gasless claims for its known users.
      ReimburseCampaigns, // As a convenience for users, but the admin is not obliged to reimburse.
      SubmitReports, // Submit collab report for campaigns that have ended.
      MakeCollabs, // Register collabs done by full accounts.
      MakeSubAccountCollabs, // Register collabs done by this admin sub-accounts.
      ClaimFeePoolShare, // We claim and send their shares to holders automatically.
      ApplyVotedFeeRate, // The admin does this once per cycle.
      */
      _ => None
    };
    
    let Some(fn_call) = maybe_fn_call else {
        return Ok(self.update().status(Skipped).save().await?);
    };

    self.info("typed_transaction", &fn_call.tx).await?;

    let executed = match fn_call.send().await {
      Err(e) => {
        let maybe_funds = e.as_middleware_error()
            .and_then(|m| m.as_error_response())
            .map(|x| x.message.clone() )
            .unwrap_or_else(|| String::new());

        if maybe_funds.starts_with("insufficient funds") {
            self.fail("rpc_error_waiting_more", format!("{e:?}")).await?;
            self.update().status_line(Some(maybe_funds)).save().await?

        } else {
            let desc = e.decode_revert::<String>().unwrap_or_else(|| format!("no_revert_error") );
            self.fail("early_revert_message", format!("{e:?}")).await?;
            self.update().status(Reverted).status_line(Some(desc)).save().await?
        }
      }
      Ok(pending_tx) => {
        let tx_hash = pending_tx.tx_hash().encode_hex();
        self.update().status(Submitted).tx_hash(Some(tx_hash)).save().await?
      }
    };

    Ok(executed)
  }

  pub async fn check_confirmation(self) -> anyhow::Result<Self> {
    use OnChainJobStatus::*;

    let Some((original_tx, receipt, error_msg)) = self.get_mined_tx_data().await? else {
      return Ok(self);
    };

    let status = if receipt.status.unwrap_or(U64::zero()) == U64::one() {
      Confirmed
    } else {
      Failed
    };

    Ok(self.update().status(status)
      .gas_used(receipt.gas_used.map(|x| x.encode_hex() ))
      .nonce(Some(original_tx.nonce.encode_hex()))
      .status_line(error_msg)
      .save().await?
      .dispatch_state_change()
      .await?
    )
  }

  pub async fn check_settlement(self) -> anyhow::Result<Self> {
    use OnChainJobStatus::*;

    let Some((_original_tx, receipt, _error_msg)) = self.get_mined_tx_data().await? else {
      return Ok(self);
    };

    let client = self.state.on_chain.asami_contract.client();

    let Some(block_number) = receipt.block_number else { return Ok(self); };

    let confs = client.get_block_number().await? - block_number;

    if confs < self.state.settings.rsk.reorg_protection_padding {
      return Ok(self);
    }

    Ok(
      self.update().status(Settled)
        .save().await?
        .dispatch_state_change()
        .await?
    )
  }

  async fn get_mined_tx_data(&self) -> anyhow::Result<
    Option<(
      ethers::core::types::Transaction,
      ethers::core::types::TransactionReceipt,
      Option<String>
    )>
  > {
    let client = self.state.on_chain.asami_contract.client();

    let Some(tx_hash) = self.tx_hash().as_ref().and_then(|x| H256::decode_hex(x).ok()) else { 
      self.fail("invaild_tx_hash_in_db", self.tx_hash()).await?;
      anyhow::bail!("invalid_tx_hash_in_db");
    };

    let Some(original_tx) = client.get_transaction(tx_hash).await? else {
      self.fail("tx_not_found_on_explorer", self.tx_hash()).await?;
      anyhow::bail!("tx_not_found_on_explorer");
    };

    let Some(receipt) = client.get_transaction_receipt(tx_hash).await? else {
      self.fail("receipt_not_found_on_explorer", tx_hash).await?;
      return Ok(None);
    };

    let error_message = if receipt.status.unwrap_or(U64::zero()) == U64::one() {
      None
    } else {
      let typed: ethers::types::transaction::eip2718::TypedTransaction = (&original_tx).into();
      let msg = match client.call(&typed, None).await {
        Err(e) => {
          self.fail("full_failure_message", format!("{e:?}")).await?;
          AsamiContractError::from_middleware_error(e).decode_revert::<String>().unwrap_or_else(|| format!("non_revert_error"))
        },
        _ => "no_failure_reason_wtf".to_string()
      };
      Some(msg)
    };

    Ok(Some((original_tx, receipt, error_message)))
  }

  pub async fn dispatch_state_change(self) -> anyhow::Result<Self> {
    use OnChainJobKind::*;

    match self.kind() {
      PromoteSubAccounts => self.promote_sub_accounts_on_state_change().await,
      AdminLegacyClaimAccount => self.admin_legacy_claim_account_on_state_change().await,
      /*
      AdminClaimOwnBalances, // The admin claims its own balances, once in a while.
      AdminClaimGaslessBalances, // The admin does gasless claims for its known users.
      ReimburseCampaigns, // As a convenience for users, but the admin is not obliged to reimburse.
      SubmitReports, // Submit collab report for campaigns that have ended.
      MakeCollabs, // Register collabs done by full accounts.
      MakeSubAccountCollabs, // Register collabs done by this admin sub-accounts.
      ClaimFeePoolShare, // We claim and send their shares to holders automatically.
      ApplyVotedFeeRate, // The admin does this once per cycle.
      */
      _ => todo!("Not implemented yet"),
    }
  }
}

model! {
  state: App,
  table: on_chain_job_accounts,
  struct OnChainJobAccount {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(int4)]
    job_id: i32,
    #[sqlx_model_hints(varchar)]
    account_id: String,
  },
  belongs_to {
    OnChainJob(job_id),
    Account(account_id),
  }
}

model! {
  state: App,
  table: on_chain_job_campaigns,
  struct OnChainJobCampaign {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(int4)]
    job_id: i32,
    #[sqlx_model_hints(int4)]
    campaign_id: i32,
  },
  belongs_to {
    OnChainJob(job_id),
    Campaign(campaign_id),
  }
}

model! {
  state: App,
  table: on_chain_job_collabs,
  struct OnChainJobCollab {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(int4)]
    job_id: i32,
    #[sqlx_model_hints(int4)]
    collab_id: i32,
  },
  belongs_to {
    OnChainJob(job_id),
    Collab(collab_id),
  }
}

model! {
  state: App,
  table: on_chain_job_holders,
  struct OnChainJobHolder {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(int4)]
    job_id: i32,
    #[sqlx_model_hints(int4)]
    holder_id: i32,
  },
  belongs_to {
    OnChainJob(job_id),
    Holder(holder_id),
  }
}

make_sql_enum![
  "on_chain_job_status",
  pub enum OnChainJobStatus {
    Scheduled,
    Skipped,
    Reverted,
    Submitted,
    Failed,
    Confirmed,
    Settled,
  }
];

make_sql_enum![
  "on_chain_job_kind",
  pub enum OnChainJobKind {
    PromoteSubAccounts, // Admin promotes the sub accounts that requested it.
    AdminLegacyClaimAccount, // The admin claims accounts in the old contract.
    AdminClaimOwnBalances, // The admin claims its own balances, once in a while.
    AdminClaimGaslessBalances, // The admin does gasless claims for its known users.
    ReimburseCampaigns, // As a convenience for users, but the admin is not obliged to reimburse.
    SubmitReports, // Submit collab report for campaigns that have ended.
    MakeCollabs, // Register collabs done by full accounts.
    MakeSubAccountCollabs, // Register collabs done by this admin sub-accounts.
    ClaimFeePoolShare, // We claim and send their shares to holders automatically.
    ApplyVotedFeeRate, // The admin does this once per cycle.
  }
];

  /*
  pub async fn send_tx<B, M, D>(&self, fn_call: FunctionCall<B, M, D>) -> sqlx::Result<OnChainJob>
  where
    B: Borrow<M>,
    M: Middleware,
    D: Detokenize,
  {
    let unsent = self
      .insert(InsertOnChainJob {
        function_name: fn_call.function.name.clone(),
      })
      .save()
      .await?;
    unsent.info("typed_transaction", &fn_call.tx).await?;

    match fn_call.send().await {
      Err(e) => {
        let desc = e.decode_revert::<String>().unwrap_or_else(|| format!("no_revert_error") );
        unsent.fail("early_revert_message", format!("{e:?}")).await?;
        let reverted = unsent.update().status(OnChainJobStatus::Reverted).message(Some(desc)).save().await?;
        Ok(reverted)
      }
      Ok(pending_tx) => {
        let tx_hash = pending_tx.tx_hash().encode_hex();
        let submitted = unsent.update().status(OnChainJobStatus::Submitted).tx_hash(Some(tx_hash)).save().await?;
        Ok(submitted)
      }
    }
  }
  */

  /*
  pub async fn apply_voted_fee_rate(&self) -> anyhow::Result<Option<OnChainJob>> {
    let c = &self.state.on_chain.asami_contract;
    let this_cycle = c.get_current_cycle().call().await?;
    let last_applied = c.last_voted_fee_rate_applied_on().await?;

    if this_cycle == last_applied {
      return Ok(None);
    }

    Ok(Some(self.send_tx(c.apply_voted_fee_rate()).await?))
  }

  pub async fn reimburse_due_campaigns(&self) -> anyhow::Result<Option<OnChainJob>> {
    let c = &self.state.on_chain.asami_contract;
    let block_number = c.client().get_block_number().await?;
    let Some(now) = c.client().get_block(block_number).await?.map(|b| b.timestamp) else {
      return Ok(None);
    };

    let candidates: Vec<Campaign> = self.state.campaign()
      .select()
      .budget_gt(weihex("0"))
      .limit(50)
      .all().await?;

    let mut past_due = vec![];

    for c in &candidates {
      if c.valid_until() > &i_to_utc(now) {
          continue;
      }
      let Some(addr) = c.account().await?.decoded_addr()? else { continue };
      past_due.push(on_chain::ReimburseCampaignsParam{ addr, briefing_hash: u256(c.briefing_hash())});
    }

    if past_due.is_empty() {
      return Ok(None);
    }

    Ok(Some(self.send_tx(c.reimburse_campaigns(past_due)).await?))
  }

  pub async fn distribute_fee_pool(&self) -> anyhow::Result<Option<OnChainJob>> {
    let c = &self.state.on_chain.asami_contract;
    let this_cycle = c.get_current_cycle().call().await?;

    let total_supply = c.total_supply().call().await?;
    let recent_tokens = c.recent_tokens(this_cycle).call().await?;
    let pool = c.get_fee_pool_before_recent_changes().call().await?;

    if (total_supply - recent_tokens) <= u("0") || pool <= u("0") {
      return Ok(None);
    }

    let holders: Vec<Address> = self.state.holder().select()
        .last_fee_pool_share_ne(this_cycle.encode_hex())
        .balance_ne(weihex("0"))
        .limit(50)
        .all().await?
        .into_iter()
        .filter_map(|h| Address::decode_hex(h.attrs.address).ok() )
        .collect();

    if holders.is_empty() {
      return Ok(None);
    }

    Ok(Some(self.send_tx(c.claim_fee_pool_share(holders)).await?))
  }

  pub async fn sync_tx_result(&self) -> anyhow::Result<()> {
    let client = self.state.on_chain.asami_contract.client();
    for tx in self.select().status_eq(OnChainJobStatus::Submitted).all().await? {
      let Some(tx_hash) = tx.tx_hash().as_ref().and_then(|x| H256::decode_hex(x).ok()) else { 
        continue
      };
      let Some(original_tx) = client.get_transaction(tx_hash).await? else {
        continue;
      };
      let Some(receipt) = client.get_transaction_receipt(tx_hash).await? else {
        continue;
      };
      let (status, message) = if receipt.status.unwrap_or(U64::zero()) == U64::one() {
        (OnChainJobStatus::Success, None)
      } else {
        let typed: ethers::types::transaction::eip2718::TypedTransaction = (&original_tx).into();
        let msg = match client.call(&typed, None).await {
          Err(e) => {
            tx.fail("full_failure_message", format!("{e:?}")).await?;
            AsamiCoreContractError::from_middleware_error(e).decode_revert::<String>().unwrap_or_else(|| format!("non_revert_error"))
          },
          _ => "no_failure_reason_wtf".to_string()
        };
        (OnChainJobStatus::Failure, Some(msg))
      };

      //let id = tx.attrs.id;

      /*
      self.state.claim_account_request().set_done(id).await?;
      self.state.handle_request().set_done(id).await?;
      self.state.set_score_and_topics_request().set_done(id).await?;
      self.state.set_price_request().set_done(id).await?;
      self.state.campaign_request().set_approved(id).await?;
      self.state.campaign_request().set_done(id).await?;
      self.state.collab_request().set_done(id).await?;
      self.state.topic_request().set_done(id).await?;
      */

      tx.update().status(status)
        .gas_used(receipt.gas_used.map(|x| x.encode_hex() ))
        .nonce(Some(original_tx.nonce.encode_hex()))
        .message(message)
        .save().await?;
    }
    Ok(())
  }
  */
