use api::on_chain::*;

app_test! { full_contract_workflow_from_collabs_to_balance_claims(a)
  let mut advertiser = a.client().await;
  advertiser.setup_as_advertiser("global advertiser for test").await;
  advertiser.pay_campaign("campaign that will be drained", u("2000"), u("deadbeef"), 2).await;

  let collabs_params = vec![MakeSubAccountCollabsParam{
    advertiser_addr: advertiser.address(),
    briefing_hash: u("deadbeef"),
    collabs: (0..20)
      .map(|i| MakeSubAccountCollabsParamItem{ sub_account_id: wei((i+1).to_string()), doc_reward: u("20") } )
      .collect()
  }];

  a.send_tx(
    "Collaborations are registered for 20 different new sub accounts.",
    "1178091",
    a.asami_contract().admin_make_sub_account_collabs(collabs_params.clone())
  ).await;

  a.send_tx(
    "A second round of collaborations now that sub accounts have been registered.",
    "391491",
    a.asami_contract().admin_make_sub_account_collabs(collabs_params.clone())
  ).await;

  let mut alice = a.client().await;
  alice.make_client_wallet().await;
  let mut bob = a.client().await;
  bob.make_client_wallet().await;
  let mut carl = a.client().await;
  carl.make_client_wallet().await;

  a.send_tx(
    "promoting 3 of those collaborator accounts",
    "378000",
    a.asami_contract().promote_sub_accounts(vec![
      PromoteSubAccountsParam{ id: alice.account_id(), addr: alice.address() },
      PromoteSubAccountsParam{ id: bob.account_id(), addr: bob.address() },
      PromoteSubAccountsParam{ id: carl.account_id(), addr: carl.address() },
    ])
  ).await;

  a.assert_balances_of("alice address after promote", alice.address(), u("10"), u("36"), u("0"), u("4800"), u("0")).await;
  a.assert_balances_of("bob address after promote", alice.address(), u("10"), u("36"), u("0"), u("4800"), u("0")).await;
  a.assert_sub_account_balances_of("alice subaccount after promote", alice.account_id(), u("0"), u("0")).await;
  a.assert_sub_account_balances_of("bob subaccount after promote", bob.account_id(), u("0"), u("0")).await;


  let admin_doc_fee = u("3");
  a.send_tx(
    "We make gasless claims for alice and bob",
    "387103",
    a.asami_contract().gasless_claim_balances(admin_doc_fee, u("1"), vec![alice.address(), bob.address()]).value(u("2")),
  ).await;

  a.assert_balances_of("alice after claim", alice.address(), u("11"), u("0"), u("33"), u("0"), u("4800")).await;
  a.assert_balances_of("bob after claim", bob.address(), u("11"), u("0"), u("33"), u("0"), u("4800")).await;
  assert_eq!(a.admin_unclaimed_doc_balance().await, u("6")); // The 3 DOC fee was charged two times.

  a.assert_balances_of("carl before claim", carl.address(), u("10"), u("36"), u("0"), u("4800"), u("0")).await;
  a.send_tx(
    "Carl makes a regular claim, paying his own fees.",
    "169722",
    carl.asami_contract().claim_balances()
  ).await;

  a.assert_balances_of("carl after claim", carl.address(),
    wei("9999878661999150634"),
    u("0"), u("36"),
    u("0"), u("4800")
  ).await;

  a.assert_admin_balances("Before its own claim.",
    u("6"), u("419998000"), u("99160000000"), // 6 unclaimed doc from gasless
    u("128000"), u("0"), u("0")
  ).await;
  a.send_tx(
    "Admin claims their unclaimed balances",
    "151402",
    a.asami_contract().claim_balances()
  ).await;
  a.assert_admin_balances("admin after its own claim.",
    u("0"), u("419998006"), u("99160000000"),
    u("0"), u("128000"), u("0")
  ).await;
}

app_test! { rejects_collabs_when_too_large_or_campaign_is_finished(a)
  let mut advertiser = a.client().await;
  advertiser.setup_as_advertiser("an advertiser for the whole test").await;
  let briefing_hash = u("deadbeef");
  advertiser.pay_campaign("campaign that will be drained", u("1000"), briefing_hash, 2).await;

  let make_collabs_call = |doc_reward: &str| {
    a.asami_contract().admin_make_sub_account_collabs(
      vec![MakeSubAccountCollabsParam{
        advertiser_addr: advertiser.address(),
        briefing_hash,
        collabs: vec![ MakeSubAccountCollabsParamItem{ sub_account_id: wei("1"), doc_reward: u(doc_reward) } ]
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

  a.wait_tx_success("200 DOC collab passes", &txs[0], "95000").await;
  a.wait_tx_failure("400 DOC collab does not pass", &txs[1], "amc4").await;

  let sent_before_end = a.send_without_mining(
    "This tx is sent before campaign ends, but will be mined after campaign ends",
    a.admin_nonce().await,
    vec![make_collabs_call("200")]
  ).await.pop().unwrap();

  a.evm_forward_to_next_cycle().await;

  a.wait_tx_failure("Sent before end, but still fails", &sent_before_end, "amc2").await;

  a.send_revert_tx( "Further attempts get reverted", "amc2", make_collabs_call("50")).await;
}

app_test! { gasless_claim_common_error_cases(a)
  let mut advertiser = a.client().await;
  advertiser.setup_as_advertiser("global advertiser for test").await;
  advertiser.pay_campaign("global campaign for test", u("2000"), u("deadbeef"), 2).await;

  let mut alice = a.client().await;
  alice.make_client_wallet().await;

  a.send_make_collab_tx("A very small collab is registered", "230000", &advertiser, u("deadbeef"), &alice, u("1")).await;

  a.assert_balances_of(
    "Alice's account has pending balances.",
    alice.address(),
    u("10"),
    milli("900"), u("0"),
    u("120"), u("0")
  ).await;

  a.send_revert_tx(
    "The BTC amount to send is less than 1e11",
    "gcb0",
    a.asami_contract().gasless_claim_balances(u("1"), wei("1000000"), vec![alice.address()]).value(wei("1000000")),
  ).await;

  a.send_revert_tx(
    "The BTC value is not exactly what's going to be distributed",
    "gcb1",
    a.asami_contract().gasless_claim_balances(u("1"), u("1"), vec![alice.address()]).value(u("2")),
  ).await;

  a.send_revert_tx(
    "Alice cannot pay the gasless 5 DOC fee yet with only 0.9 DOC",
    "gcb3",
    a.asami_contract().gasless_claim_balances(u("5"), u("1"), vec![alice.address()]).value(u("1")),
  ).await;

  a.send_revert_tx(
    "Alice cannot withdraw, even if fee is as low as 1e-11 DOC, as her account does not allow gasless claims",
    "gcb4",
    a.asami_contract().gasless_claim_balances(wei("10000000"), u("1"), vec![alice.address()]).value(u("1")),
  ).await;

  alice.setup_trusted_admin("Setting up gasless claims and trusted admin for alice").await;

  a.send_tx(
    "Alice can withdraw if fee is just 10000000 DOC sats",
    "257592",
    a.asami_contract().gasless_claim_balances(wei("10000000"), u("1"), vec![alice.address()]).value(u("1")),
  ).await;

  a.assert_balances_of(
    "Alice balances after gasless claim",
    alice.address(),
    wei("10999928884999502195"),
    u("0"), wei("899999999990000000"),
    u("0"), u("120")
  ).await;
}

app_test! { user_can_manage_a_gasless_amount_approval(a)
  let mut advertiser = a.client().await;
  advertiser.setup_as_advertiser("global advertiser for test").await;
  advertiser.pay_campaign("global campaign for test", u("2000"), u("deadbeef"), 2).await;

  let mut alice = a.client().await;
  alice.make_client_wallet().await;

  a.send_tx(
    "Admin promotes sub account, claims trust, but has no allowance as it sent no funds",
    "102000",
    a.asami_contract().promote_sub_accounts(vec![
      PromoteSubAccountsParam{ id: alice.account_id(), addr: alice.address() },
    ])
  ).await;

  let (trusted_admin, max_gasless_doc_to_spend, min_gasless_rbtc_to_receive,_,_,_,_,) = a.asami_contract().accounts(alice.address()).call().await.unwrap();
  assert_eq!(trusted_admin, a.client_admin_address());
  assert_eq!(max_gasless_doc_to_spend, u("1"));
  assert_eq!(min_gasless_rbtc_to_receive, wei("6000000000000"));

  a.send_make_collab_tx("A collab is registered generating rewards", "230000", &advertiser, u("deadbeef"), &alice, u("100")).await;
  a.assert_balances_of(
    "Alice pending balances after collab",
    alice.address(),
    u("10"),
    u("90"), u("0"),
    u("12000"), u("0")
  ).await;

  let expensive_gasless_claim =
    a.asami_contract().gasless_claim_balances(u("10"), u("1"), vec![alice.address()]).value(u("1"));

  a.send_revert_tx( "The admin cannot charge more than the default gasless approval", "gcb4", expensive_gasless_claim.clone()).await;

  a.send_tx(
    "Alice keeps her trusted admin but raises the max doc to pay for gasless",
    "71200",
    alice.asami_contract().config_account(a.client_admin_address(), u("100"), u("0"), u("0"))
  ).await;

  a.send_tx( "The admin can now do an expensive gasless claim.", "257650", expensive_gasless_claim.clone()).await;

  a.assert_balances_of(
    "Alice balances after gasless claim",
    alice.address(),
    wei("10999967872999775111"),
    u("0"), u("80"),
    u("0"), u("12000")
  ).await;

  // Alice can claim her own account and have pending balances that cannot be gasless claimed.
  a.send_make_collab_tx("A second collab for alice", "196000", &advertiser, u("deadbeef"), &alice, u("100")).await;
  a.assert_balances_of(
    "Alice balances after second collab",
    alice.address(),
    wei("10999967872999775111"),
    u("90"), u("80"),
    u("12000"), u("12000")
  ).await;

  a.send_tx(
    "Alice now sets herself as her trusted admin",
    "71200",
    alice.asami_contract().config_account(alice.address(), u("6"), u("0"), u("0"))
  ).await;

  a.send_revert_tx("The admin cannot do gasless claims", "gcb2", expensive_gasless_claim.clone()).await;
}

app_test! { simultaneous_gasless_and_regular_claims_fail(a)
  let mut advertiser = a.client().await;
  advertiser.setup_as_advertiser("global advertiser for test").await;
  advertiser.pay_campaign("global campaign for test", u("2000"), u("deadbeef"), 2).await;

  let mut alice = a.client().await;
  alice.make_client_wallet().await;
  alice.setup_trusted_admin("Alice setting up gasless claims").await;
  a.send_make_collab_tx("A collab is registered generating rewards", "230000", &advertiser, u("deadbeef"), &alice, u("100")).await;
  a.assert_balances_of(
    "Alice has balances to claim",
    alice.address(),
    wei("9999928884999502195"),
    u("90"), u("0"),
    u("12000"), u("0")
  ).await;

  let txs = a.send_without_mining(
    "These can get accepted independentnly, but last one will exceed budget",
    a.admin_nonce().await,
    vec![
      a.asami_contract().admin_claim_balances_free(vec![alice.address()]),
      a.asami_contract().gasless_claim_balances(u("4"), u("1"), vec![alice.address()]).value(u("1")),
    ]
  ).await;

  a.wait_tx_success("regular claim always passes", &txs[0], "200000").await;
  a.wait_tx_failure("gasless claim fails", &txs[1], "gcb3").await;

  a.assert_balances_of(
    "Alice balances after one claim",
    alice.address(),
    wei("9999928884999502195"),
    u("0"), u("90"),
    u("0"), u("12000")
  ).await;
}

app_test! { admin_can_maintain_sub_accounts_for_users(a)
  let briefing_hash = u("deadbeef");
  let mut advertiser = a.client().await;
  advertiser.setup_as_advertiser("global advertiser for test").await;
  advertiser.pay_campaign("global campaign for test", u("2000"), briefing_hash, 2).await;

  let mut alice = a.client().await;
  alice.make_client_wallet().await;

  let alice_sub_account_collab = a.asami_contract().admin_make_sub_account_collabs(
    vec![MakeSubAccountCollabsParam{
      advertiser_addr: advertiser.address(),
      briefing_hash,
      collabs: vec![ MakeSubAccountCollabsParamItem{ sub_account_id: alice.account_id(), doc_reward: u("10") } ]
    }]
  );

  let promote_alice = a.asami_contract().promote_sub_accounts(vec![
    PromoteSubAccountsParam{ id: alice.account_id(), addr: alice.address() },
  ]);

  a.send_tx( "Admin keeps a sub account for alice", "391798", alice_sub_account_collab.clone()).await;

  a.send_tx(
    "Admin promotes sub account, claims trust, but has no allowance as it sent no funds",
    "140000",
    promote_alice.clone()
  ).await;

  a.assert_balances_of(
    "Alice balances after promote",
    alice.address(),
    u("10"),
    u("9"), u("0"),
    u("1200"), u("0")
  ).await;

  a.send_tx( "Admin registers more sub account collabs for alice", "391798", alice_sub_account_collab).await;

  a.assert_balances_of(
    "Alice balance is unchanged when collabs where to a sub account",
    alice.address(),
    u("10"),
    u("9"), u("0"),
    u("1200"), u("0")
  ).await;

  a.send_tx(
    "Admin promotes sub account, claims trust, but has no allowance as it sent no funds",
    "102000",
    promote_alice
  ).await;

  a.assert_balances_of(
    "Alice now received the sub account amounts",
    alice.address(),
    u("10"),
    u("18"), u("0"),
    u("2400"), u("0")
  ).await;
}

app_test! { can_change_admin_for_registering_collabs(a)
  let briefing_hash = u("deadbeef");
  let mut advertiser = a.client().await;
  advertiser.setup_as_advertiser("global advertiser for test").await;
  advertiser.pay_campaign("global campaign for test", u("2000"), briefing_hash, 2).await;
  let mut alice = a.client().await;
  alice.make_client_wallet().await;
  let mut other_admin = a.client().await;
  other_admin.make_client_wallet().await;

  let sub_account_collab = other_admin.asami_contract().admin_make_sub_account_collabs(
    vec![MakeSubAccountCollabsParam{
      advertiser_addr: advertiser.address(),
      briefing_hash,
      collabs: vec![ MakeSubAccountCollabsParamItem{ sub_account_id: alice.account_id(), doc_reward: u("10") } ]
    }]
  );

  let regular_collab = other_admin.asami_contract().admin_make_collabs(
    vec![MakeCollabsParam{
      advertiser_addr: advertiser.address(),
      briefing_hash,
      collabs: vec![ MakeCollabsParamItem{ account_addr: alice.address(), doc_reward: u("10")}]
    }]
  );

  a.send_revert_tx("Sub account collab from new admin fails", "amc1", sub_account_collab.clone()).await;
  a.send_revert_tx("Regular collab from new admin fails", "amc1", regular_collab.clone()).await;

  a.send_tx(
    "Advertiser now changes his trustetd admin",
    "71200",
    advertiser.asami_contract().config_account(other_admin.address(), u("100"), u("0"), u("0"))
  ).await;

  a.send_tx("Sub account collab from new admin works", "230000", sub_account_collab).await;
  a.send_tx("Regular collab from admin now works", "127386", regular_collab).await;
}
