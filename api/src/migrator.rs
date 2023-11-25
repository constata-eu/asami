use api::app::AppConfig;

#[tokio::main]
async fn main() {
  let result = sqlx::migrate!()
    .run(&AppConfig::default().unwrap().db().await.unwrap().pool)
    .await;

  if let Err(e) = result {
    println!("Migration error {:?}", e);
  }
}
