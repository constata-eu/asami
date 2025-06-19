use super::{models, *};

#[derive(Debug, GraphQLObject, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[graphql(description = "A holder of ASAMI tokens")]
pub struct Holder {
    #[graphql(description = "Unique numeric identifier of this resource")]
    id: i32,
    address: String,
    balance: String,
    is_contract: bool,
    estimated_total_doc_claimed: String,
}

#[derive(Debug, Clone, Default, GraphQLInputObject, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HolderFilter {
    ids: Option<Vec<i32>>,
    id_eq: Option<i32>,
    address_ilike: Option<String>,
}

#[rocket::async_trait]
impl Showable<models::Holder, HolderFilter> for Holder {
    fn sort_field_to_order_by(field: &str) -> Option<models::HolderOrderBy> {
        match field {
            "id" => Some(models::HolderOrderBy::Id),
            "balance" => Some(models::HolderOrderBy::Balance),
            "estimatedTotalDocClaimed" => Some(models::HolderOrderBy::EstimatedTotalDocClaimed),
            _ => None,
        }
    }

    fn filter_to_select(_context: &Context, filter: Option<HolderFilter>) -> FieldResult<models::SelectHolder> {
        if let Some(f) = filter {
            let address = f.address_ilike.as_ref().map(|x| x.strip_prefix("0x").unwrap_or(x) );
            Ok(models::SelectHolder {
                id_in: f.ids,
                id_eq: f.id_eq,
                address_ilike: into_like_search(address),
                ..Default::default()
            })
        } else {
            Ok(Default::default())
        }
    }

    fn select_by_id(_context: &Context, id: i32) -> FieldResult<models::SelectHolder> {
        Ok(models::SelectHolder {
            id_eq: Some(id),
            ..Default::default()
        })
    }

    async fn db_to_graphql(_context: &Context, d: models::Holder) -> AsamiResult<Self> {
        Ok(Holder {
            id: d.attrs.id,
            address: d.attrs.address,
            balance: d.attrs.balance,
            is_contract: d.attrs.is_contract,
            estimated_total_doc_claimed: d.attrs.estimated_total_doc_claimed,
        })
    }
}
