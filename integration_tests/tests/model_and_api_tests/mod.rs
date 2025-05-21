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




Test helper:
* Is the global test helper, main entry point. We always make one of this.
* Has a single API server running.
* Has a single selenium session. Can login any "TestUser" to the web.
* Has an internal connection to the APP.
* Does not have a default 'User'. This is all admin side.
    * It does not know about creating handles or any of that. That's the user's responsibility.
* Has methods to create TestUsers as needed.
* It can create the selenium instance on demand. Some tests may not need it.
* If no users are needed, then no api server is needed either.
* Users can be created without being logged in to the api. In that case they just have a wallet.


TestUser:
* Has keys to interact with the contracts.
* Has a an Arc linking back to the app in case something from it is needed internally.
    Internal access to the APP model to bypass calling methods in the API and to wait for it's own side effects.
* Does not know about selenium. It can share its access credentials with the selenium runner though.


*/
