use super::{*, models::{self, *, Decimal}};
use rust_decimal::prelude::FromPrimitive;

#[derive(Debug, GraphQLObject, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[graphql(description = "A campaign started by an advertiser")]
pub struct Campaign {
  #[graphql(description = "Unique numeric identifier of this werify point")]
  id: i32,
  #[graphql(description = "The id of the account that created this.")]
  account_id: i32,
  #[graphql(description = "The total budget for this campaign to be split across users.")]
  budget: f64,
  #[graphql(description = "The site where this campaign is to be run on.")]
  site: Site,
  #[graphql(description = "The content to share.")]
  content: String,
  #[graphql(description = "The date in which this campaign was created.")]
  created_at: UtcDateTime,
  #[graphql(description = "The last time this campaign received an update.")]
  updated_at: Option<UtcDateTime>,
}

#[derive(Debug, Clone, Default, GraphQLInputObject, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CampaignFilter {
  ids: Option<Vec<i32>>,
  id_eq: Option<i32>,
  content_like: Option<String>,
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

  fn filter_to_select(context: &Context, filter: Option<CampaignFilter>) -> models::SelectCampaign {
    if let Some(f) = filter {
      models::SelectCampaign {
        id_in: f.ids,
        account_id_in: Some(context.account_ids.clone()),
        id_eq: f.id_eq,
        content_like: into_like_search(f.content_like),
        deletion_id_is_set: Some(false),
        ..Default::default()
      }
    } else {
      models::SelectCampaign {
        account_id_in: Some(context.account_ids.clone()),
        deletion_id_is_set: Some(false),
        ..Default::default()
      }
    }
  }

  fn select_by_id(context: &Context, id: i32) -> models::SelectCampaign {
    models::SelectCampaign { id_eq: Some(id), account_id_in: Some(context.account_ids.clone()), ..Default::default() }
  }

  async fn db_to_graphql(d: models::Campaign) -> AsamiResult<Self> {
    Ok(Campaign {
      id: d.attrs.id,
      account_id: d.attrs.account_id,
      budget: d.attrs.budget.to_f64().unwrap_or(0.0),
      site: d.attrs.site,
      content: d.attrs.content,
      created_at: d.attrs.created_at,
      updated_at:d.attrs.updated_at,
    })
  }
}

#[derive(Clone, GraphQLInputObject, Serialize, Deserialize)]
#[graphql(description = "The input for creating a new Campaign.")]
#[serde(rename_all = "camelCase")]
pub struct CreateCampaignInput {
  pub content: String,
  pub budget: f64,
  pub account_id: i32,
  pub site: Site,
}

impl CreateCampaignInput {
  pub async fn process(self, context: &Context) -> FieldResult<Campaign> {
    context.require_account_user(self.account_id)?;

    let campaign = context.app.campaign().insert(models::InsertCampaign{
      account_id: self.account_id,
      budget: Decimal::from_f64(self.budget).unwrap_or(Decimal::ZERO),
      site: self.site,
      content: self.content,
    }).save().await?;
    Ok(Campaign::db_to_graphql(campaign).await?)
  }
}
