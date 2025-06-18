use super::{
    models::{self, *},
    *,
};

#[derive(Debug, GraphQLObject, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[graphql(description = "An message logged for any part of the system")]
pub struct AuditLogEntry {
    #[graphql(description = "Unique numeric identifier of this resource")]
    id: i32,
    severity: AuditLogSeverity,
    created_at: UtcDateTime,
    kind: String,
    subkind: String,
    context: String,
    loggable_type: Option<String>,
    loggable_id: Option<String>,
}

#[derive(Debug, Clone, Default, GraphQLInputObject, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditLogEntryFilter {
    ids: Option<Vec<i32>>,
    id_eq: Option<i32>,
}

#[rocket::async_trait]
impl Showable<models::AuditLogEntry, AuditLogEntryFilter> for AuditLogEntry {
    fn sort_field_to_order_by(field: &str) -> Option<models::AuditLogEntryOrderBy> {
        match field {
            "id" => Some(AuditLogEntryOrderBy::Id),
            "createdAt" => Some(AuditLogEntryOrderBy::CreatedAt),
            _ => None,
        }
    }

    fn filter_to_select(
        _context: &Context,
        filter: Option<AuditLogEntryFilter>,
    ) -> FieldResult<models::SelectAuditLogEntry> {
        if let Some(f) = filter {
            Ok(models::SelectAuditLogEntry {
                id_in: f.ids,
                id_eq: f.id_eq,
                ..Default::default()
            })
        } else {
            Ok(Default::default())
        }
    }

    fn select_by_id(_context: &Context, id: i32) -> FieldResult<models::SelectAuditLogEntry> {
        Ok(models::SelectAuditLogEntry {
            id_eq: Some(id),
            ..Default::default()
        })
    }

    async fn db_to_graphql(_context: &Context, d: models::AuditLogEntry) -> AsamiResult<Self> {
        Ok(AuditLogEntry {
            id: d.attrs.id,
            severity: d.attrs.severity,
            created_at: d.attrs.created_at,
            kind: d.attrs.kind,
            subkind: d.attrs.subkind,
            context: d.attrs.context,
            loggable_type: d.attrs.loggable_type,
            loggable_id: d.attrs.loggable_id,
        })
    }
}
