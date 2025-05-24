use super::*;

#[tokio::test(flavor = "multi_thread")]
#[serial_test::file_serial]
async fn creates_campaign_using_stripe() {
    TestHelper::with_web(|h| async move {
        assert_eq!(h.a().app.account().select().count().await.unwrap(), 0);
        let account = h.a().app.get_admin_own_account().await.unwrap();
        assert_eq!(*account.addr().as_ref().unwrap(), h.a().app.on_chain.client.address().encode_hex());
        assert_eq!(*account.addr().as_ref().unwrap(), "0x000000000000000000000000ed4e67213c7a375af60893fe8e0852d0f7040913");

        let reloaded = h.a().app.get_admin_own_account().await.unwrap();
        assert_eq!(account, reloaded);
    }).await;
}


// -> I need a model that can track a user's CampaignRequest.
// -> Has all fields from CreateCampaignFromLinkInput. 
// -> Has a 'paid' flag.
// -> Has a column for stripe's payment code, clearing code, or other details.
// -> Once marked paid, a campaign is created (the campaign id is stored.)
// -> Campaign creation is funded by the admin address itself, so it needs doc.
// -> All draft campaigns require DOC to be available in the admin address?
// -> These campaigns may take up to 24 hours to become active.
// -> Any campaign that gets marked paid shoots out an email and report.
//      - this report includes the DOC balance in the admin address and the total balance of all DRAFT and PAID campaigns.

// I need a function that sets-up the admin as a user on its own.

/*
CampaignRequestStatus 

requested: Request just received.
paid: Payment on stripe done.
draft: The inner campaign exists, and is in draft status.
    -> Admin address pays the doc and marks submitted.
submitted: The inner campaign exists, and is in submitted status.
published: The inner campaign exists, and is in published status.
created: A draft campaign has been created. 
failed


*/

/*
use models::CampaignStatus;

use super::*;
browser_test! { makes_campaign (mut d)
    d.test_app().start_mining().await;

    d.goto("http://127.0.0.1:5173").await;
    d.click("#button-login-as-member").await;
    d.click("#wallet-login-button").await;
    d.link_wallet_and_sign_login().await;
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
    wait_here();
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

    d.test_app().sync_events_until("Campaign is published", || async {
        d.app().campaign().select().status_eq(CampaignStatus::Published)
            .count().await.unwrap() == 1
    }).await;

    d.test_app().stop_mining().await;
}

browser_test! { advertiser_always_needs_a_wallet (mut d)
    d.signup_with_one_time_token().await;
    d.click("#balance-card-claim-account-button").await;
    d.link_wallet_and_sign_login().await;
    d.wait_for(".MuiSnackbarContent-message").await;
    d.wait_for("#advertiser-claim-account-pending").await;
    d.test_app().wait_for_job(
        "Claiming account",
        models::OnChainJobKind::PromoteSubAccounts,
        models::OnChainJobStatus::Settled
    ).await;
    d.wait_for("#open-start-campaign-dialog").await;
}
*/
