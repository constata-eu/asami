use super::{*, models::{self, *}};

#[derive(Debug, GraphQLObject, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[graphql(description = "A campaign started by an advertiser")]
pub struct CampaignRequest {
  #[graphql(description = "Unique numeric identifier of this resource")]
  id: i32,
  #[graphql(description = "The id of the account that created this.")]
  account_id: String,
  #[graphql(description = "The total budget for this campaign to be split across users.")]
  budget: String,
  #[graphql(description = "The site where this campaign is to be run on.")]
  site: Site,
  #[graphql(description = "Status of this campaign request.")]
  status: CampaignRequestStatus,
  #[graphql(description = "The content to share.")]
  content_id: String,
  #[graphql(description = "The date in which this campaign was created.")]
  created_at: UtcDateTime,
  #[graphql(description = "The last time this campaign received an update.")]
  updated_at: Option<UtcDateTime>,
}

#[derive(Debug, Clone, Default, GraphQLInputObject, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CampaignRequestFilter {
  ids: Option<Vec<i32>>,
  id_eq: Option<i32>,
  content_id_like: Option<String>,
  status_in: Option<Vec<CampaignRequestStatus>>,
}

#[rocket::async_trait]
impl Showable<models::CampaignRequest, CampaignRequestFilter> for CampaignRequest {
  fn sort_field_to_order_by(field: &str) -> Option<models::CampaignRequestOrderBy> {
    match field {
      "id" => Some(CampaignRequestOrderBy::Id),
      "createdAt" => Some(CampaignRequestOrderBy::CreatedAt),
      "updatedAt" => Some(CampaignRequestOrderBy::UpdatedAt),
      _ => None,
    }
  }

  fn filter_to_select(context: &Context, filter: Option<CampaignRequestFilter>) -> models::SelectCampaignRequest {
    if let Some(f) = filter {
      models::SelectCampaignRequest {
        id_in: f.ids,
        account_id_eq: Some(context.account_id().to_string()),
        status_in: f.status_in,
        id_eq: f.id_eq,
        content_id_like: into_like_search(f.content_id_like),
        ..Default::default()
      }
    } else {
      models::SelectCampaignRequest {
        account_id_eq: Some(context.account_id().to_string()),
        ..Default::default()
      }
    }
  }

  fn select_by_id(context: &Context, id: i32) -> models::SelectCampaignRequest {
    models::SelectCampaignRequest { id_eq: Some(id), account_id_eq: Some(context.account_id().to_string()), ..Default::default() }
  }

  async fn db_to_graphql(d: models::CampaignRequest) -> AsamiResult<Self> {
    Ok(CampaignRequest {
      id: d.attrs.id,
      account_id: d.attrs.account_id,
      budget: d.attrs.budget,
      status: d.attrs.status,
      site: d.attrs.site,
      content_id: d.attrs.content_id,
      created_at: d.attrs.created_at,
      updated_at:d.attrs.updated_at,
    })
  }
}

#[derive(Clone, GraphQLInputObject, Serialize, Deserialize)]
#[graphql(description = "The input for creating a new CampaignRequest.")]
#[serde(rename_all = "camelCase")]
pub struct CreateCampaignRequestInput {
  pub content_id: String,
  pub budget: String,
  pub account_id: String,
  pub site: Site,
  pub price_score_ratio: String,
  pub valid_until: UtcDateTime
}

impl CreateCampaignRequestInput {
  pub async fn process(self, context: &Context) -> FieldResult<CampaignRequest> {
    let account = context.app.account().find(&self.account_id).await?;

    let req = account.create_campaign_request(
      self.site,
      &self.content_id,
      u256(self.budget),
      u256(self.price_score_ratio),
      self.valid_until,
    ).await?;

    Ok(CampaignRequest::db_to_graphql(req).await?)
  }
}
