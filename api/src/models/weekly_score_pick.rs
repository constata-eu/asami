use super::*;

model! {
  state: App,
  table: weekly_score_picks,
  struct WeeklyScorePick {
    #[sqlx_model_hints(int4, default, op_in, op_not_in)]
    id: i32,
    #[sqlx_model_hints(varchar)]
    target_account_id: String,
    #[sqlx_model_hints(varchar, default, op_is_set)]
    created_by_account_id: Option<String>,
    #[sqlx_model_hints(timestamptz)]
    starts_at: UtcDateTime,
    #[sqlx_model_hints(timestamptz)]
    ends_at: UtcDateTime,
    #[sqlx_model_hints(timestamptz, default)]
    created_at: UtcDateTime,
  }
}

impl WeeklyScorePickHub {
    pub fn current_window(now: UtcDateTime) -> (UtcDateTime, UtcDateTime) {
        let weekday = i64::from(now.weekday().num_days_from_monday());
        let start = (now - Duration::days(weekday)).date_naive().and_hms_opt(0, 0, 0).expect("valid midnight");
        let starts_at = Utc.from_utc_datetime(&start);
        let ends_at = starts_at + Duration::days(7);
        (starts_at, ends_at)
    }

    pub async fn current(&self, now: UtcDateTime) -> AsamiResult<Vec<WeeklyScorePick>> {
        let (starts_at, ends_at) = Self::current_window(now);
        Ok(self
            .select()
            .starts_at_eq(starts_at)
            .ends_at_eq(ends_at)
            .order_by(WeeklyScorePickOrderBy::Id)
            .all()
            .await?)
    }

    pub async fn ensure_current(&self, desired_count: usize) -> AsamiResult<Vec<WeeklyScorePick>> {
        let now = Utc::now();
        let (starts_at, ends_at) = Self::current_window(now);
        let mut current = self.current(now).await?;

        if current.len() >= desired_count {
            return Ok(current);
        }

        let existing_targets: Vec<String> = current.iter().map(|pick| pick.target_account_id().clone()).collect();

        let candidates = self
            .state
            .account()
            .select()
            .acceptance_status_eq(AcceptanceStatus::Active)
            .use_struct(SelectAccount {
                id_not_in: if existing_targets.is_empty() {
                    None
                } else {
                    Some(existing_targets)
                },
                ..Default::default()
            })
            .order_by(AccountOrderBy::CreatedAt)
            .limit((desired_count - current.len()).try_into()?)
            .all()
            .await?;

        for account in candidates {
            current.push(
                self.insert(InsertWeeklyScorePick {
                    target_account_id: account.id().clone(),
                    starts_at,
                    ends_at,
                })
                .save()
                .await?,
            );
        }

        Ok(current)
    }
}
