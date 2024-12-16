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

    // These columns contain on-chain values that can be updated in a time based fashion.
    // They should at least be updated when a DOC transfer is done
    // Every time the user requests to.
    // - When a collab occurs.
    // - When a gasless claim occurs.
    // - When a campaign is created.
    // - When a claim occurs.
    // - When ASAMI tokens are transferred.
    // - When DOC tokens are transferred.
    #[sqlx_model_hints(varchar, default)]
    unclaimed_asami_balance: String,
    #[sqlx_model_hints(varchar, default)]
    unclaimed_doc_balance: String,
    #[sqlx_model_hints(varchar, default)]
    asami_balance: String,
    #[sqlx_model_hints(varchar, default)]
    doc_balance: String,
    #[sqlx_model_hints(varchar, default)]
    rbtc_balance: String,
    #[sqlx_model_hints(timestamptz, default)]
    last_on_chain_sync: UtcDateTime,

    // These columns are part of the account activity report
    // they are denormalized and re-hydrated when:
    // - A campaign is set to 'Published'
    // - A collab for one of the user handles user is settled.
    // - A a collab for one of the user campaigns is settled.
    #[sqlx_model_hints(boolean, default)]
    force_hydrate: bool,
    #[sqlx_model_hints(int4, default)]
    total_collabs: i32,
    #[sqlx_model_hints(varchar, default)]
    total_collab_rewards: String,
    #[sqlx_model_hints(int4, default)]
    total_campaigns: i32,
    #[sqlx_model_hints(int4, default)]
    total_collabs_received: i32,
    #[sqlx_model_hints(varchar, default)]
    total_spent: String,
  },
  has_many {
    Handle(account_id),
    Campaign(account_id),
    CampaignPreference(account_id),
  }
}

impl AccountHub {
    pub async fn force_hydrate(&self) -> AsamiResult<()> {
        let ids = self.state.db.fetch_all_scalar(
            sqlx::query_scalar!("SELECT id FROM accounts WHERE force_hydrate = true LIMIT 50")
        ).await?;
        if ids.is_empty() {
            return Ok(())
        }
        self.hydrate_report_columns_for(ids.iter()).await?;
        self.hydrate_on_chain_columns_for(ids.iter()).await?;
        self.state.db.execute(
            sqlx::query!("UPDATE accounts SET force_hydrate = false WHERE id = ANY($1)", &ids)
        ).await?;
        Ok(())
    }

    pub async fn hydrate_report_columns_for(&self, ids: impl Iterator<Item = &String>) -> AsamiResult<()> {
        for id in ids {
            let account = self.find(id).await?;
            let mut total_collabs = 0;
            let mut total_collab_rewards = u("0");
            for collab in account.collabs_made().status_eq(CollabStatus::Cleared).all().await? {
                total_collabs += 1;
                total_collab_rewards += collab.reward_u256();
            }

            let total_campaigns = account.campaign_scope().status_eq(CampaignStatus::Published).count().await?;  

            let mut total_collabs_received = 0;
            let mut total_spent = u("0");
            for collab in account.collabs_received().status_eq(CollabStatus::Cleared).all().await? {
                total_collabs_received += 1;
                total_spent += collab.reward_u256();
            }

            account
                .update()
                .total_collabs(total_collabs)
                .total_collab_rewards(total_collab_rewards.encode_hex())
                .total_campaigns(total_campaigns.try_into()?)
                .total_collabs_received(total_collabs_received)
                .total_spent(total_spent.encode_hex())
                .save()
                .await?;
        }

        Ok(())
    }

    pub async fn hydrate_on_chain_columns_for(&self, ids: impl Iterator<Item = &String>) -> AsamiResult<()> {
        let chain = &self.state.on_chain;
        let asami = &chain.asami_contract;

        for id in ids {
            let account = self.find(id).await?;
            let decoded_address = account.decoded_addr()?;
            let mut updater = account.update();

            updater = match decoded_address {
                Some(address) => {
                    let on_chain = asami.accounts(address).call().await?;
                    updater
                        .doc_balance(
                            chain.doc_contract.balance_of(address).call().await?.encode_hex()
                        )
                        .asami_balance(
                            asami.balance_of(address).call().await?.encode_hex()
                        )
                        .rbtc_balance(
                            asami.client().get_balance(address, None).await?.encode_hex()
                        )
                        .unclaimed_doc_balance(on_chain.4.encode_hex())
                        .unclaimed_asami_balance(on_chain.3.encode_hex())
                },
                None => {
                    let admin = self.state.settings.rsk.admin_address;
                    let (docs, asamis) = asami
                        .get_sub_account(admin, u256(id))
                        .call()
                        .await
                        .map(|s| {
                            (
                                s.unclaimed_doc_balance.encode_hex(),
                                s.unclaimed_asami_balance.encode_hex(),
                            )
                        })
                        .unwrap_or_else(|_| (weihex("0"), weihex("0")));

                    updater
                        .unclaimed_asami_balance(asamis)
                        .unclaimed_doc_balance(docs)
                }
            };

            updater.save().await?;
        }

        Ok(())
    }

    pub async fn hydrate_on_chain_values_just_in_case(&self) -> AsamiResult<()> {
        let now = Utc::now();

        let items = self
            .select()
            .last_on_chain_sync_lt(now - Duration::minutes(60))
            .limit(50)
            .all()
            .await?;

        if items.is_empty() {
            return Ok(());
        }

        self.hydrate_on_chain_columns_for(items.iter().map(|i| i.id()) ).await?;

        for i in items {
            i.update().last_on_chain_sync(now).save().await?;
        }

        Ok(())
    }
}

impl Account {
    pub fn collabs_made(&self) -> SelectCollabHub {
        self.state.collab().select().member_id_eq(self.id())
    }

    pub fn collabs_received(&self) -> SelectCollabHub {
        self.state.collab().select().advertiser_id_eq(self.id())
    }

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
