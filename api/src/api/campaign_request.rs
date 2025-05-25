use rocket::{data::{self, FromData, ToByteUnit}, Data, Request};

use super::{
    models::{self, *},
    *,
};

#[derive(Debug, GraphQLObject, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[graphql(description = "A campaign request using stripe")]
pub struct CampaignRequest {
    id: i32,
    account_id: i32,
    price_per_point: String,
    min_individual_reward : String,
    max_individual_reward: String,
    thumbs_up_only: bool,
    status: CampaignRequestStatus,
    stripe_session_url: Option<String>,
    stripe_session_id: Option<String>,
    campaign_id: Option<i32>,
    topic_ids: Vec<i32>,
    created_at: UtcDateTime,
}

#[derive(Debug, Clone, Default, GraphQLInputObject, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CampaignRequestFilter {
    ids: Option<Vec<i32>>,
    id_eq: Option<i32>,
    account_id_eq: Option<i32>,
    status_eq: Option<CampaignRequestStatus>
}

#[rocket::async_trait]
impl Showable<models::CampaignRequest, CampaignRequestFilter> for CampaignRequest {
    fn sort_field_to_order_by(field: &str) -> Option<models::CampaignRequestOrderBy> {
        match field {
            "id" => Some(CampaignRequestOrderBy::Id),
            "createdAt" => Some(CampaignRequestOrderBy::CreatedAt),
            _ => None,
        }
    }

    fn filter_to_select(context: &Context, filter: Option<CampaignRequestFilter>) -> FieldResult<models::SelectCampaignRequest> {
        let initial = models::SelectCampaignRequest{
            account_id_eq: Some(context.account_id()?.clone()),
            ..Default::default()
        }; 

        if let Some(f) = filter {
            Ok(models::SelectCampaignRequest {
                id_in: f.ids,
                id_eq: f.id_eq,
                status_eq: f.status_eq,
                ..initial
            })
        } else {
            Ok(initial)
        }
    }

    fn select_by_id(context: &Context, id: i32) -> FieldResult<models::SelectCampaignRequest> {
        Ok(models::SelectCampaignRequest {
            id_eq: Some(id),
            account_id_eq: Some(context.account_id()?.clone()),
            ..Default::default()
        })
    }

    async fn db_to_graphql(_context: &Context, d: models::CampaignRequest) -> AsamiResult<Self> {
        let topic_ids = d.topic_ids().await?;
        Ok(CampaignRequest {
            id: d.attrs.id,
            account_id: hex_to_i32(&d.attrs.account_id)?,
            price_per_point: d.attrs.price_per_point,
            min_individual_reward: d.attrs.min_individual_reward,
            max_individual_reward: d.attrs.max_individual_reward,
            thumbs_up_only: d.attrs.thumbs_up_only,
            status: d.attrs.status,
            stripe_session_url: d.attrs.stripe_session_url,
            stripe_session_id: d.attrs.stripe_session_id,
            campaign_id: d.attrs.campaign_id,
            created_at: d.attrs.created_at,
            topic_ids,
        })
    }
}

#[rocket::post("/handle_stripe_events", data = "<webhook>")]
pub async fn handle_stripe_events(webhook: StripeWebhook, app: &State<App>) -> AsamiResult<serde_json::Value> {
  app.campaign_request().set_paid_from_stripe_event(&webhook.event).await?;
  Ok(serde_json::json!("OK"))
}

pub struct StripeWebhook{
  event: stripe::Event
}

#[rocket::async_trait]
impl<'r> FromData<'r> for StripeWebhook {
  type Error = Error;

  async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
    use rocket::data::Outcome;

    let secret = req
      .rocket()
      .state::<App>()
      .expect("App not configured")
      .settings
      .stripe.events_secret
      .clone();

    let Some(sig) = req.headers().get_one("stripe-signature") else {
        return Outcome::Error((Status::BadRequest, Error::validation("body", "missing event signature")));
    };

    let bytes = match data.open(512000.bytes()).into_string().await {
      Ok(read) if read.is_complete() => read.into_inner(),
      Ok(_) => return Outcome::Error((Status::PayloadTooLarge, Error::validation("payload", "payload too large"))),
      Err(_) => return Outcome::Error((Status::BadRequest, Error::validation("body", "Bad request, can't read body."))),
    };

    match stripe::Webhook::construct_event(&bytes, &sig, &secret) {
      Ok(event) => Outcome::Success(StripeWebhook{event}),
      _ => Outcome::Error((Status::BadRequest, Error::validation("body", "invalid event signature"))),
    }
  }
}

