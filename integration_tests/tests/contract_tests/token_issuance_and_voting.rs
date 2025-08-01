use ::api::models::{u, U256};

/// This module tests the token issuance algorithm and targetting,
/// and also the way tokens can be used to vote the base fee rate.
use super::*;

#[tokio::test]
#[serial_test::file_serial]
async fn has_an_adaptable_schedule_with_a_target_per_cycle() {
    TestHelper::run(|h| async move {
        let a = h.a();
        let budget = u("42000000");
        let campaign = u("deadbeef");
        let advertiser = h.advertiser_with_params(budget, budget).await;

        // We'll use a really long campaign for this: 3650 days.
        advertiser.pay_campaign("global campaign for test", budget, campaign, 3650).await;
        let alice = h.collaborator(200).await;

        let do_collab =
            |context, amount| a.send_make_collab_tx(context, "230000", &advertiser, campaign, &alice, amount);

        do_collab("First one", u("10")).await;
        assert_assigned_and_rate(a, "4000", u("4000")).await;

        // As last period only saw 1 doc collected as fee, this cycle adapts to issue 100k per doc.
        a.evm_forward_to_next_cycle().await;
        do_collab("Second one", u("100")).await;
        assert_assigned_and_rate(a, "1004000", u("100000")).await;

        a.evm_forward_to_next_cycle().await;
        assert_assigned_and_rate(a, "1004000", u("10000")).await;

        // The last period saw no collaborations, so this period falls back to the harcoded x1000
        a.evm_forward_to_next_cycle().await;
        do_collab("Third one", u("9960")).await;
        assert_assigned_and_rate(a, "2000000", u("1000")).await;

        a.evm_forward_to_next_cycle().await;
        assert_assigned_and_rate(a, "2000000", wei("100401606425702811244")).await;

        a.evm_forward_to_next_cycle().await;
        do_collab("Fourth one", u("190000")).await;
        assert_assigned_and_rate(a, "21000000", u("1000")).await;

        a.evm_forward_to_next_cycle().await;
        do_collab("Fifth one", u("100")).await;
        assert_assigned_and_rate(a, "21000000", u("1000")).await;

        a.evm_forward_to_next_cycle().await;
        do_collab("Sixth one", u("100")).await;
        assert_assigned_and_rate(a, "21000000", u("1000")).await;
    })
    .await;
}

async fn assert_assigned_and_rate(a: &TestApp, assigned: &str, rate: U256) {
    assert_eq!(
        a.asami_contract().assigned_asami_tokens().call().await.unwrap(),
        u(assigned)
    );
    assert_eq!(a.asami_contract().get_issuance_for(u("1")).call().await.unwrap(), rate);
}

#[tokio::test]
#[serial_test::file_serial]
async fn rate_can_be_voted() {
    TestHelper::run(|h| async move {
        let a = h.a();
        assert_eq!(fee_rate(a).await, u("10"));
        a.send_revert_tx(
            "Cannot apply at cycle 0",
            "afr0",
            a.asami_contract().apply_voted_fee_rate(),
        )
        .await;

        let budget = u("3000");
        let advertiser = h.advertiser_with_params(budget, budget).await;
        advertiser.pay_campaign("global campaign for test", budget, u("deadbeef"), 2).await;
        let mut alice = h.user().await;
        alice.setup_trusted_admin("Alice's admin").await;

        a.send_make_collab_tx(
            "Alice collabs and gets tokens",
            "230000",
            &advertiser,
            u("deadbeef"),
            &alice,
            budget,
        )
        .await;
        a.send_tx(
            "Claiming balances issues tokens",
            "318768",
            a.asami_contract().admin_claim_balances_free(vec![alice.address(), advertiser.address()]),
        )
        .await;

        a.send_revert_tx(
            "re-apply in initial cycle",
            "afr0",
            a.asami_contract().apply_voted_fee_rate(),
        )
        .await;

        a.evm_forward_to_next_cycle().await;

        a.send_tx(
            "admin votes for 20 with 30 votes",
            "96000",
            advertiser.asami_contract().submit_fee_rate_vote(u("20")),
        )
        .await;
        assert_eq!(voted_fee_rate_vote_count(a).await, u("360000"));
        assert_eq!(voted_fee_rate(a).await, u("20"));
        assert_eq!(fee_rate(a).await, u("10"));

        a.send_tx(
            "first apply changes rate",
            "55000",
            a.asami_contract().apply_voted_fee_rate(),
        )
        .await;
        assert_eq!(fee_rate(a).await, u("20"));

        a.send_tx(
            "second vote",
            "78900",
            alice.asami_contract().submit_fee_rate_vote(u("1")),
        )
        .await;
        assert_eq!(voted_fee_rate_vote_count(a).await, u("720000"));
        assert_eq!(voted_fee_rate(a).await, milli("10500"));

        a.send_revert_tx(
            "re-apply in second cycle",
            "afr0",
            a.asami_contract().apply_voted_fee_rate(),
        )
        .await;
        assert_eq!(fee_rate(a).await, u("20"));

        a.evm_forward_to_next_cycle().await;

        a.send_tx(
            "third apply works on next cycle",
            "38000",
            a.asami_contract().apply_voted_fee_rate(),
        )
        .await;
        assert_eq!(fee_rate(a).await, milli("10500"));

        a.send_tx(
            "advertiser removes vote",
            "33928",
            advertiser.asami_contract().remove_fee_rate_vote(),
        )
        .await;
        assert_eq!(voted_fee_rate_vote_count(a).await, u("360000"));
        assert_eq!(voted_fee_rate(a).await, u("1"));

        a.send_tx(
            "alice updates vote",
            "42900",
            alice.asami_contract().submit_fee_rate_vote(u("50")),
        )
        .await;
        assert_eq!(voted_fee_rate_vote_count(a).await, u("360000"));
        assert_eq!(voted_fee_rate(a).await, u("50"));

        a.send_revert_tx(
            "cannot vote 0",
            "srv0",
            alice.asami_contract().submit_fee_rate_vote(u("0")),
        )
        .await;
        a.send_revert_tx(
            "cannot vote 100",
            "srv0",
            alice.asami_contract().submit_fee_rate_vote(u("100")),
        )
        .await;

        let bob = h.user().await;
        a.send_revert_tx(
            "cannot vote with no tokens",
            "srv1",
            bob.asami_contract().submit_fee_rate_vote(u("10")),
        )
        .await;

        // Will remove the fee rate vote when the user moves their tokens.
        // Voted rate goes back to factory settings when no vote is applied.
        a.send_tx(
            "Will remove votes when sending tokens",
            "111000",
            alice.asami_contract().transfer(bob.address(), u("2")),
        )
        .await;

        assert_eq!(voted_fee_rate_vote_count(a).await, u("0"));
        assert_eq!(voted_fee_rate(a).await, u("10"));
    })
    .await;
}

async fn fee_rate(a: &TestApp) -> U256 {
    a.asami_contract().fee_rate().call().await.unwrap()
}
async fn voted_fee_rate(a: &TestApp) -> U256 {
    a.asami_contract().voted_fee_rate().call().await.unwrap()
}
async fn voted_fee_rate_vote_count(a: &TestApp) -> U256 {
    a.asami_contract().voted_fee_rate_vote_count().call().await.unwrap()
}
