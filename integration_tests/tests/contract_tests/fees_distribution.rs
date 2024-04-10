use ::api::on_chain::*;

app_test!{ distributes_fees_to_holders (a) 
  a.send_revert_tx(
    "no tokens, so nothing to share",
    "cfp0",
    a.asami_core().claim_fee_pool_share(vec![a.admin_treasury_address().await])
  ).await;

  /* Cycle 1:
   * Issues tokens to admin only. Pending tokens do not get paid.
   * Only admin gets paid as nobody else claimed their account.
   */
  assert_eq!(a.contract_doc_balance().await,   wei("0"));
  assert_eq!(a.admin_doc_balance().await,      u("420000000"));

  let mut advertiser = a.client().await;
  advertiser.setup_as_advertiser_with_amount("global advertiser for test", u("100000")).await;
  advertiser.make_campaign("first campaign", u("1000"), u("1"), 2).await;

  assert_eq!(a.contract_doc_balance().await,                u("1000"));
  assert_eq!(a.admin_doc_balance().await,                   u("419900000"));
  assert_eq!(advertiser.doc_balance().await,                u("99000"));

  let mut bob = a.client().await;
  bob.make_client_wallet().await;
  a.send_make_collab_tx("Bob collabs and gets tokens", "195997", &advertiser, u("1"), &bob, u("100")).await;

  a.send_tx(
    "Admin claims their ASAMI tokens too issuing all tokens", 
    "151402",
    a.asami_core().claim_admin_unclaimed_balances()
  ).await;

  // The total assigned tokens for the whole collaboration is less than what was issued.
  assert_eq!(a.asami_core().assigned_asami_tokens().call().await?,    wei("10000000000000000000"));
  assert_eq!(a.asami_core().total_supply().call().await?,             wei( "7500000000000000000"));
  assert_eq!(a.admin_asami_balance().await,                  wei( "7500000000000000000"));

  a.send_revert_tx(
    "no distribution in this cycle yet",
    "cfp0",
    a.asami_core().claim_fee_pool_share(vec![a.admin_treasury_address().await, bob.address() ])
  ).await;

  assert_eq!(a.contract_doc_balance().await,                       u("1000"));
  assert_eq!(a.admin_doc_balance().await,                     u("419900000"));
  assert_eq!(a.admin_treasury_doc_balance().await,          u("99160000000"));
  assert_eq!(a.asami_core().fee_pool().call().await?,                u("10")); 

  /* Cycle 2:
   *  - Campaign leftover funds get reimbursed to admin.
   *  - No feePool to distribute. Nothing will be distributed until next cycle.
   *  - A new campaign is created funded by the admin.
   *  - Users claim their accounts so they get their pending docs and asami tokens.
   */

  a.evm_forward_to_next_cycle().await;
  a.send_tx(
    "admin had all the tokens, gets the whole pool",
    "102095",
    a.asami_core().claim_fee_pool_share(vec![a.admin_address().await])
  ).await;
  assert_eq!(a.admin_doc_balance().await,                     u("419900010"));
  assert_eq!(a.admin_treasury_doc_balance().await,          u("99160000000"));
  assert_eq!(a.asami_core().fee_pool().call().await?,                 u("0")); 

  advertiser.make_campaign("second campaign", u("1000"), u("2"), 15 + 2).await;
  assert_eq!(a.contract_doc_balance().await,                       u("1990"));

  let mut alice = a.client().await;
  alice.make_client_wallet().await;
  a.send_make_collab_tx("Alice collabs and gets tokens", "195937", &advertiser, u("2"), &alice, u("200")).await;

  a.send_tx(
    "claiming bob, alice accounts",
    "229761",
    a.asami_core().claim_accounts(vec![
      ClaimAccountsParam{ account_id: alice.account_id(), addr: alice.address() },
      ClaimAccountsParam{ account_id: bob.account_id(), addr: bob.address() },
    ])
  ).await;

  a.send_tx(
    "Balance claims for all accounts result in token issuances",
    "318768",
    a.asami_core().claim_balances( vec![alice.account_id(), bob.account_id(), advertiser.account_id()]),
  ).await; 
  a.send_tx("Admin claims a second time", "151402", a.asami_core().claim_admin_unclaimed_balances()).await;

  // Contract paid 180 to alice for this collab. 90 to bob for the previous one when he claimed his account.
  assert_eq!(a.admin_doc_balance().await,      u("419900010"));
  assert_eq!(a.contract_doc_balance().await,        u("1720"));
  assert_eq!(bob.doc_balance().await,          u(       "90"));
  assert_eq!(alice.doc_balance().await,        u(      "180"));
  assert_eq!(advertiser.doc_balance().await,   u(    "98000"));
  assert_eq!(a.asami_core().fee_pool().call().await?,       u("20")); 

  // Asami tokens are now distributed as such:
  assert_eq!(a.asami_core().assigned_asami_tokens().call().await?,  wei("30000000000000000000"));
  assert_eq!(a.asami_core().total_supply().call().await?,           wei("30000000000000000000"));
  assert_eq!(a.admin_asami_balance().await,                         wei("22500000000000000000")); // 75%
  assert_eq!(bob.asami_balance().await,                             wei( "1500000000000000000")); // 5%
  assert_eq!(alice.asami_balance().await,                           wei( "3000000000000000000")); // 10%
  assert_eq!(advertiser.asami_balance().await,                      wei( "3000000000000000000")); // 10%

  a.send_revert_tx(
    "alice and bob just got their tokens, no sharing for them ",
    "cfp3",
    a.asami_core().claim_fee_pool_share(vec![alice.address(), bob.address() ])
  ).await;

  /* Cycle 3: 
   * Everyone gets paid from collabs on the previous period.
   * The admin sends tokens to other addresses to hold.
   * The previous campaign should have been reimbursed.
   */
  a.evm_forward_to_next_cycle().await;

  assert_eq!(a.asami_core().fee_pool().call().await?,        u("20")); 

  a.send_tx(
    "Claiming pool fees for everyone else",
    "203195",
    a.asami_core().claim_fee_pool_share(vec![a.admin_address().await, alice.address(), advertiser.address(),  bob.address() ])
  ).await;
  assert_eq!(a.asami_core().fee_pool().call().await?,       u("0")); 

  assert_eq!(a.admin_doc_balance().await,      wei("419900025000000000000000000")); 
  assert_eq!(a.contract_doc_balance().await,   wei(     "1700000000000000000000"));
  assert_eq!(bob.doc_balance().await,          wei(       "91000000000000000000"));
  assert_eq!(alice.doc_balance().await,        wei(      "182000000000000000000"));
  assert_eq!(advertiser.doc_balance().await,   wei(    "98002000000000000000000"));
  let mut unknown_holders = vec![];
  for _ in 0..102 {
    let holder = a.make_random_local_wallet().address();
    a.send_asami_to(holder, wei("100000000000005227")).await;
    unknown_holders.push(holder);
  }

  assert_eq!(a.asami_core().get_fee_pool_before_recent_changes().call().await?, u("20"));

  a.send_revert_tx(
    "advertiser cannot try to claim again",
    "cfp2",
    a.asami_core().claim_fee_pool_share(vec![advertiser.address()])
  ).await;

  a.send_revert_tx(
    "new unknown holders cannot claim yet",
    "cfp3",
    a.asami_core().claim_fee_pool_share(unknown_holders.clone())
  ).await;

  for h in &unknown_holders {
    assert_eq!(a.asami_balance_of(h).await, wei( "100000000000005227"));
    assert_eq!(a.doc_balance_of(h).await,   wei(                  "0"));
  }

  /* Cycle 4:
   * New participants join and collaborate. 
   * No fee pool is shared, as there were no collabs on previous period.
   */
  a.evm_forward_to_next_cycle().await;

  advertiser.make_campaign("third campaign", u("1000"), u("3"), 45 + 2).await;

  let mut susan = a.client().await;
  susan.make_client_wallet().await;
  a.send_make_collab_tx("susan collabs and gets tokens", "195937", &advertiser, u("3"), &susan, u("300")).await;
  a.send_tx(
    "claiming susan",
    "229761",
    a.asami_core().claim_accounts(vec![
      ClaimAccountsParam{ account_id: susan.account_id(), addr: susan.address() },
    ])
  ).await;

  a.send_tx(
    "Also claim susan's and advertiser balance",
    "172982",
    a.asami_core().claim_balances( vec![advertiser.account_id(), susan.account_id()])
  ).await; 

  a.send_tx(
    "Admin claims their ASAMI tokens too issuing all tokens", 
    "151402",
    a.asami_core().claim_admin_unclaimed_balances()
  ).await;

  assert_eq!(a.asami_core().fee_pool().call().await?,        u("30")); 
  assert_eq!(a.asami_core().get_fee_pool_before_recent_changes().call().await?, u("0"));

  let mut holders = unknown_holders.clone();
  holders.extend_from_slice(&[a.admin_address().await, alice.address(), advertiser.address(), bob.address(), susan.address()]);

  a.send_revert_tx("No holder gets paid as previous cycle had no collabs.", "cfp1", a.asami_core().claim_fee_pool_share(holders.clone())).await;
  for h in &unknown_holders {
    assert_eq!(a.asami_balance_of(h).await, wei( "100000000000005227"));
    assert_eq!(a.doc_balance_of(h).await,   wei(                  "0"));
  }

  /* Cycle 5:
   * All hodlers got paid from the previous cycle collab.
   */
  a.evm_forward_to_next_cycle().await;
  a.send_tx("everyone gets paid the 30 doc from last period", "5959059", a.asami_core().claim_fee_pool_share(holders.clone())).await;

  assert_eq!(a.asami_core().assigned_asami_tokens().call().await?,  wei("60000000000000000000"));
  assert_eq!(a.asami_core().total_supply().call().await?,           wei("60000000000000000000"));
  assert_eq!(a.admin_asami_balance().await,                         wei("34799999999999466846")); // 57.99999999999911141%
  assert_eq!(bob.asami_balance().await,                             wei( "1500000000000000000")); //  2.5%
  assert_eq!(alice.asami_balance().await,                           wei( "3000000000000000000")); //  5.0%
  assert_eq!(susan.asami_balance().await,                           wei( "4500000000000000000")); //  7.5%
  assert_eq!(advertiser.asami_balance().await,                      wei( "6000000000000000000")); // 10.0%
  
  for h in &unknown_holders { //                                                                     17.0000000000008~%
    assert_eq!(a.asami_balance_of(h).await, wei( "100000000000005227"));
    assert_eq!(a.doc_balance_of(h).await,   wei(  "50000000000002613"));
  }

  // Fee pool should have been 30 DOC now distributed here.
  assert_eq!(a.admin_doc_balance().await,                wei("419900042399999999999733423")); 
  assert_eq!(bob.doc_balance().await,                    wei(       "91750000000000000000"));
  assert_eq!(alice.doc_balance().await,                  wei(      "183500000000000000000"));
  assert_eq!(advertiser.doc_balance().await,             wei(    "97005000000000000000000")); 
  assert_eq!(susan.doc_balance().await,                  wei(      "272250000000000000000"));
  assert_eq!(a.contract_doc_balance().await,             wei(     "2400000000000000000051"));
  assert_eq!(a.asami_core().fee_pool().call().await?,    wei(                         "51"));
}

/*
 *
 * Making campaigns test:
 *   - Do not allow emtpy campaigns.
 *   - Allow extending campaigns.
 *   - Do not allow new campaigns to override old campaigns.
 */
