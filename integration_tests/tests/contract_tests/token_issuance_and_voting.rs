use ::api::on_chain::*;
use crate::support::TestApp;
use ::api::models::{U256};

app_test!{ has_a_cap_on_token_supply (a) 
  /*
   * A single very large campaign and collaboration issues all tokens, and no further tokens are issued
   * Unclaimed tokens are honored by the cap as if they had been issued
   */
  let budget = u("420000000");
  let mut advertiser = a.client().await;
  advertiser.setup_as_advertiser_with_amount("global advertiser for test", budget).await;
  advertiser.make_campaign("global campaignn for test", budget, u("deadbeef"), 2).await;
  let mut alice = a.client().await;
  alice.make_client_wallet().await;
  let mut bob = a.client().await;
  bob.make_client_wallet().await;

  a.send_tx(
    "claiming all accounts involved",
    "229761",
    a.asami_core().claim_accounts(vec![
      ClaimAccountsParam{ account_id: alice.account_id(), addr: alice.address() },
      ClaimAccountsParam{ account_id: bob.account_id(), addr: bob.address() },
    ])
  ).await;

  a.send_make_collab_tx("Alice takes up all asami tokens", "195937", &advertiser, u("deadbeef"), &alice, u("210000000")).await;
  a.send_make_collab_tx("Bob gets DOC but not ASAMI", "77327", &advertiser, u("deadbeef"), &bob, u("210000000")).await;

  assert_eq!(a.asami_core().assigned_asami_tokens().call().await?, u("21000000"), "All tokens assigned");
  assert_eq!(a.asami_core().total_supply().call().await?, u("0"), "But none issued to claims");

  a.assert_balances_of("Alice", alice.account_id(),  u("10"), u("189000000"), u("0"), u("3150000"), u("0")).await;
  a.assert_balances_of("Bob", bob.account_id(),     u("10"), u("189000000"), u("0"), u("0"),       u("0")).await;

  a.assert_admin_balances("Admin balance after collabs", u("0"), u("0"), u("99160000000"), u("15750000"), u("0"), u("0")).await;

  a.send_tx(
    "Balance claims for all accounts result in token issuance",
    "318768",
    a.asami_core().claim_balances( vec![alice.account_id(), advertiser.account_id()]),
  ).await; 

  assert_eq!(a.asami_core().assigned_asami_tokens().call().await?, u("21000000"), "All tokens assigned");
  assert_eq!(a.asami_core().total_supply().call().await?, u("5250000"), "But not all tokens issued");

  a.send_tx(
    "Admin claims their ASAMI tokens too issuing all tokens", 
    "151402",
    a.asami_core().claim_admin_unclaimed_balances()
  ).await;

  assert_eq!(a.asami_core().assigned_asami_tokens().call().await?, u("21000000"), "All tokens assigned");
  assert_eq!(a.asami_core().total_supply().call().await?, u("21000000"), "And all issued");
}

app_test!{ rate_can_be_voted (a) 
  assert_eq!(fee_rate(&a).await, u("10"));
  a.send_tx("Apply at first", "47897", a.asami_core().apply_voted_fee_rate()).await;

  let budget = u("3000");
  let mut advertiser = a.client().await;
  advertiser.setup_as_advertiser_with_amount("global advertiser for test", budget).await;
  advertiser.make_campaign("global campaign for test", budget, u("deadbeef"), 2).await;
  let mut alice = a.client().await;
  alice.make_client_wallet().await;

  a.send_tx(
    "claiming all accounts involved",
    "229761",
    a.asami_core().claim_accounts(vec![
      ClaimAccountsParam{ account_id: alice.account_id(), addr: alice.address() },
    ])
  ).await;

  a.send_make_collab_tx("Alice collabs and gets tokens", "195937", &advertiser, u("deadbeef"), &alice, budget).await;
  a.send_tx(
    "Claiming balances issues tokens",
    "318768",
    a.asami_core().claim_balances( vec![alice.account_id(), advertiser.account_id()]),
  ).await; 

  a.send_revert_tx("re-apply in initial cycle", "afr0", a.asami_core().apply_voted_fee_rate()).await;

  a.evm_forward_to_next_cycle().await;

  a.send_tx("admin votes for 20 with 30 votes", "96000", advertiser.asami_contract().submit_fee_rate_vote(u("20"))).await;
  assert_eq!(voted_fee_rate_vote_count(&a).await, u("30"));
  assert_eq!(voted_fee_rate(&a).await, u("20"));
  assert_eq!(fee_rate(&a).await, u("10"));

  a.send_tx("first apply changes rate", "47897", a.asami_core().apply_voted_fee_rate()).await;

  a.app.on_chain_tx().apply_voted_fee_rate().await.unwrap();
  assert_eq!(fee_rate(&a).await, u("20"));

  a.send_tx("second vote", "78900", alice.asami_contract().submit_fee_rate_vote(u("1"))).await;
  assert_eq!(voted_fee_rate_vote_count(&a).await, u("75"));
  assert_eq!(voted_fee_rate(&a).await, wei("8600000000000000000"));

  a.send_revert_tx("re-apply in second cycle", "afr0", a.asami_core().apply_voted_fee_rate()).await;
  assert_eq!(fee_rate(&a).await, u("20"));

  a.evm_forward_to_next_cycle().await;

  a.send_tx("third apply works on next cycle", "33597", a.asami_core().apply_voted_fee_rate()).await;
  assert_eq!(fee_rate(&a).await, wei("8600000000000000000"));

  a.send_tx("advertiser removes vote", "33928", advertiser.asami_contract().remove_fee_rate_vote()).await;
  assert_eq!(voted_fee_rate_vote_count(&a).await, u("45"));
  assert_eq!(voted_fee_rate(&a).await, u("1"));

  a.send_tx("alice updates vote", "42900", alice.asami_contract().submit_fee_rate_vote(u("50"))).await;
  assert_eq!(voted_fee_rate_vote_count(&a).await, u("45"));
  assert_eq!(voted_fee_rate(&a).await, u("50"));

  a.send_revert_tx("cannot vote 0", "srv0", alice.asami_contract().submit_fee_rate_vote(u("0"))).await;
  a.send_revert_tx("cannot vote 100", "srv0", alice.asami_contract().submit_fee_rate_vote(u("100"))).await;

  let mut bob = a.client().await;
  bob.make_client_wallet().await;
  a.send_revert_tx("cannot vote with no tokens", "srv1", bob.asami_contract().submit_fee_rate_vote(u("10"))).await;

  // Will remove the fee rate vote when the user moves their tokens.
  // Voted rate goes back to factory settings when no vote is applied.
  a.send_tx("Will remove votes when sending tokens", "107809",
    alice.asami_contract().transfer(bob.address(), u("2"))
  ).await;

  assert_eq!(voted_fee_rate_vote_count(&a).await, u("0"));
  assert_eq!(voted_fee_rate(&a).await, u("10"));
}

app_test!{ admin_can_be_voted_via_vested_votes (a)
  let og_admin_addr = a.admin_address().await;
  let og_admin_treasury_address = a.admin_treasury_address().await;

  let budget = u("3000");
  let mut advertiser = a.client().await;
  advertiser.setup_as_advertiser_with_amount("global advertiser for test", budget).await;
  advertiser.make_campaign("global campaignn for test", budget, u("deadbeef"), 2).await;
  let mut bob = a.client().await;
  bob.make_client_wallet().await;
  let mut stranger = a.client().await;
  stranger.make_client_wallet().await;

  a.send_make_collab_tx("Bob collabs and gets tokens", "195937", &advertiser, u("deadbeef"), &bob, budget).await;

  a.send_tx(
    "claiming bob's account",
    "229761",
    a.asami_core().claim_accounts(vec![
      ClaimAccountsParam{ account_id: bob.account_id(), addr: bob.address() },
    ])
  ).await;

  a.send_tx(
    "Balance claims for all accounts result in token issuance",
    "318768",
    a.asami_core().claim_balances( vec![bob.account_id(), advertiser.account_id()]),
  ).await; 

  let mut recent_admin_election = u("0");

  assert_eq!(last_admin_election(&a).await, recent_admin_election);
  assert_eq!(vested_admin_votes_total(&a).await, u("0"));
  assert_eq!(latest_admin_elections(&a).await, [Address::zero(), Address::zero(), Address::zero()]);

  a.send_revert_tx("No winner without votes", "pcw0", a.asami_core().proclaim_cycle_admin_winner(advertiser.address())).await;

  a.send_tx("advertiser starts by voting for himself", "95600",
    advertiser.asami_contract().submit_admin_vote(advertiser.address())
  ).await;
  a.send_revert_tx("first votes have not vested", "pcw0", a.asami_core().proclaim_cycle_admin_winner(advertiser.address())).await;

  a.evm_forward_to_next_cycle().await;

  a.send_tx("bob votes for himself", "95924", bob.asami_contract().submit_admin_vote(bob.address())).await;
  a.send_tx("advertiser votes vest", "165314", a.asami_core().vest_admin_votes(vec![advertiser.address()])).await;
  a.send_tx("advertiser claims victory", "79606", a.asami_core().proclaim_cycle_admin_winner(advertiser.address())).await;

  assert!(last_admin_election(&a).await > recent_admin_election);
  recent_admin_election = last_admin_election(&a).await;
  assert_eq!(vested_admin_votes_total(&a).await, u("30"));
  assert_eq!(latest_admin_elections(&a).await, [advertiser.address(), Address::zero(), Address::zero()]);

  a.evm_forward_to_next_cycle().await;

  a.send_tx("advertiser rushes to claim second victory before bob votes vest", "62644",
    a.asami_core().proclaim_cycle_admin_winner(advertiser.address())
  ).await;

  a.send_tx("bob votes vest just a bit late", "131114", a.asami_core().vest_admin_votes(vec![bob.address()])).await;
  a.send_revert_tx("bob's votes vested late for this cylce", "pcw3", a.asami_core().proclaim_cycle_admin_winner(bob.address())).await;

  assert!(last_admin_election(&a).await > recent_admin_election);
  recent_admin_election = last_admin_election(&a).await;
  assert_eq!(vested_admin_votes_total(&a).await, u("75"));
  assert_eq!(latest_admin_elections(&a).await, [advertiser.address(), advertiser.address(), Address::zero()]);

  a.evm_forward_to_next_cycle().await;

  a.send_revert_tx("advertiser cannot claim", "pcw2", a.asami_core().proclaim_cycle_admin_winner(advertiser.address())).await;
  a.send_tx("bob can claim", "65306", a.asami_core().proclaim_cycle_admin_winner(bob.address())).await;

  assert!(last_admin_election(&a).await > recent_admin_election);

  assert_eq!(latest_admin_elections(&a).await, [bob.address(), advertiser.address(), advertiser.address()]);

  assert_eq!(a.admin_address().await, og_admin_addr);
  assert_eq!(a.admin_treasury_address().await, og_admin_treasury_address);

  a.evm_forward_to_next_cycle().await;
  a.send_tx("bob claims second election", "45544", a.asami_core().proclaim_cycle_admin_winner(bob.address())).await;
  assert_eq!(latest_admin_elections(&a).await, [bob.address(), bob.address(), advertiser.address()]);

  a.evm_forward_to_next_cycle().await;
  a.send_tx("bob claims third and wins", "55637", a.asami_core().proclaim_cycle_admin_winner(bob.address())).await;
  assert_eq!(latest_admin_elections(&a).await, [bob.address(), bob.address(), bob.address()]);

  assert_eq!(a.admin_address().await, bob.address());
  assert_eq!(a.admin_treasury_address().await, bob.address());

  a.send_revert_tx(
    "advertiser can't set the admin address",
    "saa0",
    advertiser.asami_contract().set_admin_address(advertiser.address())
  ).await;
  a.send_tx("bob can set the admin address", "29072", bob.asami_contract().set_admin_address(stranger.address())).await;
  assert_eq!(a.admin_address().await, stranger.address());

  a.send_tx("sending tokens removes votes", "110594",
    advertiser.asami_contract().transfer(stranger.address(), u("2"))
  ).await;
  assert_eq!(vested_admin_votes_total(&a).await, u("45"));

  a.send_tx("bob removes his vote", "45392",
    bob.asami_contract().remove_admin_vote()
  ).await;
  assert_eq!(vested_admin_votes_total(&a).await, u("0"));

  a.evm_forward_to_next_cycle().await;
  a.send_revert_tx("no votes again, so no claims", "saa0", advertiser.asami_contract().set_admin_address(advertiser.address())).await;
  a.send_revert_tx("cannot vote for address 0", "sav0", advertiser.asami_contract().submit_admin_vote(Address::zero())).await;
}

async fn fee_rate(a: &TestApp) -> U256 {
  a.asami_core().fee_rate().call().await.unwrap()
}
async fn voted_fee_rate(a: &TestApp) -> U256 {
  a.asami_core().voted_fee_rate().call().await.unwrap()
}
async fn voted_fee_rate_vote_count(a: &TestApp) -> U256 {
  a.asami_core().voted_fee_rate_vote_count().call().await.unwrap()
}
async fn last_admin_election(a: &TestApp) -> U256 {
  a.asami_core().last_admin_election().call().await.unwrap()
}
async fn vested_admin_votes_total(a: &TestApp) -> U256 {
  a.asami_core().vested_admin_votes_total().call().await.unwrap()
}
async fn latest_admin_elections(a: &TestApp) -> [Address; 3] {
  a.asami_core().get_latest_admin_elections().call().await.unwrap()
}
