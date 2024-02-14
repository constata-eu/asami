#[macro_use]
mod support;
use ethers::signers::Signer;

app_test!{ has_a_cap_on_token_supply (a) 
  /*
   * A single very large campaign and collaboration issues all tokens, and no further tokens are issued
   * Unclaimed tokens are honored by the cap as if they had been issued
   */

  let mut advertiser = a.client().await;
  let budget = u("420000000");
  let campaign = advertiser.create_x_campaign(budget, budget).await;

  let mut bob = a.client().await;
  bob.create_x_handle("bob_on_x", u("210000000")).await;
  bob.create_x_collab(&campaign).await;
  bob.claim_account().await;

  assert_eq!(a.contract().get_claimed_asami_tokens().call().await?, u("21000000"));

  assert_eq!(a.contract().total_supply().call().await?, u("18900000"));

  assert_eq!(a.contract().balance_of(bob.local_wallet().address()).call().await?, u("3150000"));

  let mut alice = a.client().await;
  alice.create_x_handle("alice_on_x", u("210000000")).await;
  alice.create_x_collab(&campaign).await;
  alice.claim_account().await;

  assert_eq!(a.contract().balance_of(alice.local_wallet().address()).call().await?, u("0"));

  assert_eq!(a.contract().get_claimed_asami_tokens().call().await?, u("21000000"));

  assert_eq!(a.contract().total_supply().call().await?, u("18900000"));
  advertiser.claim_account().await;
  assert_eq!(a.contract().total_supply().call().await?, u("21000000"));
}

app_test!{ rate_can_be_voted (a) 
  assert_eq!(a.contract().fee_rate().call().await?, u("10"));

  let mut advertiser = a.client().await;
  let budget = u("3000");
  let campaign = advertiser.create_x_campaign(budget, budget).await;
  advertiser.claim_account().await;

  let mut bob = a.client().await;
  bob.create_x_handle("bob_on_x", budget).await;
  bob.create_x_collab(&campaign).await;
  bob.claim_account().await;

  assert!(a.contract().apply_voted_fee_rate().send().await.is_err());

  a.evm_forward_to_next_cycle().await;

  advertiser.self_submit_fee_rate_vote(u("20")).await.unwrap();
  assert_eq!(a.contract().voted_fee_rate_vote_count().call().await?, u("30"));
  assert_eq!(a.contract().voted_fee_rate().call().await?, u("20"));
  assert_eq!(a.contract().fee_rate().call().await?, u("10"));

  a.app.on_chain_tx().apply_voted_fee_rate().await.unwrap();
  assert_eq!(a.contract().fee_rate().call().await?, u("20"));

  bob.self_submit_fee_rate_vote(u("1")).await.unwrap();
  assert_eq!(a.contract().voted_fee_rate_vote_count().call().await?, u("75"));
  assert_eq!(a.contract().voted_fee_rate().call().await?, wei("8600000000000000000"));

  a.app.on_chain_tx().apply_voted_fee_rate().await.unwrap();
  assert_eq!(a.contract().fee_rate().call().await?, u("20"));

  a.evm_forward_to_next_cycle().await;

  a.app.on_chain_tx().apply_voted_fee_rate().await.unwrap();
  assert_eq!(a.contract().fee_rate().call().await?, wei("8600000000000000000"));

  advertiser.self_remove_fee_rate_vote().await?;
  assert_eq!(a.contract().voted_fee_rate_vote_count().call().await?, u("45"));
  assert_eq!(a.contract().voted_fee_rate().call().await?, u("1"));

  bob.self_submit_fee_rate_vote(u("50")).await.unwrap();
  assert_eq!(a.contract().voted_fee_rate_vote_count().call().await?, u("45"));
  assert_eq!(a.contract().voted_fee_rate().call().await?, u("50"));

  // Cannot vote out of bound values.
  assert!(advertiser.self_submit_fee_rate_vote(u("0")).await.is_err());
  assert!(advertiser.self_submit_fee_rate_vote(u("100")).await.is_err());

  // Will remove the fee rate vote when the user moves their tokens.
  // Voted rate goes back to factory settings when no vote is applied.
  bob.contract()
    .transfer(advertiser.local_wallet().address(), u("2"))
    .send().await.unwrap().await.unwrap().unwrap();
  assert_eq!(a.contract().voted_fee_rate_vote_count().call().await?, u("0"));
  assert_eq!(a.contract().voted_fee_rate().call().await?, u("10"));
}

app_test!{ admin_can_be_voted_via_vested_votes (a)
  let admin_addr = a.app.on_chain.contract.client().address();

  let mut advertiser = a.client().await;
  let budget = u("3000");
  let campaign = advertiser.create_x_campaign(budget, budget).await;
  advertiser.claim_account().await;
  let advertiser_addr = advertiser.local_wallet().address();

  let mut bob = a.client().await;
  bob.create_x_handle("bob_on_x", budget).await;
  bob.create_x_collab(&campaign).await;
  bob.claim_account().await;
  let bob_addr = bob.local_wallet().address();

  let mut last_admin_election = u("0");
  assert_eq!(a.contract().last_admin_election().call().await?, last_admin_election);
  assert_eq!(a.contract().vested_admin_votes_total().call().await?, u("0"));
  assert_eq!(a.contract().get_latest_admin_elections().call().await?,
    [Address::zero(), Address::zero(), Address::zero()]
  );
  assert!(a.app.on_chain_tx().proclaim_cycle_admin_winner().await?.is_none());

  /* Advertiser starts the process of voting for himself */
  advertiser.self_submit_admin_vote(advertiser_addr).await?;
  assert!(a.app.on_chain_tx().proclaim_cycle_admin_winner().await?.is_none());

  a.evm_forward_to_next_cycle().await;
  bob.self_submit_admin_vote(bob_addr).await?;

  // Advertiser is vesting and hurrying to claim victory.
  assert!(advertiser.self_vest_admin_vote(advertiser_addr).await.is_ok());
  a.contract().submitted_admin_votes(advertiser_addr).call().await?;
  assert!(
    a.app.on_chain_tx()
      .proclaim_cycle_admin_winner().await?
      .expect("on_chain_tx")
      .success()
  );

  assert!(a.contract().last_admin_election().call().await? > last_admin_election);
  last_admin_election = a.contract().last_admin_election().call().await?;
  assert_eq!(a.contract().vested_admin_votes_total().call().await?, u("30"));
  assert_eq!(a.contract().get_latest_admin_elections().call().await?,
    [advertiser_addr, Address::zero(), Address::zero()]
  );

  a.evm_forward_to_next_cycle().await;

  // Advertiser is hurrying again to claim its win before other votes vest.
  assert!(
    a.app.on_chain_tx()
      .proclaim_cycle_admin_winner().await?
      .expect("on_chain_tx")
      .success()
  );

  // Bob vested his votes late, so he cannot claim the election this cycle.
  assert!(bob.self_vest_admin_vote(bob_addr).await.is_ok());
  assert!(a.app.on_chain_tx().proclaim_cycle_admin_winner().await?.is_none());

  assert!(a.contract().last_admin_election().call().await? > last_admin_election);
  last_admin_election = a.contract().last_admin_election().call().await?;
  assert_eq!(a.contract().vested_admin_votes_total().call().await?, u("75"));
  assert_eq!(a.contract().get_latest_admin_elections().call().await?,
    [advertiser_addr, advertiser_addr, Address::zero()]
  );

  // Starting the next cycle, advertiser cannot claim the win anymore. 
  a.evm_forward_to_next_cycle().await;
  assert!(a.app.on_chain_tx().proclaim_cycle_admin_winner().await?.unwrap().success());

  assert!(a.contract().last_admin_election().call().await? > last_admin_election);
  assert_eq!(a.contract().get_latest_admin_elections().call().await?, [bob_addr, advertiser_addr,advertiser_addr]);
  assert_eq!(a.contract().admin().call().await?, admin_addr);
  assert_eq!(a.contract().admin_treasury().call().await?, admin_addr);

  // Bob keeps claiming the election two more times and becomes the new admin.
  a.evm_forward_to_next_cycle().await;
  assert!(a.app.on_chain_tx().proclaim_cycle_admin_winner().await?.unwrap().success());
  assert_eq!(a.contract().get_latest_admin_elections().call().await?,
    [bob_addr, bob_addr, advertiser_addr]
  );

  a.evm_forward_to_next_cycle().await;
  assert!(a.app.on_chain_tx().proclaim_cycle_admin_winner().await?.unwrap().success());
  assert_eq!(a.contract().get_latest_admin_elections().call().await?,
    [bob_addr, bob_addr, bob_addr]
  );
  assert_eq!(a.contract().admin().call().await?, bob_addr);
  assert_eq!(a.contract().admin_treasury().call().await?, bob_addr);

  // The new admin can change the hot wallet address to whatever it wants.
  assert!(advertiser.self_set_admin_address(bob_addr).await.is_err());
  assert!(bob.self_set_admin_address(advertiser_addr).await.is_ok());
  assert_eq!(a.contract().admin().call().await?, advertiser_addr);

  // Advertiser uses its tokens, which removes his votes.
  advertiser.contract().transfer(bob_addr, u("2")).send().await.unwrap().await.unwrap().unwrap();
  assert_eq!(a.contract().vested_admin_votes_total().call().await?, u("45"));

  // Bob has no need to keep its stake, so it removes it
  bob.self_remove_admin_vote().await?;
  assert_eq!(a.contract().vested_admin_votes_total().call().await?, u("0"));

  // Now that there are no votes, no claim is possible.
  a.evm_forward_to_next_cycle().await;
  assert!(a.app.on_chain_tx().proclaim_cycle_admin_winner().await?.is_none());
}

app_test!{ contract_cannot_set_cycle_winner_with_no_votes (a)
  let mut bob = a.client().await;
  bob.create_x_handle("bob_on_x", u("40")).await;
  bob.claim_account().await;
  let bob_addr = bob.local_wallet().address();
  assert!(
    a.app.on_chain.contract.proclaim_cycle_admin_winner(bob_addr)
      .send().await.unwrap_err().is_revert()
  );
}

app_test!{ admin_vote_vesting_validations (a)
  let mut advertiser = a.client().await;
  let budget = u("3000");
  let campaign = advertiser.create_x_campaign(budget, budget).await;
  advertiser.claim_account().await;
  let advertiser_addr = advertiser.local_wallet().address();

  let bob = a.client().await;
  bob.create_x_handle("bob_on_x", budget).await;
  bob.create_x_collab(&campaign).await;

  // Cannot vest without having voted.
  assert!(advertiser.self_vest_admin_vote(advertiser_addr).await.is_err());

  // Cannot vest on the same cycle.
  advertiser.self_submit_admin_vote(advertiser_addr).await?;
  a.run_idempotent_background_tasks_a_few_times().await;
  assert!(advertiser.self_vest_admin_vote(advertiser_addr).await.is_err());
  assert_eq!(a.contract().vested_admin_votes_total().call().await?, u("0"));

  // Can vest on the following cycle.
  a.evm_forward_to_next_cycle().await;
  a.run_idempotent_background_tasks_a_few_times().await;
  assert_eq!(a.contract().vested_admin_votes_total().call().await?, u("30"));

  // And cannot vest again
  assert!(advertiser.self_vest_admin_vote(advertiser_addr).await.is_err());
  assert_eq!(a.contract().vested_admin_votes_total().call().await?, u("30"));
}
