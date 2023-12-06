#[macro_use]
mod support;
//use ::api::models::*;

/*
api_test!{ signs_up_and_makes_collab_in_x (test_app, client)
  let value = "test+token";
  test_app.app.one_time_token().insert(InsertOneTimeToken{
    value: value.to_string()
  }).save().await?;

  let login_pubkey = URL_SAFE_NO_PAD.encode(
    test_app.private_key().public_key().to_pem().unwrap()
  );

  let _created: create_session::ResponseData = client.gql(
    &CreateSession::build_query(create_session::Variables{}),
    vec![
      Header::new("Auth-Action", "Login"),
      Header::new("Auth-Method-Kind", "OneTimeToken"),
      Header::new("Auth-Data", value),
      Header::new("Login-Pubkey", login_pubkey),
      Header::new("New-Session-Recaptcha-Code", "SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c"),
    ]
  ).await;

  let account = test_app.app.account().find(1).await?;

  let mut handle = account.add_handle(Site::X, "nubis_bruno").await?;

  test_app.app.handle().verify_and_appraise_x().await.unwrap();

  handle.reload().await?;

  assert_that!(&handle.attrs, structure![ HandleAttrs {
    status: eq(HandleRequestStatus::Appraised),
    fixed_id: maybe_some(rematch("179383862")),
    verification_message_id: maybe_some(rematch("1713930237598150807")),
    //score: maybe_some(eq(Decimal::new(3393, 0)))
  }]);

  let submitted = handle.submit().await?;

  assert_that!(&submitted.attrs, structure![ HandleAttrs {
    status: eq(HandleRequestStatus::Submitted),
    tx_hash: maybe_some(any_value())
  }]);

  tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

  test_app.app.sync_on_chain_events().await?;

  assert_that!(&submitted.reloaded().await?.attrs, structure![ HandleAttrs {
    status: eq(HandleRequestStatus::Active),
  }]);

  //"https://x.com/asami_club/status/1716421161867710954?s=20",
  let campaign_req = account.create_campaign_request(
    Site::X,
    "1716421161867710954",
    Decimal::new(10,0),
  ).await?
  .pay().await?
  .submit().await?;

  test_app.app.sync_on_chain_events().await.unwrap();

  let mut campaign = test_app.app.campaign().select().one().await.unwrap();

  assert_that!(&campaign.attrs, structure![ CampaignAttrs {
    budget: eq(Decimal::new(10,0)),
    remaining: eq(Decimal::new(10,0))
  }]);

  test_app.app.campaign().sync_x_collabs().await.unwrap();

  assert!(test_app.app.collab_request().select().count().await? == 1);

  test_app.app.collab_request().submit_all().await.unwrap();

  test_app.app.sync_on_chain_events().await.unwrap();

  assert!(test_app.app.collab().select().count().await? == 1);

  campaign.reload().await?;

  assert_that!(&campaign.attrs, structure![ CampaignAttrs {
    budget: eq(Decimal::new(10,0)),
    remaining: eq(Decimal::new(7,0))
  }]);
}
*/

