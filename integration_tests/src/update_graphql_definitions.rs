fn main() {
  let schema = api::api::new_graphql_schema().as_sdl();
  std::fs::write("public_api_schema.graphql", &schema).unwrap();
  std::fs::write("public_api_queries.graphql", graphql_queries_from_schema::generate_all(&schema).unwrap()).unwrap();
  println!("public_api_schema and public_api_queries updated.");
}

