/**
This module tests the contract's fees distribution mechanism
with a simulation of fees to be distributed, focusing on the amounts
for each user and the smart contract validations themselves.
There's another test module in models which only tests how the admin process
interacts with these calls, and the client side validation it does before
hitting the contract with invalid data.
*/
use super::*;

app_test! { distributes_fees_to_holders (a)
  a.send_revert_tx(
    "no tokens, so nothing to share",
    "cfp0",
    a.asami_contract().claim_fee_pool_share(vec![a.admin_treasury_address().await])
  ).await;

  //Cycle 1:
  //Issues tokens to admin only. Pending tokens do not get paid.
  //Only admin gets paid as nobody else claimed their account.

  assert_eq!(a.contract_doc_balance().await,   wei("0"));
  assert_eq!(a.admin_doc_balance().await,      u("420000000"));

  let mut advertiser = a.client().await;
  advertiser.setup_as_advertiser_with_amount("global advertiser for test", u("100000")).await;
  advertiser.pay_campaign("first campaign", u("1000"), u("1"), 2).await;

  assert_eq!(a.contract_doc_balance().await,                u("1000"));
  assert_eq!(a.admin_doc_balance().await,                   u("419900000"));
  assert_eq!(advertiser.doc_balance().await,                u("99000"));

  let mut bob = a.client().await;
  bob.make_client_wallet().await;
  bob.setup_trusted_admin("setting up trusted admin for bob").await;
  a.send_make_sub_account_collab_tx("Bob collabs and gets tokens", "229400", &advertiser, u("1"), &bob, u("100")).await;

  a.send_tx(
    "Admin claims their ASAMI tokens too issuing all tokens",
    "151402",
    a.asami_contract().claim_balances()
  ).await;

  // The total assigned tokens for the whole collaboration is less than what was issued.
  assert_eq!(a.asami_contract().assigned_asami_tokens().call().await?,    u("40000"));
  assert_eq!(a.asami_contract().total_supply().call().await?,             u("16000"));
  assert_eq!(a.admin_asami_balance().await,                               u("16000"));

  a.send_revert_tx(
    "no distribution in this cycle yet",
    "cfp0",
    a.asami_contract().claim_fee_pool_share(vec![a.admin_treasury_address().await, bob.address() ])
  ).await;

  assert_eq!(a.contract_doc_balance().await,                       u("1000"));
  assert_eq!(a.admin_doc_balance().await,                     u("419900000"));
  assert_eq!(a.admin_treasury_doc_balance().await,          u("99160000000"));
  assert_eq!(a.asami_contract().fee_pool().call().await?,                u("10"));

  // Cycle 2:
  //  - Campaign leftover funds get reimbursed to admin.
  //  - No feePool to distribute. Nothing will be distributed until next cycle.
  //  - A new campaign is created funded by the admin.
  //  - Users claim their accounts so they get their pending docs and asami tokens.

  a.evm_forward_to_next_cycle().await;
  a.send_tx(
    "admin had all the tokens, gets the whole pool",
    "108000",
    a.asami_contract().claim_fee_pool_share(vec![a.client_admin_address()])
  ).await;
  assert_eq!(a.admin_doc_balance().await,                     u("419900010"));
  assert_eq!(a.admin_treasury_doc_balance().await,          u("99160000000"));
  assert_eq!(a.asami_contract().fee_pool().call().await?,                 u("0"));

  advertiser.pay_campaign("second campaign", u("1000"), u("2"), 15 + 2).await;
  assert_eq!(a.contract_doc_balance().await,                       u("1990"));

  let mut alice = a.client().await;
  alice.make_client_wallet().await;
  alice.setup_trusted_admin("setting up trusted admin for alice").await;
  a.send_make_sub_account_collab_tx("Alice collabs and gets tokens", "200000", &advertiser, u("2"), &alice, u("200")).await;

  // We want 100k tokens per period, last period fees were 10 DOC, so, 10k each.
  assert_eq!(a.asami_contract().get_fees_collected(wei("0")).call().await.unwrap(), u("10") );
  assert_eq!(a.asami_contract().get_issuance_for(u("1")).call().await.unwrap(), u("10000"));
  a.assert_sub_account_balances_of("alice tokens", alice.account_id(), u("180"), u("60000")).await;

  a.send_tx(
    "claiming bob, alice accounts",
    "229761",
    a.asami_contract().promote_sub_accounts(vec![
      PromoteSubAccountsParam{ id: alice.account_id(), addr: alice.address() },
      PromoteSubAccountsParam{ id: bob.account_id(), addr: bob.address() },
    ])
  ).await;

  a.send_tx(
    "Balance claims for all accounts result in token issuances",
    "318768",
    a.asami_contract().admin_claim_balances_free( vec![alice.address(), bob.address(), advertiser.address()]),
  ).await;
  a.send_tx("Admin claims a second time", "151402", a.asami_contract().claim_balances()).await;

  // Contract paid 180 to alice for this collab. 90 to bob for the previous one when he claimed his account.
  assert_eq!(a.admin_doc_balance().await,      u("419900010"));
  assert_eq!(a.contract_doc_balance().await,        u("1720"));
  assert_eq!(bob.doc_balance().await,          u(       "90"));
  assert_eq!(alice.doc_balance().await,        u(      "180"));
  assert_eq!(advertiser.doc_balance().await,   u(    "98000"));
  assert_eq!(a.asami_contract().fee_pool().call().await?,       u("20"));

  // Asami tokens are now distributed as such:
  assert_eq!(a.asami_contract().assigned_asami_tokens().call().await?,  u("240000"));
  assert_eq!(a.asami_contract().total_supply().call().await?,           u("240000"));
  assert_eq!(a.admin_asami_balance().await,                             u( "96000"));
  assert_eq!(bob.asami_balance().await,                                 u( "12000"));
  assert_eq!(alice.asami_balance().await,                               u( "60000"));
  assert_eq!(advertiser.asami_balance().await,                          u( "72000"));

  a.send_revert_tx(
    "alice and bob just got their tokens, no sharing for them ",
    "cfp3",
    a.asami_contract().claim_fee_pool_share(vec![alice.address(), bob.address() ])
  ).await;

  // Cycle 3:
  // Everyone gets paid from collabs on the previous period.
  // The admin sends tokens to other addresses to hold.
  // The previous campaign should have been reimbursed.

  a.evm_forward_to_next_cycle().await;

  assert_eq!(a.asami_contract().fee_pool().call().await?,        u("20"));

  a.send_tx(
    "Claiming pool fees for everyone else",
    "213300",
    a.asami_contract().claim_fee_pool_share(vec![a.client_admin_address(), alice.address(), advertiser.address(),  bob.address() ])
  ).await;
  assert_eq!(a.asami_contract().fee_pool().call().await?,       u("0"));

  assert_eq!(a.admin_doc_balance().await,      wei("419900018000000000000000000"));
  assert_eq!(a.contract_doc_balance().await,   wei(     "1700000000000000000000"));
  assert_eq!(bob.doc_balance().await,          wei(       "91000000000000000000"));
  assert_eq!(alice.doc_balance().await,        wei(      "185000000000000000000"));
  assert_eq!(advertiser.doc_balance().await,   wei(    "98006000000000000000000"));
  let mut unknown_holders = vec![];
  for _ in 0..102 {
    let holder = a.make_random_local_wallet().address();
    a.send_asami_to(holder, wei("100000000000005227")).await;
    unknown_holders.push(holder);
  }

  assert_eq!(a.asami_contract().get_fee_pool_before_recent_changes().call().await?, u("20"));

  a.send_revert_tx(
    "advertiser cannot try to claim again",
    "cfp2",
    a.asami_contract().claim_fee_pool_share(vec![advertiser.address()])
  ).await;

  a.send_revert_tx(
    "new unknown holders cannot claim yet",
    "cfp3",
    a.asami_contract().claim_fee_pool_share(unknown_holders.clone())
  ).await;

  for h in &unknown_holders {
    assert_eq!(a.asami_balance_of(h).await, wei( "100000000000005227"));
    assert_eq!(a.doc_balance_of(h).await,   wei(                  "0"));
  }

  // Cycle 4:
  // New participants join and collaborate.
  // No fee pool is shared, as there were no collabs on previous period.

  a.evm_forward_to_next_cycle().await;

  advertiser.pay_campaign("third campaign", u("1000"), u("3"), 45 + 2).await;

  let mut susan = a.client().await;
  susan.make_client_wallet().await;
  susan.setup_trusted_admin("setting up trusted admin for susan").await;
  a.send_make_sub_account_collab_tx("susan collabs and gets tokens", "216638", &advertiser, u("3"), &susan, u("300")).await;
  a.send_tx(
    "claiming susan",
    "229761",
    a.asami_contract().promote_sub_accounts(vec![
      PromoteSubAccountsParam{ id: susan.account_id(), addr: susan.address() },
    ])
  ).await;

  a.send_tx(
    "Also claim susan's and advertiser balance",
    "179982",
    a.asami_contract().admin_claim_balances_free( vec![advertiser.address(), susan.address()])
  ).await;

  a.send_tx(
    "Admin claims their ASAMI tokens too issuing all tokens",
    "151402",
    a.asami_contract().claim_balances()
  ).await;

  assert_eq!(a.asami_contract().fee_pool().call().await?,        u("30"));
  assert_eq!(a.asami_contract().get_fee_pool_before_recent_changes().call().await?, u("0"));

  let mut holders = unknown_holders.clone();
  holders.extend_from_slice(&[a.client_admin_address(), alice.address(), advertiser.address(), bob.address(), susan.address()]);

  a.send_revert_tx("No holder gets paid as previous cycle had no collabs.", "cfp1", a.asami_contract().claim_fee_pool_share(holders.clone())).await;
  for h in &unknown_holders {
    assert_eq!(a.asami_balance_of(h).await, wei( "100000000000005227"));
    assert_eq!(a.doc_balance_of(h).await,   wei(                  "0"));
  }

  // Cycle 5:
  // All hodlers got paid from the previous cycle collab.

  a.evm_forward_to_next_cycle().await;
  a.send_tx("everyone gets paid the 30 doc from last period", "6039459", a.asami_contract().claim_fee_pool_share(holders.clone())).await;

  assert_eq!(a.asami_contract().assigned_asami_tokens().call().await?,  u("270000"));
  assert_eq!(a.asami_contract().total_supply().call().await?,           u("270000"));
  assert_eq!(a.admin_asami_balance().await,                             wei("107989799999999999466846"));
  assert_eq!(bob.asami_balance().await,                                 u( "12000"));
  assert_eq!(alice.asami_balance().await,                               u( "60000"));
  assert_eq!(susan.asami_balance().await,                               u(  "9000"));
  assert_eq!(advertiser.asami_balance().await,                          u( "81000"));

  for h in &unknown_holders { //                                                                     17.0000000000008~%
    assert_eq!(a.asami_balance_of(h).await, wei( "100000000000005227"));
    assert_eq!(a.doc_balance_of(h).await,   wei(     "11111111111111"));
  }

  // Fee pool should have been 30 DOC now distributed here.
  assert_eq!(a.admin_doc_balance().await,                wei("419900029998866666666666607"));
  assert_eq!(bob.doc_balance().await,                    wei(       "92333333333333333333"));
  assert_eq!(alice.doc_balance().await,                  wei(      "191666666666666666666"));
  assert_eq!(advertiser.doc_balance().await,             wei(    "97015000000000000000000"));
  assert_eq!(susan.doc_balance().await,                  wei(      "271000000000000000000"));
  assert_eq!(a.contract_doc_balance().await,             wei(     "2400000000000000000072"));
  assert_eq!(a.asami_contract().fee_pool().call().await?,    wei(                     "72"));
}
