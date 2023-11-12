use super::{*, models, error::Error};
use sqlx_models_orm::*;
use juniper::{
  FieldResult,
  FieldError,
  graphql_value,
  EmptySubscription,
  GraphQLObject,
  GraphQLInputObject,
  graphql_object,
  IntrospectionFormat,
};

use rocket::{State, http::Status};
use juniper_rocket::{graphiql_source, GraphQLResponse};

mod current_session;
use current_session::*;
mod campaign;
use campaign::*;
mod campaign_request;
use campaign_request::*;
mod session;
use session::*;

type JsonResult<T> = AsamiResult<Json<T>>;

#[rocket::get("/graphiql")]
pub fn graphiql() -> rocket::response::content::RawHtml<String> {
  graphiql_source("/graphql", None)
}

pub async fn in_transaction(
  app: &App,
  request: juniper::http::GraphQLBatchRequest,
  non_tx_current_session: CurrentSession,
  schema: &Schema,
) -> GraphQLResponse {
  let err = ||{ GraphQLResponse::error(field_error("unexpected_error_in_graphql","")) };

  let Ok(tx) = app.user().transactional().await else { return err() };
  let app = tx.select().state;

  let Ok(session) = non_tx_current_session.0.reloaded().await else { return err() };
  let Ok(user) = session.user().await else { return err() };
  let user_id = user.attrs.id;
  let Ok(account_ids) = user.account_ids().await else { return err() };

  let current_session = CurrentSession( session );

  let response = request.execute(&*schema, &Context{ app, current_session, user_id, account_ids}).await;

  if tx.commit().await.is_err() {
    return err();
  }

  let status = if response.is_ok() { Status::Ok } else { Status::BadRequest };
  let Ok(json) = serde_json::to_string(&response) else { return err() };
  GraphQLResponse(status, json)
}

#[rocket::post("/", data = "<current>")]
pub async fn post_handler(
  app: &State<App>,
  current: CurrentSessionAndJson<juniper::http::GraphQLBatchRequest>,
  schema: &State<Schema>,
) -> GraphQLResponse {
  in_transaction(app.inner(), current.json, current.session, schema).await
}

#[rocket::get("/introspect")]
pub async fn introspect(
  app: &State<App>,
  schema: &State<Schema>,
) -> JsonResult<juniper::Value> {
  // We use a mock session for introspection queries only.
  let session = models::Session::new(app.inner().clone(), models::SessionAttrs{
    id: "".to_string(),
    user_id: 0,
    auth_method_id: 0,
    pubkey: String::new(),
    nonce: 0,
    deletion_id: Some(0),
    created_at: chrono::Utc::now(),
    updated_at: None,
  });
  let ctx = Context{
    current_session: CurrentSession(session),
    app: app.inner().clone(),
    user_id: 0,
    account_ids: vec![],
  };
  let (res, _errors) = juniper::introspect(&*schema, &ctx, IntrospectionFormat::default())
    .map_err(|_| Error::Precondition(format!("Invalid GraphQL schema for introspection")))?;
  Ok(Json(res))
}

pub struct Context {
  app: App,
  current_session: CurrentSession,
  user_id: i32,
  account_ids: Vec<String>,
}

impl Context {
  pub fn require_account_user(&self, account_id: &str) -> FieldResult<()> {
    if !self.account_ids.iter().any(|s| s == account_id) {
      return Err(field_error("no_access_to_account", &account_id.to_string()))
    }
    return Ok(())
  }
}

impl juniper::Context for Context {}

const DEFAULT_PER_PAGE: i32 = 20;
const DEFAULT_PAGE: i32 = 0;

#[rocket::async_trait]
trait Showable<Model: SqlxModel<State=App>, Filter: Send>: Sized {
  fn sort_field_to_order_by(field: &str) -> Option<<Model as SqlxModel>::ModelOrderBy>;
  fn filter_to_select(context: &Context, f: Option<Filter>) -> <Model as SqlxModel>::SelectModel;
  fn select_by_id(context: &Context, id: <Model as SqlxModel>::Id) -> <Model as SqlxModel>::SelectModel;
  async fn db_to_graphql(d: Model) -> AsamiResult<Self>;

  async fn resource(context: &Context, id: <Model as SqlxModel>::Id) -> FieldResult<Self> 
    where <Model as SqlxModel>::Id: 'async_trait
  {
    let resource = <<Model as SqlxModel>::ModelHub>::from_state(context.app.clone())
      .select()
      .use_struct(Self::select_by_id(context, id))
      .one()
      .await?;
    Ok(Self::db_to_graphql(resource).await?)
  }

  async fn collection(
    context: &Context,
    page: Option<i32>,
    per_page: Option<i32>,
    sort_field: Option<String>,
    sort_order: Option<String>,
    filter: Option<Filter>
  ) -> FieldResult<Vec<Self>>
    where Filter: 'async_trait
  {
    let limit = per_page.unwrap_or(DEFAULT_PER_PAGE);
    if limit >= 500 {
      return Err(FieldError::new(
        "Invalid pagination",
        graphql_value!({ "internal_error": "Invalid pagination" })
      ));
    }
    let offset = page.unwrap_or(DEFAULT_PAGE) * limit;

    let maybe_order_by = match sort_field {
      None => None,
      Some(ref field) => {
        if let Some(order_by) = Self::sort_field_to_order_by(field) {
          Some(order_by)
        } else {
          return Err(FieldError::new("Invalid sort_field", graphql_value!({ "invalid_sort": format!("{:?}", &sort_field) })))
        }
      }
    }; 

    let selected = <Model as SqlxModel>::SelectModelHub::from_state(context.app.clone())
      .use_struct( Self::filter_to_select(context, filter) )
      .maybe_order_by(maybe_order_by)
      .limit(limit.into())
      .offset(offset.into())
      .desc(sort_order == Some("DESC".to_string()))
      .all().await?;

    let mut all = vec![];
    for p in selected.into_iter() {
      all.push(Self::db_to_graphql(p).await?);
    }
    Ok(all)
  }

  async fn count( context: &Context, filter: Option<Filter>) -> FieldResult<ListMetadata>
    where Filter: 'async_trait
  {
    let count = <Model as SqlxModel>::SelectModelHub::from_state(context.app.clone())
      .use_struct( Self::filter_to_select(context, filter) )
      .count().await?
      .to_i32()
      .ok_or(FieldError::new("too_many_records", graphql_value!({})))?;

    Ok(ListMetadata{count})
  }
}

#[derive(Debug, GraphQLObject, serde::Serialize, serde::Deserialize)]
pub struct ListMetadata {
  pub count: i32
}

#[derive(Debug)]
pub struct Query;

macro_rules! make_graphql_query {
  (
    $version:literal;
    showables {
      $([$resource_type:ident, $collection:ident, $meta:tt, $meta_name:literal, $filter_type:ty, $id_type:ty],)*
    }
    $($extra:tt)*
  ) => (
    #[graphql_object(context=Context)]
    impl Query {
      fn api_version() -> &'static str { $version }

      $(
        #[allow(non_snake_case)]
        async fn $resource_type(context: &Context, id: $id_type) -> FieldResult<$resource_type> {
          <$resource_type>::resource(context, id).await
        }

        #[allow(non_snake_case)]
        async fn $collection(context: &Context, page: Option<i32>, per_page: Option<i32>, sort_field: Option<String>, sort_order: Option<String>, filter: Option<$filter_type>) -> FieldResult<Vec<$resource_type>> {
          <$resource_type>::collection(context, page, per_page, sort_field, sort_order, filter).await
        }

        #[graphql(name=$meta_name)]
        #[allow(non_snake_case)]
        async fn $meta(context: &Context, _page: Option<i32>, _per_page: Option<i32>, _sort_field: Option<String>, _sort_order: Option<String>, filter: Option<$filter_type>) -> FieldResult<ListMetadata> {
          <$resource_type>::count(context, filter).await
        }
      )*

      $($extra)*
    }
  )
}

make_graphql_query!{
  "1.0";
  showables {
    [CampaignRequest, allCampaignRequests, allCampaignRequestsMeta, "_allCampaignRequestsMeta", CampaignRequestFilter, i32],
    [Campaign, allCampaigns, allCampaignsMeta, "_allCampaignsMeta", CampaignFilter, String],
  }
}

pub struct Mutation;

#[graphql_object(context=Context)]
impl Mutation {
  pub async fn create_session(context: &Context) -> FieldResult<Session> {
    Ok(Session::db_to_graphql(context.current_session.0.clone()).await?)
  }

  pub async fn create_campaign_request(context: &Context, input: CreateCampaignRequestInput) -> FieldResult<CampaignRequest> {
    input.process(context).await
  }
}

// A root schema consists of a query and a mutation.
// Request queries can be executed against a RootNode.
pub type Schema = juniper::RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub fn new_graphql_schema() -> Schema {
  Schema::new_with_scalar_value(Query, Mutation, EmptySubscription::<Context>::new())
}

fn field_error(message: &str, second_message: &str) -> FieldError {
  FieldError::new(
      message,
      graphql_value!({ "internal_error":  second_message })
    )
}

fn into_like_search(i: Option<String>) -> Option<String> {
  i.map(|s| format!("%{s}%") )
}
