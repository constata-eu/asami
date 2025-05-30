use models::CampaignStatus;

use super::*;

#[tokio::test(flavor = "multi_thread")]
#[serial_test::file_serial]
async fn builds_and_curates_advertiser_community() {
    TestHelper::with_web(|h| async move {
        let advertiser = h.advertiser().await;
        let alice = h.collaborator(300).await;
        let bob = h.collaborator(200).await;
        let carl = h.collaborator(100).await;
        let dan = h.collaborator(150).await;
        let eve = h.collaborator(200).await;

        let users = [&alice, &bob, &carl, &dan, &eve];

        let campaign = advertiser.make_campaign_one(u("1000"), 20, &[]).await;

        h.a().batch_collabs(campaign, &users).await;

        advertiser.login_to_web_with_wallet().await;
        h.web().click("#button-cancel-grant-permission-and-make-post").await;
        h.web().click("#menu-my-campaings").await;

        h.web().click(&format!("#rating-{}-BAD", alice.account_id())).await;
        h.web().wait_for_text(".MuiSnackbarContent-message", "Rating saved").await;
        h.web().wait_until_gone(".MuiSnackbarContent-message").await;

        h.web().click(&format!("#rating-{}-GOOD", bob.account_id())).await;

        h.a().start_mining().await;
        h.web()
            .open_and_fill_doc_campaign_form(
                "https://x.com/asami_club_es/status/1925491997223043127",
                "200",
                false, // Everyone
                false,
            )
            .await;
        h.web()
            .open_and_fill_doc_campaign_form(
                "https://x.com/jack/status/1925607683454869946",
                "200",
                true, // Only thumbs up
                false,
            )
            .await;

        h.a()
            .sync_events_until("Both campaigns have been created", || async {
                h.a().app.campaign().select().status_eq(CampaignStatus::Published).count().await.unwrap() == 3
                    && h.a().app.campaign().select().thumbs_up_only_eq(true).count().await.unwrap() == 1
            })
            .await;

        h.a().stop_mining().await;

        h.web().logout().await;
        h.web().login(&bob).await;
        h.web().wait_for("#button-repost-2").await;
        h.web().wait_for("#button-repost-3").await;

        h.web().logout().await;
        h.web().login(&carl).await;
        h.web().wait_for("#button-repost-2").await;
        h.web().wait_until_gone_with_timeout("#button-repost-3", 1000).await;

        h.web().logout().await;
        h.web().login(&alice).await;
        h.web().wait_for("#title-card-no-campaigns").await;
    })
    .await;
}
