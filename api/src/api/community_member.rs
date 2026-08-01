use super::{
    models::{self, *},
    *,
};

#[derive(Debug, GraphQLObject, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[graphql(description = "A request to verify a handle for an account.")]
pub struct CommunityMember {
    id: i32,
    account_id: i32,
    member_id: i32,
    rating: CommunityMemberRating,
    priority: bool,
    alignment: Option<i32>,
    trust: Option<i32>,
    contribution: Option<i32>,
    advocacy: Option<i32>,
    judgment: Option<i32>,
    rootstock_joined_at: Option<DateTime<Utc>>,
    comment: Option<String>,
    collabs: i32,
    rewards: String,
    first_collab_date: Option<DateTime<Utc>>,
    last_collab_date: Option<DateTime<Utc>>,
}

#[derive(Debug, GraphQLObject, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScorableAccount {
    account_id: i32,
    name: String,
    reason: String,
    window_start: UtcDateTime,
    scorable_until: UtcDateTime,
}

#[derive(Debug, GraphQLObject, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommunityMemberSkip {
    id: i32,
    scorer_account_id: i32,
    target_account_id: i32,
    reason: String,
    created_at: UtcDateTime,
}

#[derive(Debug, Clone, Default, GraphQLInputObject, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommunityMemberFilter {
    ids: Option<Vec<i32>>,
    id_eq: Option<i32>,
    account_id_eq: Option<i32>,
    member_id_eq: Option<i32>,
    rating_eq: Option<CommunityMemberRating>,
}

#[rocket::async_trait]
impl Showable<models::CommunityMember, CommunityMemberFilter> for CommunityMember {
    fn sort_field_to_order_by(field: &str) -> Option<models::CommunityMemberOrderBy> {
        match field {
            "id" => Some(CommunityMemberOrderBy::Id),
            "rating" => Some(CommunityMemberOrderBy::Rating),
            "collabs" => Some(CommunityMemberOrderBy::Collabs),
            "rewards" => Some(CommunityMemberOrderBy::Rewards),
            "first_collab_date" => Some(CommunityMemberOrderBy::FirstCollabDate),
            "last_collab_date" => Some(CommunityMemberOrderBy::LastCollabDate),
            _ => None,
        }
    }

    fn filter_to_select(
        _context: &Context,
        filter: Option<CommunityMemberFilter>,
    ) -> FieldResult<models::SelectCommunityMember> {
        if let Some(f) = filter {
            Ok(models::SelectCommunityMember {
                id_in: f.ids,
                id_eq: f.id_eq,
                rating_eq: f.rating_eq,
                account_id_eq: f.account_id_eq.map(i32_to_hex),
                member_id_eq: f.member_id_eq.map(i32_to_hex),
                ..Default::default()
            })
        } else {
            Ok(Default::default())
        }
    }

    fn select_by_id(_context: &Context, id: i32) -> FieldResult<models::SelectCommunityMember> {
        Ok(models::SelectCommunityMember {
            id_eq: Some(id),
            ..Default::default()
        })
    }

    async fn db_to_graphql(_context: &Context, d: models::CommunityMember) -> AsamiResult<Self> {
        Ok(CommunityMember {
            id: d.attrs.id,
            account_id: hex_to_i32(&d.attrs.account_id)?,
            member_id: hex_to_i32(&d.attrs.member_id)?,
            rating: d.attrs.rating,
            priority: d.attrs.priority,
            alignment: d.attrs.alignment,
            trust: d.attrs.trust,
            contribution: d.attrs.contribution,
            advocacy: d.attrs.advocacy,
            judgment: d.attrs.judgment,
            rootstock_joined_at: d.attrs.rootstock_joined_at,
            comment: d.attrs.comment,
            collabs: d.attrs.collabs,
            rewards: d.attrs.rewards,
            first_collab_date: d.attrs.first_collab_date,
            last_collab_date: d.attrs.last_collab_date,
        })
    }
}

impl CommunityMemberSkip {
    async fn db_to_graphql(_context: &Context, d: models::ScoreCardSkip) -> AsamiResult<Self> {
        Ok(Self {
            id: *d.id(),
            scorer_account_id: hex_to_i32(d.scorer_account_id())?,
            target_account_id: hex_to_i32(d.target_account_id())?,
            reason: d.reason().clone(),
            created_at: *d.created_at(),
        })
    }
}

#[derive(Clone, GraphQLInputObject, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCommunityMemberInput {
    member_id: i32,
    priority: bool,
    alignment: i32,
    trust: i32,
    contribution: i32,
    advocacy: i32,
    judgment: i32,
    rootstock_joined_at: DateTime<Utc>,
    comment: Option<String>,
}

impl CreateCommunityMemberInput {
    pub async fn process(self, context: &Context) -> FieldResult<CommunityMember> {
        let updated = context
            .app
            .community_member()
            .upsert_score(
                UpsertCommunityMemberScore {
                    account_id: context.account_id()?,
                    member_id: i32_to_hex(self.member_id),
                    priority: self.priority,
                    alignment: self.alignment,
                    trust: self.trust,
                    contribution: self.contribution,
                    advocacy: self.advocacy,
                    judgment: self.judgment,
                    rootstock_joined_at: self.rootstock_joined_at,
                    comment: self.comment,
                },
                *context.current_session()?.0.admin(),
            )
            .await
            .map_err(|e| e.into_field_error())?;

        CommunityMember::db_to_graphql(context, updated)
            .await
            .map_err(|e| e.into_field_error())
    }
}

#[derive(Clone, GraphQLInputObject, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCommunityMemberMembershipInput {
    member_id: i32,
}

impl CreateCommunityMemberMembershipInput {
    pub async fn process(self, context: &Context) -> FieldResult<CommunityMember> {
        let created = context
            .app
            .community_member()
            .ensure_membership(&context.account_id()?, &i32_to_hex(self.member_id))
            .await
            .map_err(|e| e.into_field_error())?;

        CommunityMember::db_to_graphql(context, created)
            .await
            .map_err(|e| e.into_field_error())
    }
}

#[derive(Clone, GraphQLInputObject, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCommunityMemberSkipInput {
    member_id: i32,
    reason: String,
}

impl CreateCommunityMemberSkipInput {
    pub async fn process(self, context: &Context) -> FieldResult<CommunityMemberSkip> {
        let created = context
            .app
            .score_card_skip()
            .create_checked(
                InsertScoreCardSkip {
                    scorer_account_id: context.account_id()?,
                    target_account_id: i32_to_hex(self.member_id),
                    reason: self.reason,
                },
                *context.current_session()?.0.admin(),
            )
            .await
            .map_err(|e| e.into_field_error())?;

        CommunityMemberSkip::db_to_graphql(context, created)
            .await
            .map_err(|e| e.into_field_error())
    }
}

pub async fn scorable_accounts_for_current_account(context: &Context) -> FieldResult<Vec<ScorableAccount>> {
    let scorable = context
        .app
        .community_member()
        .scorable_targets_for(&context.account_id()?, None)
        .await
        .map_err(|e| e.into_field_error())?;

    if scorable.is_empty() {
        return Ok(vec![]);
    }

    let accounts = context
        .app
        .account()
        .select()
        .use_struct(models::SelectAccount {
            id_in: Some(scorable.keys().cloned().collect()),
            ..Default::default()
        })
        .all()
        .await
        .map_err(|e| Error::from(e).into_field_error())?;

    let mut response = vec![];
    for account in accounts {
        if let Some(scorable_account) = scorable.get(account.id()) {
            response.push(ScorableAccount {
                account_id: hex_to_i32(account.id()).map_err(|e| Error::runtime(&e.to_string()).into_field_error())?,
                name: account.name().clone(),
                reason: scorable_account.reason.clone(),
                window_start: scorable_account.window_start,
                scorable_until: scorable_account.scorable_until,
            });
        }
    }

    response.sort_by(|a, b| a.scorable_until.cmp(&b.scorable_until).then_with(|| a.name.cmp(&b.name)));
    Ok(response)
}

#[derive(Clone, GraphQLInputObject, Serialize, Deserialize)]
#[graphql(description = "The input for creating a new CampaignRequest.")]
#[serde(rename_all = "camelCase")]
pub struct EditCommunityMemberInput {
    rating: Option<CommunityMemberRating>,
    priority: Option<bool>,
    alignment: Option<i32>,
    trust: Option<i32>,
    contribution: Option<i32>,
    advocacy: Option<i32>,
    judgment: Option<i32>,
    rootstock_joined_at: Option<DateTime<Utc>>,
    comment: Option<String>,
}

impl EditCommunityMemberInput {
    pub async fn process(self, context: &Context, community_member_id: i32) -> FieldResult<CommunityMember> {
        let community_member = context
            .app
            .community_member()
            .select()
            .account_id_eq(context.account_id()?)
            .id_eq(community_member_id)
            .one()
            .await?;

        let mut update = community_member.update();
        if let Some(rating) = self.rating {
            update = update.rating(rating);
        }
        if let Some(priority) = self.priority {
            update = update.priority(priority);
        }
        if self.alignment.is_some() {
            update = update.alignment(self.alignment);
        }
        if self.trust.is_some() {
            update = update.trust(self.trust);
        }
        if self.contribution.is_some() {
            update = update.contribution(self.contribution);
        }
        if self.advocacy.is_some() {
            update = update.advocacy(self.advocacy);
        }
        if self.judgment.is_some() {
            update = update.judgment(self.judgment);
        }
        if self.rootstock_joined_at.is_some() {
            update = update.rootstock_joined_at(self.rootstock_joined_at);
        }
        if self.comment.is_some() {
            update = update.comment(self.comment);
        }

        let updated = update.updated_at(Some(Utc::now())).save().await?;

        Ok(CommunityMember::db_to_graphql(context, updated).await?)
    }
}
