use ::api::models::*;

app_test! { claims_fee_pool_share_as_soon_as_possible(a)
    /* Calls and settles, then it is skipped, then next month it calls it again */
    let campaign = a.quick_campaign(u("2000"), 60, &[]).await;
    let handle = a.quick_handle("alice_on_x", "11111", Site::X, wei("10000")).await;
    campaign.make_collab(&handle, u("100"), "first_collab").await?;

    a.wait_for_job(
        "Collabs should be made",
        OnChainJobKind::GaslessClaimBalances,
        OnChainJobStatus::Settled
    ).await;

    a.evm_forward_to_next_cycle().await;

    assert_eq!(a.asami_contract().fee_pool().call().await?, u("10"));
    a.wait_for_job(
        "Fee pool share has not been claimed",
        OnChainJobKind::ClaimFeePoolShare,
        OnChainJobStatus::Settled
    ).await;
    assert_eq!(a.asami_contract().fee_pool().call().await?, u("0"));

    campaign.make_collab(&handle, u("50"), "second_collab").await?;

    a.wait_for_job(
        "No more shares in same cycle",
        OnChainJobKind::ClaimFeePoolShare,
        OnChainJobStatus::Skipped
    ).await;
    assert_eq!(a.asami_contract().fee_pool().call().await?, u("5"));

    a.evm_forward_to_next_cycle().await;

    a.wait_for_job(
        "Claim is possible again",
        OnChainJobKind::ClaimFeePoolShare,
        OnChainJobStatus::Settled
    ).await;
    assert_eq!(a.asami_contract().fee_pool().call().await?, u("0"));
}
