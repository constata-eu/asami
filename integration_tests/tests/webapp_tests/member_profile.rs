use models::{
    weihex, CollabStatus, EngagementScore, Handle, HandleScoringStatus, HandleStatus, OnChainJobKind, OnChainJobStatus,
    OperationalStatus, PollScore,
};

use super::*;

#[tokio::test(flavor = "multi_thread")]
async fn shows_member_profile_page() {
    let h = TestHelper::for_web().await;
    let w = h.web();
    let mut api = h.make_api_client().await;

    let handle = stub_scored_handle(&h).await;
    let mut campaign = api.quick_campaign(u("100"), 30, &[]).await;
    campaign.make_x_collab_with_user_id(handle.user_id()).await.unwrap().unwrap();
    h.app
        .wait_for_job(
            "Account collab",
            OnChainJobKind::MakeSubAccountCollabs,
            OnChainJobStatus::Settled,
        )
        .await;
    assert_eq!(handle.collab_vec().await.unwrap().len(), 1);
    assert_eq!(
        handle.collab_scope().status_eq(CollabStatus::Cleared).all().await.unwrap().len(),
        1
    );

    w.goto(&format!("http://127.0.0.1:5173/#/Account/{}/show", handle.account_id())).await;
    wait_for_enter("Check the member show page").await;
}

async fn stub_scored_handle(h: &TestHelper) -> Handle {
    let account = h.app.create_account().await;

    let handle = h.app.create_handle(&account.attrs.id, "asami_tester", "111111111111", u("0")).await;

    let scoring = h
        .app
        .app
        .handle_scoring()
        .create(handle.clone())
        .await
        .unwrap()
        .update()
        .status(HandleScoringStatus::Applied)
        .post_count(100)
        .impression_count(50000)
        .repost_fatigue(false)
        .ghost_account(false)
        .indeterminate_audience(false)
        .followed(true)
        .liked(true)
        .replied(true)
        .reposted(true)
        .mentioned(true)
        .online_engagement_score(EngagementScore::High)
        .offline_engagement_score(EngagementScore::High)
        .offline_engagement_description(Some("Founder de un proyecto WEB3".to_string()))
        .poll_id(Some("101010101010".to_string()))
        .poll_score(Some(PollScore::High))
        .operational_status_score(OperationalStatus::Enhanced)
        .referrer_score(true)
        .holder_score(true)
        .authority(100)
        .audience_size(553)
        .score(Some(weihex("553")))
        .save()
        .await
        .unwrap();

    handle
        .update()
        .current_scoring_id(Some(*scoring.id()))
        .last_scoring(Some(*scoring.created_at()))
        .score(Some(weihex("553")))
        .status(HandleStatus::Active)
        .save()
        .await
        .unwrap()
}
