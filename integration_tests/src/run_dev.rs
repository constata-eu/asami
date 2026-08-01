use api::models::{u, AcceptanceStatus, CampaignKind, CreateCampaignFromLinkInput, UpsertCommunityMemberScore, Utc};
use chrono::TimeZone;
use integration_tests::support::*;
use tokio::{task::JoinHandle, time::{sleep, Duration}};

fn default_rootstock_joined_at() -> chrono::DateTime<Utc> {
    Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap()
}

async fn seed_member(h: &TestHelper, name: &str, username: &str, user_id: &str) -> TestUser {
    let user = h.user().await.signed_up().await.unverified(username, user_id).await.active(0).await;

    user.account()
        .await
        .update()
        .name(name.to_string())
        .acceptance_status(AcceptanceStatus::Active)
        .save()
        .await
        .unwrap();

    user
}

fn start_background_processing(h: &TestHelper) -> JoinHandle<()> {
    let provider = h.a().provider.clone();
    let app = h.a().app.clone();

    tokio::spawn(async move {
        loop {
            let _ = provider.request::<Vec<()>, api::models::U256>("evm_mine", vec![]).await;
            let _ = provider
                .request::<_, serde_json::Value>("evm_setAutomine", vec![false])
                .await;
            let _ = app.synced_event().sync_on_chain_events().await;
            let _ = app.campaign().prepare_collabs().await;
            let _ = app.on_chain_job().run_scheduler().await;
            let _ = app.synced_event().sync_on_chain_events().await;
            sleep(Duration::from_secs(2)).await;
        }
    })
}

async fn seed_score(app: &api::App, scorer: &TestUser, target: &TestUser, priority: bool, score: i32, comment: &str) {
    app.community_member()
        .upsert_score(
            UpsertCommunityMemberScore {
                account_id: scorer.account_id().encode_hex(),
                member_id: target.account_id().encode_hex(),
                priority,
                alignment: score,
                trust: score,
                contribution: score,
                advocacy: score,
                judgment: score,
                rootstock_joined_at: default_rootstock_joined_at(),
                comment: Some(comment.to_string()),
            },
            true,
        )
        .await
        .unwrap();
}

async fn sync_until_campaign_budget(app: &api::App, campaign_id: i32) {
    TestApp::wait_for(80, 50, || async {
        app.synced_event().sync_on_chain_events().await.unwrap();
        app.campaign().find(campaign_id).await.unwrap().budget_u256() > u("0")
    })
    .await;
}

async fn seed_campaigns(app: &api::App, creator: &TestUser) -> anyhow::Result<()> {
    let creator_account = creator.account().await;

    let repost = CreateCampaignFromLinkInput {
        kind: CampaignKind::XRepost,
        link: Some("https://x.com/somebody/status/1758116416606163059".to_string()),
        topic_ids: vec![],
        max_individual_reward: u("25").encode_hex(),
        thumbs_up_only: false,
        all_members: false,
        member_ids: vec![],
    }
    .process(app, &creator_account)
    .await?;

    creator.pay_campaign("seeded repost campaign", u("250"), repost.decoded_briefing_hash(), 20).await;

    sync_until_campaign_budget(app, repost.attrs.id).await;

    let giveaway = CreateCampaignFromLinkInput {
        kind: CampaignKind::CommunityGiveaway,
        link: None,
        topic_ids: vec![],
        max_individual_reward: u("80").encode_hex(),
        thumbs_up_only: false,
        all_members: true,
        member_ids: vec![],
    }
    .process(app, &creator_account)
    .await?;

    creator
        .pay_campaign(
            "seeded community giveaway",
            u("400"),
            giveaway.decoded_briefing_hash(),
            20,
        )
        .await;

    sync_until_campaign_budget(app, giveaway.attrs.id).await;

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("run-dev: starting web helper");
    let h = TestHelper::for_web().await;
    let background = start_background_processing(&h);
    {
        let app = h.a().app.clone();

        println!("run-dev: creating admin creator");
        let admin_creator = h.advertiser().await;
        println!("run-dev: marking admin creator as admin");
        admin_creator.user().await.update().admin(true).save().await.unwrap();

        println!("run-dev: updating admin creator account");
        admin_creator
            .account()
            .await
            .update()
            .name("Manual Admin Creator".to_string())
            .acceptance_status(AcceptanceStatus::Active)
            .invites_per_month(12)
            .available_invites(12)
            .save()
            .await
            .unwrap();

        println!("run-dev: seeding active members");
        let alice = seed_member(&h, "Alice Rootstocker", "alice_rootstocker", "200001").await;
        let bob = seed_member(&h, "Bob Rootstocker", "bob_rootstocker", "200002").await;
        let carl = seed_member(&h, "Carl Rootstocker", "carl_rootstocker", "200003").await;
        let nubis = seed_member(&h, "Nubis Bruno", "nubis_bruno", "200004").await;

        println!("run-dev: seeding guestlist accounts");
        let guest_one = h.signed_up().await;
        guest_one
            .account()
            .await
            .update()
            .name("Guestlist One".to_string())
            .acceptance_status(AcceptanceStatus::Guestlist)
            .save()
            .await
            .unwrap();

        let guest_two = h.signed_up().await;
        guest_two
            .account()
            .await
            .update()
            .name("Guestlist Two".to_string())
            .acceptance_status(AcceptanceStatus::Guestlist)
            .save()
            .await
            .unwrap();

        println!("run-dev: seeding community scores");
        seed_score(&app, &admin_creator, &alice, true, 8, "Strong community participant").await;
        seed_score(&app, &admin_creator, &bob, false, 7, "Reliable collaborator").await;
        seed_score(&app, &admin_creator, &carl, false, 6, "Newer but aligned").await;
        seed_score(&app, &admin_creator, &nubis, false, 9, "Strong Rootstock presence").await;

        println!("run-dev: seeding campaigns");
        seed_campaigns(&app, &admin_creator).await?;

        println!("Local environment ready.");
        println!("Admin creator: Manual Admin Creator");
        println!("Active Rootstockers: Alice Rootstocker, Bob Rootstocker, Carl Rootstocker, Nubis Bruno");
        println!("Guestlist accounts: Guestlist One, Guestlist Two");
        println!("The browser will log in with the admin creator wallet and open the community dashboard.");

        println!("run-dev: opening site");
        h.web().goto("http://127.0.0.1:5173/").await;

        println!("run-dev: logging into web with wallet");
        admin_creator.login_to_web_with_wallet().await;
        println!("run-dev: navigating to advertiser dashboard");
        h.web().navigate("/dashboard?role=advertiser").await;
        println!("run-dev: waiting for advertiser dashboard");
        h.web().wait_for("#advertiser-dashboard").await;
        println!("run-dev: environment ready and waiting for enter");

        TestApp::wait_for_enter("run-dev: press enter to stop the local environment").await;
    }
    background.abort();
    h.stop().await;

    Ok(())
}
