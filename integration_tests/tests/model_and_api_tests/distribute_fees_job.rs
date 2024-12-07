// This module tests how the scheduler interacts with fees distribution.
// The inner workings and amounts distributed by the smart contract are tested elsewhere.

/*
#[macro_use]
use ethers::signers::Signer;
app_test! { distributes_fees_to_holders (_a)
    todo!("Make assertions about fee cycles and fees at");
    todo!("Try to have someone collect twice");
}
app_test!{ distributes_fees_to_holders (a)
  a.run_idempotent_background_tasks_a_few_times().await;
  // No asami tokens, so nothing to share
  let err = a.contract().claim_fee_pool_share(vec![a.contract().client().address()]).send().await.unwrap_err();
  assert_eq!(err.decode_revert::<String>().unwrap(), "cfps0");

  //Cycle 1:
  //Issues tokens to admin only. Pending tokens do not get paid.
  //Only admin gets paid as nobody else claimed their account.

  assert_eq!(a.contract_doc_balance().await,   wei("0"));
  assert_eq!(a.admin_doc_balance().await,      u("420000000"));

  let mut advertiser = a.client().await;
  let mut campaign = advertiser.create_x_campaign(u("100000"), u("300")).await;

  assert_eq!(a.contract_doc_balance().await,    u("100000"));
  assert_eq!(a.admin_doc_balance().await,    u("419900000"));

  let mut bob = a.client().await;
  bob.create_x_handle("bob_on_x", u("100")).await;
  bob.create_x_collab(&campaign).await;

  // The total claimed tokens for the whole collaboration is less than what was issued.
  assert_eq!(a.contract().get_claimed_asami_tokens().call().await?, wei("10000000000000000000"));
  assert_eq!(a.contract().total_supply().call().await?,             wei( "7500000000000000000"));
  assert_eq!(a.admin_asami_balance().await,                         wei( "7500000000000000000"));

  // Fee distribution won't happen until next cycle, bob's reward is still in contract.
  a.run_idempotent_background_tasks_a_few_times().await;
  assert_eq!(a.contract_doc_balance().await,    u("100000"));
  assert_eq!(a.admin_doc_balance().await,    u("419900000"));

  // Cycle 2:
  //  - Campaign leftover funds get reimbursed to admin.
  //  - No feePool to distribute. Nothing will be distributed until next cycle.
  //  - A new campaign is created funded by the admin.
  //  - Users claim their accounts so they get their pending docs and asami tokens.
  //
  a.run_idempotent_background_tasks_a_few_times().await;
  a.evm_forward_to_next_cycle().await;
  a.run_idempotent_background_tasks_a_few_times().await;

  // This 90 doc is the pending DOC reward for Bob.
  // The rest went back to the admin, as revenue share for having 100% of the issued tokens, and campaign reimbursement.
  assert_eq!(a.contract_doc_balance().await,     u(   "90"));
  assert_eq!(a.admin_doc_balance().await,    u("419999910"));

  assert_eq!(a.contract().get_claimed_asami_tokens().call().await?, wei("10000000000000000000"));
  assert_eq!(a.contract().total_supply().call().await?,             wei( "7500000000000000000"));

  campaign = advertiser.create_x_campaign_extra(u("100000"), u("300"), 20, &[]).await;
  assert_eq!(a.contract_doc_balance().await,     u("100090"));
  assert_eq!(a.admin_doc_balance().await,     u("419899910"));

  let mut alice = a.client().await;
  alice.create_x_handle("alice_on_x", u("200")).await;
  alice.create_x_collab(&campaign).await;
  advertiser.claim_account().await;
  bob.claim_account().await;
  alice.claim_account().await;

  // Contract paid 180 to alice for this collab. 90 to bob for the previous one when he claimed his account.
  assert_eq!(a.admin_doc_balance().await,      u("419899910"));
  assert_eq!(a.contract_doc_balance().await,   u(    "99820"));
  assert_eq!(bob.doc_balance().await,          u(       "90"));
  assert_eq!(alice.doc_balance().await,        u(      "180"));
  assert_eq!(advertiser.doc_balance().await,   u(        "0"));

  // Asami tokens are now distributed as such:
  assert_eq!(a.contract().get_claimed_asami_tokens().call().await?, wei("30000000000000000000"));
  assert_eq!(a.contract().total_supply().call().await?,             wei("30000000000000000000"));
  assert_eq!(a.admin_asami_balance().await,                         wei("22500000000000000000")); // 75%
  assert_eq!(bob.asami_balance().await,                             wei( "1500000000000000000")); // 5%
  assert_eq!(alice.asami_balance().await,                           wei( "3000000000000000000")); // 10%
  assert_eq!(advertiser.asami_balance().await,                      wei( "3000000000000000000")); // 10%

  a.run_idempotent_background_tasks_a_few_times().await;
  // Bob cannot claim yet, tokens are too recent.
  let err = a.contract().claim_fee_pool_share(vec![bob.local_wallet().address()]).send().await.unwrap_err();
  assert_eq!(err.decode_revert::<String>().unwrap(), "cfps3");

  // Cycle 3:
  // Everyone gets paid from collabs on the previous period.
  // The admin sends tokens to other addresses to hold.
  // The previous campaign should have been reimbursed.
  //
  a.run_idempotent_background_tasks_a_few_times().await;
  a.evm_forward_to_next_cycle().await;
  a.run_idempotent_background_tasks_a_few_times().await;

  // Fee pool should have been 20 DOC.
  assert_eq!(a.admin_doc_balance().await,      wei("419999725000000000000000000"));
  assert_eq!(a.contract_doc_balance().await,   wei(                          "0"));
  assert_eq!(bob.doc_balance().await,          wei(       "91000000000000000000"));
  assert_eq!(alice.doc_balance().await,        wei(      "182000000000000000000"));
  assert_eq!(advertiser.doc_balance().await,   wei(        "2000000000000000000"));

  let mut holders = vec![];
  for _ in 0..102 {
    let holder = a.make_wallet().address();
    let amount = wei("100000000000005227");
    a.contract()
      .transfer(holder, amount)
      .send().await.unwrap().await.unwrap().unwrap();

    holders.push(holder);
  }

  assert_eq!(a.contract().get_fee_pool_before_recent_changes().call().await?, u("20"));

  // Advertiser cannot try to claim again
  let err = a.contract().claim_fee_pool_share(vec![advertiser.local_wallet().address()]).send().await.unwrap_err();
  assert_eq!(err.decode_revert::<String>().unwrap(), "cfps2");

  // These people who just received a transfer cannot claim yet
  a.run_idempotent_background_tasks_a_few_times().await;
  let err = a.contract().claim_fee_pool_share(holders.clone()).send().await.unwrap_err();
  assert_eq!(err.decode_revert::<String>().unwrap(), "cfps3");

  for h in &holders {
    assert_eq!(a.asami_balance_of(h).await, wei( "100000000000005227"));
    assert_eq!(a.doc_balance_of(h).await,   wei(                  "0"));
  }

  // Cycle 4:
  // New participants join and collaborate.
  // No fee pool is shared, as there were no collabs on previous period.

  a.run_idempotent_background_tasks_a_few_times().await;
  a.evm_forward_to_next_cycle().await;
  a.run_idempotent_background_tasks_a_few_times().await;

  a.doc_contract().transfer(advertiser.local_wallet().address(), u("100000"))
    .send().await.unwrap().await.unwrap().unwrap();
  campaign = advertiser.create_self_managed_x_campaign(u("100000"), u("300"), 50).await;
  let mut susan = a.client().await;
  susan.create_x_handle("susan_on_x", u("300")).await;
  susan.create_x_collab(&campaign).await;
  susan.claim_account().await;

  a.run_idempotent_background_tasks_a_few_times().await;
  assert_eq!(a.contract().get_fee_pool_before_recent_changes().call().await?, u("0"));

  // Advertiser cannot try to claim again
  let err = a.contract().claim_fee_pool_share(vec![advertiser.local_wallet().address()]).send().await.unwrap_err();
  assert_eq!(err.decode_revert::<String>().unwrap(), "cfps1");

  // These people who just received a transfer cannot claim yet
  let err = a.contract().claim_fee_pool_share(holders.clone()).send().await.unwrap_err();
  assert_eq!(err.decode_revert::<String>().unwrap(), "cfps1");

  for h in &holders {
    assert_eq!(a.asami_balance_of(h).await, wei( "100000000000005227"));
    assert_eq!(a.doc_balance_of(h).await,   wei(                  "0"));
  }

  // Cycle 5:
  // All hodlers got paid from the previous cycle collab.

  a.run_idempotent_background_tasks_a_few_times().await;
  a.evm_forward_to_next_cycle().await;
  a.run_idempotent_background_tasks_a_few_times().await;

  assert_eq!(a.contract().get_claimed_asami_tokens().call().await?, wei("60000000000000000000"));
  assert_eq!(a.contract().total_supply().call().await?,             wei("60000000000000000000"));
  assert_eq!(a.admin_asami_balance().await,                         wei("34799999999999466846")); // 57.99999999999911141%
  assert_eq!(bob.asami_balance().await,                             wei( "1500000000000000000")); //  2.5%
  assert_eq!(alice.asami_balance().await,                           wei( "3000000000000000000")); //  5.0%
  assert_eq!(susan.asami_balance().await,                           wei( "4500000000000000000")); //  7.5%
  assert_eq!(advertiser.asami_balance().await,                      wei( "6000000000000000000")); // 10.0%

  for h in &holders { //                                                                             17.0000000000008~%
    assert_eq!(a.asami_balance_of(h).await, wei( "100000000000005227"));
    assert_eq!(a.doc_balance_of(h).await,   wei(  "50000000000002613"));
  }

  // Fee pool should have been 30 DOC.
  assert_eq!(a.admin_doc_balance().await,                wei("419899742399999999999733423"));
  assert_eq!(bob.doc_balance().await,                    wei(       "91750000000000000000"));
  assert_eq!(alice.doc_balance().await,                  wei(      "183500000000000000000"));
  assert_eq!(advertiser.doc_balance().await,             wei(    "99705000000000000000000"));
  assert_eq!(susan.doc_balance().await,                  wei(      "272250000000000000000"));
  assert_eq!(a.contract_doc_balance().await,             wei(                         "51"));
  assert_eq!(a.contract().fee_pool().call().await?,      wei(                         "51"));

  a.run_idempotent_background_tasks_a_few_times().await;
  a.evm_forward_to_next_cycle().await;
  a.run_idempotent_background_tasks_a_few_times().await;
}
*/
