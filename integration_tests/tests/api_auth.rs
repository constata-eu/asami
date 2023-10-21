#[macro_use]
mod support;
use ::api::app::asami_contract::AsamiContract;
use ::api::models::*;
use graphql_client::GraphQLQuery;
use support::gql::*;
use rocket::http::Header;
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use ethers::prelude::*;
use ethers::{
  signers::{LocalWallet, MnemonicBuilder, coins_bip39::English, Signer},
  prelude::{abigen, Abigen},
  types::Address,
  providers::{Http, Provider},
};
use rust_decimal::prelude::ToPrimitive;

api_test!{ signs_up (test_app, client)

  let value = "test+token";
  test_app.app.one_time_token().insert(InsertOneTimeToken{
    value: value.to_string()
  }).save().await?;

  let login_pubkey = URL_SAFE_NO_PAD.encode(
    test_app.private_key().public_key().to_pem().unwrap()
  );

  // A test token should not work to log in to any account.
  let created: create_session::ResponseData = client.gql(
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


  let wallet = test_app.app.wallet;
  let provider = Provider::<Http>::try_from("http://127.0.0.1:8545")?;
  let client = std::sync::Arc::new(SignerMiddleware::new(provider, wallet.with_chain_id(test_app.app.settings.rsk.chain_id)));
  let address: Address = test_app.app.settings.contract_address.parse()?;
  let contract = AsamiContract::new(address, client);

  let contract_handle = ::api::asami_contract::Handle {
    value: handle.attrs.value.clone().into(),
    fixed_id: handle.attrs.fixed_id.clone().unwrap().into(),
    price: handle.attrs.price.clone().unwrap().to_u64().unwrap().into(),
    score: handle.attrs.score.clone().unwrap().to_u64().unwrap().into(),
    topics: handle.topic_ids().await.unwrap().into_iter().map(|i| i.into() ).collect(),
    verification_message_id: handle.attrs.verification_message_id.unwrap().into(),
  };

  //AsamiContract::h
  /*
  let contract_handle = serde_json::json![{
    "value": handle.attrs.value,
    "fixedId": handle.attrs.fixed_id,
    "price": handle.attrs.price.and_then(|x| x.to_u64() ).unwrap_or(0),
    "score": handle.attrs.score.and_then(|x| x.to_u64() ).unwrap_or(0),
    "topics": handle.topic_ids().await.unwrap(),
    "verificationMessageId": handle.attrs.verification_message_id.unwrap_or("".to_string()),
  }];
  */
  
  //dbg!(contract.get_topics().call().await);
  let response = contract.add_x_handle(handle.attrs.account_id.into(), contract_handle).call().await;
  dbg!(&response);

  /*
    // craft the transaction
    let tx = TransactionRequest::new().to(wallet2.address()).value(10000);

    // send it!
    let pending_tx = client.send_transaction(tx, None).await?;

    // get the mined tx
    let receipt = pending_tx.await?.ok_or_else(|| eyre::format_err!("tx dropped from mempool"))?;
    let tx = client.get_transaction(receipt.transaction_hash).await?;



  */

  todo!("die here");

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
