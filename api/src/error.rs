use juniper::{FieldError, IntoFieldError, ScalarValue, graphql_value};
use std::error::Error as ErrorTrait;
use rocket::{
  http::Status,
  request::Request,
  response::{self, Responder},
  serde::json::{json, Json},
  warn,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("Unexpected error at initialization {0}")]
  Init(String),
  #[error(transparent)]
  DatabaseError(sqlx::Error),
  #[error("Unexpected Precondition error. Happened at runtime but may have to do with a wrong config. {0}")]
  Precondition(String),
  #[error("Unexpected error working with third party service {0}: {1}")]
  Service(String, String),
  #[error("Invalid input on {0}: {1}")]
  Validation(String, String),
}

impl From<rocket::figment::Error> for Error {
  fn from(err: rocket::figment::Error) -> Error {
    Error::Init(err.to_string())
  }
}

impl From<ethers::signers::WalletError> for Error {
  fn from(err: ethers::signers::WalletError) -> Error {
    Error::Init("Invalid mnemonic for rsk signer wallet".to_string())
  }
}

impl From<sqlx::Error> for Error {
  fn from(err: sqlx::Error) -> Error {
    Error::Init(err.to_string())
  }
}

impl From<twitter_v2::Error> for Error {
  fn from(err: twitter_v2::Error) -> Error {
    Error::Service("twitter_api_v2".to_string(), err.to_string())
  }
}

impl From<regex::Error> for Error {
  fn from(err: regex::Error) -> Error {
    Error::Precondition(format!("Error in regex {}", err.to_string()))
  }
}

impl<S: ScalarValue> IntoFieldError<S> for Error {
  fn into_field_error(self) -> FieldError<S> {
    match &self {
      Error::Validation(field, message) => FieldError::new(
        self.to_string(),
        graphql_value!({ "error": { "field": field.as_str(), "message": message.as_str()} }),
      ),
      Error::Service(service, _) => FieldError::new(
        "error_in_third_party_service".to_string(),
        graphql_value!({ "error": { "field": "third_party_service", "message": service.as_str() } }),
      ),
      _ => {
        warn!(
          "A wild error appeared: {:?}\n\n{:?}\n",
          &self,
          &self.source()
        );
        FieldError::new( "unexpected error", graphql_value!(None))
      }
    }
  }
}

impl<'r> Responder<'r, 'static> for Error {
  fn respond_to(self, request: &'r Request<'_>) -> response::Result<'static> {
    let response = match self {
      Error::Validation( field, message ) => (
        Status::UnprocessableEntity,
        Json(json![{"error": { "field": field, "message": message}}]),
      ),
      Error::DatabaseError(sqlx::Error::RowNotFound) => {
        (Status::NotFound, Json(json![{ "error": "Not found" }]))
      }
      _ => {
        warn!(
          "A wild error appeared: {:?}\n\n{:?}\n",
          &self,
          &self.source()
        );
        (
          Status::InternalServerError,
          Json(json![{ "error": "Unexpected Error" }]),
        )
      }
    };

    response.respond_to(request)
  }
}

pub type AsamiResult<T> = Result<T, Error>;