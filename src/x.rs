use base64::{engine::general_purpose, Engine};
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    XChaCha20Poly1305, XNonce,
};
use chrono::{Duration, Utc};
use rand::{distributions::Alphanumeric, Rng, RngCore};
use serde::Deserialize;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use sqlx::Row;
use url::Url;

use crate::{db, App};

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    refresh_token: Option<String>,
    expires_in: i64,
}

pub async fn authorization_url(app: &App, account_key: &str) -> anyhow::Result<String> {
    validate_account(account_key)?;
    let client_id = app.settings.x_client_id.as_deref().ok_or_else(|| anyhow::anyhow!("X_CLIENT_ID is required"))?;
    let redirect = app
        .settings
        .x_redirect_uri
        .as_deref()
        .ok_or_else(|| anyhow::anyhow!("X_REDIRECT_URI is required"))?;
    let verifier: String = rand::thread_rng().sample_iter(&Alphanumeric).take(64).map(char::from).collect();
    let state: String = rand::thread_rng().sample_iter(&Alphanumeric).take(40).map(char::from).collect();
    let challenge = general_purpose::URL_SAFE_NO_PAD.encode(Sha256::digest(verifier.as_bytes()));
    let (nonce, ciphertext) = encrypt(app, verifier.as_bytes())?;
    sqlx::query(
        "INSERT INTO x_oauth_states
         (state, account_key, verifier_nonce, verifier_ciphertext, expires_at)
         VALUES ($1, $2, $3, $4, now() + interval '15 minutes')",
    )
    .bind(&state)
    .bind(account_key)
    .bind(nonce)
    .bind(ciphertext)
    .execute(&app.db)
    .await?;

    let mut url = Url::parse("https://x.com/i/oauth2/authorize")?;
    url.query_pairs_mut()
        .append_pair("response_type", "code")
        .append_pair("client_id", client_id)
        .append_pair("redirect_uri", redirect)
        .append_pair("scope", "tweet.read tweet.write users.read offline.access")
        .append_pair("state", &state)
        .append_pair("code_challenge", &challenge)
        .append_pair("code_challenge_method", "S256");
    Ok(url.to_string())
}

pub async fn complete_authorization(app: &App, account_key: &str, redirected_url: &str) -> anyhow::Result<()> {
    validate_account(account_key)?;
    let url = Url::parse(redirected_url)?;
    let params: std::collections::HashMap<_, _> = url.query_pairs().into_owned().collect();
    let state = params.get("state").ok_or_else(|| anyhow::anyhow!("missing OAuth state"))?;
    let code = params.get("code").ok_or_else(|| anyhow::anyhow!("missing OAuth code"))?;
    let row = sqlx::query(
        "DELETE FROM x_oauth_states
         WHERE state = $1 AND account_key = $2 AND expires_at > now()
         RETURNING verifier_nonce, verifier_ciphertext",
    )
    .bind(state)
    .bind(account_key)
    .fetch_optional(&app.db)
    .await?
    .ok_or_else(|| anyhow::anyhow!("OAuth state is invalid or expired"))?;
    let verifier = String::from_utf8(decrypt(app, row.get(0), row.get(1))?)?;
    let tokens = exchange_code(app, code, &verifier).await?;
    store_tokens(app, account_key, &tokens, None).await?;
    Ok(())
}

async fn exchange_code(app: &App, code: &str, verifier: &str) -> anyhow::Result<TokenResponse> {
    let client_id = app.settings.x_client_id.as_deref().unwrap();
    let redirect = app.settings.x_redirect_uri.as_deref().unwrap();
    let form = [
        ("code", code),
        ("grant_type", "authorization_code"),
        ("client_id", client_id),
        ("redirect_uri", redirect),
        ("code_verifier", verifier),
    ];
    let mut request = app.http.post("https://api.x.com/2/oauth2/token").form(&form);
    if let Some(secret) = &app.settings.x_client_secret {
        request = request.basic_auth(client_id, Some(secret));
    }
    let response = request.send().await?;
    if !response.status().is_success() {
        anyhow::bail!("X authorization exchange failed with status {}", response.status());
    }
    Ok(response.json().await?)
}

async fn refresh(app: &App, account_key: &str, refresh_token: &str) -> anyhow::Result<TokenResponse> {
    let client_id = app.settings.x_client_id.as_deref().unwrap();
    let form = [
        ("refresh_token", refresh_token),
        ("grant_type", "refresh_token"),
        ("client_id", client_id),
    ];
    let mut request = app.http.post("https://api.x.com/2/oauth2/token").form(&form);
    if let Some(secret) = &app.settings.x_client_secret {
        request = request.basic_auth(client_id, Some(secret));
    }
    let response = request.send().await?;
    if !response.status().is_success() {
        anyhow::bail!("X token refresh failed with status {}", response.status());
    }
    let tokens: TokenResponse = response.json().await?;
    store_tokens(app, account_key, &tokens, Some(refresh_token)).await?;
    Ok(tokens)
}

async fn store_tokens(
    app: &App,
    account_key: &str,
    tokens: &TokenResponse,
    previous_refresh: Option<&str>,
) -> anyhow::Result<()> {
    let refresh = tokens
        .refresh_token
        .as_deref()
        .or(previous_refresh)
        .ok_or_else(|| anyhow::anyhow!("X did not return a refresh token"))?;
    let (access_nonce, access_ciphertext) = encrypt(app, tokens.access_token.as_bytes())?;
    let (refresh_nonce, refresh_ciphertext) = encrypt(app, refresh.as_bytes())?;
    let username = "@asami_club";
    sqlx::query(
        "INSERT INTO x_accounts
         (account_key, username, access_token_nonce, access_token_ciphertext,
          refresh_token_nonce, refresh_token_ciphertext, expires_at)
         VALUES ($1, $2, $3, $4, $5, $6, now() + make_interval(secs => $7::double precision))
         ON CONFLICT(account_key) DO UPDATE SET
           username = EXCLUDED.username,
           access_token_nonce = EXCLUDED.access_token_nonce,
           access_token_ciphertext = EXCLUDED.access_token_ciphertext,
           refresh_token_nonce = EXCLUDED.refresh_token_nonce,
           refresh_token_ciphertext = EXCLUDED.refresh_token_ciphertext,
           expires_at = EXCLUDED.expires_at,
           updated_at = now()",
    )
    .bind(account_key)
    .bind(username)
    .bind(access_nonce)
    .bind(access_ciphertext)
    .bind(refresh_nonce)
    .bind(refresh_ciphertext)
    .bind(tokens.expires_in)
    .execute(&app.db)
    .await?;
    Ok(())
}

pub async fn post_one(app: &App) -> anyhow::Result<bool> {
    sqlx::query(
        "UPDATE tweet_queue
         SET status = 'retryable', last_error = 'worker_lease_expired',
             not_before = now(), updated_at = now()
         WHERE status = 'sending'
           AND updated_at < now() - interval '15 minutes'",
    )
    .execute(&app.db)
    .await?;
    let mut tx = app.db.begin().await?;
    let row = match sqlx::query(
        "SELECT id, account_key, rendered_text
         FROM tweet_queue
         WHERE status IN ('ready', 'retryable')
           AND not_before <= now()
         ORDER BY id
         FOR UPDATE SKIP LOCKED
         LIMIT 1",
    )
    .fetch_optional(&mut *tx)
    .await?
    {
        Some(row) => row,
        None => {
            tx.commit().await?;
            return Ok(false);
        }
    };
    let id: i64 = row.get(0);
    let account_key: String = row.get(1);
    let text: String = row.get(2);
    let sent_today: i64 = sqlx::query(
        "SELECT COUNT(*) FROM tweet_queue
         WHERE account_key = $1 AND status = 'sent'
           AND sent_at >= date_trunc('day', now() AT TIME ZONE 'UTC') AT TIME ZONE 'UTC'",
    )
    .bind(&account_key)
    .fetch_one(&mut *tx)
    .await?
    .get(0);
    if sent_today >= app.settings.x_daily_post_cap {
        sqlx::query(
            "UPDATE tweet_queue SET
             not_before = date_trunc('day', now() AT TIME ZONE 'UTC') AT TIME ZONE 'UTC' + interval '1 day',
             updated_at = now()
             WHERE id = $1",
        )
        .bind(id)
        .execute(&mut *tx)
        .await?;
        tx.commit().await?;
        return Ok(false);
    }
    sqlx::query(
        "UPDATE tweet_queue SET status = 'sending', attempt_count = attempt_count + 1,
         updated_at = now() WHERE id = $1",
    )
    .bind(id)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;

    match access_token(app, &account_key).await {
        Ok(token) => {
            let response = app
                .http
                .post("https://api.x.com/2/tweets")
                .bearer_auth(token)
                .json(&json!({"text": text}))
                .send()
                .await;
            match response {
                Ok(response) if response.status().is_success() => {
                    let body: Value = response.json().await?;
                    let post_id = body
                        .pointer("/data/id")
                        .and_then(Value::as_str)
                        .ok_or_else(|| anyhow::anyhow!("X response did not include post id"))?;
                    sqlx::query(
                        "UPDATE tweet_queue SET status = 'sent', x_post_id = $2,
                         sent_at = now(), updated_at = now(), last_error = NULL WHERE id = $1",
                    )
                    .bind(id)
                    .bind(post_id)
                    .execute(&app.db)
                    .await?;
                    db::log(
                        &app.db,
                        "info",
                        "tweet",
                        "sent",
                        "Queued notification posted successfully.",
                        Some(app.settings.chain_id),
                        Some(("tweet_queue", id.to_string())),
                    )
                    .await;
                    Ok(true)
                }
                Ok(response) => {
                    retry_later(app, id, &format!("x_http_{}", response.status().as_u16())).await?;
                    Ok(true)
                }
                Err(_) => {
                    retry_later(app, id, "x_transport_error").await?;
                    Ok(true)
                }
            }
        }
        Err(_) => {
            retry_later(app, id, "x_authentication_error").await?;
            Ok(true)
        }
    }
}

async fn retry_later(app: &App, id: i64, category: &str) -> anyhow::Result<()> {
    sqlx::query(
        "UPDATE tweet_queue
         SET status = 'retryable', last_error = $2,
             not_before = now() + interval '15 minutes', updated_at = now()
         WHERE id = $1",
    )
    .bind(id)
    .bind(category)
    .execute(&app.db)
    .await?;
    db::log(
        &app.db,
        "warn",
        "tweet",
        "retryable",
        "Notification delivery failed safely and will be retried.",
        Some(app.settings.chain_id),
        Some(("tweet_queue", id.to_string())),
    )
    .await;
    Ok(())
}

async fn access_token(app: &App, account_key: &str) -> anyhow::Result<String> {
    let row = sqlx::query(
        "SELECT access_token_nonce, access_token_ciphertext,
                refresh_token_nonce, refresh_token_ciphertext, expires_at
         FROM x_accounts WHERE account_key = $1",
    )
    .bind(account_key)
    .fetch_one(&app.db)
    .await?;
    let expires_at: chrono::DateTime<Utc> = row.get(4);
    let access = String::from_utf8(decrypt(app, row.get(0), row.get(1))?)?;
    if expires_at > Utc::now() + Duration::minutes(2) {
        return Ok(access);
    }
    let refresh_token = String::from_utf8(decrypt(app, row.get(2), row.get(3))?)?;
    Ok(refresh(app, account_key, &refresh_token).await?.access_token)
}

fn encrypt(app: &App, plaintext: &[u8]) -> anyhow::Result<(Vec<u8>, Vec<u8>)> {
    let key = app.settings.encryption_key.ok_or_else(|| anyhow::anyhow!("ASAMI_ENCRYPTION_KEY is required"))?;
    let cipher = XChaCha20Poly1305::new((&key).into());
    let mut nonce = [0_u8; 24];
    rand::thread_rng().fill_bytes(&mut nonce);
    let ciphertext = cipher
        .encrypt(XNonce::from_slice(&nonce), plaintext)
        .map_err(|_| anyhow::anyhow!("token encryption failed"))?;
    Ok((nonce.to_vec(), ciphertext))
}

fn decrypt(app: &App, nonce: Vec<u8>, ciphertext: Vec<u8>) -> anyhow::Result<Vec<u8>> {
    let key = app.settings.encryption_key.ok_or_else(|| anyhow::anyhow!("ASAMI_ENCRYPTION_KEY is required"))?;
    if nonce.len() != 24 {
        anyhow::bail!("stored token nonce is invalid");
    }
    let cipher = XChaCha20Poly1305::new((&key).into());
    cipher
        .decrypt(XNonce::from_slice(&nonce), ciphertext.as_ref())
        .map_err(|_| anyhow::anyhow!("stored token decryption failed"))
}

fn validate_account(value: &str) -> anyhow::Result<()> {
    if value == "asami_english" {
        Ok(())
    } else {
        anyhow::bail!("account must be asami_english")
    }
}
