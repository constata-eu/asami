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
    description: String,
    #[sqlx_model_hints(text)]
    context: String,
  }
}

impl AuditLogEntryHub {
  pub async fn info<S: serde::Serialize>(&self, kind: &str, subkind: &str, context: S) -> anyhow::Result<AuditLogEntry> {
    self.log(AuditLogSeverity::Info, kind, subkind, context).await
  }

  pub async fn error<S: serde::Serialize>(&self, kind: &str, subkind: &str, context: S) -> anyhow::Result<AuditLogEntry> {
    self.log(AuditLogSeverity::Error, kind, subkind, context).await
  }

  pub async fn log<S: serde::Serialize>(&self, severity: AuditLogSeverity, kind: &str, subkind: &str, context: S) -> anyhow::Result<AuditLogEntry> {
    Ok(self.insert(InsertAuditLogEntry{
      severity,
      kind: kind.to_string(),
      subkind: subkind.to_string(),
      description: String::new(),
      context: serde_json::to_string(&context)?,
    }).save().await?)
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "audit_log_severity", rename_all = "snake_case")]
pub enum AuditLogSeverity {
  Trace,
  Debug,
  Info,
  Warn,
  Error,
}

impl sqlx::postgres::PgHasArrayType for AuditLogSeverity {
  fn array_type_info() -> sqlx::postgres::PgTypeInfo {
    sqlx::postgres::PgTypeInfo::with_name("_audit_log_severity")
  }
}
