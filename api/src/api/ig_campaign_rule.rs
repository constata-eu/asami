use super::{
  models::{self, *},
  *,
};

#[derive(Debug, GraphQLObject, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[graphql(description = "Rules for posting a campaign")]
pub struct IgCampaignRule {
  #[graphql(description = "Unique Id for this campaign rules")]
  id: i32,
  #[graphql(description = "The campaign this rules belong to")]
  campaign_id: String,
  #[graphql(description = "The base64 encoded image itself.")]
  image: String,
  #[graphql(description = "The caption the user must post for this image.")]
  caption: String,
}

#[derive(Debug, Clone, Default, GraphQLInputObject, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IgCampaignRuleFilter {
  ids: Option<Vec<i32>>,
  id_eq: Option<i32>,
  campaign_id_eq: Option<String>,
}

#[rocket::async_trait]
impl Showable<models::IgCampaignRule, IgCampaignRuleFilter> for IgCampaignRule {
  fn sort_field_to_order_by(field: &str) -> Option<models::IgCampaignRuleOrderBy> {
    match field {
      "id" => Some(IgCampaignRuleOrderBy::Id),
      _ => None,
    }
  }

  fn filter_to_select(
    _context: &Context,
    filter: Option<IgCampaignRuleFilter>,
  ) -> FieldResult<models::SelectIgCampaignRule> {
    if let Some(f) = filter {
      Ok(models::SelectIgCampaignRule {
        id_in: f.ids,
        campaign_id_eq: f.campaign_id_eq,
        id_eq: f.id_eq,
        ..Default::default()
      })
    } else {
      Ok(Default::default())
    }
  }

  fn select_by_id(_context: &Context, id: i32) -> FieldResult<models::SelectIgCampaignRule> {
    Ok(models::SelectIgCampaignRule {
      id_eq: Some(id),
      ..Default::default()
    })
  }

  async fn db_to_graphql(d: models::IgCampaignRule) -> AsamiResult<Self> {
    use base64::{engine::general_purpose, Engine as _};

    Ok(IgCampaignRule {
      id: d.attrs.id,
      campaign_id: d.attrs.campaign_id,
      image: general_purpose::STANDARD.encode(d.attrs.image),
      caption: d.attrs.caption,
    })
  }
}
