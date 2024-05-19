use super::*;
use crate::on_chain::{
    MakeCollabsParam, MakeCollabsParamItem, MakeSubAccountCollabsParam, MakeSubAccountCollabsParamItem,
};
use std::collections::HashMap;

impl OnChainJob {
    pub async fn admin_make_collabs_make_call(&self) -> anyhow::Result<Option<AsamiFunctionCall>> {
        let by_campaign = self
            .group_and_filter_collabs(|collab| async move {
                let item = if let Some(account_addr) = collab.account().await?.decoded_addr()? {
                    Some(MakeCollabsParamItem {
                        account_addr,
                        doc_reward: collab.reward_u256(),
                    })
                } else {
                    None
                };
                Ok(item)
            })
            .await?;

        if by_campaign.is_empty() {
            return Ok(None);
        }

        let mut params = vec![];
        for ((advertiser_addr, briefing_hash), collabs) in by_campaign {
            params.push(MakeCollabsParam {
                advertiser_addr,
                briefing_hash,
                collabs,
            });
        }

        return Ok(Some(self.state.on_chain.asami_contract.admin_make_collabs(params)));
    }

    pub async fn admin_make_sub_account_collabs_make_call(&self) -> anyhow::Result<Option<AsamiFunctionCall>> {
        let by_campaign = self
            .group_and_filter_collabs(|collab| async move {
                let item = if collab.account().await?.addr().is_some() {
                    None
                } else {
                    Some(MakeSubAccountCollabsParamItem {
                        sub_account_id: u256(&collab.attrs.member_id),
                        doc_reward: collab.reward_u256(),
                    })
                };
                Ok(item)
            })
            .await?;

        if by_campaign.is_empty() {
            return Ok(None);
        }

        let mut params = vec![];
        for ((advertiser_addr, briefing_hash), collabs) in by_campaign {
            params.push(MakeSubAccountCollabsParam {
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

    // The params to make sub account collabs and regular account collabs are similar.
    // And the collabs to be used for each should follow roughly the same base criteria,
    // The only difference is that collabs by accounts that have an address are
    // to be registered as account collabs, and the others as sub account collabs.
    pub async fn group_and_filter_collabs<F, Fut, T>(
        &self,
        collab_to_item: F,
    ) -> anyhow::Result<HashMap<(Address, U256), Vec<T>>>
    where
        F: Fn(Collab) -> Fut,
        Fut: std::future::Future<Output = anyhow::Result<Option<T>>>,
        T: Clone,
    {
        let all_collabs = self.state.collab().select().status_eq(CollabStatus::Registered).all().await?;

        // Getting a campaign and asserting if we're still the admin is costly.
        // So we cache that here, by campaign_id.
        let mut campaign_cache: HashMap<i32, Option<Campaign>> = HashMap::new();
        let mut params: HashMap<_, Vec<_>> = HashMap::new();

        for collab in all_collabs {
            let cached_campaign = match campaign_cache.get(collab.campaign_id()) {
                Some(x) => x,
                None => {
                    let campaign = collab.campaign().await?;
                    let value = if campaign.we_are_trusted_admin().await? {
                        Some(campaign)
                    } else {
                        None
                    };
                    campaign_cache.entry(*collab.campaign_id()).or_insert(value)
                }
            };

            match cached_campaign {
                Some(campaign) => {
                    let insert = InsertOnChainJobCollab {
                        job_id: self.attrs.id,
                        collab_id: collab.attrs.id,
                    };

                    let Some(item) = collab_to_item(collab).await? else {
                        continue;
                    };

                    self.state.on_chain_job_collab().insert(insert).save().await?;

                    let key = (campaign.decoded_advertiser_addr()?, campaign.decoded_briefing_hash());

                    params.entry(key).and_modify(|v| v.push(item.clone())).or_insert_with(|| vec![item.clone()]);
                }
                None => {
                    self.info(
                        "making_collabs",
                        &format!("we are not campaign trusted admin for collab {}", collab.id()),
                    )
                    .await?;
                    collab.info("making_collabs", "we_are_not_campaign_trusted_admin").await?;
                    collab.update().status(CollabStatus::Failed).save().await?;
                    continue;
                }
            }
        }

        Ok(params)
    }
}
