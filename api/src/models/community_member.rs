use super::*;

model! {
  state: App,
  table: community_members,
  struct CommunityMember {
    #[sqlx_model_hints(int4, default, op_in)]
    id: i32,
    #[sqlx_model_hints(varchar)]
    account_id: String,
    #[sqlx_model_hints(varchar)]
    member_id: String,
    #[sqlx_model_hints(community_member_rating, default)]
    rating: CommunityMemberRating,
    #[sqlx_model_hints(int4)]
    collabs: i32,
    #[sqlx_model_hints(varchar)]
    rewards: String,
    #[sqlx_model_hints(timestamptz)]
    first_collab_date: UtcDateTime,
    #[sqlx_model_hints(timestamptz)]
    last_collab_date: UtcDateTime,
    #[sqlx_model_hints(boolean, default)]
    force_hydrate: bool,
  },
  belongs_to {
    Account(account_id),
  }
}

make_sql_enum![
    "community_member_rating",
    pub enum CommunityMemberRating {
        Good,
        Normal,
        Bad,
    }
];

impl CommunityMemberHub {
    pub async fn force_hydrate(&self) -> AsamiResult<()> {
        let ids = self
            .state
            .db
            .fetch_all_scalar(sqlx::query_scalar!(
                "SELECT id FROM community_members WHERE force_hydrate = true LIMIT 50"
            ))
            .await?;
        if ids.is_empty() {
            return Ok(());
        }
        self.hydrate_report_columns_for(ids.iter().copied()).await?;
        self.state
            .db
            .execute(sqlx::query!(
                "UPDATE community_members SET force_hydrate = false WHERE id = ANY($1)",
                &ids
            ))
            .await?;
        Ok(())
    }

    pub async fn hydrate_report_columns_for(&self, ids: impl Iterator<Item = i32>) -> AsamiResult<()> {
        for id in ids {
            self.find(id).await?.hydrate_report_columns().await?;
        }

        Ok(())
    }

    pub async fn create_or_update_from_collab(&self, collab: &Collab) -> AsamiResult<CommunityMember> {
        let found = self
            .select()
            .account_id_eq(collab.advertiser_id())
            .member_id_eq(collab.member_id())
            .optional()
            .await?;

        match found {
            Some(membership) => membership.hydrate_report_columns().await,
            None => Ok(self
                .insert(InsertCommunityMember {
                    account_id: collab.advertiser_id().clone(),
                    member_id: collab.member_id().clone(),
                    collabs: 1,
                    rewards: collab.reward().clone(),
                    first_collab_date: *collab.created_at(),
                    last_collab_date: *collab.created_at(),
                })
                .save()
                .await?),
        }
    }
}

impl CommunityMember {
    pub async fn hydrate_report_columns(self) -> AsamiResult<Self> {
        let advertiser = self.account().await?;
        let all = advertiser.collabs_received().member_id_eq(&self.attrs.member_id).all().await?;

        let mut collabs = 0;
        let mut rewards = u("0");
        for c in &all {
            collabs += 1;
            rewards += c.reward_u256();
        }
        let first_collab_date = all.iter().map(|c| *c.created_at()).min().unwrap_or(Utc::now());
        let last_collab_date = all.iter().map(|c| *c.created_at()).max().unwrap_or(Utc::now());

        Ok(self
            .update()
            .collabs(collabs)
            .rewards(rewards.encode_hex())
            .first_collab_date(first_collab_date)
            .last_collab_date(last_collab_date)
            .save()
            .await?)
    }
}
