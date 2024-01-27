#[macro_use]
mod support;
use ethers::{types::Address, signers::Signer};

app_test!{ handles_can_be_updated_once_per_cycle (a) 
  let mut bob = a.client().await;
  bob.create_x_handle("bob_on_x", u("1")).await;
  let handle = bob.x_handle().await;

  assert_eq!(handle.price(), u("2"));
  assert_eq!(handle.score(), u("2"));
  assert_that!(
    handle.handle_topic_vec().into_iter().map(|x| x.topic_id).collect(),
    vec!["foo", "bar", "baz"]
   );

  /* 
   1. Bob signs up.
   2. An initial score, price and tokens are set, by the admin.
   3. Admin can submit updates to score
      -> Via HadndleScoreAndTagsUpdateRequest.
   4. Admin can submit updates to price.
      -> Via HandlePriceUpdateRequest.
   5. Updating any value again just queues up the new value.

   6. Trying to apply updates fails on this cycle.
   7. Cycle advances.
   8. Trying to apply updates now succeeds.
   9. User claims account.
   10. Admin cannot submit price updates.
      --> A manual "HandlePriceUpdateRequest" fails to be processed.
      --> Calling the low level contract function as the admin also fails.

   11. Admin can still submit tags and score requests.
   12. Cycle and update happens again.
   */



  /*
   * A single very large campaign and collaboration issues all tokens, and no further tokens are issued
   * Unclaimed tokens are honored by the cap as if they had been issued

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
   */
}
