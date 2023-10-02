#[macro_use]
mod support;
use ::api::models::*;
use graphql_client::GraphQLQuery;
use support::gql::*;
use rocket::http::Header;
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};

api_test!{ signs_up (test_app, client)
  let value = "test+token".to_string();
  test_app.app.one_time_token().insert(InsertOneTimeToken{
    value: "test+token".to_string()
  }).save().await?;

  let login_pubkey = URL_SAFE_NO_PAD.encode(
    test_app.private_key().public_key().to_pem().unwrap()
  );

  let created: create_session::ResponseData = client.gql(
    &CreateSession::build_query(create_session::Variables{}),
    vec![
      Header::new("Auth-Action", "Signup"),
      Header::new("Auth-Method-Kind", "OneTimeToken"),
      Header::new("Auth-Data", "test+token"),
      Header::new("Login-Pubkey", login_pubkey),
      Header::new("New-Session-Recaptcha-Code", "SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c"),
    ]
  ).await;

  dbg!(created);
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
