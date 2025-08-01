use super::*;

impl OnChainJob {
    pub async fn admin_claim_balances_free_make_call(&self) -> anyhow::Result<Option<AsamiFunctionCall>> {
        if self.state.on_chain.admin_rbtc_balance().await? < milli("2") {
            return Ok(None);
        }

        if self.state.on_chain.admin_unclaimed_asami_balance().await? < self.state.settings.rsk.admin_claim_trigger {
            return Ok(None);
        }

        Ok(Some(self.state.on_chain.asami_contract.admin_claim_balances_free(
            vec![self.state.settings.rsk.admin_address],
        )))
    }
}
