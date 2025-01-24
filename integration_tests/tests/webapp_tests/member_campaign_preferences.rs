use api::models::Site;
//use graphql_client::GraphQLQuery;
use crate::support::gql::{create_campaign_preference::*, *};

browser_test! { hides_ignored_and_shows_attempted_repost (mut d)
    d.login().await;

    d.api.create_handle("alice_on_x", "11111", u("200")).await;

    let ignorable = d.test_app().quick_campaign(u("10"), 2, &[]).await;
    let important = d.test_app().quick_campaign(u("100"), 2, &[]).await;

    d.goto_member_dashboard().await;

    d.wait_for("#twitter-widget-1").await;
    d.click(&format!("#open-hide-campaign-{}", &ignorable.attrs.id)).await;
    d.click("button.ra-confirm").await;
    d.wait_until_gone(&format!("#campaign-container-{}", &ignorable.attrs.id)).await;

    d.click(&format!("#button-repost-{}", &important.attrs.id)).await;
    d.wait_for(&format!("#alert-repost-attempted-{}", &important.attrs.id)).await;
}

api_test! { filters_campaigns_for_account (mut a)
    a.claim_account().await;
    a.create_handle("alice_on_x", "1111", wei("100")).await;

    let one = a.test_app.quick_campaign(u("10"), 2, &[]).await;
    let two = a.test_app.quick_campaign(u("100"), 2, &[]).await;

    let mut all = a.get_campaign_offers().await;

    assert_eq!(all.all_campaigns.len(), 2);
    assert_eq!(all.all_campaigns[0].id, one.attrs.id as i64);
    assert_eq!(all.all_campaigns[1].id, two.attrs.id as i64);

    let _res: ResponseData = a.gql(
    &CreateCampaignPreference::build_query(Variables{
      input: CreateCampaignPreferenceInput {
        campaign_id: one.attrs.id.into(),
        not_interested: true,
        attempted: false
      }
    }),
    vec![]
    ).await;

    all = a.get_campaign_offers().await;

    assert_eq!(all.all_campaigns.len(), 1);
    assert_eq!(all.all_campaigns[0].id, two.attrs.id as i64);

    let _res: ResponseData = a.gql(
    &CreateCampaignPreference::build_query(Variables{
      input: CreateCampaignPreferenceInput {
        campaign_id: two.attrs.id.into(),
        not_interested: false,
        attempted: true
      }
    }),
    vec![]
    ).await;

    assert_eq!(a.get_campaign_offers().await.all_campaigns.len(), 1);
}
