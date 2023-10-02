use validators::traits::ValidateString;
use serde::de::DeserializeOwned;
use super::models::{*, Session};
use super::*;
use jwt_simple::prelude::*;
use base64::{Engine as _, engine::general_purpose};

use rocket::{
  self,
  http::Status,
  request::{FromRequest, Outcome, Request},
  data::{self, Data, FromData, Limits},
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
  #[error("unexpected_error")]
  Unexpected,
}

impl From<Error> for ApiAuthError {
  fn from(err: Error) -> ApiAuthError {
    match err {
      Error::Validation( field, message ) => ApiAuthError::Fail(format!("{field}{message}")),
      _ => ApiAuthError::Unexpected,
    }
  }
}

impl From<sqlx::Error> for ApiAuthError {
  fn from(_err: sqlx::Error) -> ApiAuthError {
    ApiAuthError::Unexpected
  }
}

macro_rules! auth_try {
  ($expr:expr, $error:literal) => (
    $expr.map_err(|_| ApiAuthError::Fail($error.to_string()) )?
  )
}

macro_rules! auth_some {
  ($expr:expr, $error:literal) => (
    $expr.ok_or_else(|| ApiAuthError::Fail($error.to_string()) )?
  )
}

macro_rules! auth_assert {
  ($expr:expr, $error:literal) => (
    if (!$expr) {
      return Err(ApiAuthError::Fail($error.to_string()));
    }
  )
}

#[derive(Debug, PartialEq)]
pub struct CurrentSession(pub Session);

impl CurrentSession {
  async fn build(req: &Request<'_>, body: Option<&[u8]>) -> Result<Self, ApiAuthError> {
    let app = req.rocket().state::<App>().ok_or(ApiAuthError::Unexpected)?;
    let jwt = auth_some!(
      req.headers().get_one("Authentication"),
      "no_authentication_header"
    ).to_string();

    let session = match req.headers().get_one("Auth-Action") {
      Some("Signup") => Self::signup(&app, &jwt, req, body).await?,
      Some("Login") => Self::login(&app, &jwt, req, body).await?,
      _ => Self::identify(&app, &jwt,  req, body).await?,
    };

    Ok(Self(session))
  }

  async fn signup(app: &App, jwt: &str, req: &Request<'_>, body: Option<&[u8]>) -> Result<Session, ApiAuthError> {
    Self::validate_recaptcha(req, app).await?;
    let pubkey = Self::get_login_pubkey(req)?;
    let nonce = Self::validate_jwt(&jwt, &pubkey, req, &body).await?;
    let (kind, lookup_key) = Self::validate_auth_data(app, req).await?;

    let account_id = auth_try!(
      app.account().insert(InsertAccount{name:"account".to_string()}).save().await,
      "could_not_create_account"
    ).attrs.id;
    let user_id = auth_try!(
      app.user().insert(InsertUser{name:"user".to_string()}).save().await,
      "could_not_create_user"
    ).attrs.id;
    auth_try!(
      app.account_user().insert(InsertAccountUser{ account_id, user_id }).save().await,
      "could_not_bind_user_to_account"
    );

    let auth_method = auth_try!(
      app.auth_method().insert(InsertAuthMethod{ user_id, lookup_key, kind }).save().await,
      "could_not_insert_auth_method"
    );

    Self::create_session(app, &auth_method, pubkey, nonce).await
  }

  async fn login(app: &App, jwt: &str, req: &Request<'_>, body: Option<&[u8]>) -> Result<Session, ApiAuthError> {
    Self::validate_recaptcha(req, app).await?;
    let pubkey = Self::get_login_pubkey(req)?;
    let nonce = Self::validate_jwt(&jwt, &pubkey, req, &body).await?;
    let (kind, lookup_key) = Self::validate_auth_data(&app, req).await?;

    let auth_method = auth_try!(
      app.auth_method().select().kind_eq(&kind).lookup_key_eq(&lookup_key).one().await,
      "auth_method_never_signed_up"
    );

    Self::create_session(app, &auth_method, pubkey, nonce).await
  }

  async fn identify(app: &App, jwt: &str, req: &Request<'_>, body: Option<&[u8]>) -> Result<Session, ApiAuthError> {
    let jwt_meta = auth_try!(Token::decode_metadata(jwt), "bad_jwt_metadata");
    let key_id = auth_some!(jwt_meta.key_id(), "no_key_id_in_jwt").to_string();
    let session = auth_try!(app.session().find(key_id).await, "session_for_kid_not_found");
    auth_assert!( session.deletion_id().is_none(), "session_was_deleted");
    let nonce = Self::validate_jwt(jwt, &session.attrs.pubkey, req, &body).await?;
    auth_assert!(nonce > session.attrs.nonce as i64, "invalid_nonce");
    Ok(auth_try!(session.update().nonce(nonce).save().await, "could_not_update_nonce"))
  }

  async fn validate_auth_data(app: &App, req: &Request<'_>) -> Result<(AuthMethodKind, String), ApiAuthError> {
    let auth_method_kind_string = auth_some!(
      req.headers().get_one("Auth-Method-Kind"),
      "no_auth_method_kind_in_headers"
    );
    let auth_method_kind: AuthMethodKind = auth_some!(
      AuthMethodKind::from_str(&auth_method_kind_string),
      "invalid_auth_method_kind"
    );
    let auth_data = auth_some!(req.headers().get_one("Auth-Data"), "no_auth_data_in_headers");

    let lookup_key = match auth_method_kind {
      AuthMethodKind::OneTimeToken => {
        let token = auth_try!(
          app.one_time_token().select().used_eq(&false).value_eq(&auth_data.to_string()).one().await,
          "invalid_one_time_token"
        );
        format!("one_time_token:{}", token.attrs.id)
      },
      _ => return Err(ApiAuthError::Fail(format!("auth_method_not_supported_yet"))),
    };

    Ok((auth_method_kind, lookup_key))
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

    let recaptcha = req.guard::<&rocket::State<rocket_recaptcha_v3::ReCaptcha>>().await
      .success_or_else(|| ApiAuthError::Unexpected )?;

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

  pub async fn create_session(app: &App, auth_method: &AuthMethod, pubkey: String, nonce: i64) -> Result<Session, ApiAuthError> {
    let key_id = hasher::hexdigest(&pubkey.as_bytes());

    let maybe_existing = auth_try!(
      app.session().find_optional(&key_id).await,
      "could_not_find_session"
    );

    auth_assert!(maybe_existing.is_none(), "session_pubkey_exists");

    Ok(auth_try!(
      app.session().insert(InsertSession{
        id: key_id,
        user_id: auth_method.attrs.user_id,
        auth_method_id: auth_method.attrs.id,
        pubkey,
        nonce,
      }).save().await,
      "could_not_insert_session"
    ))
  }

  fn get_login_pubkey(req: &Request<'_>) -> Result<String, ApiAuthError> {
    let pubkey_pem_base64 = auth_some!(req.headers().get_one("Login-Pubkey"), "no_login_pubkey");
    let pubkey_pem_bytes = auth_try!(general_purpose::URL_SAFE_NO_PAD.decode(pubkey_pem_base64), "invalid_base64_pubkey");
    Ok(auth_try!(String::from_utf8(pubkey_pem_bytes), "invalid_utf8_pubkey"))
  }

  async fn validate_jwt(jwt: &str, pubkey: &str, req: &Request<'_>, body: &Option<&[u8]>)
    -> Result<i64, ApiAuthError>
  {

    let key = auth_try!(ES256PublicKey::from_pem(pubkey), "invalid_pubkey_pem");

    let claims = auth_try!(
      key.verify_token::<ApiRequestMetadata>(jwt, None),
      "invalid_signature_or_token_claims"
    );

    auth_assert!( claims.custom.path == req.uri().path().raw(), "wrong_auth_path");

    auth_assert!( claims.custom.method == req.method().as_str(), "wrong_auth_method");

    if let Some(query) = req.uri().query() {
      let claim_query_hash = auth_some!(claims.custom.query_hash, "jwt_needs_to_send_query_hash");
      let decoded = auth_try!(hex::decode(claim_query_hash), "claim_query_hash_is_not_hex");
      auth_assert!(decoded == hasher::digest(query.raw().as_bytes()), "claim_query_does_not_match_query");
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
      Err(e) => Outcome::Failure((Status::Unauthorized, e)),
    }
  }
}

#[derive(Debug, PartialEq)]
pub struct CurrentSessionAndJson<T>{
  pub session: CurrentSession,
  pub json: T,
}

#[rocket::async_trait]
impl<'r, T: DeserializeOwned> FromData<'r> for CurrentSessionAndJson<T> {
  type Error = ApiAuthError;

  async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
    use rocket::data::Outcome;
    let limit = req.limits().get("json").unwrap_or(Limits::JSON);

    let body_bytes = match data.open(limit).into_bytes().await {
      Ok(read) if read.is_complete() => read.into_inner(),
      _ => return Outcome::Failure((Status::BadRequest, ApiAuthError::Unexpected)),
    };
    
    match CurrentSession::build(req, Some(&body_bytes)).await {
      Ok(current) => {
        match serde_json::from_str(&String::from_utf8_lossy(&body_bytes)) {
          Ok(value) => Outcome::Success(CurrentSessionAndJson{
            session: current,
            json: value
          }),
          Err(_) => Outcome::Failure((Status::BadRequest, ApiAuthError::Unexpected)),
        }
      },
      Err(e) => Outcome::Failure((Status::Unauthorized, e)),
    }
  }
}
