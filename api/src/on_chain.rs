use super::*;

use ethers::{
  signers::{MnemonicBuilder, LocalWallet, coins_bip39::English, Signer},
  prelude::{abigen, SignerMiddleware, LogMeta},
  types::Address,
  providers::{Http, Provider},
  types::U64,
};
use std::sync::Arc;

abigen!(
  AsamiContract,
  "../contract/build/contracts/Asami.json",
  derives(serde::Deserialize, serde::Serialize),
);

abigen!(
  IERC20,
  r#"[
    function approve(address spender, uint256 value) public virtual returns (bool)
  ]"#,
  derives(serde::Deserialize, serde::Serialize),
);
//event Approval(address indexed _owner, address indexed _spender, uint256 _value)

#[derive(Clone)]
pub struct OnChain {
  pub contract: AsamiContract<SignerMiddleware<Provider<Http>, LocalWallet>>,
  pub doc_contract: IERC20<SignerMiddleware<Provider<Http>, LocalWallet>>,
}

impl OnChain {
  pub async fn new(config: &AppConfig, password: &str) -> AsamiResult<Self> {
    let wallet = MnemonicBuilder::<English>::default()
      .phrase(config.rsk.wallet_mnemonic.as_str())
      .password(&password)
      .build()?;

    let wallet_address: String = serde_json::to_string(&wallet.address())
      .and_then(|s| serde_json::from_str::<String>(&s) )
      .map_err(|_| Error::Init("Could not serialize wallet address to json".to_string()) )?;

    if wallet_address != config.rsk.admin_address {
      return Err(Error::Init("Bad wallet password. Address does not match mnemonic.".to_string()));
    }

    let provider = Provider::<Http>::try_from(&config.rsk.rpc_url)
      .map_err(|_| Error::Init("Invalid rsk rpc_url in config".to_string()) )?;
    let client = Arc::new(
      SignerMiddleware::new(provider, wallet.with_chain_id(config.rsk.chain_id))
    );
    let address: Address = config.rsk.contract_address.parse()
      .map_err(|_| Error::Init("Invalid asami contract address in config".to_string()) )?;

    let doc_address: Address = config.rsk.doc_contract_address.parse()
      .map_err(|_| Error::Init("Invalid doc contract address in config".to_string()) )?;

    Ok(Self{
      contract: AsamiContract::new(address, client.clone()),
      doc_contract: IERC20::new(doc_address, client),
    })
  }

  pub async fn events(&self, from_block: U64, to_block: U64) -> AsamiResult<Vec<(AsamiContractEvents, LogMeta)>> {
    Ok(self.contract.events().from_block(from_block).to_block(to_block).query_with_meta().await?)
  }
}