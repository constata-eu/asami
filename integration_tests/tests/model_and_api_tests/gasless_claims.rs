// This module tests how collabs are made for accounts and sub accounts.
use ::api::models::*;

app_test! { makes_gasless_claims_as_soon_as_possible(a)
    let campaign = a.quick_campaign(u("2000"), 30, &[]).await;

    // Alice and bob will collab and get some pending balance.
    let mut alice = a.client().await;
    alice.claim_account().await;
    let alice_x = alice.create_handle("alice_on_x", "11111", Site::X, wei("10000")).await;

    let mut bob = a.client().await;
    bob.claim_account().await;
    let bob_x = bob.create_handle("bob_on_x", "22222", Site::X, wei("10000")).await;
    
    // Carl won't have enough money for gasless withdrawal yet, so will be skipped.
    let mut carl = a.client().await;
    carl.claim_account().await;
    let carl_x = carl.create_handle("carl_on_x", "33333", Site::X, wei("50")).await;

    // Stranger will set himself up as his own trusted admin, so will be skipped.
    let mut stranger = a.client().await;
    stranger.claim_account().await;
    let stranger_x = stranger
        .create_handle("stranger_on_x", "55555", Site::X, wei("10000"))
        .await;

    // Susan has not claimed her account so no gasless claim is possible.
    let susan = a.client().await;
    let susan_x = susan.create_handle("susan_on_x", "44444", Site::X, wei("10000")).await;

    for user in [&alice_x, &bob_x, &carl_x, &stranger_x, &susan_x] {
        campaign.make_x_collab_with_user_id(user.user_id().as_ref().unwrap()).await?;
    }

    a.send_tx(
        "Stranger is his own admin",
        "71200",
        stranger.asami_contract().config_account(stranger.address(), u("6"), u("0"), u("0"))
    ).await;

    assert!(a.admin_rbtc_balance().await < wei("49999000000000000000"));
    assert!(a.admin_rbtc_balance().await > wei("49998000000000000000"));

    // This will run all other required jobs to get to the gasless claims part.
    let job = a.wait_for_job(
        "Claiming all balances",
        OnChainJobKind::GaslessClaimBalances,
        OnChainJobStatus::Settled
    ).await;

    a.assert_balances_of("Alice after claim", alice.address(),
        wei("10000006000000000000"),
        u("0"), wei("7999100000000000000"),
        u("0"), wei("1199880000000000000000")
    ).await;

    a.assert_balances_of("Bob after claim", bob.address(),
        wei("10000006000000000000"),
        u("0"), wei("7999100000000000000"),
        u("0"), wei("1199880000000000000000")
    ).await;

    // Not enough for claiming
    a.assert_balances_of("Carl after claim", carl.address(),
        u("10"), milli("45"), u("0"), milli("6000"), u("0")
    ).await;

    // We are not their trusted admin.
    a.assert_balances_of("Stranger after claim", stranger.address(),
        wei("9999930170000000000"),
        wei("8999100000000000000"), u("0"),
        wei("1199880000000000000000"), u("0"),
    ).await;

    // Is a sub account.
    a.assert_sub_account_balances_of("Susan after claim", susan.account_id(),
        wei("8999100000000000000"), wei("1199880000000000000000"),
    ).await;

    assert_eq!(job.on_chain_job_account_vec().await?.len(), 2);

    a.wait_for_job(
        "Claiming all balances again is not needed",
        OnChainJobKind::GaslessClaimBalances,
        OnChainJobStatus::Skipped
    ).await;

    assert!(a.admin_rbtc_balance().await < wei("49998000000000000000"));
    assert!(a.admin_rbtc_balance().await > wei("49997000000000000000"));
}


app_test! { makes_admin_claim(a)
    assert_eq!(a.admin_asami_balance().await, u("0"));
    let mut advertiser = a.client().await;
    advertiser.setup_as_advertiser_with_amount("test main advertiser", u("20000")).await;
    let campaign = advertiser.start_and_pay_campaign(
        "https://x.com/somebody/status/1716421161867710954",
        u("20000"), 30, &[]).await;

    let alice = a.quick_handle("alice_on_x", "11111", Site::X, wei("10000")).await;

    a.wait_for_job(
        "Claiming admin's own balances",
        OnChainJobKind::AdminClaimBalancesFree,
        OnChainJobStatus::Skipped
    ).await;

    campaign.make_collab(&alice, u("20000"), "unique_trigger").await?;

    a.wait_for_job(
        "Claiming admin's own balances",
        OnChainJobKind::AdminClaimBalancesFree,
        OnChainJobStatus::Settled
    ).await;

    assert_eq!(a.admin_unclaimed_asami_balance().await, u("0"));
    assert_eq!(a.admin_treasury_asami_balance().await, u("0"));
    assert_eq!(a.admin_asami_balance().await, u("3200000"));
}


/*
app_test! { test_claim_account_in_its_own_test(a)
    todo!("fail here");
}

app_test! { avoid_race_condition_detecting_problems_with_claiming_an_account 
}
*/