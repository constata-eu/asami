use api::on_chain::{MakeCollabsParam, MakeCollabsParamItem, SubmitReportsParam};

use super::*;

app_test! { full_campaign_workflow_until_reimbursed (a)
    let mut advertiser = a.client().await;
    let campaign = u("deadbeef");
    advertiser.setup_as_advertiser_with_amount("global advertiser for test", u("10000")).await;
    let account = advertiser.address();
    advertiser.pay_campaign("global campaign for test", u("2000"), campaign, 10).await;

    let mut alice = a.client().await;
    alice.make_client_wallet().await;
    a.send_make_collab_tx("a large collab drains all the funds", "226000", &advertiser, campaign, &alice, u("2000")).await;

    let attrs = a.get_campaign(account, campaign).await;
    assert_eq!(attrs.budget, u("0"));
    assert_eq!(attrs.report_hash, u("0"));

    a.send_tx("campaign is topped up", "78582", advertiser.top_up_campaign_contract_call(account, campaign, u("100"))).await;

    assert_eq!(a.get_campaign(account, campaign).await.budget, u("100"));

    a.send_make_collab_tx("a new smaller collab", "93198", &advertiser, campaign, &alice, u("50")).await;
    let attrs = a.get_campaign(account, campaign).await;
    assert_eq!(attrs.budget, u("50"));

    a.evm_forward_to_next_cycle().await;

    a.send_tx("Campaign is reimbursed", "50923",
        advertiser.reimburse_campaign_contract_call(account, campaign)
    ).await;

    let attrs = a.get_campaign(account, campaign).await;
    assert_eq!(attrs.budget, u("0"));

    a.send_tx("admin submits report", "53200", a.submit_report_contract_call(account, campaign, u("1"))).await;

    let attrs = a.get_campaign(account, campaign).await;
    assert_eq!(attrs.report_hash, u("1"));

    a.send_revert_tx("no collab allowed", "amc2", a.asami_contract().admin_make_collabs(vec![
    MakeCollabsParam{
      advertiser_addr: advertiser.address(),
      briefing_hash: campaign,
      collabs: vec![ MakeCollabsParamItem{ account_addr: alice.address(), doc_reward: u("20")}]
    }
    ])).await;

    a.send_tx("campaign is extended", "36314", advertiser.extend_campaign_contract_call(campaign, 20)).await;

    a.send_tx("campaign is topped up again", "78582",
        advertiser.top_up_campaign_contract_call(account, campaign, u("100"))
    ).await;

    a.send_make_collab_tx("a new smaller collab", "132000", &advertiser, campaign, &alice, u("20")).await;

    assert_eq!(a.doc_balance_of(&account).await, u("7850"));

    a.evm_forward_to_next_cycle().await;
    a.send_tx("Campaign is reimbursed again", "47200", advertiser.reimburse_campaign_contract_call(account, campaign)).await;

    assert_eq!(a.doc_balance_of(&account).await, u("7930"));

    let attrs = a.get_campaign(account, campaign).await;
    assert_eq!(attrs.budget, u("0"));
}

app_test! { campaign_creation_validations (a)
  let campaign = u("deadbeef");

  let mut advertiser = a.client().await;
  advertiser.make_client_wallet().await;

  let valid_pay_campaign_call = advertiser.pay_campaign_contract_call(u("1000"), campaign, 1);

  a.send_revert_tx("theres' no allowance", "ERC20: insufficient allowance", valid_pay_campaign_call.clone()).await;

  a.send_tx(
    "approving spending for creating campaign",
    "46296",
    advertiser.doc_contract().approve(a.asami_contract().address(), u("100000"))
  ).await;

  a.send_revert_tx("there's no DOC", "ERC20: transfer amount exceeds balance", valid_pay_campaign_call.clone()).await;

  a.send_doc_to(advertiser.address(), u("2000")).await;

  a.send_revert_tx("needs budget > 0", "mc0", advertiser.pay_campaign_contract_call(u("0"), campaign, 1)).await;

  a.send_revert_tx("can't be in the past", "mc1", advertiser.pay_campaign_contract_call(u("1000"), campaign, -1)).await;

  a.send_tx("finally it's ok", "117846", valid_pay_campaign_call.clone()).await;
}

app_test! { campaign_extension_validations (a)
  let mut advertiser = a.client().await;
  let campaign = u("deadbeef");
  advertiser.setup_as_advertiser_with_amount("global advertiser for test", u("10000")).await;
  advertiser.pay_campaign("global campaign expiring in 10 days", u("2000"), campaign, 10).await;

  let mut other = a.client().await;
  other.setup_as_advertiser_with_amount("stranger for authorization test", u("10000")).await;

  let mut stranger = a.client().await;
  stranger.make_client_wallet().await;

  a.send_revert_tx("can't extend unknown campaign", "ec1",
    advertiser.extend_campaign_contract_call(u("333"), 20)
  ).await;

  a.send_revert_tx("only owner can extend", "ec1",
    other.extend_campaign_contract_call(campaign, 20)
  ).await;

  a.send_revert_tx("can't propose 0 expiration", "ec2",
    advertiser.extend_campaign_contract_call(campaign, 0)
  ).await;

  a.send_revert_tx("can't make campaign end sooner", "ec2",
    advertiser.extend_campaign_contract_call(campaign, 1)
  ).await;

  a.send_tx("can extend before due date", "38314",
    advertiser.extend_campaign_contract_call(campaign, 20)
  ).await;
  assert!(a.get_campaign(advertiser.address(), campaign).await.valid_until > a.future_date(19));
}

app_test! { campaign_top_up_tests (a)
  let mut advertiser = a.client().await;
  let campaign = u("deadbeef");
  advertiser.setup_as_advertiser_with_amount("global advertiser for test", u("10000")).await;
  let account = advertiser.address();
  advertiser.pay_campaign("global campaign expiring in 10 days", u("2000"), campaign, 10).await;

  let mut other = a.client().await;
  other.setup_as_advertiser_with_amount("stranger for authorization test", u("10000")).await;

  let mut stranger = a.client().await;
  stranger.make_client_wallet().await;

  a.send_revert_tx("can't top-up a non existing campaign", "tc2",
    advertiser.top_up_campaign_contract_call(account, u("111"), u("10"))
  ).await;

  a.send_revert_tx("can't top-up with cero", "tc0",
    advertiser.top_up_campaign_contract_call(account, campaign, u("0"))
  ).await;

  a.send_tx("can top-up someone else's campaign", "61500",
    other.top_up_campaign_contract_call(account, campaign, u("10"))
  ).await;
  assert_eq!(a.get_campaign(account, campaign).await.budget, u("2010"));

  a.evm_forward_to_next_cycle().await;

  a.send_revert_tx("can top-up after done", "tc2",
    other.top_up_campaign_contract_call(account, campaign, u("10"))
  ).await;

  assert_eq!(a.get_campaign(account, campaign).await.budget, u("2010"));
}

app_test! { campaign_submit_report_tests (a)
  let mut advertiser = a.client().await;
  let campaign = u("deadbeef");
  advertiser.setup_as_advertiser_with_amount("global advertiser for test", u("10000")).await;
  let account = advertiser.address();
  advertiser.pay_campaign("global campaign expiring in 10 days", u("2000"), campaign, 10).await;

  let mut stranger = a.client().await;
  stranger.make_client_wallet().await;

  a.send_revert_tx("can't submit report before campaign finishes", "sr3",
    a.submit_report_contract_call(account, campaign, u("1000"))
  ).await;

  a.evm_forward_to_next_cycle().await;

  a.send_revert_tx("can't submit empty report", "sr0",
    a.submit_report_contract_call(account, campaign, u("0"))
  ).await;

  a.send_revert_tx("can't submit report for unknown user", "sr1",
    a.submit_report_contract_call(stranger.address(), campaign, u("1000"))
  ).await;

  a.send_revert_tx("can't submit report for unknown campaign", "sr2",
    a.submit_report_contract_call(account, u("123"), u("1000"))
  ).await;

  a.send_revert_tx("only admin can submit report", "sr1",
    advertiser.asami_contract().submit_reports(
      vec![ SubmitReportsParam{ account, briefing_hash: campaign, report_hash: u("1000") }]
    )
  ).await;

  a.send_tx("can submit correct report", "53200",
    a.submit_report_contract_call(account, campaign, u("1000"))
  ).await;

  assert_eq!(a.get_campaign(account, campaign).await.report_hash, u("1000"));

  a.send_revert_tx("can't submit report if already present", "sr4",
    a.submit_report_contract_call(account, campaign, u("2000"))
  ).await;

  assert_eq!(a.get_campaign(account, campaign).await.report_hash, u("1000"));
}

app_test! { campaign_reimbursement_tests (a)
  let mut advertiser = a.client().await;
  let campaign = u("deadbeef");
  advertiser.setup_as_advertiser_with_amount("global advertiser for test", u("10000")).await;
  advertiser.pay_campaign("global campaign expiring in 10 days", u("2000"), campaign, 10).await;
  let account = advertiser.address();

  let mut other = a.client().await;
  other.setup_as_advertiser_with_amount("stranger for authorization test", u("10000")).await;

  let mut stranger = a.client().await;
  stranger.make_client_wallet().await;

  a.send_revert_tx("cannot reimburse before campaign finishes", "rc1",
    stranger.reimburse_campaign_contract_call(account, campaign)
  ).await;

  a.evm_forward_to_next_cycle().await;

  a.send_revert_tx("cannot reimburse for unknown account", "rc0",
    stranger.reimburse_campaign_contract_call(other.address(), campaign)
  ).await;


  assert_eq!(a.get_campaign(account, campaign).await.budget, u("2000"));

  assert_eq!(a.doc_balance_of(&account).await, u("8000"));
  a.send_tx("anyone can reimburse", "43800",
    stranger.reimburse_campaign_contract_call(account, campaign)
  ).await;
  assert_eq!(a.get_campaign(account, campaign).await.budget, u("0"));
  assert_eq!(a.doc_balance_of(&account).await, u("10000"));

  a.send_revert_tx("cannot reimburse again", "rc0",
    stranger.reimburse_campaign_contract_call(account, campaign)
  ).await;
  assert_eq!(a.get_campaign(account, campaign).await.budget, u("0"));
}
