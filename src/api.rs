use std::sync::Arc;

use axum::{
    body::Body,
    extract::{Query, State},
    http::{header, StatusCode, Uri},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};
use sqlx::Row;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::App;

#[derive(RustEmbed)]
#[folder = "pwa/dist/"]
struct Frontend;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Member {
    id: String,
    name: String,
    address: String,
    score: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Asset {
    address: String,
    minimum_amount: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct LogEntry {
    id: i32,
    severity: String,
    kind: String,
    subkind: String,
    message: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize)]
struct LogQuery {
    after_id: Option<i32>,
    limit: Option<i64>,
}

pub async fn serve(app: App) -> anyhow::Result<()> {
    let address = app.settings.listen_address;
    let router = Router::new()
        .route("/members", get(members))
        .route("/assets", get(assets))
        .route("/config", get(config))
        .route("/logs", get(logs))
        .route("/health", get(|| async { "ok" }))
        .route("/ready", get(ready))
        .fallback(get(frontend))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(Arc::new(app));
    let listener = tokio::net::TcpListener::bind(address).await?;
    tracing::info!(%address, "serving API and frontend");
    axum::serve(listener, router).await?;
    Ok(())
}

async fn members(State(app): State<Arc<App>>) -> Result<Json<Vec<Member>>, ApiError> {
    let rows = sqlx::query(
        "SELECT member_id::text, name, account, score::text
         FROM current_members
         WHERE chain_id = $1
           AND account <> '0x0000000000000000000000000000000000000000'
           AND score > 0
         ORDER BY score DESC, member_id ASC",
    )
    .bind(app.settings.chain_id as i64)
    .fetch_all(&app.db)
    .await?;
    Ok(Json(
        rows.into_iter()
            .map(|row| Member {
                id: row.get(0),
                name: row.get(1),
                address: row.get(2),
                score: row.get(3),
            })
            .collect(),
    ))
}

async fn assets(State(app): State<Arc<App>>) -> Result<Json<Vec<Asset>>, ApiError> {
    let rows = sqlx::query(
        "SELECT asset_address, minimum_amount::text
         FROM current_asset_configs
         WHERE chain_id = $1 AND enabled
         ORDER BY asset_address",
    )
    .bind(app.settings.chain_id as i64)
    .fetch_all(&app.db)
    .await?;
    Ok(Json(
        rows.into_iter()
            .map(|row| Asset {
                address: row.get(0),
                minimum_amount: row.get(1),
            })
            .collect(),
    ))
}

async fn config(State(app): State<Arc<App>>) -> Json<crate::config::PublicConfig> {
    Json(app.settings.public())
}

async fn logs(State(app): State<Arc<App>>, Query(query): Query<LogQuery>) -> Result<Json<Vec<LogEntry>>, ApiError> {
    let limit = query.limit.unwrap_or(100).clamp(1, 200);
    let after = query.after_id.unwrap_or(0);
    let rows = sqlx::query(
        "SELECT id, severity::text, kind, subkind, message, created_at
         FROM audit_log_entries WHERE id > $1 ORDER BY id ASC LIMIT $2",
    )
    .bind(after)
    .bind(limit)
    .fetch_all(&app.db)
    .await?;
    Ok(Json(
        rows.into_iter()
            .map(|row| LogEntry {
                id: row.get(0),
                severity: row.get(1),
                kind: row.get(2),
                subkind: row.get(3),
                message: row.get(4),
                created_at: row.get(5),
            })
            .collect(),
    ))
}

async fn ready(State(app): State<Arc<App>>) -> StatusCode {
    match sqlx::query("SELECT 1").execute(&app.db).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::SERVICE_UNAVAILABLE,
    }
}

async fn frontend(uri: Uri) -> Response {
    let path = uri.path().trim_start_matches('/');
    let requested = if path.is_empty() { "index.html" } else { path };
    let asset = Frontend::get(requested).or_else(|| Frontend::get("index.html"));
    match asset {
        Some(asset) => {
            let content_type = match requested.rsplit('.').next() {
                Some("js") => "text/javascript; charset=utf-8",
                Some("css") => "text/css; charset=utf-8",
                Some("svg") => "image/svg+xml",
                Some("png") => "image/png",
                _ => "text/html; charset=utf-8",
            };
            (
                StatusCode::OK,
                [(header::CONTENT_TYPE, content_type)],
                Body::from(asset.data),
            )
                .into_response()
        }
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

struct ApiError(anyhow::Error);

impl<E> From<E> for ApiError
where
    E: Into<anyhow::Error>,
{
    fn from(value: E) -> Self {
        Self(value.into())
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        tracing::warn!(
            "API request failed: {}",
            crate::db::sanitize_log_message(&self.0.to_string())
        );
        (StatusCode::INTERNAL_SERVER_ERROR, "request failed").into_response()
    }
}
