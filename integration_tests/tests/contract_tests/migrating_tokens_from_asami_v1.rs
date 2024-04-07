app_test!{ measures_old_contract_gas_spending (a)
  let contract = a.contract();

  let advertiser = a.client().await;

  a.send_tx("approval", "1000", a.doc_contract().approve( contract.address(), u("800"))).await;

  let valid_until = Utc::now() + chrono::Duration::days(2);

  a.send_tx("make campaign", "2000",
    contract.admin_make_campaigns(vec![
      on_chain::AdminCampaignInput {
        account_id: u256(advertiser.account().await.attrs.id),
        attrs: on_chain::CampaignInput {
          site: Site::X as u8,
          budget: u("100"),
          content_id: "1000".to_string(),
          price_score_ratio: u("20"),
          topics: vec![],
          valid_until: utc_to_i(valid_until)
        }
      }
    ])
  ).await;

  let mut params = vec![];
  for _i in 0..20 {
    let mut bob = a.client().await;
    bob.create_x_handle("bob_on_x", u("1")).await;
    bob.claim_account().await;
    let handle = bob.x_handle().await;
    params.push(
      AdminMakeCollabsInput{ campaign_id: wei("1"), handle_id: u256(&handle.attrs.id) }
    );
  }

  let collabs = a.send_tx("making collabs", "1000", contract.admin_make_collabs(params)).await;
  dbg!(collabs.gas_used().as_ref().map(u256));
}

