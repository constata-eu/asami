use super::*;
use models::{weihex, OnChainJobKind::*, OnChainJobStatus::*};

#[tokio::test]
#[serial_test::file_serial]
async fn distributes_fees_to_holders() {
    let h = TestHelper::for_model().await;
    let a = &h.app;

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

    let mut advertiser = Box::pin(h.user()).await;
    Box::pin(advertiser.setup_as_advertiser("Setting up advertiser")).await;

    let mut campaign = Box::pin(advertiser.start_and_pay_campaign(
        "https://x.com/somebody/status/1716421161867710954",
        u("100000"),
        300,
        &[],
    ))
    .await;

    assert_eq!(Box::pin(a.contract_doc_balance()).await, u("100000"));
    assert_eq!(Box::pin(a.admin_doc_balance()).await, u("418000000"));
    assert_eq!(Box::pin(advertiser.doc_balance()).await, u("1900000"));

    let bob = h.user().await;
    let bob_handle = bob.create_handle("bob_on_x", "123456", u("100")).await;
    a.register_collab(
        "first bob collab",
        &mut campaign,
        &bob_handle,
        u("1000"),
        "bob_collab_1",
    )
    .await;

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
    assert_eq!(a.admin_doc_balance().await, u("418000000"));

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

    // New campaign and collaborations will populate the fee pool
    campaign = advertiser
        .start_and_pay_campaign(
            "https://x.com/somebody/status/1716421161867711111",
            u("100000"),
            300,
            &[],
        )
        .await;

    a.register_collab(
        "second bob collab",
        &mut campaign,
        &bob_handle,
        u("1000"),
        "bob_collab_2",
    )
    .await;

    // Third Cycle
    a.evm_forward_to_next_cycle().await;
    assert_eq!(
        a.asami_contract().get_fee_pool_before_recent_changes().call().await.unwrap(),
        u("100")
    );
    a.wait_for_job("All holders get paid now", ClaimFeePoolShare, Settled).await;

    assert_eq!(a.app.estimated_fee_pool_claim().select().count().await.unwrap(), 42);

    for h in &holders {
        assert_eq!(a.asami_balance_of(h).await, u("6000"));
        assert_eq!(a.doc_balance_of(h).await, milli("7500"));
        let holder = a.app.holder().select().address_eq(h.encode_hex()).one().await.unwrap();
        assert_eq!(*holder.balance(), u("6000").encode_hex());
        assert_eq!(*holder.last_auto_paid_cycle(), weihex("2"));
    }

    assert_eq!(a.admin_asami_balance().await, u("40000"));
    assert_eq!(a.admin_doc_balance().await, u("418000050"));
}
