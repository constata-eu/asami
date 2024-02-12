use api::{models::{self, U256}, App, AppConfig};
use jwt_simple::algorithms::*;
use std::process::Command;
use crate::support::{Truffle, ApiClient};
use ethers::{
  abi::AbiEncode,
  providers::{Http, Provider}
};
use rocket::local::asynchronous::Client as RocketClient;
use rocket::{Config, config::LogLevel};

pub struct TestApp {
  pub app: App,
  pub truffle: Truffle,
  pub provider: Provider<Http>,
  pub rocket_client: RocketClient,
}

impl TestApp {
  pub async fn init() -> Self {
    let mut config = AppConfig::default().expect("config to exist");

    Command::new("sqlx")
      .current_dir("../api")
      .env("DATABASE_URL", &config.database_uri)
      .args(&["database", "reset", "-y"])
      .output()
      .expect("SQLX not available.");

    let truffle = Truffle::start(&config.rsk.admin_address);
    config.rsk.contract_address = truffle.addresses.asami.clone();
    config.rsk.doc_contract_address = truffle.addresses.doc.clone();
    let provider = Provider::<Http>::try_from(&config.rsk.rpc_url).unwrap();
    let app = App::new("password".to_string(), config).await.unwrap();

    let fig = Config::figment().merge((Config::LOG_LEVEL, LogLevel::Off));
    let rocket_client = RocketClient::tracked( api::custom_server(app.clone(), fig) ).await.unwrap();

    Self{ rocket_client, provider, truffle, app }
  }

  pub async fn evm_increase_time(&self, seconds: U256) -> u64 {
    self.provider.request::<_, u64>("evm_increaseTime", vec![seconds.encode_hex()]).await.unwrap()
  }

  pub async fn evm_forward_to_next_cycle(&self) {
    use models::wei;
    self.evm_increase_time(wei("60") * wei("60") * wei("24") * wei("15")).await;
    self.evm_mine().await;
  }

  pub async fn evm_mine(&self) {
    self.provider.request::<_, U256>("evm_mine", None::<()>).await.unwrap();
  }

  pub async fn client(&self) -> ApiClient {
    let mut client = ApiClient::new(self.clone()).await;
    client.login().await;
    client
  }

  pub fn contract(&self) -> &api::on_chain::AsamiContractSigner {
    &self.app.on_chain.contract
  }

  pub fn doc_contract(&self) -> &api::on_chain::DocContract {
    &self.app.on_chain.doc_contract
  }

  pub async fn admin_doc_balance(&self) -> U256 {
    self.doc_contract().balance_of(self.doc_contract().client().address()).call().await.unwrap()
  }

  pub async fn contract_doc_balance(&self) -> U256 {
    self.doc_contract().balance_of(self.contract().address()).call().await.unwrap()
  }

  pub async fn mock_admin_setting_campaign_requests_as_paid(&self) {
    let all = self.app.campaign_request().select().status_eq(models::CampaignRequestStatus::Received).all().await.unwrap();
    for r in all {
      r.pay().await.unwrap();
    }
  }

  pub async fn mock_all_handles_being_verified_and_appraised(&self) {
    let all = self.app.handle_request().select().status_eq(models::HandleRequestStatus::Unverified).all().await.unwrap();
    for r in all.into_iter() {
      r.verify("179383862".into()).await.unwrap().appraise(models::u("1"), models::wei("10")).await.unwrap();
    }
  }

  pub async fn mock_collab_on_all_campaigns_with_all_handles(&self) {
    for c in self.app.campaign().select().all().await.unwrap() {
      for h in self.app.handle().select().all().await.unwrap() {
        c.make_collab(&h).await.unwrap();
      }
    }
  }

  pub async fn run_idempotent_background_tasks_a_few_times(&self) {
    for _ in 0..5 {
      self.app.run_background_tasks().await.expect("error in test background tasks");
    }
  }

  pub fn private_key(&self) -> ES256KeyPair {
    let key = ES256KeyPair::from_pem(
      "-----BEGIN PRIVATE KEY-----\nMIGHAgEAMBMGByqGSM49AgEGCCqGSM49AwEHBG0wawIBAQQg626FUHw6lA0eAlYl\nqT0TI8m/JAWj+H497JAKfoTUrkmhRANCAARJPbG33RdPLOxXXbc390w00YaFAbh8\n0Hv44ScjS0UMB6/ZjjkIs5fV1gRK1IBF1JMnxM6qWjWUBlu/z9ZjvA0b\n-----END PRIVATE KEY-----\n"
    ).unwrap();
    let id = api::models::hasher::hexdigest(&key.public_key().to_pem().unwrap().as_bytes());
    key.with_key_id(&id)
  }
}
