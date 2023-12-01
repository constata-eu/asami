use super::{*, models::{self, *}};

#[derive(Debug, GraphQLObject, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[graphql(description = "A request to verify a handle for an account.")]
pub struct HandleUpdateRequest {
  #[graphql(description = "Unique numeric identifier of this resource")]
  id: i32,
  #[graphql(description = "The id of the account that created this.")]
  account_id: String,
  #[graphql(description = "The numeric id of the handle in the smart contract. Hex encoded uint256.")]
  handle_id: String,
  #[graphql(description = "The username on the given social network. This may change by the user, it may not be a unique id.")]
  username: Option<String>,
  #[graphql(description = "The price for each collab made with this handle. This is the price for a single repost.")]
  price: Option<String>,
  #[graphql(description = "The score given to this handle by Asami's admin.")]
  score: Option<String>,
  #[graphql(description = "If this request was created by the admin or by the user themselves")]
  created_by_admin: bool,
  #[graphql(description = "Status of this campaign request.")]
  status: HandleUpdateRequestStatus,
}

#[derive(Debug, Clone, Default, GraphQLInputObject, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HandleUpdateRequestFilter {
  ids: Option<Vec<i32>>,
  id_eq: Option<i32>,
  handle_id_eq: Option<String>,
  username_like: Option<String>,
  created_by_admin_eq: Option<bool>,
  status_in: Option<Vec<HandleUpdateRequestStatus>>,
}

#[rocket::async_trait]
impl Showable<models::HandleUpdateRequest, HandleUpdateRequestFilter> for HandleUpdateRequest {
  fn sort_field_to_order_by(field: &str) -> Option<models::HandleUpdateRequestOrderBy> {
    match field {
      "id" => Some(HandleUpdateRequestOrderBy::Id),
      _ => None,
    }
  }

  fn filter_to_select(context: &Context, filter: Option<HandleUpdateRequestFilter>) -> models::SelectHandleUpdateRequest {
    if let Some(f) = filter {
      models::SelectHandleUpdateRequest {
        id_in: f.ids,
        account_id_eq: Some(context.account_id().to_string()),
        status_in: f.status_in,
        id_eq: f.id_eq,
        handle_id_eq: f.handle_id_eq,
        created_by_admin_eq: f.created_by_admin_eq,
        username_like: into_like_search(f.username_like),
        ..Default::default()
      }
    } else {
      models::SelectHandleUpdateRequest {
        account_id_eq: Some(context.account_id().to_string()),
        ..Default::default()
      }
    }
  }

  fn select_by_id(context: &Context, id: i32) -> models::SelectHandleUpdateRequest {
    models::SelectHandleUpdateRequest { id_eq: Some(id), account_id_eq: Some(context.account_id().to_string()), ..Default::default() }
  }

  async fn db_to_graphql(d: models::HandleUpdateRequest) -> AsamiResult<Self> {
    Ok(HandleUpdateRequest {
      id: d.attrs.id,
      account_id: d.attrs.account_id,
      handle_id: d.attrs.handle_id,
      username: d.attrs.username,
      price: d.attrs.price,
      score: d.attrs.score,
      created_by_admin: d.attrs.created_by_admin,
      status: d.attrs.status,
    })
  }
}

#[derive(Clone, GraphQLInputObject, Serialize, Deserialize)]
#[graphql(description = "The input for creating a new HandleUpdateRequest.")]
#[serde(rename_all = "camelCase")]
pub struct CreateHandleUpdateRequestInput {
  pub handle_id: String,
  pub price: String,
}

impl CreateHandleUpdateRequestInput {
  pub async fn process(self, context: &Context) -> FieldResult<HandleUpdateRequest> {
    let handle = context.account().await?.handle_scope().id_eq(&self.handle_id).one().await?;

    let req = context.app.handle_update_request().insert(InsertHandleUpdateRequest{
      account_id: handle.attrs.account_id,
      handle_id: handle.attrs.id,
      username: None,
      price: Some(self.price),
      score: None,
      created_by_admin: false,
    }).save().await?;

    Ok(HandleUpdateRequest::db_to_graphql(req).await?)
  }
}
