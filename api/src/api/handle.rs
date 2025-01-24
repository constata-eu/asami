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
    user_id: Option<String>,
    #[graphql(description = "The score given to this handle by Asami's admin.")]
    score: Option<String>,
    #[graphql(description = "Topics assigned to this handle")]
    topic_ids: Vec<i32>,
    #[graphql(description = "Status of this handle.")]
    status: HandleStatus,
    #[graphql(description = "Collabs made")]
    total_collabs: i32,
    #[graphql(description = "Rewards from collabs made")]
    total_collab_rewards: String,
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
                username_like: into_like_search(f.username_like),
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
            status: d.attrs.status,
            total_collabs: d.attrs.total_collabs,
            total_collab_rewards: d.attrs.total_collab_rewards,
        })
    }
}

#[derive(Clone, GraphQLInputObject, Serialize, Deserialize)]
#[graphql(description = "The input for creating a new Handle.")]
#[serde(rename_all = "camelCase")]
pub struct CreateHandleInput {
    pub username: String,
}

impl CreateHandleInput {
    pub async fn process(self, context: &Context) -> FieldResult<Handle> {
        let req = context.account().await?.create_handle(&self.username).await?;

        Ok(Handle::db_to_graphql(context, req).await?)
    }
}
