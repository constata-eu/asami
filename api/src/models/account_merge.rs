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
    #[sqlx_model_hints(timestamptz, default)]
    created_at: UtcDateTime,
    #[sqlx_model_hints(varchar)]
    source_id: String,
    #[sqlx_model_hints(varchar, default)]
    destination_id: Option<String>,
    #[sqlx_model_hints(varchar, default)]
    destination_signature: Option<String>,
  },
}

make_sql_enum![
    "account_merge_status",
    pub enum AccountMergeStatus {
        Pending,
        DestinationSigned,
        AllSigned,
        Abandoned,
    }
];

impl AccountMergeHub {
    pub async fn get_or_create(&self, account: &Account) -> AsamiResult<AccountMerge> {
        use rand::{thread_rng, Rng};
        use rand::distributions::Alphanumeric;

        if account.addr().is_none() {
            return Err(Error::validation("addr", "merge_source_must_have_a_wallet"))
        }

        let maybe_existing = self
            .select()
            .source_id_eq(account.id())
            .status_in(vec![AccountMergeStatus::Pending, AccountMergeStatus::DestinationSigned])
            .optional()
            .await?;

        if let Some(existing) = maybe_existing {
            return Ok(existing);
        }

        let code = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(6)
            .collect::<String>()
            .to_uppercase();

        Ok(self.insert(InsertAccountMerge{
            code: Some(code),
            source_id: account.id().clone(),
        }).save().await?)
    }

    pub async fn accept_with_code(&self, code: String, signature: String) -> AsamiResult<()> {
        let Some(merge) = self.select()
            .code_eq(code)
            .status_eq(AccountMergeStatus::Pending)
            .optional()
            .await? else { return Ok(()) };

        Ok(())
    }
}

impl AccountMerge {
    pub async fn source_account(&self) -> sqlx::Result<Account> {
        self.state.account().find(self.source_id()).await
    }

    pub async fn destination_account(&self) -> sqlx::Result<Option<Account>> {
        let Some(dest_id) = self.destination_id().as_ref() else {
            return Ok(None);
        };
       
        Ok(Some(self.state.account().find(dest_id).await?))
    }
}
