use ::api::models::*;

browser_test! { adds_tokens_to_wallet (mut d)
    d.goto("http://127.0.0.1:5173").await;
    d.click("#button-login-as-member").await;
    d.click("#wallet-login-button").await;
    d.link_wallet_and_sign_login().await?;

    d.test_app().wait_for_job("Claim Accounts", OnChainJobKind::PromoteSubAccounts, OnChainJobStatus::Settled).await;

    d.click("#add-to-wallet-DOC").await;

    try_until(10, 200, "No other window opened", || async {
        d.driver.windows().await.unwrap().len() == 2
    }).await;

    let handles = d.driver.windows().await.unwrap();
    d.driver.switch_to_window(handles[1].clone()).await.expect("to switch window zero");
    d.click("button[data-testid=page-container-footer-next]").await;
    d.driver.switch_to_window(handles[0].clone()).await.unwrap();

    // We can only test that the button attempts to add the token.
    // Adding token always fails on metamask because of the unsupported
    // chain id for tokenservice.
    d.wait_for_text(
        ".MuiSnackbarContent-message",
        "DOC could not be added to your wallet."
    ).await;
    d.wait_until_gone(".MuiSnackbarContent-message").await;
}
