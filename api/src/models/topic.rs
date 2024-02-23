use super::*;

model! {
  state: App,
  table: topics,
  struct Topic {
    #[sqlx_model_hints(varchar)]
    id: String,
    #[sqlx_model_hints(varchar)]
    name: String,
  }
}
