#[macro_use]
pub mod support;
use api::models::*;

app_test!{ runs_on_chain_jobs (a) 
  assert_eq!(a.app.on_chain_job().select().count().await?, 0);
  a.app.on_chain_job().run_scheduler().await?;

  let all = a.app.on_chain_job().select().all().await?;
  assert_eq!(all.len(), 10);
  assert!(all.iter().all(|x| x.status() == &OnChainJobStatus::Scheduled));

  a.app.on_chain_job().run_scheduler().await?;

  let current = a.app.on_chain_job().current().one().await?;
  assert_eq!(current.status(), &OnChainJobStatus::Skipped);
  assert_eq!(current.status_line().as_ref().unwrap(), "no_subaccounts_to_promote");

  /*
  let bob = a.client().await;

  let request = bob.account().await.create_handle(models::Site::X, "bob_on_x").await.unwrap()
    .verify("179383862".into()).await.unwrap()
    .appraise(wei("10"), wei("10")).await.unwrap();

  assert!(a.app.on_chain_job().select().all().await?.is_empty());

  a.app.handle_request().submit_all().await?;
  let mut on_chain_job = a.app.on_chain_job().find(1).await?;

  assert_eq!(on_chain_job.kind, OnChainJobKind::PromoteSubAccounts);
  assert!(on_chain_job.submitted());

  a.run_idempotent_background_tasks_a_few_times().await;

  on_chain_job.reload().await?;

  assert!(on_chain_job.success());

  assert!(
    on_chain_job.audit_log_entries().await?.iter().all(|x| *x.severity() == AuditLogSeverity::Info)
  );

  assert_eq!(*request.reloaded().await?.status(), HandleRequestStatus::Done);
  */
}

/*
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
