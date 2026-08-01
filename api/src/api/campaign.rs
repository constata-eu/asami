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
    account_id: i32,
    #[graphql(description = "The total budget for this campaign to be collected by users.")]
    budget: String,
    kind: CampaignKind,
    max_individual_reward: String,
    advertiser_name: String,
    #[graphql(description = "If the campaign is published and ready to receive collaborations.")]
    is_published: bool,
    #[graphql(description = "Whether members should treat the campaign as already closed for participation.")]
    is_ended: bool,
    #[graphql(description = "Auxiliary data related to this campaign's briefing")]
    briefing_json: String,
    #[graphql(description = "Auxiliary data related to this campaign's briefing")]
    briefing_hash: String,
    #[graphql(description = "The campaign expiration date, after which funds may be returned")]
    valid_until: Option<UtcDateTime>,
    #[graphql(description = "The date in which this campaign was created.")]
    created_at: UtcDateTime,
    #[graphql(description = "The topic ids this campaign is restricted to.")]
    topic_ids: Vec<i32>,
    #[graphql(description = "The reward you would receive. None means it does not apply.")]
    you_would_receive: Option<String>,
    #[graphql(description = "How many collabs did the campaign get")]
    total_collabs: i32,
    #[graphql(description = "How much the campaign has spent so far in rewards")]
    total_spent: String,
    #[graphql(description = "The campaign total budget: remaining + spent")]
    total_budget: String,
    #[graphql(description = "Campaign only available with users with a thumbs up from advertiser")]
    thumbs_up_only: bool,
    #[graphql(description = "These fields are private to the campaign's owner and for internal use only. ")]
    private_fields: Option<PrivateFields>,
}

#[derive(Debug, GraphQLObject, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[graphql(description = "Private fields for the campaign owner only")]
pub struct PrivateFields {
    status: CampaignStatus,
}

#[derive(Debug, Clone, Default, GraphQLInputObject, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CampaignFilter {
    ids: Option<Vec<i32>>,
    id_eq: Option<i32>,
    account_id_eq: Option<i32>,
    budget_gt: Option<String>,
    budget_lt: Option<String>,
    budget_eq: Option<String>,
    briefing_hash_like: Option<String>,
    briefing_json_like: Option<String>,
    status_ne: Option<CampaignStatus>,
    status_eq: Option<CampaignStatus>,
    kind_eq: Option<CampaignKind>,
    available_to_account_id: Option<i32>,
    is_published_eq: Option<bool>,
}

async fn make_available_to_account_id_filter(context: &Context, account_id: i32) -> FieldResult<CampaignFilter> {
    let offers = context
        .app
        .account()
        .find(&i32_to_hex(account_id))
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
            "budget" => Some(CampaignOrderBy::Budget),
            "totalCollabs" => Some(CampaignOrderBy::TotalCollabs),
            "totalSpent" => Some(CampaignOrderBy::TotalSpent),
            "totalBudget" => Some(CampaignOrderBy::TotalBudget),
            "validUntil" => Some(CampaignOrderBy::ValidUntil),
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
        if let Some(account_id) = filter.as_ref().and_then(|f| f.available_to_account_id) {
            let id_filter = make_available_to_account_id_filter(context, account_id).await?;
            Self::base_collection(context, page, per_page, sort_field, sort_order, Some(id_filter)).await
        } else {
            Self::base_collection(context, page, per_page, sort_field, sort_order, filter).await
        }
    }

    async fn count(context: &Context, filter: Option<CampaignFilter>) -> FieldResult<ListMetadata> {
        if let Some(account_ids) = filter.as_ref().and_then(|f| f.available_to_account_id) {
            let ids_filter = make_available_to_account_id_filter(context, account_ids).await?;
            Self::base_count(context, Some(ids_filter)).await
        } else {
            Self::base_count(context, filter).await
        }
    }

    fn filter_to_select(context: &Context, filter: Option<CampaignFilter>) -> FieldResult<models::SelectCampaign> {
        if let Some(f) = filter {
            // Managed by admin filter requires a logged in user, and implicitly filters
            // only the current user campaigns.
            let private_filters = f.status_eq.is_some() || f.status_ne.is_some();

            let account_id_eq = if private_filters {
                Some(context.account_id()?)
            } else {
                f.account_id_eq.map(i32_to_hex)
            };

            let (status_eq, status_ne) = match f.is_published_eq {
                Some(true) => (Some(CampaignStatus::Published), None),
                Some(false) => (None, Some(CampaignStatus::Published)),
                _ => (f.status_eq, f.status_ne),
            };

            Ok(models::SelectCampaign {
                id_in: f.ids,
                account_id_eq,
                id_eq: f.id_eq,
                budget_gt: f.budget_gt,
                budget_lt: f.budget_lt,
                budget_eq: f.budget_eq,
                status_eq,
                status_ne,
                kind_eq: f.kind_eq,
                briefing_json_like: into_like_search(f.briefing_json_like),
                briefing_hash_like: into_like_search(f.briefing_hash_like),
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
        let advertiser_name = d.account().await?.attrs.name;
        let you_would_receive = match context.current_session {
            Some(_) => d.reward_for_account(&context.account().await?).await?.map(|x| x.encode_hex()),
            None => None,
        };
        let budget = d.available_budget().await.unwrap_or(d.budget_u256()).encode_hex();

        let is_published = d.attrs.status == CampaignStatus::Published;
        let is_ended = d.attrs.collabs_gathered_at.is_some() || !d.is_visible_to_members();

        let private_fields = if context.maybe_account_id().map(|i| i == d.account_id()).unwrap_or(false) {
            Some(PrivateFields {
                status: d.attrs.status,
            })
        } else {
            None
        };

        Ok(Campaign {
            id: d.attrs.id,
            account_id: hex_to_i32(&d.attrs.account_id)?,
            budget,
            kind: d.attrs.kind,
            max_individual_reward: d.attrs.max_individual_reward,
            advertiser_name,
            valid_until: d.attrs.valid_until,
            briefing_json: d.attrs.briefing_json,
            briefing_hash: d.attrs.briefing_hash,
            created_at: d.attrs.created_at,
            topic_ids,
            you_would_receive,
            total_collabs: d.attrs.total_collabs,
            total_spent: d.attrs.total_spent,
            total_budget: d.attrs.total_budget,
            thumbs_up_only: d.attrs.thumbs_up_only,
            is_published,
            is_ended,
            private_fields,
        })
    }
}
