#[macro_use]
pub mod support;
use api::models::*;

app_test!{ logs_on_chain_txs (a) 
  let advertiser = a.client().await;
  let campaign = advertiser.create_x_campaign(u("10"), u("10")).await;

  let bob = a.client().await;
  bob.create_x_handle("bob_on_x", u("1")).await;
  bob.create_x_collab(&campaign).await;
  assert!(
    a.app.on_chain_tx().find(1).await?.audit_log_entries().await?
    .iter().all(|x| *x.severity() == AuditLogSeverity::Info)
  );
}

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
  assert!(bob_req.on_chain_tx().await?.unwrap().success());
}
