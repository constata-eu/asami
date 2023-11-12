use super::*;
use super::on_chain::{self, AsamiContractEvents};
pub use sqlx::{self, types::Decimal};
pub use serde::{Serialize, Deserialize};
use sqlx_models_orm::model;
pub use chrono::{DateTime, Duration, Utc, Datelike, TimeZone};
pub type UtcDateTime = DateTime<Utc>;
pub use juniper::GraphQLEnum;
use ethers::{ types::{ U256, U64}, abi::{AbiEncode, AbiDecode}, middleware::Middleware};

pub mod hasher;

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize, sqlx::Type, GraphQLEnum)]
#[sqlx(type_name = "site", rename_all = "snake_case")]
pub enum Site {
  X,
  Nostr,
  Instagram,
}

impl Site {
  pub fn from_on_chain(o: u8) -> Self {
    match o {
      0 => Self::X,
      1 => Self::Nostr,
      2 => Self::Instagram,
      _ => panic!("mismatched site on contract")
    }
  }
}

impl sqlx::postgres::PgHasArrayType for Site {
  fn array_type_info() -> sqlx::postgres::PgTypeInfo {
    sqlx::postgres::PgTypeInfo::with_name("_site")
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize, sqlx::Type, GraphQLEnum)]
#[sqlx(type_name = "handle_request_status", rename_all = "snake_case")]
pub enum HandleRequestStatus {
  Unverified, // Newly created, never sent on-chain.
  Verified,   // Verified off-chain.
  Appraised,  // Appraised off-chain.
  Submitted,  // Sent, after this we listen for events regarding this handle.
  Done,     // Local DB knows this handle is now on-chain.
}

impl sqlx::postgres::PgHasArrayType for HandleRequestStatus {
  fn array_type_info() -> sqlx::postgres::PgTypeInfo {
    sqlx::postgres::PgTypeInfo::with_name("_handle_request_status")
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize, sqlx::Type, GraphQLEnum)]
#[sqlx(type_name = "handle_update_request_status", rename_all = "snake_case")]
pub enum HandleUpdateRequestStatus {
  Received,
  Submitted,
  Done,
}

impl sqlx::postgres::PgHasArrayType for HandleUpdateRequestStatus {
  fn array_type_info() -> sqlx::postgres::PgTypeInfo {
    sqlx::postgres::PgTypeInfo::with_name("_handle_update_request_status")
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize, sqlx::Type, GraphQLEnum)]
#[sqlx(type_name = "campaign_request_status", rename_all = "snake_case")]
pub enum CampaignRequestStatus {
  Received,   // The request was received by a managed user to create a campaign.
  Paid,       // We've got payment (through proprietary payment methods).
  Approved,   // We've approved the on-chain DOC spend for this campaign.
  Submitted,  // We've tried to submit the request on-chain.
  Done,     // We've got the event that creates this campaign.
}

impl sqlx::postgres::PgHasArrayType for CampaignRequestStatus {
  fn array_type_info() -> sqlx::postgres::PgTypeInfo {
    sqlx::postgres::PgTypeInfo::with_name("_campaign_request_status")
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "collab_request_status", rename_all = "snake_case")]
pub enum CollabRequestStatus {
  Received,   // The request was received by a managed user to create a collab.
  Submitted,  // We've tried to submit the request on-chain.
  Done,     // We've got the event that creates this collab.
}

impl sqlx::postgres::PgHasArrayType for CollabRequestStatus {
  fn array_type_info() -> sqlx::postgres::PgTypeInfo {
    sqlx::postgres::PgTypeInfo::with_name("_collab_request_status")
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "auth_method_kind", rename_all = "snake_case")]
pub enum AuthMethodKind {
  X,
  Google,
  Instagram,
  Nostr,
  Eip712,
  OneTimeToken,
}

impl AuthMethodKind {
  pub fn from_str(s: &str) -> Option<AuthMethodKind> {
    match s {
      "X" => Some(AuthMethodKind::X),
      "Google" => Some(AuthMethodKind::Google),
      "Instagram" => Some(AuthMethodKind::Instagram),
      "Nostr" => Some(AuthMethodKind::Nostr),
      "Eip712" => Some(AuthMethodKind::Eip712),
      "OneTimeToken" => Some(AuthMethodKind::OneTimeToken),
      _ => None
    }
  }
}

impl sqlx::postgres::PgHasArrayType for AuthMethodKind {
  fn array_type_info() -> sqlx::postgres::PgTypeInfo {
    sqlx::postgres::PgTypeInfo::with_name("_auth_method_kind")
  }
}

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
    #[sqlx_model_hints(boolean)]
    nostr_self_managed: bool,
    #[sqlx_model_hints(boolean)]
    nostr_abuse_proven: bool,
    #[sqlx_model_hints(timestamptz, default)]
    created_at: UtcDateTime,
    #[sqlx_model_hints(timestamptz, default)]
    updated_at: Option<UtcDateTime>,
  }
}

impl Account {
  pub async fn create_handle_request(&self, site: Site, username: &str) -> sqlx::Result<HandleRequest> {
    self.state.handle_request().insert(InsertHandleRequest{
      account_id: self.attrs.id.clone(),
      username: username.to_string(),
      site: site,
    }).save().await
  }

  pub async fn create_campaign_request(
    &self,
    site: Site,
    content_id: &str,
    budget: U256,
    price_score_ratio: U256,
    valid_until: UtcDateTime,
  ) -> sqlx::Result<CampaignRequest> {
    self.state.campaign_request().insert(InsertCampaignRequest{
      account_id: self.attrs.id.clone(),
      site: site,
      budget: budget.encode_hex(),
      content_id: content_id.to_string(),
      price_score_ratio: price_score_ratio.encode_hex(),
      valid_until,
    }).save().await
  }
}

model!{
  state: App,
  table: handle_requests,
  struct HandleRequest {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(varchar)]
    account_id: String,
    #[sqlx_model_hints(site)]
    site: Site,
    #[sqlx_model_hints(varchar)]
    username: String,
    #[sqlx_model_hints(varchar, default)]
    user_id: Option<String>,
    #[sqlx_model_hints(varchar, default)]
    price: Option<String>,
    #[sqlx_model_hints(varchar, default)]
    score: Option<String>,
    #[sqlx_model_hints(varchar, default)]
    nostr_affine_x: Option<String>,
    #[sqlx_model_hints(varchar, default)]
    nostr_affine_y: Option<String>,
    #[sqlx_model_hints(handle_request_status, default)]
    status: HandleRequestStatus,
    #[sqlx_model_hints(varchar, default)]
    tx_hash: Option<String>,
  },
  has_many {
    HandleRequestTopic(handle_request_id),
  }
}

impl HandleRequestHub {
  pub async fn verify_and_appraise_all(&self) -> AsamiResult<Vec<HandleRequest>> {
    self.verify_and_appraise_x().await
  }

  pub async fn verify_and_appraise_x(&self) -> AsamiResult<Vec<HandleRequest>> {
    use twitter_v2::{TwitterApi, authorization::BearerToken, query::*, api_result::*};
    use rust_decimal::prelude::*;
    use tokio::time::*;

    let mut handle_requests = vec![];

    let msg_regex = regex::Regex::new(r#"\@asami_club \[(\d)\]"#)?;
    let conf = &self.state.settings.x;
    let auth = BearerToken::new(&conf.bearer_token);
    let api = TwitterApi::new(auth);
    let indexer_state = self.state.indexer_state().get().await?;

    let mentions = api
      .get_user_mentions(&conf.asami_user_id)
      .since_id(indexer_state.attrs.x_handle_verification_checkpoint.to_u64().unwrap_or(0))
      .max_results(100)
      .user_fields(vec![ UserField::Id, UserField::Username, UserField::PublicMetrics ])
      .expansions(vec![ TweetExpansion::AuthorId, ])
      .send().await?;

    let checkpoint: i64 = mentions.meta()
      .and_then(|m| m.oldest_id.clone() )
      .and_then(|i| i.parse().ok() )
      .unwrap_or(0);

    let mut page = Some(mentions);
    let mut pages = 0;

    while let Some(mentions) = page {
      let payload = mentions.payload();
      let Some(data) = payload.data() else { break };

      for post in data {
        let Some(author_id) = post.author_id else { continue };
        let Some(author) = payload.includes()
          .and_then(|i| i.users.as_ref() )
          .and_then(|i| i.iter().find(|x| x.id == author_id ) )
          else { continue };

        let Some(public_metrics) = author.public_metrics.clone() else { continue };

        if let Some(capture) = msg_regex.captures(&post.text) {
          let Ok(account_id) = capture[1].parse::<String>() else { continue };

          let Some(req) = self.state.handle_request().select()
            .status_eq(&HandleRequestStatus::Unverified)
            .site_eq(&Site::X)
            .username_eq(&author.username)
            .account_id_eq(&account_id)
            .optional().await? else { continue };

          let score = U256::from(public_metrics.followers_count) * u("85") / u("100");
          let price = u256(indexer_state.suggested_price_per_point()) * score;
          handle_requests.push(
            req.verify(author_id.to_string()).await?.appraise(price, score).await?
          );
        }
      }

      // We only fetch a backlog of 700 tweets.
      // Older mentions are dropped and should be tried again by the users.
      pages += 1;
      if pages == 5 { break; }
      page = mentions.next_page().await?;
      if page.is_some() { sleep(Duration::from_millis(200)).await; }
    }

    indexer_state.update().x_handle_verification_checkpoint(checkpoint).save().await?;

    Ok(handle_requests)
  }

  pub async fn submit_all(self) -> AsamiResult<()> {
    let rsk = &self.state.on_chain;
    let reqs = self.select().status_eq(HandleRequestStatus::Appraised).all().await?;

    let mut params = vec![];
    for r in &reqs {
      params.push(r.as_param().await?);
    }

    let tx_hash = rsk.contract
      .admin_make_handles(params)
      .send().await?
      .tx_hash()
      .encode_hex();

    for r in reqs {
      r.update().status(HandleRequestStatus::Submitted).tx_hash(Some(tx_hash.clone())).save().await?;
    }

    Ok(())
  }
}

impl HandleRequest {
  pub async fn verify(self, user_id: String) -> sqlx::Result<Self> {
    self.update()
      .user_id(Some(user_id))
      .status(HandleRequestStatus::Verified)
      .save().await
  }

  pub async fn appraise(self, price: U256, score: U256) -> sqlx::Result<Self> {
    self.update()
      .price(Some(price.encode_hex()))
      .score(Some(score.encode_hex()))
      .status(HandleRequestStatus::Appraised)
      .save().await
  }

  pub async fn done(self) -> sqlx::Result<Self> {
    self.update().status(HandleRequestStatus::Done).save().await
  }

  pub async fn topic_ids(&self) -> sqlx::Result<Vec<String>> {
    Ok(self.handle_request_topic_vec().await?.into_iter().map(|t| t.attrs.topic_id ).collect())
  }

  pub async fn as_param(&self) -> AsamiResult<on_chain::Handle> {
    let a = &self.attrs;

    let invalid_state = Err(Error::Runtime("Invalid state on appraised handle request".into()));

    let Some(price) = a.price.as_ref().map(u256) else { return invalid_state };
    let Some(score) = a.score.as_ref().map(u256) else { return invalid_state };
    let Some(user_id) = a.user_id.clone() else { return invalid_state };

    let nostr_affine_x = a.nostr_affine_x.as_ref().map(u256).unwrap_or(0.into());
    let nostr_affine_y = a.nostr_affine_y.as_ref().map(u256).unwrap_or(0.into());

    let topics = self.topic_ids().await?.iter().map(|i| u256(i) ).collect();

    Ok(on_chain::Handle {
      id: 0.into(),
      account_id: u256(&a.account_id), 
      site: a.site as u8,
      price,
      score,
      topics,
      username: a.username.clone(),
      user_id: user_id,
      nostr_affine_x,
      nostr_affine_y,
    })
  }
}

model!{
  state: App,
  table: handle_request_topics,
  struct HandleRequestTopic {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(int4)]
    handle_request_id: i32,
    #[sqlx_model_hints(varchar)]
    topic_id: String,
  }
}

model!{
  state: App,
  table: handles,
  struct Handle {
    #[sqlx_model_hints(varchar)]
    id: String,
    #[sqlx_model_hints(varchar)]
    account_id: String,
    #[sqlx_model_hints(site)]
    site: Site,
    #[sqlx_model_hints(varchar)]
    username: String,
    #[sqlx_model_hints(varchar)]
    user_id: String,
    #[sqlx_model_hints(varchar)]
    price: String,
    #[sqlx_model_hints(varchar)]
    score: String,
    #[sqlx_model_hints(varchar)]
    nostr_affine_x: String,
    #[sqlx_model_hints(varchar)]
    nostr_affine_y: String,
  },
  has_many {
    HandleTopic(handle_id),
  }
}

model!{
  state: App,
  table: handle_topics,
  struct HandleTopic {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(varchar)]
    handle_id: String,
    #[sqlx_model_hints(varchar)]
    topic_id: String,
  }
}

model!{
  state: App,
  table: handle_update_requests,
  struct HandleUpdateRequest {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(varchar)]
    handle_id: String,
    #[sqlx_model_hints(varchar)]
    username: Option<String>,
    #[sqlx_model_hints(varchar)]
    price: Option<String>,
    #[sqlx_model_hints(varchar)]
    score: Option<String>,
    #[sqlx_model_hints(handle_update_request_status)]
    status: HandleUpdateRequestStatus,
    #[sqlx_model_hints(varchar, default)]
    tx_hash: Option<String>,
  },
  has_many {
    HandleUpdateTopic(handle_update_request_id),
  }
}

impl HandleUpdateRequest {
  pub async fn done(self) -> sqlx::Result<Self> {
    self.update().status(HandleUpdateRequestStatus::Done).save().await
  }
}

model!{
  state: App,
  table: handle_update_request_topics,
  struct HandleUpdateTopic {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(int4)]
    handle_update_request_id: i32,
    #[sqlx_model_hints(varchar)]
    topic_id: String,
  }
}

model!{
  state: App,
  table: users,
  struct User {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    name: String,
    #[sqlx_model_hints(timestamptz, default)]
    created_at: UtcDateTime,
    #[sqlx_model_hints(timestamptz, default)]
    updated_at: Option<UtcDateTime>,
  },
  has_many {
    AccountUser(user_id)
  }
}

impl User {
  pub async fn account_ids(&self) -> sqlx::Result<Vec<String>> {
    Ok(self.account_user_vec().await?.into_iter().map(|x| x.attrs.account_id).collect())
  }
}

model!{
  state: App,
  table: sessions,
  struct Session {
    #[sqlx_model_hints(varchar)]
    id: String,
    #[sqlx_model_hints(int4)]
    user_id: i32,
    #[sqlx_model_hints(int4)]
    auth_method_id: i32,
    #[sqlx_model_hints(varchar)]
    pubkey: String,
    #[sqlx_model_hints(bigint)]
    nonce: i64,
    #[sqlx_model_hints(timestamptz, default)]
    created_at: UtcDateTime,
    #[sqlx_model_hints(timestamptz, default)]
    updated_at: Option<UtcDateTime>,
    #[sqlx_model_hints(int4, default)]
    deletion_id: Option<i32>,
  },
  belongs_to {
    User(user_id),
  }
}

model!{
  state: App,
  table: auth_methods,
  struct AuthMethod {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    user_id: i32,
    #[sqlx_model_hints(auth_method_kind)]
    kind: AuthMethodKind,
    #[sqlx_model_hints(Varchar)]
    lookup_key: String,
  }
}

model!{
  state: App,
  table: account_users,
  struct AccountUser {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(varchar)]
    account_id: String,
    #[sqlx_model_hints(int4)]
    user_id: i32,
  }
}

/* One time tokens are just single use strings for the one time token authentication method.
 * The authentication strategy will only check that the string exists and has not been used.
 * This token's id is referenced in the lookup key of (at least one) AuthMethod.
 */
model!{
  state: App,
  table: one_time_tokens,
  struct OneTimeToken {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(varchar)]
    value: String,
    #[sqlx_model_hints(boolean, default)]
    used: bool,
  }
}

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
    #[sqlx_model_hints(varchar, default)]
    approval_tx_hash: Option<String>,
    #[sqlx_model_hints(varchar, default)]
    submission_tx_hash: Option<String>,
    #[sqlx_model_hints(timestamptz, default)]
    created_at: UtcDateTime,
    #[sqlx_model_hints(timestamptz, default)]
    updated_at: Option<UtcDateTime>,
  },
  has_many {
    CampaignRequestTopic(campaign_request_id)
  }
}

impl CampaignRequestHub {
  pub async fn submit_approvals(&self) -> AsamiResult<()> {
    let rsk = &self.state.on_chain;
    let reqs = self.select().status_eq(CampaignRequestStatus::Paid).all().await?;
    let total: U256 = reqs.iter().map(|r| u256(r.budget()) ).fold(0.into(), |a,b| a+b);

    let tx_hash = rsk.doc_contract.approve( rsk.contract.address(), total).send().await?
      .tx_hash().encode_hex();

    for r in reqs {
      r.update().status(CampaignRequestStatus::Approved).approval_tx_hash(Some(tx_hash.clone())).save().await?;
    }

    Ok(())
  }

  pub async fn submit_all(&self) -> AsamiResult<()> {
    let rsk = &self.state.on_chain;
    let reqs = self.select().status_eq(CampaignRequestStatus::Approved).all().await?;

    let mut params = vec![];
    for r in &reqs {
      params.push(r.as_param().await?);
    }

    let tx_hash = rsk.contract.admin_make_campaigns(params).send().await?.tx_hash().encode_hex();

    for r in reqs {
      r.update().status(CampaignRequestStatus::Submitted).submission_tx_hash(Some(tx_hash.clone())).save().await?;
    }

    Ok(())
  }
}

impl CampaignRequest {
  pub async fn pay(self) -> AsamiResult<Self> {
    Ok(self.update().status(CampaignRequestStatus::Paid).save().await?)
  }

  pub async fn approve(self) -> sqlx::Result<Self> {
    self.update().status(CampaignRequestStatus::Approved).save().await
  }

  pub async fn done(self) -> sqlx::Result<Self> {
    self.update().status(CampaignRequestStatus::Done).save().await
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


model!{
  state: App,
  table: campaigns,
  struct Campaign {
    #[sqlx_model_hints(varchar)]
    id: String,
    #[sqlx_model_hints(varchar)]
    account_id: String,
    #[sqlx_model_hints(site)]
    site: Site,
    #[sqlx_model_hints(varchar)]
    budget: String,
    #[sqlx_model_hints(varchar)]
    remaining: String,
    #[sqlx_model_hints(varchar)]
    content_id: String,
    #[sqlx_model_hints(varchar)]
    price_score_ratio: String,
    #[sqlx_model_hints(timestamptz)]
    valid_until: UtcDateTime,
    #[sqlx_model_hints(boolean, default)]
    finished: bool,
    #[sqlx_model_hints(timestamptz, default)]
    created_at: UtcDateTime,
    #[sqlx_model_hints(timestamptz, default)]
    updated_at: Option<UtcDateTime>,
  }
}

impl CampaignHub {
  pub async fn sync_x_collabs(&self) -> AsamiResult<Vec<CollabRequest>> {
    use twitter_v2::{TwitterApi, authorization::BearerToken, api_result::*};
    use tokio::time::*;

    let mut reqs = vec![];
    let conf = &self.state.settings.x;
    let auth = BearerToken::new(&conf.bearer_token);
    let api = TwitterApi::new(auth);
    
    for campaign in self.select().finished_eq(false).site_eq(Site::X).all().await? {
      let post_id = campaign.attrs.content_id.parse::<u64>()
        .map_err(|_| Error::Validation("content_id".into(), "was stored in the db not as u64".into()))?;

      let reposts = api.get_tweet_retweeted_by(post_id).send().await?;

      let mut page = Some(reposts);

      while let Some(reposts) = page {
        let payload = reposts.payload();
        let Some(data) = payload.data() else { break };

        for user in data {
          let Some(handle) = self.state.handle()
            .select()
            .user_id_eq(&user.id.to_string())
            .optional().await? else { continue };

          let not_exists = self.state.collab_request().select()
            .handle_id_eq(handle.attrs.id.clone())
            .campaign_id_eq(campaign.attrs.id.clone())
            .count().await? == 0;

          if not_exists {
            reqs.push(
              self.state.collab_request().insert(InsertCollabRequest{
                campaign_id: campaign.attrs.id.clone(),
                handle_id: handle.attrs.id,
              }).save().await?
            );
          }
        }

        page = reposts.next_page().await?;
        if page.is_some() { sleep(Duration::from_millis(200)).await; }
      }
    }
    Ok(reqs)
  }
}

model!{
  state: App,
  table: collab_requests,
  struct CollabRequest {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(varchar)]
    campaign_id: String,
    #[sqlx_model_hints(varchar)]
    handle_id: String, 
    #[sqlx_model_hints(varchar, default)]
    tx_hash: Option<String>,
    #[sqlx_model_hints(collab_request_status, default)]
    status: CollabRequestStatus,
  },
  belongs_to {
    Campaign(campaign_id),
    Handle(handle_id),
  }
}

impl CollabRequestHub {
  pub async fn submit_all(&self) -> AsamiResult<Vec<CollabRequest>> {
    let mut submitted = vec![];
    let reqs = self.select().status_eq(CollabRequestStatus::Received).all().await?;
    let params = reqs.iter().map(|r| r.as_param() ).collect();

    let tx_hash = self.state.on_chain.contract
      .admin_make_collabs(params)
      .send().await?
      .tx_hash()
      .encode_hex();

    for req in reqs {
      submitted.push(
        req.update().status(CollabRequestStatus::Submitted).tx_hash(Some(tx_hash.clone())).save().await?
      );
    }

    Ok(submitted)
  }
}

impl CollabRequest {
  pub async fn done(self) -> sqlx::Result<Self> {
    self.update().status(CollabRequestStatus::Done).save().await
  }

  pub fn as_param(&self) -> on_chain::AdminMakeCollabsInput {
    on_chain::AdminMakeCollabsInput{
      campaign_id: u256(&self.attrs.campaign_id),
      handle_id: u256(&self.attrs.handle_id),
    }
  }
}

model!{
  state: App,
  table: collabs,
  struct Collab {
    #[sqlx_model_hints(varchar)]
    id: String,
    #[sqlx_model_hints(varchar)]
    campaign_id: String,
    #[sqlx_model_hints(varchar)]
    handle_id: String, 
    #[sqlx_model_hints(varchar)]
    gross: String,
    #[sqlx_model_hints(varchar)]
    fee: String,
    #[sqlx_model_hints(varchar)]
    proof: Option<String>,
    #[sqlx_model_hints(varchar)]
    created_at: String,
  }
}

model!{
  state: App,
  table: campaign_topics,
  struct CampaignTopic {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(varchar)]
    campaign_id: String,
    #[sqlx_model_hints(varchar)]
    topic_id: String,
  }
}

model!{
  state: App,
  table: topics,
  struct Topic {
    #[sqlx_model_hints(varchar)]
    id: String,
    #[sqlx_model_hints(varchar)]
    name: String,
  }
}

// This is an account profile when taking the Collaborator role.
model!{
  state: App,
  table: indexer_states,
  struct IndexerState {
    #[sqlx_model_hints(int4)]
    id: i32,
    #[sqlx_model_hints(int8, default)]
    x_handle_verification_checkpoint: i64,
    #[sqlx_model_hints(varchar, default)]
    suggested_price_per_point: String,
    #[sqlx_model_hints(decimal, default)]
    last_synced_block: Decimal,
  }
}

impl IndexerStateHub {
  pub async fn get(&self) -> sqlx::Result<IndexerState> {
    let Some(existing) = self.find_optional(1).await? else { 
      return self.insert(InsertIndexerState{id: 1}).save().await
    };
    Ok(existing)
  }
}

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
    let app = &self.state;
    let on_chain = &app.on_chain;
    let state = app.indexer_state().get().await?;
    
    let to_block = on_chain.contract.client().get_block_number().await?
      - app.settings.rsk.reorg_protection_padding;

    let events = on_chain.events(d_to_u64(state.attrs.last_synced_block), to_block).await?;

    for (event, meta) in events {
      let existing = app.synced_event().select()
        .block_number_eq( u64_to_d(meta.block_number) )
        .log_index_eq( meta.log_index.encode_hex() )
        .count().await?;

      if existing > 0 {
        continue;
      } else {
        let data = serde_json::to_string(&event)
          .map_err(|e| Error::Runtime(format!("could not serialize event {} {}", event, e)))?;

        app.synced_event().insert(InsertSyncedEvent{
          address: meta.address.encode_hex(),
          block_number: u64_to_d(meta.block_number),
          block_hash: meta.block_hash.encode_hex(),
          tx_hash: meta.transaction_hash.encode_hex(),
          tx_index: u64_to_d(meta.transaction_index),
          log_index: meta.log_index.encode_hex(),
          data
        }).save().await?;
      };

      match event {
        AsamiContractEvents::AccountSavedFilter(e) => {
          let a = e.account;
          match app.account().find_optional(a.id.encode_hex()).await? {
            Some(account) => {
              account.update()
                .addr(Some(a.addr.encode_hex()))
                .unclaimed_asami_tokens(a.unclaimed_asami_tokens.encode_hex())
                .unclaimed_doc_rewards(a.unclaimed_asami_tokens.encode_hex())
                .nostr_self_managed(a.nostr_self_managed)
                .nostr_abuse_proven(a.nostr_abuse_proven)
                .save().await?;
            },
            None => {
              app.account().insert(InsertAccount{
                addr: Some(a.addr.encode_hex()),
                name: None,
                unclaimed_asami_tokens: a.unclaimed_asami_tokens.encode_hex(),
                unclaimed_doc_rewards: a.unclaimed_asami_tokens.encode_hex(),
                nostr_self_managed: a.nostr_self_managed,
                nostr_abuse_proven: a.nostr_abuse_proven,
              })
              .save().await?
              .update().id(a.id.encode_hex())
              .save().await?;
            }
          }
        },
        AsamiContractEvents::HandleSavedFilter(e) => {
          let maybe_req = app.handle_request().select()
            .tx_hash_eq(meta.transaction_hash.encode_hex())
            .username_eq(e.handle.username.clone())
            .status_eq(HandleRequestStatus::Submitted)
            .optional().await?;

          if let Some(h) = maybe_req {
            h.done().await?;
          }

          match app.handle().find_optional(e.handle.id.encode_hex()).await? {
            Some(handle) => {
              let h = e.handle;
              for old in handle.handle_topic_vec().await? {
                old.delete().await?;
              }

              for new in h.topics {
                app.handle_topic().insert(InsertHandleTopic{
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
              app.handle().insert(InsertHandle {
                id: h.id.encode_hex(),
                account_id: h.account_id.encode_hex(),
                site: Site::from_on_chain(h.site),
                username: h.username,
                user_id: h.user_id,
                price: h.price.encode_hex(),
                score: h.score.encode_hex(),
                nostr_affine_x: h.nostr_affine_x.encode_hex(),
                nostr_affine_y: h.nostr_affine_y.encode_hex(),
              }).save().await?;

              for new in h.topics {
                app.handle_topic().insert(InsertHandleTopic{
                  handle_id: h.id.encode_hex(),
                  topic_id: new.encode_hex(),
                });
              }
            }
          }
        },
        AsamiContractEvents::ApprovalFilter(_) => {
          let req = app.campaign_request().select()
            .status_eq(CampaignRequestStatus::Paid)
            .approval_tx_hash_eq(meta.transaction_hash.encode_hex())
            .optional().await?;

          if let Some(r) = req {
            r.approve().await?;
          }
        }
        AsamiContractEvents::CampaignSavedFilter(e) => {
          let req = app.campaign_request().select()
            .content_id_eq(&e.campaign.content_id)
            .status_eq(CampaignRequestStatus::Submitted)
            .submission_tx_hash_eq(meta.transaction_hash.encode_hex())
            .optional().await?;

          if let Some(r) = req {
            r.done().await?;
          }

          match app.campaign().find_optional(e.campaign.id.encode_hex()).await? {
            Some(campaign) => {
              let remaining = e.campaign.remaining;
              campaign.update()
                .remaining(remaining.encode_hex())
                .finished(remaining == 0.into())
                .save().await?;
            },
            None => {
              let c = e.campaign;
              app.campaign().insert(InsertCampaign{
                id: c.id.encode_hex(),
                account_id: c.account_id.encode_hex(),
                site: Site::from_on_chain(c.site),
                budget: c.budget.encode_hex(),
                remaining: c.remaining.encode_hex(),
                content_id: c.content_id.encode_hex(),
                price_score_ratio: c.price_score_ratio.encode_hex(),
                valid_until: i_to_utc(c.valid_until),
              }).save().await?;

              for new in c.topics {
                app.campaign_topic().insert(InsertCampaignTopic{
                  campaign_id: c.id.encode_hex(),
                  topic_id: new.encode_hex(),
                });
              }
            }
          }
        },
        AsamiContractEvents::CollabSavedFilter(e) => {
          let req = app.collab_request().select()
            .status_eq(CollabRequestStatus::Submitted)
            .tx_hash_eq(meta.transaction_hash.encode_hex())
            .optional().await?;

          if let Some(r) = req {
            r.done().await?;
          }

          if app.collab().find_optional(e.collab.id.encode_hex()).await?.is_none() {
            let c = e.collab;
            app.collab().insert(InsertCollab{
              id: c.id.encode_hex(),
              campaign_id: c.campaign_id.encode_hex(),
              handle_id: c.handle_id.encode_hex(),
              gross: c.gross.encode_hex(),
              fee: c.fee.encode_hex(),
              proof: Some(c.proof),
              created_at: c.created_at.encode_hex(),
            }).save().await?;
          }
        },
        AsamiContractEvents::HandleUpdateSavedFilter(_) => {
          let req = app.handle_update_request().select()
            .status_eq(HandleUpdateRequestStatus::Submitted)
            .tx_hash_eq(meta.transaction_hash.encode_hex())
            .optional().await?;

          if let Some(r) = req {
            r.done().await?;
          }
        }
        _ => {}
      }
    }

    state.update().last_synced_block(u64_to_d(to_block)).save().await?;

    Ok(())
  }
}

// Unsafe conversion for values that we know for sure have an U256 hex encoded value.
pub fn u256<T: AsRef<str> + std::fmt::Debug>(u: T) -> U256 {
  U256::decode_hex(u).unwrap()
}

pub fn wei<T: AsRef<str>>(t: T) -> U256 {
  U256::from_dec_str(t.as_ref()).unwrap()
}

pub fn weihex<T: AsRef<str>>(t: T) -> String {
  wei(t).encode_hex()
}

pub fn u<T: AsRef<str>>(t: T) -> U256 {
  wei(t) * U256::from(10).pow(U256::from(18))
}

pub fn uhex<T: AsRef<str>>(t: T) -> String {
  u(t).encode_hex()
}

fn i_to_utc(u: U256) -> UtcDateTime {
  // We're absolutely sure this conversion is always possible.
  Utc.timestamp_opt(u.as_u64() as i64, 0).unwrap()
}

fn utc_to_i(d: UtcDateTime) -> U256 {
  d.timestamp().into()
}

fn u64_to_d(u: U64) -> Decimal {
  Decimal::from_u64(u.as_u64()).unwrap()
}

fn d_to_u64(d: Decimal) -> U64 {
  U64::from_dec_str(&d.to_string()).unwrap()
}

