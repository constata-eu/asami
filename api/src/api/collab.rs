use super::{
  models::{self, *},
  *,
};

#[derive(Debug, GraphQLObject, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[graphql(description = "A collab is created when a member reposts a campaign's content.")]
pub struct Collab {
  #[graphql(description = "Unique numeric identifier of this resource in the smart contract.")]
  id: String,
  #[graphql(description = "The campaign whose content was reposted.")]
  campaign_id: String,
  #[graphql(description = "The person that created the campaign.")]
  advertiser_id: String,
  #[graphql(description = "The handle that reposted the content.")]
  handle_id: String,
  #[graphql(description = "The member who owns the handle.")]
  member_id: String,
  #[graphql(description = "The gross amount paid by the advertiser (campaign creator) for this collab.")]
  gross: String,
  #[graphql(description = "The fee deducted by asami from the gross amount.")]
  fee: String,
}

#[derive(Debug, Clone, Default, GraphQLInputObject, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollabFilter {
  ids: Option<Vec<String>>,
  id_eq: Option<String>,
  campaign_id_eq: Option<String>,
  campaign_id_in: Option<Vec<String>>,
  advertiser_id_eq: Option<String>,
  advertiser_id_in: Option<Vec<String>>,
  handle_id_eq: Option<String>,
  handle_id_in: Option<Vec<String>>,
  member_id_eq: Option<String>,
  member_id_in: Option<Vec<String>>,
}

#[rocket::async_trait]
impl Showable<models::Collab, CollabFilter> for Collab {
  fn sort_field_to_order_by(field: &str) -> Option<models::CollabOrderBy> {
    match field {
      "id" => Some(CollabOrderBy::Id),
      _ => None,
    }
  }

  fn filter_to_select(_context: &Context, filter: Option<CollabFilter>) -> FieldResult<models::SelectCollab> {
    if let Some(f) = filter {
      Ok(models::SelectCollab {
        id_in: f.ids,
        campaign_id_eq: f.campaign_id_eq,
        campaign_id_in: f.campaign_id_in,
        advertiser_id_eq: f.advertiser_id_eq,
        advertiser_id_in: f.advertiser_id_in,
        handle_id_eq: f.handle_id_eq,
        handle_id_in: f.handle_id_in,
        member_id_in: f.member_id_in,
        member_id_eq: f.member_id_eq,
        id_eq: f.id_eq,
        ..Default::default()
      })
    } else {
      Ok(Default::default())
    }
  }

  fn select_by_id(_context: &Context, id: String) -> FieldResult<models::SelectCollab> {
    Ok(models::SelectCollab {
      id_eq: Some(id),
      ..Default::default()
    })
  }

  async fn db_to_graphql(d: models::Collab) -> AsamiResult<Self> {
    Ok(Collab {
      id: d.attrs.id,
      campaign_id: d.attrs.campaign_id,
      advertiser_id: d.attrs.advertiser_id,
      handle_id: d.attrs.handle_id,
      member_id: d.attrs.member_id,
      gross: d.attrs.gross,
      fee: d.attrs.fee,
    })
  }
}
