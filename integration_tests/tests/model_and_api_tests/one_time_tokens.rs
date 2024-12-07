app_test! { sends_one_time_tokens (a)
    // Creates a new one time token and someone uses it to log in.
    a.app.one_time_token().create_for_email(
      "alice@example.com".into(),
      api::Lang::Es,
      None
    ).await.unwrap();


    // The token is not longer usable after the first time.

    // The previous person adds a secondary login email.

    // A new person comes in, and claims that previous email
    // There's now only one auth method, for the most recent verifier of that email.
}

/*
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
*/
