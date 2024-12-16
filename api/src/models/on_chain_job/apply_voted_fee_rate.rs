use super::*;

impl OnChainJob {
    pub async fn apply_voted_fee_rate_make_call(&self) -> anyhow::Result<Option<AsamiFunctionCall>> {
        let c = self.contract();

        let current_cycle = c.get_current_cycle().call().await?;
        let last_applied_on = c.last_voted_fee_rate_applied_on().call().await?;

        if current_cycle == last_applied_on {
            return Ok(None);
        }

        let voted = c.voted_fee_rate().call().await?;
        let rate = c.fee_rate().call().await?;

        if voted == rate {
            return Ok(None);
        }

        Ok(Some(c.apply_voted_fee_rate()))
    }
}
