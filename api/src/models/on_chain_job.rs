use super::*;
use ethers::{
    middleware::{contract::FunctionCall, Middleware, MiddlewareError},
    prelude::{ContractError, SignerMiddleware, Wallet},
    providers::{Http, Provider},
    signers::LocalWallet,
};
use std::sync::Arc;
use strum::IntoEnumIterator;

mod admin_legacy_claim_account;
mod admin_make_collabs;
mod gasless_claim_balances;
mod promote_sub_accounts;
mod reimburse_campaigns;
mod submit_reports;
mod admin_claim_balances_free;
mod claim_fee_pool_share;
mod apply_voted_fee_rate;

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
    #[sqlx_model_hints(numeric, default)]
    block: Option<Decimal>,
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
    pub async fn run_scheduler(&self) -> anyhow::Result<Vec<OnChainJob>> {
        use OnChainJobStatus::*;

        let mut jobs = vec![];

        let Some(job) = self.current().optional().await? else {
            for kind in OnChainJobKind::iter() {
                jobs.push(self.insert(InsertOnChainJob { kind }).save().await?);
            }
            return Ok(jobs);
        };

        if job.sleep_until() > &Utc::now() {
            return Ok(vec![job]);
        }

        let cloned = job.clone();

        match job.status() {
            Scheduled => {
                job.execute().await?;
            }
            Submitted => {
                job.check_confirmation().await?;
            }
            Confirmed => {
                job.check_settlement().await?;
            }
            _ => {}
        }

        // For now we just schedule all jobs to sleep a fixed period after every action.
        // This could be smarter so we make a job sleep less for an almost immediate retry
        // Or make them sleep longer if we see an issue that may take longer to resolve.
        if let Some(next) = self.current().optional().await? {
            let cooldown = chrono::Duration::milliseconds(
                self.state.settings.rsk.blockchain_sync_cooldown.try_into().unwrap_or(10000),
            );
            next.update().sleep_until(Utc::now() + cooldown).save().await?;
        };

        return Ok(vec![cloned.reloaded().await?]);
    }
}

impl OnChainJob {
    pub async fn execute(self) -> anyhow::Result<Self> {
        use OnChainJobKind::*;
        use OnChainJobStatus::*;

        let maybe_fn_call = match self.kind() {
            PromoteSubAccounts => self.promote_sub_accounts_make_call().await?,
            AdminLegacyClaimAccount => self.admin_legacy_claim_account_make_call().await?,
            MakeCollabs => self.admin_make_collabs_make_call().await?,
            MakeSubAccountCollabs => self.admin_make_sub_account_collabs_make_call().await?,
            SubmitReports => self.submit_reports_make_call().await?,
            ReimburseCampaigns => self.reimburse_campaigns_make_call().await?,
            GaslessClaimBalances => self.gasless_claim_balances_make_call().await?,
            AdminClaimBalancesFree => self.admin_claim_balances_free_make_call().await?,
            ClaimFeePoolShare => self.claim_fee_pool_share_make_call().await?,
            ApplyVotedFeeRate => self.apply_voted_fee_rate_make_call().await?,
        };

        let Some(fn_call) = maybe_fn_call else {
            return Ok(self.update().status(Skipped).save().await?);
        };

        self.info("typed_transaction", &fn_call.tx).await?;

        let executed = match fn_call.send().await {
            Err(e) => {
                let maybe_funds = e
                    .as_middleware_error()
                    .and_then(|m| m.as_error_response())
                    .map(|x| x.message.clone())
                    .unwrap_or_else(|| String::new());

                if maybe_funds.starts_with("insufficient funds") {
                    self.fail("rpc_error_waiting_more", format!("{e:?}")).await?;
                    self.update().status_line(Some(maybe_funds)).save().await?
                } else {
                    let desc = e.decode_revert::<String>().unwrap_or_else(|| format!("no_revert_error"));
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

        Ok(self
            .update()
            .status(status)
            .gas_used(receipt.gas_used.map(|x| x.encode_hex()))
            .nonce(Some(original_tx.nonce.encode_hex()))
            .block(receipt.block_number.and_then(|i| Decimal::from_u64(i.as_u64())))
            .status_line(error_msg)
            .save()
            .await?
            .dispatch_state_change()
            .await?)
    }

    pub async fn check_settlement(self) -> anyhow::Result<Self> {
        use OnChainJobStatus::*;

        let Some((_original_tx, receipt, _error_msg)) = self.get_mined_tx_data().await? else {
            return Ok(self);
        };

        let client = self.state.on_chain.asami_contract.client();

        let Some(block_number) = receipt.block_number else {
            return Ok(self);
        };

        let confs = client.get_block_number().await? - block_number;

        if confs < self.state.settings.rsk.reorg_protection_padding {
            return Ok(self);
        }

        Ok(self.update().status(Settled).save().await?.dispatch_state_change().await?)
    }

    async fn get_mined_tx_data(
        &self,
    ) -> anyhow::Result<
        Option<(
            ethers::core::types::Transaction,
            ethers::core::types::TransactionReceipt,
            Option<String>,
        )>,
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
                    AsamiContractError::from_middleware_error(e)
                        .decode_revert::<String>()
                        .unwrap_or_else(|| format!("non_revert_error"))
                }
                _ => "no_failure_reason_wtf".to_string(),
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
            MakeCollabs => self.admin_make_collabs_on_state_change().await,
            MakeSubAccountCollabs => self.admin_make_collabs_on_state_change().await,
            SubmitReports => self.submit_reports_on_state_change().await,
            _ => Ok(self),
        }
    }

    pub fn contract(&self) -> &on_chain::AsamiContract {
        &self.state.on_chain.asami_contract
    }

    pub async fn link_account(&self, account: &Account) -> sqlx::Result<OnChainJobAccount> {
        self.state
            .on_chain_job_account()
            .insert(InsertOnChainJobAccount {
                job_id: self.attrs.id,
                account_id: account.attrs.id.clone(),
            })
            .save()
            .await
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
        PromoteSubAccounts,      // Admin promotes the sub accounts that requested it.
        AdminLegacyClaimAccount, // The admin claims accounts in the old contract.
        AdminClaimBalancesFree,  // The admin claims its own balances, once in a while.
        GaslessClaimBalances,    // The admin does gasless claims for its known users.
        ReimburseCampaigns,      // As a convenience for users, but they can do it themselves.
        SubmitReports,           // Submit collab report for campaigns that have ended.
        MakeCollabs,             // Register collabs done by full accounts.
        MakeSubAccountCollabs,   // Register collabs done by this admin sub-accounts.
        ClaimFeePoolShare,       // We claim and send their shares to holders automatically.
        ApplyVotedFeeRate,       // The admin does this once per cycle.
    }
];

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
*/
