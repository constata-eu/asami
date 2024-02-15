use super::*;

model! {
  state: App,
  table: users,
  struct User {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    name: String,
    #[sqlx_model_hints(timestamptz, default)]
    created_at: UtcDateTime,
    #[sqlx_model_hints(timestamptz, default)]
    updated_at: Option<UtcDateTime>,
  },
  has_many {
    AccountUser(user_id)
  }
}

impl User {
  pub async fn account_id(&self) -> AsamiResult<String> {
    let accounts = self.account_user_vec().await?;
    let Some(first) = accounts.first() else {
       return Err(Error::Runtime(format!("user has no account {}", &self.attrs.id)))
    };

    Ok(first.attrs.account_id.clone())
  }
}
