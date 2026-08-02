use regex::Regex;
use sqlx::{PgPool, Row};

pub async fn migrate(pool: &PgPool) -> anyhow::Result<()> {
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}

pub async fn log(
    pool: &PgPool,
    severity: &str,
    kind: &str,
    subkind: &str,
    message: &str,
    chain_id: Option<u64>,
    entity: Option<(&str, String)>,
) {
    let message = sanitize_log_message(message);
    let (entity_type, entity_id) = entity.map(|(kind, id)| (Some(kind.to_string()), Some(id))).unwrap_or_default();
    let result = sqlx::query(
        "INSERT INTO audit_log_entries
         (severity, kind, subkind, context, message, chain_id, entity_type, entity_id)
         VALUES ($1::audit_log_severity, $2, $3, '{}', $4, $5, $6, $7)",
    )
    .bind(severity)
    .bind(kind)
    .bind(subkind)
    .bind(message)
    .bind(chain_id.map(|value| value as i64))
    .bind(entity_type)
    .bind(entity_id)
    .execute(pool)
    .await;
    if result.is_err() {
        tracing::warn!(kind, subkind, "could not persist operational log");
    }
}

pub fn sanitize_log_message(input: &str) -> String {
    let compact = input.replace(['\r', '\n', '\t'], " ").split_whitespace().collect::<Vec<_>>().join(" ");
    let secrets = Regex::new(
        r"(?i)(authorization|bearer|access[_ -]?token|refresh[_ -]?token|secret|mnemonic|private[_ -]?key|password)\s*[:=]\s*\S+",
    )
    .expect("static regex");
    let mut sanitized = secrets.replace_all(&compact, "$1=[redacted]").to_string();
    if sanitized.len() > 500 {
        sanitized.truncate(500);
        sanitized.push('…');
    }
    sanitized
}

pub async fn scalar_u256(pool: &PgPool, query: &str, bind: &str) -> anyhow::Result<ethers::types::U256> {
    let row = sqlx::query(query).bind(bind).fetch_one(pool).await?;
    let value: String = row.try_get(0)?;
    Ok(ethers::types::U256::from_dec_str(&value)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn redacts_secret_values_and_lines() {
        let value = sanitize_log_message("authorization=Bearer123\nprivate_key: abc");
        assert_eq!(value, "authorization=[redacted] private_key=[redacted]");
    }
}
