use super::*;

impl OnChainJob {
    pub async fn submit_reports_make_call(&self) -> anyhow::Result<Option<AsamiFunctionCall>> {
        let campaigns = self.state.campaign().needing_report().all().await?;

        if campaigns.is_empty() {
            return Ok(None);
        }

        let mut params = vec![];

        let now = self.state.on_chain.get_timestamp().await?;

        for c in &campaigns {
            // To reduce race conditions on recently extended campaigns, that result in failed
            // transactions, we double check here that this campaign's on-chain
            // state is invalid.
            if c.find_on_chain().await?.valid_until > now {
                continue;
            }

            if c.trusted_admin_addr().await? != self.state.settings.rsk.admin_address {
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

            params.push(on_chain::SubmitReportsParam {
                account: c.decoded_advertiser_addr()?,
                briefing_hash: c.decoded_briefing_hash(),
                report_hash: c.build_report_hash().await?,
            });
        }

        if params.is_empty() {
            return Ok(None);
        }

        return Ok(Some(self.state.on_chain.asami_contract.submit_reports(params)));
    }

    pub async fn submit_reports_on_state_change(self) -> anyhow::Result<Self> {
        use OnChainJobStatus::*;

        match self.status() {
            Settled => {
                for link in self.on_chain_job_campaign_vec().await? {
                    let campaign = link.campaign().await?;
                    let report_hash = campaign.find_on_chain().await?.report_hash;
                    campaign.update().report_hash(Some(report_hash.encode_hex())).save().await?;
                }
            }
            _ => {}
        }

        return Ok(self);
    }
}
