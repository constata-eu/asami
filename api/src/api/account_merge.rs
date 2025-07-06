use super::{
    models::{self, *},
    *,
};

#[derive(Debug, GraphQLObject, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[graphql(description = "A request o merge two accounts.")]
pub struct AccountMerge {
    #[graphql(description = "Id of this merge request")]
    id: i32,
    #[graphql(description = "Unique and temporal numeric identifier of this resource")]
    code: Option<String>,
    #[graphql(description = "Address where the merge is going to")]
    destination: Option<String>,
    #[graphql(description = "Address accepting to be merged into another account")]
    source: Option<String>,
    #[graphql(description = "Date when this request was created")]
    created_at: DateTime<Utc>,
    #[graphql(description = "Status of the merge request")]
    status: AccountMergeStatus,
}

#[derive(Debug, Clone, Default, GraphQLInputObject, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountMergeFilter {
    ids: Option<Vec<String>>,
}

#[rocket::async_trait]
impl Showable<models::AccountMerge, AccountMergeFilter> for AccountMerge {
    fn sort_field_to_order_by(_field: &str) -> Option<models::AccountMergeOrderBy> {
        None // No sorting supported.
    }

    fn filter_to_select(context: &Context, _filter: Option<AccountMergeFilter>) -> FieldResult<models::SelectAccountMerge> {
        Ok(models::SelectAccountMerge {
            destination_id_eq: Some(context.account_id()?),
            created_at_gt: Some(AccountMergeHub::active_code_threshold()),
            ..Default::default()
        })
    }

    fn select_by_id(context: &Context, id: i32) -> FieldResult<models::SelectAccountMerge> {
        Ok(models::SelectAccountMerge {
            id_eq: Some(id),
            destination_id_eq: Some(context.account_id()?),
            created_at_gt: Some(AccountMergeHub::active_code_threshold()),
            ..Default::default()
        })
    }

    async fn db_to_graphql(_context: &Context, d: models::AccountMerge) -> AsamiResult<Self> {
        // Source address should always be there otherwise the request cannot be created.
        let destination = d.destination_account().await?.decoded_addr()?.map(|x| format!("{x:?}"));
        let source = d.source_account().await?
            .and_then(|a| a.decoded_addr().ok())
            .map(|x| format!("{x:?}") );

        Ok(AccountMerge {
            id: d.attrs.id,
            code: d.attrs.code,
            destination,
            source,
            created_at: d.attrs.created_at,
            status: d.attrs.status,
        })
    }
}
