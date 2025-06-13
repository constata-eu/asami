use super::*;

model! {
  state: App,
  table: holders,
  struct Holder {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(varchar)]
    address: String,
    #[sqlx_model_hints(varchar)]
    balance: String,
    #[sqlx_model_hints(varchar, default, op_lt)]
    last_auto_paid_cycle: String,
    #[sqlx_model_hints(boolean)]
    is_contract: bool,
  }
}

impl Holder {
    pub fn decoded_address(&self) -> AsamiResult<Address> {
        Address::decode_hex(self.address()).map_err(|_| Error::validation("invalid_address", self.address()))
    }
}

model! {
  state: App,
  table: estimated_fee_pool_claims,
  struct EstimatedFeePoolClaim {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(int4)]
    holder_id: i32,
    #[sqlx_model_hints(varchar)]
    amount: String,
    #[sqlx_model_hints(varchar)]
    balance: String,
    #[sqlx_model_hints(varchar)]
    contract_cycle: String,
    #[sqlx_model_hints(timestamptz, default)]
    created_at: DateTime<Utc>,
  }
}

impl EstimatedFeePoolClaimHub {
    pub async fn create_if_missing(&self, holder: &Holder, amount: U256, balance: U256, contract_cycle: U256) -> AsamiResult<()> {
        if self
            .select()
            .holder_id_eq(holder.id())
            .contract_cycle_eq(contract_cycle.encode_hex())
            .count()
            .await?
            > 0
        {
            return Ok(());
        }

        self.insert(InsertEstimatedFeePoolClaim {
            holder_id: *holder.id(),
            amount: amount.encode_hex(),
            contract_cycle: contract_cycle.encode_hex(),
            balance: balance.encode_hex(),
        })
        .save()
        .await?;

        Ok(())
    }

    pub async fn populate_new_column_data(&self) -> AsamiResult<()> {
        let snapshots = [
            (
                "0x0000000000000000000000000000000000000000000000000000000000000019".to_string(),
                u("2800000")
            ),
            (
                "0x000000000000000000000000000000000000000000000000000000000000001a".to_string(),
                u("2900000")
            ),
        ];

        for (cycle, supply) in snapshots {
            let mut pool = u("0");
            for claim in self.state
                .estimated_fee_pool_claim()
                .select()
                .contract_cycle_eq(&cycle)
                .all().await? {
                    pool += u256(claim.amount());
                }

            self.state.fee_pool_snapshot().create_if_missing(u256(&cycle), pool, supply).await?;
            let claims = self.state.estimated_fee_pool_claim().select()
                .balance_eq(weihex("0"))
                .contract_cycle_eq(&cycle)
                .all()
                .await?;

            dbg!(&claims);

            for claim in claims {
                let balance = u256(claim.amount()) * supply / pool;
                claim.update().balance(balance.encode_hex()).save().await?;
            }
        }

        Ok(())
    }
}

model! {
  state: App,
  table: fee_pool_snapshots,
  struct FeePoolSnapshot {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(varchar)]
    cycle: String,
    #[sqlx_model_hints(varchar)]
    pool: String,
    #[sqlx_model_hints(varchar)]
    supply: String,
  }
}

impl FeePoolSnapshotHub {
    pub async fn create_if_missing(&self, cycle: U256, pool: U256, supply: U256) -> AsamiResult<()> {
        if self
            .select()
            .cycle_eq(cycle.encode_hex())
            .count()
            .await?
            > 0
        {
            return Ok(());
        }

        self.insert(InsertFeePoolSnapshot {
            cycle: cycle.encode_hex(),
            pool: pool.encode_hex(),
            supply: supply.encode_hex(),
        })
        .save()
        .await?;

        Ok(())
    }
}
