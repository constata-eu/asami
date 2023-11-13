#![allow(dead_code)]

#[macro_use]
mod support;

use ::api::models::*;

browser_test!{ browser_test (test_app, d)
  let value = "test-token";
  test_app.app.one_time_token().insert(InsertOneTimeToken{
    value: value.to_string()
  }).save().await?;

  d.goto("http://127.0.0.1:5173/#/one_time_token_login?token=test-token").await;
  d.wait_for("#advertiser-dashboard").await;
  d.click("#open-start-campaign-dialog").await;
  d.fill_in("#contentUrl", "https://x.com/asami_club/status/1716421161867710954?s=20").await;
  d.fill_in("#budget", "20").await;
  d.click("#submit-start-campaign-form").await;
  d.wait_for_text(".MuiSnackbarContent-message", "Campaign will be started soon").await;
  d.wait_until_gone(".MuiSnackbarContent-message").await;
  d.wait_for("#campaign-request-list").await;

  test_app.mock_admin_setting_campaign_requests_as_paid().await;
  test_app.run_idempotent_background_tasks_a_few_times().await;
  
  //d.goto("http://127.0.0.1:5173/#/?role=member").await;

  wait_here();

  // Stub creation of an account with a one-time-token for advertiser.
  
  // Creates a campaign (request).
  // Shows the status in the advertiser dashboard (pending campaigns).
  // Shows it done in the advertiser dashboard (running campaigns).

  // Stub creation of an account with a one-time-token for member.
  // Create verification request, stub twitter api creation.
  // Sees campaigns in his dashboard.
  // Chooses to participate in one (up until opening repost UI)

  // Stub detection of retweet on twitter.

  // Campaign remaining balance is updated.
  
  // Member "pending" balance updates. Including tokens. 
  // Member hits the "claim account" button.
  //
  // Rif login opens. 
  // User choses metamask.
  // Account is then claimed.
    // Account claiming is a type of request too.
  
  /*
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

  let account = test_app.app.account().find(weihex("1")).await?;
  let mut handle_req = account.create_handle_request(Site::X, "nubis_bruno").await?;
  handle_req = handle_req.verify("179383862".into()).await?;
  handle_req = handle_req.appraise(u("10"), u("10")).await?;

  test_app.app.handle_request().submit_all().await?;
  test_app.app.synced_event().sync_on_chain_events().await?;
  assert_eq!(handle_req.reloaded().await?.attrs.status, HandleRequestStatus::Done);

  let handle = test_app.app.handle().select().one().await?;

  //"https://x.com/asami_club/status/1716421161867710954?s=20",
  let campaign_req = account.create_campaign_request(
    Site::X,
    "1716421161867710954",
    u("50"),
    u("2"),
    Utc::now() + chrono::Duration::days(2),
  ).await?
  .pay().await?;

  test_app.app.campaign_request().submit_approvals().await?;
  test_app.app.synced_event().sync_on_chain_events().await?;
  assert_eq!(campaign_req.reloaded().await?.attrs.status, CampaignRequestStatus::Approved);

  test_app.app.campaign_request().submit_all().await?;
  test_app.app.synced_event().sync_on_chain_events().await?;
  assert_eq!(campaign_req.reloaded().await?.attrs.status, CampaignRequestStatus::Done);

  let mut campaign = test_app.app.campaign().select().one().await.unwrap();

  assert_that!(&campaign.attrs, structure![ CampaignAttrs {
    budget: eq(uhex("50")),
    remaining: eq(uhex("50")),
  }]);

  test_app.app.collab_request().insert(InsertCollabRequest{
    campaign_id: campaign.attrs.id.clone(),
    handle_id: handle.attrs.id.clone(),
  }).save().await?;

  test_app.app.collab_request().submit_all().await.unwrap();
  test_app.app.synced_event().sync_on_chain_events().await?;
  assert!(test_app.app.collab().select().count().await? == 1);

  campaign.reload().await?;

  assert_that!(&campaign.attrs, structure![ CampaignAttrs {
    budget: eq(uhex("50")),
    remaining: eq(uhex("40"))
  }]);
  */
}

// Test creation of an account + session with twitter.
// Test creation of an account + session with instagram.

// Test creation of an account + session with metamask.
  // Is this account automatically used for the protocol too?
  // Does it mean that account starts immediately claimed?
