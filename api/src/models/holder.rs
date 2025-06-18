use super::*;

model! {
  state: App,
  table: holders,
  struct Holder {
    #[sqlx_model_hints(int4, default, op_in)]
    id: i32,
    #[sqlx_model_hints(varchar, op_ilike)]
    address: String,
    #[sqlx_model_hints(varchar)]
    balance: String,
    #[sqlx_model_hints(varchar, default, op_lt)]
    last_auto_paid_cycle: String,
    #[sqlx_model_hints(boolean)]
    is_contract: bool,
    #[sqlx_model_hints(varchar, default)]
    estimated_total_doc_claimed: String,
  },
  has_many {
      EstimatedFeePoolClaim(holder_id)
  }
}

impl HolderHub {
    pub async fn force_hydrate(&self) -> AsamiResult<()> {
        let ids = self
            .state
            .db
            .fetch_all_scalar(sqlx::query_scalar!(
                "SELECT id FROM holders WHERE force_hydrate = true LIMIT 50"
            ))
            .await?;
        if ids.is_empty() {
            return Ok(());
        }
        self.hydrate_estimated_total_claims_for(ids.iter().copied()).await?;
        self.state
            .db
            .execute(sqlx::query!(
                "UPDATE holders SET force_hydrate = false WHERE id = ANY($1)",
                &ids
            ))
            .await?;

        Ok(())
    }

    pub async fn hydrate_estimated_total_claims_for(&self, ids: impl Iterator<Item = i32>) -> AsamiResult<()> {
        for id in ids {
            let holder = self.find(id).await?;

            let mut total_doc = u("0");

            if *holder.is_contract() {
                continue;
            }

            for claim in holder.estimated_fee_pool_claim_vec().await? {
                if u256(claim.balance()) <= u("4000") {
                    continue;
                }
                total_doc += u256(claim.amount()); 
            }

            holder.update().estimated_total_doc_claimed(total_doc.encode_hex()).save().await?;
        }

        Ok(())
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
    #[sqlx_model_hints(varchar, op_gt)]
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
