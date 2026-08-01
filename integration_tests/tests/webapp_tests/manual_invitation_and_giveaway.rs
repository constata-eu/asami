use api::models::AcceptanceStatus;

use super::*;

#[tokio::test(flavor = "multi_thread")]
#[serial_test::file_serial]
async fn manual_invitation_and_giveaway_flow() {
    TestHelper::with_web(|h| async move {
        let admin_creator = h.advertiser().await;
        let admin_user = admin_creator.user().await;
        admin_user.update().admin(true).save().await.unwrap();

        admin_creator
            .account()
            .await
            .update()
            .name("Manual Admin Creator")
            .acceptance_status(AcceptanceStatus::Active)
            .invites_per_month(5)
            .available_invites(5)
            .save()
            .await
            .unwrap();

        let alice = h.signed_up().await;
        alice
            .account()
            .await
            .update()
            .name("Alice Rootstocker")
            .acceptance_status(AcceptanceStatus::Active)
            .save()
            .await
            .unwrap();

        let bob = h.signed_up().await;
        bob.account()
            .await
            .update()
            .name("Bob Rootstocker")
            .acceptance_status(AcceptanceStatus::Active)
            .save()
            .await
            .unwrap();

        let carl = h.signed_up().await;
        carl.account()
            .await
            .update()
            .name("Carl Rootstocker")
            .acceptance_status(AcceptanceStatus::Active)
            .save()
            .await
            .unwrap();

        let guest_one = h.signed_up().await;
        guest_one
            .account()
            .await
            .update()
            .name("Guestlist One")
            .acceptance_status(AcceptanceStatus::Guestlist)
            .save()
            .await
            .unwrap();

        let guest_two = h.signed_up().await;
        guest_two
            .account()
            .await
            .update()
            .name("Guestlist Two")
            .acceptance_status(AcceptanceStatus::Guestlist)
            .save()
            .await
            .unwrap();

        println!("Manual admin creator ready: Manual Admin Creator");
        println!("Seeded active Rootstockers: Alice Rootstocker, Bob Rootstocker, Carl Rootstocker");
        println!("Seeded guestlist accounts: Guestlist One, Guestlist Two");

        h.web().navigate("/").await;
        admin_creator.login_to_web_with_wallet().await;

        TestApp::wait_for_enter("manual_invitation_and_giveaway_flow").await;
    })
    .await;
}
