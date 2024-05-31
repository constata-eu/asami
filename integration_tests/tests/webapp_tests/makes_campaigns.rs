use ::api::models::*;

browser_test! { makes_ig_and_x_campaigns (mut d)
    d.test_app().start_mining().await;

    d.goto("http://127.0.0.1:5173").await;
    d.click("#button-login-as-member").await;
    d.click("#wallet-login-button").await;
    d.link_wallet_and_sign_login().await?;
    d.wait_for("#member-dashboard").await;
    d.click("#button-pay-to-amplify").await;
    d.wait_for("#advertiser-dashboard").await;
    d.wait_for("#advertiser-claim-account-pending").await;

    d.test_app().wait_for_job(
        "Claiming account",
        models::OnChainJobKind::PromoteSubAccounts,
        models::OnChainJobStatus::Settled
    ).await;

    d.wait_for("#advertiser-claim-account-done").await;

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

    d.fill_in("#custom-spending-cap", "500").await;
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

    d.wait_for("#campaign-list").await;
    d.wait_for_text("#campaign-list td.column-status", "Publishing soon").await;

    d.test_app().sync_events_until("Campaign is published", || async {
        d.app().campaign().select().status_eq(CampaignStatus::Published)
            .count().await.unwrap() > 0
    }).await;

    d.wait_for_text("#campaign-list td.column-status", "Has 20.0 DOC").await;

    d.wait_for("#open-start-campaign-dialog").await;
    d.click("#open-start-campaign-dialog").await;
    d.fill_in("#contentUrl", "https://instagram.com/p/C2w6_ThRgkY").await;
    d.fill_in("#budget", "40").await;

    d.click("#submit-start-campaign-form").await;
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

    d.wait_for("#campaign-list").await;
    d.wait_for_text("#campaign-list td.column-status", "Publishing soon").await;

    d.test_app().sync_events_until("Campaign is published", || async {
        d.app().campaign().select().status_eq(CampaignStatus::Published)
            .count().await.unwrap() == 2
    }).await;

    d.wait_for_text("#campaign-list td.column-status", "Has 40.0 DOC").await;
    d.test_app().stop_mining().await;
}

browser_test! { advertiser_always_needs_a_wallet (mut d) 
    d.signup_with_one_time_token().await;
    d.click("#balance-card-claim-account-button").await;
    d.link_wallet_and_sign_login().await?;
    d.wait_for(".MuiSnackbarContent-message").await;
    d.wait_for("#advertiser-claim-account-pending").await;
    d.test_app().wait_for_job(
        "Claiming account",
        models::OnChainJobKind::PromoteSubAccounts,
        models::OnChainJobStatus::Settled
    ).await;
    d.wait_for("#open-start-campaign-dialog").await;
}
