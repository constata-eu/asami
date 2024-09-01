pub use anyhow::Context;
use rocket::{self, fairing::AdHoc, figment, http::Method, routes, serde::json::Json, State};
use rocket_cors::{AllowedHeaders, AllowedOrigins, Origins};
use rocket_recaptcha_v3::ReCaptcha;
pub use rust_decimal::prelude::{Decimal, FromPrimitive, ToPrimitive};

pub mod api;
pub mod app;
pub mod error;
pub mod lang;
pub mod models;
pub mod on_chain;

pub use api::*;
pub use app::*;
pub use error::*;
pub use lang::*;

#[rocket::get("/x_login?<code>&<state>")]
pub async fn x_login(app: &State<App>, code: &str, state: &str) -> rocket::response::Redirect {
    let uri = format!(
        "{host}/#/x_login?code={code}&state={state}",
        host = app.settings.pwa_host,
        state = state.replace(' ', "+"),
    );
    rocket::response::Redirect::permanent(uri)
}

#[rocket::get("/facebook_login?<code>")]
pub async fn facebook_login(app: &State<App>, code: &str) -> rocket::response::Redirect {
    let uri = format!("{host}/#/facebook_login?code={code}", host = app.settings.pwa_host);
    rocket::response::Redirect::permanent(uri)
}

#[rocket::get("/config")]
pub async fn config(app: &State<App>) -> serde_json::Value {
    serde_json::json![{
      "instagram_verification_image_url": app.settings.instagram.verification_image_url,
      "instagram_verification_caption": app.settings.instagram.verification_caption,
      "contractAddress": app.settings.rsk.asami_contract_address.clone(),
      "docContractAddress": app.settings.rsk.doc_contract_address.clone(),
    }]
}

pub fn server(app: App) -> rocket::Rocket<rocket::Build> {
    custom_server(app, rocket::Config::figment())
}

pub fn custom_server(app: App, fig: figment::Figment) -> rocket::Rocket<rocket::Build> {
    let allowed = AllowedOrigins::some(
        &[
            "http://localhost:8000",
            "http://127.0.0.1:8000",
            "http://0.0.0.0:8000",
            "http://127.0.0.1:3000",
            "http://localhost:3000",
            "http://127.0.0.1:5173",
            "http://localhost:5173",
            "https://asami.club",
        ],
        &["file://.*", "content://.*"],
    )
    .unwrap();

    let cors = rocket_cors::CorsOptions {
        allowed_origins: AllowedOrigins::Some(Origins {
            allow_null: true,
            ..allowed
        }),
        allowed_methods: vec![Method::Get, Method::Post].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("Could not create cors.");

    rocket::custom(fig)
        .attach(AdHoc::on_ignite("app", |rocket| async move { rocket.manage(app) }))
        .attach(ReCaptcha::fairing())
        .manage(new_graphql_schema())
        .attach(cors)
        .mount("/", routes![x_login, facebook_login, config])
        .mount("/graphql", routes![graphiql, post_handler, introspect, options])
}
