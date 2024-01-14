use super::*;

model!{
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
      d_to_u64(index_state.attrs.last_synced_block)
    );

    let to_block = contract.client().get_block_number().await?
      - self.state.settings.rsk.reorg_protection_padding;
    
    self.sync_account_saved(contract, from_block, to_block).await?;
    self.sync_handle_saved(contract, from_block, to_block).await?;
    self.sync_approval(contract, from_block, to_block).await?;
    self.sync_campaign_saved(contract, from_block, to_block).await?;
    self.sync_collab_saved(contract, from_block, to_block).await?;
    self.sync_handle_update_saved(contract, from_block, to_block).await?;

    index_state.update().last_synced_block(u64_to_d(to_block)).save().await?;

    Ok(())
  }

  pub async fn save_unprocessed_event<T: serde::Serialize + std::fmt::Display>(&self, meta: &LogMeta, event: T) -> AsamiResult<bool> {
    let existing = self.state.synced_event().select()
      .block_number_eq( u64_to_d(meta.block_number) )
      .log_index_eq( meta.log_index.encode_hex() )
      .count().await?;

    if existing > 0 { return Ok(false) };

    let data = serde_json::to_string(&event)
      .map_err(|e| Error::Runtime(format!("could not serialize event {} {}", event, e)))?;

    self.state.synced_event().insert(InsertSyncedEvent{
      address: meta.address.encode_hex(),
      block_number: u64_to_d(meta.block_number),
      block_hash: meta.block_hash.encode_hex(),
      tx_hash: meta.transaction_hash.encode_hex(),
      tx_index: u64_to_d(meta.transaction_index),
      log_index: meta.log_index.encode_hex(),
      data
    }).save().await?;

    return Ok(true);
  }

  pub async fn sync_account_saved(&self, contract: &AsamiContractSigner, from_block: U64, to_block: U64) ->  AsamiResult<()> {
    let events = contract.account_saved_filter()
      .address(contract.address().into())
      .from_block(from_block)
      .to_block(to_block)
      .query_with_meta().await?;

    for (e, meta) in events {
      if !self.save_unprocessed_event(&meta, &e).await? {
        continue;
      }

      let a = e.account;
      let addr = if a.addr.is_zero() { None } else { Some(a.addr.encode_hex()) };

      let maybe_claim_req = self.state.claim_account_request().select()
        .tx_hash_eq(meta.transaction_hash.encode_hex())
        .account_id_eq(a.id.encode_hex())
        .status_eq(ClaimAccountRequestStatus::Submitted)
        .optional().await?;

      if let Some(h) = maybe_claim_req {
        h.done().await?;
      }

      match self.state.account().find_optional(a.id.encode_hex()).await? {
        Some(account) => {
          account.update()
            .addr(addr)
            .unclaimed_asami_tokens(a.unclaimed_asami_tokens.encode_hex())
            .unclaimed_doc_rewards(a.unclaimed_asami_tokens.encode_hex())
            .save().await?;
        },
        None => {
          self.state.account().insert(InsertAccount{
            addr,
            name: None,
            unclaimed_asami_tokens: a.unclaimed_asami_tokens.encode_hex(),
            unclaimed_doc_rewards: a.unclaimed_asami_tokens.encode_hex(),
          })
          .save().await?
          .update().id(a.id.encode_hex())
          .save().await?;
        }
      }
    }
    Ok(())
  }

  pub async fn sync_handle_saved(&self, contract: &AsamiContractSigner, from_block: U64, to_block: U64) ->  AsamiResult<()> {
    let events = contract.handle_saved_filter()
      .address(contract.address().into())
      .from_block(from_block)
      .to_block(to_block)
      .query_with_meta().await?;

    for (e, meta) in events {
      if !self.save_unprocessed_event(&meta, &e).await? {
        continue;
      }

      match self.state.handle().find_optional(e.handle.id.encode_hex()).await? {
        Some(handle) => {
          let h = e.handle;
          for old in handle.handle_topic_vec().await? {
            old.delete().await?;
          }

          for new in h.topics {
            self.state.handle_topic().insert(InsertHandleTopic{
              handle_id: handle.attrs.id.clone().encode_hex(),
              topic_id: new.encode_hex(),
            });
          }

          handle.update()
            .username(h.username)
            .price(h.price.encode_hex())
            .score(h.score.encode_hex())
            .save().await?;
        },
        None => {
          let h = e.handle;
          let handle = self.state.handle().insert(InsertHandle {
            id: h.id.encode_hex(),
            account_id: h.account_id.encode_hex(),
            site: Site::from_on_chain(h.site),
            username: h.username,
            user_id: h.user_id,
            price: h.price.encode_hex(),
            score: h.score.encode_hex(),
          }).save().await?;

          for new in h.topics {
            self.state.handle_topic().insert(InsertHandleTopic{
              handle_id: h.id.encode_hex(),
              topic_id: new.encode_hex(),
            });
          }

          let req = self.state.handle_request().select()
            .tx_hash_eq(meta.transaction_hash.encode_hex())
            .username_ilike(handle.attrs.username.clone())
            .status_eq(HandleRequestStatus::Submitted)
            .optional().await?;

          if let Some(r) = req {
            r.done(&handle).await?;
          }
        }
      }
    }
    Ok(())
  }

  pub async fn sync_approval(&self, contract: &AsamiContractSigner, from_block: U64, to_block: U64) ->  AsamiResult<()> {
    let doc = &self.state.on_chain.doc_contract;
    let events = contract.approval_filter()
      .address(doc.address().into())
      .topic1(H256::from(contract.client().address()))
      .from_block(from_block)
      .to_block(to_block)
      .query_with_meta().await?;

    for (e, meta) in events {
      if !self.save_unprocessed_event(&meta, &e).await? {
        continue;
      }

      let req = self.state.campaign_request().select()
        .status_eq(CampaignRequestStatus::Paid)
        .approval_tx_hash_eq(meta.transaction_hash.encode_hex())
        .optional().await?;

      if let Some(r) = req {
        r.approve().await?;
      }
    }
    Ok(())
  }

  pub async fn sync_campaign_saved(&self, contract: &AsamiContractSigner, from_block: U64, to_block: U64) ->  AsamiResult<()> {
    let events = contract.campaign_saved_filter()
      .address(contract.address().into())
      .from_block(from_block)
      .to_block(to_block)
      .query_with_meta().await?;

    for (e, meta) in events {
      if !self.save_unprocessed_event(&meta, &e).await? {
        continue;
      }

      match self.state.campaign().find_optional(e.campaign.id.encode_hex()).await? {
        Some(campaign) => {
          let remaining = e.campaign.remaining;
          campaign.update()
            .remaining(remaining.encode_hex())
            .finished(remaining == 0.into())
            .save().await?;
        },
        None => {
          let c = e.campaign;
          let campaign = self.state.campaign().insert(InsertCampaign{
            id: c.id.encode_hex(),
            account_id: c.account_id.encode_hex(),
            site: Site::from_on_chain(c.site),
            budget: c.budget.encode_hex(),
            remaining: c.remaining.encode_hex(),
            content_id: c.content_id,
            price_score_ratio: c.price_score_ratio.encode_hex(),
            valid_until: i_to_utc(c.valid_until),
          }).save().await?;

          for new in c.topics {
            self.state.campaign_topic().insert(InsertCampaignTopic{
              campaign_id: c.id.encode_hex(),
              topic_id: new.encode_hex(),
            });
          }

          let req = self.state.campaign_request().select()
            .content_id_eq(&campaign.attrs.content_id.clone())
            .status_eq(CampaignRequestStatus::Submitted)
            .submission_tx_hash_eq(meta.transaction_hash.encode_hex())
            .optional().await?;

          if let Some(r) = req {
            r.done(&campaign).await?;
          }
        }
      }
    }
    Ok(())
  }

  pub async fn sync_collab_saved(&self, contract: &AsamiContractSigner, from_block: U64, to_block: U64) ->  AsamiResult<()> {
    let events = contract.collab_saved_filter()
      .address(contract.address().into())
      .from_block(from_block)
      .to_block(to_block)
      .query_with_meta().await?;

    for (e, meta) in events {
      if !self.save_unprocessed_event(&meta, &e).await? {
        continue;
      }

      if self.state.collab().find_optional(e.collab.id.encode_hex()).await?.is_some() {
        continue;
      }

      let c = e.collab;
      let campaign = self.state.campaign().find(c.campaign_id.encode_hex()).await?;
      let handle = self.state.handle().find(c.handle_id.encode_hex()).await?;

      let collab = self.state.collab().insert(InsertCollab{
        id: c.id.encode_hex(),
        campaign_id: campaign.attrs.id,
        advertiser_id: campaign.attrs.account_id,
        handle_id: handle.attrs.id,
        member_id: handle.attrs.account_id,
        gross: c.gross.encode_hex(),
        fee: c.fee.encode_hex(),
        created_at: c.created_at.encode_hex(),
      }).save().await?;

      let req = self.state.collab_request().select()
        .status_eq(CollabRequestStatus::Submitted)
        .campaign_id_eq(c.campaign_id.encode_hex())
        .handle_id_eq(c.handle_id.encode_hex())
        .tx_hash_eq(meta.transaction_hash.encode_hex())
        .optional().await?;

      if let Some(r) = req {
        r.done(&collab).await?;
      }
    }

    Ok(())
  }

  pub async fn sync_handle_update_saved(&self, contract: &AsamiContractSigner, from_block: U64, to_block: U64) ->  AsamiResult<()> {
    let events = contract.handle_update_saved_filter()
      .address(contract.address().into())
      .from_block(from_block)
      .to_block(to_block)
      .query_with_meta().await?;

    for (e, meta) in events {
      if !self.save_unprocessed_event(&meta, &e).await? {
        continue;
      }

      let req = self.state.handle_update_request().select()
        .status_eq(HandleUpdateRequestStatus::Submitted)
        .tx_hash_eq(meta.transaction_hash.encode_hex())
        .optional().await?;

      if let Some(r) = req {
        r.done().await?;
      }
    }
    Ok(())
  }
}
