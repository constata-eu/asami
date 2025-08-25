use super::*;

#[tokio::test(flavor = "multi_thread")]
#[serial_test::file_serial]
async fn shows_landing() {
    TestHelper::with_web(|h| async move {
        let advertiser = h.advertiser().await;

        for post in &[
            "1716421161867710954",
            "1758208064467935359",
            "1758116416606163059",
            "1752961229407375400",
            "1758192957386342435",
            "1758192965703647443",
            "1758506690213732795",
        ] {
            advertiser
                .start_and_pay_campaign(&format!("https://x.com/somebody/status/{post}"), u("100"), 10, &[])
                .await;
        }

        assert_eq!(h.test_app.app.campaign().select().count().await.unwrap(), 7);
        h.web().navigate("/").await;
        TestApp::wait_for_enter("here").await;
    })
    .await;
}
