use super::*;

#[tokio::test(flavor = "multi_thread")]
#[serial_test::file_serial]
async fn builds_and_curates_advertiser_community() {
    let h = TestHelper::for_web().await;
    {
        let advertiser = h.advertiser().await;
        let alice = h.collaborator(300).await; // user.with_handle(random_handle).scored(300) // A user, with a session and linked randomly generated handle which is scored. 
        let bob = h.collaborator(200).await;
        let carl = h.collaborator(100).await;
        let dan = h.collaborator(150).await;
        let eve = h.collaborator(200).await; 

        let users = [&alice, &bob, &carl, &dan, &eve];

        let campaign = advertiser.make_campaign_one(
            u("1000"), 20, &[]
        ).await;

        h.a().batch_collabs(campaign, &users).await;

        advertiser.login_to_web().await;

        TestApp::wait_for_enter("See the advertisers page for collaborator ranking").await;
        /*
        h.login_to_web(&advertiser).await;

        // h.goto(format!("/Member/{}", advertiser.account_id())).await;
        // Go to the member dashboard, see the collaborator ranking.
        */
    }
    h.stop().await;
}
