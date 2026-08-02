use std::{
    fs::File,
    net::{SocketAddr, TcpListener},
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    str::FromStr,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use anyhow::{bail, Context};
use asami::{chain, config::Settings, db, notifications, processor, App};
use ethers::{
    abi::Abi,
    contract::{abigen, ContractFactory},
    middleware::SignerMiddleware,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::{Address, Bytes, TransactionRequest, H256, U256},
};
use serde_json::Value;
use sqlx::{PgPool, Row};
use tokio::task::JoinHandle;

use crate::Browser;

pub const CHAIN_ID: u64 = 31337;
pub const DEPLOYER: &str = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266";
pub const DEPLOYER_KEY: &str = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
pub const PROXY_ADMIN: &str = "0x70997970C51812dc3A010C7d01b50e0d17dc79C8";
pub const OWNER: &str = "0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC";
pub const OWNER_KEY: &str = "0x5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a";
pub const AUTOMATOR: &str = "0x90F79bf6EB2c4f870365E785982E1f101E93b906";
pub const AUTOMATOR_KEY: &str = "0x7c852118294e51e653712a81e05800f419141751be58f605c371e15141b007a6";
pub const MEMBER_ONE: &str = "0x15d34AAf54267DB7D7c367839AAf71A00a2C6A65";
pub const MEMBER_TWO: &str = "0x9965507D1a55bcC2695C58ba16FB37d819B0A4dc";
pub const MEMBER_THREE: &str = "0x976EA74026E726554dB657fA54763abd0C3a0aa9";
pub const ZERO_ADDRESS: &str = "0x0000000000000000000000000000000000000000";

abigen!(
    AdminAsami,
    r#"[
      function initialize(address initialOwner, address initialAutomator)
      function addMember(address account, uint96 score, string name) returns (uint256)
      function setAssetConfig(address asset, bool enabled, uint256 minimumAmount)
      function setSenderName(string name)
      function withdrawNative(address recipient, uint256 amount)
    ]"#
);

abigen!(
    TestToken,
    r#"[
      function mint(address recipient, uint256 amount)
      function transfer(address recipient, uint256 amount) returns (bool)
    ]"#
);

type ChainClient = SignerMiddleware<Provider<Http>, LocalWallet>;

pub struct TestEnvironment {
    pub app: App,
    pub proxy: Address,
    pub database_url: String,
    pub rpc_url: String,
    pub web_url: String,
    repository: PathBuf,
    temporary: PathBuf,
    postgres_name: String,
    anvil: Option<Child>,
    api_server: Option<JoinHandle<()>>,
    browser: Option<Browser>,
}

impl TestEnvironment {
    pub async fn start() -> anyhow::Result<Self> {
        let repository = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .context("integration_tests has no parent")?
            .to_path_buf();
        let postgres_port = free_port()?;
        let rpc_port = free_port()?;
        let web_port = free_port()?;
        let unique = format!(
            "{}-{}",
            std::process::id(),
            SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis()
        );
        let temporary = PathBuf::from(format!("/tmp/asami-e2e-{unique}"));
        std::fs::create_dir_all(&temporary)?;
        let postgres_name = format!("asami-e2e-postgres-{unique}");
        let database_url = format!("postgres://postgres:asami_test@127.0.0.1:{postgres_port}/asami_e2e");
        let rpc_url = format!("http://127.0.0.1:{rpc_port}");
        let web_url = format!("http://127.0.0.1:{web_port}");

        start_postgres(&postgres_name, postgres_port)?;
        if let Err(error) = wait_for_database(&database_url).await {
            remove_postgres(&postgres_name);
            return Err(error);
        }
        let mut anvil = start_anvil(&temporary, rpc_port)?;
        if let Err(error) = wait_for_chain(&rpc_url).await {
            let _ = anvil.kill();
            remove_postgres(&postgres_name);
            return Err(error);
        }

        let proxy = deploy_contract(&repository, &rpc_url).await?;
        let provider = Provider::<Http>::try_from(rpc_url.as_str())?;
        let deploy_block = provider.get_block_number().await?.as_u64();

        let settings = Settings {
            database_url: database_url.clone(),
            rpc_url: rpc_url.clone(),
            chain_id: CHAIN_ID,
            contract_address: proxy,
            contract_deploy_block: deploy_block,
            automator_mnemonic: None,
            automator_address: Some(Address::from_str(AUTOMATOR)?),
            automator_derivation_path: "m/44'/60'/0'/0/0".to_string(),
            automator_private_key: Some(AUTOMATOR_KEY.to_string()),
            listen_address: SocketAddr::from(([127, 0, 0, 1], web_port)),
            explorer_url: "https://rootstock.blockscout.com".to_string(),
            worker_interval_ms: 10,
            rpc_poll_interval_ms: 10,
            block_chunk_size: 1000,
            finality_depth: 0,
            transfer_reconsideration_limit: 100,
            min_automator_gas_reserve: U256::one(),
            gas_safety_basis_points: 12_500,
            low_gas_tweet_cooldown_hours: 24,
            x_daily_post_cap: 20,
            x_client_id: None,
            x_client_secret: None,
            x_redirect_uri: None,
            encryption_key: None,
        };
        let app = App::new(settings).await?;
        db::migrate(&app.db).await?;

        Ok(Self {
            app,
            proxy,
            database_url,
            rpc_url,
            web_url,
            repository,
            temporary,
            postgres_name,
            anvil: Some(anvil),
            api_server: None,
            browser: None,
        })
    }

    pub async fn add_member(&self, account: &str, score: u64, name: &str) -> anyhow::Result<()> {
        let call = self.contract(OWNER_KEY)?.add_member(Address::from_str(account)?, score as u128, name.to_string());
        call.send().await?.await?.context("addMember transaction disappeared")?;
        Ok(())
    }

    pub async fn configure_asset(&self, asset: Address, enabled: bool, minimum: U256) -> anyhow::Result<()> {
        let call = self.contract(OWNER_KEY)?.set_asset_config(asset, enabled, minimum);
        call.send().await?.await?.context("setAssetConfig transaction disappeared")?;
        Ok(())
    }

    pub async fn set_sender_name(&self, name: &str) -> anyhow::Result<()> {
        self.contract(DEPLOYER_KEY)?
            .set_sender_name(name.to_string())
            .send()
            .await?
            .await?
            .context("setSenderName transaction disappeared")?;
        Ok(())
    }

    pub async fn send_native(&self, amount: U256) -> anyhow::Result<()> {
        self.client(DEPLOYER_KEY)?
            .send_transaction(TransactionRequest::new().to(self.proxy).value(amount), None)
            .await?
            .await?
            .context("native transfer disappeared")?;
        Ok(())
    }

    pub async fn withdraw_native(&self, amount: U256) -> anyhow::Result<()> {
        self.contract(OWNER_KEY)?
            .withdraw_native(Address::from_str(OWNER)?, amount)
            .send()
            .await?
            .await?
            .context("withdrawNative transaction disappeared")?;
        Ok(())
    }

    pub async fn deploy_token(&self, name: &str, symbol: &str) -> anyhow::Result<Address> {
        let (abi, bytecode) = artifact(&self.repository, "contract/out/MockToken.sol/MockToken.json")?;
        let factory = ContractFactory::new(abi, bytecode, self.client(DEPLOYER_KEY)?);
        Ok(factory.deploy((name.to_owned(), symbol.to_owned()))?.send().await?.address())
    }

    pub async fn mint_and_send_token(&self, token: Address, amount: U256) -> anyhow::Result<()> {
        let token = TestToken::new(token, self.client(DEPLOYER_KEY)?);
        token
            .mint(Address::from_str(DEPLOYER)?, amount)
            .send()
            .await?
            .await?
            .context("mint transaction disappeared")?;
        token.transfer(self.proxy, amount).send().await?.await?.context("token transfer disappeared")?;
        Ok(())
    }

    pub async fn set_automator_balance(&self, amount: U256) -> anyhow::Result<()> {
        self.app
            .provider
            .request::<_, Value>("anvil_setBalance", vec![AUTOMATOR.to_string(), format!("{amount:#x}")])
            .await?;
        Ok(())
    }

    pub async fn run_cycle(&self) -> anyhow::Result<()> {
        chain::index_once(&self.app).await?;
        processor::convert_transfers(&self.app).await?;
        processor::reconcile_submitted(&self.app).await?;
        processor::process_one_payout(&self.app).await?;
        notifications::queue_unconfigured_tokens(&self.app).await?;
        notifications::render_pending(&self.app).await?;
        Ok(())
    }

    pub async fn run_cycles(&self, count: usize) -> anyhow::Result<()> {
        for _ in 0..count {
            self.run_cycle().await?;
        }
        Ok(())
    }

    pub async fn count(&self, query: &str) -> anyhow::Result<i64> {
        Ok(sqlx::query(query).fetch_one(&self.app.db).await?.try_get(0)?)
    }

    pub async fn count_legacy_payout_transactions(&self) -> anyhow::Result<usize> {
        let rows = sqlx::query(
            "SELECT transaction_hash FROM payout_attempts
             WHERE receipt_status = 1 ORDER BY id",
        )
        .fetch_all(&self.app.db)
        .await?;
        let mut legacy = 0;
        for row in rows {
            let hash: H256 = row.get::<String, _>(0).parse()?;
            let transaction =
                self.app.provider.get_transaction(hash).await?.context("confirmed payout transaction disappeared")?;
            if transaction.transaction_type.unwrap_or_default().is_zero() {
                legacy += 1;
            }
        }
        Ok(legacy)
    }

    pub async fn start_web(&mut self, headless: bool) -> anyhow::Result<()> {
        let app = self.app.clone();
        self.api_server = Some(tokio::spawn(async move {
            if let Err(error) = asami::api::serve(app).await {
                panic!("API server failed: {error}");
            }
        }));
        wait_for_http(&format!("{}/health", self.web_url)).await?;
        self.browser = Some(Browser::start(&self.repository, &self.web_url, headless).await?);
        Ok(())
    }

    pub fn browser(&self) -> &Browser {
        self.browser.as_ref().expect("start_web must be called first")
    }

    pub async fn stop(mut self) {
        if let Some(browser) = self.browser.take() {
            browser.stop().await;
        }
        if let Some(server) = self.api_server.take() {
            server.abort();
            let _ = server.await;
        }
        self.cleanup();
    }

    fn contract(&self, private_key: &str) -> anyhow::Result<AdminAsami<ChainClient>> {
        Ok(AdminAsami::new(self.proxy, self.client(private_key)?))
    }

    fn client(&self, private_key: &str) -> anyhow::Result<Arc<ChainClient>> {
        let wallet = LocalWallet::from_str(private_key)?.with_chain_id(CHAIN_ID);
        let provider = Provider::<Http>::try_from(self.rpc_url.as_str())?.interval(Duration::from_millis(10));
        Ok(Arc::new(SignerMiddleware::new(provider, wallet)))
    }

    fn cleanup(&mut self) {
        if let Some(mut anvil) = self.anvil.take() {
            let _ = anvil.kill();
            let _ = anvil.wait();
        }
        remove_postgres(&self.postgres_name);
        let _ = std::fs::remove_dir_all(&self.temporary);
    }
}

impl Drop for TestEnvironment {
    fn drop(&mut self) {
        if let Some(server) = self.api_server.take() {
            server.abort();
        }
        self.cleanup();
    }
}

fn start_postgres(name: &str, port: u16) -> anyhow::Result<()> {
    let mut command = Command::new("docker");
    command.args([
        "run",
        "--name",
        name,
        "-e",
        "POSTGRES_PASSWORD=asami_test",
        "-e",
        "POSTGRES_DB=asami_e2e",
        "-p",
        &format!("127.0.0.1:{port}:5432"),
        "-d",
        "postgres:16-alpine",
    ]);
    checked(&mut command)?;

    for _ in 0..100 {
        let ready = Command::new("docker")
            .args(["exec", name, "pg_isready", "-U", "postgres"])
            .status()
            .map(|status| status.success())
            .unwrap_or(false);
        if ready {
            return Ok(());
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    bail!("PostgreSQL did not start")
}

async fn wait_for_database(database_url: &str) -> anyhow::Result<()> {
    let mut consecutive_successes = 0;
    for _ in 0..100 {
        let ready = if let Ok(pool) = PgPool::connect(database_url).await {
            let ready = sqlx::query("SELECT 1").execute(&pool).await.is_ok();
            pool.close().await;
            ready
        } else {
            false
        };
        if ready {
            consecutive_successes += 1;
            if consecutive_successes == 10 {
                return Ok(());
            }
        } else {
            consecutive_successes = 0;
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    bail!("PostgreSQL host connection did not become ready")
}

fn remove_postgres(name: &str) {
    let _ = Command::new("docker").args(["rm", "-f", name]).output();
}

fn start_anvil(temporary: &Path, port: u16) -> anyhow::Result<Child> {
    let log = File::create(temporary.join("anvil.log"))?;
    Command::new("anvil")
        .args([
            "--port",
            &port.to_string(),
            "--mnemonic",
            "test test test test test test test test test test test junk",
            "--silent",
        ])
        .stdout(Stdio::from(log.try_clone()?))
        .stderr(Stdio::from(log))
        .spawn()
        .context("start Anvil")
}

async fn wait_for_chain(rpc_url: &str) -> anyhow::Result<()> {
    let provider = Provider::<Http>::try_from(rpc_url)?;
    for _ in 0..100 {
        if provider.get_chainid().await.is_ok() {
            return Ok(());
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    bail!("Anvil did not start")
}

async fn deploy_contract(repository: &Path, rpc_url: &str) -> anyhow::Result<Address> {
    let wallet = LocalWallet::from_str(DEPLOYER_KEY)?.with_chain_id(CHAIN_ID);
    let provider = Provider::<Http>::try_from(rpc_url)?.interval(Duration::from_millis(10));
    let client = Arc::new(SignerMiddleware::new(provider, wallet));

    let (implementation_abi, implementation_bytecode) = artifact(repository, "contract/out/Asami.sol/Asami.json")?;
    let implementation = ContractFactory::new(implementation_abi, implementation_bytecode, client.clone())
        .deploy(())?
        .send()
        .await?
        .address();
    let initialization = AdminAsami::new(implementation, client.clone())
        .initialize(Address::from_str(OWNER)?, Address::from_str(AUTOMATOR)?)
        .calldata()
        .context("encode Asami initializer")?;

    let (proxy_abi, proxy_bytecode) = artifact(
        repository,
        "contract/out/TransparentUpgradeableProxy.sol/TransparentUpgradeableProxy.json",
    )?;
    Ok(ContractFactory::new(proxy_abi, proxy_bytecode, client)
        .deploy((implementation, Address::from_str(PROXY_ADMIN)?, initialization))?
        .send()
        .await?
        .address())
}

fn artifact(repository: &Path, relative: &str) -> anyhow::Result<(Abi, Bytes)> {
    let value: Value = serde_json::from_str(&std::fs::read_to_string(repository.join(relative))?)?;
    let abi = serde_json::from_value(value.get("abi").context("artifact has no ABI")?.clone())?;
    let bytecode = Bytes::from_str(
        value.pointer("/bytecode/object").and_then(Value::as_str).context("artifact has no bytecode")?,
    )?;
    Ok((abi, bytecode))
}

async fn wait_for_http(url: &str) -> anyhow::Result<()> {
    let client = reqwest::Client::new();
    for _ in 0..100 {
        if client.get(url).send().await.map(|response| response.status().is_success()).unwrap_or(false) {
            return Ok(());
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    bail!("HTTP server did not start")
}

fn checked(command: &mut Command) -> anyhow::Result<String> {
    let debug = format!("{command:?}");
    let output = command.output().with_context(|| format!("run {debug}"))?;
    if !output.status.success() {
        bail!(
            "{debug} failed\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(String::from_utf8(output.stdout)?)
}

fn free_port() -> anyhow::Result<u16> {
    Ok(TcpListener::bind(("127.0.0.1", 0))?.local_addr()?.port())
}
