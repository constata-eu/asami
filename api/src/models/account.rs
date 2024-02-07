use super::*;

// This is an account that maps with the smart contract accounts.
// They may have several auth methods at first,
// but once they have the Eip signup method, all other methods get disabled.
model!{
  state: App,
  table: accounts,
  struct Account {
    #[sqlx_model_hints(varchar, default)]
    id: String,
    #[sqlx_model_hints(varchar)]
    name: Option<String>,
    #[sqlx_model_hints(varchar)]
    addr: Option<String>,
    #[sqlx_model_hints(varchar)]
    unclaimed_asami_tokens: String,
    #[sqlx_model_hints(varchar)]
    unclaimed_doc_rewards: String,
    #[sqlx_model_hints(timestamptz, default)]
    created_at: UtcDateTime,
    #[sqlx_model_hints(timestamptz, default)]
    updated_at: Option<UtcDateTime>,
  },
  has_many {
    Handle(account_id),
    CampaignPreference(account_id),
    ClaimAccountRequest(account_id),
  }
}

impl Account {
  pub async fn is_claimed_or_claiming(&self) -> sqlx::Result<bool> {
    Ok(self.addr().is_some() || self.claim_account_request_vec().await?.len() > 0)
  }

  pub async fn campaign_offers(&self) -> AsamiResult<Vec<Campaign>> {
    let handles = self.handle_vec().await?;
    if handles.len() == 0 { return Ok(vec![]); }

    let done: Vec<String> = self.state.collab().select().member_id_eq(self.id()).all().await?
      .into_iter().map(|x| x.attrs.campaign_id).collect();

    let ignored: Vec<String> = self.campaign_preference_scope()
      .not_interested_on_is_set(true)
      .all().await?
      .into_iter().map(|x| x.attrs.campaign_id)
      .collect();

    let mut campaigns = vec![];

    for c in self.state.campaign().select().finished_eq(false).all().await?.into_iter() {
      if c.valid_until() <= &Utc::now() { continue };
      if ignored.contains(c.id()) { continue };
      if done.contains(c.id()) { continue };
      if c.is_missing_ig_rules().await? { continue };

      for h in &handles {
        if h.validate_collaboration(&c).await.is_ok() {
          campaigns.push(c);
          break;
        }
      }
    }

    Ok(campaigns)
  }

  pub async fn create_handle_request(&self, site: Site, username: &str) -> sqlx::Result<HandleRequest> {
    self.state.handle_request().insert(InsertHandleRequest{
      account_id: self.attrs.id.clone(),
      username: username.to_string(),
      site: site,
    }).save().await
  }

  pub async fn create_claim_account_request(&self, addr: String, signature: String, session_id: String) -> AsamiResult<ClaimAccountRequest> {
    if self.is_claimed_or_claiming().await? {
      return Err(Error::validation("account", "cannot_call_on_claimed_account"));
    }

    Ok(self.state.claim_account_request().insert(InsertClaimAccountRequest{
      account_id: self.attrs.id.clone(),
      addr,
      signature,
      session_id,
    }).save().await?)
  }

  pub async fn create_campaign_request(
    &self,
    site: Site,
    content_id: &str,
    budget: U256,
    price_score_ratio: U256,
    valid_until: UtcDateTime,
  ) -> AsamiResult<CampaignRequest> {
    if self.is_claimed_or_claiming().await? {
      return Err(Error::validation("account", "cannot_call_on_claimed_account"));
    }

    Ok(self.state.campaign_request().insert(InsertCampaignRequest{
      account_id: self.attrs.id.clone(),
      site: site,
      budget: budget.encode_hex(),
      content_id: content_id.to_string(),
      price_score_ratio: price_score_ratio.encode_hex(),
      valid_until,
    }).save().await?)
  }
}

