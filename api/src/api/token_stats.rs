use super::{models::*, *};

#[derive(Debug, GraphQLObject, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenStats {
    pub id: i32,
    pub token_yield: String,
    pub price: String,
    pub payback: String,
    pub supply: String,
    pub unclaimed: String,
    pub issuance_rate: String,
    pub fee_pool: String,
    pub cycle_start: i32,
    pub cycle_end: i32,
}

impl TokenStats {
    // Price is for 1K tokens. Yield is for 1K tokens. Payback is in months.
    pub async fn price_yield_payback(app: &App) -> FieldResult<(U256, U256, U256)> {
        let period_yields: Vec<U256> = app
            .fee_pool_snapshot()
            .select()
            .all()
            .await?
            .iter()
            .map(|p| u256(p.pool()) * U256::exp10(18) / u256(p.supply()))
            .collect();

        let price = app.value_series().get(SeriesName::AsamiDocPrice).await?.value_u256() * wei("1000");

        if period_yields.is_empty() {
            Ok((price, u("0"), u("0")))
        } else {
            let average = period_yields.iter().fold(U256::zero(), |a, x| a + x) / U256::from(period_yields.len());
            let token_yield = average * wei("2") * wei("1000");
            let payback = price / token_yield;
            Ok((price, token_yield, payback))
        }
    }

    pub async fn build(app: &App) -> FieldResult<Self> {
        use SeriesName::*;
        let (price, token_yield, payback) = Self::price_yield_payback(app).await?;

        let supply = app.value_series().get(AsamiSupply).await?;
        let assigned = app.value_series().get(AsamiSupply).await?;
        let unclaimed = (assigned.value_u256() - supply.value_u256()).encode_hex();

        let cycle_length: i32 = 60 * 60 * 24 * 15;
        let cycle_start: i32 = cycle_length * (i32::try_from(Utc::now().timestamp())? / cycle_length);
        let cycle_end: i32 = cycle_start + cycle_length;

        Ok(Self {
            id: 0,
            token_yield: token_yield.encode_hex(),
            price: price.encode_hex(),
            payback: payback.encode_hex(),
            supply: supply.attrs.value,
            unclaimed,
            issuance_rate: app.value_series().get(AsamiIssuanceRate).await?.attrs.value,
            fee_pool: app.value_series().get(AsamiFeePool).await?.attrs.value,
            cycle_start,
            cycle_end,
        })
    }
}
