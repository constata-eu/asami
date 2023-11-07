use super::*;
use super::on_chain::{self, AsamiContractEvents};
pub use sqlx::{self, types::Decimal};
pub use serde::{Serialize, Deserialize};
use sqlx_models_orm::model;
pub use chrono::{DateTime, Duration, Utc, Datelike, TimeZone};
pub type UtcDateTime = DateTime<Utc>;
pub use juniper::GraphQLEnum;
use ethers::{ types::{U256, U64}, abi::AbiEncode, middleware::Middleware};
use rust_decimal::prelude::ToPrimitive;

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

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize, sqlx::Type)]
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

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize, sqlx::Type)]
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

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize, sqlx::Type)]
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
    #[sqlx_model_hints(decimal, default)]
    id: Decimal,
    #[sqlx_model_hints(varchar)]
    name: Option<String>,
    #[sqlx_model_hints(varchar)]
    addr: Option<String>,
    #[sqlx_model_hints(decimal)]
    unclaimed_asami_tokens: Decimal,
    #[sqlx_model_hints(decimal)]
    unclaimed_doc_rewards: Decimal,
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
      account_id: self.attrs.id,
      username: username.to_string(),
      site: site,
    }).save().await
  }

  pub async fn create_campaign_request(
    &self,
    site: Site,
    content_id: &str,
    budget: Decimal,
    price_score_ratio: Decimal,
    valid_until: UtcDateTime,
  ) -> sqlx::Result<CampaignRequest> {
    self.state.campaign_request().insert(InsertCampaignRequest{
      account_id: self.attrs.id,
      site: site,
      budget,
      content_id: content_id.to_string(),
      price_score_ratio,
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
    #[sqlx_model_hints(decimal)]
    account_id: Decimal,
    #[sqlx_model_hints(site)]
    site: Site,
    #[sqlx_model_hints(varchar)]
    username: String,
    #[sqlx_model_hints(varchar, default)]
    user_id: Option<String>,
    #[sqlx_model_hints(decimal, default)]
    price: Option<Decimal>,
    #[sqlx_model_hints(decimal, default)]
    score: Option<Decimal>,
    #[sqlx_model_hints(decimal, default)]
    nostr_affine_x: Option<Decimal>,
    #[sqlx_model_hints(decimal, default)]
    nostr_affine_y: Option<Decimal>,
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
    use rust_decimal_macros::*;

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
          let Ok(account_id) = capture[1].parse::<Decimal>() else { continue };

          let Some(req) = self.state.handle_request().select()
            .status_eq(&HandleRequestStatus::Unverified)
            .site_eq(&Site::X)
            .username_eq(&author.username)
            .account_id_eq(&account_id)
            .optional().await? else { continue };

          let score = Decimal::from_usize(public_metrics.followers_count).unwrap_or(dec!(1)) * dec!(0.85);
          let price = indexer_state.suggested_price_per_point() * score;
          handle_requests.push(
            req.verify(author_id.to_string()).await?
            .appraise(price, score.ceil()).await?
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

  pub async fn appraise(self, price: Decimal, score: Decimal) -> sqlx::Result<Self> {
    self.update().price(Some(price)).score(Some(score)).status(HandleRequestStatus::Appraised).save().await
  }

  pub async fn done(self) -> sqlx::Result<Self> {
    self.update().status(HandleRequestStatus::Done).save().await
  }

  pub async fn topic_ids(&self) -> sqlx::Result<Vec<Decimal>> {
    Ok(self.handle_request_topic_vec().await?.iter().map(|t| t.attrs.topic_id ).collect())
  }

  pub async fn as_param(&self) -> AsamiResult<on_chain::Handle> {
    let a = &self.attrs;

    let invalid_state = Err(Error::Runtime("Invalid state on appraised handle request".into()));

    let Some(price) = a.price.map(|i| d_to_i(&i)) else { return invalid_state };
    let Some(score) = a.score.map(|i| d_to_i(&i)) else { return invalid_state };
    let Some(user_id) = a.user_id.clone() else { return invalid_state };

    let nostr_affine_x = a.nostr_affine_x.map(|i| d_to_i(&i) ).unwrap_or(0.into());
    let nostr_affine_y = a.nostr_affine_y.map(|i| d_to_i(&i) ).unwrap_or(0.into());

    let topics = self.topic_ids().await?.iter().map(|i| d_to_i(&i) ).collect();

    Ok(on_chain::Handle {
      id: 0.into(),
      account_id: d_to_i(&a.account_id), 
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
    #[sqlx_model_hints(decimal)]
    topic_id: Decimal,
  }
}

model!{
  state: App,
  table: handles,
  struct Handle {
    #[sqlx_model_hints(decimal)]
    id: Decimal,
    #[sqlx_model_hints(decimal)]
    account_id: Decimal,
    #[sqlx_model_hints(site)]
    site: Site,
    #[sqlx_model_hints(varchar)]
    username: String,
    #[sqlx_model_hints(varchar)]
    user_id: String,
    #[sqlx_model_hints(decimal)]
    price: Decimal,
    #[sqlx_model_hints(decimal)]
    score: Decimal,
    #[sqlx_model_hints(decimal)]
    nostr_affine_x: Decimal,
    #[sqlx_model_hints(decimal)]
    nostr_affine_y: Decimal,
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
    #[sqlx_model_hints(decimal)]
    handle_id: Decimal,
    #[sqlx_model_hints(decimal)]
    topic_id: Decimal,
  }
}

model!{
  state: App,
  table: handle_update_requests,
  struct HandleUpdateRequest {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(decimal)]
    handle_id: Decimal,
    #[sqlx_model_hints(varchar)]
    username: Option<String>,
    #[sqlx_model_hints(decimal)]
    price: Option<Decimal>,
    #[sqlx_model_hints(decimal)]
    score: Option<Decimal>,
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
    #[sqlx_model_hints(decimal)]
    topic_id: Decimal,
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
  pub async fn account_ids(&self) -> sqlx::Result<Vec<Decimal>> {
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
    #[sqlx_model_hints(decimal)]
    account_id: Decimal,
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
    #[sqlx_model_hints(decimal)]
    account_id: Decimal,
    #[sqlx_model_hints(decimal)]
    budget: Decimal,
    #[sqlx_model_hints(site)]
    site: Site,
    #[sqlx_model_hints(varchar)]
    content_id: String,
    #[sqlx_model_hints(decimal)]
    price_score_ratio: Decimal,
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
    let total: Decimal = reqs.iter().map(|r| r.budget() ).sum();

    let tx_hash = rsk.doc_contract.approve(
      rsk.contract.address(),
      total.to_u64().unwrap().into()
    ).send().await?.tx_hash().encode_hex();

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
      .iter().map(|t| d_to_i(&t.attrs.topic_id) ).collect();

    Ok(on_chain::AdminCampaignInput{
      account_id: d_to_i(&self.attrs.account_id),
      attrs: on_chain::CampaignInput {
        site: self.attrs.site as u8,
        budget: d_to_i(&self.attrs.budget),
        content_id: self.attrs.content_id.clone(),
        price_score_ratio: d_to_i(&self.attrs.price_score_ratio),
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
    #[sqlx_model_hints(decimal)]
    topic_id: Decimal,
  }
}


model!{
  state: App,
  table: campaigns,
  struct Campaign {
    #[sqlx_model_hints(decimal)]
    id: Decimal,
    #[sqlx_model_hints(decimal)]
    account_id: Decimal,
    #[sqlx_model_hints(site)]
    site: Site,
    #[sqlx_model_hints(decimal)]
    budget: Decimal,
    #[sqlx_model_hints(decimal)]
    remaining: Decimal,
    #[sqlx_model_hints(varchar)]
    content_id: String,
    #[sqlx_model_hints(decimal)]
    price_score_ratio: Decimal,
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
          let Some(handle) = self.state.handle().select().user_id_eq(&user.id.to_string()).optional().await? else { continue };

          if self.state.collab_request().select().handle_id_eq(handle.attrs.id).campaign_id_eq(campaign.attrs.id).count().await? == 0 {
            reqs.push(
              self.state.collab_request().insert(InsertCollabRequest{
                campaign_id: campaign.attrs.id,
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
    #[sqlx_model_hints(decimal)]
    campaign_id: Decimal,
    #[sqlx_model_hints(decimal)]
    handle_id: Decimal, 
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
    let params = reqs.iter().map(|r| r.as_param()).collect();

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
      campaign_id: d_to_i(&self.attrs.campaign_id),
      handle_id: d_to_i(&self.attrs.handle_id)
    }
  }
}

model!{
  state: App,
  table: collabs,
  struct Collab {
    #[sqlx_model_hints(decimal)]
    id: Decimal,
    #[sqlx_model_hints(decimal)]
    campaign_id: Decimal,
    #[sqlx_model_hints(decimal)]
    handle_id: Decimal, 
    #[sqlx_model_hints(decimal)]
    gross: Decimal,
    #[sqlx_model_hints(decimal)]
    fee: Decimal,
    #[sqlx_model_hints(varchar)]
    proof: Option<String>,
    #[sqlx_model_hints(decimal)]
    created_at: Decimal,
  }
}

model!{
  state: App,
  table: campaign_topics,
  struct CampaignTopic {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(decimal)]
    campaign_id: Decimal,
    #[sqlx_model_hints(decimal)]
    topic_id: Decimal,
  }
}

model!{
  state: App,
  table: topics,
  struct Topic {
    #[sqlx_model_hints(decimal)]
    id: Decimal,
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
    #[sqlx_model_hints(decimal, default)]
    suggested_price_per_point: Decimal,
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
    #[sqlx_model_hints(decimal)]
    log_index: Decimal,
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
    
    let to_block = u64_to_d(on_chain.contract.client().get_block_number().await?)
      - app.settings.rsk.reorg_protection_padding;
    let events = on_chain.events(&state.attrs.last_synced_block, &to_block).await?;

    for (event, meta) in events {
      let existing = app.synced_event().select()
        .block_number_eq( u64_to_d(meta.block_number) )
        .log_index_eq( i_to_d(meta.log_index) )
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
          log_index: i_to_d(meta.log_index),
          data
        }).save().await?;
      };

      match event {
        AsamiContractEvents::AccountSavedFilter(e) => {
          let a = e.account;
          match app.account().find_optional(i_to_d(a.id)).await? {
            Some(account) => {
              account.update()
                .addr(Some(a.addr.encode_hex()))
                .unclaimed_asami_tokens(i_to_d(a.unclaimed_asami_tokens))
                .unclaimed_doc_rewards(i_to_d(a.unclaimed_asami_tokens))
                .nostr_self_managed(a.nostr_self_managed)
                .nostr_abuse_proven(a.nostr_abuse_proven)
                .save().await?;
            },
            None => {
              app.account().insert(InsertAccount{
                addr: Some(a.addr.encode_hex()),
                name: None,
                unclaimed_asami_tokens: i_to_d(a.unclaimed_asami_tokens),
                unclaimed_doc_rewards: i_to_d(a.unclaimed_asami_tokens),
                nostr_self_managed: a.nostr_self_managed,
                nostr_abuse_proven: a.nostr_abuse_proven,
              })
              .save().await?
              .update().id(i_to_d(a.id))
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

          match app.handle().find_optional(i_to_d(e.handle.id)).await? {
            Some(handle) => {
              let h = e.handle;
              for old in handle.handle_topic_vec().await? {
                old.delete().await?;
              }

              for new in h.topics {
                app.handle_topic().insert(InsertHandleTopic{
                  handle_id: handle.attrs.id,
                  topic_id: i_to_d(new),
                });
              }

              handle.update()
                .username(h.username)
                .price(i_to_d(h.price))
                .score(i_to_d(h.score))
                .save().await?;
            },
            None => {
              let h = e.handle;
              app.handle().insert(InsertHandle {
                id: i_to_d(h.id),
                account_id: i_to_d(h.account_id),
                site: Site::from_on_chain(h.site),
                username: h.username,
                user_id: h.user_id,
                price: i_to_d(h.price),
                score: i_to_d(h.score),
                nostr_affine_x: i_to_d(h.nostr_affine_x),
                nostr_affine_y: i_to_d(h.nostr_affine_y),
              }).save().await?;

              for new in h.topics {
                app.handle_topic().insert(InsertHandleTopic{
                  handle_id: i_to_d(h.id),
                  topic_id: i_to_d(new),
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

          match app.campaign().find_optional(i_to_d(e.campaign.id)).await? {
            Some(campaign) => {
              let remaining = i_to_d(e.campaign.remaining);
              campaign.update()
                .remaining(remaining)
                .finished(remaining == Decimal::ZERO)
                .save().await?;
            },
            None => {
              let c = e.campaign;
              app.campaign().insert(InsertCampaign{
                id: i_to_d(c.id),
                account_id: i_to_d(c.account_id),
                site: Site::from_on_chain(c.site),
                budget: i_to_d(c.budget),
                remaining: i_to_d(c.remaining),
                content_id: c.content_id,
                price_score_ratio: i_to_d(c.price_score_ratio),
                valid_until: i_to_utc(c.valid_until),
              }).save().await?;

              for new in c.topics {
                app.campaign_topic().insert(InsertCampaignTopic{
                  campaign_id: i_to_d(c.id),
                  topic_id: i_to_d(new),
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

          if app.collab().find_optional(i_to_d(e.collab.id)).await?.is_none() {
            let c = e.collab;
            app.collab().insert(InsertCollab{
              id: i_to_d(c.id),
              campaign_id: i_to_d(c.campaign_id),
              handle_id: i_to_d(c.handle_id),
              gross: i_to_d(c.gross),
              fee: i_to_d(c.fee),
              proof: Some(c.proof),
              created_at: i_to_d(c.created_at),
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

    state.update().last_synced_block(to_block).save().await?;

    Ok(())
  }
}

fn d_to_i(d: &Decimal) -> U256 {
  // We're absolutely sure this conversion is always possible.
  U256::from_dec_str(&d.to_string()).unwrap()
}

fn i_to_d(u: U256) -> Decimal {
  // We're absolutely sure this conversion is always possible.
  Decimal::from_str_radix(u.encode_hex().trim_start_matches("0x"), 16).unwrap()
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
