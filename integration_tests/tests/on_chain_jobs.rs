#[macro_use]
pub mod support;
use api::models::*;
use ::api::on_chain::{ self, *};
use crate::support::TestApp;

app_test!{ basic_workflow_with_claim_account_example (a) 
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

  try_until(100, 10, "A job for promoting sub accounts was never submitted.", || async {
    a.app.on_chain_job().run_scheduler().await.unwrap();
    let Some(job) = a.app.on_chain_job().current().optional().await.unwrap() else { return false };
    job.attrs.status == OnChainJobStatus::Submitted && job.attrs.kind == OnChainJobKind::PromoteSubAccounts
  }).await;

  let submitted = a.app.on_chain_job().current().one().await.unwrap();
  assert_eq!(submitted.status(), &OnChainJobStatus::Submitted);

  try_until(100, 10, "Submitted job never got confirmed", || async {
    a.evm_mine().await;
    a.app.on_chain_job().run_scheduler().await.unwrap();
    let reloaded = submitted.reloaded().await.unwrap();
    reloaded.attrs.status == OnChainJobStatus::Confirmed
  }).await;

  assert_eq!(alice.account().await.attrs.status, AccountStatus::Claiming);

  try_until(100, 10, "Submitted job never got settled", || async {
    a.evm_mine().await;
    a.app.on_chain_job().run_scheduler().await.unwrap();
    let reloaded = submitted.reloaded().await.unwrap();
    reloaded.attrs.status == OnChainJobStatus::Settled
  }).await;

  assert_eq!(alice.account().await.attrs.status, AccountStatus::Claimed);
}

app_test!{ admin_legacy_claim_account (a)
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
  a.app.on_chain_job().run_scheduler().await.context("first scheduler run")?;

  try_until(100, 10, "A job for AdminLegacyClaimAccount was never submitted", || async {
    a.evm_mine().await;
    a.app.on_chain_job().run_scheduler().await.unwrap();
    let Some(job) = a.app.on_chain_job().current().optional().await.unwrap() else { return false };
    job.attrs.status == OnChainJobStatus::Submitted && job.attrs.kind == OnChainJobKind::AdminLegacyClaimAccount
  }).await;

  let submitted = a.app.on_chain_job().current().one().await.unwrap();
  assert_eq!(submitted.status(), &OnChainJobStatus::Submitted);

  assert!(bob.account().await.processed_for_legacy_claim());
  assert!(!alice.account().await.processed_for_legacy_claim());

  try_until(100, 10, "Submitted job never got settled", || async {
    a.evm_mine().await;
    a.app.on_chain_job().run_scheduler().await.unwrap();
    let reloaded = submitted.reloaded().await.unwrap();
    reloaded.attrs.status == OnChainJobStatus::Settled
  }).await;

  assert!(alice.account().await.processed_for_legacy_claim());
}

/*
 
todo!("Revert because it was out of gas makes the job sleep longer and *not* change to reverted, while still logging the issue");
app_test!{ pre_validates_campaign_requests_before_bulk_submission (a)
  let mut alice = a.client().await;
  let mut alice_req = alice.build_x_campaign(u("10"), u("10"), 2, &[]).await;
  let bob = a.client().await;
  let mut bob_req = bob.build_x_campaign(u("10"), u("10"), 2, &[]).await;

  a.app.campaign_request().submit_approvals().await?;
  alice.submit_claim_account_request().await;
  a.app.claim_account_request().submit_all().await?;
  a.app.synced_event().sync_on_chain_events().await?;
  a.app.campaign_request().submit_all().await?;

  alice_req.reload().await?;
  assert_eq!(*alice_req.status(), CampaignRequestStatus::Failed);
  let entry = &alice_req.audit_log_entries().await?[0];
  assert_eq!(entry.subkind(), "cannot_submit_for_claimed_account");
  assert_eq!(*entry.severity(), AuditLogSeverity::Fail);

  bob_req.reload().await?;
  assert_eq!(*bob_req.status(), CampaignRequestStatus::Submitted);
  assert!(bob_req.submission().await?.unwrap().submitted());

  a.run_idempotent_background_tasks_a_few_times().await;
  bob_req.reload().await?;
  assert_eq!(*bob_req.status(), CampaignRequestStatus::Done);
  assert!(bob_req.submission().await?.unwrap().success());
}

app_test!{ pre_validates_admin_set_price_requests_before_bulk_submission (a)
  let mut alice = a.client().await;
  alice.create_x_handle("alice_on_x", u("10")).await;
  let bob = a.client().await;
  bob.create_x_handle("bob_on_x", u("10")).await;

  let mut bob_req = a.app.set_price_request().create(&bob.x_handle().await, u("2")).await?;
  let mut alice_req = a.app.set_price_request().create(&alice.x_handle().await, u("2")).await?;

  alice.submit_claim_account_request().await;
  a.app.claim_account_request().submit_all().await?;
  a.app.synced_event().sync_on_chain_events().await?;

  a.app.set_price_request().submit_all().await?;

  alice_req.reload().await?;
  assert_eq!(*alice_req.status(), GenericRequestStatus::Failed);
  let entry = &alice_req.audit_log_entries().await?[0];
  assert_eq!(entry.subkind(), "cannot_submit_for_claimed_account");
  assert_eq!(*entry.severity(), AuditLogSeverity::Fail);

  bob_req.reload().await?;
  assert_eq!(*bob_req.status(), GenericRequestStatus::Submitted);
  assert!(bob_req.on_chain_job().await?.unwrap().submitted());

  a.run_idempotent_background_tasks_a_few_times().await;
  bob_req.reload().await?;
  assert_eq!(*bob_req.status(), GenericRequestStatus::Done);
  assert!(bob_req.on_chain_job().await?.unwrap().success());
}
*/
