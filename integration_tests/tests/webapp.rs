#![allow(dead_code)]

#[macro_use]
mod support;

use ::api::models::*;

browser_test!{ browser_test (test_app, d)
  //d.open_metamask().await;
  //let handles = d.driver.windows().await.expect("to get the window handles");
  //d.driver.switch_to_window(handles[1].clone()).await.expect("to switch window zero");
  //popover-close
  //.page-container__header-close

  // Addres for member account.
  // 0xF92DddAE4853a8cEA0b99a55d265E3b1Ee352429

  test_app.app.one_time_token().insert(InsertOneTimeToken{value: "advertiser-token".to_string() }).save().await?;
  d.goto("http://127.0.0.1:5173/#/one_time_token_login?token=advertiser-token").await;

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

  d.wait_for("#campaign-list").await;
  d.click("#logout-menu-item").await;
  wait_here();
  d.wait_for("#button-login-as-advertiser").await;
  
  test_app.app.one_time_token().insert(InsertOneTimeToken{value: "member-token".to_string() }).save().await?;
  d.goto("http://127.0.0.1:5173/#/one_time_token_login?token=member-token").await;
  d.wait_for("#advertiser-dashboard").await;
  d.goto("http://127.0.0.1:5173/#/?role=member").await;

  d.fill_in("#username", "nubis_bruno").await;
  d.click("#submit-handle-request-form").await;
  d.wait_for(".MuiSnackbarContent-message").await;
  d.wait_until_gone(".MuiSnackbarContent-message").await;

  test_app.mock_all_handles_being_verified_and_appraised().await;
  d.wait_for("#handle-submission-in-progress-message").await;
  test_app.run_idempotent_background_tasks_a_few_times().await;
  d.wait_for("#existing-x-handle-stats").await;

  test_app.mock_collab_on_all_campaigns_with_all_handles().await;
  test_app.run_idempotent_background_tasks_a_few_times().await;

  d.wait_for("#campaign-list").await;
  d.click("#collabs-claim-account-button").await;

  // Claiming account.
  d.wait_for("#account-summary-claim-none").await;
  d.click(".rlogin-provider-icon img[alt=MetaMask]").await;

  try_until(10, "No other window opened", || async {
    d.driver.windows().await.unwrap().len() == 2
  }).await;

  let mut handles = d.driver.windows().await.unwrap();
  d.driver.switch_to_window(handles[1].clone()).await.expect("to switch window zero");

  d.fill_in("input[data-testid=unlock-password]", "password").await;
  d.click("button[data-testid=unlock-submit]").await;
  d.click("button[data-testid=page-container-footer-next]").await;
  d.wait_for(".permission-approval-container__content-container").await;

  d.click("button[data-testid=page-container-footer-next]").await;

  d.driver.switch_to_window(handles[0].clone()).await.unwrap();
  d.click("button.rlogin-button.confirm").await;

  try_until(10, "No other window opened", || async {
    d.driver.windows().await.unwrap().len() == 2
  }).await;
  let mut handles = d.driver.windows().await.unwrap();
  d.driver.switch_to_window(handles[1].clone()).await?;

  d.click("button[data-testid=page-container-footer-next]").await;

  d.driver.switch_to_window(handles[0].clone()).await?;
  d.wait_for(".MuiSnackbarContent-message").await;
  d.wait_until_gone(".MuiSnackbarContent-message").await;
  d.wait_for("#account-summary-claim-pending").await;

  wait_here();
  test_app.run_idempotent_background_tasks_a_few_times().await;
  d.wait_for("#account-summary-claim-done").await;


  // The user now has a pending balance.
  // We also show a list of collaborations.
  // The campaign does not show any longer as collabs have been returned.
  
  //#### Claim account.
  // The user claims their account.
  //    - This creates a "ClaimAccount" request.
  //    - We run its course in the background.
  //
  // For claimed accounts, we check their tokens and balance.
  //
}

// Test creation of an account + session with twitter.
// Test creation of an account + session with instagram.

// Test creation of an account + session with metamask.
  // Is this account automatically used for the protocol too?
  // Does it mean that account starts immediately claimed?
