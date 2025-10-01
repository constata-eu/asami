use std::{sync::Arc, time::Duration};

use ethers::middleware::NonceManagerMiddleware;
pub use ethers::{
    prelude::{abigen, LogMeta, Middleware, SignerMiddleware},
    providers::{Http, HttpRateLimitRetryPolicy, PendingTransaction, Provider, RetryClient, RetryClientBuilder},
    signers::{coins_bip39::English, LocalWallet, MnemonicBuilder, Signer},
    types::{Address, U256, U64},
};
use url::Url;

use async_trait::async_trait;
use ethers::providers::{HttpClientError, JsonRpcClient};
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

use super::*;

abigen!(
    LegacyContractCode,
    "../contract/ignition/deployments/chain-31337/artifacts/LocalAsami#Asami.json",
    derives(serde::Deserialize, serde::Serialize),
);

abigen!(
    AsamiContractCode,
    "../contract/ignition/deployments/chain-31337/artifacts/LocalAsami#AsamiCore.json",
    derives(serde::Deserialize, serde::Serialize),
);

abigen!(
    IERC20,
    r#"[
    function approve(address spender, uint256 value) public virtual returns (bool)
    function balanceOf(address account) external view returns (uint256)
    function transfer(address receiver, uint256 value) public virtual returns (bool)
    function allowance(address owner, address spender) external view returns (uint256)
  ]"#,
    derives(serde::Deserialize, serde::Serialize),
);
abigen!(
    CollectiveCode,
    r#"[
      event BackerRewardsClaimed(address indexed rewardToken_, address indexed backer_, uint256 amount_)
      event BuilderRewardsClaimed(address indexed rewardToken_, address indexed builder_, uint256 amount_)
      event NewAllocation(address indexed backer_, uint256 allocation_)
      event NotifyReward(address indexed rewardToken_, uint256 builderAmount_, uint256 backersAmount_)
      function allocationOf(address) view returns (uint256)
    ]"#,
    derives(serde::Deserialize, serde::Serialize),
);
abigen!(
    PriceOracleCode,
    r#"[
        function getPrice() external override view returns (uint256)
    ]"#,
    derives(serde::Deserialize, serde::Serialize),
);

abigen!(
    UniswapCode,
    r#"
    [
      {
        "inputs": [],
        "name": "slot0",
        "outputs": [
          { "internalType": "uint160", "name": "sqrtPriceX96", "type": "uint160" },
          { "internalType": "int24", "name": "tick", "type": "int24" },
          { "internalType": "uint16", "name": "observationIndex", "type": "uint16" },
          { "internalType": "uint16", "name": "observationCardinality", "type": "uint16" },
          { "internalType": "uint16", "name": "observationCardinalityNext", "type": "uint16" },
          { "internalType": "uint8", "name": "feeProtocol", "type": "uint8" },
          { "internalType": "bool", "name": "unlocked", "type": "bool" }
        ],
        "stateMutability": "view",
        "type": "function"
      }
    ]
    "#,
    derives(serde::Deserialize, serde::Serialize),
);

pub type AsamiProvider = Provider<RetryClient<LoggingHttp>>;
pub type AsamiMiddleware = SignerMiddleware<NonceManagerMiddleware<AsamiProvider>, LocalWallet>;
pub type LegacyContract = LegacyContractCode<AsamiMiddleware>;
pub type DocContract = IERC20<AsamiMiddleware>;
pub type AsamiContract = AsamiContractCode<AsamiMiddleware>;
pub type CollectiveContract = CollectiveCode<AsamiProvider>;
pub type PriceOracleContract = PriceOracleCode<AsamiProvider>;
pub type UniswapContractInner = UniswapCode<AsamiProvider>;

trait ReadonlyContract: Sized {
    fn from_config(config: &AppConfig, addr: &str) -> AsamiResult<Self> {
        let url = config.rsk.readonly_mainnet_rpc_url.as_ref().unwrap_or(&config.rsk.rpc_url);
        let interval = config
            .rsk
            .mainnet_rpc_polling_interval_milli
            .as_ref()
            .unwrap_or(&config.rsk.rpc_polling_interval_milli);

        let provider = OnChain::make_asami_provider(url, *interval)?;

        let address: Address =
            addr.parse().map_err(|_| Error::Init("Invalid readonly contract address".to_string()))?;

        Ok(Self::make(address, std::sync::Arc::new(provider)))
    }

    fn make(addr: Address, provider: Arc<AsamiProvider>) -> Self;
}

impl ReadonlyContract for CollectiveContract {
    fn make(addr: Address, provider: Arc<AsamiProvider>) -> Self {
        Self::new(addr, provider)
    }
}

impl ReadonlyContract for PriceOracleContract {
    fn make(addr: Address, provider: Arc<AsamiProvider>) -> Self {
        Self::new(addr, provider)
    }
}

impl ReadonlyContract for UniswapContractInner {
    fn make(addr: Address, provider: Arc<AsamiProvider>) -> Self {
        Self::new(addr, provider)
    }
}

#[derive(Debug, Clone)]
pub struct UniswapContract {
    inner: UniswapContractInner,
    decimals: usize,
}

impl UniswapContract {
    pub fn from_config(config: &AppConfig, addr: &str, decimals: usize) -> AsamiResult<Self> {
        Ok(Self {
            inner: UniswapContractInner::from_config(config, addr)?,
            decimals,
        })
    }

    pub async fn price(&self) -> AsamiResult<U256> {
        let sqrt_price_x96 = self.inner.slot_0().call().await?.0;
        let prod = sqrt_price_x96
            .checked_mul(sqrt_price_x96)
            .ok_or_else(|| Error::runtime("checked mul of u256 failed"))?;
        let denom = U256::from(1) << 192;
        let scale = U256::exp10(18 + (18 - self.decimals));
        Ok(prod * scale / denom)
    }
}

#[derive(Debug, Clone)]
pub struct OnChain {
    pub client: Arc<AsamiMiddleware>,
    pub legacy_contract: LegacyContract,
    pub asami_contract: AsamiContract,
    pub asami_address: Address,
    pub doc_contract: DocContract,
    pub collective_contract: CollectiveContract,
    pub doc_price_contract: PriceOracleContract,
    pub rif_price_contract: UniswapContract,
    pub asami_price_contract: UniswapContract,
}

impl OnChain {
    pub async fn new(config: &AppConfig, password: &str) -> AsamiResult<Self> {
        let wallet = MnemonicBuilder::<English>::default()
            .phrase(config.rsk.wallet_mnemonic.as_str())
            .password(password)
            .build()?;

        if wallet.address() != config.rsk.admin_address {
            return Err(Error::Init(
                "Bad wallet password. Address does not match mnemonic.".to_string(),
            ));
        }

        let provider = Self::make_asami_provider(&config.rsk.rpc_url, config.rsk.rpc_polling_interval_milli)?;

        let nonce_manager = NonceManagerMiddleware::new(provider, wallet.address());

        let signer_middleware = SignerMiddleware::new(nonce_manager, wallet.with_chain_id(config.rsk.chain_id));

        let client = Arc::new(signer_middleware);
        let legacy_address: Address = config
            .rsk
            .legacy_contract_address
            .parse()
            .map_err(|_| Error::Init("Invalid asami contract address in config".to_string()))?;

        let asami_address: Address = config
            .rsk
            .asami_contract_address
            .parse()
            .map_err(|_| Error::Init("Invalid asami contract address in config".to_string()))?;

        let doc_address: Address = config
            .rsk
            .doc_contract_address
            .parse()
            .map_err(|_| Error::Init("Invalid doc contract address in config".to_string()))?;

        Ok(Self {
            legacy_contract: LegacyContract::new(legacy_address, client.clone()),
            asami_contract: AsamiContract::new(asami_address, client.clone()),
            asami_address,
            doc_contract: IERC20::new(doc_address, client.clone()),
            collective_contract: CollectiveContract::from_config(config, "0x78349782F753a593ceBE91298dAfdB9053719228")?,
            doc_price_contract: PriceOracleContract::from_config(config, "0xe2927a0620b82A66D67F678FC9B826b0E01b1BFd")?,
            rif_price_contract: UniswapContract::from_config(config, "0xa89a86d3d9481a741833208676fa57d0f1d5c6cb", 6)?,
            asami_price_contract: UniswapContract::from_config(
                config,
                "0x90508e187c7defe5ca1768cea45e4a1ea818594b",
                18,
            )?,
            client,
        })
    }

    pub async fn get_timestamp(&self) -> anyhow::Result<U256> {
        let client = self.asami_contract.client();
        let block_number = client.get_block_number().await?;
        client
            .get_block(block_number)
            .await?
            .map(|b| b.timestamp)
            .ok_or_else(|| anyhow::anyhow!("could_not_find_block_with_timestamp"))
    }

    pub async fn admin_rbtc_balance(&self) -> anyhow::Result<U256> {
        let client = self.asami_contract.client();
        Ok(client.get_balance(client.address(), None).await?)
    }

    pub async fn admin_unclaimed_asami_balance(&self) -> anyhow::Result<U256> {
        Ok(self.asami_contract.accounts(self.asami_contract.client().address()).await?.3)
    }

    pub fn make_asami_provider(url: &str, interval: u64) -> AsamiResult<AsamiProvider> {
        let reqwest_client = reqwest::Client::builder()
            .timeout(Duration::from_secs(40))
            .pool_idle_timeout(Duration::from_secs(30))
            .pool_max_idle_per_host(1)
            .tcp_keepalive(Some(Duration::from_secs(60)))
            .build()
            .map_err(|_| Error::Init("Could not build reqwest client for on-chain".to_string()))?;

        let url = Url::parse(url).map_err(|_| Error::Init("Invalid rsk rpc_url in config".to_string()))?;

        let http = LoggingHttp::new(url, reqwest_client);

        let retry_client = RetryClientBuilder::default()
            .timeout_retries(2)
            .rate_limit_retries(1)
            .initial_backoff(Duration::from_millis(30000)) // Initial backoff duration
            .build(http, Box::new(HttpRateLimitRetryPolicy));

        let provider = Provider::new(retry_client).interval(std::time::Duration::from_millis(interval));

        Ok(provider)
    }
}

#[derive(Clone, Debug)]
pub struct LoggingHttp {
    inner: Http,
    client: reqwest::Client,
    url: url::Url,
}

impl LoggingHttp {
    pub fn new(url: url::Url, client: reqwest::Client) -> Self {
        let inner = Http::new_with_client(url.clone(), client.clone());
        Self { inner, client, url }
    }
}

#[async_trait]
impl JsonRpcClient for LoggingHttp {
    type Error = HttpClientError;

    async fn request<T, R>(&self, method: &str, params: T) -> Result<R, Self::Error>
    where
        T: Debug + Serialize + Send + Sync,
        R: DeserializeOwned + Send,
    {
        let params_json = serde_json::to_value(&params).unwrap_or_else(|_| serde_json::Value::Null);
        match self.inner.request::<T, R>(method, params).await {
            Ok(res) => Ok(res),
            Err(err) => {
                // Log raw HTTP traffic for debugging
                let body = serde_json::json!({
                    "jsonrpc": "2.0",
                    "method": method,
                    "params": params_json,
                    "id": 1,
                });

                match self.client.post(self.url.clone()).json(&body).send().await {
                    Ok(resp) => {
                        let status = resp.status();
                        let headers = resp.headers().clone();
                        let text = resp.text().await.unwrap_or_default();

                        println!("--- RPC ERROR ---");
                        println!("url: {}", self.url);
                        println!("status: {}", status);
                        println!("request_body: {}", body);
                        println!("response_headers: {:#?}", headers);
                        println!("response_body: {}", text);
                        println!("serde_error: {err:?}");
                    }
                    Err(e) => {
                        println!("--- RPC ERROR ---");
                        println!("url: {}", self.url);
                        println!("could not resend request for debug: {e:?}");
                        println!("original error: {err:?}");
                    }
                }

                Err(err)
            }
        }
    }
}
