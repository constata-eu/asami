use super::*;

impl OnChainJob {
    pub async fn reimburse_campaigns_make_call(&self) -> anyhow::Result<Option<AsamiFunctionCall>> {
        let campaigns = self.state.campaign().needing_reimburse().all().await?;

        if campaigns.is_empty() {
            return Ok(None);
        }

        let mut params = vec![];

        for c in &campaigns {
            // To reduce race conditions on campaigns reimbursed by users themselves,
            // we double check here to make sure it still needs reimbursement.
            // This race condition would just revert the TX.
            if c.find_on_chain().await?.budget == u("0") {
                continue;
            }

            self.state
                .on_chain_job_campaign()
                .insert(InsertOnChainJobCampaign {
                    job_id: self.attrs.id,
                    campaign_id: c.attrs.id.clone(),
                })
                .save()
                .await?;

            params.push(on_chain::ReimburseCampaignsParam {
                addr: c.decoded_advertiser_addr()?,
                briefing_hash: c.decoded_briefing_hash(),
            });
        }

        if params.is_empty() {
            return Ok(None);
        }

        return Ok(Some(self.state.on_chain.asami_contract.reimburse_campaigns(params)));
    }
}
