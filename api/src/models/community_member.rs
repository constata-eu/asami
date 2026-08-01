use super::*;
use std::collections::HashMap;
use sqlx::Row;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectiveCommunityMemberScoreRow {
    pub account_id: String,
    pub alignment_score: f64,
    pub trust_score: f64,
    pub contribution_score: f64,
    pub advocacy_score: f64,
    pub judgment_score: f64,
    pub rootstock_joined_at: UtcDateTime,
    pub scores_received: i64,
    pub scores_issued: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectiveCommunityMemberScore {
    pub account_id: String,
    pub alignment_score: f64,
    pub trust_score: f64,
    pub contribution_score: f64,
    pub advocacy_score: f64,
    pub judgment_score: f64,
    pub rootstock_joined_at: UtcDateTime,
    pub rootstock_longevity_score: f64,
    pub scores_received: i64,
    pub scores_issued: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatorCommunityMemberScoreRow {
    pub account_id: String,
    pub alignment_score: f64,
    pub trust_score: f64,
    pub contribution_score: f64,
    pub advocacy_score: f64,
    pub judgment_score: f64,
    pub rootstock_joined_at: UtcDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatorCommunityMemberScore {
    pub account_id: String,
    pub alignment_score: f64,
    pub trust_score: f64,
    pub contribution_score: f64,
    pub advocacy_score: f64,
    pub judgment_score: f64,
    pub rootstock_joined_at: UtcDateTime,
    pub rootstock_longevity_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScorableCommunityMemberTargetRow {
    pub account_id: String,
    pub reason: String,
    pub window_start: UtcDateTime,
    pub scorable_until: UtcDateTime,
    pub latest_score_at: Option<UtcDateTime>,
    pub latest_skip_at: Option<UtcDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScorableCommunityMemberTarget {
    pub account_id: String,
    pub reason: String,
    pub window_start: UtcDateTime,
    pub scorable_until: UtcDateTime,
    pub latest_score_at: Option<UtcDateTime>,
    pub latest_skip_at: Option<UtcDateTime>,
}

#[derive(Debug, Clone)]
pub struct UpsertCommunityMemberScore {
    pub account_id: String,
    pub member_id: String,
    pub priority: bool,
    pub alignment: i32,
    pub trust: i32,
    pub contribution: i32,
    pub advocacy: i32,
    pub judgment: i32,
    pub rootstock_joined_at: UtcDateTime,
    pub comment: Option<String>,
}

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
    #[sqlx_model_hints(int4, default)]
    collabs: i32,
    #[sqlx_model_hints(varchar, default)]
    rewards: String,
    #[sqlx_model_hints(timestamptz, default, op_is_set)]
    first_collab_date: Option<UtcDateTime>,
    #[sqlx_model_hints(timestamptz, default, op_is_set)]
    last_collab_date: Option<UtcDateTime>,
    #[sqlx_model_hints(boolean, default)]
    force_hydrate: bool,
    #[sqlx_model_hints(boolean, default)]
    priority: bool,
    #[sqlx_model_hints(int4, default, op_is_set)]
    alignment: Option<i32>,
    #[sqlx_model_hints(int4, default, op_is_set)]
    trust: Option<i32>,
    #[sqlx_model_hints(int4, default, op_is_set)]
    contribution: Option<i32>,
    #[sqlx_model_hints(int4, default, op_is_set)]
    advocacy: Option<i32>,
    #[sqlx_model_hints(int4, default, op_is_set)]
    judgment: Option<i32>,
    #[sqlx_model_hints(timestamptz, default, op_is_set)]
    rootstock_joined_at: Option<UtcDateTime>,
    #[sqlx_model_hints(text, default)]
    comment: Option<String>,
    #[sqlx_model_hints(timestamptz, default)]
    created_at: UtcDateTime,
    #[sqlx_model_hints(timestamptz, default, op_is_set)]
    updated_at: Option<UtcDateTime>,
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
                })
                .save()
                .await?
                .hydrate_report_columns()
                .await?),
        }
    }

    pub async fn ensure_membership(
        &self,
        account_id: &str,
        member_id: &str,
    ) -> AsamiResult<CommunityMember> {
        if account_id == member_id {
            return Err(Error::validation("member_id", "cannot_add_yourself"));
        }

        if let Some(existing) = self
            .select()
            .account_id_eq(account_id.to_string())
            .member_id_eq(member_id.to_string())
            .optional()
            .await?
        {
            return Ok(existing);
        }

        Ok(self
            .insert(InsertCommunityMember {
                account_id: account_id.to_string(),
                member_id: member_id.to_string(),
            })
            .save()
            .await?)
    }

    pub async fn upsert_score(
        &self,
        input: UpsertCommunityMemberScore,
        allow_any_target: bool,
    ) -> AsamiResult<CommunityMember> {
        self.ensure_scorable_from_to(&input.account_id, &input.member_id, allow_any_target)
            .await?;

        let membership = self.ensure_membership(&input.account_id, &input.member_id).await?;
        Ok(membership
            .update()
            .priority(input.priority)
            .alignment(Some(input.alignment))
            .trust(Some(input.trust))
            .contribution(Some(input.contribution))
            .advocacy(Some(input.advocacy))
            .judgment(Some(input.judgment))
            .rootstock_joined_at(Some(input.rootstock_joined_at))
            .comment(input.comment)
            .updated_at(Some(Utc::now()))
            .save()
            .await?)
    }

    pub async fn effective_scores_for_targets(
        &self,
        target_account_ids: &[String],
    ) -> AsamiResult<HashMap<String, EffectiveCommunityMemberScore>> {
        if target_account_ids.is_empty() {
            return Ok(HashMap::new());
        }

        let default_joined_at = default_rootstock_joined_at();
        let rows = self.state.db.fetch_all(
            sqlx::query(
                r#"
                WITH targets AS (
                    SELECT UNNEST($1::varchar[]) AS account_id
                ),
                latest_priority AS (
                    SELECT DISTINCT ON (member_id)
                        member_id,
                        alignment::float8 AS alignment,
                        trust::float8 AS trust,
                        contribution::float8 AS contribution,
                        advocacy::float8 AS advocacy,
                        judgment::float8 AS judgment
                    FROM community_members
                    WHERE priority = true AND member_id = ANY($1::varchar[])
                    ORDER BY member_id, COALESCE(updated_at, created_at) DESC, id DESC
                ),
                regular_scores AS (
                    SELECT
                        member_id,
                        AVG(alignment)::float8 AS alignment,
                        AVG(trust)::float8 AS trust,
                        AVG(contribution)::float8 AS contribution,
                        AVG(advocacy)::float8 AS advocacy,
                        AVG(judgment)::float8 AS judgment
                    FROM community_members
                    WHERE priority = false AND member_id = ANY($1::varchar[])
                    GROUP BY member_id
                ),
                seniority AS (
                    SELECT member_id, MIN(rootstock_joined_at) AS rootstock_joined_at
                    FROM community_members
                    WHERE member_id = ANY($1::varchar[])
                    GROUP BY member_id
                ),
                received_counts AS (
                    SELECT member_id, COUNT(*)::bigint AS scores_received
                    FROM community_members
                    WHERE member_id = ANY($1::varchar[])
                      AND (
                        alignment IS NOT NULL OR trust IS NOT NULL OR contribution IS NOT NULL OR
                        advocacy IS NOT NULL OR judgment IS NOT NULL OR rootstock_joined_at IS NOT NULL
                      )
                    GROUP BY member_id
                ),
                issued_counts AS (
                    SELECT account_id, COUNT(*)::bigint AS scores_issued
                    FROM community_members
                    WHERE account_id = ANY($1::varchar[])
                      AND (
                        alignment IS NOT NULL OR trust IS NOT NULL OR contribution IS NOT NULL OR
                        advocacy IS NOT NULL OR judgment IS NOT NULL OR rootstock_joined_at IS NOT NULL
                      )
                    GROUP BY account_id
                )
                SELECT
                    t.account_id,
                    COALESCE(
                        CASE
                            WHEN p.member_id IS NOT NULL AND r.member_id IS NOT NULL THEN (p.alignment + r.alignment) / 2
                            WHEN p.member_id IS NOT NULL THEN p.alignment
                            ELSE r.alignment
                        END,
                        5.0
                    ) AS alignment_score,
                    COALESCE(
                        CASE
                            WHEN p.member_id IS NOT NULL AND r.member_id IS NOT NULL THEN (p.trust + r.trust) / 2
                            WHEN p.member_id IS NOT NULL THEN p.trust
                            ELSE r.trust
                        END,
                        5.0
                    ) AS trust_score,
                    COALESCE(
                        CASE
                            WHEN p.member_id IS NOT NULL AND r.member_id IS NOT NULL THEN (p.contribution + r.contribution) / 2
                            WHEN p.member_id IS NOT NULL THEN p.contribution
                            ELSE r.contribution
                        END,
                        5.0
                    ) AS contribution_score,
                    COALESCE(
                        CASE
                            WHEN p.member_id IS NOT NULL AND r.member_id IS NOT NULL THEN (p.advocacy + r.advocacy) / 2
                            WHEN p.member_id IS NOT NULL THEN p.advocacy
                            ELSE r.advocacy
                        END,
                        5.0
                    ) AS advocacy_score,
                    COALESCE(
                        CASE
                            WHEN p.member_id IS NOT NULL AND r.member_id IS NOT NULL THEN (p.judgment + r.judgment) / 2
                            WHEN p.member_id IS NOT NULL THEN p.judgment
                            ELSE r.judgment
                        END,
                        5.0
                    ) AS judgment_score,
                    COALESCE(s.rootstock_joined_at, $2) AS rootstock_joined_at,
                    COALESCE(rc.scores_received, 0) AS scores_received,
                    COALESCE(ic.scores_issued, 0) AS scores_issued
                FROM targets t
                LEFT JOIN latest_priority p ON p.member_id = t.account_id
                LEFT JOIN regular_scores r ON r.member_id = t.account_id
                LEFT JOIN seniority s ON s.member_id = t.account_id
                LEFT JOIN received_counts rc ON rc.member_id = t.account_id
                LEFT JOIN issued_counts ic ON ic.account_id = t.account_id
                "#,
            )
            .bind(target_account_ids)
            .bind(default_joined_at)
            .map(|row: sqlx::postgres::PgRow| EffectiveCommunityMemberScoreRow {
                account_id: row.get("account_id"),
                alignment_score: row.get("alignment_score"),
                trust_score: row.get("trust_score"),
                contribution_score: row.get("contribution_score"),
                advocacy_score: row.get("advocacy_score"),
                judgment_score: row.get("judgment_score"),
                rootstock_joined_at: row.get("rootstock_joined_at"),
                scores_received: row.get("scores_received"),
                scores_issued: row.get("scores_issued"),
            }),
        ).await.map_err(Error::DatabaseError)?;

        Ok(rows
            .into_iter()
            .map(|row| {
                let account_id = row.account_id.clone();
                (account_id.clone(), EffectiveCommunityMemberScore::from_row(row))
            })
            .collect())
    }

    pub async fn effective_score_for_target(
        &self,
        target_account_id: &str,
    ) -> AsamiResult<Option<EffectiveCommunityMemberScore>> {
        Ok(self
            .effective_scores_for_targets(&[target_account_id.to_string()])
            .await?
            .remove(target_account_id))
    }

    pub async fn creator_scores_for_targets(
        &self,
        creator_account_id: &str,
        target_account_ids: &[String],
    ) -> AsamiResult<HashMap<String, CreatorCommunityMemberScore>> {
        if target_account_ids.is_empty() {
            return Ok(HashMap::new());
        }

        let default_joined_at = default_rootstock_joined_at();
        let rows = self.state.db.fetch_all(
            sqlx::query(
                r#"
                WITH base_scores AS (
                    SELECT *
                    FROM (
                        WITH targets AS (
                            SELECT UNNEST($2::varchar[]) AS account_id
                        ),
                        latest_priority AS (
                            SELECT DISTINCT ON (member_id)
                                member_id,
                                alignment::float8 AS alignment,
                                trust::float8 AS trust,
                                contribution::float8 AS contribution,
                                advocacy::float8 AS advocacy,
                                judgment::float8 AS judgment
                            FROM community_members
                            WHERE priority = true AND member_id = ANY($2::varchar[])
                            ORDER BY member_id, COALESCE(updated_at, created_at) DESC, id DESC
                        ),
                        regular_scores AS (
                            SELECT
                                member_id,
                                AVG(alignment)::float8 AS alignment,
                                AVG(trust)::float8 AS trust,
                                AVG(contribution)::float8 AS contribution,
                                AVG(advocacy)::float8 AS advocacy,
                                AVG(judgment)::float8 AS judgment
                            FROM community_members
                            WHERE priority = false AND member_id = ANY($2::varchar[])
                            GROUP BY member_id
                        ),
                        seniority AS (
                            SELECT member_id, MIN(rootstock_joined_at) AS rootstock_joined_at
                            FROM community_members
                            WHERE member_id = ANY($2::varchar[])
                            GROUP BY member_id
                        )
                        SELECT
                            t.account_id,
                            COALESCE(
                                CASE
                                    WHEN p.member_id IS NOT NULL AND r.member_id IS NOT NULL THEN (p.alignment + r.alignment) / 2
                                    WHEN p.member_id IS NOT NULL THEN p.alignment
                                    ELSE r.alignment
                                END,
                                5.0
                            ) AS alignment_score,
                            COALESCE(
                                CASE
                                    WHEN p.member_id IS NOT NULL AND r.member_id IS NOT NULL THEN (p.trust + r.trust) / 2
                                    WHEN p.member_id IS NOT NULL THEN p.trust
                                    ELSE r.trust
                                END,
                                5.0
                            ) AS trust_score,
                            COALESCE(
                                CASE
                                    WHEN p.member_id IS NOT NULL AND r.member_id IS NOT NULL THEN (p.contribution + r.contribution) / 2
                                    WHEN p.member_id IS NOT NULL THEN p.contribution
                                    ELSE r.contribution
                                END,
                                5.0
                            ) AS contribution_score,
                            COALESCE(
                                CASE
                                    WHEN p.member_id IS NOT NULL AND r.member_id IS NOT NULL THEN (p.advocacy + r.advocacy) / 2
                                    WHEN p.member_id IS NOT NULL THEN p.advocacy
                                    ELSE r.advocacy
                                END,
                                5.0
                            ) AS advocacy_score,
                            COALESCE(
                                CASE
                                    WHEN p.member_id IS NOT NULL AND r.member_id IS NOT NULL THEN (p.judgment + r.judgment) / 2
                                    WHEN p.member_id IS NOT NULL THEN p.judgment
                                    ELSE r.judgment
                                END,
                                5.0
                            ) AS judgment_score,
                            COALESCE(s.rootstock_joined_at, $3) AS rootstock_joined_at
                        FROM targets t
                        LEFT JOIN latest_priority p ON p.member_id = t.account_id
                        LEFT JOIN regular_scores r ON r.member_id = t.account_id
                        LEFT JOIN seniority s ON s.member_id = t.account_id
                    ) base
                ),
                local_scores AS (
                    SELECT
                        member_id,
                        alignment::float8 AS alignment_score,
                        trust::float8 AS trust_score,
                        contribution::float8 AS contribution_score,
                        advocacy::float8 AS advocacy_score,
                        judgment::float8 AS judgment_score,
                        rootstock_joined_at
                    FROM community_members
                    WHERE account_id = $1
                      AND member_id = ANY($2::varchar[])
                )
                SELECT
                    b.account_id,
                    COALESCE(l.alignment_score, b.alignment_score) AS alignment_score,
                    COALESCE(l.trust_score, b.trust_score) AS trust_score,
                    COALESCE(l.contribution_score, b.contribution_score) AS contribution_score,
                    COALESCE(l.advocacy_score, b.advocacy_score) AS advocacy_score,
                    COALESCE(l.judgment_score, b.judgment_score) AS judgment_score,
                    COALESCE(l.rootstock_joined_at, b.rootstock_joined_at) AS rootstock_joined_at
                FROM base_scores b
                LEFT JOIN local_scores l ON l.member_id = b.account_id
                "#,
            )
            .bind(creator_account_id)
            .bind(target_account_ids)
            .bind(default_joined_at)
            .map(|row: sqlx::postgres::PgRow| CreatorCommunityMemberScoreRow {
                account_id: row.get("account_id"),
                alignment_score: row.get("alignment_score"),
                trust_score: row.get("trust_score"),
                contribution_score: row.get("contribution_score"),
                advocacy_score: row.get("advocacy_score"),
                judgment_score: row.get("judgment_score"),
                rootstock_joined_at: row.get("rootstock_joined_at"),
            }),
        ).await.map_err(Error::DatabaseError)?;

        Ok(rows
            .into_iter()
            .map(|row| {
                let account_id = row.account_id.clone();
                (account_id.clone(), CreatorCommunityMemberScore::from_row(row))
            })
            .collect())
    }

    pub async fn scorable_targets_for(
        &self,
        scorer_account_id: &str,
        target_account_ids: Option<&[String]>,
    ) -> AsamiResult<HashMap<String, ScorableCommunityMemberTarget>> {
        let rows = self.state.db.fetch_all(
            sqlx::query(
                r#"
                WITH latest_scores AS (
                    SELECT
                        member_id AS target_account_id,
                        MAX(COALESCE(updated_at, created_at)) AS latest_score_at
                    FROM community_members
                    WHERE account_id = $1
                      AND (
                        alignment IS NOT NULL OR trust IS NOT NULL OR contribution IS NOT NULL OR
                        advocacy IS NOT NULL OR judgment IS NOT NULL OR rootstock_joined_at IS NOT NULL
                      )
                    GROUP BY member_id
                ),
                latest_skips AS (
                    SELECT target_account_id, MAX(created_at) AS latest_skip_at
                    FROM score_card_skips
                    WHERE scorer_account_id = $1
                    GROUP BY target_account_id
                ),
                weekly_windows AS (
                    SELECT
                        target_account_id,
                        'weekly_pick'::text AS reason,
                        starts_at AS window_start,
                        ends_at AS scorable_until
                    FROM weekly_score_picks
                    WHERE starts_at <= NOW()
                      AND ends_at > NOW()
                ),
                campaign_windows AS (
                    SELECT
                        member_id AS target_account_id,
                        'campaign_reward'::text AS reason,
                        MAX(created_at) AS window_start,
                        MAX(created_at) + INTERVAL '15 days' AS scorable_until
                    FROM collabs
                    WHERE advertiser_id = $1
                      AND status = 'cleared'
                    GROUP BY member_id
                    HAVING COALESCE(SUM(u256_hex_to_numeric(reward)) / '1e18'::numeric, 0) > 1
                ),
                raw_windows AS (
                    SELECT * FROM weekly_windows
                    UNION ALL
                    SELECT * FROM campaign_windows
                ),
                ranked_windows AS (
                    SELECT
                        account_id,
                        reason,
                        window_start,
                        scorable_until,
                        ROW_NUMBER() OVER (
                            PARTITION BY account_id
                            ORDER BY window_start DESC, scorable_until DESC
                        ) AS rn
                    FROM (
                        SELECT
                            target_account_id AS account_id,
                            reason,
                            window_start,
                            scorable_until
                        FROM raw_windows
                        WHERE scorable_until > NOW()
                    ) candidates
                )
                SELECT
                    w.account_id,
                    w.reason,
                    w.window_start,
                    w.scorable_until,
                    ls.latest_score_at,
                    sk.latest_skip_at
                FROM ranked_windows w
                LEFT JOIN latest_scores ls ON ls.target_account_id = w.account_id
                LEFT JOIN latest_skips sk ON sk.target_account_id = w.account_id
                WHERE w.rn = 1
                  AND w.account_id <> $1
                  AND (ls.latest_score_at IS NULL OR ls.latest_score_at <= NOW() - INTERVAL '30 days')
                  AND (sk.latest_skip_at IS NULL OR sk.latest_skip_at < w.window_start)
                  AND ($2::varchar[] IS NULL OR w.account_id = ANY($2::varchar[]))
                "#,
            )
            .bind(scorer_account_id)
            .bind(target_account_ids)
            .map(|row: sqlx::postgres::PgRow| ScorableCommunityMemberTargetRow {
                account_id: row.get("account_id"),
                reason: row.get("reason"),
                window_start: row.get("window_start"),
                scorable_until: row.get("scorable_until"),
                latest_score_at: row.get("latest_score_at"),
                latest_skip_at: row.get("latest_skip_at"),
            }),
        ).await.map_err(Error::DatabaseError)?;

        Ok(rows
            .into_iter()
            .map(|row| {
                let account_id = row.account_id.clone();
                (account_id.clone(), ScorableCommunityMemberTarget::from_row(row))
            })
            .collect())
    }

    pub async fn scorable_target_for_pair(
        &self,
        scorer_account_id: &str,
        target_account_id: &str,
    ) -> AsamiResult<Option<ScorableCommunityMemberTarget>> {
        Ok(self
            .scorable_targets_for(scorer_account_id, Some(&[target_account_id.to_string()]))
            .await?
            .remove(target_account_id))
    }

    pub async fn ensure_scorable_from_to(
        &self,
        scorer_account_id: &str,
        target_account_id: &str,
        allow_any_target: bool,
    ) -> AsamiResult<()> {
        if scorer_account_id == target_account_id {
            return Err(Error::validation("member_id", "cannot_score_self"));
        }
        if !allow_any_target && self.scorable_target_for_pair(scorer_account_id, target_account_id).await?.is_none() {
            return Err(Error::validation("member_id", "target_account_is_not_scorable"));
        }
        Ok(())
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
        let first_collab_date = all.iter().map(|c| *c.created_at()).min();
        let last_collab_date = all.iter().map(|c| *c.created_at()).max();

        Ok(self
            .update()
            .collabs(collabs)
            .rewards(rewards.encode_hex())
            .first_collab_date(first_collab_date)
            .last_collab_date(last_collab_date)
            .updated_at(Some(Utc::now()))
            .save()
            .await?)
    }
}

impl EffectiveCommunityMemberScore {
    pub fn total_score(&self) -> f64 {
        self.alignment_score
            + self.trust_score
            + self.contribution_score
            + self.advocacy_score
            + self.judgment_score
            + self.rootstock_longevity_score
    }

    fn from_row(row: EffectiveCommunityMemberScoreRow) -> Self {
        Self {
            account_id: row.account_id,
            alignment_score: row.alignment_score,
            trust_score: row.trust_score,
            contribution_score: row.contribution_score,
            advocacy_score: row.advocacy_score,
            judgment_score: row.judgment_score,
            rootstock_joined_at: row.rootstock_joined_at,
            rootstock_longevity_score: derive_rootstock_longevity_score(row.rootstock_joined_at),
            scores_received: row.scores_received,
            scores_issued: row.scores_issued,
        }
    }
}

impl CreatorCommunityMemberScore {
    pub fn total_score(&self) -> f64 {
        self.alignment_score
            + self.trust_score
            + self.contribution_score
            + self.advocacy_score
            + self.judgment_score
            + self.rootstock_longevity_score
    }

    fn from_row(row: CreatorCommunityMemberScoreRow) -> Self {
        Self {
            account_id: row.account_id,
            alignment_score: row.alignment_score,
            trust_score: row.trust_score,
            contribution_score: row.contribution_score,
            advocacy_score: row.advocacy_score,
            judgment_score: row.judgment_score,
            rootstock_joined_at: row.rootstock_joined_at,
            rootstock_longevity_score: derive_rootstock_longevity_score(row.rootstock_joined_at),
        }
    }
}

impl ScorableCommunityMemberTarget {
    fn from_row(row: ScorableCommunityMemberTargetRow) -> Self {
        Self {
            account_id: row.account_id,
            reason: row.reason,
            window_start: row.window_start,
            scorable_until: row.scorable_until,
            latest_score_at: row.latest_score_at,
            latest_skip_at: row.latest_skip_at,
        }
    }
}

fn default_rootstock_joined_at() -> UtcDateTime {
    Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap()
}

fn derive_rootstock_longevity_score(joined_at: UtcDateTime) -> f64 {
    let rootstock_origin = Utc.with_ymd_and_hms(2018, 1, 3, 0, 0, 0).unwrap();
    let now = Utc::now();
    let total_window = (now - rootstock_origin).num_seconds().max(1) as f64;
    let seniority_window =
        (now - joined_at).num_seconds().clamp(0, now.timestamp() - rootstock_origin.timestamp()) as f64;
    (1.0 + (9.0 * (seniority_window / total_window))).clamp(1.0, 10.0)
}
