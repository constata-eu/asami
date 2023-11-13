use super::{*, models::{self, *}};

#[derive(Debug, GraphQLObject, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[graphql(description = "A campaign started by an advertiser")]
pub struct Campaign {
  #[graphql(description = "Unique numeric identifier of this resource")]
  id: String,
  #[graphql(description = "The id of the account that created this.")]
  account_id: String,
  #[graphql(description = "The total budget for this campaign to be split across users.")]
  budget: String,
  #[graphql(description = "The amount remaining from the given budget.")]
  remaining: String,
  #[graphql(description = "The campaign is finished when it spends all its budget, or when the remaining amount is refunded to the advertiser")]
  finished: bool,
  #[graphql(description = "Unspent budget can be reimbursed after this date.")]
  valid_until: UtcDateTime,
  #[graphql(description = "The site where this campaign is to be run on.")]
  site: Site,
  #[graphql(description = "The content to share.")]
  content_id: String,
  #[graphql(description = "The date in which this campaign was created.")]
  created_at: UtcDateTime,
  #[graphql(description = "The last time this campaign received an update.")]
  updated_at: Option<UtcDateTime>,
}

#[derive(Debug, Clone, Default, GraphQLInputObject, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CampaignFilter {
  ids: Option<Vec<String>>,
  id_eq: Option<String>,
  account_id_in: Option<Vec<String>>,
  finished_eq: Option<bool>,
  content_id_like: Option<String>,
}

#[rocket::async_trait]
impl Showable<models::Campaign, CampaignFilter> for Campaign {
  fn sort_field_to_order_by(field: &str) -> Option<models::CampaignOrderBy> {
    match field {
      "id" => Some(CampaignOrderBy::Id),
      "createdAt" => Some(CampaignOrderBy::CreatedAt),
      "updatedAt" => Some(CampaignOrderBy::UpdatedAt),
      _ => None,
    }
  }

  fn filter_to_select(_context: &Context, filter: Option<CampaignFilter>) -> models::SelectCampaign {
    if let Some(f) = filter {
      models::SelectCampaign {
        id_in: f.ids,
        account_id_in: f.account_id_in,
        id_eq: f.id_eq,
        finished_eq: f.finished_eq,
        content_id_like: into_like_search(f.content_id_like),
        ..Default::default()
      }
    } else {
      Default::default()
    }
  }

  fn select_by_id(_context: &Context, id: String) -> models::SelectCampaign {
    models::SelectCampaign { id_eq: Some(id), ..Default::default() }
  }

  async fn db_to_graphql(d: models::Campaign) -> AsamiResult<Self> {
    Ok(Campaign {
      id: d.attrs.id,
      account_id: d.attrs.account_id,
      budget: d.attrs.budget,
      remaining: d.attrs.remaining,
      finished: d.attrs.finished,
      valid_until: d.attrs.valid_until,
      site: d.attrs.site,
      content_id: d.attrs.content_id,
      created_at: d.attrs.created_at,
      updated_at:d.attrs.updated_at,
    })
  }
}
