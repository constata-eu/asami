use super::{
    models::{self, *},
    *,
};

#[derive(Debug, GraphQLObject, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Topic {
    #[graphql(description = "Topic ID as integer")]
    id: i32,
    #[graphql(description = "Human readable topic name, in english")]
    name: String,
}

#[derive(Debug, Clone, Default, GraphQLInputObject, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopicFilter {
    pub ids: Option<Vec<i32>>,
    pub name_like: Option<String>,
}

#[rocket::async_trait]
impl Showable<models::Topic, TopicFilter> for Topic {
    fn sort_field_to_order_by(field: &str) -> Option<models::TopicOrderBy> {
        match field {
            "id" => Some(TopicOrderBy::Id),
            _ => None,
        }
    }

    fn filter_to_select(_context: &Context, filter: Option<TopicFilter>) -> FieldResult<models::SelectTopic> {
        if let Some(f) = filter {
            Ok(models::SelectTopic {
                id_in: f.ids,
                name_like: into_like_search(f.name_like),
                ..Default::default()
            })
        } else {
            Ok(Default::default())
        }
    }

    fn select_by_id(_context: &Context, id: i32) -> FieldResult<models::SelectTopic> {
        Ok(models::SelectTopic {
            id_eq: Some(id),
            ..Default::default()
        })
    }

    async fn db_to_graphql(_context: &Context, d: models::Topic) -> AsamiResult<Self> {
        Ok(Topic {
            id: d.attrs.id,
            name: d.attrs.name,
        })
    }
}
