use super::TestApp;
pub use galvanic_assert::{
  self,
  matchers::{collection::*, variant::*, *},
  *,
};
pub use api::models::{self, hasher};
use rocket::{
  http::{Header, Status},
  local::asynchronous::{Client, LocalResponse},
};
use jwt_simple::{ algorithms::*, prelude::*, };
pub use serde::{Serialize, de::DeserializeOwned, Deserialize};
use graphql_client;
pub use api::Decimal;
use jwt_simple::algorithms::*;

#[derive(Deserialize)]
pub struct ApiError {
  pub error: String,
}

#[allow(dead_code)]
pub struct ApiClient {
  pub client: Client,
  pub test_app: TestApp,
  pub session_key: Option<ES256KeyPair>,
  pub scenario: Box<Scenario>
}

#[derive(Clone, Default)]
pub struct Scenario {
  pub campaigns: Option<CampaignsFixture>,
}

#[derive(Clone, Default)]
pub struct CampaignsFixture {
  account: Account,
  regular_campaign: Campaign,
  high_rate_campaign: Campaign,
  low_rate_campaign: Campaign,
  low_budget_campaign: Campaign,
}

pub struct 

impl ApiClient {
  pub async fn new(test_app: TestApp) -> Self {
    Self { session_key: None, client: Client::tracked(api::server(test_app.app.clone())).await.unwrap(), test_app }
  }

  pub async fn login(&mut self) -> Account {
    self.login_with_key(self.test_app.private_key()).await
  }

  pub async fn login_with_key(&mut self, key: &ES256KeyPair) -> Account {
    self.session_key = key;

    let token = key.key_id().clone().unwrap();
    self.test_app.app.one_time_token()
      .insert(InsertOneTimeToken{
      value: token.to_string()
    }).save().await.unwrap();

    let login_pubkey = URL_SAFE_NO_PAD.encode(
      self.session_key.public_key().to_pem().unwrap()
    );

    let result = create_session::ResponseData = client.gql(
      &CreateSession::build_query(create_session::Variables{}),
      vec![
        Header::new("Auth-Action", "Login"),
        Header::new("Auth-Method-Kind", "OneTimeToken"),
        Header::new("Auth-Data", token),
        Header::new("Login-Pubkey", login_pubkey),
        Header::new("New-Session-Recaptcha-Code", "SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c"),
      ]
    ).await;

    let account = self.test_app.app.account().find(&result.account_ids[0]).await.unwrap();

    (self, account)
  }

  pub async fn build_baseline_scenario(&mut self) -> Scenario {
    // This is a full scenario to use in testing any integration.
    // It replicates the application having run for a few cicles 
    // All tests should at least be run in this scenario.
    // ToDo: Can we just cache and load instead of re-running every time?

    let campaigns = self.build_campaigns().await;
    Scenario { campaigns }
  }

  pub async fn build_campaigns(&mut self) -> CampaignsFixture {
    let account = self.login_with_key(&ES256KeyPair::generate()).await;
    let rate = u256(self.test_app.app.indexer_state().get().await?.suggested_price_per_point());
    let budget = rate * u("200");
    let post = "1716421161867710954";
    let two_days = Utc::now() + chrono::Duration::days(2);

    let campaign = |budget, rate| async {
      account.create_campaign_request(Site::X, post, budget, rate, two_days)
        .await.unwrap().pay()
        .await.unwrap()
    };
    
    let regular_campaign = campaign(budget, rate).await;
    let high_rate_campaign = campaign(budget, rate * u("2")).await;
    let low_rate_campaign = campaign(budget, rate / u("2")).await;
    let low_budget_campaign = campaign(rate * u("1"), rate).await;

    self.test_app.run_idempotent_background_tasks_a_few_times().await;

    CampaignFixture {
      account: account.reload().await?,
      regular_campaign: regular_campaign.campaign().await.unwrap(),
      high_rate_campaign: high_rate_campaign.campaign().await.unwrap(),
      low_rate_campaign: low_rate_campaign.campaign().await.unwrap(),
      low_budget_campaign: low_budget_campaign.campaign().await.unwrap(),
    }
  }

  pub async fn build_x_handle(&mut self, account: &Account, username: &str) -> models::Handle {
    let rate = u256(self.test_app.app.indexer_state().get().await?.suggested_price_per_point());

    let req = account.create_handle_request(Site::X, username).await.unwrap()
      .verify("179383862".into()).await.unwrap().
      .appraise(rate * u("10"), u("10")).await.unwrap();

    self.test_app.run_idempotent_background_tasks_a_few_times().await;

    req.handle().await.unwrap()
  }

  pub async fn gql<'a, T: core::fmt::Debug, Q>(&'a self, query: Q, extra_headers: Vec<Header<'static>>) -> T
    where Q: Serialize, T: DeserializeOwned
  {
    let query_str = serde_json::to_string(&query).expect("gql query was not JSON");
    let response = self.post::<graphql_client::Response<T>, _>("/graphql/", query_str, extra_headers).await;
    response.data.expect(&format!("No gql response. Error was {:?}", response.errors))
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
    let jwt = self.session_key.unwrap().sign(claims).unwrap();
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
    let mut req = self.client.post(path).body(body).header(header);
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

  pub async fn get<T: DeserializeOwned>(&self, path: &str) -> T {
    let response = self.get_string(path).await;
    serde_json::from_str(&response).expect(&format!("Could not parse response {}", response))
  }

  pub async fn get_magic_link(&self, path: &str) -> String {
    self.client
      .get(path)
      .dispatch().await
      .into_string().await.unwrap()
  }

  pub async fn get_string(&self, path: &str) -> String {
    self.get_response(path).await.into_string().await.unwrap()
  }

  pub async fn get_bytes(&self, path: &str) -> Vec<u8> {
    self.get_response(path).await.into_bytes().await.unwrap()
  }

  pub async fn get_response<'a>(&'a self, path: &'a str) -> LocalResponse<'a> {
    self.get_response_with_auth(path, self.ok_auth_header(path, "POST", None, None)).await
  }

  pub async fn get_response_with_auth<'a>(&'a self, path: &'a str, auth_header: Header<'static>) -> LocalResponse<'a> {
    self.client
      .post(path)
      .header(auth_header)
      .dispatch().await
  }

  pub async fn get_response_with_wrong_auth_path<'a>(&'a self, path: &'a str) -> LocalResponse<'a> {
    self.get_response_with_auth(path, self.ok_auth_header("/boguspath/", "POST", None, None)).await
  }
  
  pub async fn get_response_with_old_auth_token<'a>(&'a self, path: &'a str) -> LocalResponse<'a> {
    self.get_response_with_auth(path, self.make_auth_header(path, "POST", -1, None, None)).await
  }

  pub async fn response_with_bad_auth_signature<'a>(&'a self, path: &'a str) -> LocalResponse<'a> {
    let payload = serde_json::json![{
      "path": path,
      "method": "POST",
      "nonce": 1,
      "body_hash": null,
      "query_hash": null,
    }];

    let claims = Claims::with_custom_claims(payload, Duration::from_secs(30));
    let key = ES256KeyPair::generate();
    let id = self.test_app.app.session().select().one().await.unwrap().attrs.id.clone();
    let jwt = key.with_key_id(&id).sign(claims).unwrap();
    self.get_response_with_auth(path, Header::new("Authentication", jwt)).await
  }

  pub async fn get_response_with_malformed_payload<'a>(&'a self, path: &'a str) -> LocalResponse<'a> {
    let payload = serde_json::json![{
      "no_path": path,
      "and_nothing_else": "totally_invalid",
    }];

    let claims = Claims::with_custom_claims(payload, Duration::from_secs(30));
    let jwt = self.session_key.unwrap().sign(claims).unwrap();

    self.get_response_with_auth(path, Header::new("Authentication", jwt)).await
  }

  pub async fn assert_unauthorized_get<P: std::fmt::Display>(&self, path: P) {
    let response = self.client.get(path.to_string()).dispatch().await;
    assert_eq!(response.status(), Status::Unauthorized);
  }

  pub async fn assert_get_error<'a>(&'a self, path: &'a str, status: Status, msg: &'a str) {
    let response = self.get_response(path).await;
    assert_eq!(response.status(), status);
    let err: ApiError = serde_json::from_str(&response.into_string().await.unwrap()).unwrap();
    assert_that!(&err.error.contains(msg));
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
    CreateCampaignRequest,
    CampaignRequest,
    Campaign,
    AllCampaigns,
    AllCampaignsMeta,
  ];
}
