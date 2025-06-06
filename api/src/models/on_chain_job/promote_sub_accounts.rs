use super::*;

impl OnChainJob {
    pub async fn promote_sub_accounts_make_call(&self) -> AsamiResult<Option<AsamiFunctionCall>> {
        /*
         * Only one job of the same kind will be running at once,
         * So it is safe to assume all accounts in the 'claiming' state are up for grabs
         * for this job. If they weren't, then the smart contract will still just fail the transaction.
         */
        let accounts = self.state.account().select().status_eq(AccountStatus::Claiming).all().await?;

        if accounts.is_empty() {
            return Ok(None);
        }

        for a in &accounts {
            self.link_account(a).await?;
        }

        let params = accounts
            .into_iter()
            .filter_map(|a| {
                Some(on_chain::PromoteSubAccountsParam {
                    id: u256(a.id()),
                    addr: a.decoded_addr().ok()??,
                })
            })
            .collect();

        Ok(Some(self.state.on_chain.asami_contract.promote_sub_accounts(params)))
    }

    pub async fn promote_sub_accounts_on_state_change(self) -> anyhow::Result<Self> {
        if *self.status() == OnChainJobStatus::Settled {
            for link in self.on_chain_job_account_vec().await? {
                link.account().await?.update().status(AccountStatus::Claimed).save().await?;
            }
            self.state
                .account()
                .hydrate_on_chain_columns_for(self.on_chain_job_account_vec().await?.iter().map(|i| i.account_id()))
                .await?;
        }

        Ok(self)
    }
}
