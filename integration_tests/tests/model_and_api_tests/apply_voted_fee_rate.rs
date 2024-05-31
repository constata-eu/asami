use ::api::models::*;

app_test! { applies_voted_fee_rate_as_soon_as_possible(a)
    /* Calls and settles, then it is skipped, then next month it calls it again */
    let campaign = a.quick_campaign(u("100"), 30, &[]).await;

    let mut alice = a.client().await;
    alice.claim_account().await;
    let handle = alice.create_handle("alice_on_x", "11111", Site::X, wei("100")).await;
    campaign.make_collab(&handle, u("100"), "first_collab").await?;

    a.wait_for_job(
        "Collabs should be made",
        OnChainJobKind::GaslessClaimBalances,
        OnChainJobStatus::Settled
    ).await;

    assert_eq!(alice.asami_balance().await, u("12000"));


    a.send_tx(
        "alice votes for 20",
        "96000",
        alice.asami_contract().submit_fee_rate_vote(u("20"))
    ).await;

    a.evm_forward_to_next_cycle().await;

    assert_eq!(a.asami_contract().fee_rate().call().await?, u("10"));
    a.wait_for_job(
        "Fee rate should be applied",
        OnChainJobKind::ApplyVotedFeeRate,
        OnChainJobStatus::Settled
    ).await;
    assert_eq!(a.asami_contract().fee_rate().call().await?, u("20"));

    a.wait_for_job(
        "Fee rate should be applied",
        OnChainJobKind::ApplyVotedFeeRate,
        OnChainJobStatus::Skipped
    ).await;

    a.evm_forward_to_next_cycle().await;

    a.send_tx(
        "alice votes for 30",
        "96000",
        alice.asami_contract().submit_fee_rate_vote(u("30"))
    ).await;

    a.wait_for_job(
        "Fee rate should be applied",
        OnChainJobKind::ApplyVotedFeeRate,
        OnChainJobStatus::Skipped
    ).await;
    assert_eq!(a.asami_contract().fee_rate().call().await?, u("30"));

}

/*
app_test! { test_claim_account_in_its_own_test(a)
    todo!("fail here");
}

app_test! { avoid_race_condition_detecting_problems_with_claiming_an_account 
}
*/
