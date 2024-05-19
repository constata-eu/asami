use super::*;

impl OnChainJob {
    pub async fn gasless_claim_balances_make_call(&self) -> anyhow::Result<Option<AsamiFunctionCall>> {
        // Admin will skip gasless claims if funds are low.
        if self.state.on_chain.admin_rbtc_balance().await? < milli("5") {
            return Ok(None);
        }

        let rbtc_per_user = self.state.settings.rsk.gasless_rbtc_per_user;
        let doc_fee = self.state.settings.rsk.gasless_fee;

        let accounts = self.state.account().select().status_eq(AccountStatus::Claimed).all().await?;

        let mut addresses = vec![];

        for a in accounts {
            let Some(found) = a.find_on_chain().await? else {
                continue;
            };
            let (
                trusted_admin,
                max_gasless_doc_to_spend,
                min_gasless_rbtc_to_receive,
                _,
                unclaimed_doc_balance,
                _,
                _,
            ) = found;

            if trusted_admin != Address::default() && trusted_admin != self.state.settings.rsk.admin_address {
                continue;
            }

            if rbtc_per_user < min_gasless_rbtc_to_receive {
                continue;
            }

            if max_gasless_doc_to_spend < doc_fee {
                continue;
            }

            if unclaimed_doc_balance < doc_fee {
                continue;
            }

            let Some(addr) = a.decoded_addr()? else {
                continue;
            };
            addresses.push(addr);

            self.link_account(&a).await?;

            if addresses.len() > 50 {
                break;
            }
        }

        if addresses.is_empty() {
            return Ok(None);
        }

        let total_rbtc = U256::from(addresses.len()) * rbtc_per_user;

        return Ok(Some(
            self.contract()
                .gasless_claim_balances(doc_fee, rbtc_per_user, addresses)
                .value(total_rbtc),
        ));
    }
}
