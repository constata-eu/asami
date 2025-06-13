use std::collections::HashMap;

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

      let doc_oracle = &self.state.on_chain.doc_price_contract;
      let btc_price = wei_to_decimal_safe( doc_oracle.get_price().call().await.context("getting btc price")? )?;

      let rif_oracle = &self.state.on_chain.rif_price_contract;
      let rif_price = wei_to_decimal_safe(rif_oracle.price().await.context("getting rif price")?)?;

      let disbursement = self.insert(InsertBackerDisbursement{
          rif_claimed: wei_to_decimal_safe(rif.amount)?,
          btc_claimed: wei_to_decimal_safe(btc.amount)?,
          rif_usd_rate: rif_price,
          btc_usd_rate: btc_price,
          asami_issuance_rate,
          tx_hash: tx_hash.encode_hex()
      }).schedule_and_save().await?;

      Ok(Some(disbursement))
  }
}

impl InsertBackerDisbursementHub {
  pub async fn schedule_and_save(self) -> AsamiResult<BackerDisbursement> {
    let saved = self.save().await?;
    saved.state.db.execute(
      sqlx::query!("UPDATE backer_stakes SET disbursement_id = $1 WHERE disbursement_id IS NULL", *saved.id())
    ).await?;

    let stakes = saved.backer_stake_vec().await?;

    if stakes.is_empty() {
      return Ok(saved);
    }

    let total_stakes: Decimal = stakes.iter().map(|x| x.stake() ).sum();

    let asami_per_stake = saved.asamis_to_distribute() / total_stakes;

    for stake in stakes {
      saved.state.backer_payout().insert(InsertBackerPayout{
        backer_id: *stake.backer_id(),
        asami_amount: stake.attrs.stake * asami_per_stake,
        disbursement_id: *saved.id(),
      }).save().await?;
    }

    Ok(saved)
  } 
}

impl BackerDisbursement {
  pub fn asamis_to_distribute(&self) -> Decimal {
    let usd_claimed = self.rif_claimed() * self.rif_usd_rate() + self.btc_claimed() * self.btc_usd_rate();
    let fees_produced = usd_claimed / Decimal::new(10,0);
    let tokens_minted = fees_produced * self.asami_issuance_rate();
    let for_advertiser = tokens_minted * Decimal::new(30,0) / Decimal::new(100,0);
    let collective_bonus = Decimal::new(2,0);

    for_advertiser * collective_bonus
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

  pub async fn store_today_stakes(&self) -> AsamiResult<()> {
    let contract = &self.state.on_chain.collective_contract;

    for backer in self.select().all().await? {
      let stake = contract.allocation_of(backer.decoded_address()?).call().await?; 
      backer.store_today_stake(stake).await?;
    }

    Ok(())
  }
}

impl Backer {
  pub async fn store_today_stake(&self, amount: U256) -> AsamiResult<BackerStake> {
    let date = Utc::now().date_naive().and_hms_opt(0, 0, 0)
      .ok_or_else(|| Error::precondition("invalid start of day"))?
      .and_utc();

    let stake = wei_to_decimal_safe(amount)?;

    if let Some(found) = self.state.backer_stake().select().date_eq(date).backer_id_eq(self.id()).optional().await? {
      return Ok(found);
    }

    Ok(self.state.backer_stake().insert(InsertBackerStake{ backer_id: *self.id(), stake, date }).save().await?)
  }

  pub fn decoded_address(&self) -> AsamiResult<Address> {
    Address::decode_hex(self.address()).map_err(|_| Error::runtime("we stored a wrong backer address"))
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
    #[sqlx_model_hints(varchar, default)]
    tx_hash: Option<String>,
    #[sqlx_model_hints(timestamptz, default)]
    created_at: UtcDateTime,
  },
  belongs_to {
    Backer(backer_id),
    BackerDisbursement(disbursement_id),
  }
}

impl BackerPayoutHub {
  pub async fn pay_all(&self) -> AsamiResult<()> {
    let c = &self.state.on_chain.asami_contract;

    let unpaid = self.select().paid_eq(false).all().await?;

    if unpaid.is_empty() {
      self.state.info(
        "backer_payouts",
        "pay_all",
        "no_unpaid_backer_payouts"
      ).await;
      return Ok(());
    }

    let current_balance = wei_to_decimal_safe(c.balance_of(self.state.on_chain.client.address()).call().await?)?;

    let needed_balance: Decimal = unpaid.iter().map(|p| p.asami_amount() ).sum();

    if needed_balance > current_balance {
      return Err(Error::precondition(&format!("not_enough_asami_to_pay_backers, needed: {needed_balance}, current: {current_balance}")));
    }

    let min_payout = c.get_issuance_for(u("1")).call().await?;

    let mut by_backer: HashMap<i32, Vec<BackerPayout>> = HashMap::new();
    for payout in unpaid {
      by_backer.entry(*payout.backer_id()).or_default().push(payout);
    }

    for (backer_id, payouts) in by_backer {
      let backer = self.state.backer().find(backer_id).await?;

      let amount = decimal_to_wei_scaled_18(payouts.iter().map(|p| p.asami_amount() ).sum())?; 

      if amount < min_payout {
        self.state.info(
          "backer_payouts",
          "pay_all",
          format!("skipping for backer {backer_id} due to low amount: {amount} < {min_payout}")
        ).await;
        continue;
      }

      for p in &payouts {
        p.clone().update().paid(true).save().await?;
      }

      match c.transfer(backer.decoded_address()?, amount).send().await {
          Err(e) => {
            let desc = e.decode_revert::<String>().unwrap_or_else(|| "no_revert_error".to_string());
            self.state.fail("backer_payouts", "pay_all", format!("{e:?} - {desc}")).await;
          }
          Ok(pending_tx) => {
              let tx_hash = pending_tx.tx_hash().encode_hex();
              for p in &payouts {
                p.clone().update().tx_hash(Some(tx_hash.clone())).save().await?;
              }
              pending_tx.await?;
          }
      };
      tokio::time::sleep(tokio::time::Duration::from_secs(self.state.settings.rsk.blockchain_sync_cooldown)).await;
    }

    Ok(())
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
    #[sqlx_model_hints(timestamptz)]
    date: UtcDateTime,
    #[sqlx_model_hints(int4, default)]
    disbursement_id: Option<i32>,
  },
  belongs_to {
    Backer(backer_id),
    BackerDisbursement(disbursement_id),
  }
}


