use models::{
    weihex, EngagementScore, Handle, HandleScoring, HandleScoringStatus, HandleStatus, InsertHandleScoring,
    OperationalStatus, PollScore,
};
use rand::{thread_rng, Rng};
use rocket::serde::json::{self, Value};

use super::*;

#[tokio::test]
#[serial_test::file_serial]
async fn full_authority_scoring() {
    TestHelper::run(|h| async move {
        let account = h.user().await.signed_up().await.unverified("asami_tester", user_id()).await;
        let mut handle = account.handle.clone().unwrap();
        handle = handle
            .update()
            .poll_id(Some(poll_tweet_id()))
            .referrer_score_override(Some(true))
            .holder_score_override(Some(true))
            .offline_engagement_score(EngagementScore::High)
            .save()
            .await
            .unwrap();

        let scoring = pre_ingested_handle_scoring(
            &handle,
            me_json("347", true),
            &[
                reply_to_own_tweet(500, 0, 2, 11, 1),
                quoting_someone_elses_tweet(300, 0, 2, 11, 1),
                tweet_hello_world(300, 0, 0, 11, 0),
                tweet_goodbye_world(550, 0, 1, 11, 0),
                tweet_foo_bar(50, 0, 0, 0, 1),
                tweet_poll(300, 0, 0, 3, 0),
            ],
            mentions_json(10, 16),
            Some(poll_json(1, 3, 4, 5)),
            reposts_json(),
        )
        .await
        .apply()
        .await
        .unwrap();

        handle.reload().await.unwrap();

        ScoringExpectations {
            ..Default::default()
        }.assert_matches(&scoring, &handle);

        assert_eq!(handle.score(), scoring.score());
    })
    .await;
}

#[tokio::test]
#[serial_test::file_serial]
async fn applies_empty_score() {
    TestHelper::run(|h| async move {
        let account = h.user().await.signed_up().await.unverified("asami_tester", user_id()).await;
        let mut handle = account.handle.clone().unwrap();
        handle = handle
            .update()
            .poll_id(Some(poll_tweet_id()))
            .referrer_score_override(Some(true))
            .holder_score_override(Some(true))
            .offline_engagement_score(EngagementScore::High)
            .save()
            .await
            .unwrap();

        let scoring = handle
            .state
            .handle_scoring()
            .insert(InsertHandleScoring {
                handle_id: *handle.id(),
            })
            .save()
            .await
            .unwrap()
            .update()
            .status(HandleScoringStatus::Ingested)
            .save()
            .await
            .unwrap()
            .apply()
            .await
            .unwrap();

        handle.reload().await.unwrap();

        assert_eq!(*scoring.status(), HandleScoringStatus::Discarded);
        assert_eq!(*scoring.audience_size(), 0);
        assert_eq!(*scoring.authority(), 0);
        assert!(scoring.score().is_none());

        assert!(handle.current_scoring_id().is_none());
        assert!(handle.last_scoring().is_none());
        assert_eq!(*handle.status(), HandleStatus::Unverified);
        assert_eq!(handle.score(), &Some(weihex("0")));

        let applied_scoring = pre_ingested_handle_scoring(
            &handle,
            me_json("347", true),
            &[
                reply_to_own_tweet(500, 0, 2, 11, 1),
                quoting_someone_elses_tweet(300, 0, 2, 11, 1),
                tweet_hello_world(300, 0, 0, 11, 0),
                tweet_goodbye_world(550, 0, 1, 11, 0),
                tweet_foo_bar(50, 0, 0, 0, 1),
                tweet_poll(300, 0, 0, 3, 0),
            ],
            mentions_json(10, 16),
            Some(poll_json(1, 3, 4, 5)),
            reposts_json(),
        )
        .await
        .apply()
        .await
        .unwrap();

        handle.reload().await.unwrap();

        assert_eq!(*applied_scoring.status(), HandleScoringStatus::Applied);
        assert_eq!(*applied_scoring.audience_size(), 316);
        assert_eq!(*applied_scoring.authority(), 100);
        assert_eq!(*applied_scoring.score().as_ref().unwrap(), weihex("316"));

        assert_eq!(handle.current_scoring_id().unwrap(), *applied_scoring.id());
        assert_eq!(handle.last_scoring().unwrap(), *applied_scoring.created_at());
        assert_eq!(*handle.status(), HandleStatus::Active);
        assert_eq!(handle.score(), applied_scoring.score());

        let next_scoring = handle
            .state
            .handle_scoring()
            .insert(InsertHandleScoring {
                handle_id: *handle.id(),
            })
            .save()
            .await
            .unwrap()
            .update()
            .status(HandleScoringStatus::Ingested)
            .save()
            .await
            .unwrap()
            .apply()
            .await
            .unwrap();

        assert_eq!(*next_scoring.status(), HandleScoringStatus::Discarded);
        assert_eq!(*next_scoring.audience_size(), 0);
        assert_eq!(*next_scoring.authority(), 0);
        assert!(next_scoring.score().is_none());

        handle.reload().await.unwrap();
        assert_eq!(handle.current_scoring_id().unwrap(), *applied_scoring.id());
        assert_eq!(handle.last_scoring().unwrap(), *applied_scoring.created_at());
        assert_eq!(*handle.status(), HandleStatus::Active);
        assert_eq!(handle.score(), applied_scoring.score());
    })
    .await;
}

#[tokio::test]
#[serial_test::file_serial]
async fn average_authority_scoring() {
    TestHelper::run(|h| async move {
        let account = h.user().await.signed_up().await.unverified("asami_tester", user_id()).await;
        let mut handle = account.handle.clone().unwrap();

        let scoring = pre_ingested_handle_scoring(
            &handle,
            me_json("5", false),
            &[
                reply_to_own_tweet(50, 0, 0, 1, 0),
                quoting_someone_elses_tweet(31, 0, 0, 1, 1),
                tweet_hello_world(31, 0, 0, 1, 0),
                tweet_goodbye_world(55, 0, 0, 1, 0),
                tweet_foo_bar(5, 0, 0, 0, 0),
                tweet_poll(31, 0, 0, 3, 0),
            ],
            tweets_json([]),
            None,
            reposts_json(),
        )
        .await
        .apply()
        .await
        .unwrap();

        handle.reload().await.unwrap();

        ScoringExpectations::average().assert_matches(&scoring, &handle);

        assert_eq!(handle.score(), scoring.score());
    })
    .await;
}

#[tokio::test]
#[serial_test::file_serial]
async fn can_attempt_scoring_with_no_mentions_and_no_reposts() {
    TestHelper::run(|h| async move {
        let account = h.user().await.signed_up().await.unverified("asami_tester", user_id()).await;
        let mut handle = account.handle.clone().unwrap();

        let scoring = pre_ingested_handle_scoring(
            &handle,
            me_json("5", false),
            &[
                reply_to_own_tweet(50, 0, 0, 1, 0),
                quoting_someone_elses_tweet(31, 0, 0, 1, 1),
                tweet_hello_world(31, 0, 0, 1, 0),
                tweet_goodbye_world(55, 0, 0, 1, 0),
                tweet_foo_bar(5, 0, 0, 0, 0),
                tweet_poll(31, 0, 0, 3, 0),
            ],
            empty_result_set(),
            None,
            empty_result_set(),
        )
        .await
        .apply()
        .await
        .unwrap();

        handle.reload().await.unwrap();

        ScoringExpectations::average().assert_matches(&scoring, &handle);
        assert_eq!(handle.score(), scoring.score());
    })
    .await;
}

#[tokio::test]
#[serial_test::file_serial]
async fn ghost_account_for_short_tweets() {
    TestHelper::run(|h| async move {
        let account = h.user().await.signed_up().await.unverified("asami_tester", user_id()).await;
        let mut handle = account.handle.clone().unwrap();

        let scoring = pre_ingested_handle_scoring(
            &handle,
            me_json("5", false),
            &[
                reply_to_own_tweet(50, 0, 0, 1, 0),
                quoting_someone_elses_tweet(31, 0, 0, 1, 1),
                tweet_hello_world(31, 0, 0, 1, 0),
                tweet_goodbye_world(55, 0, 0, 1, 0),
                make_own_tweet("GM", 20000, 1, 1, 1, 1),
                make_own_tweet("My friends!", 20000, 1, 1, 1, 1),
                make_own_tweet("LFG", 10000, 0, 0, 0, 0),
            ],
            empty_result_set(),
            None,
            reposts_json(),
        )
        .await
        .apply()
        .await
        .unwrap();

        handle.reload().await.unwrap();

        ScoringExpectations {
            audience_size: 3610,
            ghost_account: true,
            online_engagement_score: EngagementScore::None,
            authority: 0,
            score: weihex("0"),
            handle_score: weihex("0"),
            ..ScoringExpectations::average()
        }.assert_matches(&scoring, &handle);
    })
    .await;
}

#[tokio::test]
#[serial_test::file_serial]
async fn ghost_account_for_low_impressions() {
    TestHelper::run(|h| async move {
        let account = h.user().await.signed_up().await.unverified("asami_tester", user_id()).await;
        let mut handle = account.handle.clone().unwrap();

        let scoring = pre_ingested_handle_scoring(
            &handle,
            me_json("5", false),
            &[
                reply_to_own_tweet(50, 0, 0, 1, 0),
                quoting_someone_elses_tweet(50, 0, 0, 1, 1),
                tweet_hello_world(50, 0, 0, 1, 0),
                tweet_goodbye_world(50, 0, 0, 1, 0),
                tweet_foo_bar(30, 0, 0, 0, 0),
                tweet_poll(30, 0, 0, 3, 0),
            ],
            tweets_json([]),
            None,
            reposts_json(),
        )
        .await
        .apply()
        .await
        .unwrap();

        handle.reload().await.unwrap();

        ScoringExpectations {
            audience_size: 46,
            ghost_account: true,
            online_engagement_score: EngagementScore::None,
            authority: 0,
            score: weihex("0"),
            handle_score: weihex("0"),
            ..ScoringExpectations::average()
        }.assert_matches(&scoring, &handle);
    })
    .await;
}

#[tokio::test]
#[serial_test::file_serial]
async fn repost_fatigue() {
    TestHelper::run(|h| async move {
        let account = h.user().await.signed_up().await.unverified("asami_tester", user_id()).await;
        let mut handle = account.handle.clone().unwrap();

        let scoring = pre_ingested_handle_scoring(
            &handle,
            me_json("5", false),
            &[
                tweet_hello_world(50, 0, 0, 1, 0),
                tweet_hello_world(50, 0, 0, 1, 0),
                tweet_hello_world(50, 0, 0, 1, 0),
                tweet_hello_world(50, 0, 0, 1, 0),
            ],
            tweets_json([]),
            None,
            tweets_json([
                make_retweet(&make_random_id()),
                make_retweet(&make_random_id()),
                make_retweet(&make_random_id()),
                make_retweet(&make_random_id()),
                make_retweet(&make_random_id()),
                make_retweet(&make_random_id()),
            ]),
        )
        .await
        .apply()
        .await
        .unwrap();

        handle.reload().await.unwrap();

        ScoringExpectations {
            audience_size: 50,
            repost_fatigue: true,
            ghost_account: true,
            online_engagement_score: EngagementScore::None,
            authority: 0,
            score: weihex("0"),
            handle_score: weihex("0"),
            ..ScoringExpectations::average()
        }.assert_matches(&scoring, &handle);
    })
    .await;
}

#[tokio::test]
#[serial_test::file_serial]
async fn full_authority_overrides_to_none() {
    TestHelper::run(|h| async move {
        let account = h.user().await.signed_up().await.unverified("asami_tester", user_id()).await;
        let mut handle = account.handle.clone().unwrap();
        handle = handle
            .update()
            .poll_id(Some(poll_tweet_id()))
            .online_engagement_override(Some(EngagementScore::None))
            .referrer_score_override(Some(true))
            .holder_score_override(Some(true))
            .save()
            .await
            .unwrap();

        let scoring = pre_ingested_handle_scoring(
            &handle,
            me_json("347", true),
            &[
                reply_to_own_tweet(500, 0, 2, 11, 1),
                quoting_someone_elses_tweet(300, 0, 2, 11, 1),
                tweet_hello_world(300, 0, 0, 11, 0),
                tweet_goodbye_world(550, 0, 1, 11, 0),
                tweet_foo_bar(50, 0, 0, 0, 1),
                tweet_poll(300, 0, 0, 3, 0),
            ],
            mentions_json(10, 16),
            Some(poll_json(1, 3, 4, 5)),
            reposts_json(),
        )
        .await
        .apply()
        .await
        .unwrap();

        handle.reload().await.unwrap();

        ScoringExpectations {
            offline_engagement_score: EngagementScore::None,
            authority: 0,
            score: weihex("0"),
            handle_score: weihex("0"),
            ..ScoringExpectations::default()
        }.assert_matches(&scoring, &handle);
    })
    .await;
}

#[tokio::test]
#[serial_test::file_serial]
async fn full_authority_overrides_to_none_for_operational_status() {
    TestHelper::run(|h| async move {
        let account = h.user().await.signed_up().await.unverified("asami_tester", user_id()).await;
        let mut handle = account.handle.clone().unwrap();
        handle = handle
            .update()
            .poll_id(Some(poll_tweet_id()))
            .operational_status_override(Some(OperationalStatus::Shadowbanned))
            .referrer_score_override(Some(true))
            .holder_score_override(Some(true))
            .save()
            .await
            .unwrap();

        let scoring = pre_ingested_handle_scoring(
            &handle,
            me_json("347", true),
            &[
                reply_to_own_tweet(500, 0, 2, 11, 1),
                quoting_someone_elses_tweet(300, 0, 2, 11, 1),
                tweet_hello_world(300, 0, 0, 11, 0),
                tweet_goodbye_world(550, 0, 1, 11, 0),
                tweet_foo_bar(50, 0, 0, 0, 1),
                tweet_poll(300, 0, 0, 3, 0),
            ],
            mentions_json(10, 16),
            Some(poll_json(1, 3, 4, 5)),
            reposts_json(),
        )
        .await
        .apply()
        .await
        .unwrap();

        handle.reload().await.unwrap();
        ScoringExpectations {
            offline_engagement_score: EngagementScore::None,
            authority: 0,
            score: weihex("0"),
            handle_score: weihex("0"),
            ..ScoringExpectations::default()
        }.assert_matches(&scoring, &handle);
    })
    .await;
}

#[tokio::test]
#[serial_test::file_serial]
async fn overrides_audience_size() {
    TestHelper::run(|h| async move {
        let account = h.user().await.signed_up().await.unverified("asami_tester", user_id()).await;
        let mut handle = account.handle.clone().unwrap();
        handle = handle
            .update()
            .poll_id(Some(poll_tweet_id()))
            .offline_engagement_score(EngagementScore::High)
            .referrer_score_override(Some(true))
            .holder_score_override(Some(true))
            .audience_size_override(Some(10))
            .save()
            .await
            .unwrap();

        let scoring = pre_ingested_handle_scoring(
            &handle,
            me_json("347", true),
            &[
                reply_to_own_tweet(500, 0, 2, 11, 1),
                quoting_someone_elses_tweet(300, 0, 2, 11, 1),
                tweet_hello_world(300, 0, 0, 11, 0),
                tweet_goodbye_world(550, 0, 1, 11, 0),
                tweet_foo_bar(50, 0, 0, 0, 1),
                tweet_poll(300, 0, 0, 3, 0),
            ],
            mentions_json(10, 16),
            Some(poll_json(1, 3, 4, 5)),
            reposts_json(),
        )
        .await
        .apply()
        .await
        .unwrap();

        handle.reload().await.unwrap();

        ScoringExpectations {
            score: weihex("10"),
            handle_score: weihex("10"),
            ..ScoringExpectations::default()
        }.assert_matches(&scoring, &handle);
    })
    .await;
}

#[tokio::test]
#[serial_test::file_serial]
async fn full_authority_overrides_to_average() {
    TestHelper::run(|h| async move {
        let account = h.user().await.signed_up().await.unverified("asami_tester", user_id()).await;
        let mut handle = account.handle.clone().unwrap();

        handle = handle
            .update()
            .poll_id(Some(poll_tweet_id()))
            .online_engagement_override(Some(EngagementScore::Average))
            .poll_override(Some(PollScore::None))
            .referrer_score_override(Some(false))
            .holder_score_override(Some(false))
            .operational_status_override(Some(OperationalStatus::Normal))
            .offline_engagement_score(EngagementScore::None)
            .save()
            .await
            .unwrap();

        let scoring = pre_ingested_handle_scoring(
            &handle,
            me_json("347", true),
            &[
                reply_to_own_tweet(500, 0, 2, 11, 1),
                quoting_someone_elses_tweet(300, 0, 2, 11, 1),
                tweet_hello_world(300, 0, 0, 11, 0),
                tweet_goodbye_world(550, 0, 1, 11, 0),
                tweet_foo_bar(50, 0, 0, 0, 1),
                tweet_poll(300, 0, 0, 3, 0),
            ],
            mentions_json(10, 16),
            Some(poll_json(1, 3, 4, 5)),
            reposts_json(),
        )
        .await
        .apply()
        .await
        .unwrap();

        handle.reload().await.unwrap();

        ScoringExpectations {
            offline_engagement_score: EngagementScore::None,
            referrer_score_override: Some(false),
            holder_score_override: Some(false),
            authority: 25,
            score: weihex("79"),
            handle_score: weihex("79"),
            ..ScoringExpectations::default()
        }.assert_matches(&scoring, &handle);
    })
    .await;
}

#[tokio::test]
#[serial_test::file_serial]
async fn ghost_account_overrides_to_full() {
    TestHelper::run(|h| async move {
        let account = h.user().await.signed_up().await.unverified("asami_tester", user_id()).await;
        let mut handle = account.handle.clone().unwrap();
        handle = handle
            .update()
            .poll_override(Some(PollScore::High))
            .referrer_score_override(Some(true))
            .holder_score_override(Some(true))
            .offline_engagement_score(EngagementScore::High)
            .online_engagement_override(Some(EngagementScore::High))
            .save()
            .await
            .unwrap();

        let scoring = pre_ingested_handle_scoring(
            &handle,
            me_json("0", false),
            &[
                reply_to_own_tweet(50, 0, 0, 1, 0),
                quoting_someone_elses_tweet(31, 0, 0, 1, 0),
                tweet_hello_world(31, 0, 0, 1, 0),
                tweet_goodbye_world(55, 0, 0, 1, 0),
                make_own_tweet("GM", 20, 1, 0, 1, 0),
                make_own_tweet("My friends!", 20, 1, 0, 1, 0),
                make_own_tweet("LFG", 10, 0, 0, 0, 0),
            ],
            tweets_json([]),
            None,
            reposts_json(),
        )
        .await
        .apply()
        .await
        .unwrap();

        handle.reload().await.unwrap();

        ScoringExpectations {
            audience_size: 31,
            online_engagement_score: EngagementScore::None,
            offline_engagement_score: EngagementScore::High,
            referrer_score_override: Some(true),
            holder_score_override: Some(true),
            poll_score: Some(PollScore::None),
            ghost_account: true,
            indeterminate_audience: true,
            authority: 100,
            score: weihex("31"),
            handle_score: weihex("31"),
            ..ScoringExpectations::average()
        }.assert_matches(&scoring, &handle);
    })
    .await;
}

#[tokio::test]
#[serial_test::file_serial]
async fn average_poll_score() {
    TestHelper::run(|h| async move {
        let account = h.user().await.signed_up().await.unverified("asami_tester", user_id()).await;
        let mut handle = account.handle.clone().unwrap();
        handle = handle
            .update()
            .poll_id(Some(poll_tweet_id()))
            .referrer_score_override(Some(true))
            .holder_score_override(Some(true))
            .offline_engagement_score(EngagementScore::High)
            .save()
            .await
            .unwrap();

        let scoring = pre_ingested_handle_scoring(
            &handle,
            me_json("347", false),
            &[
                reply_to_own_tweet(500, 0, 2, 11, 1),
                quoting_someone_elses_tweet(300, 0, 2, 11, 1),
                tweet_hello_world(300, 0, 0, 11, 0),
                tweet_goodbye_world(550, 0, 1, 11, 0),
                tweet_foo_bar(50, 0, 0, 0, 1),
                tweet_poll(300, 0, 0, 3, 0),
            ],
            mentions_json(10, 16),
            Some(poll_json(1, 4, 3, 5)),
            reposts_json(),
        )
        .await
        .apply()
        .await
        .unwrap();

        handle.reload().await.unwrap();

        ScoringExpectations {
            audience_size: 316,
            poll_score: Some(PollScore::Average),
            authority: 90,
            score: weihex("284"),
            handle_score: weihex("284"),
            operational_status_score: OperationalStatus::Normal,
            ..ScoringExpectations::default()
        }.assert_matches(&scoring, &handle);
    })
    .await;
}

#[tokio::test]
#[serial_test::file_serial]
async fn reverse_poll_score() {
    TestHelper::run(|h| async move {
        let account = h.user().await.signed_up().await.unverified("asami_tester", user_id()).await;
        let mut handle = account.handle.clone().unwrap();
        handle = handle.update().poll_id(Some(poll_tweet_id())).save().await.unwrap();

        let scoring = pre_ingested_handle_scoring(
            &handle,
            me_json("0", false),
            &[
                reply_to_own_tweet(50, 0, 0, 11, 0),
                quoting_someone_elses_tweet(50, 0, 0, 11, 0),
                tweet_hello_world(30, 0, 0, 11, 0),
                tweet_goodbye_world(55, 0, 1, 11, 0),
                tweet_foo_bar(50, 0, 0, 0, 1),
                tweet_poll(50, 0, 0, 3, 0),
            ],
            mentions_json(0, 0),
            Some(poll_json(1, 3, 4, 8)),
            reposts_json(),
        )
        .await
        .apply()
        .await
        .unwrap();

        handle.reload().await.unwrap();

        ScoringExpectations {
            audience_size: 48,
            poll_score: Some(PollScore::Reverse),
            indeterminate_audience: true,
            authority: 12,
            score: weihex("5"),
            handle_score: weihex("5"),
            ..ScoringExpectations::average()
        }.assert_matches(&scoring, &handle);
    })
    .await;
}

#[tokio::test]
#[serial_test::file_serial]
async fn none_poll_score() {
    TestHelper::run(|h| async move {
        let account = h.user().await.signed_up().await.unverified("asami_tester", user_id()).await;
        let mut handle = account.handle.clone().unwrap();
        handle = handle.update().poll_id(Some(poll_tweet_id())).save().await.unwrap();

        let scoring = pre_ingested_handle_scoring(
            &handle,
            me_json("0", false),
            &[
                reply_to_own_tweet(50, 0, 0, 11, 0),
                quoting_someone_elses_tweet(50, 0, 0, 11, 0),
                tweet_hello_world(30, 0, 0, 11, 0),
                tweet_goodbye_world(55, 0, 1, 11, 0),
                tweet_foo_bar(50, 0, 0, 0, 1),
                tweet_poll(50, 0, 0, 3, 0),
            ],
            mentions_json(0, 0),
            Some(poll_json(10, 3, 4, 8)),
            reposts_json(),
        )
        .await
        .apply()
        .await
        .unwrap();

        handle.reload().await.unwrap();

        ScoringExpectations {
            audience_size: 48,
            poll_score: Some(PollScore::None),
            indeterminate_audience: true,
            authority: 25,
            score: weihex("12"),
            handle_score: weihex("12"),
            ..ScoringExpectations::average()
        }.assert_matches(&scoring, &handle);
    })
    .await;
}

#[tokio::test]
#[serial_test::file_serial]
async fn override_poll_score() {
    TestHelper::run(|h| async move {
        let account = h.user().await.signed_up().await.unverified("asami_tester", user_id()).await;
        let mut handle = account.handle.clone().unwrap();
        handle = handle
            .update()
            .poll_id(Some(poll_tweet_id()))
            .poll_override(Some(PollScore::High))
            .save()
            .await
            .unwrap();

        let scoring = pre_ingested_handle_scoring(
            &handle,
            me_json("0", false),
            &[
                reply_to_own_tweet(50, 0, 0, 11, 0),
                quoting_someone_elses_tweet(50, 0, 0, 11, 0),
                tweet_hello_world(30, 0, 0, 11, 0),
                tweet_goodbye_world(55, 0, 1, 11, 0),
                tweet_foo_bar(50, 0, 0, 0, 1),
                tweet_poll(50, 0, 0, 3, 0),
            ],
            mentions_json(0, 0),
            Some(poll_json(1, 3, 4, 8)),
            reposts_json(),
        )
        .await
        .apply()
        .await
        .unwrap();

        handle.reload().await.unwrap();

        ScoringExpectations {
            audience_size: 48,
            poll_score: Some(PollScore::Reverse),
            indeterminate_audience: true,
            authority: 45,
            score: weihex("21"),
            handle_score: weihex("21"),
            ..ScoringExpectations::average()
        }.assert_matches(&scoring, &handle);
    })
    .await;
}

#[tokio::test]
#[serial_test::file_serial]
async fn does_not_count_own_replies_and_reposts() {
    TestHelper::run(|h| async move {
        let account = h.user().await.signed_up().await.unverified("asami_tester", user_id()).await;
        let mut handle = account.handle.clone().unwrap();
        handle = handle
            .update()
            .poll_id(Some(poll_tweet_id()))
            .referrer_score_override(Some(true))
            .holder_score_override(Some(true))
            .offline_engagement_score(EngagementScore::High)
            .save()
            .await
            .unwrap();

        let posts = [
            reply_to_own_tweet(500, 0, 2, 11, 1),
            quoting_someone_elses_tweet(300, 0, 2, 11, 1),
            tweet_hello_world(300, 0, 0, 11, 0),
            tweet_goodbye_world(550, 0, 1, 11, 0),
            tweet_foo_bar(50, 0, 0, 0, 1),
            tweet_poll(300, 0, 0, 3, 0),
        ];

        let referenced = [
            make_reply(posts[0]["id"].as_str().unwrap()),
            make_reply(posts[0]["id"].as_str().unwrap()),
            make_quote(posts[0]["id"].as_str().unwrap()),
            make_reply(posts[1]["id"].as_str().unwrap()),
            make_reply(posts[1]["id"].as_str().unwrap()),
            make_quote(posts[1]["id"].as_str().unwrap()),
        ];

        let scoring = pre_ingested_handle_scoring(
            &handle,
            me_json("347", true),
            &posts,
            mentions_json(10, 16),
            Some(poll_json(1, 3, 4, 5)),
            tweets_json(referenced),
        )
        .await
        .apply()
        .await
        .unwrap();

        handle.reload().await.unwrap();

        ScoringExpectations {
            poll_score: Some(PollScore::High),
            replied: false,
            reposted: false,
            ..ScoringExpectations::default()
        }.assert_matches(&scoring, &handle);
    })
    .await;
}

async fn pre_ingested_handle_scoring(
    handle: &Handle,
    me: Value,
    tweets: impl AsRef<[Value]>,
    mentions: Value,
    poll: Option<Value>,
    reposts: Value,
) -> HandleScoring {
    handle
        .state
        .handle_scoring()
        .insert(InsertHandleScoring {
            handle_id: *handle.id(),
        })
        .save()
        .await
        .unwrap()
        .update()
        .status(HandleScoringStatus::Ingested)
        .me_json(Some(me.to_string()))
        .tweets_json(Some(tweets_json(tweets).to_string()))
        .mentions_json(Some(mentions.to_string()))
        .reposts_json(Some(reposts.to_string()))
        .poll_json(poll.map(|x| x.to_string()))
        .save()
        .await
        .unwrap()
}

fn user_id() -> &'static str {
    "1912953217563942912"
}

fn me_json(followers: &str, verified: bool) -> Value {
    json::json![{
      "data": {
        "id": user_id(),
        "name": "Asami Acceptance Tester",
        "username": "asami_tester",
        "created_at": "2025-04-17T19:36:23Z",
        "description": "",
        "protected": false,
        "public_metrics": {
          "followers_count": 1,
          "following_count": 3,
          "tweet_count": 14,
          "listed_count": 0
        },
        "verified": verified,
        "parody": false,
        "verified_followers_count": followers, // field missing if not verified.
        "verified_type": "none", // or "blue"
        "subscription": {
          "subscribes_to_you": false
        },
        "subscription_type": "None" // or "Premium"
      }
    }]
}

fn tweets_json(data: impl AsRef<[Value]>) -> Value {
    let val = data.as_ref();
    json::json![{
      "data": val,
      "meta": {
        "result_count": val.len(),
        "newest_id": "1915364366796444108",
        "oldest_id": "1913185659373707387"
      }
    }]
}

fn empty_result_set() -> Value {
    json::json![{"meta":{"result_count":0}}]
}

fn mentions_json(retweets: u64, likes: u64) -> Value {
    json::json![{
      "data": [
        {
          "id": "1914683302000173290",
          "text": "Seguimos stremeando y probando con el @asami_tester así que le hago un mention para ver que pasa.",
          "author_id": "179383862",
          "conversation_id": "1914683302000173289",
          "created_at": "2025-04-22T14:10:46Z",
          "entities": {
            "mentions": [
              {
                "start": 38,
                "end": 51,
                "username": "asami_tester",
                "id": "1912953217563942912"
              }
            ]
          },
          "lang": "es",
          "possibly_sensitive": false,
          "public_metrics": {
            "retweet_count": retweets,
            "reply_count"  : 0,
            "like_count"   : likes,
            "quote_count"  : 0
          },
          "reply_settings": "everyone"
        },
        {
          "id": "1914683302000173289",
          "text": "Otro mention a @asami_tester.",
          "author_id": "179383862",
          "conversation_id": "1914683302000173289",
          "created_at": "2025-04-22T14:10:46Z",
          "entities": {
            "mentions": [
              {
                "start": 38,
                "end": 51,
                "username": "asami_tester",
                "id": "1912953217563942912"
              }
            ]
          },
          "lang": "es",
          "possibly_sensitive": false,
          "public_metrics": {
            "retweet_count": retweets,
            "reply_count"  : 0,
            "like_count"   : likes,
            "quote_count"  : 0
          },
          "reply_settings": "everyone"
        }
      ],
      "meta": {
        "result_count": 1,
        "newest_id": "1914683302000173290",
        "oldest_id": "1914683302000173289"
      },
      "includes": {
        "users": [
          {
            "id": "179383862",
            "name": "Nubis 🥜 - ultra fun money 🦇🤡",
            "username": "nubis_bruno",
            "created_at": "2010-08-17T04:22:13Z",
            "description": "@asami_club | @constataeu | Street level Bitcoin Maxi | Rootstockr  | ThinkPads | Rustlang | NNN | npub1uu54sz460dxkq8y578vunwjlxlnqvmpz6y63aaw5n2z3m6qjzxus6h",
            "protected": false,
            "public_metrics": {
              "followers_count":  4104,
              "following_count":  1851,
              "tweet_count"    : 31217,
              "listed_count"   :   144
            },
            "verified": true,
            "parody": false,
            "verified_type": "blue",
            "subscription": {"subscribes_to_you": false},
            "subscription_type": "None"
          }
        ]
      }
    }]
}

fn poll_tweet_id() -> String {
    "1915440701615022180".to_string()
}

fn poll_json(none: u64, average: u64, high: u64, reverse: u64) -> Value {
    json::json![{
      "data": {
        "id": poll_tweet_id(),
        "text": "If you told you to buy me a coke, what would you do?",
        "attachments": { "poll_ids": ["1915440701510139904"] },
        "author_id": "1912953217563942912",
        "conversation_id": "1915440701615022180",
        "created_at": "2025-04-24T16:20:25Z",
        "lang": "en",
        "non_public_metrics": {"impression_count": 0},
        "organic_metrics": {
          "impression_count": 0,
          "retweet_count"   : 0,
          "reply_count"     : 0,
          "like_count"      : 0
        },
        "possibly_sensitive": false,
        "public_metrics": {
          "retweet_count": 0,
          "reply_count"  : 0,
          "like_count"   : 0,
          "quote_count"  : 0
        },
        "reply_settings": "everyone"
      },
      "includes": {
        "polls": [
          {
            "id": "1915440701510139904",
            "options": [
              {"position": 1, "label": "None: I wouldn't", "votes": none},
              {"position": 2, "label": "Average: I would", "votes": average},
              {"position": 3, "label": "High: Get you 2.", "votes": high},
              {"position": 4, "label": "Reverse: Pepsi", "votes": reverse}
            ],
            "voting_status": "open"
          }
        ]
      }
    }]
}

pub fn reposts_json() -> Value {
    serde_json::json![
      {
        "data": [
          {
            "id": "1915751867046367667",
            "text": "RT @asami_tester: And here I quote my own tweet.",
            "created_at": "2025-04-25T12:56:52Z",
            "referenced_tweets": [
              {"type": "retweeted", "id": "1915751819168383001"}
            ]
          },
          {
            "id": "1915751819168383001",
            "text": "And here I quote my own tweet. https://t.co/i69mvXzkG4",
            "created_at": "2025-04-25T12:56:41Z",
            "referenced_tweets": [ {"type": "quoted", "id": "1915364366796444108"} ]
          },
          {
            "id": "1915751715191562722",
            "text": "RT @asami_tester: And replies to my own tweets.",
            "created_at": "2025-04-25T12:56:16Z",
            "referenced_tweets": [
              {"type": "retweeted", "id": "1915364225507119328"}
            ]
          },
          {
            "id": "1915364366796444108",
            "text": "Oh. Too bad. Not many impressions here.",
            "created_at": "2025-04-24T11:17:05Z",
            "referenced_tweets": [
              {"type": "replied_to", "id": "1914718611517698226"}
            ]
          },
          {
            "id": "1915364225507119328",
            "text": "And replies to my own tweets.",
            "created_at": "2025-04-24T11:16:31Z",
            "referenced_tweets": [
              {"type": "replied_to", "id": "1915364126894838112"}
            ]
          },
          {
            "id": "1915364126894838112",
            "text": "Example quote tweet here. https://t.co/CHTOSFRdeR",
            "created_at": "2025-04-24T11:16:08Z",
            "referenced_tweets": [ {"type": "quoted", "id": "1911875257733959859"} ]
          },
          {
            "id": "1915364064110489854",
            "text": "RT @asami_club: Asami is a club where members pay each other for reposts on X. You can get your content to go viral, you can get paid to re…",
            "created_at": "2025-04-24T11:15:53Z",
            "referenced_tweets": [
              {"type": "retweeted", "id": "1911875257733959859"}
            ]
          }
        ]
      }
    ]
}

pub fn reply_to_own_tweet(
    impression_count: u64,
    retweet_count: u64,
    reply_count: u64,
    like_count: u64,
    quote_count: u64,
) -> Value {
    serde_json::json!({
        "id": "1915364225507119328",
        "text": "And replies to my own tweets.",
        "author_id": "1912953217563942912",
        "lang": "en",
        "non_public_metrics": {
            "impression_count": impression_count
        },
        "organic_metrics": {
            "impression_count": impression_count,
            "retweet_count": retweet_count,
            "reply_count": reply_count,
            "like_count": like_count
        },
        "public_metrics": {
            "retweet_count": retweet_count,
            "reply_count": reply_count,
            "like_count": like_count,
            "quote_count": quote_count
        },
        "reply_settings": "everyone",
        "in_reply_to_user_id": "1912953217563942912",
        "conversation_id": "1915364126894838112"
    })
}

pub fn quoting_someone_elses_tweet(
    impression_count: u64,
    retweet_count: u64,
    reply_count: u64,
    like_count: u64,
    quote_count: u64,
) -> Value {
    serde_json::json!({
        "id": "1915364126894838112",
        "text": "Example of a quote tweet here, looks good. https://t.co/CHTOSFRdeR",
        "author_id": "1912953217563942912",
        "lang": "en",
        "non_public_metrics": {
            "impression_count": impression_count
        },
        "organic_metrics": {
            "impression_count": impression_count,
            "retweet_count": retweet_count,
            "reply_count": reply_count,
            "like_count": like_count
        },
        "public_metrics": {
            "retweet_count": retweet_count,
            "reply_count": reply_count,
            "like_count": like_count,
            "quote_count": quote_count
        },
        "reply_settings": "everyone",
        "conversation_id": "1915364126894838112",
        "entities": {
            "urls": [
                { "url": "https://t.co/CHTOSFRdeR" }
            ]
        },
        "referenced_tweets": [
            { "type": "quoted", "id": "1911875257733959859" }
        ]
    })
}

pub fn make_own_tweet(text: &str, impressions: u64, retweets: u64, replies: u64, likes: u64, quotes: u64) -> Value {
    let tweet_id = make_random_id();
    serde_json::json!({
        "id": tweet_id,
        "text": text,
        "author_id": user_id(),
        "lang": "en",
        "non_public_metrics": {
            "impression_count": impressions
        },
        "organic_metrics": {
            "impression_count": impressions,
            "retweet_count": retweets,
            "reply_count": replies,
            "like_count": likes
        },
        "public_metrics": {
            "retweet_count": retweets,
            "reply_count": replies,
            "like_count": likes,
            "quote_count": quotes
        },
        "reply_settings": "everyone",
        "conversation_id": tweet_id,
    })
}

fn make_random_id() -> String {
    thread_rng().gen_range(10u64.pow(18)..10u64.pow(19)).to_string()
}

pub fn tweet_hello_world(impressions: u64, retweets: u64, replies: u64, likes: u64, quotes: u64) -> Value {
    make_own_tweet(
        "Hello World! We're live testing new features in Asami Club, hopefully with interesting tweets.",
        impressions,
        retweets,
        replies,
        likes,
        quotes,
    )
}

pub fn tweet_goodbye_world(impressions: u64, retweets: u64, replies: u64, likes: u64, quotes: u64) -> Value {
    make_own_tweet(
        "Goodbye World! You've been great, I'll see you next weekend though.",
        impressions,
        retweets,
        replies,
        likes,
        quotes,
    )
}

pub fn tweet_foo_bar(impressions: u64, retweets: u64, replies: u64, likes: u64, quotes: u64) -> Value {
    make_own_tweet(
        "It's interesting to say something interesting, and long also.",
        impressions,
        retweets,
        replies,
        likes,
        quotes,
    )
}

pub fn make_retweet(tweet_id: &str) -> Value {
    serde_json::json![
          {
            "id": make_random_id(),
            "text": "RT this is a retweet",
            "created_at": "2025-04-24T11:15:53Z",
            "referenced_tweets": [
              {"type": "retweeted", "id": tweet_id}
            ]
          }
    ]
}

pub fn make_reply(tweet_id: &str) -> Value {
    serde_json::json![
          {
            "id": make_random_id(),
            "text": "I'm replying to this tweet",
            "created_at": "2025-04-24T11:15:53Z",
            "referenced_tweets": [
              {"type": "replied_to", "id": tweet_id}
            ]
          }
    ]
}

pub fn make_quote(tweet_id: &str) -> Value {
    serde_json::json![
          {
            "id": make_random_id(),
            "text": "Quoting this tweet",
            "created_at": "2025-04-24T11:15:53Z",
            "referenced_tweets": [
              {"type": "quoted", "id": tweet_id}
            ]
          }
    ]
}

pub fn tweet_poll(
    impression_count: u64,
    retweet_count: u64,
    reply_count: u64,
    like_count: u64,
    quote_count: u64,
) -> Value {
    serde_json::json!({
        "id": "1913187163358835110",
        "text": "If I told you to jump off a bridge, would you do it? Would you even think about it?",
        "author_id": "1912953217563942912",
        "lang": "en",
        "non_public_metrics": {
            "impression_count": impression_count
        },
        "organic_metrics": {
            "impression_count": impression_count,
            "retweet_count": retweet_count,
            "reply_count": reply_count,
            "like_count": like_count
        },
        "public_metrics": {
            "retweet_count": retweet_count,
            "reply_count": reply_count,
            "like_count": like_count,
            "quote_count": quote_count
        },
        "reply_settings": "everyone",
        "attachments": { "poll_ids": ["1913187163258130432"] },
        "conversation_id": "1913187163358835110"
    })
}

pub struct ScoringExpectations {
    pub status: HandleScoringStatus,
    pub audience_size: i32,
    pub repost_fatigue: bool,
    pub ghost_account: bool,
    pub indeterminate_audience: bool,
    pub followed: bool,
    pub liked: bool,
    pub replied: bool,
    pub reposted: bool,
    pub mentioned: bool,
    pub online_engagement_score: EngagementScore,
    pub offline_engagement_score: EngagementScore,
    pub operational_status_score: OperationalStatus,
    pub poll_score: Option<PollScore>,
    pub referrer_score_override: Option<bool>,
    pub holder_score_override: Option<bool>,
    pub authority: i32,
    pub score: String,
    pub handle_status: HandleStatus,
    pub handle_score: String,
}

impl ScoringExpectations {
    fn average() -> Self {
        ScoringExpectations {
            audience_size: 32,
            followed: false,
            liked: false,
            replied: false,
            reposted: false,
            mentioned: false,
            poll_score: Some(PollScore::None),
            online_engagement_score: EngagementScore::Average,
            offline_engagement_score: EngagementScore::None,
            operational_status_score: OperationalStatus::Normal,
            referrer_score_override: None,
            holder_score_override: None,
            authority: 25,
            score: weihex("8"),
            handle_score: weihex("8"),
            ..Default::default()
        }
    }
}

impl Default for ScoringExpectations {
    fn default() -> Self {
        Self {
            status: HandleScoringStatus::Applied,
            audience_size: 316,
            repost_fatigue: false,
            ghost_account: false,
            indeterminate_audience: false,
            followed: true,
            liked: true,
            replied: true,
            reposted: true,
            mentioned: true,
            online_engagement_score: EngagementScore::High,
            offline_engagement_score: EngagementScore::High,
            operational_status_score: OperationalStatus::Enhanced,
            poll_score: Some(PollScore::High),
            referrer_score_override: Some(true),
            holder_score_override: Some(true),
            authority: 100,
            score: weihex("316"),
            handle_status: HandleStatus::Active,
            handle_score: weihex("316"),
        }
    }
}

impl ScoringExpectations {
    pub fn assert_matches(&self, scoring: &HandleScoring, handle: &Handle) {
        assert_eq!(*scoring.status(), self.status);
        assert_eq!(*scoring.audience_size(), self.audience_size);
        assert_eq!(*scoring.repost_fatigue(), self.repost_fatigue);
        assert_eq!(*scoring.ghost_account(), self.ghost_account);
        assert_eq!(*scoring.indeterminate_audience(), self.indeterminate_audience);
        assert_eq!(*scoring.followed(), self.followed);
        assert_eq!(*scoring.liked(), self.liked);
        assert_eq!(*scoring.replied(), self.replied);
        assert_eq!(*scoring.reposted(), self.reposted);
        assert_eq!(*scoring.mentioned(), self.mentioned);
        assert_eq!(*scoring.online_engagement_score(), self.online_engagement_score);
        assert_eq!(*scoring.offline_engagement_score(), self.offline_engagement_score);
        assert_eq!(*scoring.operational_status_score(), self.operational_status_score);
        assert_eq!(*scoring.poll_score(), self.poll_score);
        assert_eq!(*scoring.referrer_score_override(), self.referrer_score_override);
        assert_eq!(*scoring.holder_score_override(), self.holder_score_override);
        assert_eq!(*scoring.authority(), self.authority);
        assert_eq!(scoring.score().as_ref().unwrap(), &self.score);
        assert_eq!(handle.current_scoring_id().unwrap(), *scoring.id());
        assert_eq!(handle.last_scoring().unwrap(), *scoring.created_at());
        assert_eq!(*handle.status(), self.handle_status);
        assert_eq!(handle.score().as_ref().unwrap(), &self.handle_score);
    }
}
