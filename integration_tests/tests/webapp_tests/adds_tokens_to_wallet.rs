/*
use api::models::*;

use super::*;

#[tokio::test]
async fn adds_tokens_to_wallet() {
    let h = TestHelper::for_web().await;
    let d = h.web();

    d.goto("http://127.0.0.1:5173").await;
    d.click("#button-login-as-member").await;
    d.click("#wallet-login-button").await;
    d.link_wallet_and_sign_login().await;

    d.test_app()
        .wait_for_job(
            "Claim Accounts",
            OnChainJobKind::PromoteSubAccounts,
            OnChainJobStatus::Settled,
        )
        .await;

    d.click("#add-to-wallet-DOC").await;

    d.go_to_extension_notification().await;
    d.click("button[data-testid=page-container-footer-next]").await;

    d.go_to_app_window().await;
    d.wait_for_text(".MuiSnackbarContent-message", "DOC added to your wallet").await;
    d.wait_until_gone(".MuiSnackbarContent-message").await;
}
*/
