use super::{
  models::{self, *},
  *,
};

#[derive(Debug, GraphQLObject, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[graphql(description = "A summary view of everything important regarding a member account.")]
pub struct Account {
  #[graphql(description = "Account ID as stored in the ASAMI contract.")]
  id: String,
  #[graphql(description = "Tokens awarded, which will be minted when the account is first claimed.")]
  unclaimed_asami_tokens: String,
  #[graphql(description = "Rewards awarded to the user, which will be transferred when the account is claimed.")]
  unclaimed_doc_rewards: String,
  #[graphql(description = "Status of this account claim request, if any.")]
  status: Option<GenericRequestStatus>,
  #[graphql(description = "The address of a claimed account.")]
  addr: Option<String>,
  #[graphql(description = "Asami Tokens in a claimed account's address.")]
  asami_balance: Option<String>,
  #[graphql(description = "Doc Balance in a claimed account's address.")]
  doc_balance: Option<String>,
}

#[derive(Debug, Clone, Default, GraphQLInputObject, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountFilter {
  ids: Option<Vec<String>>,
  id_eq: Option<String>,
  addr_eq: Option<String>,
}

#[rocket::async_trait]
impl Showable<models::Account, AccountFilter> for Account {
  fn sort_field_to_order_by(field: &str) -> Option<models::AccountOrderBy> {
    match field {
      "id" => Some(AccountOrderBy::Id),
      _ => None,
    }
  }

  fn filter_to_select(_context: &Context, filter: Option<AccountFilter>) -> FieldResult<models::SelectAccount> {
    if let Some(f) = filter {
      Ok(models::SelectAccount {
        id_in: f.ids,
        id_eq: f.id_eq,
        addr_eq: f.addr_eq,
        ..Default::default()
      })
    } else {
      Ok(Default::default())
    }
  }

  fn select_by_id(_context: &Context, id: String) -> FieldResult<models::SelectAccount> {
    Ok(models::SelectAccount {
      id_eq: Some(id),
      ..Default::default()
    })
  }

  async fn db_to_graphql(d: models::Account) -> AsamiResult<Self> {
    let status = d
      .claim_account_request_scope()
      .status_ne(&GenericRequestStatus::Failed)
      .optional()
      .await?
      .map(|x| x.attrs.status);
    let address = d.decoded_addr()?;
    let (doc_balance, asami_balance) = match address {
      Some(address) => (
        Some(d.state.on_chain.doc_contract.balance_of(address).call().await?.encode_hex()),
        Some(d.state.on_chain.asami_contract.balance_of(address).call().await?.encode_hex()),
      ),
      None => (None, None),
    };
    Ok(Account {
      id: d.attrs.id,
      unclaimed_asami_tokens: d.attrs.unclaimed_asami_tokens,
      unclaimed_doc_rewards: d.attrs.unclaimed_doc_rewards,
      status,
      addr: address.map(|x| format!("{x:?}")),
      asami_balance,
      doc_balance,
    })
  }
}
