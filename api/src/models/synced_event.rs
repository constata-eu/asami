use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use ethers::prelude::{EthLogDecode, Event};

use super::*;
use crate::{
    models::on_chain::AsamiMiddleware,
    on_chain::{
        asami_contract_code::{
            CampaignCreatedFilter, CampaignExtendedFilter, CampaignReimbursedFilter, CampaignToppedUpFilter,
        },
        AsamiContract, BuilderRewardsClaimedFilter, CollectiveContract,
    },
};

pub type AsamiEvent<D> = Event<Arc<AsamiMiddleware>, AsamiMiddleware, D>;

model! {
  state: App,
  table: synced_events,
  struct SyncedEvent {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(varchar)]
    address: String,
    #[sqlx_model_hints(decimal)]
    block_number: Decimal,
    #[sqlx_model_hints(varchar)]
    block_hash: String,
    #[sqlx_model_hints(varchar)]
    tx_hash: String,
    #[sqlx_model_hints(decimal)]
    tx_index: Decimal,
    #[sqlx_model_hints(varchar)]
    log_index: String,
    #[sqlx_model_hints(varchar)]
    data: String,
    #[sqlx_model_hints(timestamptz, default)]
    created_at: UtcDateTime,
  }
}

impl_loggable!(SyncedEvent);

impl SyncedEventHub {
    pub async fn sync_on_chain_events(&self) -> anyhow::Result<()> {
        let index_state = self.state.indexer_state().get().await?;

        let from_block = std::cmp::max(
            self.state.settings.rsk.start_sync_from_block.into(),
            d_to_u64(index_state.attrs.last_synced_block),
        );

        let to_block =
            self.contract().client().get_block_number().await? - self.state.settings.rsk.reorg_protection_padding;

        self.sync_campaign_events_for::<CampaignCreatedFilter>(from_block, to_block).await?;
        self.sync_campaign_events_for::<CampaignToppedUpFilter>(from_block, to_block).await?;
        self.sync_campaign_events_for::<CampaignExtendedFilter>(from_block, to_block).await?;
        self.sync_campaign_events_for::<CampaignReimbursedFilter>(from_block, to_block).await?;

        self.sync_asami_transfer_events(from_block, to_block).await?;

        let rewards_from_block = d_to_u64(*index_state.last_rewards_indexed_block());
        let rewards_to_block = self.collective_contract().client().get_block_number().await?
            - self.state.settings.rsk.reorg_protection_padding;
        self.sync_builder_rewards_claimed_events(rewards_from_block, rewards_to_block).await?;
        self.sync_new_allocation_events(rewards_from_block, rewards_to_block).await?;

        index_state
            .update()
            .last_rewards_indexed_block(u64_to_d(rewards_to_block))
            .last_synced_block(u64_to_d(to_block))
            .save()
            .await?;
        Ok(())
    }

    pub async fn sync_builder_rewards_claimed_events(&self, from_block: U64, to_block: U64) -> anyhow::Result<()> {
        let events = self
            .collective_contract()
            .builder_rewards_claimed_filter()
            .address(self.collective_contract().address().into())
            .from_block(from_block)
            .to_block(to_block)
            .query_with_meta()
            .await?;

        let db_tx = self.state.transactional().await?;

        let mut by_tx_hash: HashMap<H256, Vec<BuilderRewardsClaimedFilter>> = HashMap::new();

        for (e, meta) in events {
            let Some(_synced_event) = db_tx.synced_event().save_unprocessed_event(&meta, &e).await? else {
                continue;
            };

            by_tx_hash.entry(meta.transaction_hash).or_default().push(e);
        }

        for (tx_hash, group) in by_tx_hash {
            db_tx.backer_disbursement().create_from_events(tx_hash, &group).await?;
        }

        db_tx.commit().await?;
        Ok(())
    }

    pub async fn sync_new_allocation_events(&self, from_block: U64, to_block: U64) -> anyhow::Result<()> {
        let events = self
            .collective_contract()
            .new_allocation_filter()
            .address(self.collective_contract().address().into())
            .from_block(from_block)
            .to_block(to_block)
            .query_with_meta()
            .await?;

        for (e, meta) in &events {
            let tx = self.state.transactional().await?;

            let Some(_synced_event) = tx.synced_event().save_unprocessed_event(meta, &e).await? else {
                continue;
            };

            tx.backer().get_or_create(e.backer).await?;

            tx.commit().await?;
        }

        Ok(())
    }

    pub async fn sync_asami_transfer_events(&self, from_block: U64, to_block: U64) -> anyhow::Result<()> {
        let events = self
            .contract()
            .transfer_filter()
            .address(self.contract().address().into())
            .from_block(from_block)
            .to_block(to_block)
            .query_with_meta()
            .await?;

        for (e, meta) in &events {
            let tx = self.state.transactional().await?;

            let Some(_synced_event) = tx.synced_event().save_unprocessed_event(meta, &e).await? else {
                continue;
            };

            let address = e.to;

            let balance = self.contract().balance_of(address).await?;

            let maybe_holder = tx.holder().select().address_eq(address.encode_hex()).optional().await?;

            match maybe_holder {
                Some(holder) => {
                    holder.update().balance(balance.encode_hex()).save().await?;
                }
                None => {
                    let code = self.contract().client().get_code(address, None).await?;
                    let is_contract = !code.as_ref().is_empty();
                    tx.holder()
                        .insert(InsertHolder {
                            is_contract,
                            address: address.encode_hex(),
                            balance: balance.encode_hex(),
                        })
                        .save()
                        .await?;
                }
            }

            tx.commit().await?;
        }

        Ok(())
    }

    pub async fn sync_campaign_events_for<D: CampaignEventFilter>(
        &self,
        from_block: U64,
        to_block: U64,
    ) -> anyhow::Result<()> {
        let events = <D as CampaignEventFilter>::filter(self.contract())
            .address(self.contract().address().into())
            .from_block(from_block)
            .to_block(to_block)
            .query_with_meta()
            .await?;

        let mut account_ids = HashSet::new();
        let mut campaign_ids = HashSet::new();

        for (e, meta) in &events {
            let Some(synced_event) = self.save_unprocessed_event(meta, &e).await? else {
                continue;
            };

            let campaigns = self.state.campaign().select().briefing_hash_eq(e.campaign_id().encode_hex()).all().await?;

            if campaigns.is_empty() {
                synced_event
                    .info(
                        "sync_campaign_event",
                        format!(
                            "Campaign not found {}. May be a mistake or a multi-node user.",
                            &e.campaign_id().encode_hex()
                        ),
                    )
                    .await?;
                continue;
            }

            let onchain = self
                .state
                .on_chain
                .asami_contract
                .get_campaign(e.account_addr(), e.campaign_id())
                .await
                .context("Looking up campaign on sync creation")?;

            let report_hash = if onchain.report_hash == u("0") {
                None
            } else {
                Some(onchain.report_hash.encode_hex())
            };

            for campaign in campaigns {
                campaign_ids.insert(*campaign.id());
                account_ids.insert(campaign.account_id().clone());

                campaign
                    .update()
                    .budget(onchain.budget.encode_hex())
                    .valid_until(Some(models::i_to_utc(onchain.valid_until)))
                    .report_hash(report_hash.clone())
                    .status(CampaignStatus::Published)
                    .save()
                    .await
                    .context(format!("Syncing event {}", synced_event.id()))?;
            }
        }

        self.state.campaign().hydrate_report_columns_for(campaign_ids.into_iter()).await?;
        self.state.account().hydrate_report_columns_for(account_ids.iter()).await?;
        self.state.account().hydrate_on_chain_columns_for(account_ids.iter()).await?;

        Ok(())
    }

    pub async fn save_unprocessed_event<T: serde::Serialize + std::fmt::Display>(
        &self,
        meta: &LogMeta,
        event: T,
    ) -> anyhow::Result<Option<SyncedEvent>> {
        let existing = self
            .select()
            .block_number_eq(u64_to_d(meta.block_number))
            .log_index_eq(meta.log_index.encode_hex())
            .count()
            .await?;

        if existing > 0 {
            return Ok(None);
        };

        let data = serde_json::to_string(&event)
            .map_err(|e| Error::Runtime(format!("could not serialize event {} {}", event, e)))?;

        let event = self
            .state
            .synced_event()
            .insert(InsertSyncedEvent {
                address: meta.address.encode_hex(),
                block_number: u64_to_d(meta.block_number),
                block_hash: meta.block_hash.encode_hex(),
                tx_hash: meta.transaction_hash.encode_hex(),
                tx_index: u64_to_d(meta.transaction_index),
                log_index: meta.log_index.encode_hex(),
                data,
            })
            .save()
            .await?;

        Ok(Some(event))
    }

    fn contract(&self) -> &AsamiContract {
        &self.state.on_chain.asami_contract
    }

    fn collective_contract(&self) -> &CollectiveContract {
        &self.state.on_chain.collective_contract
    }
}

pub trait CampaignEventFilter: Sized + Sync + EthLogDecode + serde::Serialize + std::fmt::Display {
    fn filter(contract: &AsamiContract) -> AsamiEvent<Self>;
    fn account_addr(&self) -> Address;
    fn campaign_id(&self) -> U256;
}

macro_rules! impl_campaign_event {
    ($struct:ident, $method:ident) => {
        impl CampaignEventFilter for $struct {
            fn filter(contract: &AsamiContract) -> AsamiEvent<Self> {
                contract.$method()
            }
            fn account_addr(&self) -> Address {
                self.account
            }
            fn campaign_id(&self) -> U256 {
                self.campaign_id
            }
        }
    };
}

impl_campaign_event!(CampaignCreatedFilter, campaign_created_filter);
impl_campaign_event!(CampaignToppedUpFilter, campaign_topped_up_filter);
impl_campaign_event!(CampaignExtendedFilter, campaign_extended_filter);
impl_campaign_event!(CampaignReimbursedFilter, campaign_reimbursed_filter);
