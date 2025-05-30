use api::{
    models::*,
    on_chain::{self, *},
};

use super::*;

app_test! { basic_workflow_with_claim_account_example (a)
  assert_eq!(a.app.on_chain_job().select().count().await?, 0);
  a.app.on_chain_job().run_scheduler().await.context("first scheduler run")?;

  let all = a.app.on_chain_job().select().all().await?;
  assert_eq!(all.len(), 10);
  assert!(all.iter().all(|x| x.status() == &OnChainJobStatus::Scheduled));
  let mut current = a.app.on_chain_job().current().one().await?;

  a.app.on_chain_job().run_scheduler().await?;

  current.reload().await?;
  assert_eq!(current.status(), &OnChainJobStatus::Skipped);

  let mut alice = a.client().await;
  alice.submit_claim_account_request().await;

  let submitted = a.wait_for_job(
    "A job promoting sub accounts",
    OnChainJobKind::PromoteSubAccounts,
    OnChainJobStatus::Submitted
  ).await;

  a.wait_for_job_status("Submitted job confirms", &submitted, OnChainJobStatus::Confirmed).await;
  assert_eq!(alice.account().await.attrs.status, AccountStatus::Claiming);

  a.wait_for_job_status("Submitted job settles", &submitted, OnChainJobStatus::Settled).await;
  assert_eq!(alice.account().await.attrs.status, AccountStatus::Claimed);
}

app_test! { stops_scheduling_when_admin_cant_pay_gas (a)
  let mut alice = a.client().await;
  alice.submit_claim_account_request().await;

  { // Make sure admin has no funds.
    let admin = &a.client_admin_address();
    let almost_all = a.rbtc_balance_of(admin).await - (wei("21000") * wei("2000000000"));
    a.send_rbtc_to(alice.address(), almost_all).await;
    assert_eq!(a.rbtc_balance_of(admin).await, wei("0"));
  }

  a.wait_for_job(
    "A job promoting sub accounts",
    OnChainJobKind::PromoteSubAccounts,
    OnChainJobStatus::Scheduled
  ).await;

  let job = a.app.on_chain_job().current().one().await.unwrap();
  assert_eq!(job.kind(), &OnChainJobKind::PromoteSubAccounts);
  a.app.on_chain_job().run_scheduler().await.unwrap();

  assert_eq!(job.status(), &OnChainJobStatus::Scheduled);
  assert_eq!(job.reloaded().await?.attrs.status_line.unwrap(), "insufficient funds for gas * price + value");
}

app_test! { admin_legacy_claim_account (a)
  let mut advertiser = a.client().await;
  advertiser.make_client_wallet().await;
  let mut alice = a.client().await;
  alice.make_client_wallet().await;

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
    "Adds handle for alice",
    "323486",
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
      }
    ])
  ).await;

  a.send_tx(
    "Makes collabs for alice",
    "611985",
    a.legacy_contract().admin_make_collabs(vec![
      AdminMakeCollabsInput{ campaign_id: wei("1"), handle_id: wei("1") },
    ])
  ).await;

  // Bob is a new user and should be marked as processed for legacy claim immediately.
  let mut bob = a.client().await;
  bob.make_client_wallet().await;

  alice.submit_claim_account_request().await;
  bob.submit_claim_account_request().await;

  let submitted = a.wait_for_job(
    "Admin Legacy claim account job",
    OnChainJobKind::AdminLegacyClaimAccount,
    OnChainJobStatus::Submitted
  ).await;

  // As expected, bob is marked as processed without actually going through the process.
  assert!(bob.account().await.processed_for_legacy_claim());
  assert!(!alice.account().await.processed_for_legacy_claim());

  a.wait_for_job_status("submitted job settles", &submitted, OnChainJobStatus::Settled).await;
  assert!(alice.account().await.processed_for_legacy_claim());
}
