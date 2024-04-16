use super::{
  models::{self, *},
  *,
};

#[derive(Debug, GraphQLObject, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[graphql(description = "A campaign")]
pub struct Campaign {
  #[graphql(description = "Unique numeric identifier of this resource")]
  id: i32,
  #[graphql(description = "The id of the account that created this.")]
  account_id: String,
  #[graphql(description = "The total budget for this campaign to be collected by users.")]
  budget: String,
  #[graphql(description = "The site where this campaign is to be run on.")]
  campaign_kind: CampaignKind,
  #[graphql(description = "Auxiliary data related to this campaign's briefing")]
  briefing_json: String,
  #[graphql(description = "Auxiliary data related to this campaign's briefing")]
  briefing_hash: String,
  #[graphql(description = "The date in which this campaign was created.")]
  created_at: UtcDateTime,
  #[graphql(description = "The topic ids this campaign is restricted to.")]
  topic_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, GraphQLInputObject, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CampaignFilter {
  ids: Option<Vec<String>>,
  id_eq: Option<String>,
  account_id_eq: Option<String>,
  budget_gt: Option<String>,
  budget_lt: Option<String>,
  budget_eq: Option<String>,
  briefing_hash_eq: Option<String>,
  available_to_account_id: Option<String>,
}

async fn make_available_to_account_id_filter(context: &Context, account_id: String) -> FieldResult<CampaignFilter> {
  let offers = context
    .app
    .account()
    .find(account_id)
    .await?
    .campaign_offers()
    .await?
    .into_iter()
    .map(|x| x.attrs.id)
    .collect();
  Ok(CampaignFilter {
    finished_eq: Some(false),
    ids: Some(offers),
    ..Default::default()
  })
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

  async fn collection(
    context: &Context,
    page: Option<i32>,
    per_page: Option<i32>,
    sort_field: Option<String>,
    sort_order: Option<String>,
    filter: Option<CampaignFilter>,
  ) -> FieldResult<Vec<Self>> {
    if let Some(account_id) = filter.as_ref().and_then(|f| f.available_to_account_id.clone()) {
      let id_filter = make_available_to_account_id_filter(context, account_id).await?;
      Self::base_collection(context, page, per_page, sort_field, sort_order, Some(id_filter)).await
    } else {
      Self::base_collection(context, page, per_page, sort_field, sort_order, filter).await
    }
  }

  async fn count(context: &Context, filter: Option<CampaignFilter>) -> FieldResult<ListMetadata> {
    if let Some(account_ids) = filter.as_ref().and_then(|f| f.available_to_account_id.clone()) {
      let ids_filter = make_available_to_account_id_filter(context, account_ids).await?;
      Self::base_count(context, Some(ids_filter)).await
    } else {
      Self::base_count(context, filter).await
    }
  }

  fn filter_to_select(_context: &Context, filter: Option<CampaignFilter>) -> FieldResult<models::SelectCampaign> {
    if let Some(f) = filter {
      Ok(models::SelectCampaign {
        id_in: f.ids,
        account_id_eq: f.account_id_eq,
        id_eq: f.id_eq,
        finished_eq: f.finished_eq,
        remaining_gt: f.remaining_gt,
        content_id_like: into_like_search(f.content_id_like),
        ..Default::default()
      })
    } else {
      Ok(Default::default())
    }
  }

  fn select_by_id(_context: &Context, id: String) -> FieldResult<models::SelectCampaign> {
    Ok(models::SelectCampaign {
      id_eq: Some(id),
      ..Default::default()
    })
  }

  async fn db_to_graphql(d: models::Campaign) -> AsamiResult<Self> {
    Ok(Campaign {
      id: d.attrs.id,
      account_id: d.attrs.account_id,
      budget: d.attrs.budget,
      price_score_ratio: d.attrs.price_score_ratio,
      finished: d.attrs.finished,
      valid_until: d.attrs.valid_until,
      site: d.attrs.site,
      content_id: d.attrs.content_id,
      created_at: d.attrs.created_at,
      updated_at: d.attrs.updated_at,
    })
  }
}

#[derive(Clone, GraphQLInputObject, Serialize, Deserialize)]
#[graphql(description = "The input for creating a new CampaignRequest.")]
#[serde(rename_all = "camelCase")]
pub struct CreateCampaignFromLinkInput {
  pub link: String,
  pub valid_until: UtcDateTime,
  pub topic_ids: Vec<String>,
}

impl CreateCampaignFromLinkInput {
  pub async fn process(self, context: &Context) -> FieldResult<Campaign> {
    let topics = context.app.topic().select().id_in(&self.topic_ids).all().await?;

    let req = context.account().await?
      .create_campaign(
        self.site,
        &self.content_id,
        u256(self.budget),
        u256(self.price_score_ratio),
        self.valid_until,
        &topics,
      )
      .await?;

    Ok(CampaignRequest::db_to_graphql(req).await?)
  }
}
