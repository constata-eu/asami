use super::*;

#[tokio::test(flavor = "multi_thread")]
#[serial_test::file_serial]
async fn only_walletconnect_on_embedded_mode() {
    TestHelper::with_web(|h| async move {
        let mut bob = h.user().await.signed_up().await;
        bob.claim_account().await;

        h.web().navigate("/embedded").await;
        h.web().wait_for("w3m-modal").await;
    })
    .await;
}

#[tokio::test(flavor = "multi_thread")]
#[serial_test::file_serial]
async fn embedded_mode_has_simplified_ux() {
    TestHelper::with_web(|h| async move {
        let mut bob = h.user().await.signed_up().await;
        bob.claim_account().await;
        // Bob needs to login before going to embedded mode,
        // because tests cannot login on WC embedded mode.
        bob.login_to_web_with_wallet().await;

        h.web().navigate("/embedded").await;
        h.web().driver.refresh().await.unwrap();

        h.web().navigate("/").await;
        h.web().wait_for("#member-dashboard").await;
    })
    .await;
}

#[tokio::test(flavor = "multi_thread")]
#[serial_test::file_serial]
async fn migrates_account_that_was_connected() {
    TestHelper::with_web(|h| async move {
        let advertiser = h.advertiser().await;
        let campaign = advertiser.make_campaign_one(u("100"), 20, &[]).await;
        let mut alice = h.collaborator(12500).await;
        let eve = h.collaborator(12500).await;
        alice.claim_account().await;
        h.a().batch_collabs(campaign, &[&alice, &eve]).await;

        // Now bob should login to web.
        let mut bob = h.user().await.signed_up().await;
        bob.claim_account().await;

        assert!(*alice.account().await.total_collabs() == 1);
        assert!(*bob.account().await.total_collabs() == 0);

        // Bob needs to login before going to embedded mode,
        // because tests cannot login on WC embedded mode.
        bob.login_to_web_with_wallet().await;

        h.web().navigate("/embedded").await;
        // A strong refresh is needed for the embedded flag to kick in.
        h.web().driver.refresh().await.unwrap();

        h.web().click("#button-grant-permission-and-make-post").await;
        h.web().click("#device-option-merge_account_button").await;
        h.web().click("#start-merging-button").await;

        h.web().wait_for("#merge-link-text").await;
        // A strong refresh later, it still shows the same merge.
        h.web().driver.refresh().await.unwrap();
        h.web().wait_for("#merge-link-text").await;

        let code = h.a().app.account_merge().find(1).await.unwrap().attrs.code.unwrap();

        h.web().click("#changed-my-mind").await;
        h.web().click("#menu-logout").await;

        // A strong refresh is needed for the undo of the embedding to kick in.
        h.web().navigate("/un-embedded").await;
        h.web().driver.refresh().await.unwrap();

        alice.login_to_web_with_otp().await;

        h.web().navigate(&format!("/m/{code}")).await;

        h.web().click("#accept-merge-button").await;
        h.web().click("#confirm-merge-button").await;
        h.web().wait_for("#merge-done").await;

        bob.login_to_web_with_otp().await;
        h.web().navigate("/merge-accounts").await;

        h.web().wait_for("#account-merged-succesfully").await;
        h.web().navigate("/dashboard").await;
        h.web().wait_for("tr.RaDatagrid-row[resource=Collab]").await;

        h.a().app.account().force_hydrate().await.unwrap();

        assert!(*alice.account().await.total_collabs() == 0);
        assert!(*bob.account().await.total_collabs() == 1);
    })
    .await;
}

#[tokio::test(flavor = "multi_thread")]
#[serial_test::file_serial]
async fn enforces_recent_session_for_merge() {
    TestHelper::with_web(|h| async move {
        let advertiser = h.advertiser().await;
        let campaign = advertiser.make_campaign_one(u("100"), 20, &[]).await;
        let mut alice = h.collaborator(12500).await;
        let eve = h.collaborator(12500).await;
        alice.claim_account().await;
        h.a().batch_collabs(campaign, &[&alice, &eve]).await;

        // Now bob should login to web.
        let mut bob = h.user().await.signed_up().await;
        bob.claim_account().await;

        assert!(*alice.account().await.total_collabs() == 1);
        assert!(*bob.account().await.total_collabs() == 0);

        bob.login_to_web_with_wallet().await;
        h.web().wait_for("#button-grant-permission-and-make-post").await;
        h.web().navigate("/merge-accounts").await;
        h.web().click("#start-merging-button").await;
        h.web().wait_for("#merge-link-text").await;

        h.web().navigate("/").await;
        h.web().click("#menu-logout").await;

        alice.login_to_web_with_otp().await;

        let code = h.a().app.account_merge().find(1).await.unwrap().attrs.code.unwrap();
        h.web().navigate(&format!("/m/{code}")).await;
        h.web().wait_for("#accept-merge-button").await;

        h.web()
            .driver
            .execute(
                r#"
            localStorage.setItem(
              "session",
              JSON.stringify({
                ...JSON.parse(localStorage.getItem("session")),
                "createdAt": "2020-01-01T00:00:00.000000Z"
              })
            )"#,
                vec![],
            )
            .await
            .unwrap();

        h.web().driver.refresh().await.unwrap();

        h.web().click("#login-again").await;
        h.web().login_base(&alice).await;
        h.web().click("#accept-merge-button").await;
        h.web().click("#confirm-merge-button").await;
        h.web().wait_for("#merge-done").await;
    })
    .await;
}

#[tokio::test(flavor = "multi_thread")]
#[serial_test::file_serial]
async fn merge_link_times_out_can_create_a_new_one() {
    TestHelper::with_web(|h| async move {
        let mut alice = h.collaborator(12500).await;
        alice.claim_account().await;
        let mut bob = h.user().await.signed_up().await;
        bob.claim_account().await;

        bob.login_to_web_with_wallet().await;
        h.web().wait_for("#button-grant-permission-and-make-post").await;
        h.web().navigate("/merge-accounts").await;
        h.web().click("#start-merging-button").await;
        h.web().wait_for("#merge-link-text").await;
        h.web().navigate("/").await;
        h.web().click("#menu-logout").await;

        let code = h
            .a()
            .app
            .account_merge()
            .find(1)
            .await
            .unwrap()
            .update()
            .created_at(Utc::now() - chrono::Duration::days(1))
            .save()
            .await
            .unwrap()
            .attrs
            .code
            .unwrap();

        alice.login_to_web_with_otp().await;
        h.web().navigate(&format!("/m/{code}")).await;

        h.web().click("#accept-merge-button").await;
        h.web().click("#confirm-merge-button").await;
        h.web().wait_for("#alert-error-no_active_code").await;

        bob.login_to_web_with_otp().await;
        h.web().navigate("/merge-accounts").await;
        h.web().click("#start-merging-button").await;
        h.web().wait_for("#merge-link-text").await;

        let new_code = h.a().app.account_merge().find(2).await.unwrap().attrs.code.unwrap();

        alice.login_to_web_with_otp().await;
        h.web().navigate(&format!("/m/{new_code}")).await;
        h.web().click("#accept-merge-button").await;
        h.web().click("#confirm-merge-button").await;
        h.web().wait_for("#merge-done").await;
    })
    .await;
}

#[tokio::test(flavor = "multi_thread")]
#[serial_test::file_serial]
async fn session_age_is_server_side_validated() {
    TestHelper::with_web(|h| async move {
        let mut alice = h.collaborator(12500).await;
        alice.claim_account().await;
        let mut bob = h.user().await.signed_up().await;
        bob.claim_account().await;

        bob.login_to_web_with_wallet().await;
        h.web().wait_for("#button-grant-permission-and-make-post").await;
        h.web().navigate("/merge-accounts").await;
        h.web().click("#start-merging-button").await;
        h.web().wait_for("#merge-link-text").await;
        h.web().navigate("/").await;
        h.web().click("#menu-logout").await;

        let code = h.a().app.account_merge().find(1).await.unwrap().attrs.code.unwrap();

        alice.login_to_web_with_otp().await;
        h.web().navigate(&format!("/m/{code}")).await;

        // Session gets old here, before attempting the merge.
        h.a()
            .app
            .session()
            .select()
            .user_id_eq(alice.user_id())
            .order_by(models::SessionOrderBy::CreatedAt)
            .desc(true)
            .one()
            .await
            .unwrap()
            .update()
            .created_at(Utc::now() - chrono::Duration::days(1))
            .save()
            .await
            .unwrap();

        h.web().click("#accept-merge-button").await;
        h.web().click("#confirm-merge-button").await;
        h.web().wait_for("#alert-error-session_is_too_old").await;
    })
    .await;
}

#[tokio::test(flavor = "multi_thread")]
#[serial_test::file_serial]
async fn cannot_merge_to_itself() {
    TestHelper::with_web(|h| async move {
        let mut bob = h.user().await.signed_up().await;
        bob.claim_account().await;

        bob.login_to_web_with_wallet().await;
        h.web().wait_for("#button-grant-permission-and-make-post").await;
        h.web().navigate("/merge-accounts").await;
        h.web().click("#start-merging-button").await;
        h.web().wait_for("#merge-link-text").await;
        h.web().navigate("/").await;
        h.web().click("#menu-logout").await;
        bob.login_to_web_with_otp().await;

        let code = h.a().app.account_merge().find(1).await.unwrap().attrs.code.unwrap();

        h.web().navigate(&format!("/m/{code}")).await;

        h.web().click("#accept-merge-button").await;
        h.web().click("#confirm-merge-button").await;
        h.web().wait_for("#alert-error-cannot_merge_to_yourself").await;
    })
    .await;
}

#[tokio::test(flavor = "multi_thread")]
#[serial_test::file_serial]
async fn can_continue_connecting_to_x_elsewhere() {
    TestHelper::with_web(|h| async move {
        let mut alice = h.user().await.signed_up().await;
        alice.claim_account().await;

        alice.login_to_web_with_wallet().await;

        h.web().navigate("/embedded").await;
        // A strong refresh is needed for the embedded flag to kick in.
        h.web().driver.refresh().await.unwrap();

        h.web().click("#button-grant-permission-and-make-post").await;
        h.web().click("#device-option-other_device_button").await;
        h.web().wait_for("#grant-elsewhere-container").await;

        let code = h
            .a()
            .app
            .one_time_token()
            .select()
            .order_by(models::OneTimeTokenOrderBy::Id)
            .desc(true)
            .one()
            .await
            .unwrap()
            .attrs
            .value;

        h.web().navigate(&format!("/s/{code}")).await;
        TestApp::try_until(10, 10, "redirect to X login", || async {
            h.web().driver.current_url().await.unwrap().to_string().starts_with("https://x.com/i/flow/login")
        });
    })
    .await;
}
