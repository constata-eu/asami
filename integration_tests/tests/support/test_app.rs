use super::*;
use api::{
  on_chain,
  models::{self, U256, u},
  App,
  AppConfig
};
use jwt_simple::algorithms::*;
use std::process::Command;
use crate::support::{Truffle, ApiClient};
pub use ethers::{
  abi::{Address, AbiEncode, Detokenize},
  providers::{Http, Provider},
  middleware::{contract::FunctionCall, Middleware},
  types::TransactionRequest,
  signers::{LocalWallet, Signer}
};
use rand::thread_rng;
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
    let mut config = AppConfig::default_figment().expect("config to exist");

    Command::new("sqlx")
      .current_dir("../api")
      .env("DATABASE_URL", &config.database_uri)
      .args(&["database", "reset", "-y"])
      .output()
      .expect("SQLX not available.");

    let truffle = Truffle::start(&config.rsk.admin_address);
    config.rsk.contract_address = truffle.addresses.asami.clone();
    config.rsk.doc_contract_address = truffle.addresses.doc.clone();
    config.rsk.asami_contract_address = truffle.addresses.asami_core.clone();
    let provider = Provider::<Http>::try_from(&config.rsk.rpc_url).unwrap().interval(std::time::Duration::from_millis(10));
    let app = App::new("password".to_string(), config).await.unwrap();

    let fig = Config::figment().merge((Config::LOG_LEVEL, LogLevel::Off));
    let rocket_client = RocketClient::tracked( api::custom_server(app.clone(), fig) ).await.unwrap();

    provider.request::<_, bool>("miner_stop", ()).await.unwrap();

    Self{ rocket_client, provider, truffle, app }
  }

  pub async fn evm_increase_time(&self, seconds: U256) -> u64 {
    self.provider.request::<_, u64>("evm_increaseTime", vec![seconds.encode_hex()]).await.unwrap()
  }

  pub async fn evm_forward_to_next_cycle(&self) {
    self.evm_increase_time(wei("60") * wei("60") * wei("24") * wei("15")).await;
    self.evm_mine().await;
  }

  pub async fn evm_mine(&self) {
    self.provider.request::<_, U256>("evm_mine", None::<()>).await.unwrap();
  }

  pub async fn stop_mining(&self) {
    self.provider.request::<_, bool>("miner_stop", ()).await.unwrap();
  }

  pub async fn start_mining(&self) {
    self.provider.request::<_, bool>("miner_start", ()).await.unwrap();
  }

  pub async fn admin_nonce(&self) -> U256 {
    self.provider.get_transaction_count(self.client_admin_address(), None).await.unwrap()
  }

  pub async fn client(&self) -> ApiClient {
    let mut client = ApiClient::new(self).await;
    client.login().await;
    client
  }

  pub fn make_random_local_wallet(&self) -> LocalWallet {
    LocalWallet::new(&mut thread_rng()).with_chain_id(self.app.settings.rsk.chain_id)
  }

  pub async fn make_wallet(&self) -> LocalWallet {
    let wallet = self.make_random_local_wallet();
    let tx = TransactionRequest::new().to(wallet.address()).value(u("10"));
    self.start_mining().await;
    self.app.on_chain.asami.client()
      .send_transaction(tx, None)
      .await.expect("sending tx")
      .interval(std::time::Duration::from_millis(10))
      .await.expect("waiting tx confirmation")
      .expect("tx result");
    self.stop_mining().await;
    wallet
  }

  pub fn legacy_contract(&self) -> &api::on_chain::AsamiContractSigner {
    &self.app.on_chain.contract
  }

  pub fn asami_core(&self) -> &api::on_chain::AsamiCoreContractSigner {
    &self.app.on_chain.asami
  }

  pub fn doc_contract(&self) -> &api::on_chain::DocContract {
    &self.app.on_chain.doc_contract
  }

  pub fn client_admin_address(&self) -> Address {
    self.asami_core().client().address()
  }

  pub async fn admin_address(&self) -> Address {
    self.asami_core().admin().call().await.expect("getting admin address")
  }

  pub async fn admin_treasury_address(&self) -> Address {
    self.asami_core().admin_treasury().call().await.expect("getting admin treasury address")
  }

  pub async fn rbtc_balance_of(&self, addr: &Address) -> U256 {
    self.asami_core().client().get_balance(*addr, None).await
      .expect(&format!("getting RBTC balance of {addr}"))
  }

  pub async fn doc_balance_of(&self, addr: &Address) -> U256 {
    self.doc_contract().balance_of(*addr).await
      .expect(&format!("getting DOC balance of {addr}"))
  }

  pub async fn asami_balance_of(&self, addr: &Address) -> U256 {
    self.asami_core().balance_of(*addr).await
      .expect(&format!("getting ASAMI balance of {addr}"))
  }

  pub async fn assert_balances_of(
    &self,
    reference: &str,
    account_id: U256,
    expected_rbtc: U256,
    expected_unclaimed_doc: U256,
    expected_doc: U256,
    expected_unclaimed_asami: U256,
    expected_asami: U256
  ) {
    let (addr, _, unclaimed_asami, unclaimed_doc) = self.asami_core().accounts(account_id).call().await
      .expect(&format!("Cannot find account balances for {account_id}"));
    let (asami_balance, rbtc_balance, doc_balance) = if addr == Address::zero() {
      (u("0"), u("0"), u("0"))
    } else {
      (
        self.asami_balance_of(&addr).await,
        self.rbtc_balance_of(&addr).await,
        self.doc_balance_of(&addr).await,
      )
    };

    assert_eq!(rbtc_balance, expected_rbtc, "rbtc balance mismatch on '{reference}'");
    assert_eq!(unclaimed_doc, expected_unclaimed_doc, "unclaimed doc mismatch on '{reference}'");
    assert_eq!(doc_balance, expected_doc, "doc balance mismatch on '{reference}'");
    assert_eq!(asami_balance, expected_asami, "asami balance mismatch on '{reference}'");
    assert_eq!(unclaimed_asami, expected_unclaimed_asami, "unclaimed asami mismatch on '{reference}'");
  }

  pub async fn admin_rbtc_balance(&self) -> U256 {
    self.rbtc_balance_of(&self.client_admin_address()).await
  }

  pub async fn admin_unclaimed_doc_balance(&self) -> U256 {
    self.asami_core().admin_unclaimed_doc_balance().call().await.unwrap()
  }

  pub async fn admin_doc_balance(&self) -> U256 {
    self.doc_balance_of(&self.client_admin_address()).await
  }

  pub async fn admin_unclaimed_asami_balance(&self) -> U256 {
    self.asami_core().admin_unclaimed_asami_balance().call().await.unwrap()
  }

  pub async fn admin_asami_balance(&self) -> U256 {
    self.asami_balance_of(&self.client_admin_address()).await
  }

  pub async fn admin_treasury_rbtc_balance(&self) -> U256 {
    self.rbtc_balance_of(&self.admin_treasury_address().await).await
  }

  pub async fn admin_treasury_doc_balance(&self) -> U256 {
    self.doc_balance_of(&self.admin_treasury_address().await).await
  }

  pub async fn admin_treasury_asami_balance(&self) -> U256 {
    self.asami_balance_of(&self.admin_treasury_address().await).await
  }

  pub async fn assert_admin_balances(
    &self,
    reference: &str,
    expected_unclaimed_doc: U256,
    expected_doc: U256,
    expected_treasury_doc: U256,
    expected_unclaimed_asami: U256,
    expected_asami: U256,
    expected_treasury_asami: U256,
  ) {
    assert_eq!(self.admin_unclaimed_doc_balance().await, expected_unclaimed_doc, "unclaimed doc mismatch on '{reference}'");
    assert_eq!(self.admin_doc_balance().await, expected_doc, "doc balance mismatch on '{reference}'");
    assert_eq!(self.admin_treasury_doc_balance().await, expected_treasury_doc, "doc treasury balance mismatch on '{reference}'");

    assert_eq!(self.admin_unclaimed_asami_balance().await, expected_unclaimed_asami, "unclaimed asami mismatch on '{reference}'");
    assert_eq!(self.admin_asami_balance().await, expected_asami, "asami balance mismatch on '{reference}'");
    assert_eq!(self.admin_treasury_asami_balance().await, expected_treasury_asami, "asami treasury balance mismatch on '{reference}'");
  }

  pub async fn contract_doc_balance(&self) -> U256 {
    self.doc_balance_of(&self.asami_core().address()).await
  }

  pub async fn send_doc_to(&self, addr: Address, amount: U256) {
    self.start_mining().await;
    self.doc_contract().transfer(addr, amount).send()
      .await.unwrap()
      .interval(std::time::Duration::from_millis(10))
      .await.unwrap().unwrap();
    self.stop_mining().await;
  }

  pub async fn send_asami_to(&self, addr: Address, amount: U256) {
    self.start_mining().await;
    self.asami_core().transfer(addr, amount).send()
      .await.unwrap()
      .interval(std::time::Duration::from_millis(10))
      .await.unwrap().unwrap();
    self.stop_mining().await;
  }

  pub async fn send_make_collab_tx(&self, reference: &str, max_gas: &str, advertiser: &ApiClient<'_>, briefing: U256, member: &ApiClient<'_>, reward: U256){
    self.send_tx(
      reference,
      max_gas,
      self.asami_core().admin_make_collabs(
        vec![on_chain::MakeCollabsParam{
          advertiser_id: advertiser.account_id(),
          briefing,
          collabs: vec![ on_chain::MakeCollabsParamItem{ account_id: member.account_id(), doc_reward: reward}]
        }]
      )
    ).await;
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
      r.verify("179383862".into()).await.unwrap().appraise(models::u("1"), models::wei("10000")).await.unwrap();
    }
  }

  pub async fn mock_collab_on_all_campaigns_with_all_handles(&self) {
    for site in &[models::Site::X, models::Site::Instagram] {
      for c in self.app.campaign().select().site_eq(site).all().await.unwrap() {
        for h in self.app.handle().select().site_eq(site).all().await.unwrap() {
          c.make_collab(&h).await.unwrap();
        }
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

  pub async fn send_tx<B, M, D>(&self, reference: &str, max_gas: &str, fn_call: FunctionCall<B, M, D>) -> models::OnChainTx
    where
      B: std::borrow::Borrow<M>,
      M: Middleware,
      D: Detokenize,
  {
    let mut tx = self.app.on_chain_tx().send_tx(fn_call).await.expect(&format!("db error {reference}"));
    self.wait_tx_state(reference, &tx, models::OnChainTxStatus::Success).await;
    tx.reload().await.expect(&format!("Could not reload tx '{reference}'"));
    let gas = tx.gas_used().as_ref().map(u256).expect(&format!("No gas used in {reference}"));
    assert!( gas <= wei(max_gas), "Sending tx with reference '{reference}' max gas was {max_gas} but used {gas}");
    tx
  }

  pub async fn send_revert_tx<B, M, D>(&self, reference: &str, expected_message: &str, fn_call: FunctionCall<B, M, D>) -> models::OnChainTx
    where
      B: std::borrow::Borrow<M>,
      M: Middleware,
      D: Detokenize,
  {
    let tx = self.app.on_chain_tx().send_tx(fn_call).await.expect(&format!("db error '{reference}'"));
    self.wait_tx_state(reference, &tx, models::OnChainTxStatus::Reverted).await;
    assert_eq!(
      tx.reloaded().await.expect("db error on '{reference}'").message().as_deref(),
      Some(expected_message),
      "wrong error message for ➲ '{reference}'"
    );
    tx
  }

  pub async fn send_without_mining<B, M, D>(&self, reference: &str, nonce: U256, fn_calls: Vec<FunctionCall<B, M, D>>) -> Vec<models::OnChainTx>
    where
      B: std::borrow::Borrow<M>,
      M: Middleware,
      D: Detokenize,
  {
    let mut txs = vec![];
    for (i, fn_call) in fn_calls.into_iter().enumerate() {
      txs.push(
        self
          .app
          .on_chain_tx()
          .send_tx(fn_call.nonce(nonce + U256::from(i)))
          .await
          .expect(&format!("db error position {i} {reference}"))
      );
    }
    txs
  }

  pub async fn wait_tx_state(&self, reference: &str, tx: &models::OnChainTx, status: models::OnChainTxStatus) {
    let success = wait_for(100, 10, || async {
      self.evm_mine().await;
      self.app.on_chain_tx().sync_tx_result().await.expect(&format!("on chain sync tx result failed: {reference}"));
      tx.reloaded().await.unwrap().attrs.status == status
    }).await;
    let reloaded = tx.reloaded().await.unwrap();
    assert!(
      success,
      "Waiting for tx status on '{}' to be '{:?}' but was '{:?}' ({:?})",
      reference,
      status,
      reloaded.status(),
      reloaded.message()
    ); 
  }

  pub async fn wait_tx_failure(&self, reference: &str, tx: &models::OnChainTx, expected_message: &str) {
    self.wait_tx_state(&format!("waiting for failure on {reference}"), tx, models::OnChainTxStatus::Failure).await;
    assert_eq!(
      tx.reloaded().await.expect(&format!("db error on {reference}")).message().as_deref(),
      Some(expected_message),
      "wrong error message for {reference}"
    );
  }
}
