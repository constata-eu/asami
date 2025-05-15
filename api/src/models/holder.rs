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
    contract_cycle: String,
    #[sqlx_model_hints(timestamptz, default)]
    created_at: DateTime<Utc>,
  }
}

impl EstimatedFeePoolClaimHub {
  pub async fn create_if_missing(&self, holder: &Holder, amount: U256, contract_cycle: U256) -> AsamiResult<()> {
    if self.select().holder_id_eq(holder.id()).contract_cycle_eq(contract_cycle.encode_hex()).count().await? > 0 {
      return Ok(());
    }

    self.insert(InsertEstimatedFeePoolClaim{
      holder_id: *holder.id(),
      amount: amount.encode_hex(),
      contract_cycle: contract_cycle.encode_hex()
    }).save().await?;

    Ok(())
  }
}
