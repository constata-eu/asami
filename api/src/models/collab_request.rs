use super::*;

model!{
  state: App,
  table: collab_requests,
  struct CollabRequest {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(varchar)]
    campaign_id: String,
    #[sqlx_model_hints(varchar)]
    handle_id: String, 
    #[sqlx_model_hints(varchar, default)]
    tx_hash: Option<String>,
    #[sqlx_model_hints(varchar, default)]
    collab_id: Option<String>,
    #[sqlx_model_hints(collab_request_status, default)]
    status: CollabRequestStatus,
  },
  belongs_to {
    Campaign(campaign_id),
    Handle(handle_id),
    Collab(collab_id),
  }
}

impl CollabRequestHub {
  pub async fn submit_all(&self) -> AsamiResult<Vec<CollabRequest>> {
    let mut submitted = vec![];
    let reqs = self.select().status_eq(CollabRequestStatus::Received).all().await?;
    if reqs.is_empty() { return Ok(submitted); }

    let params = reqs.iter().map(|r| r.as_param() ).collect();

    let tx_hash = self.state.on_chain.contract
      .admin_make_collabs(params)
      .send().await?.await?
      .ok_or_else(|| Error::service("rsk_blockchain", "no_tx_recepit_for_admin_make_collabs"))?
      .transaction_hash.encode_hex();

    for req in reqs {
      submitted.push(
        req.update().status(CollabRequestStatus::Submitted).tx_hash(Some(tx_hash.clone())).save().await?
      );
    }

    Ok(submitted)
  }
}

impl CollabRequest {
  pub async fn done(self, collab: &Collab) -> sqlx::Result<Self> {
    self.update().status(CollabRequestStatus::Done)
      .collab_id(Some(collab.attrs.id.clone()))
      .save().await
  }

  pub fn as_param(&self) -> on_chain::AdminMakeCollabsInput {
    on_chain::AdminMakeCollabsInput{
      campaign_id: u256(&self.attrs.campaign_id),
      handle_id: u256(&self.attrs.handle_id),
    }
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "collab_request_status", rename_all = "snake_case")]
pub enum CollabRequestStatus {
  Received,   // The request was received by a managed user to create a collab.
  Submitted,  // We've tried to submit the request on-chain.
  Done,     // We've got the event that creates this collab.
}

impl sqlx::postgres::PgHasArrayType for CollabRequestStatus {
  fn array_type_info() -> sqlx::postgres::PgTypeInfo {
    sqlx::postgres::PgTypeInfo::with_name("_collab_request_status")
  }
}

