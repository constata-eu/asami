use ::api::on_chain::*;

app_test!{ full_contract_workflow_from_collabs_to_voting(a)
  let mut advertiser = a.client().await;
  advertiser.setup_as_advertiser("global advertiser for test").await;
  advertiser.make_campaign("campaign that will be drained", u("2000"), u("deadbeef"), 2).await;

  let collabs_params = vec![MakeCollabsParam{
    advertiser_id: advertiser.account_id(),
    briefing: u("deadbeef"),
    collabs: (0..20)
      .map(|i| MakeCollabsParamItem{ account_id: wei((i+1).to_string()), doc_reward: u("20") } )
      .collect()
  }];

  a.send_tx(
    "Collaborations are registered for 20 different new accounts.",
    "1084176",
    a.asami_core().admin_make_collabs(collabs_params.clone())
  ).await;

  a.send_tx(
    "A second round of collaborations now that accounts have been registered.",
    "331798",
    a.asami_core().admin_make_collabs(collabs_params.clone())
  ).await;

  let mut alice = a.client().await;
  alice.make_client_wallet().await;
  let mut bob = a.client().await;
  bob.make_client_wallet().await;
  let mut carl = a.client().await;
  carl.make_client_wallet().await;

  a.send_tx(
    "claiming 3 of those collaborator accounts",
    "229761",
    a.asami_core().claim_accounts(vec![
      ClaimAccountsParam{ account_id: alice.account_id(), addr: alice.address() },
      ClaimAccountsParam{ account_id: bob.account_id(), addr: bob.address() },
      ClaimAccountsParam{ account_id: carl.account_id(), addr: carl.address() },
    ])
  ).await;

  a.assert_balances_of("alice before claim", alice.account_id(), u("10"), u("36"), u("0"), wei("600000000000000000"), u("0")).await;
  a.assert_balances_of("bob before claim", bob.account_id(), u("10"), u("36"), u("0"), wei("600000000000000000"), u("0")).await;

  let admin_doc_fee = u("3");
  a.send_tx(
    "We make gasless claims for alice and bob",
    "387103",
    a.asami_core().gasless_claim_balances(admin_doc_fee, u("1"), vec![alice.account_id(), bob.account_id()]).value(u("2")),
  ).await;

  a.assert_balances_of("alice after claim", alice.account_id(), u("11"), u("0"), u("33"), u("0"), wei("600000000000000000")).await;
  a.assert_balances_of("bob after claim", bob.account_id(), u("11"), u("0"), u("33"), u("0"), wei("600000000000000000")).await;
  assert_eq!(a.admin_unclaimed_doc_balance().await, u("6")); // The 3 DOC fee was charged twice.

  a.assert_balances_of("carl before claim", carl.account_id(), u("10"), u("36"), u("0"), wei("600000000000000000"), u("0")).await;
  a.send_tx(
    "Carl makes a regular claim, paying his own fees.",
    "169722",
    carl.asami_contract().claim_balances(vec![carl.account_id()])
  ).await;

  a.assert_balances_of("carl after claim", carl.account_id(),
    wei("9999660556000000000"),
    u("0"), u("36"),
    u("0"), wei("600000000000000000")
  ).await;

  a.assert_admin_balances("Before its own claim.",
    u("6"), u("419997000"), u("99160000000"), // 6 unclaimed doc from gasless 
    u("60"), u("0"), u("0")
  ).await;
  a.send_tx(
    "Admin claims their unclaimed balances", 
    "151402",
    a.asami_core().claim_admin_unclaimed_balances()
  ).await;
  a.assert_admin_balances("admin after its own claim.",
    u("0"), u("419997000"), u("99160000006"),
    u("0"), u("0"), u("60")
  ).await;
}

app_test!{ rejects_collabs_when_too_large_or_campaign_is_finished(a)
  let mut advertiser = a.client().await;
  advertiser.setup_as_advertiser("an advertiser for the whole test").await;
  let briefing = u("deadbeef");
  advertiser.make_campaign("campaign that will be drained", u("1000"), briefing, 2).await;

  let make_collabs_call = |doc_reward: &str| {
    a.asami_core().admin_make_collabs(
      vec![MakeCollabsParam{
        advertiser_id: advertiser.account_id(),
        briefing,
        collabs: vec![ MakeCollabsParamItem{ account_id: wei("1"), doc_reward: u(doc_reward) } ]
      }]
    )
  };

  a.send_tx("Collab drains half the budget", "1084176", make_collabs_call("500") ).await;

  a.send_revert_tx("Collab is too big for budget", "amc4", make_collabs_call("800")).await;

  let txs = a.send_without_mining(
    "These can get accepted independentnly, but last one will exceed budget",
    a.admin_nonce().await,
    vec![ make_collabs_call("200"), make_collabs_call("400") ]
  ).await;

  a.wait_tx_state("200 DOC collab passes", &txs[0], models::OnChainTxStatus::Success).await;
  a.wait_tx_failure("400 DOC collab does not pass", &txs[1], "amc4").await;

  let sent_before_end = a.send_without_mining(
    "This tx is sent before campaign ends, but will be mined after campaign ends",
    a.admin_nonce().await,
    vec![make_collabs_call("200")]
  ).await.pop().unwrap();

  a.evm_forward_to_next_cycle().await;

  a.wait_tx_failure("Sent before end, but still fails", &sent_before_end, "amc1").await;

  a.send_revert_tx( "Further attempts get reverted", "amc1", make_collabs_call("50")).await;
}

app_test!{ gasless_claim_common_error_cases(a)
  let mut advertiser = a.client().await;
  advertiser.setup_as_advertiser("global advertiser for test").await;
  advertiser.make_campaign("global campaign for test", u("2000"), u("deadbeef"), 2).await;

  let mut alice = a.client().await;
  alice.make_client_wallet().await;

  a.send_make_collab_tx("A very small collab is registered", "10000", &advertiser, u("deadbeef"), &alice, u("1")).await;

  a.assert_balances_of(
    "Alice balances after collab", 
    alice.account_id(),
    wei("0"),
    wei("900000000000000000"), u("0"),
    wei("15000000000000000"), u("0")
  ).await;

  a.send_revert_tx(
    "The BTC amount to send is less than 1e11",
    "gcb0",
    a.asami_core().gasless_claim_balances(u("1"), wei("1000000"), vec![alice.account_id()]).value(wei("1000000")),
  ).await;

  a.send_revert_tx(
    "The BTC value is not exactly what's going to be distributed",
    "gcb1",
    a.asami_core().gasless_claim_balances(u("1"), u("1"), vec![alice.account_id()]).value(u("2")),
  ).await;

  a.send_revert_tx(
    "Alice has not claimed her account yet, so cannot claim pending balances",
    "gcb2",
    a.asami_core().gasless_claim_balances(u("5"), u("1"), vec![alice.account_id()]).value(u("1")),
  ).await;

  a.send_tx(
    "Claiming Alice account",
    "229761",
    a.asami_core().claim_accounts(vec![
      ClaimAccountsParam{ account_id: alice.account_id(), addr: alice.address() },
    ])
  ).await;

  a.send_revert_tx(
    "Alice cannot pay the gasless DOC fee",
    "gcb3",
    a.asami_core().gasless_claim_balances(u("5"), u("1"), vec![alice.account_id()]).value(u("1")),
  ).await;

  a.send_tx(
    "Alice can withdraw if fee is low enough ",
    "257592",
    a.asami_core().gasless_claim_balances(wei("10000000"), u("1"), vec![alice.account_id()]).value(u("1")),
  ).await;

  a.assert_balances_of(
    "Alice balances after gasless claim", 
    alice.account_id(),
    u("11"),
    u("0"), wei("899999999990000000"),
    u("0"), wei("15000000000000000")
  ).await;
}

app_test!{ user_can_manage_a_gasless_amount_approval(a)
  let mut advertiser = a.client().await;
  advertiser.setup_as_advertiser("global advertiser for test").await;
  advertiser.make_campaign("global campaign for test", u("2000"), u("deadbeef"), 2).await;

  let mut alice = a.client().await;
  alice.make_client_wallet().await;
  a.send_make_collab_tx("A collab is registered generating rewards", "10000", &advertiser, u("deadbeef"), &alice, u("100")).await;

  a.send_revert_tx(
    "Alice cannot change her approvedGaslessAmount before claiming",
    "cga0",
    alice.asami_contract().change_gasless_approval(u("6"))
  ).await;

  a.send_tx(
    "Claiming Alice account",
    "229761",
    a.asami_core().claim_accounts(vec![
      ClaimAccountsParam{ account_id: alice.account_id(), addr: alice.address() },
    ])
  ).await;

  a.send_revert_tx(
    "The admin cannot charge more than the gasless approval, which defaults to 5",
    "gcb4",
    a.asami_core().gasless_claim_balances(u("6"), u("1"), vec![alice.account_id()]).value(u("1")),
  ).await;

  a.send_tx(
    "Alice can change her gasless approval now",
    "30955",
    alice.asami_contract().change_gasless_approval(u("6"))
  ).await;

  a.send_tx(
    "The admin can now do a gasless claim.",
    "257650",
    a.asami_core().gasless_claim_balances(u("6"), u("1"), vec![alice.account_id()]).value(u("1")),
  ).await;
}

app_test!{ simultaneous_gasless_and_regular_claims_fail(a)
  let mut advertiser = a.client().await;
  advertiser.setup_as_advertiser("global advertiser for test").await;
  advertiser.make_campaign("global campaign for test", u("2000"), u("deadbeef"), 2).await;

  let mut alice = a.client().await;
  alice.make_client_wallet().await;
  a.send_make_collab_tx("A collab is registered generating rewards", "10000", &advertiser, u("deadbeef"), &alice, u("100")).await;

  a.send_tx(
    "Claiming Alice account",
    "229761",
    a.asami_core().claim_accounts(vec![
      ClaimAccountsParam{ account_id: alice.account_id(), addr: alice.address() },
    ])
  ).await;

  let txs = a.send_without_mining(
    "These can get accepted independentnly, but last one will exceed budget",
    a.admin_nonce().await,
    vec![
      a.asami_core().claim_balances(vec![alice.account_id()]),
      a.asami_core().gasless_claim_balances(u("4"), u("1"), vec![alice.account_id()]).value(u("1")),
    ]
  ).await;

  a.wait_tx_state("regular claim always passes", &txs[0], models::OnChainTxStatus::Success).await;
  a.wait_tx_failure("gasless claim fails", &txs[1], "gcb3").await;
}

app_test!{ accounts_are_tied_to_an_address_forever(a) 
  let mut alice = a.client().await;
  alice.make_client_wallet().await;
  let mut bob = a.client().await;
  bob.make_client_wallet().await;

  a.send_tx(
    "Alice can claim her account",
    "229761",
    a.asami_core().claim_accounts(vec![
      ClaimAccountsParam{ account_id: alice.account_id(), addr: alice.address() },
    ])
  ).await;

  a.send_revert_tx(
    "Alice cannot claim her account again",
    "ca0",
    a.asami_core().claim_accounts(vec![
      ClaimAccountsParam{ account_id: alice.account_id(), addr: alice.address() },
    ])
  ).await;

  a.send_revert_tx(
    "Alice cannot reuse her address to claim bob account id",
    "ca1",
    a.asami_core().claim_accounts(vec![
      ClaimAccountsParam{ account_id: bob.account_id(), addr: alice.address() },
    ])
  ).await;

  a.send_tx(
    "Bob can claim his account",
    "229761",
    a.asami_core().claim_accounts(vec![
      ClaimAccountsParam{ account_id: bob.account_id(), addr: bob.address() },
    ])
  ).await;
}


// Migrate fee distribution tests here.
