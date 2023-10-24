#[macro_use]
mod support;
use ::api::models::*;
use graphql_client::GraphQLQuery;
use support::gql::*;
use rocket::http::Header;
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};


/*
 Creates account.
 Participates in campaign.
 Gets paid.
 Claims account.
 Moves money out of account.

 -- Use the API calls instead of models for all interactions.
*/


api_test!{ signs_up (test_app, client)

  let value = "test+token";
  test_app.app.one_time_token().insert(InsertOneTimeToken{
    value: value.to_string()
  }).save().await?;

  let login_pubkey = URL_SAFE_NO_PAD.encode(
    test_app.private_key().public_key().to_pem().unwrap()
  );

  let _created: create_session::ResponseData = client.gql(
    &CreateSession::build_query(create_session::Variables{}),
    vec![
      Header::new("Auth-Action", "Login"),
      Header::new("Auth-Method-Kind", "OneTimeToken"),
      Header::new("Auth-Data", value),
      Header::new("Login-Pubkey", login_pubkey),
      Header::new("New-Session-Recaptcha-Code", "SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c"),
    ]
  ).await;

  let account = test_app.app.account().find(1).await?;

  let mut handle = account.add_handle(Site::X, "nubis_bruno").await?;

  test_app.app.handle().verify_and_appraise_x().await.unwrap();

  handle.reload().await?;

  assert_that!(&handle.attrs, structure![ HandleAttrs {
    status: eq(HandleStatus::Appraised),
    fixed_id: maybe_some(rematch("179383862")),
    verification_message_id: maybe_some(rematch("1713930237598150807")),
    //score: maybe_some(eq(Decimal::new(3393, 0)))
  }]);

  let submitted = handle.submit().await?;

  assert_that!(&submitted.attrs, structure![ HandleAttrs {
    status: eq(HandleStatus::Submitted),
    tx_hash: maybe_some(any_value())
  }]);

  tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

  test_app.app.sync_on_chain_events().await?;

  assert_that!(&submitted.reloaded().await?.attrs, structure![ HandleAttrs {
    status: eq(HandleStatus::Active),
  }]);

  //"https://x.com/asami_club/status/1716421161867710954?s=20",
  let campaign_req = account.create_campaign_request(
    Site::X,
    "1716421161867710954",
    Decimal::new(10,0),
  ).await?
  .pay().await?
  .submit().await?;

  test_app.app.sync_on_chain_events().await.unwrap();

  let campaign = test_app.app.campaign().select().one().await.unwrap();

  assert_that!(&campaign.attrs, structure![ CampaignAttrs {
    budget: eq(Decimal::new(10,0))
  }]);

  test_app.app.campaign().sync_x_collabs().await.unwrap();

  assert!(test_app.app.collab_request().select().count().await? == 1);

  todo!("If we're here then we have an active handle properly synced on-chain.");

  // Sign up an advertiser too.
  // Make the advertiser create a campaign, and sync it on-chain
  
  // https://x.com/asami_club/status/1716421161867710954?s=20
  //
  // Monitor the twitter URL retweets to know if they have retweeted it.
  //
  // Validate collab and Sync the collab on-chain:
  //   -- Check the sender is a registered account.
  //   -- Check 

  /*

  use gql::{
    *,
    create_attestation as create,
    attestation as show,
    all_attestations as all,
    attestation_html_export as export,
    attestation_set_published as publish,
  };

  client.signer.verify_email("test@example.com").await;

  let vars = create::Variables{
    input: create::AttestationInput {
      documents: vec![
        client.signer.signed_payload(b"hello world").into(),
        client.signer.signed_payload(b"goodbye world").into(),
      ],
      open_until: Some(chrono::Utc.with_ymd_and_hms(2050, 1, 1, 1, 1, 1).unwrap()),
      markers: Some("foo bar baz".to_string()),
      email_admin_access_url_to: vec!["foo@example.com".to_string(), "bar@example.com".to_string()]
    }
  };

  let created: create::ResponseData = client.gql(&CreateAttestation::build_query(vars)).await;
  */

}

/*
browser_test!{ logs_in_with_one_time_token (app, selenium)
  todo!("Fail here");
}
*/

/*
 * Create the initial campaign topics.
 *
 * Login to the API with one time link, as an advertiser and start a campaign.
 *
 * Login as an advocate, create a profile, link social networks (by posting the example viral campaign).
 *
 * Participate in the recently created campaign.
 *
 * >>> time goes by
 *
 * Get paid for participating in the campaign.
 *
 * Convert the paid money into tokens 
 *
*/
