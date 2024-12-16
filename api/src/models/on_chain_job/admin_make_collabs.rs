use super::*;
use crate::on_chain::{
    MakeCollabsParam, MakeCollabsParamItem, MakeSubAccountCollabsParam, MakeSubAccountCollabsParamItem,
};
use std::collections::{HashMap, HashSet};

impl OnChainJob {
    pub async fn admin_make_collabs_make_call(&self) -> anyhow::Result<Option<AsamiFunctionCall>> {
        let by_campaign = self
            .group_and_filter_collabs(|collab| async move {
                Ok(collab.account().await?.decoded_addr()?.map(|account_addr|{
                    MakeCollabsParamItem {
                        account_addr,
                        doc_reward: collab.reward_u256(),
                    }
                }))
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

        Ok(Some(self.state.on_chain.asami_contract.admin_make_collabs(params)))
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

        Ok(Some(
            self.state.on_chain.asami_contract.admin_make_sub_account_collabs(params),
        ))
    }

    /// When an OnChainJob for making collabs is done, we sync the collabs and campaigns
    /// state from the blockchain. We do not rely on events for this checks.
    pub async fn admin_make_collabs_on_state_change(self) -> anyhow::Result<Self> {
        if *self.status() == OnChainJobStatus::Settled {
            let contract = &self.state.on_chain.asami_contract;

            let Some(block) = self.block().and_then(|d| d.to_u64()).map(U64::from) else {
                anyhow::bail!("Expected a valid block number for on chain job {}", self.id());
            };

            let fee_rate = contract.fee_rate().block(block).call().await?;

            // All collabs, by definition, are made by our own admin address, so it's easy
            // to know the admin fee. We still check it on-chain for that block to avoid
            // race conditions.
            let admin_fee_rate =
                contract.accounts(self.state.settings.rsk.admin_address).block(block).call().await?.5; // In this position is for feeRateWhenAdmin

            // After a number of collabs have been made, and before we make the collabs cleared,
            // we first update the campaign budgets, to prevent new collabs coming in racey.
            // We need to update the campaign budgets before moving the collabs to cleared.
            let mut campaigns = HashMap::new();

            for link in self.on_chain_job_collab_vec().await? {
                let campaign = link.collab().await?.campaign().await?;
                campaigns.insert(campaign.attrs.id, campaign);
            }

            for (_, campaign) in &mut campaigns {
                let budget = contract
                    .get_campaign(campaign.decoded_advertiser_addr()?, campaign.decoded_briefing_hash())
                    .block(block)
                    .call()
                    .await?
                    .budget;
                campaign.clone().update().budget(budget.encode_hex()).save().await?;
            }

            let mut account_ids = HashSet::new();
            let mut handle_ids = HashSet::new();

            for link in self.on_chain_job_collab_vec().await? {
                let collab = link.collab().await?;
                account_ids.insert(collab.attrs.member_id.clone());
                account_ids.insert(collab.attrs.advertiser_id.clone());
                handle_ids.insert(collab.attrs.handle_id); 

                // We do this exactly as it's done in the contract to have the same
                // precision loss the contract may have.
                let gross = collab.reward_u256();
                let fee = (gross * fee_rate) / u("100");
                let admin_fee = (gross * admin_fee_rate) / u("100");
                let full_fee = fee + admin_fee;
                collab.update().status(CollabStatus::Cleared).fee(Some(full_fee.encode_hex())).save().await?;
            }

            self.state.account().hydrate_report_columns_for(account_ids.iter()).await?;
            self.state.account().hydrate_on_chain_columns_for(account_ids.iter()).await?;
            self.state.handle().hydrate_report_columns_for(handle_ids.into_iter()).await?;
            self.state.campaign().hydrate_report_columns_for(campaigns.keys().copied()).await?;
        }

        Ok(self)
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
        // We store the budget one more time before trying to send the transaction.
        // If the collabs where registered and not rejected immediately by the X processor
        // due to a stale campaign budget or other unexpected problem, we weed them out here.
        // This is bad for the user, as they should learn about rejectiosn ASAP, but at least
        // we prevent over-budget failed transactions being built.
        let mut campaign_cache: HashMap<i32, Option<(U256, Campaign)>> = HashMap::new();

        let mut params: HashMap<_, Vec<_>> = HashMap::new();

        for collab in all_collabs {
            let cached_campaign = match campaign_cache.get_mut(collab.campaign_id()) {
                Some(x) => x,
                None => {
                    let campaign = collab.campaign().await?;
                    let budget = self.state.on_chain.asami_contract.get_campaign(
                        campaign.decoded_advertiser_addr()?,
                        campaign.decoded_briefing_hash()
                    ).call().await?.budget;

                    let value = if campaign.we_are_trusted_admin().await? {
                        Some((budget,campaign))
                    } else {
                        None
                    };
                    campaign_cache.entry(*collab.campaign_id()).or_insert(value)
                }
            };

            match cached_campaign {
                Some((budget, campaign)) => {
                    if collab.reward_u256() > *budget {
                        self.info(
                            "making_collabs",
                            format!("on double check, collab was over-budget {}", collab.id()),
                        )
                        .await?;
                        collab.update().status(CollabStatus::Failed).save().await?;
                        continue;
                    }

                    *budget -= collab.reward_u256(); 

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
                        format!("we are not campaign trusted admin for collab {}", collab.id()),
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
