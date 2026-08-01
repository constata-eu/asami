use super::*;

model! {
  state: App,
  table: score_card_skips,
  struct ScoreCardSkip {
    #[sqlx_model_hints(int4, default, op_in)]
    id: i32,
    #[sqlx_model_hints(varchar)]
    scorer_account_id: String,
    #[sqlx_model_hints(varchar)]
    target_account_id: String,
    #[sqlx_model_hints(text)]
    reason: String,
    #[sqlx_model_hints(timestamptz, default)]
    created_at: UtcDateTime,
  }
}

impl ScoreCardSkipHub {
    pub async fn latest_for_pair(
        &self,
        scorer_account_id: &str,
        target_account_id: &str,
    ) -> AsamiResult<Option<ScoreCardSkip>> {
        Ok(self
            .select()
            .scorer_account_id_eq(scorer_account_id.to_string())
            .target_account_id_eq(target_account_id.to_string())
            .order_by(ScoreCardSkipOrderBy::CreatedAt)
            .desc(true)
            .optional()
            .await?)
    }

    pub async fn create_checked(
        &self,
        input: InsertScoreCardSkip,
        allow_any_target: bool,
    ) -> AsamiResult<ScoreCardSkip> {
        self.state
            .community_member()
            .ensure_scorable_from_to(&input.scorer_account_id, &input.target_account_id, allow_any_target)
            .await?;
        Ok(self.insert(input).save().await?)
    }
}
