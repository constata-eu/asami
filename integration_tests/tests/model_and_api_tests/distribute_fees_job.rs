use super::*;
use models::{weihex, OnChainJobKind::*, OnChainJobStatus::*};

#[tokio::test]
#[serial_test::file_serial]
async fn distributes_fees_to_holders() {
    TestHelper::run(|h| async move {
        let a = &h.a();

        a.wait_for_job("initial wait", ClaimFeePoolShare, Skipped).await;

        // No asami tokens, so nothing to share
        let err = a
            .asami_contract()
            .claim_fee_pool_share(vec![a.asami_contract().client().address()])
            .send()
            .await
            .unwrap_err();
        assert_eq!(err.decode_revert::<String>().unwrap(), "cfp0");

        assert_eq!(a.contract_doc_balance().await, wei("0"));
        assert_eq!(a.admin_doc_balance().await, u("420000000"));

        let advertiser = Box::pin(h.advertiser_with_params(u("1000000"), u("1000000"))).await;

        let mut campaign = Box::pin(advertiser.start_and_pay_campaign_with_params(
            "https://x.com/somebody/status/1716421161867710954",
            u("100000"),
            300,
            &[],
            u("1000"),
            u("1000"),
            milli("1"),
            false,
        ))
        .await;

        assert_eq!(Box::pin(a.contract_doc_balance()).await, u("100000"));
        assert_eq!(Box::pin(a.admin_doc_balance()).await, u("419000000"));
        assert_eq!(Box::pin(advertiser.doc_balance()).await, u("900000"));

        let bob = h.collaborator(10000).await;
        h.a().batch_collabs(campaign, &[&bob]).await;

        // The total claimed tokens for the whole collaboration is less than what was issued.
        assert_eq!(
            a.asami_contract().assigned_asami_tokens().call().await.unwrap(),
            u("400000")
        );

        a.send_tx(
            "Admin claims pending balance",
            "151402",
            a.asami_contract().claim_balances(),
        )
        .await;

        assert_eq!(a.asami_contract().total_supply().call().await.unwrap(), u("160000"));
        assert_eq!(a.admin_asami_balance().await, u("160000"));

        assert_eq!(a.contract_doc_balance().await, u("100000"));
        assert_eq!(a.admin_doc_balance().await, u("419000000"));

        let mut holders = vec![];
        for _ in 0..20 {
            let holder = a.make_wallet().await.address();
            a.send_asami_to(holder, u("6000")).await;
            holders.push(holder);
        }

        assert_eq!(a.admin_asami_balance().await, u("40000"));

        a.sync_events_until("we have 21 holders", || async {
            a.app.holder().select().count().await.unwrap() == 21
        })
        .await;

        for h in &holders {
            assert_eq!(a.asami_balance_of(h).await, u("6000"));
            assert_eq!(a.doc_balance_of(h).await, wei("0"));
        }

        // Second Cycle: The fee pool has leftover funds so it can pay. Otherwise it wouldn't.
        a.evm_forward_to_next_cycle().await;
        assert_eq!(
            a.asami_contract().get_fee_pool_before_recent_changes().call().await.unwrap(),
            u("100")
        );
        assert_eq!(a.asami_contract().total_supply().call().await.unwrap(), u("160000"));
        a.wait_for_job("Nobody gets paid yet, but tokens vested", ClaimFeePoolShare, Settled).await;
        for h in &holders {
            assert_eq!(a.asami_balance_of(h).await, u("6000"));
            assert_eq!(a.doc_balance_of(h).await, milli("3750"));
        }

        campaign = Box::pin(advertiser.start_and_pay_campaign_with_params(
            "https://x.com/somebody/status/1716421161867711111",
            u("100000"),
            300,
            &[],
            u("1000"),
            u("1000"),
            milli("1"),
            false,
        ))
        .await;
        h.a().batch_collabs(campaign, &[&bob]).await;

        // Third Cycle
        a.evm_forward_to_next_cycle().await;
        assert_eq!(
            a.asami_contract().get_fee_pool_before_recent_changes().call().await.unwrap(),
            u("100")
        );

        assert_eq!(
            a.asami_contract().assigned_asami_tokens().call().await.unwrap(),
            u("500000")
        );

        a.wait_for_job("All holders get paid now", ClaimFeePoolShare, Settled).await;

        for h in &holders {
            assert_eq!(a.asami_balance_of(h).await, u("6000"));
            assert_eq!(a.doc_balance_of(h).await, milli("7500"));
            let holder = a.app.holder().select().address_eq(h.encode_hex()).one().await.unwrap();
            assert_eq!(*holder.balance(), u("6000").encode_hex());
            assert_eq!(*holder.last_auto_paid_cycle(), weihex("2"));
        }

        assert_eq!(a.admin_asami_balance().await, u("40000"));
        assert_eq!(a.admin_doc_balance().await, u("419000050"));
    })
    .await;
}
