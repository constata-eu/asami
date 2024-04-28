/// Claiming accounts in the old contract is something we will do during the migration.
/// To make sure that unclaimed DOC and ASAMI rewards in that contract are delivered too.
use super::*;

impl OnChainJob {
  pub async fn admin_legacy_claim_account_make_call(&self) -> AsamiResult<Option<AsamiFunctionCall>> {
    let not_processed = self.state.account().select().addr_is_set(true).processed_for_legacy_claim_eq(false).all().await?;
    
    let mut accounts = vec![];

    /* Accounts that either don't exist in the old contract, or didn't have pending doc
     * do not need to have their claim called in the old contract */
    for account in not_processed {
      let unclaimed_docs = self.state.on_chain.legacy_contract.accounts(u256(account.id())).call().await?.3;
      if unclaimed_docs > wei("0") {
        accounts.push(account);
      } else {
        account.update().processed_for_legacy_claim(true).save().await?;
      }
    }

    if accounts.is_empty() {
      return Ok(None);
    }

    for a in &accounts {
      self.state.on_chain_job_account().insert(InsertOnChainJobAccount{
        job_id: self.attrs.id,
        account_id: a.attrs.id.clone(),
      }).save().await?;
    }

    let params = accounts.into_iter()
      .filter_map(|a| Some(on_chain::AdminClaimAccountsInput{ account_id: u256(a.id()), addr: a.decoded_addr().ok()??}) )
      .collect();
      
    return Ok(Some(self.state.on_chain.legacy_contract.claim_accounts(params)));
  }

  pub async fn admin_legacy_claim_account_on_state_change(self) -> anyhow::Result<Self> {
    use OnChainJobStatus::*;

    match self.status() {
      Settled => {
        for link in self.on_chain_job_account_vec().await? {
          link.account().await?.update().processed_for_legacy_claim(true).save().await?;
        }
      },
      _ => {}
    }

    return Ok(self);
  }
}
