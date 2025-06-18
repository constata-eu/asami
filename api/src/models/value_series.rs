use super::*;

model! {
  state: App,
  table: value_series,
  struct ValueSeries {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(series_name)]
    name: SeriesName,
    #[sqlx_model_hints(varchar)]
    value: String,
    #[sqlx_model_hints(timestamptz, default)]
    created_at: DateTime<Utc>,
  }
}

make_sql_enum![
    "series_name",
    pub enum SeriesName {
        AsamiDocPrice,
        AsamiSupply,
        AsamiAssignedTokens,
        AsamiIssuanceRate,
        AsamiFeePool,
    }
];

impl ValueSeriesHub {
    pub async fn store(&self, name: SeriesName) -> AsamiResult<ValueSeries> {
        use SeriesName::*;

        let asami = &self.state.on_chain.asami_contract;

        let value = match name {
            AsamiDocPrice => self.state.on_chain.asami_price_contract.price().await?,
            AsamiSupply => dbg!(asami.total_supply().call().await?),
            AsamiAssignedTokens => asami.assigned_asami_tokens().call().await?,
            AsamiIssuanceRate => asami.get_issuance_for(u("1")).call().await?,
            AsamiFeePool => asami.fee_pool().call().await?,
        }
        .encode_hex();

        if let Some(current) =
            self.select().name_eq(name).order_by(ValueSeriesOrderBy::CreatedAt).desc(true).optional().await?
        {
            if *current.value() == value {
                return Ok(current);
            }
        }

        Ok(self.insert(InsertValueSeries { name, value }).save().await?)
    }

    pub async fn get(&self, name: SeriesName) -> AsamiResult<ValueSeries> {
        Ok(self.select().name_eq(name).order_by(ValueSeriesOrderBy::CreatedAt).desc(true).one().await?)
    }
}

impl ValueSeries {
    pub fn value_u256(&self) -> U256 {
        u256(self.value())
    }
}
