pub use integration_tests::{
    api_test, app_test, browser_test,
    support::{self, *},
    test,
};
pub mod apply_voted_fee_rate;
pub mod claim_fee_pool_share;
pub mod distribute_fees_job;
pub mod handle_scoring;
/*
pub mod full_campaign_workflow;
pub mod gasless_claims;
*/
/*
pub mod making_collabs;
pub mod on_chain_jobs;
pub mod one_time_tokens;
*/

/*
app_test! { test_claim_account_in_its_own_test(a)
    todo!("fail here");
}

app_test! { avoid_race_condition_detecting_problems_with_claiming_an_account
}
/*
#[macro_use]
use ethers::signers::Signer;
app_test! { distributes_fees_to_holders (_a)
    todo!("Make assertions about fee cycles and fees at");
    todo!("Try to have someone collect twice");
}
*/
*/

/*

Maybe "quick collab" ?

    let mut alice = h.user().await;
    alice.claim_account().await;
    let handle = alice.create_handle("alice_on_x", "11111", wei("100")).await;
    campaign.make_collab(&handle, u("100"), "first_collab").await.unwrap();



*/
