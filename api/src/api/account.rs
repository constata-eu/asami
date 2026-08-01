use super::{
    models::{self, *},
    *,
};
use sqlx::Row;

#[derive(Debug, GraphQLObject, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[graphql(description = "A summary view of everything important regarding a member account.")]
pub struct Account {
    #[graphql(description = "Account ID as integer")]
    id: i32,
    #[graphql(description = "Status of this account claim request, if any.")]
    status: AccountStatus,
    #[graphql(description = "Whether this account is active in the club, on the waitlist, or inactive.")]
    acceptance_status: AcceptanceStatus,
    #[graphql(description = "The address of a claimed account.")]
    addr: Option<String>,
    #[graphql(description = "The account name.")]
    name: String,
    #[graphql(description = "The account ID that invited this member, if any.")]
    invited_by_account_id: Option<i32>,
    #[graphql(description = "The account name that invited this member, if any.")]
    invited_by_name: Option<String>,
    #[graphql(description = "How many invites this account receives per month.")]
    invites_per_month: i32,
    #[graphql(description = "How many invites are currently available for this account.")]
    available_invites: i32,
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
    #[graphql(description = "Historic confirmed yield from tokens")]
    confirmed_yield: String,
    #[graphql(description = "Reputation alignment score aggregated from member scores")]
    alignment_score: Option<f64>,
    #[graphql(description = "Reputation trust score aggregated from member scores")]
    trust_score: Option<f64>,
    #[graphql(description = "Reputation contribution score aggregated from member scores")]
    contribution_score: Option<f64>,
    #[graphql(description = "Reputation advocacy score aggregated from member scores")]
    advocacy_score: Option<f64>,
    #[graphql(description = "Reputation judgment score aggregated from member scores")]
    judgment_score: Option<f64>,
    #[graphql(description = "Derived Rootstock longevity score")]
    rootstock_longevity_score: Option<f64>,
    #[graphql(description = "Earliest claimed date of joining Rootstock")]
    rootstock_joined_at: Option<UtcDateTime>,
    #[graphql(description = "How many member scores this account has received")]
    scores_received_count: i32,
    #[graphql(description = "How many member scores this account has issued")]
    scores_issued_count: i32,
    #[graphql(description = "Their profile pic URL")]
    profile_pic: Option<String>,
}

#[derive(Debug, GraphQLObject, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LandingMember {
    account_id: i32,
    name: String,
    x_username: Option<String>,
    score: f64,
    profile_pic: Option<String>,
}

#[derive(Debug, Clone, Default, GraphQLInputObject, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountFilter {
    ids: Option<Vec<i32>>,
    id_eq: Option<i32>,
    addr_like: Option<String>,
    name_like: Option<String>,
    acceptance_status_eq: Option<AcceptanceStatus>,
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
                acceptance_status_eq: f.acceptance_status_eq,
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

    async fn resource(context: &Context, id: String) -> FieldResult<Self> {
        let resource = context.app.account().select().use_struct(Self::select_by_id(context, id)?).one().await?;
        let scores = context.app.community_member().effective_score_for_target(resource.id()).await?;
        let inviters = load_inviter_names(&context.app, std::slice::from_ref(&resource)).await?;
        Ok(Self::from_model(context, resource, scores.as_ref(), &inviters).await?)
    }

    async fn collection(
        context: &Context,
        page: Option<i32>,
        per_page: Option<i32>,
        sort_field: Option<String>,
        sort_order: Option<String>,
        filter: Option<AccountFilter>,
    ) -> FieldResult<Vec<Self>> {
        let limit = per_page.unwrap_or(DEFAULT_PER_PAGE);
        if limit >= 1001 {
            return Err(FieldError::new(
                "Invalid pagination",
                graphql_value!({ "internal_error": "Invalid pagination" }),
            ));
        }
        let offset = page.unwrap_or(DEFAULT_PAGE) * limit;
        let maybe_order_by = match sort_field {
            None => None,
            Some(ref field) => Some(Self::sort_field_to_order_by(field).ok_or(FieldError::new(
                "Invalid sort_field",
                graphql_value!({ "invalid_sort": format!("{:?}", &sort_field) }),
            ))?),
        };

        let selected = context
            .app
            .account()
            .select()
            .use_struct(Self::filter_to_select(context, filter)?)
            .maybe_order_by(maybe_order_by)
            .limit(limit.into())
            .offset(offset.into())
            .desc(sort_order == Some("DESC".to_string()))
            .all()
            .await?;

        let ids: Vec<String> = selected.iter().map(|account| account.id().clone()).collect();
        let score_map = context.app.community_member().effective_scores_for_targets(&ids).await?;
        let inviter_names = load_inviter_names(&context.app, &selected).await?;

        let mut accounts = vec![];
        for account in selected {
            accounts
                .push(Self::from_model(context, account.clone(), score_map.get(account.id()), &inviter_names).await?);
        }
        Ok(accounts)
    }

    async fn db_to_graphql(context: &Context, d: models::Account) -> AsamiResult<Self> {
        let inviter_names = load_inviter_names(&context.app, std::slice::from_ref(&d)).await?;
        let scores = context.app.community_member().effective_score_for_target(d.id()).await?;
        Self::from_model(context, d, scores.as_ref(), &inviter_names).await
    }
}

impl Account {
    async fn from_model(
        context: &Context,
        d: models::Account,
        score: Option<&models::EffectiveCommunityMemberScore>,
        inviter_names: &std::collections::HashMap<String, String>,
    ) -> AsamiResult<Self> {
        let addr = d.decoded_addr()?.map(|x| format!("{x:?}"));

        let confirmed_yield = if let Some(a) = d.addr().as_ref() {
            match context.app.holder().select().address_eq(a).optional().await? {
                Some(holder) => holder.attrs.estimated_total_doc_claimed,
                None => weihex("1"),
            }
        } else {
            weihex("0")
        };

        let invited_by_account_id = d.invited_by_account_id().as_deref().map(hex_to_i32).transpose()?;
        let invited_by_name = d.invited_by_account_id().as_ref().and_then(|id| inviter_names.get(id).cloned());
        let invites_per_month = *d.invites_per_month();
        let available_invites = *d.available_invites();

        Ok(Account {
            id: hex_to_i32(&d.attrs.id)?,
            name: d.attrs.name,
            status: d.attrs.status,
            acceptance_status: d.attrs.acceptance_status,
            addr,
            invited_by_account_id,
            invited_by_name,
            invites_per_month,
            available_invites,
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
            confirmed_yield,
            alignment_score: score.map(|s| s.alignment_score),
            trust_score: score.map(|s| s.trust_score),
            contribution_score: score.map(|s| s.contribution_score),
            advocacy_score: score.map(|s| s.advocacy_score),
            judgment_score: score.map(|s| s.judgment_score),
            rootstock_longevity_score: score.map(|s| s.rootstock_longevity_score),
            rootstock_joined_at: score.map(|s| s.rootstock_joined_at),
            scores_received_count: score.map(|s| s.scores_received as i32).unwrap_or(0),
            scores_issued_count: score.map(|s| s.scores_issued as i32).unwrap_or(0),
            profile_pic: d.attrs.profile_pic,
        })
    }
}

pub async fn top_rootstockers(context: &Context, limit: i32) -> FieldResult<Vec<LandingMember>> {
    let accounts = context
        .app
        .account()
        .select()
        .acceptance_status_eq(AcceptanceStatus::Active)
        .all()
        .await
        .map_err(|e| Error::from(e).into_field_error())?;

    if accounts.is_empty() {
        return Ok(vec![]);
    }

    let ids: Vec<String> = accounts.iter().map(|account| account.id().clone()).collect();
    let score_map = context
        .app
        .community_member()
        .effective_scores_for_targets(&ids)
        .await
        .map_err(|e| e.into_field_error())?;

    let handles = context
        .app
        .db
        .fetch_all(
            sqlx::query(
                r#"
                SELECT DISTINCT ON (account_id) account_id, username
                FROM handles
                WHERE account_id = ANY($1)
                ORDER BY account_id, id DESC
                "#,
            )
            .bind(&ids)
            .map(|row: sqlx::postgres::PgRow| (row.get::<String, _>("account_id"), row.get::<String, _>("username"))),
        )
        .await
        .map_err(|e| Error::from(e).into_field_error())?;

    let handle_map: std::collections::HashMap<String, String> = handles.into_iter().collect();

    let mut members: Vec<LandingMember> = accounts
        .into_iter()
        .map(|account| {
            let score = score_map.get(account.id()).map(|entry| entry.total_score()).unwrap_or(30.0);
            LandingMember {
                account_id: hex_to_i32(account.id()).unwrap_or_default(),
                name: account.name().clone(),
                x_username: handle_map.get(account.id()).cloned(),
                score,
                profile_pic: account.profile_pic().clone(),
            }
        })
        .collect();

    members.sort_by(|a, b| {
        b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal).then_with(|| a.name.cmp(&b.name))
    });
    members.truncate(limit.max(0) as usize);
    Ok(members)
}

async fn load_inviter_names(
    app: &App,
    accounts: &[models::Account],
) -> AsamiResult<std::collections::HashMap<String, String>> {
    let inviter_ids: Vec<String> =
        accounts.iter().filter_map(|account| account.invited_by_account_id().clone()).collect();

    if inviter_ids.is_empty() {
        return Ok(Default::default());
    }

    Ok(app
        .account()
        .select()
        .use_struct(models::SelectAccount {
            id_in: Some(inviter_ids),
            ..Default::default()
        })
        .all()
        .await?
        .into_iter()
        .map(|inviter| (inviter.id().clone(), inviter.name().clone()))
        .collect())
}

#[derive(Clone, GraphQLInputObject, Serialize, Deserialize)]
#[graphql(description = "Admin fields for updating an account.")]
#[serde(rename_all = "camelCase")]
pub struct AdminEditAccountInput {
    acceptance_status: Option<AcceptanceStatus>,
    invited_by_account_id: Option<i32>,
    invites_per_month: Option<i32>,
    available_invites: Option<i32>,
}

impl AdminEditAccountInput {
    pub async fn process(self, context: &Context, account_id: String) -> FieldResult<Account> {
        if !context.current_session()?.0.admin() {
            return Err(Error::service("authentication", "asami_authentication_needed").into_field_error());
        }

        let account = context.app.account().find(account_id).await?;
        let mut update = account.update();

        if let Some(acceptance_status) = self.acceptance_status {
            update = update.acceptance_status(acceptance_status);
        }
        if let Some(invited_by_account_id) = self.invited_by_account_id {
            update = update.invited_by_account_id(Some(i32_to_hex(invited_by_account_id)));
        }
        if let Some(invites_per_month) = self.invites_per_month {
            update = update.invites_per_month(invites_per_month);
        }
        if let Some(available_invites) = self.available_invites {
            update = update.available_invites(available_invites);
        }

        let updated = update.save().await?;
        Account::db_to_graphql(context, updated).await.map_err(|e| e.into_field_error())
    }
}
