use super::*;

#[tokio::test]
#[serial_test::file_serial]
async fn has_default_account_names() {
    TestHelper::run(|h| async move {
        let alice = h.advertiser().await;
        let mut bob = h.signed_up().await;
        let carl = h.collaborator(200).await;

        assert_eq!(
            alice.account().await.reloaded().await.unwrap().name(),
            "Antonia Templum Pius #1"
        );
        assert_eq!(
            bob.account().await.reloaded().await.unwrap().name(),
            "Aurelius Hasta Mysticus #2"
        );
        assert!(carl.account().await.reloaded().await.unwrap().name().starts_with("twittero_"));

        bob = bob.rand_unverified().await.active(200).await;

        assert!(bob.account().await.reloaded().await.unwrap().name().starts_with("twittero_"));
    })
    .await
}
