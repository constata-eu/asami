use super::{models::*, *};

#[derive(Debug, GraphQLObject, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[graphql(description = "Global site stats.")]
pub struct Stats {
    #[graphql(description = "Unique numeric identifier of this resource")]
    pub id: i32,

    #[graphql(description = "Members to ever be active")]
    pub total_active_members: i32,
    #[graphql(description = "Total signups")]
    pub total_signups: i32,
    #[graphql(description = "Members that created campaigns")]
    pub total_advertisers: i32,

    #[graphql(description = "Handles that made at least one collab")]
    pub total_active_handles: i32,
    #[graphql(description = "Handles that have collaborated last 30 days")]
    pub currently_active: i32,
    #[graphql(description = "Handles who joined in the past 30 days")]
    pub joined_recently: i32,

    #[graphql(description = "How many campaigns were run")]
    pub total_campaigns: i32,
    #[graphql(description = "How many campaigns in the last 30 days.")]
    pub recent_campaigns: i32,
    #[graphql(description = "How many campaigns a month on average.")]
    pub thirty_day_average_campaigns: Decimal,

    #[graphql(description = "How many collabs were made")]
    pub total_collabs: i32,
    #[graphql(description = "How many collabs in teh past 30 days")]
    pub recent_collabs: i32,
    #[graphql(description = "Whats the average collabs per month")]
    pub thirty_day_average_collabs: Decimal,

    #[graphql(description = "How much money was paid in rewards")]
    pub total_rewards_paid: Decimal,
    #[graphql(description = "How much was paid in the last 30 days")]
    pub recent_rewards_paid: Decimal,
    #[graphql(description = "What's the average paid in rewards in 30 days")]
    pub thirty_day_average_rewards_paid: Decimal,
    #[graphql(description = "Date in which these stats were calculated")]
    pub date: UtcDateTime,
}

impl Stats {
    pub async fn build(app: &App) -> FieldResult<Self> {
        let thirty_days_ago = Utc::now() - chrono::Duration::days(30);
        let oldest_date = app
            .account()
            .select()
            .order_by(AccountOrderBy::Id)
            .desc(false)
            .optional()
            .await?
            .map(|a| a.attrs.created_at)
            .unwrap_or(Utc::now());

        let total_days = std::cmp::max(1, (Utc::now() - oldest_date).num_days());
        let thirty_day_average = |x: i32| Decimal::from(x) / Decimal::from(total_days) * Decimal::from(30);

        let total_active_members = app
            .db
            .fetch_one_scalar::<i32>(sqlx::query_scalar!(
                r#"SELECT COUNT(*)::INT4 AS "count!"
                FROM (
                    SELECT distinct(member_id) as "count!" FROM collabs
                    UNION
                    SELECT distinct(account_id) as "count!" FROM campaigns
                ) AS combined_users"#
            ))
            .await?;

        let total_signups: i32 = app.account().select().count().await?.try_into()?;

        let total_advertisers = app
            .db
            .fetch_one_scalar::<i32>(sqlx::query_scalar!(
                r#"SELECT count(distinct account_id)::INT4 as "count!" FROM campaigns"#
            ))
            .await?;

        let total_active_handles = app
            .db
            .fetch_one_scalar::<i32>(sqlx::query_scalar!(
                r#"SELECT count(distinct handle_id)::INT4 as "count!" FROM collabs"#
            ))
            .await?;
        let currently_active: i32 = app.handle().select().status_eq(HandleStatus::Active).count().await?.try_into()?;
        let joined_recently: i32 = app
            .handle()
            .select()
            .status_eq(HandleStatus::Active)
            .created_at_gt(thirty_days_ago)
            .count()
            .await?
            .try_into()?;

        let total_collabs: i32 = app.collab().select().count().await?.try_into()?;
        let recent_collabs: i32 = app.collab().select().created_at_gt(thirty_days_ago).count().await?.try_into()?;

        let total_campaigns: i32 =
            app.campaign().select().status_eq(CampaignStatus::Published).count().await?.try_into()?;
        let recent_campaigns: i32 = app
            .campaign()
            .select()
            .status_eq(CampaignStatus::Published)
            .created_at_gt(thirty_days_ago)
            .count()
            .await?
            .try_into()?;

        let total_rewards_paid: Decimal = app.db
            .fetch_one_scalar(sqlx::query_scalar!(
                r#"SELECT COALESCE(SUM(u256_hex_to_numeric(reward)) / '1e18'::numeric, 0) AS "sum!"
                    FROM collabs WHERE status = 'cleared'"#)).await?;

        let recent_rewards_paid: Decimal = app.db
            .fetch_one_scalar(sqlx::query_scalar!(
                r#"SELECT COALESCE(SUM(u256_hex_to_numeric(reward)) / '1e18'::numeric, 1) AS "sum!"
                    FROM collabs WHERE status = 'cleared' AND created_at > $1"#,
                thirty_days_ago)).await?;

        Ok(Stats {
            id: 0,
            total_active_members,
            total_signups,
            total_advertisers,
            total_active_handles,
            currently_active,
            joined_recently,

            total_collabs,
            recent_collabs,
            thirty_day_average_collabs: thirty_day_average(total_collabs),

            total_campaigns,
            recent_campaigns,
            thirty_day_average_campaigns: thirty_day_average(total_campaigns),

            total_rewards_paid,
            recent_rewards_paid,
            thirty_day_average_rewards_paid: total_rewards_paid / Decimal::from(total_days) * Decimal::from(30),
            date: chrono::Utc::now(),
        })
    }
}
