use std::collections::HashMap;

use twitter_v2::{
    authorization::Oauth2Token,
    data::ReferencedTweetKind,
    id::NumericId,
    meta::TweetsMeta,
    query::{Exclude, MediaField, PollField, TweetExpansion, TweetField, UserField},
    ApiPayload, Tweet, TwitterApi, User as TwitterUser,
};

use super::*;

model! {
  state: App,
  table: handle_scorings,
  struct HandleScoring {
    #[sqlx_model_hints(int4, default, op_in)]
    id: i32,
    #[sqlx_model_hints(int4)]
    handle_id : i32,
    #[sqlx_model_hints(handle_scoring_status)]
    status: HandleScoringStatus,
    #[sqlx_model_hints(timestamptz, default)]
    created_at: DateTime<Utc>,
    #[sqlx_model_hints(text)]
    me_json: Option<String>,
    #[sqlx_model_hints(text)]
    tweets_json: Option<String>,
    #[sqlx_model_hints(text)]
    mentions_json: Option<String>,
    #[sqlx_model_hints(text)]
    reposts_json: Option<String>,
    #[sqlx_model_hints(text)]
    poll_json: Option<String>,

    #[sqlx_model_hints(int4, default)]
    post_count: i32,
    #[sqlx_model_hints(int4, default)]
    impression_count: i32,

    #[sqlx_model_hints(boolean, default)]
    ghost_account: bool,
    #[sqlx_model_hints(boolean, default)]
    repost_fatigue: bool,
    #[sqlx_model_hints(boolean, default)]
    indeterminate_audience: bool,
    #[sqlx_model_hints(boolean, default)]
    followed: bool,
    #[sqlx_model_hints(boolean, default)]
    liked: bool,
    #[sqlx_model_hints(boolean, default)]
    replied: bool,
    #[sqlx_model_hints(boolean, default)]
    reposted: bool,
    #[sqlx_model_hints(boolean, default)]
    mentioned: bool,

    #[sqlx_model_hints(engagement_score, default)]
    online_engagement_score: EngagementScore,
    #[sqlx_model_hints(engagement_score, default)]
    online_engagement_override: Option<EngagementScore>,
    #[sqlx_model_hints(text, default)]
    online_engagement_override_reason: Option<String>,

    #[sqlx_model_hints(engagement_score, default)]
    offline_engagement_score: EngagementScore,
    #[sqlx_model_hints(text, default)]
    offline_engagement_description: Option<String>,

    #[sqlx_model_hints(varchar, default)]
    poll_id: Option<String>,
    #[sqlx_model_hints(poll_score, default)]
    poll_score: Option<PollScore>,
    #[sqlx_model_hints(poll_score, default)]
    poll_override: Option<PollScore>,
    #[sqlx_model_hints(varchar, default)]
    poll_override_reason: Option<String>,

    #[sqlx_model_hints(operational_status, default)]
    operational_status_score: OperationalStatus,
    #[sqlx_model_hints(operational_status, default)]
    operational_status_override: Option<OperationalStatus>,
    #[sqlx_model_hints(text, default)]
    operational_status_override_reason: Option<String>,

    #[sqlx_model_hints(boolean, default)]
    referrer_score: bool,
    #[sqlx_model_hints(boolean, default)]
    referrer_score_override: Option<bool>,
    #[sqlx_model_hints(text, default)]
    referrer_score_override_reason: Option<String>,

    #[sqlx_model_hints(boolean, default)]
    holder_score: bool,
    #[sqlx_model_hints(boolean, default)]
    holder_score_override: Option<bool>,
    #[sqlx_model_hints(text, default)]
    holder_score_override_reason: Option<String>,

    #[sqlx_model_hints(int4, default)]
    authority: i32,

    #[sqlx_model_hints(int4, default)]
    audience_size: i32,
    #[sqlx_model_hints(int4, default)]
    audience_size_override: Option<i32>,
    #[sqlx_model_hints(text, default)]
    audience_size_override_reason: Option<String>,

    #[sqlx_model_hints(varchar, default)]
    score: Option<String>,

  },
  belongs_to {
      Handle(handle_id)
  }
}

const NOT_ENOUGH_IMPRESSIONS: usize = 300;

impl_loggable!(HandleScoring);

impl HandleScoringHub {
    pub async fn create_and_apply(&self, item: Handle) -> AsamiResult<()> {
        let id = *item.id();

        match self.create(item).await {
            Ok(scoring) => {
                scoring.apply().await?;
            }
            Err(e) => {
                self.state.fail("score_pending", "creating_scoring", format!("handle:{id} {e:?}")).await;
            }
        }

        Ok(())
    }

    pub async fn create(&self, handle: Handle) -> AsamiResult<HandleScoring> {
        let api = handle.clone().x_api_client().await?;

        let (me, tweets, mentions, reposts, maybe_poll) = self.ingest(api, handle.poll_id()).await?;

        Ok(self
            .insert(InsertHandleScoring {
                handle_id: handle.attrs.id,
                status: HandleScoringStatus::Ingested,
                me_json: Some(me),
                tweets_json: Some(tweets),
                mentions_json: Some(mentions),
                reposts_json: Some(reposts),
                poll_json: maybe_poll,
            })
            .save()
            .await?)
    }

    pub async fn ingest(
        &self,
        api: TwitterApi<Oauth2Token>,
        poll_id: &Option<String>,
    ) -> AsamiResult<(String, String, String, String, Option<String>)> {
        let me = api.get_users_me().user_fields(build_user_fields()).send().await?;

        let me_json = serde_json::to_string(&me)?;

        let Some(user_id) = me.payload().data().map(|u| u.id) else {
            return Err(Error::runtime(&format!("user_data_was_empty: {me:?}")));
        };

        let now = Utc::now();

        // The window to score is one month starting a week ago.
        // Se we don't count old tweets, but we also give recent
        // tweets a week to gain impressions.
        let start_time = time::OffsetDateTime::from_unix_timestamp((now - chrono::Duration::days(30)).timestamp())?
            .to_offset(time::UtcOffset::UTC);

        let end_time = time::OffsetDateTime::from_unix_timestamp((now - chrono::Duration::days(0)).timestamp())?
            .to_offset(time::UtcOffset::UTC);

        self.state.info("score_pending", "getting_tweets_json", ()).await;
        let tweets_json = serde_json::to_string(
            &api.get_user_tweets(user_id)
                .max_results(100)
                .start_time(start_time)
                .end_time(end_time)
                .exclude([Exclude::Retweets])
                .expansions(build_expansions())
                .user_fields(build_user_fields())
                .tweet_fields(build_tweet_fields(&[
                    TweetField::OrganicMetrics,
                    TweetField::NonPublicMetrics,
                ]))
                .media_fields(build_media_fields(&[
                    MediaField::NonPublicMetrics,
                    MediaField::OrganicMetrics,
                    MediaField::PromotedMetrics,
                ]))
                .poll_fields([PollField::VotingStatus, PollField::Options, PollField::Id])
                .send()
                .await?,
        )?;
        self.state.info("score_pending", "sleeping", ()).await;
        tokio::time::sleep(tokio::time::Duration::from_millis(
            self.state.settings.x.score_cooldown_seconds * 500,
        ))
        .await;

        self.state.info("score_pending", "getting_mentions_json", ()).await;
        let mentions_json = serde_json::to_string(
            &api.get_user_mentions(user_id)
                .max_results(100)
                .start_time(start_time)
                .end_time(end_time)
                .expansions(build_expansions())
                .user_fields(build_user_fields())
                .tweet_fields(build_tweet_fields(&[]))
                .media_fields(build_media_fields(&[]))
                .poll_fields([PollField::VotingStatus, PollField::Options, PollField::Id])
                .send()
                .await?,
        )?;
        self.state.info("score_pending", "sleeping", ()).await;
        tokio::time::sleep(tokio::time::Duration::from_millis(
            self.state.settings.x.score_cooldown_seconds * 500,
        ))
        .await;

        self.state.info("score_pending", "getting_reposts_json", ()).await;
        let reposts_json = serde_json::to_string(
            &api.get_user_tweets(user_id)
                .max_results(100)
                .start_time(start_time)
                .end_time(end_time)
                .tweet_fields([TweetField::CreatedAt, TweetField::ReferencedTweets, TweetField::Id])
                .send()
                .await?,
        )?;
        self.state.info("score_pending", "sleeping", ()).await;
        tokio::time::sleep(tokio::time::Duration::from_millis(
            self.state.settings.x.score_cooldown_seconds * 500,
        ))
        .await;

        self.state.info("score_pending", "getting_poll_json", ()).await;
        let maybe_poll_json = if let Some(tweet_id_string) = poll_id {
            let tweet_id: u64 = tweet_id_string.parse()?;
            Some(serde_json::to_string(
                &api.get_tweet(tweet_id)
                    .expansions(build_expansions())
                    .user_fields(build_user_fields())
                    .tweet_fields(build_tweet_fields(&[
                        TweetField::OrganicMetrics,
                        TweetField::NonPublicMetrics,
                    ]))
                    .poll_fields([PollField::VotingStatus, PollField::Options, PollField::Id])
                    .send()
                    .await?,
            )?)
        } else {
            None
        };
        self.state.info("score_pending", "sleeping", ()).await;
        tokio::time::sleep(tokio::time::Duration::from_millis(
            self.state.settings.x.score_cooldown_seconds * 500,
        ))
        .await;

        Ok((me_json, tweets_json, mentions_json, reposts_json, maybe_poll_json))
    }
}

impl HandleScoring {
    pub async fn apply(self) -> AsamiResult<Self> {
        let handle = self.handle().await?;

        let Some(me) = self.me_payload()?.and_then(|x| x.into_data()) else {
            let _ = self.fail("me_payload_missing", ()).await;
            self.state.info("score_pending", "me_payload_missing", ()).await;
            return self.discard_or_apply_empty().await;
        };

        let Some(tweets) = self.tweets_payload()?.and_then(|x| x.into_data()) else {
            let _ = self.fail("tweets_missing", ()).await;
            self.state.info("score_pending", "tweets_missing", ()).await;
            return self.discard_or_apply_empty().await;
        };

        let mentions = match self.mentions_payload()? {
            None => vec![],
            Some(payload) => {
                if payload.errors().is_some() && payload.data().is_none() {
                    let _ = self.fail("errors_on_mentions", ()).await;
                    self.state.info("score_pending", "errors_on_mentions", ()).await;
                    return self.discard_or_apply_empty().await;
                }
                payload.into_data().unwrap_or(vec![])
            }
        };

        let reposts = match self.reposts_payload()? {
            None => vec![],
            Some(payload) => {
                if payload.errors().is_some() && payload.data().is_none() {
                    self.state.info("score_pending", "errors_on_reposts", ()).await;
                    return self.discard_or_apply_empty().await;
                }
                payload.into_data().unwrap_or(vec![])
            }
        };

        let Some(poll_score) = Self::get_poll_score(&self.poll_payload()?) else {
            let _ = self.fail("poll_score_missing", ()).await;
            self.state.info("score_pending", "poll_score_is_missing", ()).await;
            return self.discard_or_apply_empty().await;
        };

        let referenced = Self::get_referenced_tweets(&reposts);

        let ghost_account = Self::is_ghost_account(&tweets);

        let repost_fatigue = Self::is_repost_fatigue(&tweets, &referenced);

        let (indeterminate_audience, followed) = Self::is_indeterminate_audience_or_followed(&me);

        let liked = Self::is_liked(&tweets);

        let replied = Self::is_replied(&tweets, &referenced);

        let reposted = Self::is_reposted(&tweets, &referenced);

        let mentioned = Self::is_mentioned(&mentions);

        let online_engagement_score = if ghost_account || repost_fatigue {
            EngagementScore::None
        } else if indeterminate_audience {
            EngagementScore::Average
        } else if [followed, liked, replied, reposted, mentioned].into_iter().filter(|x| *x).count() >= 3 {
            EngagementScore::High
        } else {
            EngagementScore::Average
        };

        let operational_status_score = Self::get_operational_status_score(&me);

        let (audience_size, impression_count) = Self::get_audience_and_impression_count(&tweets);

        // These two are not implemented yet, so they're false unless overriden.
        let referrer_score = false;
        let holder_score = false;

        let authority = Self::get_authority(
            &handle,
            online_engagement_score,
            poll_score,
            operational_status_score,
            referrer_score,
            holder_score,
        );

        let score = i32_to_hex(handle.audience_size_override().unwrap_or(audience_size) * authority / 100);

        let h = handle
            .update()
            .current_scoring_id(Some(*self.id()))
            .last_scoring(Some(*self.created_at()))
            .next_scoring(Some(Utc::now() + chrono::Duration::days(20)))
            .score(Some(score.clone()))
            .status(HandleStatus::Active)
            .save()
            .await?
            .attrs;

        Ok(self
            .update()
            .status(HandleScoringStatus::Applied)
            .post_count(tweets.len().try_into()?)
            .impression_count(impression_count)
            .repost_fatigue(repost_fatigue)
            .ghost_account(ghost_account)
            .indeterminate_audience(indeterminate_audience)
            .followed(followed)
            .liked(liked)
            .replied(replied)
            .reposted(reposted)
            .mentioned(mentioned)
            .online_engagement_score(online_engagement_score)
            .online_engagement_override(h.online_engagement_override)
            .online_engagement_override_reason(h.online_engagement_override_reason)
            .offline_engagement_score(h.offline_engagement_score)
            .offline_engagement_description(h.offline_engagement_description)
            .poll_id(h.poll_id)
            .poll_score(Some(poll_score))
            .poll_override(h.poll_override)
            .poll_override_reason(h.poll_override_reason)
            .operational_status_score(operational_status_score)
            .operational_status_override(h.operational_status_override)
            .operational_status_override_reason(h.operational_status_override_reason)
            .referrer_score(referrer_score)
            .referrer_score_override(h.referrer_score_override)
            .referrer_score_override_reason(h.referrer_score_override_reason)
            .holder_score(holder_score)
            .holder_score_override(h.holder_score_override)
            .holder_score_override_reason(h.holder_score_override_reason)
            .authority(authority)
            .audience_size(audience_size)
            .audience_size_override(h.audience_size_override)
            .audience_size_override_reason(h.audience_size_override_reason)
            .score(Some(score))
            .save()
            .await?)
    }

    fn get_referenced_tweets(reposts: &[Tweet]) -> HashMap<NumericId, ReferencedStats> {
        let mut stats = HashMap::new();

        for tweet in reposts {
            let Some(references) = tweet.referenced_tweets.as_ref() else {
                continue;
            };

            for reference in references {
                let entry = stats.entry(reference.id).or_insert_with(ReferencedStats::default);
                match reference.kind {
                    ReferencedTweetKind::Quoted => entry.quoted += 1,
                    ReferencedTweetKind::RepliedTo => entry.replied += 1,
                    ReferencedTweetKind::Retweeted => entry.retweeted += 1,
                }
            }
        }

        stats
    }

    fn get_authority(
        handle: &Handle,
        online_engagement_score: EngagementScore,
        poll_score: PollScore,
        operational_status_score: OperationalStatus,
        referrer_score: bool,
        holder_score: bool,
    ) -> i32 {
        let mut authority = match handle.online_engagement_override().unwrap_or(online_engagement_score) {
            EngagementScore::None => return 0,
            EngagementScore::Average => 25,
            EngagementScore::High => 50,
        };

        match handle.poll_override().unwrap_or(poll_score) {
            PollScore::None => (),
            PollScore::Average => authority += 10,
            PollScore::High => authority += 20,
            PollScore::Reverse => authority /= 2,
        };

        match handle.offline_engagement_score() {
            EngagementScore::None => (),
            EngagementScore::Average => authority += 5,
            EngagementScore::High => authority += 10,
        };

        match handle.operational_status_override().unwrap_or(operational_status_score) {
            OperationalStatus::Banned | OperationalStatus::Shadowbanned => return 0,
            OperationalStatus::Enhanced => authority += 10,
            _ => (),
        };

        if handle.referrer_score_override().unwrap_or(referrer_score) {
            authority += 10;
        }

        if handle.holder_score_override().unwrap_or(holder_score) {
            authority += 10;
        }

        std::cmp::min(100, authority)
    }

    // Repost fatigue occurs when an account has very little original content
    // in comparison to their reposts. More original content should offset
    // repost fatigue.
    fn is_repost_fatigue(tweets: &[Tweet], referenced: &HashMap<NumericId, ReferencedStats>) -> bool {
        let repost_count: u64 = referenced.iter().map(|(_, r)| r.retweeted).sum();

        let good_post_count: u64 = tweets.iter().filter(|t| t.text.len() > 50).count().try_into().unwrap_or(0);

        if repost_count == 0 {
            return false;
        }

        // Reposts can't be more than 1 and 1/2 the good posts.
        repost_count * 100 / good_post_count >= 150
    }

    // Ghost accounts tweet too little and generate no impressions.
    fn is_ghost_account(tweets: &[Tweet]) -> bool {
        tweets.iter().filter(|t| t.text.len() > 50).count() < 5
            || tweets
                .iter()
                .filter(|t| t.organic_metrics.as_ref().map(|m| m.impression_count > 30).unwrap_or(false))
                .count()
                < 5
    }

    fn is_indeterminate_audience_or_followed(me: &TwitterUser) -> (bool, bool) {
        let Some(m) = me.public_metrics.as_ref() else {
            return (true, false);
        };

        let Some(Ok(verified_count)) = me.verified_followers_count.map(usize::try_from) else {
            return (true, false);
        };

        let is_low_verified_audience = verified_count == 0 || m.followers_count / verified_count > 70;

        let is_followed = (m.following_count == 0 || verified_count / m.following_count >= 1) && verified_count > 200;

        (is_low_verified_audience, is_followed)
    }

    /*
    verified_count: 354,
    follower_count: 9548,
    following_count: 100,
    */

    fn is_liked(tweets: &[Tweet]) -> bool {
        let mut very_liked = 0;
        let mut liked_enough = 0;

        for t in tweets {
            let Some(m) = t.organic_metrics.as_ref() else { continue };
            if m.like_count == 0 || m.impression_count < NOT_ENOUGH_IMPRESSIONS {
                continue;
            }

            if m.like_count > 10 {
                liked_enough += 1;
            }

            if std::cmp::min(m.impression_count, 30000) / m.like_count < 150 {
                very_liked += 1;
            }
        }

        liked_enough > 3 && very_liked * 100 / tweets.len() > 70
    }

    fn is_replied(tweets: &[Tweet], referenced: &HashMap<NumericId, ReferencedStats>) -> bool {
        let mut impression_count: i32 = 0;
        let mut reply_count: i32 = 0;

        for tweet in tweets {
            let Some(o) = tweet.organic_metrics.as_ref() else {
                continue;
            };
            let Some(p) = tweet.public_metrics.as_ref() else {
                continue;
            };

            if o.impression_count < NOT_ENOUGH_IMPRESSIONS {
                continue;
            }

            impression_count += o.impression_count.try_into().unwrap_or(0);

            reply_count += (p.reply_count + p.quote_count.unwrap_or(0)).try_into().unwrap_or(0);

            if let Some(r) = referenced.get(&tweet.id) {
                reply_count -= (r.replied + r.quoted).try_into().unwrap_or(0);
            }
        }

        if reply_count <= 0 {
            return false;
        }

        std::cmp::min(impression_count, 30000) / reply_count < 600
    }

    // Means a number (30%) of tweets have at least 1 retweet for every 400 impressions.
    fn is_reposted(tweets: &[Tweet], referenced: &HashMap<NumericId, ReferencedStats>) -> bool {
        let mut reposted = 0;

        for t in tweets {
            let Some(o) = t.organic_metrics.as_ref() else { continue };
            let Some(p) = t.public_metrics.as_ref() else { continue };

            let impression_count: i32 = std::cmp::min(o.impression_count, 30000).try_into().unwrap_or(0);

            let mut retweet_count = (p.retweet_count + p.quote_count.unwrap_or(0)).try_into().unwrap_or(0);

            if let Some(r) = referenced.get(&t.id) {
                retweet_count -= (r.retweeted + r.quoted).try_into().unwrap_or(0);
            }

            if retweet_count <= 0 {
                continue;
            }

            if impression_count / retweet_count < 400 {
                reposted += 1;
            }
        }

        reposted * 100 / tweets.len() > 30
    }

    fn is_mentioned(tweets: &[Tweet]) -> bool {
        tweets
            .iter()
            .filter(|t| t.public_metrics.as_ref().map(|m| m.retweet_count + m.like_count > 25).unwrap_or(false))
            .count()
            >= 2
    }

    fn get_poll_score(maybe_payload: &Option<ApiPayload<Tweet, ()>>) -> Option<PollScore> {
        let Some(payload) = maybe_payload else {
            return Some(PollScore::None);
        };

        // The user can delete the poll, we're fine with it, so not-found should not discard the scoring.
        if payload.errors().map(|v| v.iter().any(|e| e.title == "Not Found Error")).unwrap_or(false) {
            return Some(PollScore::None);
        }

        let mut options = payload.includes().as_ref()?.polls.as_ref()?.first().as_ref()?.options.clone();
        options.sort_by_key(|i| i.position);
        let [none, average, high, reverse] = options.as_slice() else {
            return None;
        };

        let score = if high.votes + average.votes > none.votes + reverse.votes {
            if high.votes > average.votes {
                PollScore::High
            } else {
                PollScore::Average
            }
        } else if none.votes > reverse.votes {
            PollScore::None
        } else {
            PollScore::Reverse
        };

        Some(score)
    }

    fn get_operational_status_score(me: &TwitterUser) -> OperationalStatus {
        if me.verified.unwrap_or(false) {
            OperationalStatus::Enhanced
        } else if me.protected.unwrap_or(false) {
            OperationalStatus::Shadowbanned
        } else {
            OperationalStatus::Normal
        }
    }

    fn get_audience_and_impression_count(tweets: &[Tweet]) -> (i32, i32) {
        let mut impressions: Vec<usize> =
            tweets.iter().map(|t| t.organic_metrics.as_ref().map(|m| m.impression_count).unwrap_or(0)).collect();

        let total: usize = impressions.iter().sum();

        impressions.sort();
        let median = *impressions.get(impressions.len() / 2).unwrap_or(&0);
        let mean = total / tweets.len();
        let audience = i32::try_from((mean + median) / 2).unwrap_or(0);

        (audience, i32::try_from(total).unwrap_or(0))
    }

    pub async fn discard_or_apply_empty(self) -> AsamiResult<Self> {
        /* We retry scoring 3 times, if there are 3 consecutive discarded scorings, we apply it with a 0 value */
        let handle = self.handle().await?;

        let some_recently_applied = handle
            .handle_scoring_scope()
            .order_by(HandleScoringOrderBy::CreatedAt)
            .desc(true)
            .limit(4)
            .all()
            .await?
            .iter()
            .filter(|s| *s.status() == HandleScoringStatus::Applied)
            .count()
            > 0;

        let never_applied_any =
            handle.handle_scoring_scope().status_eq(HandleScoringStatus::Applied).count().await? == 0;

        if some_recently_applied || never_applied_any {
            Ok(self.update().status(HandleScoringStatus::Discarded).save().await?)
        } else {
            self.handle()
                .await?
                .update()
                .current_scoring_id(Some(*self.id()))
                .last_scoring(Some(*self.created_at()))
                .next_scoring(Some(Utc::now() + chrono::Duration::days(7)))
                .score(Some(weihex("0")))
                .status(HandleStatus::Active)
                .save()
                .await?;
            let _ = self.info("applied_empty_score", ()).await;
            Ok(self.update().status(HandleScoringStatus::Applied).save().await?)
        }
    }

    pub fn me_payload(&self) -> AsamiResult<Option<ApiPayload<TwitterUser, ()>>> {
        self.parse_payload(self.me_json(), "me_json")
    }

    pub fn tweets_payload(&self) -> AsamiResult<Option<ApiPayload<Vec<Tweet>, TweetsMeta>>> {
        self.parse_payload(self.tweets_json(), "tweets_payload")
    }

    pub fn mentions_payload(&self) -> AsamiResult<Option<ApiPayload<Vec<Tweet>, TweetsMeta>>> {
        self.parse_payload(self.mentions_json(), "mentions_payload")
    }

    pub fn reposts_payload(&self) -> AsamiResult<Option<ApiPayload<Vec<Tweet>, TweetsMeta>>> {
        self.parse_payload(self.reposts_json(), "reposts_payload")
    }

    pub fn poll_payload(&self) -> AsamiResult<Option<ApiPayload<Tweet, ()>>> {
        self.parse_payload(self.poll_json(), "poll_payload")
    }

    pub fn parse_payload<T, A>(&self, json: &Option<String>, col: &str) -> AsamiResult<Option<ApiPayload<T, A>>>
    where
        T: serde::de::DeserializeOwned,
        A: serde::de::DeserializeOwned,
    {
        json.as_deref()
            .map(serde_json::from_str)
            .transpose()
            .map_err(|_| Error::runtime(&format!("Parsing column {} for scoring {}", col, self.id())))
    }
}

make_sql_enum![
    "engagement_score",
    pub enum EngagementScore {
        None,
        Average,
        High,
    }
];

make_sql_enum![
    "poll_score",
    pub enum PollScore {
        None,
        Average,
        High,
        Reverse,
    }
];

make_sql_enum![
    "operational_status",
    pub enum OperationalStatus {
        Banned,
        Shadowbanned,
        Normal,
        Enhanced,
    }
];

make_sql_enum![
    "handle_scoring_status",
    pub enum HandleScoringStatus {
        Pending,
        Ingested,
        Applied,
        Discarded,
    }
];

fn build_user_fields() -> Vec<UserField> {
    vec![
        UserField::Protected,
        UserField::ConnectionStatus,
        UserField::Name,
        UserField::PinnedTweetId,
        UserField::Affiliation,
        UserField::Parody,
        UserField::VerifiedFollowersCount,
        UserField::VerifiedType,
        UserField::Subscription,
        UserField::SubscriptionType,
        UserField::CreatedAt,
        UserField::Id,
        UserField::Username,
        UserField::Verified,
        UserField::Withheld,
        UserField::PublicMetrics,
        UserField::Description,
    ]
}

fn build_tweet_fields(extra: &[TweetField]) -> Vec<TweetField> {
    let mut base = vec![
        TweetField::Attachments,
        TweetField::NoteTweet,
        TweetField::AuthorId,
        TweetField::ReplySettings,
        TweetField::Lang,
        TweetField::CreatedAt,
        TweetField::Text,
        TweetField::Id,
        TweetField::Entities,
        TweetField::Geo,
        TweetField::Withheld,
        TweetField::PublicMetrics,
        TweetField::Source,
        TweetField::PossiblySensitive,
        TweetField::ContextAnnotations,
        TweetField::ConversationId,
        TweetField::InReplyToUserId,
        TweetField::ReferencedTweets,
    ];
    base.extend_from_slice(extra);
    base
}

fn build_expansions() -> Vec<TweetExpansion> {
    vec![
        TweetExpansion::AttachmentsPollIds,
        TweetExpansion::AuthorId,
        TweetExpansion::InReplyToUserId,
        TweetExpansion::ReferencedTweetsIdAuthorId,
    ]
}

fn build_media_fields(extra: &[MediaField]) -> Vec<MediaField> {
    let mut base = vec![
        MediaField::DurationMs,
        MediaField::MediaKey,
        MediaField::Type,
        MediaField::PublicMetrics,
        MediaField::AltText,
    ];
    base.extend_from_slice(extra);
    base
}

#[derive(Debug, Default)]
struct ReferencedStats {
    replied: u64,
    retweeted: u64,
    quoted: u64,
}
