#[macro_use]
pub mod support;
use graphql_client::GraphQLQuery;
use support::{*, gql::{*, create_campaign_preference::*}};

browser_test!{ hides_ignored_and_shows_attempted_repost (mut d)
  let scenario = d.api.build_baseline_scenario().await;
  d.api.create_x_handle("nubis_bruno", wei("10000000000000000")).await;
  d.goto_member_dashboard().await;

  d.wait_for("#twitter-widget-1").await;
  d.click(&format!("#open-hide-campaign-{}", &scenario.regular_campaign.attrs.id)).await;
  d.click("button.ra-confirm").await;
  d.wait_until_gone(&format!("#campaign-container-{}", &scenario.regular_campaign.attrs.id)).await;

  d.click(&format!("#button-repost-{}", &scenario.high_rate_campaign.attrs.id)).await;
  d.wait_for(&format!("#alert-repost-attempted-{}", &scenario.high_rate_campaign.attrs.id)).await;
}


api_test!{ filters_campaigns_for_account (mut c)
  let scenario = c.build_baseline_scenario().await;
  c.create_x_handle("nubis_bruno", wei("10000000000000000")).await;

  let mut all = get_campaign_offers(&mut c).await;

  assert_eq!(all.all_campaigns.len(), 2);
  assert_eq!(&all.all_campaigns[0].id, &scenario.regular_campaign.attrs.id);
  assert_eq!(&all.all_campaigns[1].id, &scenario.high_rate_campaign.attrs.id);

  let _res: ResponseData = c.gql(
    &CreateCampaignPreference::build_query(Variables{
      input: CreateCampaignPreferenceInput {
        campaign_id: scenario.regular_campaign.attrs.id.clone(),
        not_interested: true,
        attempted: false
      }
    }),
    vec![]
  ).await;

  all = get_campaign_offers(&mut c).await;

  assert_eq!(all.all_campaigns.len(), 1);
  assert_eq!(&all.all_campaigns[0].id, &scenario.high_rate_campaign.attrs.id);

  let _res: ResponseData = c.gql(
    &CreateCampaignPreference::build_query(Variables{
      input: CreateCampaignPreferenceInput {
        campaign_id: scenario.high_rate_campaign.attrs.id.clone(),
        not_interested: false,
        attempted: true
      }
    }),
    vec![]
  ).await;

  assert_eq!(get_campaign_offers(&mut c).await.all_campaigns.len(), 1);
}

async fn get_campaign_offers(client: &mut ApiClient<'_>) -> all_campaigns::ResponseData {
  client.gql(
    &AllCampaigns::build_query(all_campaigns::Variables{
      filter: Some(all_campaigns::CampaignFilter {
        available_to_account_id: Some(client.account().await.attrs.id),
        ids: None,
        id_eq: None,
        account_id_eq: None,
        finished_eq: None,
        content_id_like: None,
      }),
      page: None,
      per_page: None,
      sort_field: None,
      sort_order: None,
    }),
    vec![]
  ).await
}
