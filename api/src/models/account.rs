use super::*;
use ethers::types::Address;

// This is an account that maps with the smart contract accounts.
// They may have several auth methods at first,
// but once they have the Eip signup method, all other methods get disabled.
model! {
  state: App,
  table: accounts,
  struct Account {
    #[sqlx_model_hints(varchar, default)]
    id: String,
    #[sqlx_model_hints(varchar)]
    name: Option<String>,
    #[sqlx_model_hints(account_status, default)]
    status: AccountStatus,
    #[sqlx_model_hints(varchar)]
    addr: Option<String>,
    #[sqlx_model_hints(timestamptz, default)]
    created_at: UtcDateTime,
    #[sqlx_model_hints(timestamptz, default)]
    updated_at: Option<UtcDateTime>,
    #[sqlx_model_hints(varchar, default)]
    claim_signature: Option<String>,
    #[sqlx_model_hints(varchar, default)]
    claim_session_id: Option<String>,
    #[sqlx_model_hints(boolean, default)]
    processed_for_legacy_claim: bool,
    #[sqlx_model_hints(boolean, default)]
    allows_gasless: bool,
  },
  has_many {
    Handle(account_id),
    Campaign(account_id),
    CampaignPreference(account_id),
  }
}

impl Account {
    pub fn is_claimed_or_claiming(&self) -> bool {
        matches!(self.status(), AccountStatus::Claiming | AccountStatus::Claimed)
    }

    pub fn decoded_addr(&self) -> AsamiResult<Option<Address>> {
        let Some(addr) = self.addr() else { return Ok(None) };
        let decoded: Address = Address::decode_hex(addr).map_err(|_| Error::validation("invalid_address", addr))?;
        Ok(Some(decoded))
    }

    pub async fn find_on_chain(
        &self,
    ) -> anyhow::Result<Option<(Address, U256, U256, U256, U256, U256, on_chain::FeeRateVote)>> {
        let Some(addr) = self.decoded_addr()? else {
            return Ok(None);
        };
        Ok(Some(self.state.on_chain.asami_contract.accounts(addr).call().await?))
    }

    pub async fn campaign_offers(&self) -> AsamiResult<Vec<Campaign>> {
        let handles = self.handle_scope().status_eq(HandleStatus::Active).all().await?;
        if handles.is_empty() {
            return Ok(vec![]);
        }

        let done: Vec<i32> = self
            .state
            .collab()
            .select()
            .member_id_eq(self.id())
            .all()
            .await?
            .into_iter()
            .map(|x| x.attrs.campaign_id)
            .collect();

        let ignored: Vec<i32> = self
            .campaign_preference_scope()
            .not_interested_on_is_set(true)
            .all()
            .await?
            .into_iter()
            .map(|x| x.attrs.campaign_id)
            .collect();

        let mut campaigns = vec![];

        for c in self.state.campaign().select().budget_gt(weihex("0")).all().await?.into_iter() {
            if c.valid_until().map(|end| end <= Utc::now()).unwrap_or(true) {
                continue;
            };
            if ignored.contains(c.id()) {
                continue;
            };
            if done.contains(c.id()) {
                continue;
            };
            if c.is_missing_ig_rules().await? {
                continue;
            };

            for h in &handles {
                let Some(trigger) = h.user_id() else {
                    continue;
                };
                if h.validate_collaboration(&c, h.reward_for(&c).unwrap_or_default(), trigger).await.is_ok() {
                    campaigns.push(c);
                    break;
                }
            }
        }

        Ok(campaigns)
    }

    pub async fn create_handle(&self, site: Site, username: &str) -> sqlx::Result<Handle> {
        self.state
            .handle()
            .insert(InsertHandle {
                account_id: self.attrs.id.clone(),
                username: username.to_string(),
                site,
            })
            .save()
            .await
    }

    pub async fn create_claim_account_request(
        &self,
        addr: String,
        signature: String,
        session_id: String,
    ) -> AsamiResult<Self> {
        if self.is_claimed_or_claiming() {
            return Err(Error::validation("account", "cannot_call_on_claimed_account"));
        }

        Ok(self
            .clone()
            .update()
            .addr(Some(addr))
            .status(AccountStatus::Claiming)
            .claim_signature(Some(signature))
            .claim_session_id(Some(session_id))
            .save()
            .await?)
    }

    pub async fn allow_gasless(self) -> AsamiResult<Self> {
        Ok(self.update().allows_gasless(true).save().await?)
    }

    pub async fn disallow_gasless(self) -> AsamiResult<Self> {
        Ok(self.update().allows_gasless(false).save().await?)
    }
}

make_sql_enum![
    "account_status",
    pub enum AccountStatus {
        Managed,
        Claiming,
        Claimed,
        Banned,
    }
];
