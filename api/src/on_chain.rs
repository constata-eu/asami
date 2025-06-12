use std::sync::Arc;

use ethers::middleware::NonceManagerMiddleware;
pub use ethers::{
    prelude::{abigen, LogMeta, Middleware, SignerMiddleware},
    providers::{Http, PendingTransaction, Provider},
    signers::{coins_bip39::English, LocalWallet, MnemonicBuilder, Signer},
    types::{Address, U256, U64},
};

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

pub type AsamiMiddleware = SignerMiddleware<NonceManagerMiddleware<Provider<Http>>, LocalWallet>;
pub type LegacyContract = LegacyContractCode<AsamiMiddleware>;
pub type DocContract = IERC20<AsamiMiddleware>;
pub type AsamiContract = AsamiContractCode<AsamiMiddleware>;
pub type CollectiveContract = CollectiveCode<Provider<Http>>;
pub type PriceOracleContract = PriceOracleCode<Provider<Http>>;
pub type UniswapContract = UniswapCode<Provider<Http>>;

impl CollectiveContract {
    pub fn from_config(config: &AppConfig) -> AsamiResult<Self> {
        let provider = Provider::<Http>::try_from(&config.rsk.mainnet_readonly_rpc_url)
            .map_err(|_| Error::Init("Invalid rsk mainnet_readonly_rpc_url in config".to_string()))?;

        let address: Address = "0x78349782F753a593ceBE91298dAfdB9053719228"
            .parse()
            .map_err(|_| Error::Init("Invalid collective_gague contract address".to_string()))?;

        Ok(Self::new(address, std::sync::Arc::new(provider)))
    }
}

impl PriceOracleContract {
    pub fn from_config(config: &AppConfig) -> AsamiResult<Self> {
        let provider = Provider::<Http>::try_from(&config.rsk.mainnet_readonly_rpc_url)
            .map_err(|_| Error::Init("Invalid rsk mainnet_readonly_rpc_url in config".to_string()))?;

        let address: Address = "0xe2927a0620b82A66D67F678FC9B826b0E01b1BFd"
            .parse()
            .map_err(|_| Error::Init("Invalid doc oracle contract address".to_string()))?;

        Ok(Self::new(address, std::sync::Arc::new(provider)))
    }
}

impl UniswapContract {
    pub fn from_config(config: &AppConfig) -> AsamiResult<Self> {
        let provider = Provider::<Http>::try_from(&config.rsk.mainnet_readonly_rpc_url)
            .map_err(|_| Error::Init("Invalid rsk mainnet_readonly_rpc_url in config".to_string()))?;

        let address: Address = "0xa89a86d3d9481a741833208676fa57d0f1d5c6cb"
            .parse()
            .map_err(|_| Error::Init("Invalid uniswap contract address".to_string()))?;

        Ok(Self::new(address, std::sync::Arc::new(provider)))
    }

    pub async fn price(&self) -> AsamiResult<U256> {
        let sqrt_price_x96 = self.slot_0().call().await?.0;
        let prod = sqrt_price_x96.checked_mul(sqrt_price_x96).ok_or_else(|| Error::runtime("checked mul of u256 failed"))?;
        let denom = U256::from(1) << 192;
        // 18 to make the number 'wei'. 12 to fix for USDT precision.
        let scale = U256::exp10(18 + 12);
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

        let provider = Provider::<Http>::try_from(&config.rsk.rpc_url)
            .map_err(|_| Error::Init("Invalid rsk rpc_url in config".to_string()))?;

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
            collective_contract: CollectiveContract::from_config(&config)?,
            doc_contract: IERC20::new(doc_address, client.clone()),
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
}

