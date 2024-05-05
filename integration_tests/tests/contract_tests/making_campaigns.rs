use ::api::{models::{U256, AsamiFunctionCall}, on_chain::*};
use crate::support::{TestApp, ApiClient};

app_test!{ full_campaign_workflow_until_reimbursed (a) 
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

  a.send_tx("campaign is topped up", "78582", make_top_up_campaign_call(&advertiser, account, campaign, u("100"))).await;

  assert_eq!(a.get_campaign(account, campaign).await.budget, u("100"));

  a.send_make_collab_tx("a new smaller collab", "93198", &advertiser, campaign, &alice, u("50")).await;
  let attrs = a.get_campaign(account, campaign).await;
  assert_eq!(attrs.budget, u("50"));

  a.evm_forward_to_next_cycle().await;

  a.send_tx("Campaign is reimbursed", "50923", 
    make_reimburse_campaign_call(&advertiser, account, campaign)
  ).await;

  let attrs = a.get_campaign(account, campaign).await;
  assert_eq!(attrs.budget, u("0"));

  a.send_tx("admin submits report", "53200", make_submit_report_call(&a, account, campaign, u("1"))).await;

  let attrs = a.get_campaign(account, campaign).await;
  assert_eq!(attrs.report_hash, u("1"));

  a.send_revert_tx("no collab allowed", "amc2", a.asami_contract().admin_make_collabs(vec![
    MakeCollabsParam{
      advertiser_addr: advertiser.address(),
      briefing_hash: campaign,
      collabs: vec![ MakeCollabsParamItem{ account_addr: alice.address(), doc_reward: u("20")}]
    }
  ])).await;

  a.send_tx("campaign is extended", "36314", make_extend_campaign_call(&advertiser, campaign, 20)).await;

  a.send_tx("campaign is topped up again", "78582", 
    make_top_up_campaign_call(&advertiser, account, campaign, u("100"))
  ).await;

  a.send_make_collab_tx("a new smaller collab", "132000", &advertiser, campaign, &alice, u("20")).await;

  a.evm_forward_to_next_cycle().await;
  a.send_tx("Campaign is reimbursed again", "43326", make_reimburse_campaign_call(&advertiser, account, campaign)).await;

  let attrs = a.get_campaign(account, campaign).await;
  assert_eq!(attrs.budget, u("0"));
}

app_test!{ campaign_creation_validations (a) 
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


app_test!{ campaign_extension_validations (a)
  let mut advertiser = a.client().await;
  let campaign = u("deadbeef");
  advertiser.setup_as_advertiser_with_amount("global advertiser for test", u("10000")).await;
  advertiser.pay_campaign("global campaign expiring in 10 days", u("2000"), campaign, 10).await;

  let mut other = a.client().await;
  other.setup_as_advertiser_with_amount("stranger for authorization test", u("10000")).await;

  let mut stranger = a.client().await;
  stranger.make_client_wallet().await;

  a.send_revert_tx("can't extend unknown campaign", "ec1",
    make_extend_campaign_call(&advertiser, u("333"), 20)
  ).await;

  a.send_revert_tx("only owner can extend", "ec1",
    make_extend_campaign_call(&other, campaign, 20)
  ).await;

  a.send_revert_tx("can't propose 0 expiration", "ec2",
    make_extend_campaign_call(&advertiser, campaign, 0)
  ).await;

  a.send_revert_tx("can't make campaign end sooner", "ec2",
    make_extend_campaign_call(&advertiser, campaign, 1)
  ).await;

  a.send_tx("can extend before due date", "38314", 
    make_extend_campaign_call(&advertiser, campaign, 20)
  ).await;
  assert!(a.get_campaign(advertiser.address(), campaign).await.valid_until > a.future_date(19));
}

app_test!{ campaign_top_up_tests (a)
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
    make_top_up_campaign_call(&advertiser, account, u("111"), u("10"))
  ).await;

  a.send_revert_tx("can't top-up with cero", "tc0",
    make_top_up_campaign_call(&advertiser, account, campaign, u("0"))
  ).await;

  a.send_tx("can top-up someone else's campaign", "61500",
    make_top_up_campaign_call(&other, account, campaign, u("10"))
  ).await;
  assert_eq!(a.get_campaign(account, campaign).await.budget, u("2010"));

  a.evm_forward_to_next_cycle().await;

  a.send_revert_tx("can top-up after done", "tc2",
    make_top_up_campaign_call(&other, account, campaign, u("10"))
  ).await;

  assert_eq!(a.get_campaign(account, campaign).await.budget, u("2010"));
}

app_test!{ campaign_submit_report_tests (a)
  let mut advertiser = a.client().await;
  let campaign = u("deadbeef");
  advertiser.setup_as_advertiser_with_amount("global advertiser for test", u("10000")).await;
  let account = advertiser.address();
  advertiser.pay_campaign("global campaign expiring in 10 days", u("2000"), campaign, 10).await;

  let mut stranger = a.client().await;
  stranger.make_client_wallet().await;

  a.send_revert_tx("can't submit report before campaign finishes", "sr3",
    make_submit_report_call(&a, account, campaign, u("1000"))
  ).await;

  a.evm_forward_to_next_cycle().await;

  a.send_revert_tx("can't submit empty report", "sr0",
    make_submit_report_call(&a, account, campaign, u("0"))
  ).await;

  a.send_revert_tx("can't submit report for unknown user", "sr1",
    make_submit_report_call(&a, stranger.address(), campaign, u("1000"))
  ).await;

  a.send_revert_tx("can't submit report for unknown campaign", "sr2",
    make_submit_report_call(&a, account, u("123"), u("1000"))
  ).await;

  a.send_revert_tx("only admin can submit report", "sr1",
    advertiser.asami_contract().submit_reports(
      vec![ SubmitReportsParam{ account: account, briefing_hash: campaign, report_hash: u("1000") }]
    )
  ).await;

  a.send_tx("can submit correct report", "53200",
    make_submit_report_call(&a, account, campaign, u("1000"))
  ).await;

  assert_eq!(a.get_campaign(account, campaign).await.report_hash, u("1000"));

  a.send_revert_tx("can't submit report if already present", "sr4",
    make_submit_report_call(&a, account, campaign, u("2000"))
  ).await;

  assert_eq!(a.get_campaign(account, campaign).await.report_hash, u("1000"));
}

app_test!{ campaign_reimbursement_tests (a)
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
    make_reimburse_campaign_call(&stranger, account, campaign)
  ).await;

  a.evm_forward_to_next_cycle().await;

  a.send_revert_tx("cannot reimburse for unknown account", "rc0",
    make_reimburse_campaign_call(&stranger, other.address(), campaign)
  ).await;
  assert_eq!(a.get_campaign(account, campaign).await.budget, u("2000"));

  a.send_tx("anyone can reimburse", "43326",
    make_reimburse_campaign_call(&stranger, account, campaign)
  ).await;
  assert_eq!(a.get_campaign(account, campaign).await.budget, u("0"));

  a.send_revert_tx("cannot reimburse again", "rc0",
    make_reimburse_campaign_call(&stranger, account, campaign)
  ).await;
  assert_eq!(a.get_campaign(account, campaign).await.budget, u("0"));
}

fn make_extend_campaign_call(a: &ApiClient, campaign: U256, valid_until: i64) -> AsamiFunctionCall {
  a.asami_contract().extend_campaigns(vec![
    ExtendCampaignsParam{
      briefing_hash: campaign,
      valid_until: a.test_app.future_date(valid_until),
    },
  ])
}

fn make_top_up_campaign_call(a: &ApiClient, account: Address, campaign: U256, budget: U256) -> AsamiFunctionCall {
  a.asami_contract().top_up_campaigns(vec![
    TopUpCampaignsParam{ account, briefing_hash: campaign, budget, },
  ])
}

fn make_submit_report_call(a: &TestApp, account: Address, campaign: U256, report_hash: U256) -> AsamiFunctionCall {
  a.asami_contract().submit_reports( vec![ SubmitReportsParam{ account, briefing_hash: campaign, report_hash} ])
}

fn make_reimburse_campaign_call(a: &ApiClient, addr: Address, briefing_hash: U256) -> AsamiFunctionCall {
  a.asami_contract().reimburse_campaigns( vec![ ReimburseCampaignsParam{ addr, briefing_hash} ])
}

// Only trusted admin can make collabs 
// Only trusted admin can make subaccount collabs.
// mc0 error now is campaigns are too high.
//   a.send_revert_tx("advertiser has not claimed their account yet", "mc0", valid_pay_campaign_call.clone()).await;


