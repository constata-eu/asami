use ::api::{models::{U256, AsamiFunctionCall}, on_chain::*};
use crate::support::{TestApp, ApiClient};

app_test!{ testing_pending (a) 
    todo!("uncomment this one");
}
/*
app_test!{ full_campaign_workflow_until_reimbursed (a) 
  let mut advertiser = a.client().await;
  let account = advertiser.account_id();
  let campaign = u("deadbeef");

  advertiser.setup_as_advertiser_with_amount("global advertiser for test", u("10000")).await;
  advertiser.make_campaign("global campaign for test", u("2000"), campaign, 10).await;

  let mut alice = a.client().await;
  alice.make_client_wallet().await;
  a.send_make_collab_tx("a large collab drains all the funds", "191103", &advertiser, campaign, &alice, u("2000")).await;

  let attrs = a.get_campaign(account, campaign).await;
  assert_eq!(attrs.budget, u("0"));
  assert_eq!(attrs.report_hash, u("0"));

  a.send_tx("campaign is topped up", "78582", make_top_up_campaign_call(&advertiser, account, campaign, u("100"))).await;

  assert_eq!(a.get_campaign(account, campaign).await.budget, u("100"));

  a.send_make_collab_tx("a new smaller collab", "76203", &advertiser, campaign, &alice, u("50")).await;
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

  a.send_revert_tx("no collab allowed", "amc1", a.asami_core().admin_make_collabs(vec![
    MakeCollabsParam{
      advertiser_id: advertiser.account_id(),
      briefing: campaign,
      collabs: vec![ MakeCollabsParamItem{ account_id: alice.account_id(), doc_reward: u("20")}]
    }
  ])).await;

  a.send_tx("campaign is extended", "36314", make_extend_campaign_call(&advertiser, campaign, 20)).await;

  a.send_tx("campaign is topped up again", "78582", 
    make_top_up_campaign_call(&advertiser, account, campaign, u("100"))
  ).await;

  a.send_make_collab_tx("a new smaller collab", "93303", &advertiser, campaign, &alice, u("20")).await;

  a.evm_forward_to_next_cycle().await;
  a.send_tx("Campaign is reimbursed again", "43326", make_reimburse_campaign_call(&advertiser, account, campaign)).await;

  let attrs = a.get_campaign(account, campaign).await;
  assert_eq!(attrs.budget, u("0"));
}

app_test!{ campaign_creation_validations (a) 
  let campaign = u("deadbeef");

  let mut advertiser = a.client().await;
  advertiser.make_client_wallet().await;

  let valid_make_campaign_call = advertiser.make_campaign_contract_call(u("1000"), campaign, 1);

  a.send_revert_tx("advertiser has not claimed their account yet", "mc0", valid_make_campaign_call.clone()).await;

  a.send_tx(
    "advertiser claims account, and stops being a stranger",
    "94550",
    a.asami_core().promot_sub_accounts(vec![
      PromoteSubAccountsParam{ id: advertiser.account_id(), addr: advertiser.address() },
    ])
  ).await;

  a.send_revert_tx("theres' no allowance", "ERC20: insufficient allowance", valid_make_campaign_call.clone()).await;

  a.send_tx(
    "approving spending for creating campaign",
    "46296",
    advertiser.doc_contract().approve(a.asami_core().address(), u("100000"))
  ).await;

  a.send_revert_tx("there's no DOC", "ERC20: transfer amount exceeds balance", valid_make_campaign_call.clone()).await;

  a.send_doc_to(advertiser.address(), u("2000")).await;

  a.send_revert_tx("needs budget > 0", "mc1", advertiser.make_campaign_contract_call(u("0"), campaign, 1)).await;

  a.send_revert_tx("can't be in the past", "mc2", advertiser.make_campaign_contract_call(u("1000"), campaign, -1)).await;

  a.send_tx("finally it's ok", "117846", valid_make_campaign_call.clone()).await;
}

app_test!{ campaign_extension_validations (a)
  let mut advertiser = a.client().await;
  let account = advertiser.account_id();
  let campaign = u("deadbeef");
  advertiser.setup_as_advertiser_with_amount("global advertiser for test", u("10000")).await;
  advertiser.make_campaign("global campaign expiring in 10 days", u("2000"), campaign, 10).await;

  let mut other = a.client().await;
  other.setup_as_advertiser_with_amount("stranger for authorization test", u("10000")).await;

  let mut stranger = a.client().await;
  stranger.make_client_wallet().await;
  a.send_revert_tx("can't use unknown account", "ec0",
    make_extend_campaign_call(&stranger, campaign, 0)
  ).await;

  a.send_revert_tx("can't propose 0 expiration", "ec3",
    make_extend_campaign_call(&advertiser, campaign, 0)
  ).await;

  a.send_revert_tx("can't extend unknown campaign", "ec2",
    make_extend_campaign_call(&advertiser, u("333"), 20)
  ).await;

  a.send_revert_tx("only owner can extend", "ec2",
    make_extend_campaign_call(&other, campaign, 20)
  ).await;

  a.send_revert_tx("can't make campaign end sooner", "ec3",
    make_extend_campaign_call(&advertiser, campaign, 1)
  ).await;

  a.send_tx("can extend before due date", "38314", 
    make_extend_campaign_call(&advertiser, campaign, 20)
  ).await;
  assert!(a.get_campaign(account, campaign).await.valid_until > a.future_date(19));
}

app_test!{ campaign_top_up_tests (a)
  let mut advertiser = a.client().await;
  let account = advertiser.account_id();
  let campaign = u("deadbeef");
  advertiser.setup_as_advertiser_with_amount("global advertiser for test", u("10000")).await;
  advertiser.make_campaign("global campaign expiring in 10 days", u("2000"), campaign, 10).await;

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
  let account = advertiser.account_id();
  let campaign = u("deadbeef");
  advertiser.setup_as_advertiser_with_amount("global advertiser for test", u("10000")).await;
  advertiser.make_campaign("global campaign expiring in 10 days", u("2000"), campaign, 10).await;

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
    make_submit_report_call(&a, stranger.account_id(), campaign, u("1000"))
  ).await;

  a.send_revert_tx("can't submit report for unknown campaign", "sr2",
    make_submit_report_call(&a, account, u("123"), u("1000"))
  ).await;

  a.send_revert_tx("only admin can submit report", "no_revert_error",
    advertiser.asami_contract().submit_reports(
      vec![ SubmitReportsParam{ account_id: account, briefing: campaign, report_hash: u("1000") }]
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
  let account = advertiser.account_id();
  let campaign = u("deadbeef");
  advertiser.setup_as_advertiser_with_amount("global advertiser for test", u("10000")).await;
  advertiser.make_campaign("global campaign expiring in 10 days", u("2000"), campaign, 10).await;

  let mut stranger = a.client().await;
  stranger.make_client_wallet().await;

  a.send_revert_tx("cannot reimburse before campaign finishes", "rc3",
    make_reimburse_campaign_call(&stranger, account, campaign)
  ).await;

  a.evm_forward_to_next_cycle().await;

  a.send_revert_tx("cannot reimburse for unknown account", "rc1",
    make_reimburse_campaign_call(&stranger, u("555"), campaign)
  ).await;
  assert_eq!(a.get_campaign(account, campaign).await.budget, u("2000"));

  a.send_tx("anyone can reimburse", "43326",
    make_reimburse_campaign_call(&stranger, account, campaign)
  ).await;
  assert_eq!(a.get_campaign(account, campaign).await.budget, u("0"));

  a.send_revert_tx("cannot reimburse again", "rc2",
    make_reimburse_campaign_call(&stranger, account, campaign)
  ).await;
  assert_eq!(a.get_campaign(account, campaign).await.budget, u("0"));
}

fn make_extend_campaign_call(a: &ApiClient, campaign: U256, valid_until: i64) -> AsamiFunctionCall {
  a.asami_contract().extend_campaigns(vec![
    ExtendCampaignsParam{
      briefing: campaign,
      valid_until: a.test_app.future_date(valid_until),
    },
  ])
}

fn make_top_up_campaign_call(a: &ApiClient, account_id: U256, campaign: U256, budget: U256) -> AsamiFunctionCall {
  a.asami_contract().top_up_campaigns(vec![
    TopUpCampaignsParam{ account_id, briefing: campaign, budget, },
  ])
}

fn make_submit_report_call(a: &TestApp, account_id: U256, campaign: U256, report_hash: U256) -> AsamiFunctionCall {
  a.asami_core().submit_reports( vec![ SubmitReportsParam{ account_id, briefing: campaign, report_hash} ])
}

fn make_reimburse_campaign_call(a: &ApiClient, account_id: U256, briefing: U256) -> AsamiFunctionCall {
  a.asami_contract().reimburse_campaigns( vec![ ReimburseCampaignsParam{ account_id, briefing} ])
}

*/

// cannot reimburse before due date.
// cannot reimburse an unknown campaign - cannot reimburse for unknown account id.
// cannot reimburse if balance is empty
// anyone can trigger reimbursement.
