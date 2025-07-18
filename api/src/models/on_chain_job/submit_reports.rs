use std::collections::HashSet;

use super::*;

impl OnChainJob {
    pub async fn submit_reports_make_call(&self) -> anyhow::Result<Option<AsamiFunctionCall>> {
        let campaigns = self.state.campaign().needing_report().all().await?;

        if campaigns.is_empty() {
            return Ok(None);
        }

        let mut params = vec![];

        let now = self.state.on_chain.get_timestamp().await?;

        let mut seen = HashSet::new();

        for c in &campaigns {
            let account = c.decoded_advertiser_addr()?;
            let briefing_hash = c.decoded_briefing_hash();

            // To reduce race conditions on recently extended campaigns, that result in failed
            // transactions, we double check here that this campaign's on-chain
            // state is invalid.
            let on_chain = c.find_on_chain().await?;
            if on_chain.valid_until > now || on_chain.report_hash != u("0") {
                continue;
            }

            if c.trusted_admin_addr().await? != self.state.settings.rsk.admin_address {
                continue;
            }
            let _ = self.info("adding_campaign", (&c, &on_chain)).await;

            // Campaigns could be duplicated if the same user attempts
            // to publish the same campaign twice.
            // This should be prevented elsewhere though.
            // But in any case, it should not be duplicated in the params.
            if seen.contains(&(account, briefing_hash)) {
                continue;
            }

            self.state
                .on_chain_job_campaign()
                .insert(InsertOnChainJobCampaign {
                    job_id: self.attrs.id,
                    campaign_id: c.attrs.id,
                })
                .save()
                .await?;

            params.push(on_chain::SubmitReportsParam {
                account,
                briefing_hash,
                report_hash: c.build_report_hash().await?,
            });

            seen.insert((account, briefing_hash));

            if params.len() == 20 {
                break;
            }
        }

        if params.is_empty() {
            return Ok(None);
        }

        Ok(Some(self.state.on_chain.asami_contract.submit_reports(params)))
    }

    pub async fn submit_reports_on_state_change(self) -> anyhow::Result<Self> {
        if *self.status() == OnChainJobStatus::Settled {
            for link in self.on_chain_job_campaign_vec().await? {
                let campaign = link.campaign().await?;
                let report_hash = campaign.find_on_chain().await?.report_hash;
                campaign.update().report_hash(Some(report_hash.encode_hex())).save().await?;
            }
        }

        Ok(self)
    }
}
