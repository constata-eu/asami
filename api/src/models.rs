use super::*;
pub use sqlx::{self, types::Decimal};
pub use serde::{Serialize, Deserialize};
use sqlx_models_orm::model;
pub use chrono::{DateTime, Duration, Utc, Datelike, TimeZone};
pub type UtcDateTime = DateTime<Utc>;
pub use juniper::GraphQLEnum;

pub mod hasher;

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize, sqlx::Type, GraphQLEnum)]
#[sqlx(type_name = "site", rename_all = "snake_case")]
pub enum Site {
  X,
  Instagram,
  Nostr,
}

impl sqlx::postgres::PgHasArrayType for Site {
  fn array_type_info() -> sqlx::postgres::PgTypeInfo {
    sqlx::postgres::PgTypeInfo::with_name("_site")
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "account_status", rename_all = "snake_case")]
pub enum AccountStatus {
  Managed,  // Managed by the administrator.
  Claiming, // In process of being claimed by the account holder.
  Claimed,  // Claimed by the account holder, not managed by admin again.
}

impl sqlx::postgres::PgHasArrayType for AccountStatus {
  fn array_type_info() -> sqlx::postgres::PgTypeInfo {
    sqlx::postgres::PgTypeInfo::with_name("_account_status")
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "handle_status", rename_all = "snake_case")]
pub enum HandleStatus {
  Unverified, // Newly created, never sent on-chain.
  Verified,   // Verified off-chain.
  Appraised,  // Appraised off-chain.
  Submitted,  // Sent, after this we listen for events regarding this handle.
  Active,     // Local DB knows this handle is now on-chain.
}

impl sqlx::postgres::PgHasArrayType for HandleStatus {
  fn array_type_info() -> sqlx::postgres::PgTypeInfo {
    sqlx::postgres::PgTypeInfo::with_name("_handle_status")
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "campaign_request_status", rename_all = "snake_case")]
pub enum CampaignRequestStatus {
  Received,   // The request was received by a managed user to create a campaign.
  Paid,       // We've got payment (through proprietary payment methods).
  Submitted,  // We've tried to submit the request on-chain.
  Active,     // We've got the event that creates this campaign.
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
  Active,     // We've got the event that creates this collab.
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
    #[sqlx_model_hints(int4, default)]
    id: i32,
    name: String,
    #[sqlx_model_hints(timestamptz, default)]
    created_at: UtcDateTime,
    #[sqlx_model_hints(timestamptz, default)]
    updated_at: Option<UtcDateTime>,
    #[sqlx_model_hints(account_status, default)]
    status: AccountStatus,
  }
}

impl Account {
  pub async fn add_handle(&self, site: Site, value: &str) -> sqlx::Result<Handle> {
    self.state.handle().insert(InsertHandle{
      account_id: self.attrs.id,
      value: value.to_string(),
      site: site,
    }).save().await
  }

  pub async fn create_campaign_request(&self, site: Site, content_id: &str, budget: Decimal) -> sqlx::Result<CampaignRequest> {
    self.state.campaign_request().insert(InsertCampaignRequest{
      account_id: self.attrs.id,
      site: site,
      budget,
      content_id: content_id.to_string()
    }).save().await
  }
}

model!{
  state: App,
  table: handles,
  struct Handle {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(int4)]
    account_id: i32,
    #[sqlx_model_hints(varchar)]
    value: String,
    #[sqlx_model_hints(site)]
    site: Site,
    #[sqlx_model_hints(handle_status, default)]
    status: HandleStatus,
    #[sqlx_model_hints(varchar, default)]
    fixed_id: Option<String>,
    #[sqlx_model_hints(decimal, default)]
    price: Option<Decimal>,
    #[sqlx_model_hints(decimal, default)]
    score: Option<Decimal>,
    #[sqlx_model_hints(varchar, default)]
    verification_message_id: Option<String>,
    #[sqlx_model_hints(varchar, default)]
    tx_hash: Option<String>,
  },
  has_many {
    HandleTopic(handle_id)
  }
}

impl HandleHub {
  pub async fn verify_and_appraise_all(&self) -> AsamiResult<Vec<Handle>> {
    self.verify_and_appraise_x().await
  }

  pub async fn verify_and_appraise_x(&self) -> AsamiResult<Vec<Handle>> {
    use twitter_v2::{TwitterApi, authorization::BearerToken, query::*, api_result::*};
    use rust_decimal::prelude::*;
    use tokio::time::*;
    use rust_decimal_macros::*;

    let mut handles = vec![];

    let msg_regex = regex::Regex::new(r#"\@asami_club \[(\d)\]"#)?;
    let conf = &self.state.settings.x;
    let auth = BearerToken::new(&conf.bearer_token);
    let api = TwitterApi::new(auth);
    let indexer_state = self.state.indexer_state().get().await?;

    dbg!("here");
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
          let Ok(account_id) = capture[1].parse::<i32>() else { continue };

          let Some(handle) = self.state.handle().select()
            .status_eq(&HandleStatus::Unverified)
            .site_eq(&Site::X)
            .value_eq(&author.username)
            .account_id_eq(&account_id)
            .optional().await? else { continue };

          let score = Decimal::from_usize(public_metrics.followers_count).unwrap_or(dec!(1)) * dec!(0.85);
          let price = indexer_state.suggested_price_per_point() * score;
          handles.push(
            handle.verify(post.id.to_string(), author_id.to_string()).await?
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

    Ok(handles)
  }
}

impl Handle {
  pub async fn verify(self, verification_message_id: String, fixed_id: String) -> sqlx::Result<Self> {
    self.update()
      .verification_message_id(Some(verification_message_id))
      .fixed_id(Some(fixed_id))
      .status(HandleStatus::Verified)
      .save().await
  }

  pub async fn appraise(self, price: Decimal, score: Decimal) -> sqlx::Result<Self> {
    self.update().price(Some(price)).score(Some(score)).status(HandleStatus::Appraised).save().await
  }

  pub async fn submit(self) -> AsamiResult<Self> {
    let tx_hash = self.state.on_chain.send_add_handle(&self).await?;
    Ok(self.update().status(HandleStatus::Submitted).tx_hash(Some(tx_hash)).save().await?)
  }

  pub async fn activate(self) -> sqlx::Result<Self> {
    self.update().status(HandleStatus::Active).save().await
  }

  pub async fn topic_ids(&self) -> sqlx::Result<Vec<i32>> {
    Ok(self.handle_topic_vec().await?.iter().map(|t| t.attrs.topic_id ).collect())
  }
}

// This is an account profile when taking the Collaborator role.
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
  pub async fn account_ids(&self) -> sqlx::Result<Vec<i32>> {
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
    #[sqlx_model_hints(int4)]
    account_id: i32,
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

model!{
  state: App,
  table: campaign_requests,
  struct CampaignRequest {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(int4)]
    account_id: i32,
    #[sqlx_model_hints(decimal)]
    budget: Decimal,
    #[sqlx_model_hints(site)]
    site: Site,
    #[sqlx_model_hints(varchar)]
    content_id: String,
    #[sqlx_model_hints(campaign_request_status, default)]
    status: CampaignRequestStatus,
    #[sqlx_model_hints(varchar, default)]
    tx_hash: Option<String>,
    #[sqlx_model_hints(timestamptz, default)]
    created_at: UtcDateTime,
    #[sqlx_model_hints(timestamptz, default)]
    updated_at: Option<UtcDateTime>,
  }
}

impl CampaignRequest {
  pub async fn pay(self) -> AsamiResult<Self> {
    Ok(self.update().status(CampaignRequestStatus::Paid).save().await?)
  }

  pub async fn submit(self) -> AsamiResult<Self> {
    let tx_hash = self.state.on_chain.send_campaign_request(&self).await?;
    Ok(self.update().status(CampaignRequestStatus::Submitted).tx_hash(Some(tx_hash)).save().await?)
  }

  pub async fn activate(self) -> sqlx::Result<Self> {
    self.update().status(CampaignRequestStatus::Active).save().await
  }
}

model!{
  state: App,
  table: campaigns,
  struct Campaign {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(decimal)]
    on_chain_id: Decimal,
    #[sqlx_model_hints(int4)]
    account_id: i32,
    #[sqlx_model_hints(site)]
    site: Site,
    #[sqlx_model_hints(decimal)]
    budget: Decimal,
    #[sqlx_model_hints(decimal)]
    remaining: Decimal,
    #[sqlx_model_hints(varchar)]
    content_id: String,
    #[sqlx_model_hints(varchar)]
    tx_hash: String,
    #[sqlx_model_hints(decimal)]
    block_number: Decimal,
    #[sqlx_model_hints(decimal)]
    log_index: Decimal,
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
    use twitter_v2::{TwitterApi, authorization::BearerToken, query::*, api_result::*};
    use rust_decimal::prelude::*;
    use tokio::time::*;
    use rust_decimal_macros::*;

    let mut reqs = vec![];
    let conf = &self.state.settings.x;
    let auth = BearerToken::new(&conf.bearer_token);
    let api = TwitterApi::new(auth);
    
    for campaign in self.select().finished_eq(false).site_eq(Site::X).all().await? {
      let post_id = campaign.attrs.content_id.parse::<u64>()
        .map_err(|_| Error::Validation("content_id".into(), "was stored in the db not as u64".into()))?;

      dbg!(&post_id);

      let reposts = api.get_tweet_retweeted_by(post_id).send().await?;

      dbg!(&reposts);

      let mut page = Some(reposts);

      while let Some(reposts) = page {
        let payload = reposts.payload();
        let Some(data) = payload.data() else { break };

        for user in data {
          let Some(handle) = self.state.handle().select().fixed_id_eq(&user.id.to_string()).optional().await? else { continue };

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
    #[sqlx_model_hints(int4)]
    campaign_id: i32,
    #[sqlx_model_hints(int4)]
    handle_id: i32, 
    #[sqlx_model_hints(collab_request_status, default)]
    status: CollabRequestStatus,
  }
}

model!{
  state: App,
  table: collabs,
  struct Collab {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(int4)]
    campaign_id: i32,
    #[sqlx_model_hints(int4)]
    handle_id: i32, 
    #[sqlx_model_hints(decimal)]
    reward: Decimal,
  }
}

model!{
  state: App,
  table: campaign_topics,
  struct CampaignTopic {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    campaign_id: i32,
    topic_id: i32,
  }
}

model!{
  state: App,
  table: topics,
  struct Topic {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(varchar)]
    name: String,
  }
}

model!{
  state: App,
  table: handle_topics,
  struct HandleTopic {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(int4)]
    handle_id: i32,
    #[sqlx_model_hints(int4)]
    topic_id: i32,
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
    #[sqlx_model_hints(int8, default)]
    last_synced_block: i64,
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
