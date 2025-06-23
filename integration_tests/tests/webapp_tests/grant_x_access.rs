use api::models::HandleStatus;
use models::HandleScoringStatus;

use super::*;

/*
-- Scoring should not be attempted for 'paused' handles.
-- A failure when scoring should not leave an empty scoring.
-- Failures to retrieve data with a valid api token should leave an empty scoring.

-- A notification or twitter message is sent when their x access is lost.

-- Usually, a handle will be paused *before* getting a 0 score applied, but this is still possible.
    -- We need a better distinction between scoring errors:
        - Other api connection errors will discard the scoring and re-schedule. Never sets a 0 score. 
        - Actual empty data from the API can discard the scoring and re-schedule or apply it as 0 if stale. 
            - Staleness is decided by date, not by how many attempts.
            - Stale is twice whatever the 're-check' threshold is.
*/

#[tokio::test(flavor = "multi_thread")]
#[serial_test::file_serial]
async fn grant_x_access() {
    let h = TestHelper::for_web().await;
    let w = h.web();
    let a = &h.test_app.app;

    let tester_user = std::env::var("X_TESTER_USERNAME").unwrap();
    let tester_pass = std::env::var("X_TESTER_PASSWORD").unwrap();

    w.signup_with_one_time_token().await;

    w.click("#button-grant-permission-and-make-post").await;
    w.fill_in_with_enter("input[autocomplete=username]", &tester_user, true).await;
    w.fill_in_with_enter("input[type=password]", &tester_pass, true).await;

    w.click("[data-testid=OAuth_Consent_Button]").await;

    w.wait_for("#handle-x-submission-in-progress-message").await;

    // With the refresh token, a poll will be created.
    let mut handle = a.handle().select().username_eq(&tester_user).one().await.unwrap();
    assert!(handle.poll_id().is_none());
    let old_refresh_token = handle.x_refresh_token().clone();

    a.handle().setup_pending().await.unwrap();
    handle.reload().await.unwrap();

    assert!(handle.poll_id().is_some());
    assert_ne!(handle.attrs.x_refresh_token, old_refresh_token);

    a.handle().score_pending().await.unwrap();

    handle.reload().await.unwrap();

    assert_eq!(handle.handle_scoring_scope().count().await.unwrap(), 1);

    let scoring = handle.handle_scoring().await.unwrap().expect("expected a scoring");

    assert_eq!(*scoring.status(), HandleScoringStatus::Applied);

    assert_eq!(
        *scoring.me_payload().unwrap().unwrap().data.unwrap().username,
        tester_user
    );

    assert!(scoring.tweets_payload().unwrap().unwrap().data.unwrap().first().unwrap().organic_metrics.is_some());

    assert!(scoring.mentions_payload().unwrap().unwrap().data().is_some());

    w.driver.refresh().await.unwrap();
    w.wait_for("#existing-x-handle-stats").await;

    handle = handle.update().x_refresh_token(Some("invalid_token_now".to_string())).save().await.unwrap();

    assert!(handle.clone().x_api_client().await.is_err());

    handle.reload().await.unwrap();

    assert!(handle.x_refresh_token().is_none());
    assert_eq!(*handle.status(), HandleStatus::Disconnected);

    // Go to the public profile and check the disconnected message. 

    w.driver.refresh().await.unwrap();
    w.wait_for("#grant-x-permission-again").await;
    w.click("#button-grant-x-permission-again").await;
    w.click("[data-testid=OAuth_Consent_Button]").await;

    w.wait_for("#existing-x-handle-stats").await;
    // Go to member dashboard and see the 'reconnecting' text which is like scoring but different.

    h.stop().await;
}

#[tokio::test(flavor = "multi_thread")]
#[serial_test::file_serial]
async fn can_grant_from_login() {
    TestHelper::with_web(|h| async move {
        let w = h.web();

        let tester_user = std::env::var("X_TESTER_USERNAME").unwrap();
        let tester_pass = std::env::var("X_TESTER_PASSWORD").unwrap();

        w.navigate("/").await;
        w.click("#menu-login").await;
        w.click("#x-login-button").await;

        TestApp::wait_for_enter("attempt login").await;
        w.fill_in_with_enter("input[autocomplete=username]", &tester_user, true).await;
        w.fill_in_with_enter("input[type=password]", &tester_pass, true).await;

        w.click("[data-testid=OAuth_Consent_Button]").await;

        w.wait_for("#handle-x-submission-in-progress-message").await;
    }).await;
}

#[tokio::test(flavor = "multi_thread")]
#[serial_test::file_serial]
async fn reproduce_revoked_access_bug() {
    TestHelper::with_web(|h| async move {
        let w = h.web();
        let a = &h.test_app.app;

        let tester_user = std::env::var("X_TESTER_USERNAME").unwrap();
        let tester_pass = std::env::var("X_TESTER_PASSWORD").unwrap();

        w.signup_with_one_time_token().await;

        w.click("#button-grant-permission-and-make-post").await;
        w.fill_in_with_enter("input[autocomplete=username]", &tester_user, true).await;
        w.fill_in_with_enter("input[type=password]", &tester_pass, true).await;
        TestApp::wait_for_enter("logging in").await;

        w.click("[data-testid=OAuth_Consent_Button]").await;

        w.wait_for("#handle-x-submission-in-progress-message").await;

        TestApp::wait_for_enter("logging in").await;

        // With the refresh token, a poll will be created.
        let mut handle = a.handle().select().username_eq(&tester_user).one().await.unwrap();
        assert!(handle.poll_id().is_none());

        a.handle().setup_pending().await.unwrap();
        handle.reload().await.unwrap();

        assert!(handle.poll_id().is_some());

        dbg!(handle.x_refresh_token());
        handle.reload().await.unwrap();
        dbg!(handle.x_refresh_token());
        handle.clone().x_api_client().await.expect("creating the api should work");
        handle.reload().await.unwrap();
        dbg!(handle.x_refresh_token());
        TestApp::wait_for_enter("Logout here, and log back in, attempt breaking it").await;

        handle.reload().await.unwrap();
        dbg!(handle.x_refresh_token());
        dbg!(handle.clone().x_api_client().await).unwrap();
    }).await;
}

#[tokio::test(flavor = "multi_thread")]
#[serial_test::file_serial]
async fn shows_disconnected_and_reconnecting_states() {
    TestHelper::with_web(|h| async move {
        let alice = h.collaborator(3_000).await;
        alice.login_to_web_with_otp().await;

        h.web().navigate("/Account/1/show").await;
        h.web().wait_for_text("#score-value", "2400").await;
        h.web().wait_until_gone("#account-disconnected").await;

        let mut handle = alice.x_handle().await;

        assert!(handle.clone().x_api_client().await.is_err());

        handle.reload().await.unwrap();
        assert_eq!(*handle.status(), HandleStatus::Disconnected);

        h.web().driver.refresh().await.unwrap();
        h.web().wait_for("#account-disconnected").await;
        h.web().wait_for_text("#score-value", "2400").await;

        h.web().click("#menu-my-collabs").await;
        h.web().wait_for("#grant-permission-again-dialog").await;
        h.web().click("#button-cancel-grant-permissions-again").await;
        h.web().wait_until_gone("#grant-permission-again-dialog").await;

        h.web().click("#menu-my-campaigns").await;
        h.web().wait_for("#grant-permission-again-dialog").await;
        h.web().click("#button-cancel-grant-permissions-again").await;
        h.web().wait_until_gone("#grant-permission-again-dialog").await;

        let account_id = handle.account_id().to_string();
        handle = handle.handle_update_refresh_token("somenewtoken".to_string(), account_id).await.unwrap();

        assert_eq!(*handle.status(), HandleStatus::Reconnecting);
        h.web().navigate("/Account/1/show").await;
        h.web().wait_for("#account-disconnected").await;
        h.web().wait_for_text("#score-value", "2400").await;

        h.web().click("#menu-my-collabs").await;
        h.web().wait_for("#handle-x-reconnecting-message").await;
    }).await;
}
