use super::*;

#[tokio::test(flavor = "multi_thread")]
#[serial_test::file_serial]
async fn supports_all_login_options() {
    TestHelper::with_web(|h| async move {
        h.web().navigate("/").await;

        TestApp::wait_for_enter("try_logging_in_with_trezor").await;
    })
    .await;
}

/*
use super::*;
TODO: Test all login methods here. You have X. You can mimic email.
TODO: Can you add something for wallets other than metamask?
api_test! { api_creates_and_sends_email_login (mut c)
    let client = c.test_app.client().await;
    let _result: gql::create_email_login_link::ResponseData = client.gql(
        &gql::CreateEmailLoginLink::build_query(
            gql::create_email_login_link::Variables{ email: "yo@nubis.im".to_string()}
        ),
        vec![],
    )
    .await;

    c.app().one_time_token().send_email_tokens().await?;
}
*/

/* TODO
browser_test! { creates_login_link_from_frontend (mut d)
    d.goto("http://127.0.0.1:5173").await;
    wait_here();
    // I would like to test that a user using a mobile browser
    // that tries to log-in with X, sees a 'warning' about the action
    // possibly failing. And a button to continue logging in with X.
    todo!("I should write this test");
}

browser_test! { shows_warning_when_trying_to_log_in_with_x_mobile (mut d)
    d.goto("http://127.0.0.1:5173").await;
    wait_here();
    // I would like to test that a user using a mobile browser
    // that tries to log-in with X, sees a 'warning' about the action
    // possibly failing. And a button to continue logging in with X.
    todo!("I should write this test");
}
*/

/*

browser_test! { account_is_web3_from_the_start (mut d)
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

*/
