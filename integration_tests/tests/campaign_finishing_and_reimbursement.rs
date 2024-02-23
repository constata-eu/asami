#[macro_use]
pub mod support;
use api::on_chain::Signer;

app_test!{ reimburses_to_admin (a) 
  assert_eq!( a.admin_doc_balance().await, u("420000000"));
  let mut advertiser = a.client().await;
  let campaign = advertiser.create_x_campaign(u("10"), u("10")).await;

  assert_eq!(a.admin_doc_balance().await,     wei("419999990000000000000000000"));
  assert_eq!(a.contract_doc_balance().await,  wei(       "10000000000000000000"));

  let mut bob = a.client().await;
  bob.create_x_handle("bob_on_x", u("1")).await;
  bob.create_x_collab(&campaign).await;

  assert_eq!(a.admin_doc_balance().await,     wei("419999990100000000000000000"));
  assert_eq!(a.contract_doc_balance().await,  wei(        "9900000000000000000"));

  bob.claim_account().await;
  advertiser.claim_account().await;

  assert_eq!(a.admin_doc_balance().await,     wei("419999990100000000000000000"));
  assert_eq!(a.contract_doc_balance().await,  wei(        "9000000000000000000"));
  assert_eq!(bob.doc_balance().await,         wei(         "900000000000000000"));
  assert_eq!(advertiser.doc_balance().await,  wei(                          "0"));

  a.evm_forward_to_next_cycle().await;
  a.run_idempotent_background_tasks_a_few_times().await;

  // Reimbursement went to admin, even when advertiser has claimed its account.
  assert_eq!(a.admin_doc_balance().await,     wei("419999999100000000000000000"));
  assert_eq!(a.contract_doc_balance().await,  wei(                          "0"));
  assert_eq!(bob.doc_balance().await,         wei(         "900000000000000000"));
  assert_eq!(advertiser.doc_balance().await,  wei(                          "0"));
}

app_test!{ reimburses_to_advertiser (a) 
  assert_eq!( a.admin_doc_balance().await, u("420000000"));

  let mut advertiser = a.client().await;
  advertiser.claim_account().await;
  a.doc_contract().transfer(advertiser.local_wallet().address(), u("100"))
    .send().await.unwrap().await.unwrap().unwrap();

  assert_eq!(a.contract_doc_balance().await, u(        "0"));
  assert_eq!(a.admin_doc_balance().await,    u("419999900"));
  assert_eq!(advertiser.doc_balance().await, u(      "100"));

  let campaign = advertiser.create_self_managed_x_campaign(u("10"), u("10"), 2).await;
  assert_eq!( a.contract_doc_balance().await, u("10"));
  assert_eq!( advertiser.doc_balance().await, u("90"));

  let mut bob = a.client().await;
  bob.create_x_handle("bob_on_x", u("1")).await;
  bob.create_x_collab(&campaign).await;
  bob.claim_account().await;

  assert_eq!(a.admin_doc_balance().await,     wei("419999900088235294117647058"));
  assert_eq!( a.contract_doc_balance().await, wei(        "9000000000000000001"));
  assert_eq!( bob.doc_balance().await,        wei(         "900000000000000000"));
  assert_eq!( advertiser.doc_balance().await, wei(       "90011764705882352941"));

  a.evm_forward_to_next_cycle().await;
  a.run_idempotent_background_tasks_a_few_times().await;

  assert_eq!(a.admin_doc_balance().await,     wei("419999900088235294117647058"));
  assert_eq!( a.contract_doc_balance().await, wei(                          "1"));
  assert_eq!( bob.doc_balance().await,        wei(         "900000000000000000"));
  assert_eq!( advertiser.doc_balance().await, wei(       "99011764705882352941"));
}
