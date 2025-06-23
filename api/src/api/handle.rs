use super::{
    models::{self, *},
    *,
};

#[derive(Debug, GraphQLObject, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[graphql(description = "A request to verify a handle for an account.")]
pub struct Handle {
    #[graphql(description = "Unique numeric identifier of this resource")]
    id: i32,
    #[graphql(description = "The id of the account that made the request.")]
    account_id: i32,
    #[graphql(
        description = "The username on the given social network. This may change by the user, it may not be a unique id."
    )]
    username: String,
    #[graphql(description = "The unique user_id in the given social network. This never changes.")]
    user_id: String,

    #[graphql(description = "The score given to this handle by Asami's admin.")]
    score: Option<String>,
    #[graphql(description = "The last scoring successfully applied to this handle.")]
    current_scoring_id: Option<i32>,
    #[graphql(description = "Date in which the last scoring took place.")]
    last_scoring: Option<DateTime<Utc>>,
    #[graphql(description = "Date in which a new scoring will happen.")]
    next_scoring: Option<DateTime<Utc>>,

    #[graphql(description = "Topics assigned to this handle")]
    topic_ids: Vec<i32>,

    #[graphql(description = "Status of this handle.")]
    status: HandleStatus,
    #[graphql(description = "Collabs made")]
    total_collabs: i32,
    #[graphql(description = "Rewards from collabs made")]
    total_collab_rewards: String,

    online_engagement_override: Option<EngagementScore>,
    online_engagement_override_reason: Option<String>,
    offline_engagement_score: EngagementScore,
    offline_engagement_description: Option<String>,
    poll_override: Option<PollScore>,
    poll_override_reason: Option<String>,
    operational_status_override: Option<OperationalStatus>,
    operational_status_override_reason: Option<String>,
    referrer_score_override: Option<bool>,
    referrer_score_override_reason: Option<String>,
    holder_score_override: Option<bool>,
    holder_score_override_reason: Option<String>,
    audience_size_override: Option<i32>,
    audience_size_override_reason: Option<String>,
}

#[derive(Debug, Clone, Default, GraphQLInputObject, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HandleFilter {
    ids: Option<Vec<i32>>,
    id_eq: Option<i32>,
    username_like: Option<String>,
    status_in: Option<Vec<HandleStatus>>,
    user_id_like: Option<String>,
    account_id_eq: Option<i32>,
}

#[rocket::async_trait]
impl Showable<models::Handle, HandleFilter> for Handle {
    fn sort_field_to_order_by(field: &str) -> Option<models::HandleOrderBy> {
        match field {
            "id" => Some(HandleOrderBy::Id),
            "score" => Some(HandleOrderBy::Score),
            "totalCollabs" => Some(HandleOrderBy::TotalCollabs),
            "totalCollabRewards" => Some(HandleOrderBy::TotalCollabRewards),
            _ => None,
        }
    }

    fn filter_to_select(_context: &Context, filter: Option<HandleFilter>) -> FieldResult<models::SelectHandle> {
        if let Some(f) = filter {
            Ok(models::SelectHandle {
                id_in: f.ids,
                id_eq: f.id_eq,
                username_ilike: into_like_search(f.username_like),
                status_in: f.status_in,
                user_id_like: f.user_id_like,
                account_id_eq: f.account_id_eq.map(i32_to_hex),
                ..Default::default()
            })
        } else {
            Ok(Default::default())
        }
    }

    fn select_by_id(_context: &Context, id: i32) -> FieldResult<models::SelectHandle> {
        Ok(models::SelectHandle {
            id_eq: Some(id),
            ..Default::default()
        })
    }

    async fn db_to_graphql(_context: &Context, d: models::Handle) -> AsamiResult<Self> {
        let topic_ids = d.topic_ids().await?;
        Ok(Handle {
            id: d.attrs.id,
            account_id: hex_to_i32(&d.attrs.account_id)?,
            username: d.attrs.username,
            user_id: d.attrs.user_id,
            score: d.attrs.score,
            topic_ids,
            current_scoring_id: d.attrs.current_scoring_id,
            last_scoring: d.attrs.last_scoring,
            next_scoring: d.attrs.next_scoring,
            status: d.attrs.status,
            total_collabs: d.attrs.total_collabs,
            total_collab_rewards: d.attrs.total_collab_rewards,
            online_engagement_override: d.attrs.online_engagement_override,
            online_engagement_override_reason: d.attrs.online_engagement_override_reason,
            offline_engagement_score: d.attrs.offline_engagement_score,
            offline_engagement_description: d.attrs.offline_engagement_description,
            poll_override: d.attrs.poll_override,
            poll_override_reason: d.attrs.poll_override_reason,
            operational_status_override: d.attrs.operational_status_override,
            operational_status_override_reason: d.attrs.operational_status_override_reason,
            referrer_score_override: d.attrs.referrer_score_override,
            referrer_score_override_reason: d.attrs.referrer_score_override_reason,
            holder_score_override: d.attrs.holder_score_override,
            holder_score_override_reason: d.attrs.holder_score_override_reason,
            audience_size_override: d.attrs.audience_size_override,
            audience_size_override_reason: d.attrs.audience_size_override_reason,
        })
    }
}

#[derive(Clone, GraphQLInputObject, Serialize, Deserialize)]
#[graphql(description = "The input for creating a new CampaignRequest.")]
#[serde(rename_all = "camelCase")]
pub struct AdminEditHandleInput {
    pub topic_ids: Vec<i32>,
    pub online_engagement_override: Option<EngagementScore>,
    pub online_engagement_override_reason: Option<String>,
    pub offline_engagement_score: EngagementScore,
    pub offline_engagement_description: Option<String>,
    pub poll_override: Option<PollScore>,
    pub poll_override_reason: Option<String>,
    pub operational_status_override: Option<OperationalStatus>,
    pub operational_status_override_reason: Option<String>,
    pub referrer_score_override: Option<bool>,
    pub referrer_score_override_reason: Option<String>,
    pub holder_score_override: Option<bool>,
    pub holder_score_override_reason: Option<String>,
    pub audience_size_override: Option<i32>,
    pub audience_size_override_reason: Option<String>,
}

impl AdminEditHandleInput {
    pub async fn process(self, context: &Context, handle_id: i32) -> FieldResult<Handle> {
        let handle = context.app.handle().find(handle_id).await?;

        let topics = context.app.topic().select().id_in(&self.topic_ids).all().await?;

        for t in handle.handle_topic_vec().await? {
            t.delete().await?;
        }
        for t in topics {
            handle.add_topic(&t).await?;
        }

        let updated = handle
            .update()
            .online_engagement_override(self.online_engagement_override)
            .online_engagement_override_reason(self.online_engagement_override_reason)
            .offline_engagement_score(self.offline_engagement_score)
            .offline_engagement_description(self.offline_engagement_description)
            .poll_override(self.poll_override)
            .poll_override_reason(self.poll_override_reason)
            .operational_status_override(self.operational_status_override)
            .operational_status_override_reason(self.operational_status_override_reason)
            .referrer_score_override(self.referrer_score_override)
            .referrer_score_override_reason(self.referrer_score_override_reason)
            .holder_score_override(self.holder_score_override)
            .holder_score_override_reason(self.holder_score_override_reason)
            .audience_size_override(self.audience_size_override)
            .audience_size_override_reason(self.audience_size_override_reason)
            .next_scoring(Some(Utc::now()))
            .save()
            .await?;

        Ok(Handle::db_to_graphql(context, updated).await?)
    }
}
