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
    collabs: i32,
    rewards: String,
    first_collab_date: DateTime<Utc>,
    last_collab_date: DateTime<Utc>,
}

#[derive(Debug, Clone, Default, GraphQLInputObject, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommunityMemberFilter {
    ids: Option<Vec<i32>>,
    id_eq: Option<i32>,
    account_id_eq: Option<i32>,
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
            collabs: d.attrs.collabs,
            rewards: d.attrs.rewards,
            first_collab_date: d.attrs.first_collab_date,
            last_collab_date: d.attrs.last_collab_date,
        })
    }
}

#[derive(Clone, GraphQLInputObject, Serialize, Deserialize)]
#[graphql(description = "The input for creating a new CampaignRequest.")]
#[serde(rename_all = "camelCase")]
pub struct EditCommunityMemberInput {
    rating: CommunityMemberRating,
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

        let updated = community_member.update().rating(self.rating).save().await?;

        Ok(CommunityMember::db_to_graphql(context, updated).await?)
    }
}
