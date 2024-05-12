use super::{models::eip_712_sig_to_address, *};

#[derive(Clone, GraphQLInputObject, Serialize, Deserialize)]
#[graphql(description = "The input for creating a new ClaimAccountRequest.")]
#[serde(rename_all = "camelCase")]
pub struct CreateClaimAccountRequestInput {
    pub signature: String,
}

impl CreateClaimAccountRequestInput {
    pub async fn process(self, context: &Context) -> FieldResult<Account> {
        let address = eip_712_sig_to_address(context.app.settings.rsk.chain_id, &self.signature)
            .map_err(|msg| Error::Validation("eip_712_sig".to_string(), msg))?;

        let account = context
            .account()
            .await?
            .create_claim_account_request(
                address.clone(),
                self.signature,
                context.current_session()?.0.attrs.id.clone(),
            )
            .await?;

        Ok(Account::db_to_graphql(account).await?)
    }
}
