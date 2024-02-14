use super::{*, models::{self, *}};

#[derive(Debug, GraphQLObject, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[graphql(description = "A request to set the price for an unclaimed account.")]
pub struct SetPriceRequest {
  #[graphql(description = "Unique numeric identifier of this resource")]
  id: i32,
  #[graphql(description = "The id of the account that created this.")]
  account_id: String,
  #[graphql(description = "The numeric id of the handle in the smart contract. Hex encoded uint256.")]
  handle_id: String,
  #[graphql(description = "The price for each collab made with this handle. This is the price for a single repost.")]
  price: String,
  #[graphql(description = "Status of this request.")]
  status: GenericRequestStatus,
}

#[derive(Debug, Clone, Default, GraphQLInputObject, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetPriceRequestFilter {
  ids: Option<Vec<i32>>,
  id_eq: Option<i32>,
  handle_id_eq: Option<String>,
  status_in: Option<Vec<GenericRequestStatus>>,
}

#[rocket::async_trait]
impl Showable<models::SetPriceRequest, SetPriceRequestFilter> for SetPriceRequest {
  fn sort_field_to_order_by(field: &str) -> Option<models::SetPriceRequestOrderBy> {
    match field {
      "id" => Some(SetPriceRequestOrderBy::Id),
      _ => None,
    }
  }

  fn filter_to_select(context: &Context, filter: Option<SetPriceRequestFilter>) -> models::SelectSetPriceRequest {
    if let Some(f) = filter {
      models::SelectSetPriceRequest {
        id_in: f.ids,
        account_id_eq: Some(context.account_id()),
        status_in: f.status_in,
        id_eq: f.id_eq,
        handle_id_eq: f.handle_id_eq,
        ..Default::default()
      }
    } else {
      models::SelectSetPriceRequest {
        account_id_eq: Some(context.account_id()),
        ..Default::default()
      }
    }
  }

  fn select_by_id(context: &Context, id: i32) -> models::SelectSetPriceRequest {
    models::SelectSetPriceRequest { id_eq: Some(id), account_id_eq: Some(context.account_id()), ..Default::default() }
  }

  async fn db_to_graphql(d: models::SetPriceRequest) -> AsamiResult<Self> {
    Ok(SetPriceRequest {
      id: d.attrs.id,
      account_id: d.attrs.account_id,
      handle_id: d.attrs.handle_id,
      price: d.attrs.price,
      status: d.attrs.status,
    })
  }
}

#[derive(Clone, GraphQLInputObject, Serialize, Deserialize)]
#[graphql(description = "The input for creating a new SetPriceRequest.")]
#[serde(rename_all = "camelCase")]
pub struct CreateSetPriceRequestInput {
  pub handle_id: String,
  pub price: String,
}

impl CreateSetPriceRequestInput {
  pub async fn process(self, context: &Context) -> FieldResult<SetPriceRequest> {
    let handle = context.account().await?.handle_scope().id_eq(&self.handle_id).one().await?;

    let req = context.app.set_price_request().insert(InsertSetPriceRequest{
      account_id: handle.attrs.account_id,
      handle_id: handle.attrs.id,
      price: self.price,
    }).save().await?;

    Ok(SetPriceRequest::db_to_graphql(req).await?)
  }
}
