use super::{
    models::{self, *},
    *,
};

#[derive(Debug, GraphQLObject, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[graphql(description = "A summary view of everything important regarding a member account.")]
pub struct Account {
    #[graphql(description = "Account ID as integer")]
    id: i32,
    #[graphql(description = "Status of this account claim request, if any.")]
    status: AccountStatus,
    #[graphql(description = "The address of a claimed account.")]
    addr: Option<String>,
    #[graphql(description = "The account name.")]
    name: String,
    #[graphql(description = "Tokens awarded, which will be minted when the account is first claimed.")]
    unclaimed_asami_balance: String,
    #[graphql(description = "Rewards awarded to the user, which will be transferred when the account is claimed.")]
    unclaimed_doc_balance: String,
    #[graphql(description = "Asami Tokens in a claimed account's address.")]
    asami_balance: String,
    #[graphql(description = "Doc Balance in a claimed account's address.")]
    doc_balance: String,
    #[graphql(description = "RBTC balance in a claimed account's address.")]
    rbtc_balance: String,
    #[graphql(
        description = "Is the account happy with receiving gasless claims if they are allowed in the smart contract?"
    )]
    allows_gasless: bool,
    #[graphql(description = "Date in which this account was created")]
    created_at: UtcDateTime,

    #[graphql(description = "Collabs made")]
    total_collabs: i32,
    #[graphql(description = "Rewards from collabs made")]
    total_collab_rewards: String,
    #[graphql(description = "Campaigns created")]
    total_campaigns: i32,
    #[graphql(description = "Collabs received in campaings")]
    total_collabs_received: i32,
    #[graphql(description = "Total spent on collabs received")]
    total_spent: String,
}

#[derive(Debug, Clone, Default, GraphQLInputObject, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountFilter {
    ids: Option<Vec<i32>>,
    id_eq: Option<i32>,
    addr_like: Option<String>,
    name_like: Option<String>,
}

#[rocket::async_trait]
impl Showable<models::Account, AccountFilter> for Account {
    fn sort_field_to_order_by(field: &str) -> Option<models::AccountOrderBy> {
        match field {
            "id" => Some(AccountOrderBy::Id),
            "asamiBalance" => Some(AccountOrderBy::AsamiBalance),
            "docBalance" => Some(AccountOrderBy::DocBalance),
            "rbtcBalance" => Some(AccountOrderBy::RbtcBalance),
            "unclaimedAsamiBalance" => Some(AccountOrderBy::UnclaimedAsamiBalance),
            "unclaimedDocBalance" => Some(AccountOrderBy::UnclaimedDocBalance),
            "totalCollabs" => Some(AccountOrderBy::TotalCollabs),
            "totalCollabRewards" => Some(AccountOrderBy::TotalCollabRewards),
            "totalCampaigns" => Some(AccountOrderBy::TotalCampaigns),
            "totalCollabsReceived" => Some(AccountOrderBy::TotalCollabsReceived),
            "totalSpent" => Some(AccountOrderBy::TotalSpent),
            "createdAt" => Some(AccountOrderBy::CreatedAt),
            _ => None,
        }
    }

    fn filter_to_select(_context: &Context, filter: Option<AccountFilter>) -> FieldResult<models::SelectAccount> {
        if let Some(f) = filter {
            Ok(models::SelectAccount {
                id_in: f.ids.map(|ids| ids.into_iter().map(i32_to_hex).collect()),
                id_eq: f.id_eq.map(i32_to_hex),
                addr_ilike: into_like_search(f.addr_like),
                name_ilike: into_like_search(f.name_like),
                ..Default::default()
            })
        } else {
            Ok(Default::default())
        }
    }

    fn select_by_id(_context: &Context, id: String) -> FieldResult<models::SelectAccount> {
        Ok(models::SelectAccount {
            id_eq: Some(wei(id).encode_hex()),
            ..Default::default()
        })
    }

    async fn db_to_graphql(_context: &Context, d: models::Account) -> AsamiResult<Self> {
        let addr = d.decoded_addr()?.map(|x| format!("{x:?}"));

        Ok(Account {
            id: hex_to_i32(&d.attrs.id)?,
            name: d.attrs.name,
            status: d.attrs.status,
            addr,
            asami_balance: d.attrs.asami_balance,
            doc_balance: d.attrs.doc_balance,
            rbtc_balance: d.attrs.rbtc_balance,
            unclaimed_asami_balance: d.attrs.unclaimed_asami_balance,
            unclaimed_doc_balance: d.attrs.unclaimed_doc_balance,
            allows_gasless: d.attrs.allows_gasless,
            created_at: d.attrs.created_at,
            total_collabs: d.attrs.total_collabs,
            total_collab_rewards: d.attrs.total_collab_rewards,
            total_campaigns: d.attrs.total_campaigns,
            total_collabs_received: d.attrs.total_collabs_received,
            total_spent: d.attrs.total_spent,
        })
    }
}
