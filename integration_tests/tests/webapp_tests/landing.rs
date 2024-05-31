browser_test! { shows_campaigns_in_landing (mut d)
    d.test_app().start_mining().await;

    let a = d.test_app();

    let mut advertiser = a.client().await;
    advertiser.setup_as_advertiser("test main advertiser").await;

    for post in &[
        "1716421161867710954",
        "1758208064467935359",
        "1758116416606163059",
        "1752961229407375400",
        "1758192957386342435",
        "1758192965703647443",
        "1758506690213732795",
    ] {
        advertiser.start_and_pay_campaign(
            &format!("https://x.com/somebody/status/{post}"), u("100"), 10, &[]
        ).await;
    }

    for post in &[
        "C2w6_ThRgkY",
        "C3TJIgLgULU",
        "C3PuyCPKNWT",
        "C3GOsATrTTb",
        "C3Ai4_KLqo7",
        "C3SaabGsI1j",
        "C222POpvKho",
    ] {
        advertiser.start_and_pay_campaign(
            &format!("https://instagram.com/status/{post}"), u("100"), 10, &[]
        ).await;
    }

    assert_eq!(a.app.campaign().select().count().await?, 14);

    a.app.ig_crawl().do_everything().await.unwrap();
    let crawl = a.app.ig_crawl().find(1).await?;

    try_until(100, 5000, "no ig crawl", || async {
        a.app.ig_crawl().do_everything().await.unwrap();
        *crawl.reloaded().await.unwrap().processed_for_campaign_rules()
    }).await;

    d.goto("http://127.0.0.1:5173").await;
    d.wait_for("#button-login-as-member").await;
    for i in 1..15 {
        d.wait_for(&format!("#campaign-container-{i}")).await;
    }

    d.test_app().stop_mining().await;
}
