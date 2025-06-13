use models::HandleScoringStatus;

use super::*;

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

    a.handle().verify_pending().await.unwrap();
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

    w.driver.refresh().await.unwrap();
    w.wait_for("#grant-x-permission-again").await;
    w.click("#button-grant-x-permission-again").await;
    w.click("[data-testid=OAuth_Consent_Button]").await;
    w.wait_for("#existing-x-handle-stats").await;

    h.stop().await;
}
