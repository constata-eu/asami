use models::{OnChainJobKind, OnChainJobStatus};

use super::*;

#[tokio::test]
#[serial_test::file_serial]
async fn applies_voted_fee_rate_as_soon_as_possible() {
    let h = TestHelper::for_model().await;

    let mut campaign = h.user().await.quick_campaign(u("100"), 30, &[]).await;

    let mut alice = h.user().await;
    alice.claim_account().await;
    let handle = alice.create_handle("alice_on_x", "11111", wei("100")).await;
    campaign.make_collab(&handle, u("100"), "first_collab").await.unwrap();

    h.app
        .wait_for_job(
            "Balances should be claimed",
            OnChainJobKind::GaslessClaimBalances,
            OnChainJobStatus::Settled,
        )
        .await;

    assert_eq!(alice.asami_balance().await, u("12000"));

    h.app
        .send_tx(
            "alice votes for 20",
            "96000",
            alice.asami_contract().submit_fee_rate_vote(u("20")),
        )
        .await;

    h.app.evm_forward_to_next_cycle().await;

    assert_eq!(h.app.asami_contract().fee_rate().call().await.unwrap(), u("10"));
    h.app
        .wait_for_job(
            "Fee rate should be applied",
            OnChainJobKind::ApplyVotedFeeRate,
            OnChainJobStatus::Settled,
        )
        .await;
    assert_eq!(h.app.asami_contract().fee_rate().call().await.unwrap(), u("20"));

    h.app
        .wait_for_job(
            "Fee rate should be applied",
            OnChainJobKind::ApplyVotedFeeRate,
            OnChainJobStatus::Skipped,
        )
        .await;

    h.app.evm_forward_to_next_cycle().await;

    h.app
        .send_tx(
            "alice votes for 30",
            "96000",
            alice.asami_contract().submit_fee_rate_vote(u("30")),
        )
        .await;

    h.app
        .wait_for_job(
            "Fee rate should be applied",
            OnChainJobKind::ApplyVotedFeeRate,
            OnChainJobStatus::Skipped,
        )
        .await;
    assert_eq!(h.app.asami_contract().fee_rate().call().await.unwrap(), u("30"));
}
