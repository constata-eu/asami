use super::*;
use juniper::GraphQLInputObject;

model! {
  state: App,
  table: campaign_requests,
  struct CampaignRequest {
    #[sqlx_model_hints(int4, default, op_in)]
    id: i32,
    #[sqlx_model_hints(varchar)]
    account_id: String,
    #[sqlx_model_hints(varchar)]
    link: String,
    #[sqlx_model_hints(int4, op_lt, op_gt)]
    unit_amount: i32, // Amount in cents of a dollar.
    #[sqlx_model_hints(varchar, op_lt, op_gt)]
    price_per_point: String,
    #[sqlx_model_hints(varchar, op_lt, op_gt)]
    min_individual_reward : String,
    #[sqlx_model_hints(varchar, op_lt, op_gt)]
    max_individual_reward: String,
    #[sqlx_model_hints(boolean)]
    thumbs_up_only: bool,
    #[sqlx_model_hints(campaign_request_status, default)]
    status: CampaignRequestStatus,
    #[sqlx_model_hints(varchar, default)]
    stripe_session_url: Option<String>,
    #[sqlx_model_hints(varchar, default)]
    stripe_session_id: Option<String>,
    #[sqlx_model_hints(int4, default)]
    campaign_id: Option<i32>,
    #[sqlx_model_hints(timestamptz, default)]
    created_at: DateTime<Utc>,
  },
  has_many {
    CampaignRequestTopic(campaign_request_id),
  },
  belongs_to {
    Account(account_id),
    Campaign(campaign_id),
  }
}

model! {
  state: App,
  table: campaign_request_topics,
  struct CampaignRequestTopic {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(int4)]
    campaign_request_id: i32,
    #[sqlx_model_hints(int4)]
    topic_id: i32,
  }
}

make_sql_enum![
    "campaign_request_status",
    pub enum CampaignRequestStatus {
        Requested, // A customer sent this request.
        SessionCreated, // A stripe payment session was created.
        Paid,      // Stripe told us this was paid.
        Draft,     // We created a draft campaign in the system, ready to fund with DOC.
        Submitted, // We mark the campaign as submitted, a transaction was made on-chain for it.
        Published, // The transaction confirmed, so the campaign is public.
        Failed,    // Something went wrong, a refund may or may not be needed.
        Refunded,  // If failure also included a refund, this means the refund was done..
    }
];

impl CampaignRequestHub {
  pub async fn set_paid_from_stripe_event(&self, e: &stripe::Event) -> AsamiResult<Option<CampaignRequest>> {
    use stripe::{EventType, EventObject};

    let (EventType::PaymentIntentSucceeded, EventObject::PaymentIntent(i)) = (&e.event_type, &e.data.object)  else {
      return Ok(None)
    };

    let customer_id = i.customer.as_ref().map(|c| c.id().to_string() ).ok_or(Error::validation("customer","missing"))?;
    let Some(account) = self.state.account()
      .select()
      .stripe_customer_id_eq(customer_id)
      .optional()
      .await? else { return Ok(None) };

    let Some(request_id) = i.metadata.get("campaign_request_id").and_then(|i| i.parse::<i32>().ok() ) else {
      return Err(Error::validation("campaign_request_id", "no_request_id_on_stripe_event"));
    };

    let Some(campaign_request) = self.state.campaign_request().select()
      .status_eq(CampaignRequestStatus::SessionCreated)
      .account_id_eq(account.id())
      .id_eq(request_id)
      .optional()
      .await? else {
      return Err(Error::validation("campaign_request", "campaign_request_not_found"));
    };

    Ok(Some(campaign_request.update().status(CampaignRequestStatus::Paid).save().await?))
  }

  pub async fn make_campaigns_for_all_paid(&self) -> AsamiResult<()> {
    for request in self.select().status_eq(CampaignRequestStatus::Paid).all().await? {
      let on_chain_balance = self.state.on_chain.doc_contract.balance_of(self.state.on_chain.client.address()).call().await?;

      if request.campaign_budget() > on_chain_balance {
        continue;
      } 

      let tx = self.state.transactional().await?;
      tx.campaign_request().find(request.id()).await?.create_campaign().await?;
      tx.commit().await?;
    }

    Ok(())
  }
}

impl CampaignRequest {
    pub async fn create_campaign(self) -> AsamiResult<Self> {
      if self.campaign_id().is_some() {
        return Err(Error::precondition("campaign_was_already_there"));
      }

      let app = self.state.clone();

      let account = self.state.get_admin_own_account().await?;

      let campaign = CreateCampaignFromLinkInput {
        link: self.link().clone(),
        topic_ids: self.topic_ids().await?,
        price_per_point: self.price_per_point().clone(),
        max_individual_reward: self.max_individual_reward().clone(),
        min_individual_reward: self.min_individual_reward().clone(),
        thumbs_up_only: *self.thumbs_up_only(),
      }.process(&app, &account).await?;
      let draft_req = self.update().status(CampaignRequestStatus::Draft).save().await?;

      app.on_chain.asami_contract.make_campaigns(vec![
        on_chain::MakeCampaignsParam {
          budget: draft_req.campaign_budget(),
          briefing_hash: campaign.decoded_briefing_hash(),
          valid_until: models::utc_to_i(Utc::now() + chrono::Duration::days(15))
        }
      ]).send().await?;

      campaign.mark_submitted().await?;
      Ok(draft_req.update().status(CampaignRequestStatus::Submitted).save().await?)
    }

    pub async fn topic_ids(&self) -> sqlx::Result<Vec<i32>> {
        Ok(self.campaign_request_topic_vec().await?.into_iter().map(|t| t.attrs.topic_id).collect())
    }

    pub fn campaign_budget(&self) -> U256 {
      let processing_fee_percent = U256::from(20);
      let unit_amount = U256::from(*self.unit_amount()) * U256::from(10).pow(U256::from(16));
      unit_amount * processing_fee_percent / U256::from(100)
    }
}

#[derive(Clone, GraphQLInputObject, Serialize, Deserialize)]
#[graphql(description = "The input for creating a new CampaignRequest.")]
#[serde(rename_all = "camelCase")]
pub struct CreateCampaignRequestInput {
    pub link: String,
    pub unit_amount: i32,
    pub topic_ids: Vec<i32>,
    pub price_per_point: String,
    pub max_individual_reward: String,
    pub min_individual_reward: String,
    pub thumbs_up_only: bool,
}

impl CreateCampaignRequestInput {
    pub async fn process(self, app: &App, account: &Account) -> AsamiResult<CampaignRequest> {
        use serde_json::json;
        use stripe::CheckoutSession;

        let _ = CreateCampaignFromLinkInput::validate_x_link_and_get_briefing(&self.link)?;

        let client = &app.stripe_client;
        let customer_id = account.get_or_create_stripe_customer_id().await?;
        let settings = &app.settings.stripe;

        let request = app.campaign_request().insert(InsertCampaignRequest{
          account_id: account.attrs.id.clone(),
          link: self.link.clone(),
          unit_amount: self.unit_amount,
          price_per_point: self.price_per_point,
          min_individual_reward: self.min_individual_reward,
          max_individual_reward: self.max_individual_reward,
          thumbs_up_only: self.thumbs_up_only,
        }).save().await?;

        let topics = app.topic().select().id_in(&self.topic_ids).all().await?;

        for t in topics {
            app.campaign_request_topic()
                .insert(InsertCampaignRequestTopic {
                    campaign_request_id: request.attrs.id,
                    topic_id: t.attrs.id,
                })
                .save()
                .await?;
        }

        let stripe_session : CheckoutSession = client.post_form("/checkout/sessions", json![{
          "success_url": settings.success_url,
          "cancel_url": settings.cancel_url,
          "customer": customer_id,
          "payment_method_types": ["card"],
          "mode": "payment",
          "payment_intent_data": {
            "metadata": {
              "campaign_request_id": request.id(),
            }
          },
          "line_items": [
            {
              "quantity": 1,
              "price_data": {
                "currency": "USD",
                "unit_amount": i64::from(self.unit_amount), // This amount is expressed in cents.
                "product_data": {
                  "name": "Managed Asami Campaign",
                  "description": format!("Boost for post: {}", self.link),
                }
              }
            }
          ]
        }]).await?;

        Ok(request.update()
          .status(CampaignRequestStatus::SessionCreated)
          .stripe_session_url(Some(stripe_session.url))
          .stripe_session_id(Some(stripe_session.id.to_string()))
          .save()
          .await?)
    }
}
