use super::*;
use rocket::{
    self,
    request::{self, FromRequest, Outcome, Request},
};

#[derive(sqlx::Type, Copy, Clone, Debug, serde::Deserialize, PartialEq, serde::Serialize)]
#[sqlx(type_name = "language", rename_all = "lowercase")]
pub enum Lang {
    Es,
    En,
}

impl Lang {
    pub fn code(&self) -> &'static str {
        match self {
            Lang::Es => "es",
            Lang::En => "en",
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Lang {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        Outcome::Success(MaybeLang::from_request_base(req).value.unwrap_or(Lang::En))
    }
}

pub struct MaybeLang {
    pub value: Option<Lang>,
}

impl MaybeLang {
    pub fn from_request_base(r: &Request) -> Self {
        let value = r
            .headers()
            .get_one("Accept-Language")
            .unwrap_or("en")
            .split(",")
            // Get the locale, not the country code
            .filter_map(|l| l.split(['-', ';']).nth(0))
            // Get the first requested locale we support
            .find(|l| *l == "en" || *l == "es")
            .map(|l| if l == "es" { Lang::Es } else { Lang::En });

        Self { value }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for MaybeLang {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        Outcome::Success(Self::from_request_base(req))
    }
}
