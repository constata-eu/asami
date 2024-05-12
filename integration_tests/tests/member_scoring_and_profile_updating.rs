#[macro_use]
pub mod support;
use api::models::*;
use support::TestApp;

app_test! { handles_can_be_updated_once_per_cycle (a)
  for i in &["sports", "crypto", "beauty"] {
    a.app.topic_request().create(i).await?;
    a.run_idempotent_background_tasks_a_few_times().await;
  }
  let sports = a.app.topic().find(weihex("1")).await?;
  let crypto = a.app.topic().find(weihex("2")).await?;
  let beauty = a.app.topic().find(weihex("3")).await?;

  let mut bob_client = a.client().await;
  bob_client.create_x_handle("bob_on_x", u("1")).await;
  let bob = bob_client.x_handle().await;

  let alice_client = a.client().await;
  alice_client.create_x_handle("alice_on_x", u("1")).await;
  let alice = alice_client.x_handle().await;

  assert_eq!(u256(bob.price()), u("1"));
  assert_eq!(u256(bob.score()), wei("10"));
  assert_eq!(
    bob.handle_topic_vec().await?.into_iter().map(|x| x.attrs.topic_id).collect::<Vec<String>>(),
    Vec::<String>::new(),
   );

  assert_handle(&a, 0, bob.id(), u("1"), wei("10"), &[], false, "the starting point").await;

  a.run_idempotent_background_tasks_a_few_times().await;
  assert_non_updateable(&a, "at the beginning").await;

  assert_handle(&a, 0, bob.id(), u("1"), wei("10"), &[], false, "still not update attempted").await;

  a.app.set_score_and_topics_request().create(&bob, wei("2000"), &[&sports]).await?;
  a.app.set_price_request().create(&bob, u("2")).await?;
  a.app.set_score_and_topics_request().create(&alice, wei("3000"), &[&crypto]).await?;
  a.app.set_price_request().create(&alice, u("3")).await?;

  a.app.set_price_request().submit_all().await?;
  a.app.set_score_and_topics_request().submit_all().await?;

  assert_handle(&a, 0, bob.id(), u("1"), wei("10"), &[], true, "first time update needed").await;

  a.run_idempotent_background_tasks_a_few_times().await;
  assert_non_updateable(&a, "after first update").await;

  assert_handle(&a, 0, bob.id(), u("2"), wei("2000"), &[&sports], false, "First bob update").await;
  assert_handle(&a, 1, alice.id(), u("3"), wei("3000"), &[&crypto], false, "First alice update").await;

  a.app.set_score_and_topics_request().create(&bob, wei("4000"), &[&sports, &beauty]).await?;
  a.app.set_price_request().create(&bob, u("4")).await?;
  a.app.set_score_and_topics_request().create(&alice, wei("5000"), &[&crypto, &beauty]).await?;
  a.app.set_price_request().create(&alice, u("5")).await?;
  a.run_idempotent_background_tasks_a_few_times().await;
  assert_non_updateable(&a, "after further update, but still on same cycle").await;

  assert_handle(&a, 0, bob.id(), u("2"), wei("2000"), &[&sports], true, "second bob update in same cycle").await;
  assert_handle(&a, 1, alice.id(), u("3"), wei("3000"), &[&crypto], true, "second alice in same cycle").await;

  a.evm_forward_to_next_cycle().await;
  a.run_idempotent_background_tasks_a_few_times().await;
  assert_handle(&a, 0, bob.id(), u("4"), wei("4000"), &[&sports, &beauty], false, "bob third update").await;
  assert_handle(&a, 1, alice.id(), u("5"), wei("5000"), &[&crypto, &beauty], false, "alice third update").await;

  a.evm_forward_to_next_cycle().await;

  bob_client.claim_account().await;
  a.app.set_score_and_topics_request().create(&bob, wei("6000"), &[&crypto, &sports]).await?;
  a.run_idempotent_background_tasks_a_few_times().await;

  assert_eq!(
    a.app.set_price_request().create(&bob, u("6")).await.unwrap_err().to_string(),
    "Invalid input on handle: cannot_set_price_on_claimed_account".to_string()
  );

  let raw_set_price_input = vec![api::on_chain::AdminSetPriceInput{
    handle_id: u256(bob.id()),
    price: u("6")
  }];
  assert!(a.contract().admin_set_price(raw_set_price_input).send().await.is_err());
  assert!(a.contract().set_price(u256(bob.id()), u("6")).send().await.is_err());
  a.run_idempotent_background_tasks_a_few_times().await;
  assert_handle(&a, 0, bob.id(), u("4"), wei("6000"), &[&crypto, &sports], false, "admin could only set bob's score and tags").await;

  a.evm_forward_to_next_cycle().await;

  assert!(bob_client.contract().set_price(u256(bob.id()), u("6")).send().await?.await.unwrap().is_some());
  a.run_idempotent_background_tasks_a_few_times().await;
  assert_handle(&a, 0, bob.id(), u("6"), wei("6000"), &[&crypto, &sports], false, "bob updated his price").await;
}

app_test! { does_not_allow_more_than_one_claim_account_request (a)
  let mut alice = a.client().await;
  alice.submit_claim_account_request().await;
  assert_eq!(
    alice.gql_claim_account_request(alice.local_wallet()).await.errors.unwrap()[0].message,
    "Invalid input on account: cannot_call_on_claimed_account"
  );
}

app_test! { can_have_a_zero_score_handle (a)
  let advertiser = a.client().await;
  let campaign = advertiser.create_x_campaign(u("10"), u("5")).await;

  let bob = a.client().await;
  bob.create_x_handle_with_score("bob_on_x", u("0"), u("0")).await;
  let handle = bob.x_handle().await;
  assert_eq!(handle.attrs.score, weihex("0"));

  assert!(bob.get_campaign_offers().await.all_campaigns.len() == 0);

  assert_that!(&campaign.make_collab(&handle).await.unwrap_err(),
    structure!{api::Error::Validation[eq("score".to_string()), eq("handle_score_is_zero".to_string())]}
  );
}

async fn assert_non_updateable(a: &TestApp, msg: &str) {
    assert!(
        a.app.on_chain_tx().apply_handle_updates().await.expect(msg).is_none(),
        "{msg} on_chain_tx"
    );
    assert!(
        matches!(
            a.contract().apply_handle_updates(vec![wei("0"), wei("1")]).send().await.unwrap_err(),
            ethers::contract::ContractError::Revert(_)
        ),
        "{msg} on contract"
    );
}

async fn assert_handle(
    a: &TestApp,
    pos: usize,
    id: &str,
    price: U256,
    score: U256,
    topics: &[&Topic],
    needs_update: bool,
    msg: &str,
) {
    let topic_ids: Vec<U256> = topics.iter().map(|t| u256(t.id())).collect();

    let values = a.contract().get_handles().call().await.expect(msg).remove(pos);
    assert_eq!(values.id, u256(id), "{msg} id");
    assert_eq!(values.price, price, "{} price", msg);
    assert_eq!(values.score, score, "{} score", msg);
    assert_eq!(values.topics, topic_ids, "{} topics", msg);
    assert_eq!(values.needs_update, needs_update, "{} needs_update", msg);

    let db = a.app.handle().find(&id.to_string()).await.expect(msg);
    assert_eq!(u256(db.price()), price, "{} db price", msg);
    assert_eq!(u256(db.score()), score, "{} db score", msg);
    assert_eq!(
        db.handle_topic_vec().await.unwrap().iter().map(|t| u256(t.topic_id())).collect::<Vec<U256>>(),
        topic_ids,
        "{msg} db topics"
    );
}
