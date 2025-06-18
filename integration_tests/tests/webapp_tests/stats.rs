use api::models::{InsertBackerDisbursement, OnChainJobKind, OnChainJobStatus};
use super::*;

#[tokio::test(flavor = "multi_thread")]
#[serial_test::file_serial]
async fn shows_stats() {
    TestHelper::with_web(|h| async move {
        let advertiser = h.advertiser().await;
        let alice = h.collaborator(3_000_000).await;
        let bob = h.collaborator(3_000_000).await;
        let carl = h.collaborator(3_000_000).await;
        let dan = h.collaborator(3_000_000).await;
        let people = vec![&alice, &bob, &carl, &dan];
        let campaign = advertiser.make_campaign_one(u("1000000"), 20, &[]).await;
        h.a().batch_collabs(campaign, &people).await;

        for mut person in [alice, bob, carl, dan] {
            person.claim_account().await;
        }

        h.test_app
            .wait_for_job(
                "gasless claim for everyone",
                OnChainJobKind::GaslessClaimBalances,
                OnChainJobStatus::Settled,
            )
            .await;

        h.a().sync_events_until( "holders are created", || async {
            h.a().app.holder().select().count().await.unwrap() == 5
        }).await;

        for x in 0..5 {
            let backer = h.test_app.app.backer().get_or_create(models::H160::random()).await.unwrap();
            backer.store_today_stake(U256::from(x * 3) * U256::exp10(18)).await.unwrap();
        }

        let filled = h
            .test_app
            .app
            .backer_disbursement()
            .insert(InsertBackerDisbursement {
                rif_claimed: Decimal::new(100, 0),
                rif_usd_rate: Decimal::new(5, 3),
                btc_claimed: Decimal::new(1, 2),
                btc_usd_rate: Decimal::new(100_000, 0),
                asami_issuance_rate: Decimal::new(4000, 0),
                tx_hash: H256::random().encode_hex(),
            })
            .schedule_and_save()
            .await
            .unwrap();

        assert_eq!(filled.backer_stake_scope().count().await.unwrap(), 5);
        h.a().start_mining().await;
        h.a().app.backer_payout().pay_all().await.unwrap();
        h.a().stop_mining().await;
        h.a().app.fee_pool_snapshot().create_if_missing(wei("1"), milli("1"), u("100")).await.unwrap();

        h.web().navigate("/").await;
        TestApp::wait_for_enter("test").await;
    })
    .await;
}
