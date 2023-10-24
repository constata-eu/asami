use super::models;
use super::*;

use ethers::{
  signers::{MnemonicBuilder, LocalWallet, coins_bip39::English, Signer},
  prelude::{abigen, SignerMiddleware, LogMeta},
  types::Address,
  providers::{Http, Provider},
  abi::AbiEncode,
};
use std::sync::Arc;

use rust_decimal::prelude::ToPrimitive;

abigen!(AsamiContract, "../contract/build/contracts/Asami.json");

abigen!( IERC20,
  r#"[
    function approve(address spender, uint256 value) public virtual returns (bool)
  ]"#
);

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

  pub async fn send_add_handle(&self, handle: &models::Handle) -> AsamiResult<String> {
    let contract_handle = Handle {
      value: handle.attrs.value.clone().into(),
      fixed_id: handle.attrs.fixed_id.clone().unwrap().into(),
      price: handle.attrs.price.clone().unwrap().to_u64().unwrap().into(),
      score: handle.attrs.score.clone().unwrap().to_u64().unwrap().into(),
      topics: handle.topic_ids().await.unwrap().into_iter().map(|i| i.into() ).collect(),
      verification_message_id: handle.attrs.verification_message_id.as_ref().unwrap().into(),
    };
    
    Ok(self.contract
      .add_x_handle(handle.attrs.account_id.into(), contract_handle)
      .send().await?
      .tx_hash()
      .encode_hex()
    )
  }

  pub async fn send_campaign_request(&self, req: &models::CampaignRequest) -> AsamiResult<String> {
    let amount = req.attrs.budget.to_u64().unwrap().into();
    let campaign = Campaign {
      budget: amount,
      remaining: amount,
      content_id: req.attrs.content_id.clone()
    };

    self.doc_contract.approve(self.contract.address(), amount).send()
      .await?
      .confirmations(1)
      .await?;

    Ok(self.contract
      .add_requested_x_campaign(req.attrs.account_id.into(), campaign)
      .send().await?
      .tx_hash()
      .encode_hex()
    )
  }

  pub async fn events(&self, from_block: i64, to_block: i64) -> AsamiResult<Vec<(AsamiContractEvents, LogMeta)>> {
    Ok(self.contract.events().from_block(from_block).to_block(to_block).query_with_meta().await?)
  }
}
