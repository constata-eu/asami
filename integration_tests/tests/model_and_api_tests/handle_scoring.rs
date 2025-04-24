use models::{
    weihex, Handle, HandleScoring, HandleStatus, InsertHandleScoring,
};
use rocket::serde::json;

use super::*;

#[tokio::test(flavor = "multi_thread")]
async fn scores_unused_handle() {
    let h = TestHelper::for_model().await;
    let account = h.app.create_account().await;
    let mut handle = h.app.create_handle(&account.attrs.id, "asami_tester", user_id(), u("0")).await;

    let scoring = test_handle_scoring(&handle).await.apply().await.unwrap();

    handle.reload().await.unwrap();

    assert_eq!(handle.current_scoring_id().unwrap(), *scoring.id());
    assert_eq!(handle.last_scoring().unwrap(), *scoring.created_at());
    assert_eq!(*handle.status(), HandleStatus::Active);
    assert_eq!(*handle.score().as_ref().unwrap(), weihex("0"));
}

async fn test_handle_scoring(handle: &Handle) -> HandleScoring {
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
        .me_json(Some(me_json()))
        .tweets_json(Some(tweets_json()))
        .mentions_json(Some(mentions_json()))
        .save()
        .await
        .unwrap()
}

fn user_id() -> &'static str {
    "1912953217563942912"
}

fn me_json() -> String {
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
        "verified": false,
        "parody": false,
        "verified_type": "none",
        "subscription": {
          "subscribes_to_you": false
        },
        "subscription_type": "None"
      }
    }]
    .to_string()
}

fn tweets_json() -> String {
    json::json![{
      "data": [
        {
          "id": "1914718611517698226",
          "text": "Hello World! We're live testing new features in Asami Club.",
          "author_id": "1912953217563942912",
          "conversation_id": "1914718611517698226",
          "created_at": "2025-04-22T16:31:05Z",
          "entities": {
            "annotations": [
              {
                "start": 48,
                "end": 57,
                "probability": 0.8074,
                "type": "Other",
                "normalized_text": "Asami Club"
              }
            ]
          },
          "lang": "en",
          "non_public_metrics": {
            "impression_count": 0
          },
          "organic_metrics": {
            "impression_count": 0,
            "retweet_count": 1,
            "reply_count": 0,
            "like_count": 0
          },
          "possibly_sensitive": false,
          "public_metrics": {
            "retweet_count": 1,
            "reply_count": 0,
            "like_count": 0,
            "quote_count": 0
          },
          "reply_settings": "everyone"
        },
        {
          "id": "1913187163358835110",
          "text": "What would you do",
          "attachments": {
            "poll_ids": [
              "1913187163258130432"
            ]
          },
          "author_id": "1912953217563942912",
          "conversation_id": "1913187163358835110",
          "created_at": "2025-04-18T11:05:39Z",
          "lang": "en",
          "non_public_metrics": {
            "impression_count": 7
          },
          "organic_metrics": {
            "impression_count": 7,
            "retweet_count": 0,
            "reply_count": 0,
            "like_count": 0
          },
          "possibly_sensitive": false,
          "public_metrics": {
            "retweet_count": 0,
            "reply_count": 0,
            "like_count": 0,
            "quote_count": 0
          },
          "reply_settings": "everyone"
        },
        {
          "id": "1913185659373707387",
          "text": "What would you do",
          "attachments": {
            "poll_ids": [
              "1913185659294023680"
            ]
          },
          "author_id": "1912953217563942912",
          "conversation_id": "1913185659373707387",
          "created_at": "2025-04-18T10:59:41Z",
          "lang": "en",
          "non_public_metrics": {
            "impression_count": 7
          },
          "organic_metrics": {
            "impression_count": 7,
            "retweet_count": 0,
            "reply_count": 0,
            "like_count": 1
          },
          "possibly_sensitive": false,
          "public_metrics": {
            "retweet_count": 0,
            "reply_count": 0,
            "like_count": 1,
            "quote_count": 0
          },
          "reply_settings": "everyone"
        }
      ],
      "meta": {
        "result_count": 3,
        "newest_id": "1914718611517698226",
        "oldest_id": "1913185659373707387"
      }
    }]
    .to_string()
}

fn mentions_json() -> String {
    json::json![{
      "data": [
        {
          "id": "1914683302000173289",
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
            "retweet_count": 0,
            "reply_count": 0,
            "like_count": 0,
            "quote_count": 0
          },
          "reply_settings": "everyone"
        }
      ],
      "meta": {
        "result_count": 1,
        "newest_id": "1914683302000173289",
        "oldest_id": "1914683302000173289"
      }
    }]
    .to_string()
}
