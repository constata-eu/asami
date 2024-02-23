use super::{
  models::{self, *},
  *,
};

#[derive(Debug, GraphQLObject, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[graphql(description = "A user claim to manage their own account")]
pub struct ClaimAccountRequest {
  #[graphql(description = "Unique numeric identifier of this resource")]
  id: i32,
  #[graphql(description = "The id of the account that created this.")]
  account_id: String,
  #[graphql(description = "The address taking ownership.")]
  addr: String,
  #[graphql(description = "Status of this request.")]
  status: GenericRequestStatus,
}

#[derive(Debug, Clone, Default, GraphQLInputObject, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaimAccountRequestFilter {
  ids: Option<Vec<i32>>,
  id_eq: Option<i32>,
  addr_eq: Option<String>,
  status_in: Option<Vec<GenericRequestStatus>>,
}

#[rocket::async_trait]
impl Showable<models::ClaimAccountRequest, ClaimAccountRequestFilter> for ClaimAccountRequest {
  fn sort_field_to_order_by(field: &str) -> Option<models::ClaimAccountRequestOrderBy> {
    match field {
      "id" => Some(ClaimAccountRequestOrderBy::Id),
      _ => None,
    }
  }

  fn filter_to_select(
    context: &Context,
    filter: Option<ClaimAccountRequestFilter>,
  ) -> FieldResult<models::SelectClaimAccountRequest> {
    if let Some(f) = filter {
      Ok(models::SelectClaimAccountRequest {
        id_in: f.ids,
        account_id_eq: Some(context.account_id()?),
        status_in: f.status_in,
        id_eq: f.id_eq,
        addr_eq: f.addr_eq,
        ..Default::default()
      })
    } else {
      Ok(models::SelectClaimAccountRequest {
        account_id_eq: Some(context.account_id()?),
        ..Default::default()
      })
    }
  }

  fn select_by_id(context: &Context, id: i32) -> FieldResult<models::SelectClaimAccountRequest> {
    Ok(models::SelectClaimAccountRequest {
      id_eq: Some(id),
      account_id_eq: Some(context.account_id()?),
      ..Default::default()
    })
  }

  async fn db_to_graphql(d: models::ClaimAccountRequest) -> AsamiResult<Self> {
    Ok(ClaimAccountRequest {
      id: d.attrs.id,
      account_id: d.attrs.account_id,
      addr: d.attrs.addr,
      status: d.attrs.status,
    })
  }
}

#[derive(Clone, GraphQLInputObject, Serialize, Deserialize)]
#[graphql(description = "The input for creating a new ClaimAccountRequest.")]
#[serde(rename_all = "camelCase")]
pub struct CreateClaimAccountRequestInput {
  pub signature: String,
}

impl CreateClaimAccountRequestInput {
  pub async fn process(self, context: &Context) -> FieldResult<ClaimAccountRequest> {
    let address = eip_712_sig_to_address(context.app.settings.rsk.chain_id, &self.signature)
      .map_err(|msg| Error::Validation("eip_712_sig".to_string(), msg))?;

    let req = context
      .account()
      .await?
      .create_claim_account_request(
        address.clone(),
        self.signature,
        context.current_session()?.0.attrs.id.clone(),
      )
      .await?;

    context
      .app
      .auth_method()
      .insert(InsertAuthMethod {
        user_id: context.user_id()?,
        lookup_key: address,
        kind: AuthMethodKind::Eip712,
      })
      .save()
      .await?;

    Ok(ClaimAccountRequest::db_to_graphql(req).await?)
  }
}
