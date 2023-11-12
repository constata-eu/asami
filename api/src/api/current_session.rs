use validators::traits::ValidateString;
use serde::de::DeserializeOwned;
use super::models::{*, Session};
use super::*;
use jwt_simple::prelude::*;
use base64::{Engine as _, engine::general_purpose};
use ethers::{
  abi::AbiEncode,
  types::{Signature, transaction::eip712::TypedData}
};
use std::str::FromStr;

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
  #[error("unexpected_error:{0}")]
  Unexpected(&'static str),
}

impl From<Error> for ApiAuthError {
  fn from(err: Error) -> ApiAuthError {
    match err {
      Error::Validation( field, message ) => ApiAuthError::Fail(format!("{field}{message}")),
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

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct OauthCodeAndVerifier {
  code: String,
  oauth_verifier: String,
}

#[derive(serde::Deserialize)]
struct InstagramOauthToken {
  //access_token: String,
  user_id: u64,
}

#[derive(Debug, PartialEq)]
pub struct CurrentSession(pub Session);

impl CurrentSession {
  async fn build(req: &Request<'_>, body: Option<&[u8]>) -> Result<Self, ApiAuthError> {
    let app = req.rocket().state::<App>().ok_or(ApiAuthError::Unexpected("rocket_error"))?;
    let jwt = auth_some!(
      req.headers().get_one("Authentication"),
      "no_authentication_header"
    ).to_string();

    let session = match req.headers().get_one("Auth-Action") {
      Some("Login") => Self::login(&app, &jwt, req, body).await?,
      _ => Self::identify(&app, &jwt,  req, body).await?,
    };

    Ok(Self(session))
  }

  async fn login(app: &App, jwt: &str, req: &Request<'_>, body: Option<&[u8]>) -> Result<Session, ApiAuthError> {
    Self::validate_recaptcha(req, app).await?;
    let pubkey = Self::get_login_pubkey(req)?;
    let nonce = Self::validate_jwt(&jwt, &pubkey, req, &body).await?;
    let (kind, lookup_key) = Self::validate_auth_data(app, req).await?;

    let maybe_auth_method = app.auth_method().select().kind_eq(&kind).lookup_key_eq(&lookup_key).optional().await?;

    let auth_method = match maybe_auth_method {
      Some(method) => method,
      None => {
        let account_id = auth_try!(
          app.account().insert(InsertAccount{
            name: Some("account".to_string()),
            addr: None,
            unclaimed_asami_tokens: u("0").encode_hex(),
            unclaimed_doc_rewards: u("0").encode_hex(),
            nostr_self_managed: false,
            nostr_abuse_proven: false
          }).save().await,
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

        auth_try!(
          app.auth_method().insert(InsertAuthMethod{ user_id, lookup_key, kind }).save().await,
          "could_not_insert_auth_method"
        )
      }
    };

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
      AuthMethodKind::X => {
        let oauth_data: OauthCodeAndVerifier = auth_try!(
          serde_json::from_str(&auth_data),
          "could_not_parse_oauth_data"
        );

        let verifier = twitter_v2::oauth2::PkceCodeVerifier::new(oauth_data.oauth_verifier);
        let cb_url = auth_try!( app.settings.x.redirect_uri.parse(), "could_not_parse_cb_url");

        let client = twitter_v2::authorization::Oauth2Client::new(
          app.settings.x.client_id.clone(),
          app.settings.x.client_secret.clone(),
          cb_url
        );

        let auth_code = twitter_v2::oauth2::AuthorizationCode::new(oauth_data.code);
        let res = client.request_token(auth_code, verifier).await;
        let token = auth_try!( res, "could_not_fetch_oauth_token");
        let twitter = twitter_v2::TwitterApi::new(token);

        let x = auth_try!(
          twitter.get_users_me().send().await,
          "could_not_find_twitter_me"
        );
        
        format!("{}", auth_some!(x.payload().data.as_ref(), "no_twitter_payload_data").id)
      },
      AuthMethodKind::Instagram => {
        let result = ureq::post("https://api.instagram.com/oauth/access_token")
          .send_form(&[
            ("client_id", &app.settings.instagram.client_id),
            ("client_secret", &app.settings.instagram.client_secret),
            ("grant_type", "authorization_code"),
            ("redirect_uri", &app.settings.instagram.redirect_uri),
            ("code", auth_data)
          ]);

        let response = auth_try!(result, "could_not_request_instagram_token");
        auth_try!(response.into_json::<InstagramOauthToken>(), "instagram_token_was_not_json").user_id.to_string()
      },
      AuthMethodKind::Eip712 => {
        let json = serde_json::json!( {
          "types": {
            "EIP712Domain": [
              { "name": "name", "type": "string" },
              { "name": "version", "type": "string" },
              { "name": "chainId", "type": "uint256" }
            ],
            "Acceptance": [
              { "name": "content", "type": "string" }
            ]
          },
          "primaryType": "Acceptance",
          "domain": { "name": "Asami", "version": "1", "chainId": app.settings.rsk.chain_id.to_string() },
          "message": { "content": "Login to Asami" }
        });

        let payload: TypedData = auth_try!(serde_json::from_value(json), "unexpected_invalid_json");
        let sig = auth_try!(Signature::from_str(auth_data), "invalid_auth_data_signature");
        auth_try!(sig.recover_typed_data(&payload), "could_not_recover_typed_data_from_sig").to_string()
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
      .success_or_else(|| ApiAuthError::Unexpected("recaptcha") )?;

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
      _ => return Outcome::Failure((Status::BadRequest, ApiAuthError::Unexpected("no_body_bytes"))),
    };
    
    match CurrentSession::build(req, Some(&body_bytes)).await {
      Ok(current) => {
        match serde_json::from_str(&String::from_utf8_lossy(&body_bytes)) {
          Ok(value) => Outcome::Success(CurrentSessionAndJson{
            session: current,
            json: value
          }),
          Err(_) => Outcome::Failure((Status::BadRequest, ApiAuthError::Unexpected("invalid_body_json"))),
        }
      },
      Err(e) => Outcome::Failure((Status::Unauthorized, e)),
    }
  }
}
