use ::api::models::*;
use api::Error;

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

        h.a()
            .send_tx(
                "Advertiser tops up",
                "78582",
                advertiser.top_up_campaign_contract_call(
                    advertiser.address(),
                    campaign.decoded_briefing_hash(),
                    u("15"),
                ),
            )
            .await;

        h.a()
            .sync_events_until("Campaign is topped up", || async {
                campaign.reloaded().await.unwrap().budget_u256() == u("15")
            })
            .await;

        let original_valid_until = campaign.valid_until();

        h.a()
            .send_tx(
                "Advertiser extends",
                "35000",
                advertiser.extend_campaign_contract_call(campaign.decoded_briefing_hash(), 40),
            )
            .await;

        h.a()
            .sync_events_until("Campaign is extended", || async {
                campaign.reloaded().await.unwrap().valid_until() > original_valid_until
            })
            .await;

        campaign.reload().await.unwrap();
        assert_eq!(campaign.available_budget().await.unwrap(), u("15"));
        assert_eq!(campaign.budget_u256(), u("15"));

        expire_campaign(h.a(), &campaign).await;

        let job = h
            .a()
            .wait_for_job(
                "Courtesy reimbursement",
                OnChainJobKind::ReimburseCampaigns,
                OnChainJobStatus::Submitted,
            )
            .await;

        h.a().wait_for_job_status("Courtesy reimbursement confirms", &job, OnChainJobStatus::Settled).await;

        h.a()
            .sync_events_until("Campaign is reimbursed", || async {
                campaign.reloaded().await.unwrap().budget_u256() == u("0")
            })
            .await;

        campaign.reload().await.unwrap();
        assert_eq!(campaign.available_budget().await.unwrap(), u("0"));
        assert_eq!(campaign.budget_u256(), u("0"));
    })
    .await
}

#[tokio::test]
#[serial_test::file_serial]
async fn cannot_duplicate_briefing_hashes() {
    TestHelper::run(|h| async move {
        let advertiser = h.advertiser().await;
        let link = "https://x.com/somebody/status/1716421161867710954";

        advertiser.start_and_pay_campaign(link, u("100"), 15, &[]).await;

        let input = CreateCampaignFromLinkInput {
            link: link.to_string(),
            managed_unit_amount: None,
            topic_ids: vec![],
            price_per_point: milli("1").encode_hex(),
            max_individual_reward: u("2").encode_hex(),
            min_individual_reward: milli("200").encode_hex(),
            thumbs_up_only: false,
        };

        assert!(matches!(
            input.process(&h.a().app, &advertiser.account().await ).await.unwrap_err(),
            Error::Validation(x, y) if x == "link" && y == "campaign_already_exists"
        ));
    })
    .await;
}

#[tokio::test]
#[serial_test::file_serial]
async fn campaign_submit_report_tests() {
    TestHelper::run(|h| async move {
        let advertiser = h.advertiser().await;
        let mut campaign = advertiser
            .start_and_pay_campaign("https://x.com/somebody/status/1716421161867710954", u("100"), 30, &[])
            .await;

        let first_hash = "0x475a9d2a8fc2ae999436694d9bef697e790e6110330d88ebbbd5def7e340d17f";
        assert_eq!(campaign.build_report_hash().await.unwrap().encode_hex(), first_hash);

        let alice = h.collaborator(100).await;
        campaign = h.a().batch_collabs(campaign, &[&alice]).await;

        let second_hash = campaign.build_report_hash().await.unwrap().encode_hex();
        assert_ne!(first_hash, second_hash);

        h.a()
            .wait_for_job(
                "A skipped job to submit reports",
                OnChainJobKind::SubmitReports,
                OnChainJobStatus::Skipped,
            )
            .await;

        expire_campaign(h.a(), &campaign).await;

        let job = h
            .a()
            .wait_for_job(
                "A job to submit reports",
                OnChainJobKind::SubmitReports,
                OnChainJobStatus::Submitted,
            )
            .await;

        h.a().wait_for_job_status("Submitted report confirms", &job, OnChainJobStatus::Settled).await;

        campaign.reload().await.unwrap();
        assert_eq!(campaign.report_hash().as_deref().unwrap(), second_hash);

        h.a()
            .wait_for_job(
                "No report needed",
                OnChainJobKind::SubmitReports,
                OnChainJobStatus::Skipped,
            )
            .await;

        h.a()
            .send_tx(
                "Campaign is extended",
                "38314",
                advertiser.extend_campaign_contract_call(campaign.decoded_briefing_hash(), 50),
            )
            .await;

        h.a()
            .sync_events_until("Campaign is extended", || async {
                campaign.reloaded().await.unwrap().report_hash().is_none()
            })
            .await;
    })
    .await
}

#[tokio::test]
#[serial_test::file_serial]
async fn checks_if_still_admin_before_sending_report() {
    TestHelper::run(|h| async move {
        let advertiser = h.advertiser().await;
        let campaign = advertiser
            .start_and_pay_campaign("https://x.com/somebody/status/1716421161867710954", u("100"), 30, &[])
            .await;

        expire_campaign(h.a(), &campaign).await;

        h.a()
            .send_tx(
                "Advertiser wants to be its own admin",
                "71200",
                advertiser.asami_contract().config_account(advertiser.address(), u("6"), u("0"), u("0")),
            )
            .await;

        h.a()
            .wait_for_job(
                "Job is skipped",
                OnChainJobKind::SubmitReports,
                OnChainJobStatus::Skipped,
            )
            .await;
    })
    .await
}

#[tokio::test]
#[serial_test::file_serial]
async fn reduce_race_condition_risk_submitting_report() {
    TestHelper::run(|h| async move {
        let advertiser = h.advertiser().await;
        let campaign = advertiser
            .start_and_pay_campaign("https://x.com/somebody/status/1716421161867710954", u("100"), 30, &[])
            .await;

        expire_campaign(h.a(), &campaign).await;

        /* The campaign is extended, but the model doesn't know it because we don't sync the event */
        h.a()
            .send_tx(
                "Campaign is extended",
                "38314",
                advertiser.extend_campaign_contract_call(campaign.decoded_briefing_hash(), 50),
            )
            .await;

        h.a()
            .wait_for_job(
                "Job is skipped",
                OnChainJobKind::SubmitReports,
                OnChainJobStatus::Skipped,
            )
            .await;
    })
    .await
}

#[tokio::test]
#[serial_test::file_serial]
async fn rejects_collabs_if_registered_over_budget() {
    TestHelper::run(|h| async move {
        let advertiser = h.advertiser().await;
        let mut campaign = advertiser
            .start_and_pay_campaign("https://x.com/somebody/status/1716421161867710954", u("10"), 30, &[])
            .await;
        assert_eq!(campaign.budget_u256(), u("10"));

        let alice = h.collaborator(5000).await;
        let bob = h.collaborator(5000).await;
        let carl = h.collaborator(5000).await;

        let alice_collab = campaign.make_x_collab_with_user_id(alice.x_user_id()).await.unwrap().unwrap();
        let mut bob_collab = campaign.make_x_collab_with_user_id(bob.x_user_id()).await.unwrap().unwrap();

        assert!(matches!(
            campaign.make_collab(&carl.x_handle().await, u("10"), "carl_trigger").await.unwrap_err(),
            Error::Validation(_, y) if y == "campaign_has_not_enough_funds"
        ));

        bob_collab = bob_collab.update().reward(u("10").encode_hex()).save().await.unwrap();
        campaign.reload().await.unwrap();
        assert_eq!(campaign.available_budget().await.unwrap(), u("0"));
        assert_eq!(campaign.budget_u256(), u("10"));

        let job = h
            .a()
            .wait_for_job(
                "Making collabs",
                OnChainJobKind::MakeSubAccountCollabs,
                OnChainJobStatus::Settled,
            )
            .await;

        assert_eq!(*bob_collab.reloaded().await.unwrap().status(), CollabStatus::Failed);
        let job_collabs = job.on_chain_job_collab_vec().await.unwrap();
        assert_eq!(job_collabs.len(), 1);
        assert_eq!(job_collabs[0].attrs.collab_id, alice_collab.attrs.id);
    })
    .await
}

#[tokio::test]
#[serial_test::file_serial]
async fn reduce_race_condition_risk_with_reimbursements() {
    TestHelper::run(|h| async move {
        let advertiser = h.advertiser().await;
        let mut campaign = advertiser.make_campaign_one(u("100"), 30, &[]).await;

        expire_campaign(h.a(), &campaign).await;

        h.a()
            .send_tx(
                "Advertiser reimburses",
                "45000",
                advertiser.reimburse_campaign_contract_call(advertiser.address(), campaign.decoded_briefing_hash()),
            )
            .await;

        h.a()
            .wait_for_job(
                "Job is skipped",
                OnChainJobKind::SubmitReports,
                OnChainJobStatus::Skipped,
            )
            .await;

        h.a()
            .sync_events_until("Campaign is reimbursed", || async {
                campaign.reloaded().await.unwrap().budget_u256() == u("0")
            })
            .await;

        campaign.reload().await.unwrap();
        assert_eq!(campaign.available_budget().await.unwrap(), u("0"));
        assert_eq!(campaign.budget_u256(), u("0"));
    })
    .await
}

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
