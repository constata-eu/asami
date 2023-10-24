use rocket::{
  self,
  fairing::AdHoc,
  routes,
  serde::json::Json,
  http::Method,
  State
};
use rocket_cors::{AllowedHeaders, AllowedOrigins, Origins};
use rocket_recaptcha_v3::ReCaptcha;

pub mod app;
pub mod on_chain;
pub mod error;
pub mod api;
pub mod models;

pub use app::*;
pub use error::*;
pub use api::*;

#[rocket::get("/x_login?<code>&<state>")]
pub async fn x_login(app: &State<App>, code: &str, state: &str) -> rocket::response::Redirect {
  let uri = format!(
    "{host}/#/x_login?code={code}&state={state}",
    host = app.settings.pwa_host,
    state = state.replace(" ", "+"),
  );
  rocket::response::Redirect::permanent(uri)
}

#[rocket::get("/instagram_login?<code>")]
pub async fn instagram_login(app: &State<App>, code: &str) -> rocket::response::Redirect {
  let uri = format!( "{host}/#/instagram_login?code={code}", host = app.settings.pwa_host );
  rocket::response::Redirect::permanent(uri)
}

pub fn server(app: App) -> rocket::Rocket<rocket::Build> {
  let allowed = AllowedOrigins::some(
    &[
      "http://localhost:8000",
      "http://127.0.0.1:8000",
      "http://0.0.0.0:8000",
      "http://127.0.0.1:3000",
      "http://localhost:3000",
      "http://127.0.0.1:5173",
      "http://localhost:5173",
    ],
    &["file://.*", "content://.*", "https://.*"]
  ).unwrap();

  let cors = rocket_cors::CorsOptions {
    allowed_origins: AllowedOrigins::Some(Origins{ allow_null: true, ..allowed}),
    allowed_methods: vec![Method::Get, Method::Post].into_iter().map(From::from).collect(),
    allowed_headers: AllowedHeaders::all(),
    allow_credentials: true,
    ..Default::default()
  }
  .to_cors().expect("Could not create cors.");

  rocket::build()
    .attach(AdHoc::on_ignite("app", |rocket| async move {
      rocket.manage(app)
    }))
    .attach(ReCaptcha::fairing())
    .manage(new_graphql_schema())
    .attach(cors)
    .mount("/", routes![x_login, instagram_login])
    .mount("/graphql", routes![graphiql, post_handler, introspect])
}
