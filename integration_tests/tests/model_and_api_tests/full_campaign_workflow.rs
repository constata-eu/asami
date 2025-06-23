use ::api::models::*;

use super::*;

#[tokio::test]
#[serial_test::file_serial]
async fn creates_campaign_registers_collabs_and_reimburses() {
    TestHelper::run(|h| async move {
        let advertiser = h.advertiser().await;
        let mut campaign = advertiser.make_campaign_one(u("100"), 20, &[]).await;
        let alice = h.collaborator(12500).await;
        let bob = h.collaborator(112500).await;
        campaign = h.a().batch_collabs(campaign, &[&alice]).await;

        campaign.reload().await.unwrap();

        assert_eq!(campaign.available_budget().await.unwrap(), u("90"));
        assert_eq!(campaign.budget_u256(), u("90"));

        campaign = h.a().batch_collabs(campaign, &[&bob]).await;

        campaign.reload().await.unwrap();
        assert_eq!(campaign.available_budget().await.unwrap(), u("0"));
        assert_eq!(campaign.budget_u256(), u("0"));

        h.a().send_tx("Advertiser tops up", "78582",
            advertiser.top_up_campaign_contract_call(advertiser.address(), campaign.decoded_briefing_hash(), u("15"))
        ).await;

        h.a().sync_events_until("Campaign is topped up", || async {
            campaign.reloaded().await.unwrap().budget_u256() == u("15")
        }).await;

        let original_valid_until = campaign.valid_until();

        h.a().send_tx("Advertiser extends", "35000",
            advertiser.extend_campaign_contract_call(campaign.decoded_briefing_hash(), 40)
        ).await;

        h.a().sync_events_until("Campaign is extended", || async {
            campaign.reloaded().await.unwrap().valid_until() > original_valid_until
        }).await;

        campaign.reload().await.unwrap();
        assert_eq!(campaign.available_budget().await.unwrap(), u("15"));
        assert_eq!(campaign.budget_u256(), u("15"));

        expire_campaign(h.a(), &campaign).await;

        let job = h.a().wait_for_job(
            "Courtesy reimbursement",
            OnChainJobKind::ReimburseCampaigns,
            OnChainJobStatus::Submitted
        ).await;

        h.a().wait_for_job_status("Courtesy reimbursement confirms", &job, OnChainJobStatus::Settled).await;

        h.a().sync_events_until("Campaign is reimbursed", || async {
            campaign.reloaded().await.unwrap().budget_u256() == u("0")
        }).await;

        campaign.reload().await.unwrap();
        assert_eq!(campaign.available_budget().await.unwrap(), u("0"));
        assert_eq!(campaign.budget_u256(), u("0"));
    }).await
}

#[tokio::test]
#[serial_test::file_serial]
async fn notifies_twitter_about_campaign() {
    TestHelper::run(|h| async move {
        let advertiser = h.advertiser().await;
        let mut campaign = advertiser.make_campaign_one(u("100"), 20, &[]).await;

        // h.a().app.campaign().announce_new_campaigns().await?;

        // New campaign by 'aoeuaoeuaoue', pays 0.001 DOC per influence score point. Max 1 DOC, min 0.10.

        // Log-in to asami to check applicability.

        // Not a member yet? Get your free influence reading now and start collaborating.

        // Let's get one billion people into web3.
    }).await
}


/*
app_test! { old_creates_campaign_register_collabs_and_reimburses(a)
    let mut advertiser = a.client().await;
    advertiser.setup_as_advertiser("test main advertiser").await;
    let mut campaign = advertiser.start_and_pay_campaign("https://x.com/somebody/status/1716421161867710954", u("100"), 30, &[]).await;
    assert_eq!(campaign.budget_u256(), u("100"));

    let mut alice = a.client().await;
    alice.claim_account().await;
    let handle = alice.create_handle("alice_on_x", "11111", u("5000")).await;

    a.register_collab("collab for alice", &mut campaign, &handle, u("10"), "unique_post_trigger").await;

    campaign.reload().await?;

    assert_eq!(campaign.available_budget().await.unwrap(), u("90"));
    assert_eq!(campaign.budget_u256(), u("90"));

    a.register_collab("second collab for alice", &mut campaign, &handle, u("90"), "second_post_trigger").await;

    campaign.reload().await?;
    assert_eq!(campaign.available_budget().await.unwrap(), u("0"));
    assert_eq!(campaign.budget_u256(), u("0"));

    a.send_tx("Advertiser tops up", "78582",
        advertiser.top_up_campaign_contract_call(advertiser.address(), campaign.decoded_briefing_hash(), u("15"))
    ).await;

    a.sync_events_until("Campaign is topped up", || async {
        campaign.reloaded().await.unwrap().budget_u256() == u("15")
    }).await;

    let original_valid_until = campaign.valid_until();

    a.send_tx("Advertiser extends", "35000",
        advertiser.extend_campaign_contract_call(campaign.decoded_briefing_hash(), 40)
    ).await;

    a.sync_events_until("Campaign is extended", || async {
        campaign.reloaded().await.unwrap().valid_until() > original_valid_until
    }).await;

    campaign.reload().await?;
    assert_eq!(campaign.available_budget().await.unwrap(), u("15"));
    assert_eq!(campaign.budget_u256(), u("15"));

    expire_campaign(&a, &campaign).await;

    let job = a.wait_for_job(
        "Courtesy reimbursement",
        OnChainJobKind::ReimburseCampaigns,
        OnChainJobStatus::Submitted
    ).await;

    a.wait_for_job_status("Courtesy reimbursement confirms", &job, OnChainJobStatus::Settled).await;

    a.sync_events_until("Campaign is reimbursed", || async {
        campaign.reloaded().await.unwrap().budget_u256() == u("0")
    }).await;

    campaign.reload().await?;
    assert_eq!(campaign.available_budget().await.unwrap(), u("0"));
    assert_eq!(campaign.budget_u256(), u("0"));
}

app_test! { campaign_submit_report_tests(a)
    let mut advertiser = a.client().await;
    advertiser.setup_as_advertiser("test main advertiser").await;
    let mut campaign = advertiser.start_and_pay_campaign("https://x.com/somebody/status/1716421161867710954", u("100"), 30, &[]).await;

    let first_hash = "0x475a9d2a8fc2ae999436694d9bef697e790e6110330d88ebbbd5def7e340d17f";
    assert_eq!(campaign.build_report_hash().await?.encode_hex(), first_hash);

    let mut alice = a.client().await;
    alice.claim_account().await;
    let handle = alice.create_handle("alice_on_x", "11111", u("5000")).await;
    a.register_collab("collab for alice", &mut campaign, &handle, u("10"), "unique_post_trigger").await;

    let second_hash = campaign.build_report_hash().await?.encode_hex();
    assert_ne!(first_hash, second_hash);

    a.wait_for_job(
        "A skipped job to submit reports",
        OnChainJobKind::SubmitReports,
        OnChainJobStatus::Skipped
    ).await;

    expire_campaign(&a, &campaign).await;

    let job = a.wait_for_job(
        "A job to submit reports",
        OnChainJobKind::SubmitReports,
        OnChainJobStatus::Submitted
    ).await;

    a.wait_for_job_status("Submitted report confirms", &job, OnChainJobStatus::Settled).await;

    campaign.reload().await?;
    assert_eq!(campaign.report_hash().as_deref().unwrap(), second_hash);

    a.wait_for_job("No report needed", OnChainJobKind::SubmitReports, OnChainJobStatus::Skipped).await;

    a.send_tx("Campaign is extended", "38314",
        advertiser.extend_campaign_contract_call(campaign.decoded_briefing_hash(), 50)
    ).await;

    a.sync_events_until("Campaign is extended", || async {
        campaign.reloaded().await.unwrap().report_hash().is_none()
    }).await;
}

app_test! { checks_if_still_admin_before_sending_report (a)
    let mut advertiser = a.client().await;
    advertiser.setup_as_advertiser("test main advertiser").await;
    let campaign = advertiser.start_and_pay_campaign("https://x.com/somebody/status/1716421161867710954", u("100"), 30, &[]).await;

    expire_campaign(&a, &campaign).await;

    a.send_tx(
        "Advertiser wants to be its own admin",
        "71200",
        advertiser.asami_contract().config_account(advertiser.address(), u("6"), u("0"), u("0"))
    ).await;

    a.wait_for_job(
        "Job is skipped",
        OnChainJobKind::SubmitReports,
        OnChainJobStatus::Skipped
    ).await;
}

app_test! { reduce_race_condition_risk_submitting_report(a)
    let mut advertiser = a.client().await;
    advertiser.setup_as_advertiser("test main advertiser").await;
    let campaign = advertiser.start_and_pay_campaign("https://x.com/somebody/status/1716421161867710954", u("100"), 30, &[]).await;

    expire_campaign(&a, &campaign).await;

    /* The campaign is extended, but the model doesn't know it because we don't sync the event */
    a.send_tx("Campaign is extended", "38314",
        advertiser.extend_campaign_contract_call(campaign.decoded_briefing_hash(), 50)
    ).await;

    a.wait_for_job(
        "Job is skipped",
        OnChainJobKind::SubmitReports,
        OnChainJobStatus::Skipped
    ).await;
}

app_test! { rejects_collabs_if_registered_over_budget(a)
    let mut advertiser = a.client().await;
    advertiser.setup_as_advertiser("test main advertiser").await;
    let mut campaign = advertiser.start_and_pay_campaign("https://x.com/somebody/status/1716421161867710954", u("100"), 30, &[]).await;
    assert_eq!(campaign.budget_u256(), u("100"));

    let mut alice = a.client().await;
    alice.claim_account().await;
    let handle = alice.create_handle("alice_on_x", "11111", u("5000")).await;

    let one = campaign.make_collab(&handle, u("95"), "trigger_one").await.unwrap();
    let mut two = campaign.make_collab(&handle, u("5"), "trigger_two").await.unwrap();

    assert!(campaign.make_collab(&handle, u("10"), "trigger_two").await.is_err());

    // This update corrupts data, making the collab be registered for more than it could.
    two = two.update().reward(u("50").encode_hex()).save().await.unwrap();

    campaign.reload().await?;
    assert_eq!(campaign.available_budget().await.unwrap(), u("0"));
    assert_eq!(campaign.budget_u256(), u("100"));

    let job = a.wait_for_job(
        "Making collabs",
        OnChainJobKind::MakeCollabs,
        OnChainJobStatus::Settled
    ).await;

    assert_eq!(*two.reloaded().await.unwrap().status(), CollabStatus::Failed);
    let job_collabs = job.on_chain_job_collab_vec().await.unwrap();
    assert_eq!(job_collabs.len(), 1);
    assert_eq!(job_collabs[0].attrs.collab_id, one.attrs.id);
}

app_test! { reduce_race_condition_risk_with_reimbursements(a)
    let mut advertiser = a.client().await;
    advertiser.setup_as_advertiser("test main advertiser").await;
    let mut campaign = advertiser.start_and_pay_campaign("https://x.com/somebody/status/1716421161867710954", u("100"), 30, &[]).await;

    expire_campaign(&a, &campaign).await;

    a.send_tx("Advertiser reimburses", "45000",
        advertiser.reimburse_campaign_contract_call(advertiser.address(), campaign.decoded_briefing_hash())
    ).await;

    a.wait_for_job(
        "Job is skipped",
        OnChainJobKind::SubmitReports,
        OnChainJobStatus::Skipped
    ).await;

    a.sync_events_until("Campaign is reimbursed", || async {
        campaign.reloaded().await.unwrap().budget_u256() == u("0")
    }).await;

    campaign.reload().await?;
    assert_eq!(campaign.available_budget().await.unwrap(), u("0"));
    assert_eq!(campaign.budget_u256(), u("0"));
}

*/

async fn expire_campaign(a: &TestApp, campaign: &Campaign) {
    campaign
        .clone()
        .update()
        .valid_until(Some(Utc::now() - chrono::Duration::days(30)))
        .save()
        .await
        .unwrap();
    a.evm_forward_to_next_cycle().await;
    a.evm_forward_to_next_cycle().await;
    a.evm_forward_to_next_cycle().await;
}
