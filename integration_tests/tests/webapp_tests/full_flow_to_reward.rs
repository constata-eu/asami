use api::models::*;

use super::*;

/*
browser_test! { full_flow_to_reward_for_web2 (mut d)
    let mut advertiser = d.test_app().client().await;
    advertiser.setup_as_advertiser("test main advertiser").await;

    let mut x_campaign = advertiser.start_and_pay_campaign(
        "https://x.com/somebody/status/1758116416606163059",
        u("100"), 20, &[]
    ).await;

    d.goto("http://127.0.0.1:5173").await;

    d.login().await;
    d.click("#button-post-to-earn").await;

    d.wait_for("#member-dashboard").await;

    d.fill_in("#x-handle-request-form #username", "nubis_bruno").await;
    d.click("#submit-x-handle-request-form").await;
    d.wait_for(".MuiSnackbarContent-message").await;
    d.wait_until_gone(".MuiSnackbarContent-message").await;

    for (i, h) in d.app().handle().select().all().await?.into_iter().enumerate() {
        d.test_app().app.handle_topic().insert(InsertHandleTopic{
            handle_id: h.attrs.id,
            topic_id: 2
        }).save().await.unwrap();
        h.verify((1000 + i).to_string()).await?.update().score(Some(weihex("1234"))).status(HandleStatus::Active).save().await?;
    }

    d.wait_for("#existing-x-handle-stats").await;

    for h in d.app().handle().select().all().await? {
        x_campaign.make_collab(&h, u("2"), &h.attrs.username).await?;
    }

    d.wait_for_text("td.column-status", "Registered").await;

    d.test_app().wait_for_job("Subaccount collabs", OnChainJobKind::MakeSubAccountCollabs, OnChainJobStatus::Settled).await;
    d.wait_for_text(".ra-field-unclaimedAsamiBalance span", "240.0 ASAMI").await;

    d.wait_for("#help-card-no-campaigns").await;
    d.click("#balance-card-claim-account-button").await;
    d.link_wallet_and_sign_login().await;
    d.wait_for(".MuiSnackbarContent-message").await;
    d.wait_until_gone(".MuiSnackbarContent-message").await;
    d.wait_for("#account-summary-claim-pending").await;

    d.test_app().wait_for_job("Claim Accounts", OnChainJobKind::PromoteSubAccounts, OnChainJobStatus::Settled).await;
    d.wait_for("#gasless-claim-button").await;

    d.click("#gasless-claim-button").await;
    d.wait_until_gone(".MuiSnackbarContent-message").await;

    d.test_app().wait_for_job("Gasless Claim", OnChainJobKind::GaslessClaimBalances, OnChainJobStatus::Settled).await;

    d.wait_for_text(".ra-field-unclaimedAsamiBalance span", "^0.0 ASAMI").await;

    d.test_app().start_mining().await;

    for h in d.app().handle().select().all().await? {
        x_campaign.make_collab(&h, u("20"), &format!("{}-2", h.attrs.username)).await?;
    }
    d.test_app().wait_for_job("Account collabs", OnChainJobKind::MakeCollabs, OnChainJobStatus::Settled).await;

    d.wait_for_text(".ra-field-unclaimedAsamiBalance span", "2400.0 ASAMI").await;
    d.click("#claim-balances-button").await;

    try_until(10, 200, "No other window opened", || async {
        d.driver.windows().await.unwrap().len() == 2
    }).await;
    let handles = d.driver.windows().await.unwrap();
    d.driver.switch_to_window(handles[1].clone()).await.expect("to switch window zero");
    d.click("button[data-testid=page-container-footer-next]").await;


    d.driver.switch_to_window(handles[0].clone()).await.unwrap();
    d.wait_until_gone(".MuiSnackbarContent-message").await;

    d.wait_for_text(".ra-field-unclaimedAsamiBalance span", "^0.0 ASAMI").await;
    d.test_app().stop_mining().await;
}
*/
