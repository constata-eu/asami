#[macro_use]
pub mod support;
use ::api::models::*;

browser_test!{ screenshot_capturer (mut d)
  let a = d.test_app();
  let app = &a.app;
  let advertiser = a.client().await;
  let account = advertiser.account().await;
  let valid_until = Utc::now() + chrono::Duration::days(10);

  for post in &[
    ("C2w6_ThRgkY", "120"),
    ("C3TJIgLgULU", "415"),
  ] {
    account.create_campaign_request(models::Site::Instagram, post.0, u(post.1), u("1"), valid_until, &[])
      .await.unwrap().pay()
      .await.unwrap();
  }

  for post in &[
    ("1716421161867710954", "33"),
    ("1758208064467935359", "20"),
    ("1758116416606163059", "55"),
    ("1752961229407375400", "60"),
    ("1758192957386342435", "18"),
    ("1758192965703647443", "45"),
  ] {
    account.create_campaign_request(models::Site::X, post.0, u(post.1), u("1"), valid_until, &[])
      .await.unwrap().pay()
      .await.unwrap();
  }

  for post in &[
    ("C3PuyCPKNWT", "200"),
    ("C3GOsATrTTb", "411"),
    ("C3Ai4_KLqo7", "125"),
    ("C3SaabGsI1j", "366"),
    ("C222POpvKho", "72"),
  ] {
    account.create_campaign_request(models::Site::Instagram, post.0, u(post.1), u("1"), valid_until, &[])
      .await.unwrap().pay()
      .await.unwrap();
  }

  dbg!("Done creating campaign requests");

  try_until(10, 500, "campaigns not created", || async {
    a.run_idempotent_background_tasks_a_few_times().await;
    a.app.campaign().select().count().await.unwrap() == 13
  }).await;

  dbg!("Crawling for campaign rules on IG");
  a.app.ig_crawl().do_everything().await.unwrap();
  let crawl = a.app.ig_crawl().find(1).await?;

  try_until(20, 5000, "no ig crawl", || async {
    a.app.ig_crawl().do_everything().await.unwrap();
    *crawl.reloaded().await.unwrap().processed_for_campaign_rules()
  }).await;

  dbg!("Crawling for campaign rules on IG");
  d.goto("http://127.0.0.1:5173").await;
  d.wait_for("#button-login-as-member").await;
  pause_a_bit();
  pause_a_bit();
  pause_a_bit();
  pause_a_bit();
  pause_a_bit();
  pause_a_bit();
  d.driver.screenshot(std::path::Path::new("landing.png")).await.unwrap();
  d.click("#button-login-as-member").await;
  dbg!("now focus on login options");
  pause_a_bit();
  pause_a_bit();
  pause_a_bit();
  pause_a_bit();
  d.driver.screenshot(std::path::Path::new("login_options.png")).await.unwrap();

  app.one_time_token().insert(InsertOneTimeToken{value: "advertiser-token".to_string() }).save().await?;
  d.goto("http://127.0.0.1:5173/#/one_time_token_login?token=advertiser-token").await;
  d.wait_for("#member-dashboard").await;
  d.goto("http://127.0.0.1:5173/#/?role=advertiser").await;
  d.wait_for("#advertiser-dashboard").await;

  d.click("#open-start-campaign-request-dialog").await;
  d.fill_in("#contentUrl", "https://x.com/asami_club/status/1716421161867710954?s=20").await;
  d.driver.screenshot(std::path::Path::new("suggesting_campaign.png")).await.unwrap();
  d.click("#submit-start-campaign-form").await;
  d.wait_for_text(".MuiSnackbarContent-message", "Your suggestion has been received!").await;
  d.wait_until_gone(".MuiSnackbarContent-message").await;
  d.wait_for("#campaign-request-list").await;
  dbg!("now focus on campaign received");
  pause_a_bit();
  pause_a_bit();
  pause_a_bit();
  pause_a_bit();
  d.driver.screenshot(std::path::Path::new("campaign_request_received.png")).await.unwrap();
  d.click("#button-logout").await;

  app.one_time_token().insert(InsertOneTimeToken{value: "member-token".to_string() }).save().await?;
  d.goto("http://127.0.0.1:5173/#/one_time_token_login?token=member-token").await;
  d.wait_for("#advertiser-dashboard").await;

  d.goto("http://127.0.0.1:5173/#/?role=member").await;
  d.wait_for("#member-dashboard").await;
  dbg!("focus member dashboard");
  pause_a_bit();
  pause_a_bit();
  pause_a_bit();
  pause_a_bit();
  d.driver.screenshot(std::path::Path::new("member_dashboard.png")).await.unwrap();

  d.fill_in("#x-handle-request-form #username", "john_doe").await;
  d.fill_in("#ig-handle-request-form #username", "john_doe").await;
  dbg!("focus member handle request usernames");
  pause_a_bit();
  pause_a_bit();
  pause_a_bit();
  pause_a_bit();
  d.driver.screenshot(std::path::Path::new("handle_request_usernames.png")).await.unwrap();
  d.click("#submit-x-handle-request-form").await;
  d.wait_for(".MuiSnackbarContent-message").await;
  d.wait_until_gone(".MuiSnackbarContent-message").await;

  d.fill_in("#ig-handle-request-form #username", "john_doe").await;
  d.click("#submit-ig-handle-request-form").await;
  d.wait_for(".MuiSnackbarContent-message").await;
  d.wait_until_gone(".MuiSnackbarContent-message").await;

  dbg!("focus on instructions");
  pause_a_bit();
  pause_a_bit();
  pause_a_bit();
  pause_a_bit();
  d.driver.screenshot(std::path::Path::new("handle_request_instructions.png")).await.unwrap();

  app.handle_request().find(1).await.unwrap()
    .verify("12345678".into()).await.unwrap()
    .appraise(u("1"), wei("10400")).await.unwrap();

  app.handle_request().find(2).await.unwrap()
    .verify("734894398".into()).await.unwrap()
    .appraise(u("2"), wei("22100")).await.unwrap();
  d.wait_for("#handle-x-submission-in-progress-message").await;
  d.wait_for("#handle-ig-submission-in-progress-message").await;
  d.driver.screenshot(std::path::Path::new("handle_request_submitting.png")).await.unwrap();

  a.run_idempotent_background_tasks_a_few_times().await;
  d.wait_for("#existing-x-handle-stats").await;
  d.wait_for("#existing-ig-handle-stats").await;
  dbg!("focus on handle stats");
  pause_a_bit();
  pause_a_bit();
  pause_a_bit();
  pause_a_bit();
  d.driver.screenshot(std::path::Path::new("handle_request_done.png")).await.unwrap();

  let ig_campaign = app.campaign().select().site_eq(Site::Instagram).one().await.unwrap();
  let ig_handle = app.handle().select().site_eq(Site::Instagram).one().await.unwrap();
  ig_campaign.make_collab(&ig_handle).await.unwrap();

  let x_campaign = app.campaign().select().site_eq(Site::X).one().await.unwrap();
  let x_handle = app.handle().select().site_eq(Site::X).one().await.unwrap();
  x_campaign.make_collab(&x_handle).await.unwrap();
  a.run_idempotent_background_tasks_a_few_times().await;

  d.wait_for("#collab-list").await;
  dbg!("focus on collab list");
  pause_a_bit();
  pause_a_bit();
  pause_a_bit();
  pause_a_bit();
  d.driver.screenshot(std::path::Path::new("dashboard_with_collabs.png")).await.unwrap();

  d.click("#balance-card-claim-account-button").await;
  d.wait_for("#account-summary-claim-none").await;
  d.link_wallet_and_sign_login().await?;
  d.wait_for(".MuiSnackbarContent-message").await;
  d.wait_until_gone(".MuiSnackbarContent-message").await;
  d.wait_for("#account-summary-claim-pending").await;
  dbg!("focus on claiming");
  pause_a_bit();
  pause_a_bit();
  pause_a_bit();
  pause_a_bit();
  d.driver.screenshot(std::path::Path::new("dashboard_claiming.png")).await.unwrap();
  a.run_idempotent_background_tasks_a_few_times().await;
  d.wait_for("#account-summary-claim-done").await;
  dbg!("focus on claimed");
  pause_a_bit();
  pause_a_bit();
  pause_a_bit();
  pause_a_bit();
  d.driver.screenshot(std::path::Path::new("dashboard_claimed.png")).await.unwrap();
}

browser_test!{ shows_campaigns_in_dashboard (mut d)
  let a = d.test_app();
  let advertiser = a.client().await;
  let account = advertiser.account().await;
  let valid_until = Utc::now() + chrono::Duration::days(10);

  for post in &[
    "1716421161867710954",
    "1758208064467935359",
    "1758116416606163059",
    "1752961229407375400",
    "1758192957386342435",
    "1758192965703647443",
    "1758506690213732795",
  ] {
    account.create_campaign_request(models::Site::X, post, u("100"), u("1"), valid_until, &[])
      .await.unwrap().pay()
      .await.unwrap();
  }

  for post in &[
    "C2w6_ThRgkY",
    "C3TJIgLgULU",
    "C3PuyCPKNWT",
    "C3GOsATrTTb",
    "C3Ai4_KLqo7",
    "C3SaabGsI1j",
    "C222POpvKho",
  ] {
    account.create_campaign_request(models::Site::Instagram, post, u("100"), u("1"), valid_until, &[])
      .await.unwrap().pay()
      .await.unwrap();
  }

  try_until(100, 5000, "campaigns not created", || async {
    a.run_idempotent_background_tasks_a_few_times().await;
    a.app.campaign().select().count().await.unwrap() == 14
  }).await;

  a.app.ig_crawl().do_everything().await.unwrap();
  let crawl = a.app.ig_crawl().find(1).await?;

  try_until(100, 5000, "no ig crawl", || async {
    a.app.ig_crawl().do_everything().await.unwrap();
    *crawl.reloaded().await.unwrap().processed_for_campaign_rules()
  }).await;

  d.goto("http://127.0.0.1:5173").await;
  d.wait_for("#button-login-as-member").await;
}

browser_test!{ full_flow_to_reward_in_browser (mut d)
  d.signup_with_one_time_token().await;

  d.click("#open-start-campaign-request-dialog").await;
  d.fill_in("#contentUrl", "https://x.com/asami_club/status/1716421161867710954?s=20").await;
  d.click("#submit-start-campaign-form").await;
  d.wait_for_text(".MuiSnackbarContent-message", "Your suggestion has been received!").await;
  d.wait_until_gone(".MuiSnackbarContent-message").await;
  d.wait_for("#campaign-request-list").await;

  d.click("#open-start-campaign-request-dialog").await;
  d.fill_in("#contentUrl", "https://instagram.com/p/C2w6_ThRgkY").await;
  d.click("#submit-start-campaign-form").await;
  d.wait_for_text(".MuiSnackbarContent-message", "Your suggestion has been received!").await;
  d.wait_until_gone(".MuiSnackbarContent-message").await;

  let a = d.test_app();
  d.api.test_app.mock_admin_setting_campaign_requests_as_paid().await;

  try_until(10, 500, "no ig crawl", || async {
    d.api.test_app.run_idempotent_background_tasks_a_few_times().await;
    a.app.campaign().select().count().await.unwrap() == 2
  }).await;

  a.app.ig_crawl().do_everything().await.unwrap();
  let crawl = a.app.ig_crawl().find(1).await?;
  assert_eq!(*crawl.processed_for_campaign_rules(), false);
  try_until(100, 5000, "no ig crawl", || async {
    a.app.ig_crawl().do_everything().await.unwrap();
    *crawl.reloaded().await.unwrap().processed_for_campaign_rules()
  }).await;

  d.wait_for("#campaign-list").await;
  d.click("#button-logout").await;
  d.wait_for("#login-form-and-landing").await;
  
  d.api.test_app.app.one_time_token().insert(InsertOneTimeToken{value: "member-token".to_string() }).save().await?;
  d.goto("http://127.0.0.1:5173/#/one_time_token_login?token=member-token").await;
  d.wait_for("#advertiser-dashboard").await;

  d.goto("http://127.0.0.1:5173/#/?role=member").await;
  d.wait_for("#member-dashboard").await;

  d.fill_in("#x-handle-request-form #username", "nubis_bruno").await;
  d.click("#submit-x-handle-request-form").await;
  d.wait_for(".MuiSnackbarContent-message").await;
  d.wait_until_gone(".MuiSnackbarContent-message").await;

  d.fill_in("#ig-handle-request-form #username", "nubis_bruno").await;
  d.click("#submit-ig-handle-request-form").await;
  d.wait_for(".MuiSnackbarContent-message").await;
  d.wait_until_gone(".MuiSnackbarContent-message").await;

  d.api.test_app.mock_all_handles_being_verified_and_appraised().await;
  d.wait_for("#handle-x-submission-in-progress-message").await;
  d.wait_for("#handle-ig-submission-in-progress-message").await;
  d.api.test_app.run_idempotent_background_tasks_a_few_times().await;
  d.wait_for("#existing-x-handle-stats").await;
  d.wait_for("#existing-ig-handle-stats").await;

  d.api.test_app.mock_collab_on_all_campaigns_with_all_handles().await;
  d.api.test_app.run_idempotent_background_tasks_a_few_times().await;

  d.wait_for("#help-card-no-campaigns").await;
  d.click("#balance-card-claim-account-button").await;

  // Claiming account.
  d.wait_for("#account-summary-claim-none").await;
  d.link_wallet_and_sign_login().await?;
  d.wait_for(".MuiSnackbarContent-message").await;
  d.wait_until_gone(".MuiSnackbarContent-message").await;

  d.wait_for("#account-summary-claim-pending").await;

  d.api.test_app.run_idempotent_background_tasks_a_few_times().await;
  d.wait_for("#account-summary-claim-done").await;

  // Now logging back in.
  d.click("#button-logout").await;
  d.click("#button-login-as-member").await;
  d.click("#wallet-login-button").await;
  d.login_with_wallet().await?;
  d.wait_for("#member-dashboard").await;
}

api_test!{ full_flow_to_reward_from_api (mut c)
  c.create_x_handle("nubis_bruno", wei("10000000000000000")).await;
  let account = c.account().await;
  let handle = c.x_handle().await;

  //"https://x.com/asami_club/status/1716421161867710954?s=20",
  let campaign_req = account.create_campaign_request(
    Site::X,
    "1716421161867710954",
    u("50"),
    u("2"),
    Utc::now() + chrono::Duration::days(2),
    &[],
  ).await?
  .pay().await?;

  c.test_app.app.campaign_request().submit_approvals().await?;
  c.test_app.app.synced_event().sync_on_chain_events().await?;
  assert_eq!(campaign_req.reloaded().await?.attrs.status, CampaignRequestStatus::Approved);

  c.test_app.app.campaign_request().submit_all().await?;
  c.test_app.app.synced_event().sync_on_chain_events().await?;
  assert_eq!(campaign_req.reloaded().await?.attrs.status, CampaignRequestStatus::Done);

  let mut campaign = c.test_app.app.campaign().select().one().await.unwrap();

  assert_that!(&campaign.attrs, structure![ CampaignAttrs {
    budget: eq(uhex("50")),
    remaining: eq(uhex("50")),
  }]);

  c.test_app.app.collab_request().insert(InsertCollabRequest{
    campaign_id: campaign.attrs.id.clone(),
    handle_id: handle.attrs.id.clone(),
  }).save().await?;

  c.test_app.app.collab_request().submit_all().await.unwrap();
  c.test_app.app.synced_event().sync_on_chain_events().await?;
  assert!(c.test_app.app.collab().select().count().await? == 1);

  campaign.reload().await?;

  assert_that!(&campaign.attrs, structure![ CampaignAttrs {
    budget: eq(uhex("50")),
    remaining: eq(weihex("49900000000000000000"))
  }]);
}

browser_test!{ advertiser_claims_account (mut d)
  d.signup_with_one_time_token().await;
  d.click("#advertiser-claim-account-button").await;
  d.link_wallet_and_sign_login().await?;
  d.wait_for(".MuiSnackbarContent-message").await;
  d.wait_until_gone(".MuiSnackbarContent-message").await;
  d.wait_for("#advertiser-claim-account-pending").await;
  d.api.test_app.run_idempotent_background_tasks_a_few_times().await;
  d.wait_until_gone("#advertiser-claim-account-pending").await;
}

browser_test!{ account_is_web3_from_the_start (mut d)
  d.goto("http://127.0.0.1:5173").await;
  d.click("#button-login-as-member").await;
  d.click("#wallet-login-button").await;
  d.link_wallet_and_sign_login().await?;
  d.wait_for("#member-dashboard").await;
  d.click("#button-pay-to-amplify").await;
  d.wait_for("#advertiser-dashboard").await;

  d.wait_for("#advertiser-claim-account-pending").await;
  d.api.test_app.run_idempotent_background_tasks_a_few_times().await;
  
  d.goto("http://127.0.0.1:5173/#/?role=member").await;
  d.wait_for("#member-dashboard").await;
  d.goto("http://127.0.0.1:5173/#/?role=advertiser").await;

  d.wait_for("#open-start-campaign-dialog").await;
  d.click("#open-start-campaign-dialog").await;
  d.fill_in("#contentUrl", "https://x.com/asami_club/status/1716421161867710954?s=20").await;
  d.fill_in("#budget", "20").await;

  d.click("#submit-start-campaign-form").await;
  d.wait_for("#approval-waiter").await;

  try_until(10, 200, "No other window opened", || async {
    d.driver.windows().await.unwrap().len() == 2
  }).await;
  let mut handles = d.driver.windows().await.unwrap();
  d.driver.switch_to_window(handles[1].clone()).await.expect("to switch window zero");

  d.wait_for(".token-allowance-container").await;
  d.click("button[data-testid=page-container-footer-next]").await;
  d.wait_for(".review-spending-cap").await;
  d.click("button[data-testid=page-container-footer-next]").await;
  d.driver.switch_to_window(handles[0].clone()).await.unwrap();

  d.wait_until_gone("#approval-waiter").await;
  d.wait_for("#creation-waiter").await;

  try_until(10, 200, "No other window opened", || async {
    d.driver.windows().await.unwrap().len() == 2
  }).await;
  handles = d.driver.windows().await.unwrap();
  d.driver.switch_to_window(handles[1].clone()).await.expect("to switch window zero");
  d.click("button[data-testid=page-container-footer-next]").await;

  d.driver.switch_to_window(handles[0].clone()).await.unwrap();
  d.wait_for("#campaign-done").await;
  d.click("#campaign-done-close").await;
  d.api.test_app.run_idempotent_background_tasks_a_few_times().await;
  d.wait_for("#campaign-list").await;
}

browser_test!{ can_add_tokens_to_wallet (mut d)
  d.goto("http://127.0.0.1:5173").await;
  d.click("#button-login-as-member").await;
  d.click("#wallet-login-button").await;
  d.link_wallet_and_sign_login().await?;
  d.api.test_app.run_idempotent_background_tasks_a_few_times().await;
  d.click("#add-to-wallet-DOC").await;

  try_until(10, 200, "No other window opened", || async {
    d.driver.windows().await.unwrap().len() == 2
  }).await;

  let handles = d.driver.windows().await.unwrap();
  d.driver.switch_to_window(handles[1].clone()).await.expect("to switch window zero");
  d.click("button[data-testid=page-container-footer-next]").await;
  d.driver.switch_to_window(handles[0].clone()).await.unwrap();
  // Adding token always fails on metamask because of the unsupported chain id for tokenservice.
  d.wait_for_text(".MuiSnackbarContent-message", "DOC could not be added to your wallet.").await;
  d.wait_until_gone(".MuiSnackbarContent-message").await;
}
