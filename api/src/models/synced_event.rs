use super::*;

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

impl SyncedEventHub {
  pub async fn sync_on_chain_events(&self) -> AsamiResult<()> {
    let contract = &self.state.on_chain.contract;
    let index_state = self.state.indexer_state().get().await?;

    let from_block = std::cmp::max(
      self.state.settings.rsk.start_sync_from_block.into(),
      d_to_u64(index_state.attrs.last_synced_block),
    );

    let to_block = contract.client().get_block_number().await? - self.state.settings.rsk.reorg_protection_padding;

    self.sync_account_saved(contract, from_block, to_block).await?;
    self.sync_handle_saved(contract, from_block, to_block).await?;
    self.sync_approval(contract, from_block, to_block).await?;
    self.sync_campaign_saved(contract, from_block, to_block).await?;
    self.sync_collab_saved(contract, from_block, to_block).await?;
    self.sync_topic_saved(contract, from_block, to_block).await?;

    index_state.update().last_synced_block(u64_to_d(to_block)).save().await?;

    Ok(())
  }

  pub async fn save_unprocessed_event<T: serde::Serialize + std::fmt::Display>(
    &self,
    meta: &LogMeta,
    event: T,
  ) -> AsamiResult<bool> {
    let existing = self
      .state
      .synced_event()
      .select()
      .block_number_eq(u64_to_d(meta.block_number))
      .log_index_eq(meta.log_index.encode_hex())
      .count()
      .await?;

    if existing > 0 {
      return Ok(false);
    };

    let data = serde_json::to_string(&event)
      .map_err(|e| Error::Runtime(format!("could not serialize event {} {}", event, e)))?;

    self
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

    Ok(true)
  }

  pub async fn sync_account_saved(
    &self,
    contract: &AsamiContractSigner,
    from_block: U64,
    to_block: U64,
  ) -> AsamiResult<()> {
    let events = contract
      .account_saved_filter()
      .address(contract.address().into())
      .from_block(from_block)
      .to_block(to_block)
      .query_with_meta()
      .await?;

    for (e, meta) in &events {
      if !self.save_unprocessed_event(meta, &e).await? {
        continue;
      }

      let a = &e.account;
      let addr = if a.addr.is_zero() {
        None
      } else {
        Some(a.addr.encode_hex())
      };

      match self.state.account().find_optional(a.id.encode_hex()).await? {
        Some(account) => {
          account
            .update()
            .addr(addr)
            .unclaimed_asami_tokens(a.unclaimed_asami_tokens.encode_hex())
            .unclaimed_doc_rewards(a.unclaimed_asami_tokens.encode_hex())
            .save()
            .await?;
        }
        None => {
          self
            .state
            .account()
            .insert(InsertAccount {
              addr,
              name: None,
              unclaimed_asami_tokens: a.unclaimed_asami_tokens.encode_hex(),
              unclaimed_doc_rewards: a.unclaimed_asami_tokens.encode_hex(),
            })
            .save()
            .await?
            .update()
            .id(a.id.encode_hex())
            .save()
            .await?;
        }
      }
    }

    for (_, meta) in &events {
      let Some(on_chain_tx) = self.find_on_chain_tx(meta).await? else {
        continue;
      };
      self.state.claim_account_request().set_done(on_chain_tx.attrs.id).await?;
    }

    Ok(())
  }

  pub async fn sync_handle_saved(
    &self,
    contract: &AsamiContractSigner,
    from_block: U64,
    to_block: U64,
  ) -> AsamiResult<()> {
    let events = contract
      .handle_saved_filter()
      .address(contract.address().into())
      .from_block(from_block)
      .to_block(to_block)
      .query_with_meta()
      .await?;

    for (e, meta) in &events {
      if !self.save_unprocessed_event(meta, &e).await? {
        continue;
      }

      match self.state.handle().find_optional(e.handle.id.encode_hex()).await? {
        Some(handle) => {
          let h = &e.handle;
          for old in handle.handle_topic_vec().await? {
            old.delete().await?;
          }

          for new in &h.topics {
            self
              .state
              .handle_topic()
              .insert(InsertHandleTopic {
                handle_id: h.id.encode_hex(),
                topic_id: new.encode_hex(),
              })
              .save()
              .await?;
          }

          handle
            .update()
            .username(h.username.clone())
            .price(h.price.encode_hex())
            .score(h.score.encode_hex())
            .save()
            .await?;
        }
        None => {
          let h = &e.handle;
          self
            .state
            .handle()
            .insert(InsertHandle {
              id: h.id.encode_hex(),
              account_id: h.account_id.encode_hex(),
              site: Site::from_on_chain(h.site),
              username: h.username.clone(),
              user_id: h.user_id.clone(),
              price: h.price.encode_hex(),
              score: h.score.encode_hex(),
            })
            .save()
            .await?;

          for new in &h.topics {
            self
              .state
              .handle_topic()
              .insert(InsertHandleTopic {
                handle_id: h.id.encode_hex(),
                topic_id: new.encode_hex(),
              })
              .save()
              .await?;
          }
        }
      }
    }

    for (_, meta) in events {
      let Some(on_chain_tx) = self.find_on_chain_tx(&meta).await? else {
        continue;
      };
      self.state.handle_request().set_done(on_chain_tx.attrs.id).await?;
      self.state.set_score_and_topics_request().set_done(on_chain_tx.attrs.id).await?;
      self.state.set_price_request().set_done(on_chain_tx.attrs.id).await?;
    }

    Ok(())
  }

  pub async fn sync_approval(&self, contract: &AsamiContractSigner, from_block: U64, to_block: U64) -> AsamiResult<()> {
    let doc = &self.state.on_chain.doc_contract;
    let events = contract
      .approval_filter()
      .address(doc.address().into())
      .topic1(H256::from(contract.client().address()))
      .from_block(from_block)
      .to_block(to_block)
      .query_with_meta()
      .await?;

    for (e, meta) in events {
      if !self.save_unprocessed_event(&meta, &e).await? {
        continue;
      }

      let Some(on_chain_tx) = self.find_on_chain_tx(&meta).await? else {
        continue;
      };

      let req = self
        .state
        .campaign_request()
        .select()
        .status_eq(CampaignRequestStatus::Paid)
        .approval_id_eq(on_chain_tx.attrs.id)
        .optional()
        .await?;

      if let Some(r) = req {
        r.approve().await?;
      }
    }
    Ok(())
  }

  pub async fn sync_campaign_saved(
    &self,
    contract: &AsamiContractSigner,
    from_block: U64,
    to_block: U64,
  ) -> AsamiResult<()> {
    let events = contract
      .campaign_saved_filter()
      .address(contract.address().into())
      .from_block(from_block)
      .to_block(to_block)
      .query_with_meta()
      .await?;

    for (e, meta) in &events {
      if !self.save_unprocessed_event(meta, &e).await? {
        continue;
      }

      match self.state.campaign().find_optional(e.campaign.id.encode_hex()).await? {
        Some(campaign) => {
          let remaining = e.campaign.remaining;
          campaign.update().remaining(remaining.encode_hex()).finished(remaining == 0.into()).save().await?;
        }
        None => {
          let c = &e.campaign;

          self
            .state
            .campaign()
            .insert(InsertCampaign {
              id: c.id.encode_hex(),
              account_id: c.account_id.encode_hex(),
              site: Site::from_on_chain(c.site),
              budget: c.budget.encode_hex(),
              remaining: c.remaining.encode_hex(),
              content_id: c.content_id.clone(),
              price_score_ratio: c.price_score_ratio.encode_hex(),
              funded_by_admin: c.funded_by_admin,
              valid_until: i_to_utc(c.valid_until),
              tx_hash: meta.transaction_hash.encode_hex(),
            })
            .save()
            .await?;

          for new in &c.topics {
            self.state.campaign_topic().insert(InsertCampaignTopic {
              campaign_id: c.id.encode_hex(),
              topic_id: new.encode_hex(),
            }).save().await?;
          }
        }
      }
    }

    for (_, meta) in &events {
      let Some(on_chain_tx) = self.find_on_chain_tx(meta).await? else {
        continue;
      };
      self.state.campaign_request().set_done(on_chain_tx.attrs.id).await?;
    }
    Ok(())
  }

  pub async fn sync_collab_saved(
    &self,
    contract: &AsamiContractSigner,
    from_block: U64,
    to_block: U64,
  ) -> AsamiResult<()> {
    let events = contract
      .collab_saved_filter()
      .address(contract.address().into())
      .from_block(from_block)
      .to_block(to_block)
      .query_with_meta()
      .await?;

    for (e, meta) in &events {
      if !self.save_unprocessed_event(meta, &e).await? {
        continue;
      }

      if self.state.collab().find_optional(e.collab.id.encode_hex()).await?.is_some() {
        continue;
      }

      let c = &e.collab;
      let campaign = self.state.campaign().find(c.campaign_id.encode_hex()).await?;
      let handle = self.state.handle().find(c.handle_id.encode_hex()).await?;

      self
        .state
        .collab()
        .insert(InsertCollab {
          id: c.id.encode_hex(),
          campaign_id: campaign.attrs.id,
          advertiser_id: campaign.attrs.account_id,
          handle_id: handle.attrs.id,
          member_id: handle.attrs.account_id,
          gross: c.gross.encode_hex(),
          fee: c.fee.encode_hex(),
          created_at: c.created_at.encode_hex(),
        })
        .save()
        .await?;
    }

    for (_, meta) in events {
      let Some(on_chain_tx) = self.find_on_chain_tx(&meta).await? else {
        continue;
      };
      self.state.collab_request().set_done(on_chain_tx.attrs.id).await?;
    }

    Ok(())
  }

  pub async fn sync_topic_saved(
    &self,
    contract: &AsamiContractSigner,
    from_block: U64,
    to_block: U64,
  ) -> AsamiResult<()> {
    let events = contract
      .topic_saved_filter()
      .address(contract.address().into())
      .from_block(from_block)
      .to_block(to_block)
      .query_with_meta()
      .await?;

    for (e, meta) in &events {
      if !self.save_unprocessed_event(meta, &e).await? {
        continue;
      }

      self
        .state
        .topic()
        .insert(InsertTopic {
          id: e.id.encode_hex(),
          name: e.name.clone(),
        })
        .save()
        .await?;
    }

    for (_, meta) in &events {
      let Some(on_chain_tx) = self.find_on_chain_tx(meta).await? else {
        continue;
      };
      self.state.topic_request().set_done(on_chain_tx.attrs.id).await?;
    }

    Ok(())
  }

  async fn find_on_chain_tx(&self, meta: &LogMeta) -> sqlx::Result<Option<OnChainTx>> {
    let tx_hash = meta.transaction_hash.encode_hex();
    self.state.on_chain_tx().select().tx_hash_eq(tx_hash).optional().await
  }
}
