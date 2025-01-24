use super::*;

impl OnChainJob {
    pub async fn gasless_claim_balances_make_call(&self) -> anyhow::Result<Option<AsamiFunctionCall>> {
        // Admin will skip gasless claims if funds are low.
        if self.state.on_chain.admin_rbtc_balance().await? < milli("2") {
            return Ok(None);
        }

        let rbtc_per_user = self.state.settings.rsk.gasless_rbtc_per_user;
        let doc_fee = self.state.settings.rsk.gasless_fee;

        let accounts = self.state.account().select().status_eq(AccountStatus::Claimed).all().await?;

        let mut addresses = std::collections::HashSet::new();

        for a in accounts {
            let Some(found) = a.find_on_chain().await? else {
                continue;
            };

            if !a.allows_gasless() {
                continue;
            }

            let (trusted_admin, max_gasless_doc_to_spend, min_gasless_rbtc_to_receive, _, unclaimed_doc_balance, _, _) =
                found;

            if trusted_admin != Address::default() && trusted_admin != self.state.settings.rsk.admin_address {
                continue;
            }

            if rbtc_per_user < min_gasless_rbtc_to_receive {
                continue;
            }

            if max_gasless_doc_to_spend < doc_fee {
                continue;
            }

            if unclaimed_doc_balance < doc_fee {
                continue;
            }

            let Some(addr) = a.decoded_addr()? else {
                continue;
            };
            addresses.insert(addr);

            self.link_account(&a).await?;

            if addresses.len() > 50 {
                break;
            }

            a.disallow_gasless().await?;
        }

        if addresses.is_empty() {
            return Ok(None);
        }

        let total_rbtc = U256::from(addresses.len()) * rbtc_per_user;

        return Ok(Some(
            self.contract().gasless_claim_balances(doc_fee, rbtc_per_user, Vec::from_iter(addresses)).value(total_rbtc),
        ));
    }

    /// When an OnChainJob for making collabs is done, we sync the collabs and campaigns
    /// state from the blockchain. We do not rely on events for this checks.
    pub async fn gasless_claim_balances_on_state_change(self) -> anyhow::Result<Self> {
        if *self.status() == OnChainJobStatus::Settled {
            self.state.account().hydrate_on_chain_columns_for(
                self.on_chain_job_account_vec().await?.iter().map(|i| i.account_id() )
            ).await?;
        }

        Ok(self)
    }
}
