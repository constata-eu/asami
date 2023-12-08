#[macro_use]
mod support;

use graphql_client::GraphQLQuery;
use support::{
  *,
  gql::{
    *,
    create_handle_request as chr,
    all_handle_requests as ahr,
    all_collabs as ac,
  }
};
use api::models::*;

browser_test!{ browser_flow_until_instagram_reward (mut d)
  todo!("Handles are also sent, not only unverified handle requests.");

  d.signup_with_one_time_token().await;

  d.click("#open-start-campaign-dialog").await;
  d.fill_in("#contentUrl", "https://www.instagram.com/p/C0T1wKQMS0v/?utm_source=ig_web_copy_link&igshid=MzRlODBiNWFlZA==").await;
  d.fill_in("#budget", "20").await;
  d.click("#submit-start-campaign-form").await;
  d.wait_for_text(".MuiSnackbarContent-message", "Campaign will be started soon").await;
  d.wait_until_gone(".MuiSnackbarContent-message").await;
  d.wait_for("#campaign-request-list").await;

  d.api.test_app.mock_admin_setting_campaign_requests_as_paid().await;
  d.api.test_app.run_idempotent_background_tasks_a_few_times().await;

  d.wait_for("#campaign-list").await;
  d.click("#logout-menu-item").await;
  d.wait_for("#button-login-as-advertiser").await;
  
  d.api.test_app.app.one_time_token().insert(InsertOneTimeToken{value: "member-token".to_string() }).save().await?;
  d.goto("http://127.0.0.1:5173/#/one_time_token_login?token=member-token").await;
  d.wait_for("#advertiser-dashboard").await;
  d.goto("http://127.0.0.1:5173/#/?role=member").await;
  d.wait_for("#member-dashboard").await;

  d.fill_in("#ig_username", "nubis_bruno").await;
  d.click("#submit-instagram-handle-request-form").await;

  d.wait_for(".MuiSnackbarContent-message").await;
  d.wait_until_gone(".MuiSnackbarContent-message").await;

  let hub = d.app().ig_crawl();
  hub.schedule_new().await?;
  let mut crawl = hub.find(1).await?;
  hub.submit_scheduled().await?;
  crawl.reload().await?;
  try_until(90, 2000, "crawl was not done", || async {
    hub.collect_all_responses().await.unwrap();
    crawl.reloaded().await.unwrap().attrs.status == models::IgCrawlStatus::Responded
  }).await;

  hub.process_for_handle_requests().await?;

  d.wait_for("#handle-instagram-submission-in-progress-message").await;
  d.api.test_app.run_idempotent_background_tasks_a_few_times().await;
  d.wait_for("#existing-instagram-handle-stats").await;
  d.wait_for("#campaign-list-empty").await;

  hub.process_for_campaign_rules().await?;
  d.wait_for("#campaign-container-0x0000000000000000000000000000000000000000000000000000000000000001").await;

  hub.process_for_collabs().await?;
  d.api.test_app.run_idempotent_background_tasks_a_few_times().await;
  d.wait_for("#campaign-list-empty").await;
}

api_test! { supports_instagram_collaboration (mut c)
  let _scenario = c.build_baseline_scenario().await;

  c.login().await;
  
  let handle_req = c.gql::<chr::ResponseData, _>(
    &CreateHandleRequest::build_query(chr::Variables{
      input: chr::CreateHandleRequestInput {
        site: chr::Site::INSTAGRAM,
        username: "nubis_bruno".to_string(),
      }
    }),
    vec![]
  ).await.create_handle_request;

  let hub = c.app().ig_crawl();

  hub.schedule_new().await?;

  assert_eq!(hub.select().count().await?, 1);
  let mut crawl = hub.find(1).await?;
  let direct_urls: Vec<String> = serde_json::from_str::<serde_json::Value>(crawl.input())?["directUrls"]
    .as_array().unwrap().iter().map(|s| s.as_str().unwrap().to_string()).collect();
  assert!(direct_urls.contains(&"https://www.instagram.com/nubis_bruno".to_string()));
  assert!(direct_urls.contains(&"https://www.instagram.com/p/C0T1wKQMS0v".to_string()));

  if false {
    crawl = crawl.update()
      .status(models::IgCrawlStatus::Submitted)
      .apify_id(Some("FWtqLuS4KucLLZVqc".to_string()))
      .save().await?;
  } else {
    hub.submit_scheduled().await?;
    crawl.reload().await?;
    assert!(crawl.apify_id().is_some());
  }

  try_until(90, 2000, "crawl was not done", || async {
    hub.collect_all_responses().await.unwrap();
    crawl.reloaded().await.unwrap().attrs.status == models::IgCrawlStatus::Responded
  }).await;

  crawl.reload().await?;
  assert_eq!(crawl.ig_crawl_result_vec().await?.len(), 2);

  hub.process_for_campaign_rules().await?;
  crawl.reload().await?;
  assert!(crawl.attrs.processed_for_campaign_rules);
  let rule = c.app().ig_campaign_rule().find(1).await?;

  assert_eq!(rule.caption(), "");
  assert_eq!(rule.image_hash(), "APjzA6bTAPjvD9guAeCNL3DZBkAb98BlDTLtnAGXNMI8JgNcSgynMAhwqXmc4oHk9WZhil+Z182JKd0jXrcnptTGeFiOkNae4+WdQ1xrx482jlN4SL/KPE7jsey6+TgJYzJrZsJkzMmozRkTsyMjt2McjMUsXbbhYB6z+DmGkznuhCdwxuw4A0/AE2xGDLhFWqARPcACxBPPfAAunzL4coBQcgJ4aBBCZBG4wDMAzUVx4QUJnJXBABckchJTQxiZwEPMTDGaZA4TMcTcYNj4cBDTgsO458BITgbexIBjADEZhwEDIVimUQMOAMzIbAZ8JjyS/AzwufgI/hnQY2InOGdhDgWZMcMAOQQKxgZDzIDwFQ+E+QPBEgYY0h8AABhoxuwAAGiBA98HAIA1nsd4cAgPmbV5wEEcQJ//gEluwEFAwAP/wWUEBA/8ARPXABywYXzod/jA8vGx4MMD64gHAB4X3EA+AIBcoMPoBAB0AQOz6wHQBwaEDkFDDwwAfwAdPRgIOABx8HEg4RlQ2McQwDMGz5+BAR/2MPoADnz+j+kKZvAT/4ghDsCf4weGGAD/cH4YcAD8h8lh0AFxv2SG0QXE/5KHJh5Q/0s2nGjA/C88zowD9/9weJsMzq/h4C02ePrBIFn84Dnj+LLxgyfiOWDRTY7gHJChNTlCADSn5OAMQJwMFoMx+PsbCUzG/8E5NDE43x+OScVg/DnhRjUDsX8VH0UY5r8FeGYICQP+gAtjICwPDCwchLCHY7F0WcrD8ZjTMZDj8HgEb3LjODgcePR0PDyZgmlzPHg2RfjlPXy8mkWbk3j8IkMvhvB4SaZ4mPL5tg3iYeBwSBrlwwE");

  hub.process_for_handle_requests().await?;
  crawl.reload().await?;
  assert!(crawl.attrs.processed_for_handle_requests);

  let verified = get_handle_requests(&mut c, handle_req.id).await; 
  assert_eq!(verified.status, ahr::HandleRequestStatus::APPRAISED);
  assert_eq!(verified.price.unwrap(), weihex("2920000000000000000"));
  assert_eq!(verified.score.unwrap(), weihex("292"));

  c.app().handle_request().find(1).await?.update().price(Some(uhex("1"))).save().await?;

  c.test_app.run_idempotent_background_tasks_a_few_times().await;

  hub.process_for_collabs().await?;
  crawl.reload().await?;
  assert!(crawl.attrs.processed_for_collabs);

  c.test_app.run_idempotent_background_tasks_a_few_times().await;

  let collab = get_collabs(&mut c).await;
  assert_eq!(collab.campaign_id, weihex("2"));
  assert_eq!(collab.member_id, weihex("2"));
  assert_eq!(collab.gross, uhex("1"));
}

async fn get_handle_requests(c: &mut ApiClient, id: i64) -> ahr::AllHandleRequestsAllHandleRequests {
  c.gql::<ahr::ResponseData, _>(
    &AllHandleRequests::build_query(ahr::Variables{
      filter: Some(ahr::HandleRequestFilter {
        ids: None,
        id_eq: Some(id),
        username_like: None,
        status_in: None,
        site_eq: Some(ahr::Site::INSTAGRAM),
      }),
      page: None,
      per_page: None,
      sort_field: None,
      sort_order: None,
    }),
    vec![]
  ).await.all_handle_requests.pop().unwrap()
}

async fn get_collabs(c: &mut ApiClient) -> ac::AllCollabsAllCollabs {
  c.gql::<ac::ResponseData, _>(
    &AllCollabs::build_query(ac::Variables{
      filter: None,
      page: None,
      per_page: None,
      sort_field: None,
      sort_order: None,
    }),
    vec![]
  ).await.all_collabs.pop().unwrap()
}
