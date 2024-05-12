use super::*;
use std::collections::HashMap;

impl OnChainJob {
    pub async fn admin_make_collabs_make_call(&self) -> AsamiResult<Option<AsamiFunctionCall>> {
        let all_collabs = self.state.collab().select().status_eq(CollabStatus::Registered).all().await?;

        let mut by_campaign: HashMap<(Address, U256), Vec<_>> = HashMap::new();
        for collab in &all_collabs {
            let Some(member_addr) = collab.account().await?.decoded_addr()? else {
                continue;
            };

            let campaign = collab.campaign().await?;
            let key = (campaign.decoded_advertiser_addr()?, u256(campaign.attrs.briefing_hash));
            by_campaign
                .entry(key)
                .and_modify(|v| v.push((collab, member_addr)))
                .or_insert(vec![(collab, member_addr)]);
        }

        if by_campaign.is_empty() {
            return Ok(None);
        }

        let mut params = vec![];

        for ((advertiser_addr, briefing_hash), db_collabs) in by_campaign {
            let mut collabs = vec![];

            for (collab, account_addr) in db_collabs {
                self.state
                    .on_chain_job_collab()
                    .insert(InsertOnChainJobCollab {
                        job_id: self.attrs.id,
                        collab_id: collab.attrs.id.clone(),
                    })
                    .save()
                    .await?;

                collabs.push(on_chain::MakeCollabsParamItem {
                    account_addr,
                    doc_reward: u256(&collab.attrs.reward),
                });
            }

            params.push(on_chain::MakeCollabsParam {
                advertiser_addr,
                briefing_hash,
                collabs,
            });
        }

        return Ok(Some(self.state.on_chain.asami_contract.admin_make_collabs(params)));
    }

    pub async fn admin_make_sub_account_collabs_make_call(&self) -> AsamiResult<Option<AsamiFunctionCall>> {
        let all_collabs = self.state.collab().select().status_eq(CollabStatus::Registered).all().await?;

        let mut by_campaign: HashMap<(Address, U256), Vec<_>> = HashMap::new();
        for collab in &all_collabs {
            if collab.account().await?.addr().is_some() {
                continue;
            }
            let campaign = collab.campaign().await?;
            let key = (campaign.decoded_advertiser_addr()?, u256(campaign.attrs.briefing_hash));
            by_campaign.entry(key).and_modify(|v| v.push(collab.clone())).or_insert(vec![collab.clone()]);
        }

        if by_campaign.is_empty() {
            return Ok(None);
        }

        let mut params = vec![];

        for ((advertiser_addr, briefing_hash), db_collabs) in by_campaign {
            let mut collabs = vec![];

            for collab in db_collabs {
                self.state
                    .on_chain_job_collab()
                    .insert(InsertOnChainJobCollab {
                        job_id: self.attrs.id,
                        collab_id: collab.attrs.id.clone(),
                    })
                    .save()
                    .await?;

                collabs.push(on_chain::MakeSubAccountCollabsParamItem {
                    sub_account_id: u256(collab.attrs.member_id),
                    doc_reward: u256(collab.attrs.reward),
                });
            }

            params.push(on_chain::MakeSubAccountCollabsParam {
                advertiser_addr,
                briefing_hash,
                collabs,
            });
        }

        return Ok(Some(
            self.state.on_chain.asami_contract.admin_make_sub_account_collabs(params),
        ));
    }

    /// When an OnChainJob for making collabs is done, we sync the collabs and campaigns
    /// state from the blockchain. We do not rely on events for this checks.
    pub async fn admin_make_collabs_on_state_change(self) -> anyhow::Result<Self> {
        use OnChainJobStatus::*;

        match self.status() {
            Settled => {
                let contract = &self.state.on_chain.asami_contract;

                let Some(block) = self.block().and_then(|d| d.to_u64()).map(|u| U64::from(u)) else {
                    anyhow::bail!("Expected a valid block number for on chain job {}", self.id());
                };

                let fee_rate = contract.fee_rate().block(block).call().await?;

                // All collabs, by definition, are made by our own admin address, so it's easy
                // to know the admin fee. We still check it on-chain for that block to avoid
                // race conditions.
                let admin_fee_rate =
                    contract.accounts(self.state.settings.rsk.admin_address).block(block).call().await?.5; // In this position is for feeRateWhenAdmin

                let mut campaigns = HashMap::new();

                for link in self.on_chain_job_collab_vec().await? {
                    let collab = link.collab().await?;
                    let campaign = collab.campaign().await?;
                    campaigns.insert(campaign.attrs.id.clone(), campaign);

                    let fee = collab.reward_u256() * (fee_rate + admin_fee_rate);
                    collab.update().status(CollabStatus::Cleared).fee(Some(fee.encode_hex())).save().await?;
                }

                for (_, campaign) in campaigns {
                    let budget = contract
                        .get_campaign(campaign.decoded_advertiser_addr()?, campaign.decoded_briefing_hash())
                        .block(block)
                        .call()
                        .await?
                        .budget;
                    campaign.update().budget(budget.encode_hex()).save().await?;
                }
            }
            _ => {}
        }

        return Ok(self);
    }
}
