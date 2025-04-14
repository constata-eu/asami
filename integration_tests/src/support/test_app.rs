use api::{
    models::{self, on_chain_job::AsamiFunctionCall, u, U256},
    on_chain, App, AppConfig,
};
pub use ethers::{
    abi::{AbiEncode, Address, Detokenize},
    middleware::{contract::FunctionCall, Middleware},
    providers::{Http, Provider},
    signers::{LocalWallet, Signer},
    types::{transaction::eip2718::TypedTransaction, TransactionReceipt, TransactionRequest, H256, U64},
};
use jwt_simple::algorithms::*;
use rand::thread_rng;
use rocket::{config::LogLevel, local::asynchronous::Client as RocketClient, Config};

use super::*;
use crate::support::{ApiClient, Truffle};

pub struct TestApp {
    pub app: App,
    pub truffle: Truffle,
    pub provider: Provider<Http>,
    pub rocket_client: RocketClient,
}
use sqlx_cli::*;
use clap_builder::Parser;

impl TestApp {
    pub async fn init() -> Self {
        let mut config = AppConfig::default_figment().expect("config to exist");

        run(Opt::parse_from(vec![
            "sqlx",
            "database",
            "reset",
            "-y",
            "--database-url",
            &config.database_uri,
            "--source",
            &format!( "{}/../api/migrations", env!("CARGO_MANIFEST_DIR") ),
        ]))
        .await
        .unwrap();

        let truffle = Truffle::start();
        config.rsk.legacy_contract_address.clone_from(&truffle.addresses.legacy);
        config.rsk.doc_contract_address.clone_from(&truffle.addresses.doc);
        config.rsk.asami_contract_address.clone_from(&truffle.addresses.asami);

        let provider = Provider::<Http>::try_from(&config.rsk.rpc_url)
            .unwrap()
            .interval(std::time::Duration::from_millis(10));
        provider.request::<_, bool>("evm_setAutomine", vec![false]).await.unwrap();
        provider.request::<_, bool>("evm_setIntervalMining", vec![0]).await.unwrap();
        let app = App::new("password".to_string(), config).await.unwrap();

        let fig = Config::figment().merge((Config::LOG_LEVEL, LogLevel::Off));
        let rocket_client = RocketClient::tracked(api::custom_server(app.clone(), fig)).await.unwrap();

        Self {
            rocket_client,
            provider,
            truffle,
            app,
        }
    }

    pub async fn evm_increase_time(&self, seconds: U256) -> u64 {
        self.provider
            .request::<_, String>("evm_increaseTime", vec![seconds.encode_hex()])
            .await
            .unwrap()
            .parse()
            .unwrap()
    }

    pub async fn evm_forward_to_next_cycle(&self) {
        self.evm_increase_time(wei("60") * wei("60") * wei("24") * wei("15")).await;
        self.evm_mine().await;
    }

    pub async fn evm_mine(&self) {
        self.provider.request::<Vec<()>, U256>("evm_mine", vec![]).await.unwrap();
        self.provider.request::<_, bool>("evm_setAutomine", vec![false]).await.unwrap();
    }

    pub async fn stop_mining(&self) {
        self.provider.request::<_, bool>("evm_setIntervalMining", vec![0]).await.unwrap();
        self.provider.request::<_, bool>("evm_setAutomine", vec![false]).await.unwrap();
    }

    pub async fn start_mining(&self) {
        self.provider.request::<_, bool>("evm_setIntervalMining", vec![1]).await.unwrap();
        self.provider.request::<_, bool>("evm_setAutomine", vec![false]).await.unwrap();
    }

    pub async fn admin_nonce(&self) -> U256 {
        self.provider.get_transaction_count(self.client_admin_address(), None).await.unwrap()
    }

    pub async fn client(&self) -> ApiClient {
        let mut client = ApiClient::new(self).await;
        client.login().await;
        client
    }

    pub async fn quick_campaign(&self, budget: U256, duration: i64, topic_ids: &[i32]) -> models::Campaign {
        let mut client = self.client().await;
        client.setup_as_advertiser("test main advertiser").await;
        client
            .start_and_pay_campaign(
                "https://x.com/somebody/status/1716421161867710954",
                budget,
                duration,
                topic_ids,
            )
            .await
    }

    pub async fn quick_handle(&self, username: &str, user_id: &str, score: U256) -> models::Handle {
        let mut client = self.client().await;
        client.claim_account().await;
        client.create_handle(username, user_id, score).await
    }

    pub fn make_random_local_wallet(&self) -> LocalWallet {
        LocalWallet::new(&mut thread_rng()).with_chain_id(self.app.settings.rsk.chain_id)
    }

    pub async fn make_wallet(&self) -> LocalWallet {
        let wallet = self.make_random_local_wallet();
        let tx = TransactionRequest::new().to(wallet.address()).value(u("10"));

        self.start_mining().await;
        self.app
            .on_chain
            .asami_contract
            .client()
            .send_transaction(tx, None)
            .await
            .expect("sending tx")
            .interval(std::time::Duration::from_millis(10))
            .await
            .expect("waiting tx confirmation")
            .expect("tx result");
        self.stop_mining().await;
        wallet
    }

    pub fn legacy_contract(&self) -> &api::on_chain::LegacyContract {
        &self.app.on_chain.legacy_contract
    }

    pub fn asami_contract(&self) -> &api::on_chain::AsamiContract {
        &self.app.on_chain.asami_contract
    }

    pub fn doc_contract(&self) -> &api::on_chain::DocContract {
        &self.app.on_chain.doc_contract
    }

    pub fn client_admin_address(&self) -> Address {
        self.asami_contract().client().address()
    }

    pub async fn admin_treasury_address(&self) -> Address {
        self.asami_contract().admin_treasury().call().await.expect("getting admin treasury address")
    }

    pub async fn rbtc_balance_of(&self, addr: &Address) -> U256 {
        self.asami_contract()
            .client()
            .get_balance(*addr, None)
            .await
            .unwrap_or_else(|_| panic!("getting RBTC balance of {addr}"))
    }

    pub async fn doc_balance_of(&self, addr: &Address) -> U256 {
        self.doc_contract()
            .balance_of(*addr)
            .await
            .unwrap_or_else(|_| panic!("getting DOC balance of {addr}"))
    }

    pub async fn asami_balance_of(&self, addr: &Address) -> U256 {
        self.asami_contract()
            .balance_of(*addr)
            .await
            .unwrap_or_else(|_| panic!("getting ASAMI balance of {addr}"))
    }

    pub async fn assert_sub_account_balances_of(
        &self,
        reference: &str,
        sub_account: U256,
        expected_unclaimed_doc: U256,
        expected_unclaimed_asami: U256,
    ) {
        let sub = self
            .asami_contract()
            .get_sub_account(self.client_admin_address(), sub_account)
            .call()
            .await
            .unwrap_or_else(|_| panic!("Cannot find sub account balances for {sub_account}"));

        assert_eq!(
            sub.unclaimed_doc_balance, expected_unclaimed_doc,
            "unclaimed doc mismatch on '{reference}'"
        );
        assert_eq!(
            sub.unclaimed_asami_balance, expected_unclaimed_asami,
            "unclaimed asami mismatch on '{reference}'"
        );
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn assert_balances_of(
        &self,
        reference: &str,
        account: Address,
        expected_rbtc: U256,
        expected_unclaimed_doc: U256,
        expected_doc: U256,
        expected_unclaimed_asami: U256,
        expected_asami: U256,
    ) {
        let (_, _, _, unclaimed_asami, unclaimed_doc, _, _) = self
            .asami_contract()
            .accounts(account)
            .call()
            .await
            .unwrap_or_else(|_| panic!("Cannot find account balances for {account}"));
        assert_eq!(
            self.rbtc_balance_of(&account).await / wei("100000000000"),
            expected_rbtc / wei("100000000000"),
            "rbtc balance mismatch on '{reference}'"
        );
        assert_eq!(
            unclaimed_doc, expected_unclaimed_doc,
            "unclaimed doc mismatch on '{reference}'"
        );
        assert_eq!(
            self.doc_balance_of(&account).await,
            expected_doc,
            "doc balance mismatch on '{reference}'"
        );
        assert_eq!(
            unclaimed_asami, expected_unclaimed_asami,
            "unclaimed asami mismatch on '{reference}'"
        );
        assert_eq!(
            self.asami_balance_of(&account).await,
            expected_asami,
            "asami balance mismatch on '{reference}'"
        );
    }

    pub async fn admin_rbtc_balance(&self) -> U256 {
        self.rbtc_balance_of(&self.client_admin_address()).await
    }

    pub async fn admin_unclaimed_doc_balance(&self) -> U256 {
        self.asami_contract().accounts(self.client_admin_address()).call().await.unwrap().4
    }

    pub async fn admin_doc_balance(&self) -> U256 {
        self.doc_balance_of(&self.client_admin_address()).await
    }

    pub async fn admin_unclaimed_asami_balance(&self) -> U256 {
        self.asami_contract().accounts(self.client_admin_address()).call().await.unwrap().3
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

    #[allow(clippy::too_many_arguments)]
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
        assert_eq!(
            self.admin_unclaimed_doc_balance().await,
            expected_unclaimed_doc,
            "unclaimed doc mismatch on '{reference}'"
        );
        assert_eq!(
            self.admin_doc_balance().await,
            expected_doc,
            "doc balance mismatch on '{reference}'"
        );
        assert_eq!(
            self.admin_treasury_doc_balance().await,
            expected_treasury_doc,
            "doc treasury balance mismatch on '{reference}'"
        );

        assert_eq!(
            self.admin_unclaimed_asami_balance().await,
            expected_unclaimed_asami,
            "unclaimed asami mismatch on '{reference}'"
        );
        assert_eq!(
            self.admin_asami_balance().await,
            expected_asami,
            "asami balance mismatch on '{reference}'"
        );
        assert_eq!(
            self.admin_treasury_asami_balance().await,
            expected_treasury_asami,
            "asami treasury balance mismatch on '{reference}'"
        );
    }

    pub async fn contract_doc_balance(&self) -> U256 {
        self.doc_balance_of(&self.asami_contract().address()).await
    }

    pub async fn send_rbtc_to(&self, addr: Address, amount: U256) {
        let tx = TransactionRequest::new().to(addr).value(amount);
        self.start_mining().await;
        self.app
            .on_chain
            .asami_contract
            .client()
            .send_transaction(tx, None)
            .await
            .expect("sending tx")
            .interval(std::time::Duration::from_millis(10))
            .await
            .expect("waiting tx confirmation")
            .expect("tx result");
        self.stop_mining().await;
    }

    pub async fn send_doc_to(&self, addr: Address, amount: U256) {
        self.start_mining().await;
        self.doc_contract()
            .transfer(addr, amount)
            .send()
            .await
            .unwrap()
            .interval(std::time::Duration::from_millis(10))
            .await
            .unwrap()
            .unwrap();
        self.stop_mining().await;
    }

    pub async fn send_asami_to(&self, addr: Address, amount: U256) {
        self.start_mining().await;
        self.asami_contract()
            .transfer(addr, amount)
            .send()
            .await
            .unwrap()
            .interval(std::time::Duration::from_millis(10))
            .await
            .unwrap()
            .unwrap();
        self.stop_mining().await;
    }

    pub async fn send_make_collab_tx(
        &self,
        reference: &str,
        max_gas: &str,
        advertiser: &ApiClient<'_>,
        briefing_hash: U256,
        member: &ApiClient<'_>,
        doc_reward: U256,
    ) {
        self.send_tx(
            reference,
            max_gas,
            self.asami_contract().admin_make_collabs(vec![on_chain::MakeCollabsParam {
                advertiser_addr: advertiser.address(),
                briefing_hash,
                collabs: vec![on_chain::MakeCollabsParamItem {
                    account_addr: member.address(),
                    doc_reward,
                }],
            }]),
        )
        .await;
    }

    pub async fn send_make_sub_account_collab_tx(
        &self,
        reference: &str,
        max_gas: &str,
        advertiser: &ApiClient<'_>,
        briefing_hash: U256,
        member: &ApiClient<'_>,
        doc_reward: U256,
    ) {
        self.send_tx(
            reference,
            max_gas,
            self.asami_contract().admin_make_sub_account_collabs(vec![on_chain::MakeSubAccountCollabsParam {
                advertiser_addr: advertiser.address(),
                briefing_hash,
                collabs: vec![on_chain::MakeSubAccountCollabsParamItem {
                    sub_account_id: member.account_id(),
                    doc_reward,
                }],
            }]),
        )
        .await;
    }

    pub async fn register_collab(
        &self,
        context: &str,
        campaign: &mut models::Campaign,
        handle: &models::Handle,
        reward: U256,
        trigger: &str,
    ) -> models::Collab {
        use api::models::{OnChainJobKind, OnChainJobStatus};
        let job_kind = if handle.account().await.unwrap().addr().is_some() {
            OnChainJobKind::MakeCollabs
        } else {
            OnChainJobKind::MakeSubAccountCollabs
        };

        self.wait_for_job(
            &format!("Waiting for old job to be skipped in context {context}"),
            job_kind,
            OnChainJobStatus::Skipped,
        )
        .await;

        let collab = campaign
            .make_collab(handle, reward, trigger)
            .await
            .unwrap_or_else(|_| panic!("Collab to be created in context '{context}'"));

        let job = self
            .wait_for_job(
                &format!("A job making the collab in context {context}"),
                job_kind,
                OnChainJobStatus::Scheduled,
            )
            .await;

        self.wait_for_job_status(
            &format!("Job never confirmed {job_kind:?} in {context}"),
            &job,
            OnChainJobStatus::Settled,
        )
        .await;

        let cleared = collab.reloaded().await.unwrap();
        assert_eq!(
            *cleared.status(),
            models::CollabStatus::Cleared,
            "Collab did not clear in {context}"
        );
        cleared
    }

    pub async fn wait_for_job(
        &self,
        context: &str,
        kind: models::OnChainJobKind,
        status: models::OnChainJobStatus,
    ) -> models::OnChainJob {
        let found = wait_for(150, 50, || async {
            self.evm_mine().await;
            let new_jobs = self.app.on_chain_job().run_scheduler().await.unwrap();
            new_jobs.into_iter().any(|job| job.attrs.status == status && job.attrs.kind == kind)
        })
        .await;

        let jobs_query =
            self.app.on_chain_job().select().kind_eq(kind).order_by(models::OnChainJobOrderBy::Id).desc(true);

        if !found {
            panic!(
                "Could not find job '{context}'. Jobs where {:#?}",
                jobs_query.all().await.unwrap()
            );
        }

        jobs_query.status_eq(status).one().await.unwrap()
    }

    pub async fn wait_for_job_status(
        &self,
        context: &str,
        job: &models::OnChainJob,
        status: models::OnChainJobStatus,
    ) -> models::OnChainJob {
        let success = wait_for(100, 10, || async {
            self.evm_mine().await;
            self.app.on_chain_job().run_scheduler().await.unwrap();
            let reloaded = job.reloaded().await.unwrap();
            reloaded.attrs.status == status
        })
        .await;
        let reloaded = job.reloaded().await.unwrap();
        if !success {
            panic!("Job never got to status {status:?} in '{context}', job was: {reloaded:?}");
        }
        reloaded
    }

    pub fn private_key(&self) -> ES256KeyPair {
        let key = ES256KeyPair::from_pem(
      "-----BEGIN PRIVATE KEY-----\nMIGHAgEAMBMGByqGSM49AgEGCCqGSM49AwEHBG0wawIBAQQg626FUHw6lA0eAlYl\nqT0TI8m/JAWj+H497JAKfoTUrkmhRANCAARJPbG33RdPLOxXXbc390w00YaFAbh8\n0Hv44ScjS0UMB6/ZjjkIs5fV1gRK1IBF1JMnxM6qWjWUBlu/z9ZjvA0b\n-----END PRIVATE KEY-----\n"
    ).unwrap();
        let id = api::models::hasher::hexdigest(key.public_key().to_pem().unwrap().as_bytes());
        key.with_key_id(&id)
    }

    pub async fn get_campaign(
        &self,
        advertiser_addr: Address,
        briefing_id: U256,
    ) -> on_chain::asami_contract_code::Campaign {
        self.asami_contract()
            .get_campaign(advertiser_addr, briefing_id)
            .call()
            .await
            .unwrap_or_else(|_| panic!("no campaign {advertiser_addr}-{briefing_id}"))
    }

    pub async fn send_tx<B, M, D>(
        &self,
        reference: &str,
        max_gas: &str,
        fn_call: FunctionCall<B, M, D>,
    ) -> TransactionReceipt
    where
        B: std::borrow::Borrow<M>,
        M: Middleware,
        D: Detokenize,
    {
        match fn_call.send().await {
            Err(e) => {
                let desc = e.decode_revert::<String>().unwrap_or_else(|| format!("no_revert_error: {e:?}"));
                panic!("Sending tx with reference '{reference}' failed with: {desc}");
            }
            Ok(pending_tx) => {
                self.start_mining().await;
                let tx_hash = pending_tx.tx_hash();
                let receipt = pending_tx
                    .interval(std::time::Duration::from_millis(10))
                    .await
                    .expect("Error waiting on tx '{reference}'")
                    .expect("No receipt for tx '{reference'}");
                self.stop_mining().await;

                if receipt.status.unwrap_or(U64::zero()) == U64::zero() {
                    let original_tx = self.asami_contract().client().get_transaction(tx_hash).await.unwrap().unwrap();
                    let typed: ethers::types::transaction::eip2718::TypedTransaction = (&original_tx).into();
                    let msg = match self.asami_contract().client().call(&typed, None).await {
                        Err(e) => {
                            let default_error = format!("non_revert_error {e:?}");
                            models::on_chain_job::AsamiContractError::from_middleware_error(e)
                                .decode_revert::<String>()
                                .unwrap_or(default_error)
                        }
                        _ => "no_failure_reason_wtf".to_string(),
                    };

                    panic!("Error in receipt status for '{reference}' : {msg}");
                };

                let gas = receipt.gas_used.expect("No gas used in '{reference}'");
                assert!(
                    gas <= wei(max_gas),
                    "Sending tx with reference '{reference}' max gas was {max_gas} but used {gas}"
                );
                receipt
            }
        }
    }

    pub async fn send_revert_tx<B, M, D>(&self, reference: &str, expected_message: &str, fn_call: FunctionCall<B, M, D>)
    where
        B: std::borrow::Borrow<M>,
        M: Middleware,
        D: Detokenize,
    {
        match fn_call.send().await {
            Err(e) => {
                let desc = e.decode_revert::<String>().unwrap_or_else(|| "no_revert_error".to_string());
                assert_eq!(&desc, expected_message, "Wrong revert message on '{reference}'");
            }
            Ok(_pending_tx) => {
                panic!("Transaction should have been reverted '{reference}'");
            }
        }
    }

    pub async fn send_without_mining<B, M, D>(
        &self,
        reference: &str,
        nonce: U256,
        fn_calls: Vec<FunctionCall<B, M, D>>,
    ) -> Vec<H256>
    where
        B: std::borrow::Borrow<M>,
        M: Middleware,
        D: Detokenize,
    {
        let mut txs = vec![];
        for (i, fn_call) in fn_calls.into_iter().enumerate() {
            match fn_call.gas(1000000).nonce(nonce + U256::from(i)).send().await {
                Err(e) => {
                    let desc = e.decode_revert::<String>().unwrap_or_else(|| "no_revert_error".to_string());
                    panic!("Sending tx with reference '{reference}' failed with: {desc}");
                }
                Ok(pending_tx) => {
                    txs.push(pending_tx.tx_hash());
                }
            }
        }
        txs
    }

    pub async fn wait_tx_failure(&self, reference: &str, tx_hash: &H256, expected_message: &str) {
        let client = self.asami_contract().client();
        try_until(100, 10, &format!("Transaction '{reference}' was not found"), || async {
            self.evm_mine().await;
            client.get_transaction_receipt(*tx_hash).await.unwrap().is_some()
        })
        .await;

        let receipt = client
            .get_transaction_receipt(*tx_hash)
            .await
            .unwrap_or_else(|_| panic!("Receipt query failed when found before in '{reference}'"))
            .unwrap_or_else(|| panic!("Receipt query returned None in '{reference}'"));

        let original_tx = client
            .get_transaction(*tx_hash)
            .await
            .unwrap_or_else(|_| panic!("Original tx query failed on '{reference}'"))
            .unwrap_or_else(|| panic!("Original query was None in '{reference}'"));

        if receipt.status.unwrap_or(U64::zero()) == U64::one() {
            panic!("Transaction succeeded '{reference}' but we expected error {expected_message}");
        }

        let typed: ethers::types::transaction::eip2718::TypedTransaction = (&original_tx).into();
        let msg = match client.call(&typed, None).await {
            Err(e) => api::models::AsamiContractError::from_middleware_error(e)
                .decode_revert::<String>()
                .unwrap_or_else(|| "non_revert_error".to_string()),
            _ => "no_failure_reason_wtf".to_string(),
        };
        assert_eq!(&msg, expected_message, "Wrong revert message on '{reference}'");
    }

    pub async fn wait_tx_success(&self, reference: &str, tx_hash: &H256, max_gas: &str) -> TransactionReceipt {
        let client = self.asami_contract().client();
        try_until(
            100,
            10,
            &format!("Transaction '{reference}' never had a receipt"),
            || async {
                self.evm_mine().await;
                client.get_transaction_receipt(*tx_hash).await.unwrap().is_some()
            },
        )
        .await;

        let receipt = client
            .get_transaction_receipt(*tx_hash)
            .await
            .unwrap_or_else(|_| panic!("Receipt query failed when found before in '{reference}'"))
            .unwrap_or_else(|| panic!("Receipt query returned None in '{reference}'"));

        let original_tx = client
            .get_transaction(*tx_hash)
            .await
            .unwrap_or_else(|_| panic!("Original tx query failed on '{reference}'"))
            .unwrap_or_else(|| panic!("Original query was None in '{reference}'"));

        if receipt.status.unwrap_or(U64::zero()) == U64::zero() {
            let typed: ethers::types::transaction::eip2718::TypedTransaction = (&original_tx).into();
            let msg = match client.call(&typed, None).await {
                Err(e) => api::models::AsamiContractError::from_middleware_error(e)
                    .decode_revert::<String>()
                    .unwrap_or_else(|| "non_revert_error".to_string()),
                _ => "no_failure_reason_wtf".to_string(),
            };
            panic!("Tx failed but expected success on '{reference}'. Error was: {msg}");
        };
        let gas = receipt.gas_used.expect("No gas used in '{reference}'");
        assert!(
            gas <= wei(max_gas),
            "Sending tx with reference '{reference}' max gas was {max_gas} but used {gas}"
        );
        receipt
    }

    pub async fn sync_events_until<T: std::future::Future<Output = bool>>(&self, context: &str, call: impl Fn() -> T) {
        try_until(
            100,
            20,
            &format!("Syncing events did not have the expected effect for '{context}'"),
            || async {
                self.app.synced_event().sync_on_chain_events().await.unwrap();
                call().await
            },
        )
        .await;
    }

    pub fn submit_report_contract_call(
        &self,
        account: Address,
        briefing_hash: U256,
        report_hash: U256,
    ) -> AsamiFunctionCall {
        self.asami_contract().submit_reports(vec![on_chain::SubmitReportsParam {
            account,
            briefing_hash,
            report_hash,
        }])
    }

    pub fn future_date(&self, days: i64) -> U256 {
        models::utc_to_i(Utc::now() + chrono::Duration::days(days))
    }
}
