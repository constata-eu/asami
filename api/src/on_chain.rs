use super::*;

pub use ethers::{
  prelude::{abigen, LogMeta, SignerMiddleware},
  providers::{Http, Provider},
  signers::{coins_bip39::English, LocalWallet, MnemonicBuilder, Signer},
  types::{Address, U64},
};
use std::sync::Arc;

abigen!(
  LegacyContract,
  "../contract/build/contracts/Asami.json",
  derives(serde::Deserialize, serde::Serialize),
);

abigen!(
  AsamiContract,
  "../contract/build/contracts/AsamiCore.json",
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

pub type LegacyContract = LegacyContract<SignerMiddleware<Provider<Http>, LocalWallet>>;
pub type DocContract = IERC20<SignerMiddleware<Provider<Http>, LocalWallet>>;
pub type AsamiContract = AsamiContract<SignerMiddleware<Provider<Http>, LocalWallet>>;

#[derive(Clone)]
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

    let wallet_address: String = serde_json::to_string(&wallet.address())
      .and_then(|s| serde_json::from_str::<String>(&s))
      .map_err(|_| Error::Init("Could not serialize wallet address to json".to_string()))?;

    if wallet_address != config.rsk.admin_address {
      return Err(Error::Init(
        "Bad wallet password. Address does not match mnemonic.".to_string(),
      ));
    }

    let provider = Provider::<Http>::try_from(&config.rsk.rpc_url)
      .map_err(|_| Error::Init("Invalid rsk rpc_url in config".to_string()))?;

    let client = Arc::new(SignerMiddleware::new(
      provider,
      wallet.with_chain_id(config.rsk.chain_id),
    ));
    let address: Address = config
      .rsk
      .contract_address
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
      legacy_contract: LegacyContract::new(address, client.clone()),
      asami_contract: AsamiContract::new(asami_address, client.clone()),
      doc_contract: IERC20::new(doc_address, client),
    })
  }
}
