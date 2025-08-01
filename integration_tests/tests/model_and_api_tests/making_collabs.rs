use api::models::InsertTopic;
use models::{CollabStatus, OnChainJobKind, OnChainJobStatus};

// This module tests how collabs are made for accounts and sub accounts.
use super::*;

#[tokio::test]
#[serial_test::file_serial]
async fn sends_collabs_for_accounts_and_sub_accounts() {
    TestHelper::run(|h| async move {
        let advertiser = Box::pin(h.advertiser_with_params(u("1000000"), u("1000000"))).await;
        let mut campaign = Box::pin(advertiser.start_and_pay_campaign_with_params(
            "https://x.com/somebody/status/1716421161867710954",
            u("100"),
            30,
            &[],
            milli("9999"),
            milli("45"),
            milli("10"),
            false,
        ))
        .await;

        let mut alice = h.collaborator(6250).await;
        alice.claim_account().await;
        let claimed = alice.x_handle().await;
        let mut first_collab = campaign.make_x_collab_with_user_id(alice.x_user_id()).await.unwrap().unwrap();

        h.a().wait_for_job("Account collab", OnChainJobKind::MakeCollabs, OnChainJobStatus::Settled).await;
        assert_eq!(claimed.collab_vec().await.unwrap().len(), 1);
        assert_eq!(
            claimed.collab_scope().status_eq(CollabStatus::Cleared).all().await.unwrap().len(),
            1
        );

        let unclaimed_client = h.collaborator(6250).await;
        let unclaimed = unclaimed_client.x_handle().await;
        let mut second_collab =
            campaign.make_x_collab_with_user_id(unclaimed_client.x_user_id()).await.unwrap().unwrap();

        h.a()
            .wait_for_job(
                "Subaccount collabs",
                OnChainJobKind::MakeSubAccountCollabs,
                OnChainJobStatus::Settled,
            )
            .await;
        assert_eq!(unclaimed.collab_vec().await.unwrap().len(), 1);
        assert_eq!(
            unclaimed.collab_scope().status_eq(CollabStatus::Cleared).all().await.unwrap().len(),
            1
        );

        assert_eq!(
            h.a().app.collab().select().status_ne(CollabStatus::Cleared).count().await.unwrap(),
            0
        );
        assert_eq!(
            h.a().app.collab().select().status_eq(CollabStatus::Cleared).count().await.unwrap(),
            2
        );

        h.a()
            .wait_for_job(
                "No more accounts",
                OnChainJobKind::MakeCollabs,
                OnChainJobStatus::Skipped,
            )
            .await;
        h.a()
            .wait_for_job(
                "No more subaccount",
                OnChainJobKind::MakeSubAccountCollabs,
                OnChainJobStatus::Skipped,
            )
            .await;

        campaign.reload().await.unwrap();
        assert_eq!(campaign.available_budget().await.unwrap(), milli("80002"));
        assert_eq!(campaign.budget_u256(), milli("80002"));

        first_collab.reload().await.unwrap();
        assert_eq!(first_collab.reward_u256(), milli("9999"));
        assert_eq!(first_collab.fee_u256().unwrap(), wei("999900000000000000"));

        second_collab.reload().await.unwrap();
        assert_eq!(second_collab.reward_u256(), milli("9999"));
        assert_eq!(second_collab.fee_u256().unwrap(), wei("999900000000000000"));
    })
    .await;
}

#[tokio::test]
#[serial_test::file_serial]
async fn skips_if_we_are_no_longer_campaign_admins() {
    TestHelper::run(|h| async move {
        let advertiser = Box::pin(h.advertiser_with_params(u("1000000"), u("1000000"))).await;
        let mut campaign = advertiser.make_campaign_one(u("100"), 30, &[]).await;

        let mut alice = h.collaborator(6250).await;
        alice.claim_account().await;
        campaign.make_x_collab_with_user_id(alice.x_user_id()).await.unwrap();

        h.a()
            .send_tx(
                "Advertiser wants to be its own admin",
                "71200",
                advertiser.asami_contract().config_account(advertiser.address(), u("6"), u("0"), u("0")),
            )
            .await;

        h.a()
            .wait_for_job(
                "Account collab is skipped",
                OnChainJobKind::MakeCollabs,
                OnChainJobStatus::Skipped,
            )
            .await;

        let unclaimed_client = h.collaborator(6250).await;
        campaign.make_x_collab_with_user_id(unclaimed_client.x_user_id()).await.unwrap();
        h.a()
            .wait_for_job(
                "Sub Account collab is skipped",
                OnChainJobKind::MakeSubAccountCollabs,
                OnChainJobStatus::Skipped,
            )
            .await;
    })
    .await
}

#[tokio::test]
#[serial_test::file_serial]
async fn prevents_race_conditions_using_available_budget() {
    TestHelper::run(|h| async move {
        let advertiser = Box::pin(h.advertiser_with_params(u("1000000"), u("1000000"))).await;
        let mut campaign = Box::pin(advertiser.start_and_pay_campaign_with_params(
            "https://x.com/somebody/status/1716421161867710954",
            u("100"),
            30,
            &[],
            u("100"),
            u("10"),
            milli("10"),
            false,
        ))
        .await;

        let alice = h.collaborator(1250).await;
        campaign.make_x_collab_with_user_id(alice.x_user_id()).await.unwrap();

        campaign.reload().await.unwrap();
        assert_eq!(campaign.available_budget().await.unwrap(), u("90"));

        let bob = h.collaborator(11300).await;
        let failed = campaign.make_x_collab_with_user_id(bob.x_user_id()).await.unwrap().unwrap();
        assert_eq!(*failed.status(), CollabStatus::Failed);
        assert_eq!(
            failed.dispute_reason().as_ref().unwrap(),
            "campaign_has_not_enough_funds"
        );
    })
    .await;
}

#[tokio::test]
#[serial_test::file_serial]
async fn fails_when_handle_is_missing_topics() {
    TestHelper::run(|h| async move {
        for i in &["sports", "crypto", "beauty"] {
            h.a().app.topic().insert(InsertTopic { name: i.to_string() }).save().await.unwrap();
        }
        let sports = h.a().app.topic().find(1).await.unwrap();
        let crypto = h.a().app.topic().find(2).await.unwrap();
        let beauty = h.a().app.topic().find(3).await.unwrap();

        let alice = h.collaborator(5000).await;
        let handle = alice.x_handle().await;
        handle.add_topic(&sports).await.unwrap();
        handle.add_topic(&beauty).await.unwrap();

        let advertiser = h.advertiser().await;
        let mut met_topics = advertiser.make_campaign_one(u("100"), 30, &[*beauty.id()]).await;

        assert!(met_topics.make_collab(&handle, u("10"), "first_post_trigger").await.is_ok());

        let mut no_topics = advertiser.make_campaign_one(u("100"), 30, &[]).await;
        assert!(no_topics.make_collab(&handle, u("10"), "second_post_trigger").await.is_ok());

        let mut unmet_topics = advertiser.make_campaign_one(u("100"), 30, &[*crypto.id(), *sports.id()]).await;

        assert_eq!(
            unmet_topics.make_collab(&handle, u("20"), "third_post_trigger").await.unwrap_err().to_string(),
            "Invalid input on topics: handle_is_missing_topics"
        );
    })
    .await;
}
#[tokio::test]
#[serial_test::file_serial]
async fn has_max_and_min_reward() {
    TestHelper::run(|h| async move {
        let advertiser = h.advertiser().await;
        let mut campaign = advertiser
            .start_and_pay_campaign_with_params(
                "https://x.com/somebody/status/1716421161867710954",
                u("100"),
                30,
                &[],
                u("5"),
                milli("50"),
                milli("1"),
                false,
            )
            .await;

        let alice = h.collaborator(3_000_000).await;
        let alice_collab = campaign.make_x_collab_with_user_id(alice.x_user_id()).await.unwrap().unwrap();

        assert_eq!(alice_collab.reward_u256(), u("5"));

        // Bob is a small account.
        let bob = h.collaborator(10).await;
        let bob_collab = campaign.make_x_collab_with_user_id(bob.x_user_id()).await.unwrap().unwrap();
        assert_eq!(bob_collab.reward_u256(), milli("50"));
    })
    .await;
}
/*
// TODO: Do not make collabs or show campaigns for user that was downvoted.
// TODO: Do not make collabs if campaign is for thumbs_up_only and user is not thumbs up.
*/
