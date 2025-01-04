use super::TestApp;
pub use api::{
    lang,
    models::{self, hasher, milli, u, u256, wei, Utc, U256},
    Decimal,
};
use api::{
    models::{on_chain_job::AsamiFunctionCall, AbiDecode},
    on_chain::{self, Address, AsamiContract, DocContract, Http, LegacyContract, Provider, SignerMiddleware, IERC20},
};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use ethers::signers::{LocalWallet, Signer};
pub use galvanic_assert::{
    self,
    matchers::{collection::*, variant::*, *},
    *,
};
pub use graphql_client::{self, GraphQLQuery};
use jwt_simple::{algorithms::*, prelude::*};
use rocket::{http::Header, local::asynchronous::LocalResponse};
pub use serde::{de::DeserializeOwned, Deserialize, Serialize};

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
    pub account_id: Option<U256>,
    pub legacy_contract: Option<LegacyContract>,
    pub asami_contract: Option<AsamiContract>,
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
            account_id: None,
            legacy_contract: None,
            doc_contract: None,
            asami_contract: None,
        }
    }

    pub fn app(&self) -> api::App {
        self.test_app.app.clone()
    }

    pub fn account_id(&self) -> U256 {
        self.account_id.clone().unwrap()
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

    pub fn local_wallet(&self) -> &LocalWallet {
        self.local_wallet.as_ref().unwrap()
    }

    pub fn address(&self) -> Address {
        self.local_wallet().address()
    }

    pub fn legacy_contract(&self) -> &LegacyContract {
        self.legacy_contract.as_ref().unwrap()
    }

    pub fn asami_contract(&self) -> &AsamiContract {
        self.asami_contract.as_ref().unwrap()
    }

    pub fn doc_contract(&self) -> &DocContract {
        self.doc_contract.as_ref().unwrap()
    }

    pub async fn asami_balance(&self) -> U256 {
        self.test_app.asami_balance_of(&self.address()).await
    }

    pub async fn doc_balance(&self) -> U256 {
        self.test_app.doc_balance_of(&self.address()).await
    }

    pub async fn rbtc_balance(&self) -> U256 {
        self.test_app.rbtc_balance_of(&self.address()).await
    }

    pub async fn login(&mut self) {
        self.login_with_key(ES256KeyPair::generate()).await
    }

    pub async fn login_with_key(&mut self, key: ES256KeyPair) {
        let token = api::models::hasher::hexdigest(key.public_key().to_pem().unwrap().as_bytes());
        self.session_key = Some(key.with_key_id(&token));

        self.test_app
            .app
            .one_time_token()
            .insert(models::InsertOneTimeToken {
                value: token.to_string(),
                email: None,
                lang: lang::Lang::Es,
                lookup_key: format!("one_time_token:{}", &token),
                user_id: None,
            })
            .save()
            .await
            .unwrap();

        let login_pubkey = URL_SAFE_NO_PAD.encode(self.session_key.as_ref().unwrap().public_key().to_pem().unwrap());

        let result: gql::create_session::ResponseData = self
            .gql(
                &gql::CreateSession::build_query(gql::create_session::Variables {}),
                vec![
                    Header::new("Auth-Action", "Login"),
                    Header::new("Auth-Method-Kind", "OneTimeToken"),
                    Header::new("Auth-Data", token),
                    Header::new("Login-Pubkey", login_pubkey),
                    Header::new(
                        "New-Session-Recaptcha-Code",
                        "SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c",
                    ),
                ],
            )
            .await;

        let session = self.test_app.app.session().find(&result.create_session.id).await.unwrap();
        self.account_id = Some(u256(session.account_id()));
        self.session = Some(session);
    }

    pub async fn create_handle(
        &self,
        username: &str,
        user_id: &str,
        site: models::Site,
        score: U256,
    ) -> models::Handle {
        use ethers::abi::AbiEncode;
        use gql::create_handle::*;
        let gql_site = match site {
            models::Site::X => Site::X,
            models::Site::Instagram => Site::INSTAGRAM,
            _ => panic!("Site not supported in tests yet {site:?}"),
        };

        let response: graphql_client::Response<ResponseData> = self
            .gql_response(
                &gql::CreateHandle::build_query(Variables {
                    input: CreateHandleInput {
                        username: username.to_string(),
                        site: gql_site,
                    },
                }),
                vec![],
            )
            .await;

        let gql_handle = response.data.unwrap().create_handle;

        // This code is stubbing the actual IG or X code that does
        // verification and scoring.
        self.app()
            .handle()
            .find(i32::try_from(gql_handle.id).unwrap())
            .await
            .unwrap()
            .verify(user_id.into())
            .await
            .unwrap()
            .update()
            .score(Some(score.encode_hex()))
            .save()
            .await
            .unwrap()
    }

    pub async fn gql_claim_account_request(
        &self,
        wallet: &LocalWallet,
    ) -> graphql_client::Response<gql::create_claim_account_request::ResponseData> {
        let rsk = &self.test_app.app.settings.rsk;
        let payload = models::make_login_to_asami_typed_data(rsk.chain_id).unwrap();
        let signature = wallet.sign_typed_data(&payload).await.unwrap().to_string();

        self.gql(
            &gql::CreateClaimAccountRequest::build_query(gql::create_claim_account_request::Variables {
                input: gql::create_claim_account_request::CreateClaimAccountRequestInput { signature },
            }),
            vec![],
        )
        .await
    }

    pub async fn setup_trusted_admin(&mut self, message: &str) {
        self.test_app
            .send_tx(
                &format!("Configuring trusted admin: {message}"),
                "71200",
                self.asami_contract().config_account(self.test_app.client_admin_address(), u("5"), u("0"), u("0")),
            )
            .await;
    }

    pub async fn setup_as_advertiser(&mut self, message: &str) {
        self.setup_as_advertiser_with_amount(message, u("2000")).await;
    }

    pub async fn setup_as_advertiser_with_amount(&mut self, message: &str, amount: U256) {
        // Submitting teh claim account request is enough to set an address for the account
        // Even if teh promote account is not called.
        // As a side effect, this will create the advertiser's local wallet.
        self.submit_claim_account_request().await;

        self.test_app.send_doc_to(self.address(), amount.clone()).await;

        self.setup_trusted_admin(message).await;

        self.test_app
            .send_tx(
                &format!("Approving spending for setting up as advertiser: {message}"),
                "46296",
                self.doc_contract().approve(self.test_app.asami_contract().address(), amount.clone()),
            )
            .await;
    }

    /* Gets a campaign briefing hash, and pays for it */
    pub async fn start_and_pay_campaign(
        &self,
        link: &str,
        budget: U256,
        duration: i64,
        topic_ids: &[i32],
    ) -> models::Campaign {
        use gql::create_campaign_from_link::*;
        let response: graphql_client::Response<ResponseData> = self
            .gql_response(
                &gql::CreateCampaignFromLink::build_query(Variables {
                    input: CreateCampaignFromLinkInput {
                        link: link.to_string(),
                        topic_ids: topic_ids.iter().map(|x| *x as i64).collect(),
                    },
                }),
                vec![],
            )
            .await;

        let gql_campaign = response.data.unwrap().create_campaign_from_link;
        let briefing_hash = U256::decode_hex(&gql_campaign.briefing_hash).unwrap();

        self.pay_campaign(&format!("making campaign {link}"), budget, briefing_hash, duration).await;

        let campaign = self.app().campaign().find(i32::try_from(gql_campaign.id).unwrap()).await.unwrap();

        self.test_app
            .sync_events_until(&format!("Campaign should be paid '{link}'"), || async {
                campaign.reloaded().await.unwrap().budget_u256() > u("0")
            })
            .await;

        campaign.reloaded().await.unwrap()
    }

    pub async fn pay_campaign(&self, message: &str, budget: U256, briefing_hash: U256, duration_days: i64) {
        self.test_app
            .send_tx(
                &format!("Making campaign: {message}"),
                "118000",
                self.pay_campaign_contract_call(budget, briefing_hash, duration_days),
            )
            .await;
    }

    pub fn pay_campaign_contract_call(
        &self,
        budget: U256,
        briefing_hash: U256,
        duration_days: i64,
    ) -> AsamiFunctionCall {
        self.asami_contract().make_campaigns(vec![on_chain::MakeCampaignsParam {
            budget,
            briefing_hash,
            valid_until: models::utc_to_i(Utc::now() + chrono::Duration::days(duration_days)),
        }])
    }

    pub fn top_up_campaign_contract_call(
        &self,
        account: Address,
        briefing_hash: U256,
        budget: U256,
    ) -> AsamiFunctionCall {
        self.asami_contract().top_up_campaigns(vec![on_chain::TopUpCampaignsParam {
            account,
            briefing_hash,
            budget,
        }])
    }

    pub fn extend_campaign_contract_call(&self, briefing_hash: U256, valid_until: i64) -> AsamiFunctionCall {
        self.asami_contract().extend_campaigns(vec![on_chain::ExtendCampaignsParam {
            valid_until: self.test_app.future_date(valid_until),
            briefing_hash,
        }])
    }

    pub fn reimburse_campaign_contract_call(&self, addr: Address, briefing_hash: U256) -> AsamiFunctionCall {
        self.asami_contract()
            .reimburse_campaigns(vec![on_chain::ReimburseCampaignsParam { addr, briefing_hash }])
    }

    pub async fn make_client_wallet(&mut self) {
        let rsk = &self.app().settings.rsk;
        let wallet = self.test_app.make_wallet().await;

        let provider = Provider::<Http>::try_from(&rsk.rpc_url).unwrap().interval(std::time::Duration::from_millis(10));
        let client = std::sync::Arc::new(SignerMiddleware::new(provider, wallet.clone()));
        let legacy_address: Address = rsk.legacy_contract_address.parse().unwrap();
        let asami_address: Address = rsk.asami_contract_address.parse().unwrap();
        self.legacy_contract = Some(LegacyContract::new(legacy_address, client.clone()));
        self.asami_contract = Some(AsamiContract::new(asami_address, client.clone()));

        let doc_address: Address = rsk.doc_contract_address.parse().unwrap();
        self.doc_contract = Some(IERC20::new(doc_address, client.clone()));
        self.local_wallet = Some(wallet);
    }

    pub async fn submit_claim_account_request(&mut self) {
        self.make_client_wallet().await;
        self.gql_claim_account_request(&self.local_wallet()).await;
    }

    pub async fn claim_account(&mut self) {
        self.submit_claim_account_request().await;

        self.account().await.update().allows_gasless(true).save().await.unwrap();

        self.test_app.wait_for_job(
            &format!("Claming account {:?}", self.account_id()),
            models::OnChainJobKind::PromoteSubAccounts,
            models::OnChainJobStatus::Settled
        ).await;
    }

    pub async fn gql<'a, T: core::fmt::Debug, Q>(&'a self, query: Q, extra_headers: Vec<Header<'static>>) -> T
    where
        Q: Serialize,
        T: DeserializeOwned,
    {
        let response = self.gql_response(query, extra_headers).await;
        response.data.expect(&format!("No gql response. Error was {:?}", response.errors))
    }

    pub async fn gql_response<'a, T: core::fmt::Debug, Q>(
        &'a self,
        query: Q,
        extra_headers: Vec<Header<'static>>,
    ) -> graphql_client::Response<T>
    where
        Q: Serialize,
        T: DeserializeOwned,
    {
        let query_str = serde_json::to_string(&query).expect("gql query was not JSON");
        self.post::<graphql_client::Response<T>, _>("/graphql/", query_str, extra_headers).await
    }

    pub fn make_auth_header<'a>(
        &'a self,
        path: &str,
        method: &str,
        nonce: i64,
        body: Option<&str>,
        query: Option<&str>,
    ) -> Header<'static> {
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
        self.make_auth_header(path, method, chrono::Utc::now().timestamp_millis(), body, query)
    }

    pub async fn post_response<'a, B>(
        &'a self,
        path: &'a str,
        body: B,
        extra_headers: Vec<Header<'static>>,
    ) -> LocalResponse<'a>
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
        self.post_response(path, body, extra_headers).await.into_string().await.unwrap()
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
        self.test_app
            .rocket_client
            .post(path)
            .header(self.ok_auth_header(path, "POST", None, None))
            .dispatch()
            .await
    }

    pub async fn get_campaign_offers(&self) -> gql::all_campaigns::ResponseData {
      self.gql(
        &gql::AllCampaigns::build_query(gql::all_campaigns::Variables{
          filter: Some(gql::all_campaigns::CampaignFilter {
            available_to_account_id: Some(api::models::hex_to_i32(&self.account().await.attrs.id).unwrap().into()),
            ids: None,
            id_eq: None,
            account_id_eq: None,
            briefing_hash_eq: None,
            briefing_json_like: None,
            budget_eq: None,
            budget_gt: None,
            budget_lt: None,
            status_ne: None,
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
        CreateCampaignFromLink,
        CreateEmailLoginLink,
        Campaign,
        AllCampaigns,
        AllCampaignsMeta,
        CampaignPreference,
        AllCampaignPreferences,
        AllCampaignPreferencesMeta,
        CreateCampaignPreference,
        CreateHandle,
        AllHandles,
        AllHandlesMeta,
        AllCollabs,
        AllCollabsMeta,
    ];
}
