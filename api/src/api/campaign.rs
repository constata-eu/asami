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
    #[graphql(description = "The kind of campaign, what's expected from the member.")]
    campaign_kind: CampaignKind,
    #[graphql(description = "Auxiliary data related to this campaign's briefing")]
    briefing_json: String,
    #[graphql(description = "Auxiliary data related to this campaign's briefing")]
    briefing_hash: String,
    #[graphql(description = "The campaign expiration date, after which funds may be returned")]
    valid_until: Option<UtcDateTime>,
    #[graphql(description = "The on-chain publication status of this campaign.")]
    status: CampaignStatus,
    #[graphql(description = "The date in which this campaign was created.")]
    created_at: UtcDateTime,
    #[graphql(description = "The topic ids this campaign is restricted to.")]
    topic_ids: Vec<i32>,
    #[graphql(description = "The reward you would receive. None means it does not apply.")]
    you_would_receive: Option<String>,
}

#[derive(Debug, Clone, Default, GraphQLInputObject, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CampaignFilter {
    ids: Option<Vec<i32>>,
    id_eq: Option<i32>,
    account_id_eq: Option<String>,
    budget_gt: Option<String>,
    budget_lt: Option<String>,
    budget_eq: Option<String>,
    briefing_hash_eq: Option<String>,
    briefing_json_like: Option<String>,
    status_ne: Option<CampaignStatus>,
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
                budget_gt: f.budget_gt,
                status_ne: f.status_ne,
                briefing_json_like: into_like_search(f.briefing_json_like),
                ..Default::default()
            })
        } else {
            Ok(Default::default())
        }
    }

    fn select_by_id(_context: &Context, id: i32) -> FieldResult<models::SelectCampaign> {
        Ok(models::SelectCampaign {
            id_eq: Some(id),
            ..Default::default()
        })
    }
    async fn db_to_graphql(context: &Context, d: models::Campaign) -> AsamiResult<Self> {
        let topic_ids = d.topic_ids().await?;
        let you_would_receive = match context.current_session {
            Some(_) => d.reward_for_account(&context.account().await?).await?.map(|x| x.encode_hex()),
            None => None 
        };
        Ok(Campaign {
            id: d.attrs.id,
            account_id: d.attrs.account_id,
            budget: d.attrs.budget,
            valid_until: d.attrs.valid_until,
            campaign_kind: d.attrs.campaign_kind,
            briefing_json: d.attrs.briefing_json,
            briefing_hash: d.attrs.briefing_hash,
            created_at: d.attrs.created_at,
            status: d.attrs.status,
            topic_ids,
            you_would_receive,
        })
    }
}

#[derive(Clone, GraphQLInputObject, Serialize, Deserialize)]
#[graphql(description = "The input for creating a new CampaignRequest.")]
#[serde(rename_all = "camelCase")]
pub struct CreateCampaignFromLinkInput {
    pub link: String,
    pub topic_ids: Vec<i32>,
}

impl CreateCampaignFromLinkInput {
    pub async fn process(self, context: &Context) -> FieldResult<Campaign> {
        let topics = context.app.topic().select().id_in(&self.topic_ids).all().await?;
        let campaign = context.app.campaign().create_from_link(&context.account().await?, &self.link, &topics).await?;

        Ok(Campaign::db_to_graphql(context,campaign).await?)
    }
}
