use super::*;

model! {
  state: App,
  table: topics,
  struct Topic {
    #[sqlx_model_hints(int4, default, op_in)]
    id: i32,
    #[sqlx_model_hints(varchar, op_like)]
    name: String,
  }
}
