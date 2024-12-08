use super::{
    models::{self, *},
    *,
};

#[derive(Debug, GraphQLObject, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[graphql(description = "A contract call made by the campaign manager or oracle.")]
pub struct OnChainJob {
    #[graphql(description = "Unique numeric identifier of this resource")]
    id: i32,
    #[graphql(description = "The status of this job")]
    status: OnChainJobStatus,
    #[graphql(description = "What kind of job is it, corresponds to the contract function being called.")]
    kind: OnChainJobKind,
    #[graphql(description = "The transaction hash if the job has been submitted.")]
    tx_hash: Option<String>,
    #[graphql(description = "Gas used making this call")]
    gas_used: Option<String>,
    #[graphql(description = "Nonce used to submit the transaction")]
    nonce: Option<String>,
    #[graphql(description = "Block in which the transaction was confirmed")]
    block: Option<Decimal>,
    #[graphql(description = "An extended description about the job status as a single line")]
    status_line: Option<String>,
    #[graphql(description = "When this job will be attempted")]
    sleep_until: UtcDateTime,
    #[graphql(description = "When this job was first scheduled")]
    created_at: UtcDateTime,
}

#[derive(Debug, Clone, Default, GraphQLInputObject, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnChainJobFilter {
    ids: Option<Vec<i32>>,
    id_eq: Option<i32>,
}

#[rocket::async_trait]
impl Showable<models::OnChainJob, OnChainJobFilter> for OnChainJob {
    fn sort_field_to_order_by(field: &str) -> Option<models::OnChainJobOrderBy> {
        match field {
            "id" => Some(OnChainJobOrderBy::Id),
            "created_at" => Some(OnChainJobOrderBy::CreatedAt),
            _ => None,
        }
    }

    fn filter_to_select(_context: &Context, filter: Option<OnChainJobFilter>) -> FieldResult<models::SelectOnChainJob> {
        if let Some(f) = filter {
            Ok(models::SelectOnChainJob {
                id_in: f.ids,
                id_eq: f.id_eq,
                ..Default::default()
            })
        } else {
            Ok(Default::default())
        }
    }

    fn select_by_id(_context: &Context, id: i32) -> FieldResult<models::SelectOnChainJob> {
        Ok(models::SelectOnChainJob {
            id_eq: Some(id),
            ..Default::default()
        })
    }

    async fn db_to_graphql(_context: &Context, d: models::OnChainJob) -> AsamiResult<Self> {
        Ok(OnChainJob {
            id: d.attrs.id,
            status: d.attrs.status,
            kind: d.attrs.kind,
            tx_hash: d.attrs.tx_hash,
            gas_used: d.attrs.gas_used,
            nonce: d.attrs.nonce,
            block: d.attrs.block,
            status_line: d.attrs.status_line,
            sleep_until: d.attrs.sleep_until,
            created_at: d.attrs.created_at,
        })
    }
}
