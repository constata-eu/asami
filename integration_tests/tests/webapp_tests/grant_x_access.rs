use api::models::HandleStatus;
use models::HandleScoringStatus;

use super::*;


/*
-- Both reconnecting and verified need to be scored.

-- In handle settings "HandleSubmissionInProgress" should differentiate "Connecting" or "Reconnecting".


-- Scoring should not be attempted for 'paused' handles.
-- A failure when scoring should not leave an empty scoring.
-- Failures to retrieve data with a valid api token should leave an empty scoring.
-- No collabs should be registered for 'paused' handles.
-- A handle is paused as soon as their X access is lost.
    -- A notification is sent when their x access is lost.
    -- Profile page shows the current score, as usual, but also a banner:
        "This account is paused because we lost access to their X account.
         They'll be able to continue collaborting when they log-in and re-connect to X."

    -- Advertiser dashboard should check for handles, and show the promt if their handle was paused.
       -- Otherwise they will need to go to 'reposts' and we don't want that.

-- A new state re-connecting means a handle got to be active but then their connection was lost.
   -- IN this case they will be able to show their last applied scoring.
   -- The public profile should show the existing score instead of a message of "we're scoring ..."

-- Usually, a handle will be paused *before* getting a 0 score applied, but this is still possible.
    -- We need a better distinction between scoring errors:
        - Invalid token will pause the handle and discard the scoring. Never sets a 0 score.
        - Other api connection errors will discard the scoring and re-schedule. Never sets a 0 score. 
        - Actual empty data from the API can discard the scoring and re-schedule or apply it as 0 if stale. 
            - Staleness is decided by date, not by how many attempts.
            - Stale is twice whatever the 're-check' threshold is.


-- Never allow a new handle to be created if the account already had an account.
    -- Create and update from refresh token are totally different flows.
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
async fn shows_disconnected_and_reconnecting_states() {
    TestHelper::with_web(|h| async move {
        let alice = h.collaborator(3_000).await;
        alice.login_to_web_with_otp().await;

        h.web().navigate("/Account/1/show").await;
        h.web().wait_for_text("#score-value", "2400").await;

        let mut handle = alice.x_handle().await;

        assert!(handle.clone().x_api_client().await.is_err());

        handle.reload().await.unwrap();
        assert_eq!(*handle.status(), HandleStatus::Disconnected);

        // The public profile page should say it's disconnected.

        // Their private advertiser and member dashboards should show the notice to reconnect.

        // Simulate the reconnection, the handle is now 'reconnecting'

        // Check the member dashboard says it's reconnecting.

        // Check the public profile, the last valid scoring is shown.

        // Simulate a new handle scoring. Make sure reconnecting accounts are also in the pending scoring.

        TestApp::wait_for_enter("stop here, do the steps").await;
    }).await;
}
