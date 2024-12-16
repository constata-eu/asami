use super::{
    models::{self, *},
    *,
};

#[derive(Debug, GraphQLObject, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[graphql(description = "A collab is created when a member reposts a campaign's content.")]
pub struct Collab {
    #[graphql(description = "Unique numeric identifier of this resource in the smart contract.")]
    id: i32,
    #[graphql(description = "The campaign whose content was reposted.")]
    campaign_id: i32,
    #[graphql(description = "The person that created the campaign.")]
    advertiser_id: i32,
    #[graphql(description = "The handle that reposted the content.")]
    handle_id: i32,
    #[graphql(description = "The member who owns the handle.")]
    member_id: i32,
    #[graphql(description = "Status of this collab.")]
    status: CollabStatus,
    #[graphql(description = "Reason to dispute this collab, if any.")]
    dispute_reason: Option<String>,
    #[graphql(description = "The gross amount paid by the advertiser (campaign creator) for this collab.")]
    reward: String,
    #[graphql(
        description = "The fee deducted by asami from the gross amount, field only available when reward cleared."
    )]
    fee: Option<String>,
    collab_trigger_unique_id: String,
}

#[derive(Debug, Clone, Default, GraphQLInputObject, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollabFilter {
    ids: Option<Vec<i32>>,
    id_eq: Option<i32>,
    campaign_id_eq: Option<i32>,
    campaign_id_in: Option<Vec<i32>>,
    advertiser_id_eq: Option<i32>,
    advertiser_id_in: Option<Vec<i32>>,
    handle_id_eq: Option<i32>,
    handle_id_in: Option<Vec<i32>>,
    member_id_eq: Option<i32>,
    member_id_in: Option<Vec<i32>>,
    status_eq: Option<CollabStatus>,
    status_ne: Option<CollabStatus>,
}

#[rocket::async_trait]
impl Showable<models::Collab, CollabFilter> for Collab {
    fn sort_field_to_order_by(field: &str) -> Option<models::CollabOrderBy> {
        match field {
            "id" => Some(CollabOrderBy::Id),
            "reward" => Some(CollabOrderBy::Reward),
            "fee" => Some(CollabOrderBy::Fee),
            _ => None,
        }
    }

    fn filter_to_select(_context: &Context, filter: Option<CollabFilter>) -> FieldResult<models::SelectCollab> {
        if let Some(f) = filter {
            Ok(models::SelectCollab {
                id_in: f.ids,
                campaign_id_eq: f.campaign_id_eq,
                campaign_id_in: f.campaign_id_in,
                advertiser_id_eq: f.advertiser_id_eq.map(i32_to_hex),
                advertiser_id_in: f.advertiser_id_in.map(|ids| ids.into_iter().map(i32_to_hex).collect() ),
                handle_id_eq: f.handle_id_eq,
                handle_id_in: f.handle_id_in,
                member_id_eq: f.member_id_eq.map(i32_to_hex),
                member_id_in: f.member_id_in.map(|ids| ids.into_iter().map(i32_to_hex).collect() ),
                id_eq: f.id_eq,
                ..Default::default()
            })
        } else {
            Ok(Default::default())
        }
    }

    fn select_by_id(_context: &Context, id: i32) -> FieldResult<models::SelectCollab> {
        Ok(models::SelectCollab {
            id_eq: Some(id),
            ..Default::default()
        })
    }

    async fn db_to_graphql(_context: &Context, d: models::Collab) -> AsamiResult<Self> {
        Ok(Collab {
            id: d.attrs.id,
            campaign_id: d.attrs.campaign_id,
            advertiser_id: hex_to_i32(&d.attrs.advertiser_id)?,
            handle_id: d.attrs.handle_id,
            member_id: hex_to_i32(&d.attrs.member_id)?,
            status: d.attrs.status,
            dispute_reason: d.attrs.dispute_reason,
            reward: d.attrs.reward,
            fee: d.attrs.fee,
            collab_trigger_unique_id: d.attrs.collab_trigger_unique_id,
        })
    }
}
