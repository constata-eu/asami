use ::api::{ models::U256, on_chain::{ self, *}};
use crate::support::TestApp;

app_test!{ recreates_old_contract_and_performs_migration (a)
  let mut advertiser = a.client().await;
  advertiser.make_client_wallet().await;
  let mut alice = a.client().await;
  alice.make_client_wallet().await;
  let mut bob = a.client().await;
  bob.make_client_wallet().await;
  let mut stranger = a.client().await;
  stranger.make_client_wallet().await;

  a.send_tx(
    "Approving campaign budget on legacy contract",
    "46284",
    a.doc_contract().approve( a.legacy_contract().address(), u("100"))
  ).await;

  a.send_tx(
    "Makes a campaign in the old contract",
    "391077",
    a.legacy_contract().admin_make_campaigns(vec![AdminCampaignInput {
      account_id: advertiser.account_id(),
      attrs: on_chain::CampaignInput {
        site: models::Site::X as u8,
        budget: u("100"),
        content_id: "12121212".to_string(),
        price_score_ratio: u("10"),
        topics: vec![],
        valid_until: models::utc_to_i(Utc::now() + chrono::Duration::days(10)),
      }
    }])
  ).await;

  a.send_tx(
    "Adds handles for alice and bob",
    "596368",
    a.legacy_contract().admin_make_handles(vec![
      on_chain::Handle {
        id: U256::zero(),
        account_id: alice.account_id(),
        site: models::Site::X as u8,
        price: u("10"),
        score: wei("100"),
        topics: vec![],
        username: "alice_on_x".into(),
        user_id: "12345".into(),
        last_updated: 0.into(),
        new_price: u("10"),
        new_score: wei("100"),
        new_topics: vec![],
        new_username: "alice_on_x".into(),
        needs_update: false,
      },
      on_chain::Handle {
        id: U256::zero(),
        account_id: bob.account_id(),
        site: models::Site::X as u8,
        price: u("20"),
        score: wei("200"),
        topics: vec![],
        username: "bob_on_x".into(),
        user_id: "67890".into(),
        last_updated: 0.into(),
        new_price: u("20"),
        new_score: wei("200"),
        new_topics: vec![],
        new_username: "bob_on_x".into(),
        needs_update: false,
      },
    ])
  ).await;

  a.send_tx(
    "Makes collabs for alice and bob",
    "888997",
    a.legacy_contract().admin_make_collabs(vec![
      AdminMakeCollabsInput{ campaign_id: wei("1"), handle_id: wei("1") },
      AdminMakeCollabsInput{ campaign_id: wei("1"), handle_id: wei("2") },
    ])
  ).await;

  a.send_tx(
    "Claiming advertiser and alice accounts so they get tokens",
    "387500",
    a.legacy_contract().claim_accounts(vec![
      AdminClaimAccountsInput{ account_id: advertiser.account_id(), addr: advertiser.address()},
      AdminClaimAccountsInput{ account_id: alice.account_id(), addr: alice.address()},
    ])
  ).await;

  a.start_mining().await;
  alice.legacy_contract().transfer(stranger.address(), milli("50")).send().await.unwrap().await.unwrap().unwrap();
  a.stop_mining().await;

  assert_legacy_account(&a, advertiser.account_id(), advertiser.address(), u("0"), u("0") ).await;
  assert_legacy_account(&a, alice.account_id(), alice.address(), u("0"), u("0") ).await;
  assert_legacy_account(&a, bob.account_id(), Address::zero(), milli("300"), u("18") ).await;

  assert_legacy_balance(&a, "advertiser", advertiser.address(), milli("300")).await;
  assert_legacy_balance(&a, "alice", alice.address(), milli("100")).await;
  assert_legacy_balance(&a, "bob", bob.address(), u("0")).await;
  assert_legacy_balance(&a, "admin", a.admin_treasury_address().await, milli("2250")).await;
  assert_legacy_balance(&a, "stranger", stranger.address(), milli("50")).await;

  assert_eq!(
    a.legacy_contract().get_holders().call().await.unwrap(),
    vec![ a.admin_treasury_address().await, advertiser.address(), alice.address(), stranger.address()]
  );

  assert_eq!(a.legacy_contract().total_supply().call().await.unwrap(), milli("2700"));

  a.send_tx(
    "Migrating 2 accounst and 2 holders",
    "478045",
    a.asami_core().migrate_tokens_from_old_contract(a.legacy_contract().address(), wei("2"))
  ).await;

  a.send_tx(
    "Migrating all the rest",
    "307984",
    a.asami_core().migrate_tokens_from_old_contract(a.legacy_contract().address(), wei("10"))
  ).await;

  a.send_revert_tx(
    "All done, no more migration allowed.",
    "mig0",
    a.asami_core().migrate_tokens_from_old_contract(a.legacy_contract().address(), wei("20"))
  ).await;

  assert_account(&a, advertiser.account_id(), advertiser.address(), u("5"), u("0")).await;
  assert_account(&a, alice.account_id(), alice.address(), u("5"), u("0")).await;
  assert_account(&a, bob.account_id(), Address::zero(), u("0"), milli("300")).await;
  
  a.assert_balances_of("new advertiser", advertiser.account_id(), u("10"), u("0"), u("0"), u("0"), milli("300")).await;
  a.assert_balances_of("new alice", alice.account_id(), wei("9999698360000000000"), u("0"), u("9"), u("0"), milli("100")).await;
  a.assert_balances_of("new bob", bob.account_id(), u("0"), u("0"), u("0"), milli("300"), milli("0")).await;
  assert_eq!(a.asami_balance_of(&a.admin_treasury_address().await).await, milli("2250"));
  assert_eq!(a.asami_balance_of(&stranger.address()).await, milli("50"));

  assert_eq!(a.asami_core().total_supply().call().await.unwrap(), milli("2700"));
}

async fn assert_legacy_account(a: &TestApp, account_id: U256, addr: Address, unclaimed_asami: U256, unclaimed_doc: U256) {
  let account = a.legacy_contract().accounts(account_id).call().await
    .expect(&format!("could not fetch account {account_id}"));
  assert_eq!( account, (account_id, addr, unclaimed_asami, unclaimed_doc) );
}

async fn assert_account(a: &TestApp, account_id: U256, addr: Address, allowance: U256, unclaimed_asami: U256) {
  let account = a.asami_core().accounts(account_id).call().await
    .expect(&format!("could not fetch account {account_id}"));
  assert_eq!( account, (addr, allowance, unclaimed_asami, ::api::models::u("0")) );
}
async fn assert_legacy_balance(a: &TestApp, msg: &str, addr: Address, expected: U256) {
  assert_eq!( a.legacy_contract().balance_of(addr).call().await.unwrap(), expected, "on {msg}");
}