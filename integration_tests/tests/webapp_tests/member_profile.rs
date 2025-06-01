use models::EngagementScore;

use super::*;

#[tokio::test(flavor = "multi_thread")]
#[serial_test::file_serial]
async fn shows_member_profile_page() {
    TestHelper::with_web(|h| async move {
        let w = h.web();
        let advertiser = h.advertiser().await;

        let campaign = advertiser.make_campaign_one(u("1000"), 30, &[]).await;

        let mut user = h.user().await.signed_up().await.rand_unverified().await;
        user.handle
            .as_ref()
            .unwrap()
            .add_topic(&h.test_app.app.topic().find(1).await.unwrap())
            .await
            .unwrap();
        user.handle = Some(
            user.handle
                .unwrap()
                .update()
                .offline_engagement_score(EngagementScore::High)
                .offline_engagement_description(Some("Founder de un proyecto WEB3".to_string()))
                .holder_score_override(Some(true))
                .referrer_score_override(Some(true))
                .save()
                .await
                .unwrap(),
        );
        user = user.active(553).await;

        h.a().batch_collabs(campaign, &[&user]).await;

        let admin = h.signed_up().await;
        admin.user().await.update().admin(true).save().await.unwrap();

        admin.login_to_web_with_otp().await;

        w.navigate("/Handle/1").await;
        w.wait_for_text(".MuiChip-label", "English speaking").await;
        w.wait_for("input[name='offlineEngagementDescription']").await;
        w.fill_in("input[name='offlineEngagementDescription']", " en argentina").await;
        w.click("button[type='submit']").await;
        w.wait_for_text(".MuiSnackbarContent-message", "Element updated").await;

        let reloaded = user.handle.as_ref().unwrap().reloaded().await.unwrap();
        assert!(reloaded.last_scoring().is_none());
        assert_eq!(
            reloaded.attrs.offline_engagement_description.unwrap(),
            "Founder de un proyecto WEB3 en argentina"
        );
        TestApp::wait_for_enter("wait here").await;
    })
    .await;
}
