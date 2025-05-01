use super::{
    models::{self, *},
    *,
};

#[derive(Debug, GraphQLObject, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[graphql(description = "Tracks one instance of the process to score a given handle")]
pub struct HandleScoring {
    id: i32,
    handle_id: i32,
    status: HandleScoringStatus,
    post_count: i32,

    impression_count: i32,
    ghost_account: bool,
    repost_fatigue: bool,
    indeterminate_audience: bool,
    followed: bool,
    liked: bool,
    replied: bool,
    reposted: bool,
    mentioned: bool,

    online_engagement_score: EngagementScore,
    online_engagement_override: Option<EngagementScore>,
    online_engagement_override_reason: Option<String>,

    offline_engagement_score: EngagementScore,
    offline_engagement_description: Option<String>,

    poll_id: Option<String>,
    poll_score: Option<PollScore>,
    poll_override: Option<PollScore>,
    poll_override_reason: Option<String>,

    operational_status_score: OperationalStatus,
    operational_status_override: Option<OperationalStatus>,
    operational_status_override_reason: Option<String>,

    referrer_score: bool,
    referrer_score_override: Option<bool>,
    referrer_score_override_reason: Option<String>,

    holder_score: bool,
    holder_score_override: Option<bool>,
    holder_score_override_reason: Option<String>,

    authority: i32,
    audience_size: i32,
    audience_size_override: Option<i32>,
    audience_size_override_reason: Option<String>,

    score: Option<String>,
}

#[derive(Debug, Clone, Default, GraphQLInputObject, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HandleScoringFilter {
    ids: Option<Vec<i32>>,
    id_eq: Option<i32>,
}

#[rocket::async_trait]
impl Showable<models::HandleScoring, HandleScoringFilter> for HandleScoring {
    fn sort_field_to_order_by(field: &str) -> Option<models::HandleScoringOrderBy> {
        match field {
            "id" => Some(HandleOrderBy::Id),
            _ => None,
        }
    }

    fn filter_to_select(_context: &Context, filter: Option<HandleScoringFilter>) -> FieldResult<models::SelectHandleScoring> {
        if let Some(f) = filter {
            Ok(models::SelectHandleScoring {
                id_in: f.ids,
                id_eq: f.id_eq,
                ..Default::default()
            })
        } else {
            Ok(Default::default())
        }
    }

    fn select_by_id(_context: &Context, id: i32) -> FieldResult<models::SelectHandleScoring> {
        Ok(models::SelectHandleScoring {
            id_eq: Some(id),
            ..Default::default()
        })
    }

    async fn db_to_graphql(_context: &Context, model: models::HandleScoring) -> AsamiResult<Self> {
        let d = model.attrs;

        Ok(HandleScoring {
            id: d.id,
            handle_id: d.handle_id,
            status: d.status,
            post_count: d.post_count,
            impression_count: d.impression_count,
            ghost_account: d.ghost_account,
            repost_fatigue: d.repost_fatigue,
            indeterminate_audience: d.indeterminate_audience,
            followed: d.followed,
            liked: d.liked,
            replied: d.replied,
            reposted: d.reposted,
            mentioned: d.mentioned,
            online_engagement_score: d.online_engagement_score,
            online_engagement_override: d.online_engagement_override,
            online_engagement_override_reason: d.online_engagement_override_reason,
            offline_engagement_score: d.offline_engagement_score,
            offline_engagement_description: d.offline_engagement_description,
            poll_id: d.poll_id,
            poll_score: d.poll_score,
            poll_override: d.poll_override,
            poll_override_reason: d.poll_override_reason,
            operational_status_score: d.operational_status_score,
            operational_status_override: d.operational_status_override,
            operational_status_override_reason: d.operational_status_override_reason,
            referrer_score: d.referrer_score,
            referrer_score_override: d.referrer_score_override,
            referrer_score_override_reason: d.referrer_score_override_reason,
            holder_score: d.holder_score,
            holder_score_override: d.holder_score_override,
            holder_score_override_reason: d.holder_score_override_reason,
            authority: d.authority,
            audience_size: d.audience_size,
            audience_size_override: d.audience_size_override,
            audience_size_override_reason: d.audience_size_override_reason,
            score: d.score,
        })
    }
}
