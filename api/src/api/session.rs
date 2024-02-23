use super::{
  models::{self, *},
  *,
};

#[derive(Debug, GraphQLObject, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[graphql(description = "A campaign started by an advertiser")]
pub struct Session {
  #[graphql(description = "The pubkey ID of this session.")]
  id: String,
  #[graphql(description = "The user associated to this session.")]
  user_id: i32,
  #[graphql(description = "The account IDS associated with this user.")]
  account_id: String,
  #[graphql(description = "The pubkey associated to this session.")]
  pubkey: String,
  #[graphql(description = "The content to share.")]
  nonce: String,
  #[graphql(description = "The date in which this session was created.")]
  created_at: UtcDateTime,
  #[graphql(description = "The last time this session was updated.")]
  updated_at: Option<UtcDateTime>,
}

#[derive(Debug, Clone, Default, GraphQLInputObject, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionFilter {
  ids: Option<Vec<String>>,
  id_eq: Option<String>,
  pubkey_eq: Option<String>,
}

#[rocket::async_trait]
impl Showable<models::Session, SessionFilter> for Session {
  fn sort_field_to_order_by(field: &str) -> Option<models::SessionOrderBy> {
    match field {
      "id" => Some(SessionOrderBy::Id),
      "createdAt" => Some(SessionOrderBy::CreatedAt),
      "updatedAt" => Some(SessionOrderBy::UpdatedAt),
      _ => None,
    }
  }

  fn filter_to_select(context: &Context, filter: Option<SessionFilter>) -> FieldResult<models::SelectSession> {
    if let Some(f) = filter {
      Ok(models::SelectSession {
        id_in: f.ids,
        user_id_eq: Some(context.user_id()?),
        id_eq: f.id_eq,
        pubkey_eq: f.pubkey_eq,
        ..Default::default()
      })
    } else {
      Ok(models::SelectSession {
        user_id_eq: Some(context.user_id()?),
        ..Default::default()
      })
    }
  }

  fn select_by_id(context: &Context, id: String) -> FieldResult<models::SelectSession> {
    Ok(models::SelectSession {
      id_eq: Some(id),
      user_id_eq: Some(context.user_id()?),
      ..Default::default()
    })
  }

  async fn db_to_graphql(d: models::Session) -> AsamiResult<Self> {
    Ok(Session {
      id: d.attrs.id,
      user_id: d.attrs.user_id,
      account_id: d.attrs.account_id,
      pubkey: d.attrs.pubkey,
      nonce: d.attrs.nonce.to_string(),
      created_at: d.attrs.created_at,
      updated_at: d.attrs.updated_at,
    })
  }
}
