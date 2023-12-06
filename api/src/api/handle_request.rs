use super::{*, models::{self, *}};

#[derive(Debug, GraphQLObject, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[graphql(description = "A request to verify a handle for an account.")]
pub struct HandleRequest {
  #[graphql(description = "Unique numeric identifier of this resource")]
  id: i32,
  #[graphql(description = "The id of the account that made the request.")]
  account_id: String,
  #[graphql(description = "The social network of this handle: X, Instagram, Nostr.")]
  site: Site,
  #[graphql(description = "The username on the given social network. This may change by the user, it may not be a unique id.")]
  username: String,
  #[graphql(description = "The unique user_id in the given social network. This never changes.")]
  user_id: Option<String>,
  #[graphql(description = "The price for each collab made with this handle. This is the price for a single repost.")]
  price: Option<String>,
  #[graphql(description = "The score given to this handle by Asami's admin.")]
  score: Option<String>,
  #[graphql(description = "The X coord of a nostr pubkey, for on-chain verification of nostr collabs.")]
  nostr_affine_x: Option<String>,
  #[graphql(description = "The Y coord of a nostr pubkey, for on-chain verification of nostr collabs.")]
  nostr_affine_y: Option<String>,
  #[graphql(description = "Status of this campaign request.")]
  status: HandleRequestStatus,
}

#[derive(Debug, Clone, Default, GraphQLInputObject, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HandleRequestFilter {
  ids: Option<Vec<i32>>,
  id_eq: Option<i32>,
  username_like: Option<String>,
  status_in: Option<Vec<HandleRequestStatus>>,
  site_eq: Option<Site>,
}

#[rocket::async_trait]
impl Showable<models::HandleRequest, HandleRequestFilter> for HandleRequest {
  fn sort_field_to_order_by(field: &str) -> Option<models::HandleRequestOrderBy> {
    match field {
      "id" => Some(HandleRequestOrderBy::Id),
      _ => None,
    }
  }

  fn filter_to_select(context: &Context, filter: Option<HandleRequestFilter>) -> models::SelectHandleRequest {
    if let Some(f) = filter {
      models::SelectHandleRequest {
        id_in: f.ids,
        account_id_eq: Some(context.account_id().to_string()),
        status_in: f.status_in,
        id_eq: f.id_eq,
        site_eq: f.site_eq,
        username_like: into_like_search(f.username_like),
        ..Default::default()
      }
    } else {
      models::SelectHandleRequest {
        account_id_eq: Some(context.account_id().to_string()),
        ..Default::default()
      }
    }
  }

  fn select_by_id(context: &Context, id: i32) -> models::SelectHandleRequest {
    models::SelectHandleRequest { id_eq: Some(id), account_id_eq: Some(context.account_id().to_string()), ..Default::default() }
  }

  async fn db_to_graphql(d: models::HandleRequest) -> AsamiResult<Self> {
    Ok(HandleRequest {
      id: d.attrs.id,
      account_id: d.attrs.account_id,
      site: d.attrs.site,
      username: d.attrs.username,
      user_id: d.attrs.user_id,
      price: d.attrs.price,
      score: d.attrs.score,
      nostr_affine_x: d.attrs.nostr_affine_x,
      nostr_affine_y: d.attrs.nostr_affine_y,
      status: d.attrs.status,
    })
  }
}

#[derive(Clone, GraphQLInputObject, Serialize, Deserialize)]
#[graphql(description = "The input for creating a new HandleRequest.")]
#[serde(rename_all = "camelCase")]
pub struct CreateHandleRequestInput {
  pub username: String,
  pub site: Site,
}

impl CreateHandleRequestInput {
  pub async fn process(self, context: &Context) -> FieldResult<HandleRequest> {
    let req = context.account().await?.create_handle_request(
      self.site,
      &self.username,
    ).await?;

    Ok(HandleRequest::db_to_graphql(req).await?)
  }
}
