use std::sync::Arc;

pub use ethers::{
    prelude::{abigen, LogMeta, Middleware, SignerMiddleware},
    providers::{Http, Provider, PendingTransaction},
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
  ]"#,
    derives(serde::Deserialize, serde::Serialize),
);

pub type AsamiMiddleware = SignerMiddleware<Provider<Http>,LocalWallet>;
pub type LegacyContract = LegacyContractCode<AsamiMiddleware>;
pub type DocContract = IERC20<AsamiMiddleware>;
pub type AsamiContract = AsamiContractCode<AsamiMiddleware>;

#[derive(Debug, Clone)]
pub struct OnChain {
    pub legacy_contract: LegacyContract,
    pub asami_contract: AsamiContract,
    pub doc_contract: DocContract,
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

        let signer_middleware = SignerMiddleware::new(
            provider,
            wallet.with_chain_id(config.rsk.chain_id),
        );

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
            doc_contract: IERC20::new(doc_address, client),
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
