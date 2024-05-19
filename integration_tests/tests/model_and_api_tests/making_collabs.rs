// This module tests how collabs are made for accounts and sub accounts.
use ::api::models::*;

app_test! { sends_collabs_for_accounts_and_sub_accounts(a)
    let campaign = a.quick_campaign(u("100"), 30, &[]).await;

    let claimed = a.quick_handle("alice_on_x", "11111", Site::X, u("5000")).await;
    campaign.make_x_collab_with_user_id(claimed.user_id().as_ref().unwrap()).await?;

    a.wait_for_job("Account collab", OnChainJobKind::MakeCollabs, OnChainJobStatus::Settled).await;
    assert_eq!(claimed.collab_vec().await?.len(), 1);
    assert_eq!(claimed.collab_scope().status_eq(CollabStatus::Cleared).all().await?.len(), 1);

    let unclaimed_client = a.client().await;
    let unclaimed = unclaimed_client.create_handle("bob_on_x", "12121", Site::X, u("5000")).await;
    campaign.make_x_collab_with_user_id(unclaimed.user_id().as_ref().unwrap()).await?;

    a.wait_for_job("Subaccount collabs", OnChainJobKind::MakeSubAccountCollabs, OnChainJobStatus::Settled).await;
    assert_eq!(unclaimed.collab_vec().await?.len(), 1);
    assert_eq!(unclaimed.collab_scope().status_eq(CollabStatus::Cleared).all().await?.len(), 1);

    assert_eq!(a.app.collab().select().status_ne(CollabStatus::Cleared).count().await?, 0);
    assert_eq!(a.app.collab().select().status_eq(CollabStatus::Cleared).count().await?, 2);

    a.wait_for_job("No more accounts", OnChainJobKind::MakeCollabs, OnChainJobStatus::Skipped).await;
    a.wait_for_job("No more subaccount", OnChainJobKind::MakeSubAccountCollabs, OnChainJobStatus::Skipped).await;
}

app_test! { skips_if_we_are_no_longer_campaign_admins(a)
    let mut advertiser = a.client().await;
    advertiser.setup_as_advertiser("test main advertiser").await;
    let campaign = advertiser.start_and_pay_campaign("https://x.com/somebody/status/1716421161867710954", u("100"), 30, &[]).await;

    let handle = a.quick_handle("alice_on_x", "11111", Site::X, u("5000")).await;
    campaign.make_x_collab_with_user_id(handle.user_id().as_ref().unwrap()).await?;

    a.send_tx(
        "Advertiser wants to be its own admin",
        "71200",
        advertiser.asami_contract().config_account(advertiser.address(), u("6"), u("0"), u("0"))
    ).await;

    a.wait_for_job(
        "Account collab is skipped",
        OnChainJobKind::MakeCollabs,
        OnChainJobStatus::Skipped
    ).await;

    let unclaimed_client = a.client().await;
    let unclaimed = unclaimed_client
        .create_handle("bob_on_x", "12121", Site::X, u("5000"))
        .await;
    campaign.make_x_collab_with_user_id(unclaimed.user_id().as_ref().unwrap()).await?;
    a.wait_for_job(
        "Sub Account collab is skipped",
        OnChainJobKind::MakeSubAccountCollabs,
        OnChainJobStatus::Skipped
    ).await;
}


app_test! { prevents_race_conditions_using_available_budget (a)
    let mut campaign = a.quick_campaign(u("100"), 30, &[]).await;
    let handle = a.quick_handle("alice_on_x", "11111", Site::X, u("5000")).await;
    campaign.make_collab(&handle, u("10"), "first_post_trigger").await?;
    campaign.reload().await?;
    assert_eq!(campaign.available_budget().await?, u("90"));

    assert_eq!(
        campaign.make_collab(&handle, u("200"), "second_post_trigger").await.unwrap_err().to_string(),
        "Invalid input on site: campaign_has_not_enough_funds"
    );
}

app_test! { fails_when_handle_is_missing_topics (a)
    for i in &["sports", "crypto", "beauty"] {
        a.app.topic().insert(InsertTopic{ name: i.to_string() }).save().await?;
    }
    let sports = a.app.topic().find(1).await?;
    let crypto = a.app.topic().find(2).await?;
    let beauty = a.app.topic().find(3).await?;

    let handle = a.quick_handle("bob_on_x", "11111", Site::X, u("5000")).await;
    handle.add_topic(&sports).await?;
    handle.add_topic(&beauty).await?;

    let mut advertiser = a.client().await;
    advertiser.setup_as_advertiser("test main advertiser").await;
    let met_topics = a.quick_campaign( u("100"), 30, &[*beauty.id()]).await;
    assert!(met_topics.make_collab(&handle, u("10"), "first_post_trigger").await.is_ok());

    let no_topics = a.quick_campaign(u("100"), 30, &[]).await;
    assert!(no_topics.make_collab(&handle, u("10"), "second_post_trigger").await.is_ok());

    let unmet_topics = a.quick_campaign(u("100"), 30, &[*crypto.id(), *sports.id()]).await;

    assert_eq!(
        unmet_topics.make_collab(&handle, u("20"), "third_post_trigger").await.unwrap_err().to_string(),
        "Invalid input on topics: handle_is_missing_topics"
    );
}

app_test! { registers_collab_for_last_accepted_handle(a)
    // If someone loses their account, they can create a new one and re-bind their handles.
    // So collabs should always register to the most recently linked handle.
    let campaign = a.quick_campaign(u("100"), 30, &[]).await;

    let global_user_id = "11111";

    let old_handle = a.quick_handle("bob_on_x", global_user_id, Site::X, u("5000")).await;

    campaign.make_x_collab_with_user_id(&global_user_id).await?;
    assert_eq!(old_handle.collab_vec().await?.len(), 1);

    let new_handle = a.quick_handle("bob_on_x", global_user_id, Site::X, u("5000")).await;
    assert!(campaign.make_x_collab_with_user_id(&global_user_id).await?.is_none());

    assert_eq!(old_handle.collab_vec().await?.len(), 1);
    assert_eq!(new_handle.collab_vec().await?.len(), 0);

    let second_campaign = a.quick_campaign(u("100"), 30, &[]).await;
    assert!(second_campaign.make_x_collab_with_user_id(&global_user_id).await?.is_some());

    assert_eq!(old_handle.collab_vec().await?.len(), 1);
    assert_eq!(new_handle.collab_vec().await?.len(), 1);
}

app_test! { handle_has_a_max_reward_for_campaign(a)
    let campaign = a.quick_campaign(u("100"), 30, &[]).await;
    let small = a.quick_handle("small_on_x", "11111", Site::X, wei("5000")).await;
    let big = a.quick_handle("big_on_x", "22222", Site::X, wei("15000")).await;

    assert_eq!(small.reward_for(&campaign).unwrap(), u("5"));
    assert_eq!(big.reward_for(&campaign).unwrap(), milli("9999"));
}

app_test! { validates_handle_type_with_campaign_kind(a) 
    let campaign = a.quick_campaign(u("100"), 30, &[]).await;
    let ig = a.quick_handle("small_on_x", "11111", Site::Instagram, wei("5000")).await;
    assert_eq!(
        campaign.make_collab(&ig, u("10"), "first_post_trigger").await.unwrap_err().to_string(),
        "Invalid input on site: campaign_and_handle_sites_dont_match"
    );
}
