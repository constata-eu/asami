use api::models::CampaignStatus;

use super::*;

#[tokio::test(flavor = "multi_thread")]
#[serial_test::file_serial]
async fn sends_announcements() {
    TestHelper::with_web(|h| async move {
        let w = h.web();
        let a = &h.test_app.app;
        let advertiser = h.advertiser().await;
        let mut good = advertiser.make_campaign_one(u("15"), 11, &[]).await;
        let mut too_small = advertiser.start_and_pay_campaign(
            "https://x.com/somebody/status/1933626442178326847",
            u("5"),
            11,
            &[]
        )
        .await;

        let tester_user = std::env::var("X_TESTER_USERNAME").unwrap();
        let tester_pass = std::env::var("X_TESTER_PASSWORD").unwrap();

        w.signup_with_one_time_token().await;

        w.click("#button-grant-permission-and-make-post").await;
        w.fill_in_with_enter("input[autocomplete=username]", &tester_user, true).await;
        w.fill_in_with_enter("input[type=password]", &tester_pass, true).await;

        w.click("[data-testid=OAuth_Consent_Button]").await;

        w.wait_for("#handle-x-submission-in-progress-message").await;

        let handle = a.handle().select().username_eq(&tester_user).one().await.unwrap();

        assert_eq!(*good.status(), CampaignStatus::Published);
        assert_eq!(*too_small.status(), CampaignStatus::Published);

        a.campaign_announcement().send_pending_announcements().await.unwrap();
        good.reload().await.unwrap();
        too_small.reload().await.unwrap();

        assert!(good.x_announcement_id_es().is_none());
        assert!(good.x_announcement_id_en().is_none());
        assert!(too_small.x_announcement_id_es().is_none());
        assert!(too_small.x_announcement_id_en().is_none());

    
        handle.update().username("asami_club_es".to_string()).save().await.unwrap();
        a.campaign_announcement().send_pending_announcements().await.unwrap();
        good.reload().await.unwrap();
        too_small.reload().await.unwrap();

        let old_id = good.x_announcement_id_es().clone();
        assert!(good.x_announcement_id_es().is_some());
        assert!(good.x_announcement_id_en().is_none());
        assert!(too_small.x_announcement_id_es().is_none());
        assert!(too_small.x_announcement_id_en().is_none());
    
        a.campaign_announcement().send_pending_announcements().await.unwrap();
        good.reload().await.unwrap();
        too_small.reload().await.unwrap();

        assert_eq!(old_id, *good.x_announcement_id_es());
    }).await;
}
