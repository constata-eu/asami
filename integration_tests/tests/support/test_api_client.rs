use super::TestApp;
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
pub use galvanic_assert::{
  self,
  matchers::{collection::*, variant::*, *},
  *,
};
pub use api::{
  models::{self, u, U256, u256, hasher, Utc, wei},
  on_chain::{self, AsamiContractSigner, DocContract, AsamiContract, IERC20, Provider, SignerMiddleware, Address, Http}
};
use rocket::{ http::Header, local::asynchronous::LocalResponse };
use jwt_simple::{ algorithms::*, prelude::*, };
pub use serde::{Serialize, de::DeserializeOwned, Deserialize};
use graphql_client;
use graphql_client::GraphQLQuery;
pub use api::Decimal;
use ethers::{middleware::Middleware, types::TransactionRequest, signers::{LocalWallet, Signer}};

#[derive(Deserialize)]
pub struct ApiError {
  pub error: String,
}

#[allow(dead_code)]
pub struct ApiClient<'a> {
  pub test_app: &'a TestApp,
  pub session_key: Option<ES256KeyPair>,
  pub session: Option<models::Session>,
  pub local_wallet: Option<LocalWallet>,
  pub contract: Option<AsamiContractSigner>,
  pub doc_contract: Option<DocContract>,
}

#[derive(Clone)]
pub struct BaseLineScenario {
  pub regular_campaign: models::Campaign,
  pub high_rate_campaign: models::Campaign,
  pub low_rate_campaign: models::Campaign,
  pub low_budget_campaign: models::Campaign,
}

impl<'b> ApiClient<'b> {
  pub async fn new(test_app: &'b TestApp) -> ApiClient<'b> {
    Self {
      test_app,
      local_wallet: None,
      session_key: None, 
      session: None,
      contract: None,
      doc_contract: None
    }
  }

  pub fn app(&self) -> api::App {
    self.test_app.app.clone()
  }

  pub async fn account(&self) -> models::Account {
    self.session.as_ref().unwrap().account().await.unwrap()
  }

  pub async fn user(&self) -> models::User {
    self.session.as_ref().unwrap().user().await.unwrap()
  }

  pub async fn x_handle(&self) -> models::Handle {
    self.account().await.handle_scope().site_eq(models::Site::X).one().await.unwrap()
  }

  pub async fn login(&mut self) {
    self.login_with_key( ES256KeyPair::generate() ).await
  }

  pub fn local_wallet(&self) -> &LocalWallet {
    self.local_wallet.as_ref().unwrap()
  }

  pub fn contract(&self) -> &AsamiContractSigner {
    self.contract.as_ref().unwrap()
  }

  pub fn doc_contract(&self) -> &DocContract {
    self.doc_contract.as_ref().unwrap()
  }

  pub async fn login_with_key(&mut self, key: ES256KeyPair) {
    let token = api::models::hasher::hexdigest(key.public_key().to_pem().unwrap().as_bytes());
    self.session_key = Some(key.with_key_id(&token));

    self.test_app.app.one_time_token()
      .insert(models::InsertOneTimeToken{
      value: token.to_string()
    }).save().await.unwrap();

    let login_pubkey = URL_SAFE_NO_PAD.encode(
      self.session_key.as_ref().unwrap().public_key().to_pem().unwrap()
    );

    let result: gql::create_session::ResponseData = self.gql(
      &gql::CreateSession::build_query(gql::create_session::Variables{}),
      vec![
        Header::new("Auth-Action", "Login"),
        Header::new("Auth-Method-Kind", "OneTimeToken"),
        Header::new("Auth-Data", token),
        Header::new("Login-Pubkey", login_pubkey),
        Header::new("New-Session-Recaptcha-Code", "SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c"),
      ]
    ).await;

    self.session = Some(self.test_app.app.session().find(&result.create_session.id).await.unwrap());
  }

  pub async fn build_baseline_scenario(&mut self) -> BaseLineScenario {
    self.login_with_key(ES256KeyPair::generate()).await;
    let rate = u256(self.test_app.app.indexer_state().get().await.unwrap().suggested_price_per_point());
    let budget = || { rate * wei("200") };

    let regular_campaign = self.build_x_campaign(budget(), rate, 2, &[]).await;
    let high_rate_campaign = self.build_instagram_campaign(budget(), rate * wei("2")).await;
    let low_rate_campaign = self.build_x_campaign(budget(), rate / wei("2"), 2, &[]).await;
    let low_budget_campaign = self.build_x_campaign(rate * wei("1"), rate, 2, &[]).await;

    self.test_app.run_idempotent_background_tasks_a_few_times().await;

    BaseLineScenario {
      regular_campaign: regular_campaign.reloaded().await.unwrap().campaign().await.unwrap().expect("regular"),
      high_rate_campaign: high_rate_campaign.reloaded().await.unwrap().campaign().await.unwrap().expect("high_rate"),
      low_rate_campaign: low_rate_campaign.reloaded().await.unwrap().campaign().await.unwrap().expect("low_rate"),
      low_budget_campaign: low_budget_campaign.reloaded().await.unwrap().campaign().await.unwrap().expect("low_budget"),
    }
  }

  pub async fn build_instagram_campaign(&self, budget: U256, rate: U256) -> models::CampaignRequest {
    let post = "C0T1wKQMS0v"; // This is the post shortcode.
    let two_days = Utc::now() + chrono::Duration::days(2);

    self.account().await.create_campaign_request(models::Site::Instagram, post, budget, rate, two_days, &[])
      .await.unwrap().pay()
      .await.unwrap()
  }

  pub async fn build_x_campaign(&self, budget: U256, rate: U256, days: i64, topics: &[models::Topic]) -> models::CampaignRequest {
    let post = "1716421161867710954";
    let valid_until = Utc::now() + chrono::Duration::days(days);

    self.account().await.create_campaign_request(models::Site::X, post, budget, rate, valid_until, topics)
      .await.unwrap().pay()
      .await.unwrap()
  }

  pub async fn create_x_campaign(&self, budget: U256, rate: U256) -> models::Campaign {
    self.create_x_campaign_extra(budget, rate, 2, &[]).await
  }

  pub async fn create_x_campaign_extra(&self, budget: U256, rate: U256, days: i64, topics: &[models::Topic]) -> models::Campaign {
    let campaign = self.build_x_campaign(budget, rate, days, topics).await;
    self.test_app.run_idempotent_background_tasks_a_few_times().await;
    campaign.reloaded().await.unwrap().campaign().await.unwrap().expect("campaign")
  }

  pub async fn create_self_managed_x_campaign(&self, budget: U256, rate: U256, days: i64) -> models::Campaign {
    let two_days = Utc::now() + chrono::Duration::days(days);

    self.doc_contract()
      .approve(self.contract().address(), budget)
      .send().await.expect("sending approval")
      .await.expect("getting approval receipt")
      .expect("approval receipt");

    let tx = self.app().on_chain_tx().send_tx(
      self.contract().make_campaigns(vec![on_chain::CampaignInput{
        site: models::Site::X as u8,
        budget: budget,
        content_id: "1716421161867710954".to_string(),
        price_score_ratio: rate,
        topics: vec![],
        valid_until: models::utc_to_i(two_days),
      }])
    ).await.unwrap();

    assert!(tx.submitted(), "not successful tx {}", tx.attrs.id);

    self.test_app.run_idempotent_background_tasks_a_few_times().await;

    self.test_app.app.campaign().select().order_by(models::CampaignOrderBy::Id).desc(true).one().await.unwrap()
  }

  pub async fn create_x_handle(&self, username: &str, rate: U256) {
    self.create_x_handle_with_score(username, rate, wei("10")).await
  }

  pub async fn create_x_handle_with_score(&self, username: &str, rate: U256, score: U256) {
    self.account().await.create_handle_request(models::Site::X, username).await.unwrap()
      .verify("179383862".into()).await.unwrap()
      .appraise(rate, score).await.unwrap();
    self.test_app.run_idempotent_background_tasks_a_few_times().await;
  }

  pub async fn gql_claim_account_request(&self, wallet: &LocalWallet) -> graphql_client::Response<gql::create_claim_account_request::ResponseData> {
    let rsk = &self.test_app.app.settings.rsk;
    let payload = models::make_login_to_asami_typed_data(rsk.chain_id).unwrap();
    let signature = wallet.sign_typed_data(&payload).await.unwrap().to_string();

    self.gql_response(
      &gql::CreateClaimAccountRequest::build_query(gql::create_claim_account_request::Variables{
        input: gql::create_claim_account_request::CreateClaimAccountRequestInput{signature}
      }),
      vec![]
    ).await
  }

  pub async fn submit_claim_account_request(&mut self) {
    let rsk = &self.app().settings.rsk;
    let wallet = self.test_app.make_wallet();
    self.gql_claim_account_request(&wallet).await;

    let tx = TransactionRequest::new().to(wallet.address()).value(u("10"));
    self.test_app.app.on_chain.contract.client()
      .send_transaction(tx, None)
      .await.expect("sending tx")
      .await.expect("waiting tx confirmation")
      .expect("tx result");

    let provider = Provider::<Http>::try_from(&rsk.rpc_url).unwrap();
    let client = std::sync::Arc::new(SignerMiddleware::new(provider, wallet.clone()));
    let address: Address = rsk.contract_address.parse().unwrap();
    self.contract = Some(AsamiContract::new(address, client.clone()));

    let doc_address: Address = rsk.doc_contract_address.parse().unwrap();
    self.doc_contract = Some(IERC20::new(doc_address, client.clone()));
    self.local_wallet = Some(wallet);
  }

  pub async fn claim_account(&mut self) { // Rsk address as string.
    self.submit_claim_account_request().await;
    self.test_app.run_idempotent_background_tasks_a_few_times().await;
  }

  pub async fn create_x_collab(&self, campaign: &models::Campaign) {
    campaign.make_collab(&self.x_handle().await).await.unwrap();
    self.test_app.run_idempotent_background_tasks_a_few_times().await;
  }

  pub async fn self_submit_fee_rate_vote(&self, rate: U256) -> api::AsamiResult<()> {
    self.contract().submit_fee_rate_vote(rate)
      .send().await?
      .await?
      .expect("extracting receipt");
    Ok(())
  }

  pub async fn self_remove_fee_rate_vote(&self) -> api::AsamiResult<()> {
    self.contract().remove_fee_rate_vote()
      .send().await?
      .await?
      .expect("extracting receipt");
    Ok(())
  }

  pub async fn self_submit_admin_vote(&self, candidate: Address) -> api::AsamiResult<()> {
    self.contract().submit_admin_vote(candidate)
      .send().await?
      .await?
      .expect("extracting receipt");
    Ok(())
  }

  pub async fn self_remove_admin_vote(&self) -> api::AsamiResult<()> {
    self.contract().remove_admin_vote()
      .send().await?
      .await?
      .expect("extracting receipt");
    Ok(())
  }

  pub async fn self_vest_admin_vote(&self, candidate: Address) -> api::AsamiResult<()> {
    self.contract().vest_admin_votes(vec![candidate])
      .send().await?
      .await?
      .expect("extracting receipt");
    Ok(())
  }

  pub async fn self_set_admin_address(&self, address: Address) -> api::AsamiResult<()> {
    self.contract().set_admin_address(address)
      .send().await?
      .await?
      .expect("extracting receipt");
    Ok(())
  }

  pub async fn gql<'a, T: core::fmt::Debug, Q>(&'a self, query: Q, extra_headers: Vec<Header<'static>>) -> T
    where Q: Serialize, T: DeserializeOwned
  {
    let response = self.gql_response(query, extra_headers).await;
    response.data.expect(&format!("No gql response. Error was {:?}", response.errors))
  }

  pub async fn gql_response<'a, T: core::fmt::Debug, Q>(&'a self, query: Q, extra_headers: Vec<Header<'static>>) -> graphql_client::Response<T>
    where Q: Serialize, T: DeserializeOwned
  {
    let query_str = serde_json::to_string(&query).expect("gql query was not JSON");
    self.post::<graphql_client::Response<T>, _>("/graphql/", query_str, extra_headers).await
  }

  pub fn make_auth_header<'a>(&'a self, path: &str, method: &str, nonce: i64, body: Option<&str>, query: Option<&str>) -> Header<'static> {
    let body_hash = body.map(|x| hasher::hexdigest(x.as_bytes()));
    let query_hash = query.map(|x| hasher::hexdigest(x.as_bytes()));
    let payload = serde_json::json![{
      "path": path,
      "method": method,
      "nonce": nonce.to_string(),
      "body_hash": body_hash,
      "query_hash": query_hash,
    }];

    let claims = Claims::with_custom_claims(payload, Duration::from_secs(30));
    let jwt = self.session_key.as_ref().unwrap().sign(claims).unwrap();
    Header::new("Authentication", jwt)
  }

  pub fn ok_auth_header(&self, path: &str, method: &str, body: Option<&str>, query: Option<&str>) -> Header<'static> {
    self.make_auth_header(path , method, chrono::Utc::now().timestamp_millis(), body, query)
  }

  pub async fn post_response<'a, B>(&'a self, path: &'a str, body: B, extra_headers: Vec<Header<'static>>) -> LocalResponse<'a>
  where
    B: AsRef<str> + AsRef<[u8]>,
  {
    let header = self.ok_auth_header(path, "POST", Some(body.as_ref()), None);
    let mut req = self.test_app.rocket_client.post(path).body(body).header(header);
    for h in extra_headers {
      req = req.header(h);
    }

    req.dispatch().await
  }

  pub async fn post_string<'a, B>(&'a self, path: &'a str, body: B, extra_headers: Vec<Header<'static>>) -> String
  where
    B: AsRef<str> + AsRef<[u8]>,
  {
    self
      .post_response(path, body, extra_headers)
      .await
      .into_string()
      .await
      .unwrap()
  }

  pub async fn post<'a, T, B>(&'a self, path: &'a str, body: B, extra_headers: Vec<Header<'static>>) -> T
  where
    T: DeserializeOwned,
    B: AsRef<str> + AsRef<[u8]>,
  {
    let string = self.post_string(path, body, extra_headers).await;
    serde_json::from_str(&string).unwrap_or_else(|_| panic!("Could not parse response {}", string))
  }

  pub async fn get_response<'a>(&'a self, path: &'a str) -> LocalResponse<'a> {
    self.test_app.rocket_client
      .post(path)
      .header(self.ok_auth_header(path, "POST", None, None))
      .dispatch().await
  }

  pub async fn asami_balance(&self) -> U256 {
    self.test_app.contract().balance_of(self.local_wallet().address()).call().await.unwrap()
  }

  pub async fn doc_balance(&self) -> U256 {
    self.test_app.doc_contract().balance_of(self.local_wallet().address()).call().await.unwrap()
  }

  pub async fn get_campaign_offers(&self) -> gql::all_campaigns::ResponseData {
    self.gql(
      &gql::AllCampaigns::build_query(gql::all_campaigns::Variables{
        filter: Some(gql::all_campaigns::CampaignFilter {
          available_to_account_id: Some(self.account().await.attrs.id),
          ids: None,
          id_eq: None,
          account_id_eq: None,
          finished_eq: None,
          content_id_like: None,
        }),
        page: None,
        per_page: None,
        sort_field: None,
        sort_order: None,
      }),
      vec![]
    ).await
  }
}

macro_rules! make_graphql_queries {
  ($($type:ident,)*) => {
    $(
      #[derive(graphql_client::GraphQLQuery)]
      #[graphql(
          schema_path = "public_api_schema.graphql",
          query_path = "public_api_queries.graphql",
          response_derives = "Debug,Serialize,Deserialize,PartialEq",
          normalization = "Normalization::Rust",
      )]
      pub struct $type;
    )*
  };
}

pub mod gql {
  type DateTime = chrono::DateTime<chrono::Utc>;

  make_graphql_queries![
    CreateSession,
    CreateClaimAccountRequest,
    CreateCampaignRequest,
    CampaignRequest,
    Campaign,
    AllCampaigns,
    AllCampaignsMeta,
    CampaignPreference,
    AllCampaignPreferences,
    AllCampaignPreferencesMeta,
    CreateCampaignPreference,
    CreateHandleRequest,
    AllHandleRequests,
    AllHandleRequestsMeta,
    AllCollabs,
    AllCollabsMeta,
  ];
}
