use models::{OnChainJobKind, OnChainJobStatus};

use super::*;

#[tokio::test]
#[serial_test::file_serial]
async fn claims_fee_pool_share_as_soon_as_possible() {
    TestHelper::run(|h| async move {
        let advertiser = h.advertiser().await;
        let campaign = advertiser.make_campaign_one(u("1000000"), 30, &[]).await;

        let mut alice = h.collaborator(3_000_000).await;
        alice.claim_account().await;

        h.a().batch_collabs(campaign, &[&alice]).await;
        alice.account().await.update().allows_gasless(true).save().await.unwrap();

        h.a()
            .wait_for_job(
                "Collabs should be made",
                OnChainJobKind::GaslessClaimBalances,
                OnChainJobStatus::Settled,
            )
            .await;

        h.a()
            .sync_events_until("alice is holder", || async {
                h.a().app.holder().select().address_ilike(alice.address().encode_hex()).count().await.unwrap() == 1
            })
            .await;

        h.a().evm_forward_to_next_cycle().await;

        assert_eq!(h.a().asami_contract().fee_pool().call().await.unwrap(), u("200"));

        h.a()
            .wait_for_job(
                "Fee pool shares are claimed",
                OnChainJobKind::ClaimFeePoolShare,
                OnChainJobStatus::Settled,
            )
            .await;
        assert_eq!(h.a().asami_contract().fee_pool().call().await.unwrap(), wei("1"));

        let campaign_two = advertiser
            .start_and_pay_campaign(
                "https://x.com/somebody/status/1716421161867710955",
                u("200000"),
                30,
                &[],
            )
            .await;
        h.a().batch_collabs(campaign_two, &[&alice]).await;

        h.a()
            .wait_for_job(
                "No more shares in same cycle",
                OnChainJobKind::ClaimFeePoolShare,
                OnChainJobStatus::Skipped,
            )
            .await;
        assert_eq!(
            h.a().asami_contract().fee_pool().call().await.unwrap(),
            wei("200000000000000000001")
        );

        h.a().evm_forward_to_next_cycle().await;

        h.a()
            .wait_for_job(
                "Claim is possible again",
                OnChainJobKind::ClaimFeePoolShare,
                OnChainJobStatus::Settled,
            )
            .await;
        assert_eq!(h.a().asami_contract().fee_pool().call().await.unwrap(), wei("1"));
    })
    .await;
}
