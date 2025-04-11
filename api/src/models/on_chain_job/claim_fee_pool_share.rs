use super::*;

impl OnChainJob {
    pub async fn claim_fee_pool_share_make_call(&self) -> anyhow::Result<Option<AsamiFunctionCall>> {
        /* Claiming their fee pool share is something users can do themselves,
         * but as a convenience we do it for them if it makes sense economically
         */
        let c = self.contract();

        let accounts = self.state.account().select().status_eq(AccountStatus::Claimed).all().await?;

        let current_cycle = c.get_current_cycle().call().await?;
        let supply = c.total_supply().call().await? - c.recent_tokens(current_cycle).call().await?;
        let pool = c.get_fee_pool_before_recent_changes().call().await?;

        if pool <= u("0") {
            self.info("skipping", "fees pool is empty").await?;
            return Ok(None);
        }
        if supply <= u("0") {
            self.info("skipping", "no supply").await?;
            return Ok(None);
        }

        let mut params = vec![];

        for a in accounts {
            let Some(addr) = a.decoded_addr()? else {
                continue;
            };

            if c.last_fee_pool_share_cycles(addr).call().await? >= current_cycle {
                continue;
            }

            if c.get_balance_before_recent_changes(addr).call().await? <= u("4000") {
                continue;
            }

            self.link_account(&a).await?;

            params.push(addr);

            if params.len() == 50 {
                break;
            }
        }

        params.sort();
        params.dedup();

        if params.is_empty() {
            return Ok(None);
        }

        Ok(Some(c.claim_fee_pool_share(params)))
    }

    /// When an OnChainJob for making collabs is done, we sync the collabs and campaigns
    /// state from the blockchain. We do not rely on events for this checks.
    pub async fn claim_fee_pool_share_on_state_change(self) -> anyhow::Result<Self> {
        if *self.status() == OnChainJobStatus::Settled {
            self.state
                .account()
                .hydrate_on_chain_columns_for(self.on_chain_job_account_vec().await?.iter().map(|i| i.account_id()))
                .await?;
        }

        Ok(self)
    }
}
