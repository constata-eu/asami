use super::*;

model! {
  state: App,
  table: campaign_requests,
  struct CampaignRequest {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(varchar)]
    account_id: String,
    #[sqlx_model_hints(varchar)]
    budget: String,
    #[sqlx_model_hints(site)]
    site: Site,
    #[sqlx_model_hints(varchar)]
    content_id: String,
    #[sqlx_model_hints(varchar)]
    price_score_ratio: String,
    #[sqlx_model_hints(timestamptz)]
    valid_until: UtcDateTime,
    #[sqlx_model_hints(campaign_request_status, default)]
    status: CampaignRequestStatus,
    #[sqlx_model_hints(int4, default)]
    approval_id: Option<i32>,
    #[sqlx_model_hints(int4, default)]
    on_chain_tx_id: Option<i32>,
    #[sqlx_model_hints(timestamptz, default)]
    created_at: UtcDateTime,
    #[sqlx_model_hints(timestamptz, default)]
    updated_at: Option<UtcDateTime>,
  },
  has_many {
    CampaignRequestTopic(campaign_request_id)
  },
  belongs_to {
    Account(account_id),
    OnChainTx(on_chain_tx_id),
  }
}

impl_loggable!(CampaignRequest);

impl_on_chain_tx_request! { CampaignRequestHub {
  type Model = CampaignRequest;
  type Update = UpdateCampaignRequestHub;
  type Status = CampaignRequestStatus;
  type Param = on_chain::AdminCampaignInput;

  async fn as_param(&self, model: &Self::Model) -> sqlx::Result<Self::Param> {
    let topics = model.campaign_request_topic_vec().await?
      .into_iter().map(|t| u256(t.attrs.topic_id) ).collect();

    Ok(Self::Param {
      account_id: u256(&model.attrs.account_id),
      attrs: on_chain::CampaignInput {
        site: model.attrs.site as u8,
        budget: u256(&model.attrs.budget),
        content_id: model.attrs.content_id.clone(),
        price_score_ratio: u256(&model.attrs.price_score_ratio),
        topics,
        valid_until: utc_to_i(model.attrs.valid_until)
      }
    })
  }

  async fn validate_before_submit_all(&self) -> sqlx::Result<()> {
    let pending = self.select().status_not_in(vec![
      CampaignRequestStatus::Submitted,
      CampaignRequestStatus::Failed,
      CampaignRequestStatus::Done,
    ]).all().await?;

    for p in pending {
      if p.account().await?.is_claimed_or_claiming().await? {
        p.fail("cannot_submit_for_claimed_account", &p).await?;
        p.update().status(CampaignRequestStatus::Failed).save().await?;
      }
    }
    Ok(())
  }

  fn fn_call(&self, params: Vec<Self::Param>) -> AsamiFunctionCall {
    self.state.on_chain.contract.admin_make_campaigns(params)
  }

  fn pending_status() -> Self::Status { CampaignRequestStatus::Approved }
  fn submitted_status() -> Self::Status { CampaignRequestStatus::Submitted }
  fn done_status() -> Self::Status { CampaignRequestStatus::Done }
}}

impl CampaignRequestHub {
  pub async fn submit_approvals(&self) -> anyhow::Result<()> {
    let rsk = &self.state.on_chain;
    let reqs = self.select().status_eq(CampaignRequestStatus::Paid).all().await?;
    let total: U256 = reqs.iter().map(|r| u256(r.budget())).fold(0.into(), |a, b| a + b);

    if reqs.is_empty() {
      return Ok(());
    }

    let on_chain_tx = self.state.on_chain_tx().send_tx(rsk.doc_contract.approve(rsk.contract.address(), total)).await?;

    for r in reqs {
      r.update()
        .status(CampaignRequestStatus::Approving)
        .approval_id(Some(on_chain_tx.attrs.id))
        .save()
        .await?;
    }

    Ok(())
  }

  pub async fn set_approved(&self, tx_id: i32) -> sqlx::Result<()> {
    for r in self.select().status_eq(CampaignRequestStatus::Approving).approval_id_eq(tx_id).all().await? {
      r.update().status(CampaignRequestStatus::Approved).save().await?;
    }
    Ok(())
  }
}

impl CampaignRequest {
  pub async fn campaign(&self) -> sqlx::Result<Option<Campaign>> {
    let Some(on_chain_tx) = self.on_chain_tx().await? else {
      return Ok(None);
    };
    let Some(tx_hash) = on_chain_tx.tx_hash() else {
      return Ok(None);
    };

    self
      .state
      .campaign()
      .select()
      .tx_hash_eq(tx_hash)
      .site_eq(self.site())
      .content_id_eq(self.content_id())
      .account_id_eq(self.account_id())
      .optional()
      .await
  }

  pub async fn topic_ids(&self) -> sqlx::Result<Vec<String>> {
    Ok(self.campaign_request_topic_vec().await?.into_iter().map(|x| x.attrs.topic_id).collect())
  }

  pub async fn approval(&self) -> sqlx::Result<Option<OnChainTx>> {
    let Some(id) = self.approval_id().as_ref() else {
      return Ok(None);
    };
    self.state.on_chain_tx().find_optional(id).await
  }

  pub async fn submission(&self) -> sqlx::Result<Option<OnChainTx>> {
    self.on_chain_tx().await
  }

  pub async fn pay(self) -> AsamiResult<Self> {
    Ok(self.update().status(CampaignRequestStatus::Paid).save().await?)
  }

  pub async fn approve(self) -> sqlx::Result<Self> {
    self.update().status(CampaignRequestStatus::Approved).save().await
  }
}

model! {
  state: App,
  table: campaign_request_topics,
  struct CampaignRequestTopic {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(int4)]
    campaign_request_id: i32,
    #[sqlx_model_hints(varchar)]
    topic_id: String,
  }
}

make_sql_enum![
  "campaign_request_status",
  pub enum CampaignRequestStatus {
    Received,  // The request was received by a managed user to create a campaign.
    Paid,      // We've got payment (through proprietary payment methods).
    Approving, // The approval transaction has been sent.
    Approved,  // We've approved the on-chain DOC spend for this campaign.
    Submitted, // We've tried to submit the request on-chain.
    Failed,    // This campaign was rendered invalid, and will be left out of upcoming batches.
    Done,      // The campaign is created and available on chain with enough confirmations.
  }
];
