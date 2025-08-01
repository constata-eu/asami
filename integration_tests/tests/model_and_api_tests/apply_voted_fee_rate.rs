use models::{OnChainJobKind, OnChainJobStatus};

use super::*;

#[tokio::test]
#[serial_test::file_serial]
async fn applies_voted_fee_rate() {
    TestHelper::run(|h| async move {
        let advertiser = h.advertiser().await;
        let campaign = advertiser.make_campaign_one(u("1000000"), 30, &[]).await;

        let mut alice = h.collaborator(3_000_000).await;
        alice.claim_account().await;
        alice.account().await.update().allows_gasless(true).save().await.unwrap();

        h.a().batch_collabs(campaign, &[&alice]).await;

        h.a()
            .wait_for_job(
                "Balances should be claimed",
                OnChainJobKind::GaslessClaimBalances,
                OnChainJobStatus::Settled,
            )
            .await;

        assert_eq!(alice.asami_balance().await, u("240000"));

        h.a()
            .send_tx(
                "alice votes for 20",
                "96000",
                alice.asami_contract().submit_fee_rate_vote(u("20")),
            )
            .await;

        h.a().evm_forward_to_next_cycle().await;

        assert_eq!(h.a().asami_contract().fee_rate().call().await.unwrap(), u("10"));
        h.a()
            .wait_for_job(
                "Fee rate should be applied",
                OnChainJobKind::ApplyVotedFeeRate,
                OnChainJobStatus::Settled,
            )
            .await;
        assert_eq!(h.a().asami_contract().fee_rate().call().await.unwrap(), u("20"));

        h.a()
            .wait_for_job(
                "Fee rate should be applied",
                OnChainJobKind::ApplyVotedFeeRate,
                OnChainJobStatus::Skipped,
            )
            .await;

        h.a().evm_forward_to_next_cycle().await;

        h.a()
            .send_tx(
                "alice votes for 30",
                "96000",
                alice.asami_contract().submit_fee_rate_vote(u("30")),
            )
            .await;

        h.a()
            .wait_for_job(
                "Fee rate should be applied",
                OnChainJobKind::ApplyVotedFeeRate,
                OnChainJobStatus::Skipped,
            )
            .await;
        assert_eq!(h.a().asami_contract().fee_rate().call().await.unwrap(), u("30"));
    })
    .await;
}
