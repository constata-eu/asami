use super::{*, models::{self, *}};

#[derive(Debug, GraphQLObject, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[graphql(description = "A social network handle associated to an account.")]
pub struct Handle {
  #[graphql(description = "Unique numeric identifier of this resource in the smart contract.")]
  id: String,
  #[graphql(description = "The id of the account that created this.")]
  account_id: String,
  #[graphql(description = "The social network of this handle: X, Instagram, Nostr.")]
  site: Site,
  #[graphql(description = "The username on the given social network. This may change by the user, it may not be a unique id.")]
  username: String,
  #[graphql(description = "The unique user_id in the given social network. This never changes.")]
  user_id: String,
  #[graphql(description = "The price for each collab made with this handle. This is the price for a single repost.")]
  price: String,
  #[graphql(description = "The score given to this handle by Asami's admin.")]
  score: String,
  #[graphql(description = "The X coord of a nostr pubkey, for on-chain verification of nostr collabs.")]
  nostr_affine_x: String,
  #[graphql(description = "The Y coord of a nostr pubkey, for on-chain verification of nostr collabs.")]
  nostr_affine_y: String,
}

#[derive(Debug, Clone, Default, GraphQLInputObject, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HandleFilter {
  ids: Option<Vec<String>>,
  id_eq: Option<String>,
  username_like: Option<String>,
  user_id_like: Option<String>,
  site_eq: Option<Site>,
  account_id_in: Option<Vec<String>>,
}

#[rocket::async_trait]
impl Showable<models::Handle, HandleFilter> for Handle {
  fn sort_field_to_order_by(field: &str) -> Option<models::HandleOrderBy> {
    match field {
      "id" => Some(HandleOrderBy::Id),
      _ => None,
    }
  }

  fn filter_to_select(context: &Context, filter: Option<HandleFilter>) -> models::SelectHandle {
    if let Some(f) = filter {
      models::SelectHandle {
        id_in: f.ids,
        account_id_in: f.account_id_in,
        id_eq: f.id_eq,
        username_like: f.username_like,
        user_id_like: f.user_id_like,
        site_eq: f.site_eq,
        ..Default::default()
      }
    } else {
      models::SelectHandle {
        account_id_in: Some(context.account_ids.clone()),
        ..Default::default()
      }
    }
  }

  fn select_by_id(_context: &Context, id: String) -> models::SelectHandle {
    models::SelectHandle { id_eq: Some(id), ..Default::default() }
  }

  async fn db_to_graphql(d: models::Handle) -> AsamiResult<Self> {
    Ok(Handle {
      id: d.attrs.id,
      account_id: d.attrs.account_id,
      site: d.attrs.site,
      username: d.attrs.username,
      user_id: d.attrs.user_id,
      price: d.attrs.price,
      score: d.attrs.score,
      nostr_affine_x: d.attrs.nostr_affine_x,
      nostr_affine_y: d.attrs.nostr_affine_y,
    })
  }
}