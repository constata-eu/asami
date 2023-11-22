#[macro_use]
mod support;
use support::*;
use ::api::models::*;

// Addres for member account.
// 0xF92DddAE4853a8cEA0b99a55d265E3b1Ee352429

browser_test!{ browser_test (test_app, d)
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
  d.wait_for("#button-login-as-advertiser").await;
  
  test_app.app.one_time_token().insert(InsertOneTimeToken{value: "member-token".to_string() }).save().await?;
  d.goto("http://127.0.0.1:5173/#/one_time_token_login?token=member-token").await;
  d.wait_for("#advertiser-dashboard").await;
  d.goto("http://127.0.0.1:5173/#/?role=member").await;
  d.wait_for("#member-dashboard").await;


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

  d.wait_for("#campaign-list-empty").await;
  d.click("#collabs-claim-account-button").await;

  // Claiming account.
  d.wait_for("#account-summary-claim-none").await;
  link_wallet_and_sign_login(&d).await?;
  d.wait_for(".MuiSnackbarContent-message").await;
  d.wait_until_gone(".MuiSnackbarContent-message").await;

  d.wait_for("#account-summary-claim-pending").await;

  test_app.run_idempotent_background_tasks_a_few_times().await;
  d.wait_for("#account-summary-claim-done").await;

  wait_here();
  // Now logging back in.
  d.click("#logout-menu-item").await;
  d.click("#button-login-as-member").await;
  d.click("#wallet-login-button").await;
  d.click("button[data-testid=page-container-footer-next]").await;
  d.wait_for("#member-dashboard").await;
}

browser_test!{ advertiser_claims_account (test_app, d)
  test_app.app.one_time_token().insert(InsertOneTimeToken{value: "member-token".to_string() }).save().await?;
  d.goto("http://127.0.0.1:5173/#/one_time_token_login?token=member-token").await;
  d.wait_for("#advertiser-dashboard").await;
  
  d.click("#advertiser-claim-account-button").await;
  link_wallet_and_sign_login(&d).await?;
  d.wait_for(".MuiSnackbarContent-message").await;
  d.wait_until_gone(".MuiSnackbarContent-message").await;
  d.wait_for("#advertiser-claim-account-pending").await;
  test_app.run_idempotent_background_tasks_a_few_times().await;
  d.wait_until_gone("#advertiser-claim-account-pending").await;
}

browser_test!{ account_is_web3_from_the_start (test_app, d)
  d.goto("http://127.0.0.1:5173").await;
  d.click("#button-login-as-advertiser").await;
  d.click("#wallet-login-button").await;
  link_wallet_and_sign_login(&d).await?;
  d.wait_for("#advertiser-dashboard").await;

  d.wait_for("#advertiser-claim-account-pending").await;
  test_app.run_idempotent_background_tasks_a_few_times().await;
  d.wait_for("#open-start-campaign-dialog").await;

  d.click("#open-start-campaign-dialog").await;
  d.fill_in("#contentUrl", "https://x.com/asami_club/status/1716421161867710954?s=20").await;
  d.fill_in("#budget", "20").await;

  d.click("#submit-start-campaign-form").await;

  try_until(10, "No other window opened", || async {
    d.driver.windows().await.unwrap().len() == 2
  }).await;
  let mut handles = d.driver.windows().await.unwrap();
  d.driver.switch_to_window(handles[1].clone()).await.expect("to switch window zero");

  d.wait_for(".token-allowance-container").await;
  d.click("button[data-testid=page-container-footer-next]").await;
  d.wait_for(".review-spending-cap").await;
  d.click("button[data-testid=page-container-footer-next]").await;

  d.driver.switch_to_window(handles[0].clone()).await.unwrap();
  d.wait_for_text(".MuiSnackbarContent-message", "Campaign budget approved.").await;
  d.wait_until_gone(".MuiSnackbarContent-message").await;

  try_until(10, "No other window opened", || async {
    d.driver.windows().await.unwrap().len() == 2
  }).await;
  handles = d.driver.windows().await.unwrap();
  d.driver.switch_to_window(handles[1].clone()).await.expect("to switch window zero");
  d.click("button[data-testid=page-container-footer-next]").await;

  d.driver.switch_to_window(handles[0].clone()).await.unwrap();
  d.wait_for_text(".MuiSnackbarContent-message", "Campaign will be started soon").await;
  d.wait_until_gone(".MuiSnackbarContent-message").await;
  test_app.run_idempotent_background_tasks_a_few_times().await;
  d.wait_for("#campaign-list").await;
}

async fn link_wallet_and_sign_login(d: &Selenium) -> anyhow::Result<()> {
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
  handles = d.driver.windows().await.unwrap();
  d.driver.switch_to_window(handles[1].clone()).await?;

  d.click("button[data-testid=page-container-footer-next]").await;

  d.driver.switch_to_window(handles[0].clone()).await?;
  Ok(())
}
