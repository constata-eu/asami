use api::models::{InsertBackerDisbursement, OnChainJobKind, OnChainJobStatus};

use super::*;

#[tokio::test]
#[serial_test::file_serial]
async fn pays_out_backers() {
    std::env::set_var("ROCKET_RSK", "{admin_claim_trigger=0x0}");

    TestHelper::run(|h| async move {
        let advertiser = h.advertiser().await;
        let alice = h.collaborator(3_000_000).await;
        let bob = h.collaborator(3_000_000).await;
        let carl = h.collaborator(3_000_000).await;
        let dan = h.collaborator(3_000_000).await;
        let campaign = advertiser.make_campaign_one(u("1000000"), 20, &[]).await;
        h.a().batch_collabs(campaign, &[&alice, &bob, &carl, &dan]).await;

        h.test_app.start_mining().await;
        h.test_app.app.synced_event().sync_on_chain_events().await.unwrap();
        h.test_app.stop_mining().await;

        h.test_app
            .wait_for_job(
                "admin claimed balance",
                OnChainJobKind::AdminClaimBalancesFree,
                OnChainJobStatus::Settled,
            )
            .await;

        let empty = h
            .test_app
            .app
            .backer_disbursement()
            .insert(InsertBackerDisbursement {
                rif_claimed: Decimal::new(100, 0),
                btc_claimed: Decimal::new(100, 3),
                rif_usd_rate: Decimal::new(5, 3),
                btc_usd_rate: Decimal::new(100_000, 0),
                asami_issuance_rate: Decimal::new(4000, 0),
                tx_hash: H256::random().encode_hex(),
            })
            .schedule_and_save()
            .await
            .unwrap();

        assert_eq!(empty.backer_stake_scope().count().await.unwrap(), 0);
        assert_eq!(empty.backer_payout_scope().count().await.unwrap(), 0);

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
        assert_eq!(filled.backer_payout_scope().count().await.unwrap(), 5);

        // Only 10% of the funds will generate asami tokens, and from that,
        // only 30% are for the advertiser.
        // And then we make it double as a bonus.
        assert_eq!(filled.asamis_to_distribute(), Decimal::new(240_120, 0));

        let payouts = filled.backer_payout_vec().await.unwrap();

        assert_eq!(payouts.len(), 5);
        assert!(payouts.iter().all(|x| !x.paid()));

        assert_eq!(h.test_app.admin_asami_balance().await, u("1280000"));
        assert_eq!(filled.asamis_to_distribute(), Decimal::new(240_120, 0));
        h.a().start_mining().await;
        h.test_app.app.backer_payout().pay_all().await.unwrap();
        h.a().stop_mining().await;
        assert_eq!(h.test_app.admin_asami_balance().await, u("1039880"));

        let holdout = h.test_app.app.backer_payout().find(1).await.unwrap();
        assert!(!holdout.paid());
        assert!(holdout.tx_hash().is_none());

        for id in 2..=5 {
            let payout = h.test_app.app.backer_payout().find(id).await.unwrap();
            assert!(*payout.paid());
            assert!(payout.tx_hash().is_some());
        }

        let another_empty = h
            .test_app
            .app
            .backer_disbursement()
            .insert(InsertBackerDisbursement {
                rif_claimed: Decimal::new(100, 0),
                btc_claimed: Decimal::new(100, 3),
                rif_usd_rate: Decimal::new(5, 3),
                btc_usd_rate: Decimal::new(100_000, 0),
                asami_issuance_rate: Decimal::new(4000, 0),
                tx_hash: H256::random().encode_hex(),
            })
            .schedule_and_save()
            .await
            .unwrap();

        assert_eq!(another_empty.backer_stake_scope().count().await.unwrap(), 0);
        assert_eq!(another_empty.backer_payout_scope().count().await.unwrap(), 0);
        assert_eq!(h.test_app.app.backer_payout().select().count().await.unwrap(), 5);
        assert_eq!(h.test_app.app.backer_stake().select().count().await.unwrap(), 5);
    })
    .await;
}

#[tokio::test]
#[serial_test::file_serial]
async fn creates_disbursements_from_on_chain_data() {
    std::env::set_var(
        "ROCKET_RSK",
        format!(
            "{{ readonly_mainnet_rpc_url={} }}",
            std::env::var("MAINNET_READONLY_RPC").unwrap()
        ),
    );

    TestHelper::run(|h| async move {
        h.test_app
            .sync_events_until("Backers and disbursements are created", || async {
                h.test_app.app.backer_disbursement().select().count().await.unwrap() > 0
                    && h.test_app.app.backer().select().count().await.unwrap() > 0
            })
            .await;

        let this = h.test_app.app.backer_disbursement().find(1).await.unwrap();
        assert_eq!(*this.asami_issuance_rate(), Decimal::new(4000, 0));
        assert!(*this.btc_claimed() > Decimal::new(0, 0));
        assert!(*this.btc_claimed() < Decimal::new(9, 2));
        assert!(*this.rif_claimed() > Decimal::new(0, 0));
        assert!(*this.rif_claimed() < Decimal::new(1000, 0));
        assert!(*this.btc_usd_rate() > Decimal::new(50_000, 0));
        assert!(*this.btc_usd_rate() < Decimal::new(500_000, 0));
        assert!(*this.rif_usd_rate() > Decimal::new(1, 2));
        assert!(*this.rif_usd_rate() < Decimal::new(10, 0));

        let backer_count = h.test_app.app.backer().select().count().await.unwrap();
        assert!(backer_count > 0);

        for _ in 0..3 {
            // It's idempotent
            h.test_app.app.backer().store_today_stakes().await.unwrap()
        }
        let stake_count = h.test_app.app.backer_stake().select().count().await.unwrap();
        assert_eq!(stake_count, backer_count);
    })
    .await;
}
