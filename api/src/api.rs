use ethers::{abi::AbiEncode, types::U256};
use juniper::{
    graphql_object, graphql_value, EmptySubscription, FieldError, FieldResult, GraphQLInputObject, GraphQLObject,
    IntoFieldError, IntrospectionFormat,
};
use juniper_rocket::{graphiql_source, GraphQLResponse};
use rocket::{http::Status, serde::json::Json, State};
use sqlx_models_orm::*;

use super::{error::Error, models, *};
use crate::models::CreateCampaignFromLinkInput;

mod current_session;
use current_session::*;
mod account;
use account::*;
pub mod campaign;
use campaign::*;
mod session;
use session::*;
mod handle;
use handle::*;
mod collab;
use collab::*;
mod claim_account_request;
use claim_account_request::*;
mod campaign_preference;
use campaign_preference::*;
mod on_chain_job;
use on_chain_job::*;
mod audit_log_entry;
use audit_log_entry::*;
mod stats;
use stats::*;
mod token_stats;
use token_stats::*;
mod topic;
use topic::*;
mod handle_scoring;
use handle_scoring::*;
mod community_member;
use community_member::*;
mod holder;
use holder::*;
mod account_merge;
use account_merge::*;

type JsonResult<T> = AsamiResult<Json<T>>;

#[rocket::get("/graphiql")]
pub fn graphiql() -> rocket::response::content::RawHtml<String> {
    graphiql_source("/graphql", None)
}

#[rocket::options("/")]
pub fn options() -> Status {
    Status::Ok
}

pub async fn in_transaction(
    app: &App,
    request: juniper::http::GraphQLBatchRequest,
    non_tx_current_session: Option<CurrentSession>,
    schema: &Schema,
    lang: lang::Lang,
) -> GraphQLResponse {
    let err = || GraphQLResponse::error(field_error("unexpected_error_in_graphql", ""));

    let Ok(tx) = app.user().transactional().await else {
        return err();
    };
    let app = tx.select().state;

    let context = match non_tx_current_session {
        Some(current) => {
            let Ok(session) = current.0.reloaded().await else {
                return err();
            };
            Context {
                app,
                current_session: Some(CurrentSession(session)),
                lang,
            }
        }
        None => Context {
            app,
            current_session: None,
            lang,
        },
    };

    let response = request.execute(schema, &context).await;

    if tx.commit().await.is_err() {
        return err();
    }

    let status = if response.is_ok() {
        Status::Ok
    } else {
        Status::BadRequest
    };
    let Ok(json) = serde_json::to_string(&response) else {
        return err();
    };
    GraphQLResponse(status, json)
}

#[rocket::post("/", data = "<session_and_json>")]
pub async fn post_handler(
    app: &State<App>,
    session_and_json: Result<CurrentSessionAndJson<juniper::http::GraphQLBatchRequest>, ApiAuthError>,
    schema: &State<Schema>,
    lang: lang::Lang,
) -> GraphQLResponse {
    match session_and_json {
        Ok(current) => in_transaction(app.inner(), current.json, current.session, schema, lang).await,
        Err(auth_error) => {
            let (msg, status) = match &auth_error {
                ApiAuthError::Fail(msg) => (msg.as_str(), Status::Unauthorized),
                ApiAuthError::Unexpected(msg) => (*msg, Status::InternalServerError),
            };
            GraphQLResponse(status, serde_json::json!({"authError": msg}).to_string())
        }
    }
}

#[rocket::get("/introspect")]
pub async fn introspect(app: &State<App>, schema: &State<Schema>) -> JsonResult<juniper::Value> {
    let ctx = Context {
        current_session: None,
        app: app.inner().clone(),
        lang: lang::Lang::En,
    };

    let (res, _errors) = juniper::introspect(schema, &ctx, IntrospectionFormat::default())
        .map_err(|_| Error::Precondition("Invalid GraphQL schema for introspection".to_string()))?;
    Ok(Json(res))
}

pub struct Context {
    app: App,
    current_session: Option<CurrentSession>,
    lang: lang::Lang,
}

impl Context {
    pub fn current_session(&self) -> AsamiResult<CurrentSession> {
        self.current_session.clone().ok_or(Error::service("authentication", "asami_authentication_needed"))
    }

    pub fn user_id(&self) -> AsamiResult<i32> {
        Ok(*self.current_session()?.0.user_id())
    }

    pub fn account_id(&self) -> AsamiResult<String> {
        Ok(self.current_session()?.0.attrs.account_id.clone())
    }

    pub fn maybe_account_id(&self) -> Option<&str> {
        self.current_session.as_ref().map(|s| s.0.attrs.account_id.as_str())
    }

    pub async fn account(&self) -> AsamiResult<models::Account> {
        Ok(self.app.account().find(self.account_id()?).await?)
    }
}

impl juniper::Context for Context {}

const DEFAULT_PER_PAGE: i32 = 60;
const DEFAULT_PAGE: i32 = 0;

#[rocket::async_trait]
trait Showable<Model: SqlxModel<State = App>, Filter: Send>: Sized {
    fn sort_field_to_order_by(field: &str) -> Option<<Model as SqlxModel>::ModelOrderBy>;
    fn filter_to_select(context: &Context, f: Option<Filter>) -> FieldResult<<Model as SqlxModel>::SelectModel>;
    fn select_by_id(context: &Context, id: <Model as SqlxModel>::Id) -> FieldResult<<Model as SqlxModel>::SelectModel>;
    async fn db_to_graphql(context: &Context, d: Model) -> AsamiResult<Self>;

    async fn resource(context: &Context, id: <Model as SqlxModel>::Id) -> FieldResult<Self>
    where
        <Model as SqlxModel>::Id: 'async_trait,
    {
        let resource = <<Model as SqlxModel>::ModelHub>::from_state(context.app.clone())
            .select()
            .use_struct(Self::select_by_id(context, id)?)
            .one()
            .await?;
        Ok(Self::db_to_graphql(context, resource).await?)
    }

    async fn collection(
        context: &Context,
        page: Option<i32>,
        per_page: Option<i32>,
        sort_field: Option<String>,
        sort_order: Option<String>,
        filter: Option<Filter>,
    ) -> FieldResult<Vec<Self>>
    where
        Filter: 'async_trait,
    {
        Self::base_collection(context, page, per_page, sort_field, sort_order, filter).await
    }

    async fn base_collection(
        context: &Context,
        page: Option<i32>,
        per_page: Option<i32>,
        sort_field: Option<String>,
        sort_order: Option<String>,
        filter: Option<Filter>,
    ) -> FieldResult<Vec<Self>>
    where
        Filter: 'async_trait,
    {
        let limit = per_page.unwrap_or(DEFAULT_PER_PAGE);
        if limit >= 1001 {
            return Err(FieldError::new(
                "Invalid pagination",
                graphql_value!({ "internal_error": "Invalid pagination" }),
            ));
        }
        let offset = page.unwrap_or(DEFAULT_PAGE) * limit;

        let maybe_order_by = match sort_field {
            None => None,
            Some(ref field) => {
                if let Some(order_by) = Self::sort_field_to_order_by(field) {
                    Some(order_by)
                } else {
                    return Err(FieldError::new(
                        "Invalid sort_field",
                        graphql_value!({ "invalid_sort": format!("{:?}", &sort_field) }),
                    ));
                }
            }
        };

        let selected = <Model as SqlxModel>::SelectModelHub::from_state(context.app.clone())
            .use_struct(Self::filter_to_select(context, filter)?)
            .maybe_order_by(maybe_order_by)
            .limit(limit.into())
            .offset(offset.into())
            .desc(sort_order == Some("DESC".to_string()))
            .all()
            .await?;

        let mut all = vec![];
        for p in selected.into_iter() {
            all.push(Self::db_to_graphql(context, p).await?);
        }
        Ok(all)
    }

    async fn count(context: &Context, filter: Option<Filter>) -> FieldResult<ListMetadata>
    where
        Filter: 'async_trait,
    {
        Self::base_count(context, filter).await
    }

    async fn base_count(context: &Context, filter: Option<Filter>) -> FieldResult<ListMetadata>
    where
        Filter: 'async_trait,
    {
        let count = <Model as SqlxModel>::SelectModelHub::from_state(context.app.clone())
            .use_struct(Self::filter_to_select(context, filter)?)
            .count()
            .await?
            .to_i32()
            .ok_or(FieldError::new("too_many_records", graphql_value!({})))?;

        Ok(ListMetadata { count })
    }
}

#[derive(Debug, GraphQLObject, serde::Serialize, serde::Deserialize)]
pub struct ListMetadata {
    pub count: i32,
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

make_graphql_query! {
  "1.0";
  showables {
    [Account, allAccounts, allAccountsMeta, "_allAccountsMeta", AccountFilter, String],
    [Campaign, allCampaigns, allCampaignsMeta, "_allCampaignsMeta", CampaignFilter, i32],
    [Handle, allHandles, allHandlesMeta, "_allHandlesMeta", HandleFilter, i32],
    [HandleScoring, allHandleScorings, allHandleScoringsMeta, "_allHandleScoringsMeta", HandleScoringFilter, i32],
    [Collab, allCollabs, allCollabsMeta, "_allCollabsMeta", CollabFilter, i32],
    [CampaignPreference, allCampaignPreferences, allCampaignPreferencesMeta, "_allCampaignPreferencesMeta", CampaignPreferenceFilter, i32],
    [OnChainJob, allOnChainJobs, allOnChainJobsMeta, "_allOnChainJobsMeta", OnChainJobFilter, i32],
    [Topic, allTopics, allTopicsMeta, "_allTopicsMeta", TopicFilter, i32],
    [AuditLogEntry, allAuditLogEntries, allAuditLogEntriesMeta, "_allAuditLogEntriesMeta", AuditLogEntryFilter, i32],
    [CommunityMember, allCommunityMembers, allCommunityMembersMeta, "_allCommunityMembersMeta", CommunityMemberFilter, i32],
    [Holder, allHolders, allHoldersMeta, "_allHoldersMeta", HolderFilter, i32],
    [AccountMerge, allAccountMerges, allAccountMergesMeta, "_allAccountMergesMeta", AccountMergeFilter, i32],
  }

  #[graphql(name="Stats")]
  async fn stats(context: &Context, _id: i32) -> FieldResult<Stats> {
      Stats::build(&context.app).await
  }

  #[graphql(name="TokenStats")]
  async fn token_stats(context: &Context, _id: i32) -> FieldResult<TokenStats> {
      TokenStats::build(&context.app).await
  }
}

pub struct Mutation;

#[graphql_object(context=Context)]
impl Mutation {
    pub async fn create_session(context: &Context) -> FieldResult<Session> {
        Ok(Session::db_to_graphql(context, context.current_session()?.0.clone()).await?)
    }

    pub async fn create_campaign_from_link(
        context: &Context,
        input: CreateCampaignFromLinkInput,
    ) -> FieldResult<Campaign> {
        let campaign = input.process(&context.app, &context.account().await?).await?;
        Ok(Campaign::db_to_graphql(context, campaign).await?)
    }

    pub async fn update_campaign(context: &Context, id: i32) -> FieldResult<Campaign> {
        let campaign = context.account().await?.campaign_scope().id_eq(id).one().await?.mark_submitted().await?;
        Ok(Campaign::db_to_graphql(context, campaign).await?)
    }

    pub async fn create_gasless_allowance(context: &Context) -> FieldResult<Account> {
        Ok(Account::db_to_graphql(context, context.account().await?.allow_gasless().await?).await?)
    }

    pub async fn create_claim_account_request(
        context: &Context,
        input: CreateClaimAccountRequestInput,
    ) -> FieldResult<Account> {
        input.process(context).await
    }

    pub async fn create_campaign_preference(
        context: &Context,
        input: CreateCampaignPreferenceInput,
    ) -> FieldResult<CampaignPreference> {
        input.process(context).await
    }

    pub async fn create_email_login_link(context: &Context, email: String) -> FieldResult<EmailLoginLink> {
        Ok(EmailLoginLink {
            id: context
                .app
                .one_time_token()
                .create_for_email(email.to_lowercase(), context.lang, None)
                .await?
                .attrs
                .id,
        })
    }

    pub async fn create_one_time_token(context: &Context) -> FieldResult<models::OneTimeTokenAttrs> {
        Ok(context.app.one_time_token().create_for_session_migration(context.user_id()?).await?.attrs)
    }

    pub async fn create_account_merge(context: &Context) -> FieldResult<AccountMerge> {
        let merge = context.app.account_merge().get_or_create(&context.account().await?).await?;

        Ok(AccountMerge::db_to_graphql(context, merge).await?)
    }

    pub async fn update_account_merge(context: &Context, id: String) -> FieldResult<bool> {
        context
            .app
            .account_merge()
            .accept_with_code(&context.current_session()?.0, context.account().await?, id)
            .await
            .map_err(|e| e.into_field_error())?;
        Ok(true)
    }

    pub async fn create_x_refresh_token(context: &Context, token: String, verifier: String) -> FieldResult<Handle> {
        let handle = context
            .app
            .handle()
            .create_or_update_from_refresh_token(context.account_id()?, token, verifier)
            .await?;
        Ok(Handle::db_to_graphql(context, handle).await?)
    }

    pub async fn update_handle(context: &Context, id: i32, data: AdminEditHandleInput) -> FieldResult<Handle> {
        if !context.current_session()?.0.admin() {
            return Err(Error::service("authentication", "asami_authentication_needed").into_field_error());
        }

        data.process(context, id).await
    }

    pub async fn update_community_member(
        context: &Context,
        id: i32,
        data: EditCommunityMemberInput,
    ) -> FieldResult<CommunityMember> {
        data.process(context, id).await
    }
}

#[derive(Debug, GraphQLObject, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[graphql(description = "A request to verify a handle for an account.")]
pub struct EmailLoginLink {
    #[graphql(description = "Unique numeric identifier of this resource")]
    id: i32,
}

// A root schema consists of a query and a mutation.
// Request queries can be executed against a RootNode.
pub type Schema = juniper::RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub fn new_graphql_schema() -> Schema {
    Schema::new_with_scalar_value(Query, Mutation, EmptySubscription::<Context>::new())
}

fn field_error(message: &str, second_message: &str) -> FieldError {
    FieldError::new(message, graphql_value!({ "internal_error": second_message }))
}

fn into_like_search(i: Option<impl std::fmt::Display>) -> Option<String> {
    i.map(|s| format!("%{s}%"))
}
