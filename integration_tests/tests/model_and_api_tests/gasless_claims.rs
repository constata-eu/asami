use models::{OnChainJobKind, OnChainJobStatus};

// This module tests how collabs are made for accounts and sub accounts.
use super::*;

#[tokio::test]
#[serial_test::file_serial]
async fn makes_gasless_claims_as_soon_as_possible() {
    TestHelper::run(|h| async move {
        let advertiser = Box::pin(h.advertiser_with_params(u("1000000"), u("1000000"))).await;
        let campaign = Box::pin(advertiser.start_and_pay_campaign_with_params(
            "https://x.com/somebody/status/1716421161867710954",
            u("200000"),
            30,
            &[],
            milli("9999"),
            milli("45"),
            milli("10"),
            false,
        ))
        .await;

        // Alice and bob will collab and get some pending balance.
        let mut alice = h.collaborator(1250).await;
        alice.claim_account().await;

        let mut bob = h.collaborator(1250).await;
        bob.claim_account().await;

        // Carl won't have enough money for gasless withdrawal yet, so will be skipped.
        let mut carl = h.collaborator(10).await;
        carl.claim_account().await;

        // Stranger will set himself up as his own trusted admin, so will be skipped.
        let mut stranger = h.collaborator(1250).await;
        stranger.claim_account().await;

        // Susan has not claimed her account so no gasless claim is possible.
        let susan = h.collaborator(1250).await;

        // Eve has not allowed gasless claims so we honour it.
        let mut eve = h.collaborator(1250).await;
        eve.claim_account().await;

        // Disallow gasless for everyone to make sure we don't
        // process them as a side effect.
        for client in [&alice, &bob, &carl, &stranger, &susan, &eve] {
            client.account().await.disallow_gasless().await.unwrap();
        }

        h.a().batch_collabs(campaign, &[&alice, &bob, &carl, &stranger, &susan, &eve]).await;

        for client in [&alice, &bob, &carl, &stranger, &susan] {
            client.account().await.allow_gasless().await.unwrap();
        }

        h.a()
            .send_tx(
                "Stranger is his own admin",
                "71200",
                stranger.asami_contract().config_account(stranger.address(), u("6"), u("0"), u("0")),
            )
            .await;

        assert!(h.a().admin_rbtc_balance().await < wei("999929000000000000000"));
        assert!(h.a().admin_rbtc_balance().await > wei("999928000000000000000"));

        // This will run all other required jobs to get to the gasless claims part.
        let job = h
            .a()
            .wait_for_job(
                "Claiming all balances",
                OnChainJobKind::GaslessClaimBalances,
                OnChainJobStatus::Settled,
            )
            .await;

        h.a()
            .assert_balances_of(
                "Alice after claim",
                alice.address(),
                wei("10006000000000000"),
                u("0"),
                wei("7999100000000000000"),
                u("0"),
                wei("1199880000000000000000"),
            )
            .await;

        h.a()
            .assert_balances_of(
                "Bob after claim",
                bob.address(),
                wei("10006000000000000"),
                u("0"),
                wei("7999100000000000000"),
                u("0"),
                wei("1199880000000000000000"),
            )
            .await;

        // Not enough for claiming
        h.a()
            .assert_balances_of(
                "Carl after claim",
                carl.address(),
                milli("10"),
                milli("72"),
                u("0"),
                milli("9600"),
                u("0"),
            )
            .await;

        // We are not their trusted admin.
        h.a()
            .assert_balances_of(
                "Stranger after claim",
                stranger.address(),
                wei("9999965096999755679"),
                wei("8999100000000000000"),
                u("0"),
                wei("1199880000000000000000"),
                u("0"),
            )
            .await;

        // Is a sub account.
        h.a()
            .assert_sub_account_balances_of(
                "Susan after claim",
                susan.account_id(),
                wei("8999100000000000000"),
                wei("1199880000000000000000"),
            )
            .await;

        h.a()
            .assert_balances_of(
                "Eve should have not claimed",
                eve.address(),
                wei("10"),
                wei("8999100000000000000"),
                u("0"),
                wei("1199880000000000000000"),
                u("0"),
            )
            .await;

        assert_eq!(job.on_chain_job_account_vec().await.unwrap().len(), 2);

        h.a()
            .wait_for_job(
                "Claiming all balances again is not needed",
                OnChainJobKind::GaslessClaimBalances,
                OnChainJobStatus::Skipped,
            )
            .await;

        let balance = h.a().admin_rbtc_balance().await;
        assert!(balance < wei("39999000000000000000"));
        assert!(balance > wei("39998000000000000000"));
    })
    .await;
}

/// 0xd9FCAe4315920387f00725C78285D6D41C30b967
#[tokio::test]
#[serial_test::file_serial]
async fn admin_claims_their_own_balances() {
    TestHelper::run(|h| async move {
        let advertiser = h.advertiser().await;
        let campaign = Box::pin(advertiser.start_and_pay_campaign_with_params(
            "https://x.com/somebody/status/1716421161867710954",
            u("200000"),
            30,
            &[],
            u("200000"),
            milli("0"),
            u("1"),
            false,
        ))
        .await;

        assert_eq!(h.a().admin_asami_balance().await, u("0"));

        let alice = h.collaborator(3000).await;

        h.a()
            .wait_for_job(
                "Claiming admin's own balances",
                OnChainJobKind::AdminClaimBalancesFree,
                OnChainJobStatus::Skipped,
            )
            .await;

        h.a().batch_collabs(campaign, &[&alice]).await;

        h.a()
            .wait_for_job(
                "Claiming admin's own balances",
                OnChainJobKind::AdminClaimBalancesFree,
                OnChainJobStatus::Settled,
            )
            .await;

        assert_eq!(h.a().admin_unclaimed_asami_balance().await, u("0"));
        assert_eq!(h.a().admin_treasury_asami_balance().await, u("0"));
        assert_eq!(h.a().admin_asami_balance().await, u("384000"));
    })
    .await;
}
