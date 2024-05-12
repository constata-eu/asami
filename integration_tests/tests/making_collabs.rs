use crate::support;
use ::api::models::*;

app_test! { makes_collab_consumes_remaining (a)
  let advertiser = a.client().await;
  let campaign = advertiser.create_x_campaign(u("10"), u("1")).await;

  let bob = a.client().await;
  bob.create_x_handle("bob_on_x", u("1")).await;
  bob.create_x_collab(&campaign).await;

  assert_that!(&campaign.reloaded().await?.attrs, structure![ CampaignAttrs {
    budget: eq(uhex("10")),
    remaining: eq(uhex("9")),
  }]);
}

app_test! { fails_when_collab_would_be_over_budget (a)
  let advertiser = a.client().await;
  let campaign = advertiser.create_x_campaign(u("10"), u("5")).await;

  let bob = a.client().await;
  bob.create_x_handle("bob_on_x", u("11")).await;
  let handle = bob.x_handle().await;

  assert!(a.app.collab_request().select().count().await? == 0);
  assert_that!(&campaign.make_collab(&handle).await.unwrap_err(),
    structure!{api::Error::Validation[eq("price".to_string()), eq("campaign_funds_too_low".to_string())]}
  );

  let err = a.app.on_chain.contract
    .admin_make_collabs(vec![on_chain::AdminMakeCollabsInput{
      campaign_id: u256(&campaign.attrs.id),
      handle_id: u256(&handle.attrs.id),
    }])
    .send().await.unwrap_err();

  assert_eq!(err.decode_revert::<String>().unwrap(), "amc5");
}

app_test! { fails_when_campaign_sparkle_price_is_too_low (a)
  let advertiser = a.client().await;
  let campaign = advertiser.create_x_campaign(u("20"), u("1")).await;

  let bob = a.client().await;
  bob.create_x_handle("bob_on_x", u("11")).await;
  let handle = bob.x_handle().await;

  assert!(a.app.collab_request().select().count().await? == 0);
  assert_that!(&campaign.make_collab(&handle).await.unwrap_err(),
    structure!{api::Error::Validation[eq("price_score_ratio".to_string()), eq("campaign_pays_too_little".to_string())]}
  );

  let err = a.app.on_chain.contract
    .admin_make_collabs(vec![on_chain::AdminMakeCollabsInput{
      campaign_id: u256(&campaign.attrs.id),
      handle_id: u256(&handle.attrs.id),
    }])
    .send().await.unwrap_err();

  assert_eq!(err.decode_revert::<String>().unwrap(), "amc7");
}

app_test! { fails_when_handle_is_missing_topics (a)
  for i in &["sports", "crypto", "beauty"] {
    a.app.topic_request().create(i).await?;
  }
  a.run_idempotent_background_tasks_a_few_times().await;
  let sports = a.app.topic().find(weihex("1")).await?;
  let crypto = a.app.topic().find(weihex("2")).await?;
  let beauty = a.app.topic().find(weihex("3")).await?;

  let bob_client = a.client().await;
  bob_client.create_x_handle("bob_on_x", u("1")).await;
  let handle = bob_client.x_handle().await;
  a.app.set_score_and_topics_request().create(&handle, wei("2000"), &[&sports, &beauty]).await?;
  a.run_idempotent_background_tasks_a_few_times().await;

  let advertiser = a.client().await;

  let met_topics = advertiser.create_x_campaign_extra(u("20"), u("1"), 2, &[beauty]).await;
  bob_client.create_x_collab(&met_topics).await;

  let no_topics = advertiser.create_x_campaign_extra(u("20"), u("1"), 2, &[]).await;
  bob_client.create_x_collab(&no_topics).await;

  assert!(a.app.collab_request().select().count().await? == 2);

  let unmet_topics = advertiser.create_x_campaign_extra(u("20"), u("1"), 2, &[crypto, sports]).await;

  assert_that!(&unmet_topics.make_collab(&handle).await.unwrap_err(),
    structure!{api::Error::Validation[eq("topics".to_string()), eq("handle_is_missing_topics".to_string())]}
  );

  let err = a.app.on_chain.contract
    .admin_make_collabs(vec![on_chain::AdminMakeCollabsInput{
      campaign_id: u256(&unmet_topics.attrs.id),
      handle_id: u256(&handle.attrs.id),
    }])
    .send().await.unwrap_err();

  assert_eq!(err.decode_revert::<String>().unwrap(), "amc4");

  assert!(a.app.collab_request().select().count().await? == 2);
}

app_test! { registers_collab_for_last_accepted_handle(a)
  // If someone loses their account, they can create a new one and re-bind their handles.
  // So collabs should always register to the most recently linked handle.
  let advertiser = a.client().await;
  let campaign = advertiser.create_x_campaign(u("10"), u("1")).await;

  let old_bob = a.client().await;
  old_bob.create_x_handle("bob_on_x", u("1")).await;
  let user_id = old_bob.x_handle().await.attrs.user_id;

  a.app.campaign().try_x_collab_for_newest_handle(&campaign, &user_id).await?;
  a.run_idempotent_background_tasks_a_few_times().await;
  assert_eq!(old_bob.x_handle().await.collab_vec().await?.len(), 1);

  let new_bob = a.client().await;
  new_bob.create_x_handle("bob_on_x", u("1")).await;
  a.app.campaign().try_x_collab_for_newest_handle(&campaign, &user_id).await?;
  a.run_idempotent_background_tasks_a_few_times().await;
  assert_eq!(new_bob.x_handle().await.collab_vec().await?.len(), 1);
  assert_eq!(old_bob.x_handle().await.collab_vec().await?.len(), 1);
}

app_test! { test(a)
  todo!("Make collaborations track the handle used, not only the account. Do not allow handle re-claims participating in campaigns again.");
  // A user can collab, get paid, register a new account, re-link the handle, and have a new collaboration paid to them.
}

app_test! { test_b(a)
    todo!("When registering collabs, store more information about the process.");

}

app_test! { test_c(a)
    todo!("Prevent collabs from being registered twice?");
}
