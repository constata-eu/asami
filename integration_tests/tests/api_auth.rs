#[macro_use]
mod support;
use ::api::models::*;
use graphql_client::GraphQLQuery;
use support::gql::*;
use rocket::http::Header;
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};

api_test!{ signs_up_and_makes_x_collab_stubbing (test_app, client)
  client.login().await?;

  let account = test_app.app.account().find(weihex("1")).await?;
  let mut handle_req = account.create_handle_request(Site::X, "nubis_bruno").await?;
  handle_req = handle_req.verify("179383862".into()).await?;
  handle_req = handle_req.appraise(u("10"), u("10")).await?;

  test_app.app.handle_request().submit_all().await?;
  test_app.app.synced_event().sync_on_chain_events().await?;
  assert_eq!(handle_req.reloaded().await?.attrs.status, HandleRequestStatus::Done);

  let handle = test_app.app.handle().select().one().await?;

  //"https://x.com/asami_club/status/1716421161867710954?s=20",
  let campaign_req = account.create_campaign_request(
    Site::X,
    "1716421161867710954",
    u("50"),
    u("2"),
    Utc::now() + chrono::Duration::days(2),
  ).await?
  .pay().await?;

  test_app.app.campaign_request().submit_approvals().await?;
  test_app.app.synced_event().sync_on_chain_events().await?;
  assert_eq!(campaign_req.reloaded().await?.attrs.status, CampaignRequestStatus::Approved);

  test_app.app.campaign_request().submit_all().await?;
  test_app.app.synced_event().sync_on_chain_events().await?;
  assert_eq!(campaign_req.reloaded().await?.attrs.status, CampaignRequestStatus::Done);

  let mut campaign = test_app.app.campaign().select().one().await.unwrap();

  assert_that!(&campaign.attrs, structure![ CampaignAttrs {
    budget: eq(uhex("50")),
    remaining: eq(uhex("50")),
  }]);

  test_app.app.collab_request().insert(InsertCollabRequest{
    campaign_id: campaign.attrs.id.clone(),
    handle_id: handle.attrs.id.clone(),
  }).save().await?;

  test_app.app.collab_request().submit_all().await.unwrap();
  test_app.app.synced_event().sync_on_chain_events().await?;
  assert!(test_app.app.collab().select().count().await? == 1);

  campaign.reload().await?;

  assert_that!(&campaign.attrs, structure![ CampaignAttrs {
    budget: eq(uhex("50")),
    remaining: eq(uhex("40"))
  }]);
}

/*
api_test!{ signs_up_and_makes_collab_in_x (test_app, client)
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
    status: eq(HandleRequestStatus::Appraised),
    fixed_id: maybe_some(rematch("179383862")),
    verification_message_id: maybe_some(rematch("1713930237598150807")),
    //score: maybe_some(eq(Decimal::new(3393, 0)))
  }]);

  let submitted = handle.submit().await?;

  assert_that!(&submitted.attrs, structure![ HandleAttrs {
    status: eq(HandleRequestStatus::Submitted),
    tx_hash: maybe_some(any_value())
  }]);

  tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

  test_app.app.sync_on_chain_events().await?;

  assert_that!(&submitted.reloaded().await?.attrs, structure![ HandleAttrs {
    status: eq(HandleRequestStatus::Active),
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

  let mut campaign = test_app.app.campaign().select().one().await.unwrap();

  assert_that!(&campaign.attrs, structure![ CampaignAttrs {
    budget: eq(Decimal::new(10,0)),
    remaining: eq(Decimal::new(10,0))
  }]);

  test_app.app.campaign().sync_x_collabs().await.unwrap();

  assert!(test_app.app.collab_request().select().count().await? == 1);

  test_app.app.collab_request().submit_all().await.unwrap();

  test_app.app.sync_on_chain_events().await.unwrap();

  assert!(test_app.app.collab().select().count().await? == 1);

  campaign.reload().await?;

  assert_that!(&campaign.attrs, structure![ CampaignAttrs {
    budget: eq(Decimal::new(10,0)),
    remaining: eq(Decimal::new(7,0))
  }]);
}
*/

