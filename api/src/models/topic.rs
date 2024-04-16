use super::*;

model! {
  state: App,
  table: topics,
  struct Topic {
    #[sqlx_model_hints(int4)]
    id: i32,
    #[sqlx_model_hints(varchar)]
    name: String,
  }
}
