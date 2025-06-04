use rocket::{
    data::{self, FromData, ToByteUnit},
    Data, Request,
};

use super::{
    models::{self, *},
    *,
};

#[derive(Debug, GraphQLObject, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[graphql(description = "A campaign")]
pub struct Campaign {
    #[graphql(description = "Unique numeric identifier of this resource")]
    id: i32,
    #[graphql(description = "The id of the account that created this.")]
    account_id: i32,
    #[graphql(description = "The total budget for this campaign to be collected by users.")]
    budget: String,
    #[graphql(description = "If the campaign is published and ready to receive collaborations.")]
    is_published: bool,
    #[graphql(description = "Auxiliary data related to this campaign's briefing")]
    briefing_json: String,
    #[graphql(description = "Auxiliary data related to this campaign's briefing")]
    briefing_hash: String,
    #[graphql(description = "The campaign expiration date, after which funds may be returned")]
    valid_until: Option<UtcDateTime>,
    #[graphql(description = "The date in which this campaign was created.")]
    created_at: UtcDateTime,
    #[graphql(description = "The topic ids this campaign is restricted to.")]
    topic_ids: Vec<i32>,
    #[graphql(description = "The reward you would receive. None means it does not apply.")]
    you_would_receive: Option<String>,
    #[graphql(description = "How many collabs did the campaign get")]
    total_collabs: i32,
    #[graphql(description = "How much the campaign has spent so far in rewards")]
    total_spent: String,
    #[graphql(description = "The campaign total budget: remaining + spent")]
    total_budget: String,
    #[graphql(description = "Campaign only available with users with a thumbs up from advertiser")]
    thumbs_up_only: bool,
    #[graphql(description = "These fields are private to the campaign's owner and for internal use only. ")]
    private_fields: Option<PrivateFields>,
}

#[derive(Debug, GraphQLObject, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[graphql(description = "Private fields for the campaign owner only")]
pub struct PrivateFields {
    managed_by_admin: bool,
    managed_unit_amount: Option<i32>,
    stripe_session_url: Option<String>,
    stripe_session_id: Option<String>,
    status: CampaignStatus,
}

#[derive(Debug, Clone, Default, GraphQLInputObject, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CampaignFilter {
    ids: Option<Vec<i32>>,
    id_eq: Option<i32>,
    account_id_eq: Option<i32>,
    budget_gt: Option<String>,
    budget_lt: Option<String>,
    budget_eq: Option<String>,
    briefing_hash_like: Option<String>,
    briefing_json_like: Option<String>,
    status_ne: Option<CampaignStatus>,
    status_eq: Option<CampaignStatus>,
    available_to_account_id: Option<i32>,
    managed_by_admin_eq: Option<bool>,
    is_published_eq: Option<bool>,
}

async fn make_available_to_account_id_filter(context: &Context, account_id: i32) -> FieldResult<CampaignFilter> {
    let offers = context
        .app
        .account()
        .find(&i32_to_hex(account_id))
        .await?
        .campaign_offers()
        .await?
        .into_iter()
        .map(|x| x.attrs.id)
        .collect();
    Ok(CampaignFilter {
        ids: Some(offers),
        ..Default::default()
    })
}

#[rocket::async_trait]
impl Showable<models::Campaign, CampaignFilter> for Campaign {
    fn sort_field_to_order_by(field: &str) -> Option<models::CampaignOrderBy> {
        match field {
            "id" => Some(CampaignOrderBy::Id),
            "budget" => Some(CampaignOrderBy::Budget),
            "totalCollabs" => Some(CampaignOrderBy::TotalCollabs),
            "totalSpent" => Some(CampaignOrderBy::TotalSpent),
            "totalBudget" => Some(CampaignOrderBy::TotalBudget),
            "validUntil" => Some(CampaignOrderBy::ValidUntil),
            "createdAt" => Some(CampaignOrderBy::CreatedAt),
            _ => None,
        }
    }

    async fn collection(
        context: &Context,
        page: Option<i32>,
        per_page: Option<i32>,
        sort_field: Option<String>,
        sort_order: Option<String>,
        filter: Option<CampaignFilter>,
    ) -> FieldResult<Vec<Self>> {
        if let Some(account_id) = filter.as_ref().and_then(|f| f.available_to_account_id) {
            let id_filter = make_available_to_account_id_filter(context, account_id).await?;
            Self::base_collection(context, page, per_page, sort_field, sort_order, Some(id_filter)).await
        } else {
            Self::base_collection(context, page, per_page, sort_field, sort_order, filter).await
        }
    }

    async fn count(context: &Context, filter: Option<CampaignFilter>) -> FieldResult<ListMetadata> {
        if let Some(account_ids) = filter.as_ref().and_then(|f| f.available_to_account_id) {
            let ids_filter = make_available_to_account_id_filter(context, account_ids).await?;
            Self::base_count(context, Some(ids_filter)).await
        } else {
            Self::base_count(context, filter).await
        }
    }

    fn filter_to_select(context: &Context, filter: Option<CampaignFilter>) -> FieldResult<models::SelectCampaign> {
        if let Some(f) = filter {
            // Managed by admin filter requires a logged in user, and implicitly filters
            // only the current user campaigns.
            let private_filters = f.managed_by_admin_eq.is_some() || f.status_eq.is_some() || f.status_ne.is_some();

            let account_id_eq = if private_filters {
                Some(context.account_id()?)
            } else {
                f.account_id_eq.map(i32_to_hex)
            };

            let (status_eq, status_ne) = match f.is_published_eq {
                Some(true) => (Some(CampaignStatus::Published), None),
                Some(false) => (None, Some(CampaignStatus::Published)),
                _ => (f.status_eq, f.status_ne),
            };

            Ok(models::SelectCampaign {
                id_in: f.ids,
                account_id_eq,
                id_eq: f.id_eq,
                budget_gt: f.budget_gt,
                budget_lt: f.budget_lt,
                budget_eq: f.budget_eq,
                status_eq,
                status_ne,
                briefing_json_like: into_like_search(f.briefing_json_like),
                briefing_hash_like: into_like_search(f.briefing_hash_like),
                managed_by_admin_eq: f.managed_by_admin_eq,
                ..Default::default()
            })
        } else {
            Ok(Default::default())
        }
    }

    fn select_by_id(_context: &Context, id: i32) -> FieldResult<models::SelectCampaign> {
        Ok(models::SelectCampaign {
            id_eq: Some(id),
            ..Default::default()
        })
    }
    async fn db_to_graphql(context: &Context, d: models::Campaign) -> AsamiResult<Self> {
        let topic_ids = d.topic_ids().await?;
        let you_would_receive = match context.current_session {
            Some(_) => d.reward_for_account(&context.account().await?).await?.map(|x| x.encode_hex()),
            None => None,
        };
        let budget = d.available_budget().await.unwrap_or(d.budget_u256()).encode_hex();

        let is_published = d.attrs.status == CampaignStatus::Published;

        let private_fields = if context.maybe_account_id().map(|i| i == d.account_id()).unwrap_or(false) {
            Some(PrivateFields {
                managed_by_admin: d.attrs.managed_by_admin,
                managed_unit_amount: d.attrs.managed_unit_amount,
                stripe_session_url: d.attrs.stripe_session_url,
                stripe_session_id: d.attrs.stripe_session_id,
                status: d.attrs.status,
            })
        } else {
            None
        };

        Ok(Campaign {
            id: d.attrs.id,
            account_id: hex_to_i32(&d.attrs.account_id)?,
            budget,
            valid_until: d.attrs.valid_until,
            briefing_json: d.attrs.briefing_json,
            briefing_hash: d.attrs.briefing_hash,
            created_at: d.attrs.created_at,
            topic_ids,
            you_would_receive,
            total_collabs: d.attrs.total_collabs,
            total_spent: d.attrs.total_spent,
            total_budget: d.attrs.total_budget,
            thumbs_up_only: d.attrs.thumbs_up_only,
            is_published,
            private_fields,
        })
    }
}

#[rocket::post("/handle_stripe_events", data = "<webhook>")]
pub async fn handle_stripe_events(webhook: StripeWebhook, app: &State<App>) -> AsamiResult<serde_json::Value> {
    app.campaign().set_status_from_stripe_event(&webhook.event).await?;
    Ok(serde_json::json!("OK"))
}

pub struct StripeWebhook {
    event: stripe::Event,
}

#[rocket::async_trait]
impl<'r> FromData<'r> for StripeWebhook {
    type Error = Error;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
        use rocket::data::Outcome;

        let secret = req.rocket().state::<App>().expect("App not configured").settings.stripe.events_secret.clone();

        let Some(sig) = req.headers().get_one("stripe-signature") else {
            return Outcome::Error((Status::BadRequest, Error::validation("body", "missing event signature")));
        };

        let bytes = match data.open(512000.bytes()).into_string().await {
            Ok(read) if read.is_complete() => read.into_inner(),
            Ok(_) => {
                return Outcome::Error((
                    Status::PayloadTooLarge,
                    Error::validation("payload", "payload too large"),
                ))
            }
            Err(_) => {
                return Outcome::Error((
                    Status::BadRequest,
                    Error::validation("body", "Bad request, can't read body."),
                ))
            }
        };

        match stripe::Webhook::construct_event(&bytes, sig, &secret) {
            Ok(event) => Outcome::Success(StripeWebhook { event }),
            Err(_) => Outcome::Error((
                Status::BadRequest,
                Error::validation("body", "could not construct event"),
            )),
        }
    }
}
