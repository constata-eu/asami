use super::models;
use super::*;

use ethers::{
  signers::{LocalWallet, MnemonicBuilder, coins_bip39::English, Signer},
  prelude::{abigen, Abigen},
  types::Address,
  providers::{Http, Provider},
};

abigen!(AsamiContract, "../contract/build/contracts/Asami.json");

struct OnChain {
  contract: AsamiContract,
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

    let provider = Provider::<Http>::try_from(config.rsk.rpc_url)?;
    let client = std::sync::Arc::new(
      SignerMiddleware::new(provider, wallet.with_chain_id(config.rsk.chain_id))
    );
    let address: Address = config.rsk.contract_address.parse()?;

    Ok(Self{ contract: AsamiContract::new(address, client) })
  }

  pub async fn send_add_handle(handle: &models::Handle) -> AsamiResult<String> {
    let contract_handle = ::api::asami_contract::Handle {
      value: handle.attrs.value.clone().into(),
      fixed_id: handle.attrs.fixed_id.clone().unwrap().into(),
      price: handle.attrs.price.clone().unwrap().to_u64().unwrap().into(),
      score: handle.attrs.score.clone().unwrap().to_u64().unwrap().into(),
      topics: handle.topic_ids().await.unwrap().into_iter().map(|i| i.into() ).collect(),
      verification_message_id: handle.attrs.verification_message_id.unwrap().into(),
    };
    
    Ok(self.contract
      .add_x_handle(handle.attrs.account_id.into(), contract_handle)
      .send().await?
      .hex_encode()
    )
  }

  pub async fn events(from_block: i32) -> AsamiResult<ethers::abi::Events<'_>> {
    Ok(self.on_chain.contract.events().from_block(from_block).query_with_meta().await?)
  }
}
