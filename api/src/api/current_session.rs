use base64::{engine::general_purpose, Engine as _};
use jwt_simple::prelude::*;
use rocket::{
    self,
    data::{self, Data, FromData, Limits},
    http::Status,
    request::{FromRequest, Outcome, Request},
};
use serde::de::DeserializeOwned;
use validators::traits::ValidateString;

use super::{
    models::{Session, *},
    *,
};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ApiRequestMetadata {
    pub path: String,
    pub method: String,
    pub body_hash: Option<String>,
    pub query_hash: Option<String>,
}

#[derive(Serialize, Debug, thiserror::Error)]
pub enum ApiAuthError {
    #[error("{0}")]
    Fail(String),
    #[error("unexpected_error:{0}")]
    Unexpected(&'static str),
}

impl From<Error> for ApiAuthError {
    fn from(err: Error) -> ApiAuthError {
        match err {
            Error::Validation(field, message) => ApiAuthError::Fail(format!("{field}: {message}")),
            _ => ApiAuthError::Unexpected("some_base_error"),
        }
    }
}

impl From<sqlx::Error> for ApiAuthError {
    fn from(_err: sqlx::Error) -> ApiAuthError {
        ApiAuthError::Unexpected("db_error")
    }
}

macro_rules! auth_try {
    ($expr:expr, $error:literal) => {
        $expr.map_err(|_| ApiAuthError::Fail($error.to_string()))?
    };
}

macro_rules! auth_some {
    ($expr:expr, $error:literal) => {
        $expr.ok_or_else(|| ApiAuthError::Fail($error.to_string()))?
    };
}

macro_rules! auth_assert {
    ($expr:expr, $error:literal) => {
        #[allow(clippy::neg_cmp_op_on_partial_ord)]
        if (!$expr) {
            return Err(ApiAuthError::Fail($error.to_string()));
        }
    };
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct OauthCodeAndVerifier {
    code: String,
    oauth_verifier: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CurrentSession(pub Session);

impl CurrentSession {
    async fn build(req: &Request<'_>, body: Option<&[u8]>) -> Result<Self, ApiAuthError> {
        let app = req
            .rocket()
            .state::<App>()
            .ok_or(ApiAuthError::Unexpected("could_not_make_app_on_current_session"))?
            .transactional()
            .await?;
        let jwt = auth_some!(req.headers().get_one("Authentication"), "no_authentication_header").to_string();

        let session = match req.headers().get_one("Auth-Action") {
            Some("Login") => Self::login(&app, &jwt, req, body).await?,
            _ => Self::identify(&app, &jwt, req, body).await?,
        };

        app.commit().await?;

        Ok(Self(session))
    }

    /// Login creates a session from credentials, and signs up in the process if it needs to.
    async fn login(app: &App, jwt: &str, req: &Request<'_>, body: Option<&[u8]>) -> Result<Session, ApiAuthError> {
        Self::validate_recaptcha(req, app).await?;
        let pubkey = Self::get_login_pubkey(req)?;
        let nonce = Self::validate_jwt(jwt, &pubkey, req, &body).await?;
        let (kind, lookup_key, auth_data, x_username, x_refresh_token) = Self::validate_auth_data(app, req).await?;
        let Outcome::Success(lang) = Lang::from_request(req).await else {
            return Err(ApiAuthError::Unexpected("could_not_retrieve_lang"));
        };

        let maybe_auth_method = app.auth_method().select().kind_eq(kind).lookup_key_eq(&lookup_key).optional().await?;

        let (auth_method, new_account) = match maybe_auth_method {
            Some(method) => (method, None),
            None => {
                let account = auth_try!(
                    app.account().insert(InsertAccount { addr: None, lang }).save().await,
                    "could_not_create_account"
                );
                let account_id = account.attrs.id.clone();

                let user_id = auth_try!(
                    app.user()
                        .insert(InsertUser {
                            name: "user".to_string()
                        })
                        .save()
                        .await,
                    "could_not_create_user"
                )
                .attrs
                .id;

                auth_try!(
                    app.account_user().insert(InsertAccountUser { account_id, user_id }).save().await,
                    "could_not_bind_user_to_account"
                );

                let method = auth_try!(
                    app.auth_method()
                        .insert(InsertAuthMethod {
                            user_id,
                            lookup_key: lookup_key.clone(),
                            kind
                        })
                        .save()
                        .await,
                    "could_not_insert_auth_method"
                );

                (method, Some(account))
            }
        };

        let key_id = hasher::hexdigest(pubkey.as_bytes());

        let maybe_existing = auth_try!(app.session().find_optional(&key_id).await, "could_not_find_session");

        auth_assert!(maybe_existing.is_none(), "session_pubkey_exists");

        let user = auth_method.user().await?;

        let account_id = user.account_id().await?;

        let session = auth_try!(
            app.session()
                .insert(InsertSession {
                    id: key_id,
                    user_id: user.attrs.id,
                    account_id: account_id.clone(),
                    auth_method_id: auth_method.attrs.id,
                    pubkey,
                    nonce,
                    admin: user.attrs.admin,
                })
                .save()
                .await,
            "could_not_insert_session"
        );

        if kind == AuthMethodKind::X {
            if let Some(username) = x_username {
                if let Some(refresh_token) = x_refresh_token {
                    let user_id = auth_method.lookup_key().clone();
                    let existing = app.handle().select().user_id_eq(&user_id).optional().await?;

                    if let Some(h) = existing {
                        h.update().x_refresh_token(Some(refresh_token)).save().await?;
                    } else {
                        app.handle()
                            .setup_with_refresh_token(
                                refresh_token,
                                user_id.to_string(),
                                username.to_string(),
                                account_id,
                            )
                            .await?;
                    };
                }
            }
        }

        if let Some(account) = new_account {
            if kind == AuthMethodKind::Eip712 {
                account.create_claim_account_request(lookup_key, auth_data, session.attrs.id.clone()).await?;
            }
        }

        app.account().hydrate_on_chain_columns_for([session.account_id().clone()].iter()).await?;

        Ok(session)
    }

    /// Identify just validates a current valid active session that was created with login.
    async fn identify(app: &App, jwt: &str, req: &Request<'_>, body: Option<&[u8]>) -> Result<Session, ApiAuthError> {
        let jwt_meta = auth_try!(Token::decode_metadata(jwt), "bad_jwt_metadata");
        let key_id = auth_some!(jwt_meta.key_id(), "no_key_id_in_jwt").to_string();
        let session = auth_try!(app.session().find(key_id).await, "session_for_kid_not_found");
        auth_assert!(session.logged_out_at().is_none(), "session_was_logged_out");
        let nonce = Self::validate_jwt(jwt, &session.attrs.pubkey, req, &body).await?;
        auth_assert!(nonce > session.attrs.nonce, "invalid_nonce");
        Ok(auth_try!(
            session.update().nonce(nonce).save().await,
            "could_not_update_nonce"
        ))
    }

    async fn validate_auth_data(
        app: &App,
        req: &Request<'_>,
    ) -> Result<(AuthMethodKind, String, String, Option<String>, Option<String>), ApiAuthError> {
        let auth_method_kind_string = auth_some!(
            req.headers().get_one("Auth-Method-Kind"),
            "no_auth_method_kind_in_headers"
        );
        let auth_method_kind: AuthMethodKind = auth_some!(
            AuthMethodKind::from_text(auth_method_kind_string),
            "invalid_auth_method_kind"
        );
        let auth_data = auth_some!(req.headers().get_one("Auth-Data"), "no_auth_data_in_headers");

        let (lookup_key, x_username, x_refresh_token) = match auth_method_kind {
            AuthMethodKind::OneTimeToken => {
                let key = auth_try!(
                    app.one_time_token()
                        .select()
                        .expires_at_gt(Utc::now())
                        .used_eq(false)
                        .value_eq(auth_data.to_string())
                        .one()
                        .await,
                    "invalid_one_time_token"
                )
                .update()
                .used(true)
                .save()
                .await?
                .attrs
                .lookup_key;

                (key, None, None)
            }
            AuthMethodKind::X => {
                let client = auth_try!(app.settings.x.oauth_client("/x_login"), "could_not_make_oauth_client");

                let oauth_data: OauthCodeAndVerifier =
                    auth_try!(serde_json::from_str(auth_data), "could_not_parse_oauth_data");
                let auth_code = twitter_v2::oauth2::AuthorizationCode::new(oauth_data.code);
                let verifier = twitter_v2::oauth2::PkceCodeVerifier::new(oauth_data.oauth_verifier);

                let res = client.request_token(auth_code, verifier).await;
                let token = auth_try!(res, "could_not_fetch_oauth_token");
                let refresh_token = auth_some!(
                    token.refresh_token().map(|t| t.secret().clone()),
                    "no_x_refresh_token_on_login"
                );
                let twitter = twitter_v2::TwitterApi::new(token);

                let x = auth_try!(twitter.get_users_me().send().await, "could_not_find_twitter_me");
                let user = auth_some!(x.into_data(), "no_twitter_payload_data");

                (user.id.to_string(), Some(user.username), Some(refresh_token))
            }
            AuthMethodKind::Eip712 => {
                let key = eip_712_sig_to_address(app.settings.rsk.chain_id, auth_data).map_err(ApiAuthError::Fail)?;
                (key, None, None)
            }
        };

        Ok((
            auth_method_kind,
            lookup_key,
            auth_data.to_string(),
            x_username,
            x_refresh_token,
        ))
    }

    async fn validate_recaptcha(req: &Request<'_>, app: &App) -> Result<(), ApiAuthError> {
        let recaptcha_code = auth_some!(
            req.headers().get_one("New-Session-Recaptcha-Code"),
            "no_session_recaptcha_code_for_new_session"
        );

        let recaptcha_token = auth_try!(
            rocket_recaptcha_v3::ReCaptchaToken::parse_str(recaptcha_code),
            "bad_recaptcha_token"
        );

        let recaptcha = req
            .guard::<&rocket::State<rocket_recaptcha_v3::ReCaptcha>>()
            .await
            .success_or_else(|| ApiAuthError::Unexpected("recaptcha"))?;

        if app.settings.recaptcha_threshold == 0.0 {
            return Ok(());
        }

        let verification = auth_try!(
            recaptcha.verify(&recaptcha_token, None).await,
            "recaptcha_verification_failed"
        );

        auth_assert!(
            verification.score > app.settings.recaptcha_threshold,
            "recaptcha_threshold_not_met"
        );

        Ok(())
    }

    fn get_login_pubkey(req: &Request<'_>) -> Result<String, ApiAuthError> {
        let pubkey_pem_base64 = auth_some!(req.headers().get_one("Login-Pubkey"), "no_login_pubkey");
        let pubkey_pem_bytes = auth_try!(
            general_purpose::URL_SAFE_NO_PAD.decode(pubkey_pem_base64),
            "invalid_base64_pubkey"
        );
        Ok(auth_try!(String::from_utf8(pubkey_pem_bytes), "invalid_utf8_pubkey"))
    }

    async fn validate_jwt(
        jwt: &str,
        pubkey: &str,
        req: &Request<'_>,
        body: &Option<&[u8]>,
    ) -> Result<i64, ApiAuthError> {
        let key = auth_try!(ES256PublicKey::from_pem(pubkey), "invalid_pubkey_pem");

        let claims = auth_try!(
            key.verify_token::<ApiRequestMetadata>(jwt, None),
            "invalid_signature_or_token_claims"
        );

        auth_assert!(claims.custom.path == req.uri().path().raw(), "wrong_auth_path");

        auth_assert!(claims.custom.method == req.method().as_str(), "wrong_auth_method");

        if let Some(query) = req.uri().query() {
            let claim_query_hash = auth_some!(claims.custom.query_hash, "jwt_needs_to_send_query_hash");
            let decoded = auth_try!(hex::decode(claim_query_hash), "claim_query_hash_is_not_hex");
            auth_assert!(
                decoded == hasher::digest(query.raw().as_bytes()),
                "claim_query_does_not_match_query"
            );
        }

        if let Some(bytes) = body {
            let claim_body_hash = auth_some!(claims.custom.body_hash, "jwt_needs_to_send_body_hash");
            let decoded = auth_try!(hex::decode(claim_body_hash), "claim_body_hash_is_not_hex");
            auth_assert!(decoded == hasher::digest(bytes), "claim_body_does_not_match_body");
        }

        let nonce_str: String = auth_some!(claims.nonce, "claims_had_no_nonce");
        let nonce = auth_try!(str::parse::<i64>(&nonce_str), "nonce_must_be_an_integer");

        Ok(nonce)
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for CurrentSession {
    type Error = ApiAuthError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match CurrentSession::build(req, None).await {
            Ok(current) => Outcome::Success(current),
            Err(e) => Outcome::Error((Status::Unauthorized, e)),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct CurrentSessionAndJson<T> {
    pub session: Option<CurrentSession>,
    pub json: T,
}

#[rocket::async_trait]
impl<'r, T: DeserializeOwned + std::marker::Send> FromData<'r> for CurrentSessionAndJson<T> {
    type Error = ApiAuthError;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
        use rocket::data::Outcome;
        let limit = req.limits().get("json").unwrap_or(Limits::JSON);

        let body_bytes = match data.open(limit).into_bytes().await {
            Ok(read) if read.is_complete() => read.into_inner(),
            _ => return Outcome::Error((Status::BadRequest, ApiAuthError::Unexpected("no_body_bytes"))),
        };

        match serde_json::from_str(&String::from_utf8_lossy(&body_bytes)) {
            Ok(value) => {
                let session = if req.headers().get_one("Authentication").is_some() {
                    match CurrentSession::build(req, Some(&body_bytes)).await {
                        Ok(session) => Some(session),
                        Err(e) => {
                            let Some(app) = req.rocket().state::<App>() else {
                                return Outcome::Error((
                                    Status::InternalServerError,
                                    ApiAuthError::Unexpected("could_not_make_app_on_current_session"),
                                ));
                            };
                            app.info("authentication", "authentication", &e).await;
                            return Outcome::Error((Status::Unauthorized, e));
                        }
                    }
                } else {
                    None
                };
                Outcome::Success(CurrentSessionAndJson { session, json: value })
            }
            Err(_) => Outcome::Error((Status::BadRequest, ApiAuthError::Unexpected("invalid_body_json"))),
        }
    }
}
