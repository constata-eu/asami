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
    submission_id: Option<i32>,
    #[sqlx_model_hints(varchar, default)]
    campaign_id: Option<String>,
    #[sqlx_model_hints(timestamptz, default)]
    created_at: UtcDateTime,
    #[sqlx_model_hints(timestamptz, default)]
    updated_at: Option<UtcDateTime>,
  },
  has_many {
    CampaignRequestTopic(campaign_request_id)
  },
  belongs_to {
    Campaign(campaign_id),
    Account(account_id),
  }
}

impl_loggable!(CampaignRequest);

impl CampaignRequestHub {
  pub async fn submit_approvals(&self) -> anyhow::Result<()> {
    let rsk = &self.state.on_chain;
    let reqs = self.select().status_eq(CampaignRequestStatus::Paid).all().await?;
    let total: U256 = reqs.iter().map(|r| u256(r.budget()) ).fold(0.into(), |a,b| a+b);

    if reqs.is_empty() { return Ok(()); }

    let on_chain_tx = self.state.on_chain_tx()
      .send(rsk.doc_contract.approve(rsk.contract.address(), total)).await?;

    for r in reqs {
      r.update()
        .status(CampaignRequestStatus::Approved)
        .approval_id(Some(on_chain_tx.attrs.id))
        .save().await?;
    }

    Ok(())
  }

  pub async fn validate_before_submit_all(&self) -> sqlx::Result<()> {
    let pending = self.select().status_not_in(vec![
      CampaignRequestStatus::Submitted,
      CampaignRequestStatus::Failed,
      CampaignRequestStatus::Done,
    ]).all().await?;

    for p in pending {
      if p.account().await?.is_claimed_or_claiming().await? {
        p.mark_failed().await?;
      }
    }
    Ok(())
  }

  pub async fn submit_all(&self) -> anyhow::Result<()> {
    self.validate_before_submit_all().await?;

    let reqs = self.select().status_eq(CampaignRequestStatus::Approved).all().await?;

    if reqs.is_empty() { return Ok(()); }

    let mut params = vec![];
    for r in &reqs {
      params.push(r.as_param().await?);
    }

    let on_chain_tx = self.state.on_chain_tx()
      .send(self.state.on_chain.contract.admin_make_campaigns(params)).await?;

    for r in reqs {
      r.update()
        .status(CampaignRequestStatus::Submitted)
        .submission_id(Some(on_chain_tx.attrs.id))
        .save().await?;
    }

    Ok(())
  }
}

impl CampaignRequest {
  pub async fn approval(&self) -> sqlx::Result<Option<OnChainTx>> {
    let Some(id) = self.approval_id().as_ref() else { return Ok(None) };
    self.state.on_chain_tx().find_optional(id).await
  }

  pub async fn submission(&self) -> sqlx::Result<Option<OnChainTx>> {
    let Some(id) = self.submission_id().as_ref() else { return Ok(None) };
    self.state.on_chain_tx().find_optional(id).await
  }

  pub async fn pay(self) -> AsamiResult<Self> {
    Ok(self.update().status(CampaignRequestStatus::Paid).save().await?)
  }

  pub async fn mark_failed(self) -> sqlx::Result<Self> {
    self.fail("cannot_submit_for_claimed_account", &self).await?;
    self.update().status(CampaignRequestStatus::Failed).save().await
  }

  pub async fn approve(self) -> sqlx::Result<Self> {
    self.update().status(CampaignRequestStatus::Approved).save().await
  }

  pub async fn done(self, campaign: &Campaign) -> sqlx::Result<Self> {
    self.update().status(CampaignRequestStatus::Done)
      .campaign_id(Some(campaign.attrs.id.clone()))
      .save().await
  }

  pub async fn as_param(&self) -> AsamiResult<on_chain::AdminCampaignInput> {
    let topics = self.campaign_request_topic_vec().await?
      .into_iter().map(|t| u256(t.attrs.topic_id) ).collect();

    Ok(on_chain::AdminCampaignInput{
      account_id: u256(&self.attrs.account_id),
      attrs: on_chain::CampaignInput {
        site: self.attrs.site as u8,
        budget: u256(&self.attrs.budget),
        content_id: self.attrs.content_id.clone(),
        price_score_ratio: u256(&self.attrs.price_score_ratio),
        topics,
        valid_until: utc_to_i(self.attrs.valid_until)
      }
    })
  }
}

model!{
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
    Received,   // The request was received by a managed user to create a campaign.
    Paid,       // We've got payment (through proprietary payment methods).
    Approved,   // We've approved the on-chain DOC spend for this campaign.
    Submitted,  // We've tried to submit the request on-chain.
    Failed,     // This campaign was rendered invalid, and will be left out of upcoming batches.
    Done,       // The campaign is created and available on chain with enough confirmations.
  }
];
