use super::*;

model!{
  state: App,
  table: audit_log_entries,
  struct AuditLogEntry {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(audit_log_severity)]
    severity: AuditLogSeverity,
    #[sqlx_model_hints(timestamptz, default)]
    created_at: UtcDateTime,
    #[sqlx_model_hints(varchar)]
    kind: String,
    #[sqlx_model_hints(varchar)]
    subkind: String,
    #[sqlx_model_hints(text)]
    context: String,
    #[sqlx_model_hints(varchar)]
    loggable_type: Option<String>,
    #[sqlx_model_hints(varchar)]
    loggable_id: Option<String>,
  }
}

impl AuditLogEntryHub {
  pub async fn info<S: serde::Serialize>(&self, kind: &str, subkind: &str, context: S) -> sqlx::Result<AuditLogEntry> {
    self.log(AuditLogSeverity::Info, kind, subkind, context, None, None).await
  }

  pub async fn fail<S: serde::Serialize>(&self, kind: &str, subkind: &str, context: S) -> sqlx::Result<AuditLogEntry> {
    self.log(AuditLogSeverity::Fail, kind, subkind, context, None, None).await
  }

  pub async fn log<S: serde::Serialize>(
    &self,
    severity: AuditLogSeverity,
    kind: &str,
    subkind: &str,
    context_obj: S,
    loggable_type: Option<String>,
    loggable_id: Option<String>,
  ) -> sqlx::Result<AuditLogEntry> {
    let context = serde_json::to_string(&context_obj)
      .unwrap_or_else(|reference| serde_json::json![{
        "error": "could not serialize context",
        "reference": reference.to_string(),
      }].to_string());

    self.insert(InsertAuditLogEntry{
      severity,
      context,
      kind: kind.to_string(),
      subkind: subkind.to_string(),
      loggable_type,
      loggable_id,
    }).save().await
  }
}

make_sql_enum![
  "audit_log_severity",
  pub enum AuditLogSeverity {
    Trace,
    Debug,
    Info,
    Warn,
    Fail,
  }
];

#[rocket::async_trait]
pub trait Loggable: Send {
  fn loggable_type(&self) -> String;
  fn loggable_id(&self) -> String;
  fn app(&self) -> &App;

  async fn audit_log_entries(&self) -> sqlx::Result<Vec<AuditLogEntry>> {
    self.app().audit_log_entry().select()
      .loggable_type_eq(self.loggable_type())
      .loggable_id_eq(self.loggable_id())
      .all().await
  }

  async fn info<S: serde::Serialize + Send>(&self, subkind: &str, context: S) -> sqlx::Result<AuditLogEntry> {
    self.log(AuditLogSeverity::Info, subkind, context).await
  }

  async fn fail<S: serde::Serialize + Send>(&self, subkind: &str, context: S) -> sqlx::Result<AuditLogEntry> {
    self.log(AuditLogSeverity::Fail, subkind, context).await
  }

  async fn log<S: serde::Serialize + Send>(&self, severity: AuditLogSeverity, subkind: &str, context: S) -> sqlx::Result<AuditLogEntry> {
    self.app().audit_log_entry()
      .log(severity, &self.loggable_type(), subkind, context, Some(self.loggable_type()), Some(self.loggable_id()))
      .await
  }
}

#[macro_export]
macro_rules! impl_loggable {
  ($model:ident) => (
    impl $crate::models::audit_log_entry::Loggable for $model {
      fn loggable_type(&self) -> String {
        stringify!($model).to_string()
      }

      fn loggable_id(&self) -> String {
        self.id().to_string()
      }

      fn app(&self) -> &App {
        &self.state
      }
    }
  )
}
