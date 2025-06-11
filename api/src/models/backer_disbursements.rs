use crate::on_chain::BuilderRewardsClaimedFilter;

use super::*;

model! {
  state: App,
  table: backer_disbursements,
  struct BackerDisbursement {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(timestamptz, default)]
    created_at: UtcDateTime,

    #[sqlx_model_hints(decimal)]
    rif_claimed: Decimal,
    #[sqlx_model_hints(decimal)]
    btc_claimed: Decimal,

    #[sqlx_model_hints(decimal)]
    rif_usd_rate: Decimal,
    #[sqlx_model_hints(decimal)]
    btc_usd_rate: Decimal,
    #[sqlx_model_hints(decimal)]
    asami_issuance_rate: Decimal,

    #[sqlx_model_hints(varchar)]
    tx_hash: String,
  },
  has_many {
    BackerPayout(disbursement_id),
    BackerStake(disbursement_id),
  }
}

impl BackerDisbursementHub {
  pub async fn create_from_events(&self, tx_hash: H256, events: &[BuilderRewardsClaimedFilter]) -> anyhow::Result<Option<BackerDisbursement>> {
      let rif_addr = "0x2acc95758f8b5f583470ba265eb685a8f45fc9d5".parse()?;
      let btc_addr = "0xf7ab6cfaebbadfe8b5494022c4c6db776bd63b6b".parse()?;
      let builder = "0xd9fcae4315920387f00725c78285d6d41c30b967".parse()?;

      let found = self.select().tx_hash_eq(tx_hash.to_string()).optional().await?;

      if found.is_some() {
          return Ok(None);
      }

      let Some(rif) = events.iter().find(|i| i.reward_token == rif_addr && i.builder == builder) else {
          return Ok(None);
      };
      let Some(btc) = events.iter().find(|i| i.reward_token == btc_addr && i.builder == builder) else {
          return Ok(None);
      };

      let raw_issuance = self.state.on_chain.asami_contract.get_issuance_for(u("1")).call().await.context("getting asami issuance rate")?;
      let asami_issuance_rate = wei_to_decimal_safe(raw_issuance)?;

      let doc_oracle = on_chain::PriceOracleContract::from_config(&self.state.settings)?;
      let btc_price = wei_to_decimal_safe( doc_oracle.get_price().call().await.context("getting btc price")? )?;

      let rif_oracle = on_chain::UniswapContract::from_config(&self.state.settings)?;
      let rif_price = wei_to_decimal_safe( rif_oracle.price().await.context("getting rif price")?)?;

      let disbursement = self.insert(InsertBackerDisbursement{
          rif_claimed: wei_to_decimal_safe(rif.amount)?,
          btc_claimed: wei_to_decimal_safe(btc.amount)?,
          rif_usd_rate: rif_price,
          btc_usd_rate: btc_price,
          asami_issuance_rate,
          tx_hash: tx_hash.encode_hex()
      }).save().await?;


      Ok(Some(disbursement))
  }
}

model! {
  state: App,
  table: backers,
  struct Backer {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(varchar)]
    address: String,
    #[sqlx_model_hints(timestamptz, default)]
    created_at: UtcDateTime,
  },
  has_many {
    BackerPayout(backer_id),
    BackerStake(backer_id),
  }
}

impl BackerHub {
  pub async fn get_or_create(&self, addr: Address) -> sqlx::Result<Backer> {
    if let Some(found) = self.select().address_eq(addr.encode_hex()).optional().await? {
      return Ok(found);
    }

    self.insert(InsertBacker{ address: addr.encode_hex()}).save().await
  }
}

model! {
  state: App,
  table: backer_payouts,
  struct BackerPayout {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(int4)]
    backer_id: i32,
    #[sqlx_model_hints(decimal)]
    asami_amount: Decimal,
    #[sqlx_model_hints(int4)]
    disbursement_id: i32,
    #[sqlx_model_hints(boolean, default)]
    paid: bool,
    #[sqlx_model_hints(timestamptz, default)]
    created_at: UtcDateTime,
  },
  belongs_to {
    Backer(backer_id),
    BackerDisbursement(disbursement_id),
  }
}

model! {
  state: App,
  table: backer_stakes,
  struct BackerStake {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(int4)]
    backer_id: i32,
    #[sqlx_model_hints(decimal)]
    stake: Decimal,
    #[sqlx_model_hints(timestamptz, default)]
    date: UtcDateTime,
    #[sqlx_model_hints(int4)]
    disbursement_id: i32,
  },
  belongs_to {
    Backer(backer_id),
    BackerDisbursement(disbursement_id),
  }
}


impl BackerHub {
  pub async fn store_today(&self, amount: U256) -> sqlx::Result<Backer> {
    let date = Utc::now().
    if let Some(found) = self.select().address_eq(addr.encode_hex()).optional().await? {
      return Ok(found);
    }

    self.insert(InsertBacker{ address: addr.encode_hex()}).save().await
  }
}
