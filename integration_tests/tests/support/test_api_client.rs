use super::TestApp;
pub use galvanic_assert::{
  self,
  matchers::{collection::*, variant::*, *},
  *,
};

pub use api::models::hasher;
use rocket::{
  http::{Header, Status},
  local::asynchronous::{Client, LocalResponse},
};
use jwt_simple::{ algorithms::*, prelude::*, };
pub use serde::{Serialize, de::DeserializeOwned, Deserialize};
use graphql_client;

#[derive(Deserialize)]
pub struct ApiError {
  pub error: String,
}

pub struct ApiClient {
  pub client: Client,
  pub test_app: TestApp,
}

impl ApiClient {
  pub async fn new(test_app: TestApp) -> Self {
    Self { client: Client::tracked(api::server(test_app.app.clone())).await.unwrap(), test_app }
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
    let jwt = self.test_app.private_key().sign(claims).unwrap();
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

  pub async fn gql<'a, T: core::fmt::Debug, Q>(&'a self, query: Q, extra_headers: Vec<Header<'static>>) -> T
    where Q: Serialize, T: DeserializeOwned
  {
    let query_str = serde_json::to_string(&query).expect("gql query was not JSON");
    let response = self.post::<graphql_client::Response<T>, _>("/graphql/", query_str, extra_headers).await;
    response.data.expect(&format!("No gql response. Error was {:?}", response.errors))
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
    let jwt = self.test_app.private_key().sign(claims).unwrap();

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
