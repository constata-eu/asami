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
  assert_eq!(a.contract().fee_rate().call().await?, wei("10"));

  let mut advertiser = a.client().await;
  let budget = u("3000");
  let campaign = advertiser.create_x_campaign(budget, budget).await;
  advertiser.claim_account().await;

  let mut bob = a.client().await;
  bob.create_x_handle("bob_on_x", budget).await;
  bob.create_x_collab(&campaign).await;
  bob.claim_account().await;

  advertiser.self_submit_fee_rate_vote(wei("20")).await?;
  assert_eq!(a.contract().voted_fee_rate_vote_count().call().await?, u("30"));
  assert_eq!(a.contract().voted_fee_rate().call().await?, u("20"));
  assert_eq!(a.contract().fee_rate().call().await?, wei("10"));

  a.contract().apply_voted_fee_rate().send().await.unwrap().await.unwrap().unwrap();
  assert_eq!(a.contract().fee_rate().call().await?, wei("20"));

  bob.self_submit_fee_rate_vote(wei("1")).await?;
  assert_eq!(a.contract().voted_fee_rate_vote_count().call().await?, u("75"));
  assert_eq!(a.contract().voted_fee_rate().call().await?, wei("8600000000000000000"));

  assert!(a.contract().apply_voted_fee_rate().send().await.is_err());
  assert_eq!(a.contract().fee_rate().call().await?, wei("20"));

  a.evm_increase_time(wei("60") * wei("60") * wei("24") * wei("15")).await;
  a.evm_mine().await;

  a.contract().apply_voted_fee_rate().send().await.unwrap().await.unwrap().unwrap();
  assert_eq!(a.contract().fee_rate().call().await?, wei("8"));

  advertiser.self_remove_fee_rate_vote().await?;
  assert_eq!(a.contract().voted_fee_rate_vote_count().call().await?, u("45"));
  assert_eq!(a.contract().voted_fee_rate().call().await?, u("1"));

  bob.self_submit_fee_rate_vote(wei("50")).await.unwrap();
  assert_eq!(a.contract().voted_fee_rate_vote_count().call().await?, u("45"));
  assert_eq!(a.contract().voted_fee_rate().call().await?, u("50"));

  // Cannot vote out of bound values.
  assert!(advertiser.self_submit_fee_rate_vote(wei("0")).await.is_err());
  assert!(advertiser.self_submit_fee_rate_vote(wei("100")).await.is_err());

  // Will remove the fee rate vote when the user moves their tokens.
  // Voted rate goes back to factory settings when no vote is applied.
  bob.contract().transfer(advertiser.local_wallet().address(), u("2")).send().await.unwrap().await.unwrap().unwrap();
  assert_eq!(a.contract().voted_fee_rate_vote_count().call().await?, u("0"));
  assert_eq!(a.contract().voted_fee_rate().call().await?, u("10"));


  /*
  dbg!(c.increase_time("100000").await);
  c.evm_mine().await;
  dbg!(c.provider.get_block(c.provider.get_block_number().await.unwrap()).await.unwrap())
  */
  // Remove vote.
  // Sending tokens removes vote too.
  // Vote over 99 fails.
  // Vote under 1 fails.
}

app_test!{ admin_can_be_voted_via_vested_votes (a)
  /*
   *
  
  * Vote doing / undoing with vesting.
  * Election result is stored on chain somewhere.
  * Election result is applied by another call which is guarded to happen only once in a time period.

  We make one collab to issue tokens to three entities.

  A vote is for all issues:
    map (address => Vote) adminElectionVotes;
    address[] adminElectionVoters;

    map (address => uint256) adminElectionVotesPerCandidate;
    address[] adminElectionCandidates;

    TallyAdminElectionVotes:
      - Clear the adminElectionCandidates array. (or use an in-memory one?)
      - What if counting votes goes over the gas limit?
        

      - For each admin election voter
        - Check their vote.createdAt is lower than threshold.
        - Add their token balance to the candidate.
      
    Revisamos el voto de todos los voters, contamos 


    Vote:
      Admin: 
      Fee:


  Test 1:
    - Ballot is for a single address, to set both as the admin and admin treasury.
    - One period passes, tallying votes does nothing.
    - Two periods pass, tallying votes does nothing.
    - Winner can go ahead and change the admin address.


  Majority of votes held for three periods will change the admin.

  Test 2: 
   - Less than 50% vote for another candidate for 3 periods, the candidate does not change.

  Test 3:
    - Try voting more than 100%
    - Try voting less than 0%

  */

  /*
  dbg!(c.increase_time("100000").await);
  c.evm_mine().await;
  dbg!(c.provider.get_block(c.provider.get_block_number().await.unwrap()).await.unwrap())
  */
}

// A campaign paying rewards for 210000000000000000000000000 will issue all outstanding tokens.
