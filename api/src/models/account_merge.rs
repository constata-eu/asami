use super::*;

model! {
  state: App,
  table: account_merges,
  struct AccountMerge {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(varchar)]
    code: Option<String>,
    #[sqlx_model_hints(account_merge_status, default, op_in)]
    status: AccountMergeStatus,
    #[sqlx_model_hints(timestamptz, default, op_gt)]
    created_at: UtcDateTime,
    #[sqlx_model_hints(varchar)]
    destination_id: String,
    #[sqlx_model_hints(varchar, default)]
    source_id: Option<String>,
  },
}

make_sql_enum![
    "account_merge_status",
    pub enum AccountMergeStatus {
        Pending,
        Done,
        Abandoned,
    }
];

impl AccountMergeHub {
    pub async fn get_or_create(&self, account: &Account) -> AsamiResult<AccountMerge> {
        if account.addr().is_none() {
            return Err(Error::validation("addr", "merge_source_must_have_a_wallet"));
        }

        let maybe_existing =
            self.select().source_id_eq(account.id()).status_eq(AccountMergeStatus::Pending).optional().await?;

        if let Some(existing) = maybe_existing {
            return Ok(existing);
        }

        let code = OneTimeTokenHub::rand_value();

        Ok(self
            .insert(InsertAccountMerge {
                code: Some(code),
                destination_id: account.id().clone(),
            })
            .save()
            .await?)
    }

    pub async fn accept_with_code(&self, session: &Session, account: Account, code: String) -> AsamiResult<()> {
        let Some(merge) = self
            .select()
            .code_eq(code.clone())
            .status_eq(AccountMergeStatus::Pending)
            .created_at_gt(Self::active_code_threshold())
            .optional()
            .await?
        else {
            return Err(Error::validation("code", "no_active_code"));
        };

        if session.created_at() < merge.created_at() {
            return Err(Error::validation("session", "session_is_too_old"));
        }

        merge.accept(account).await?;

        Ok(())
    }

    pub fn active_code_threshold() -> DateTime<Utc> {
        Utc::now() - chrono::TimeDelta::minutes(15)
    }
}

impl AccountMerge {
    pub async fn destination_account(&self) -> sqlx::Result<Account> {
        self.state.account().find(self.destination_id()).await
    }

    pub async fn source_account(&self) -> sqlx::Result<Option<Account>> {
        let Some(dest_id) = self.source_id().as_ref() else {
            return Ok(None);
        };

        Ok(Some(self.state.account().find(dest_id).await?))
    }

    pub async fn accept(&self, source: Account) -> AsamiResult<()> {
        if source.id() == self.destination_id() {
            return Err(Error::validation("source_id", "cannot_merge_to_yourself"));
        }

        let tx = self.state.transactional().await?;

        tx.db
            .execute(sqlx::query!(
                "UPDATE handles SET account_id = $1 WHERE account_id = $2",
                self.destination_id(),
                source.id()
            ))
            .await?;
        tx.db
            .execute(sqlx::query!(
                "UPDATE sessions SET logged_out_at = now() WHERE account_id = $1",
                source.id()
            ))
            .await?;
        tx.db
            .execute(sqlx::query!(
                "UPDATE account_users SET account_id = $1 WHERE account_id = $2",
                self.destination_id(),
                source.id()
            ))
            .await?;
        tx.db
            .execute(sqlx::query!(
                "UPDATE campaigns SET account_id = $1 WHERE account_id = $2",
                self.destination_id(),
                source.id()
            ))
            .await?;
        tx.db
            .execute(sqlx::query!(
                "UPDATE campaigns SET account_id = $1 WHERE account_id = $2",
                self.destination_id(),
                source.id()
            ))
            .await?;
        tx.db
            .execute(sqlx::query!(
                "UPDATE collabs SET member_id = $1 WHERE member_id = $2",
                self.destination_id(),
                source.id()
            ))
            .await?;
        tx.db
            .execute(sqlx::query!(
                "UPDATE collabs SET advertiser_id = $1 WHERE advertiser_id = $2",
                self.destination_id(),
                source.id()
            ))
            .await?;
        tx.db
            .execute(sqlx::query!(
                "UPDATE on_chain_job_accounts SET account_id = $1 WHERE account_id = $2",
                self.destination_id(),
                source.id()
            ))
            .await?;
        tx.db
            .execute(sqlx::query!(
                "UPDATE campaign_preferences SET account_id = $1 WHERE account_id = $2",
                self.destination_id(),
                source.id()
            ))
            .await?;
        tx.db
            .execute(sqlx::query!(
                "UPDATE community_members SET account_id = $1 WHERE account_id = $2",
                self.destination_id(),
                source.id()
            ))
            .await?;
        tx.db
            .execute(sqlx::query!(
                "UPDATE community_members SET member_id = $1 WHERE member_id = $2",
                self.destination_id(),
                source.id()
            ))
            .await?;

        tx.account()
            .find(source.id())
            .await?
            .update()
            .force_hydrate(true)
            .status(AccountStatus::Banned)
            .name(format!("⟾ {}", self.destination_id()))
            .save()
            .await?;
        tx.account().find(self.destination_id()).await?.update().force_hydrate(true).save().await?;

        tx.account_merge()
            .find(self.id())
            .await?
            .update()
            .status(AccountMergeStatus::Done)
            .source_id(Some(source.attrs.id))
            .save()
            .await?;

        tx.commit().await?;

        Ok(())
    }
}
