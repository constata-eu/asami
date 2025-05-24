use api::models::{Handle, HandleScoring, HandleScoringStatus, InsertHandleScoring};
use rand::{thread_rng, Rng};
use rocket::serde::json;
use serde_json::Value;

pub async fn pre_ingested_handle_scoring(
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

pub fn me_json(followers: &str, verified: bool) -> Value {
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

pub fn tweets_json(data: impl AsRef<[Value]>) -> Value {
    json::json![{
      "data": data.as_ref(),
      "meta": {
        "result_count": 6,
        "newest_id": "1915364366796444108",
        "oldest_id": "1913185659373707387"
      }
    }]
}

pub fn mentions_json(retweets: u64, likes: u64) -> Value {
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

pub fn poll_json(none: u64, average: u64, high: u64, reverse: u64) -> Value {
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
