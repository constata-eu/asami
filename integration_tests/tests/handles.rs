#[macro_use]
pub mod support;
use ::api::models::*;

api_test! { verifies_twitter_handle (mut c)
  c.create_x_handle("nubis_bruno", wei("10000000000000000")).await;
  let account = c.account().await;
  let handle = c.x_handle().await;

  //"https://x.com/asami_club/status/1716421161867710954?s=20",
  let campaign_req = account.create_campaign_request(
    Site::X,
    "1716421161867710954",
    u("50"),
    u("2"),
    Utc::now() + chrono::Duration::days(2),
    &[],
  ).await?
  .pay().await?;

  c.test_app.app.campaign_request().submit_approvals().await?;
  c.test_app.app.synced_event().sync_on_chain_events().await?;
  assert_eq!(campaign_req.reloaded().await?.attrs.status, CampaignRequestStatus::Approved);

  c.test_app.app.campaign_request().submit_all().await?;
  c.test_app.app.synced_event().sync_on_chain_events().await?;
  assert_eq!(campaign_req.reloaded().await?.attrs.status, CampaignRequestStatus::Done);

  let mut campaign = c.test_app.app.campaign().select().one().await.unwrap();

  assert_that!(&campaign.attrs, structure![ CampaignAttrs {
    budget: eq(uhex("50")),
    remaining: eq(uhex("50")),
  }]);

  c.test_app.app.collab_request().insert(InsertCollabRequest{
    campaign_id: campaign.attrs.id.clone(),
    handle_id: handle.attrs.id.clone(),
  }).save().await?;

  c.test_app.app.collab_request().submit_all().await.unwrap();
  c.test_app.app.synced_event().sync_on_chain_events().await?;
  assert!(c.test_app.app.collab().select().count().await? == 1);

  campaign.reload().await?;

  assert_that!(&campaign.attrs, structure![ CampaignAttrs {
    budget: eq(uhex("50")),
    remaining: eq(weihex("49900000000000000000"))
  }]);
}

// Associates IG handle
// Re-associates twitter handle.
// Does not get rewards re-awarded after re-associating handle.
