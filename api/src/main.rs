#[rocket::launch]
async fn rocket() -> rocket::Rocket<rocket::Build> {
  // It's ok to unwrap here as it will panic when the process launches, which helps us know and fix it right away.
  api::server(api::App::from_stdin_password().await.unwrap())
}
