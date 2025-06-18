use super::*;

impl OnChainJob {
    pub async fn claim_fee_pool_share_make_call(&self) -> anyhow::Result<Option<AsamiFunctionCall>> {
        /* Claiming their fee pool share is something users can do themselves,
         * but as a convenience we do it for them if it makes sense economically
         */
        let c = self.contract();

        let current_cycle = c.get_current_cycle().call().await?;
        let supply = c.total_supply().call().await? - c.recent_tokens(current_cycle).call().await?;
        let pool = c.get_fee_pool_before_recent_changes().call().await?;

        let holders = self.state.holder().select().last_auto_paid_cycle_lt(current_cycle.encode_hex()).all().await?;

        if pool <= u("0") {
            self.info("skipping", "fees pool is empty").await?;
            return Ok(None);
        }
        if supply <= u("0") {
            self.info("skipping", "no supply").await?;
            return Ok(None);
        }

        self.state.fee_pool_snapshot().create_if_missing(current_cycle, pool, supply).await?;

        let mut params = vec![];

        for h in holders {
            let addr = h.decoded_address().context("When claiming fee pool")?;

            let balance = c.get_balance_before_recent_changes(addr).call().await?;

            let reward = balance * pool / supply;

            self.state.estimated_fee_pool_claim().create_if_missing(&h, reward, balance, current_cycle).await?;

            // We don't intentionally burn DOC by paying dividends to contracts.
            // But whatever, if they want to do it they can.
            if *h.is_contract() {
                continue;
            }

            if c.last_fee_pool_share_cycles(addr).call().await? >= current_cycle {
                continue;
            }

            // Accounts with less than 4000 asami tokens don't get their revenue sharing
            // paid by us. They can still claim it themselves.
            if balance <= u("4000") {
                continue;
            }

            self.state
                .on_chain_job_holder()
                .insert(InsertOnChainJobHolder {
                    job_id: self.attrs.id,
                    holder_id: *h.id(),
                })
                .save()
                .await?;

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
        let current_cycle = self.contract().get_current_cycle().call().await?.encode_hex();

        if *self.status() == OnChainJobStatus::Settled {
            let mut ids: Vec<i32> = vec![];
            let mut account_ids: Vec<String> = vec![];

            for rel in self.on_chain_job_holder_vec().await? {
                ids.push(*rel.holder_id());
                let holder = rel.holder().await?;
                if let Some(account) = self.state.account().select().addr_eq(holder.address()).optional().await? {
                    account_ids.push(account.attrs.id);
                }
            }
            self.state
                .db
                .execute(sqlx::query!(
                    "UPDATE holders SET last_auto_paid_cycle = $1 WHERE id = ANY($2)",
                    current_cycle,
                    &ids
                ))
                .await?;

            self.state.holder().hydrate_estimated_total_claims_for(ids.iter().copied()).await?;

            self.state.account().hydrate_on_chain_columns_for(account_ids.iter()).await?;
        }

        Ok(self)
    }
}
