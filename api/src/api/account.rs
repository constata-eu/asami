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
    #[graphql(description = "Status of this account claim request, if any.")]
    status: AccountStatus,
    #[graphql(description = "The address of a claimed account.")]
    addr: Option<String>,
    #[graphql(description = "Tokens awarded, which will be minted when the account is first claimed.")]
    unclaimed_asami_balance: String,
    #[graphql(description = "Rewards awarded to the user, which will be transferred when the account is claimed.")]
    unclaimed_doc_balance: String,
    #[graphql(description = "Asami Tokens in a claimed account's address.")]
    asami_balance: String,
    #[graphql(description = "Doc Balance in a claimed account's address.")]
    doc_balance: String,
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
        let asami = &d.state.on_chain.asami_contract;
        let address = d.decoded_addr()?;

        let (doc_balance, asami_balance, unclaimed_doc_balance, unclaimed_asami_balance) = match address {
            Some(address) => {
                let account = asami.accounts(address).call().await?;
                (
                    d.state.on_chain.doc_contract.balance_of(address).call().await?.encode_hex(),
                    asami.balance_of(address).call().await?.encode_hex(),
                    account.4.encode_hex(),
                    account.3.encode_hex(),
                )
            }
            None => {
                let admin = d.state.settings.rsk.admin_address;
                let (unclaimed_doc, unclaimed_asami) = asami
                    .get_sub_account(admin, u256(d.id()))
                    .call()
                    .await
                    .map(|s| {
                        (
                            s.unclaimed_doc_balance.encode_hex(),
                            s.unclaimed_asami_balance.encode_hex(),
                        )
                    })
                    .unwrap_or_else(|_| (weihex("0"), weihex("0")));

                (weihex("0"), weihex("0"), unclaimed_doc, unclaimed_asami)
            }
        };

        Ok(Account {
            id: d.attrs.id,
            status: d.attrs.status,
            addr: address.map(|x| format!("{x:?}")),
            asami_balance,
            doc_balance,
            unclaimed_asami_balance,
            unclaimed_doc_balance,
        })
    }
}
