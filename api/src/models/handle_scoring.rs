use twitter_v2::{
    authorization::Oauth2Token,
    meta::TweetsMeta,
    query::{MediaField, PollField, TweetField, UserField},
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
    #[sqlx_model_hints(handle_scoring_status, default)]
    status: HandleScoringStatus,
    #[sqlx_model_hints(timestamptz, default)]
    created_at: DateTime<Utc>,
    #[sqlx_model_hints(text, default)]
    me_json: Option<String>,
    #[sqlx_model_hints(text, default)]
    tweets_json: Option<String>,
    #[sqlx_model_hints(text, default)]
    mentions_json: Option<String>,

    #[sqlx_model_hints(int4, default)]
    post_count: i32, // V
    #[sqlx_model_hints(int4, default)]
    impression_count: i32, // V

    #[sqlx_model_hints(boolean, default)]
    ghost_account: bool,
    #[sqlx_model_hints(boolean, default)]
    inflated_audience: bool,
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
    #[sqlx_model_hints(timestamptz, default)]
    poll_ends_at: Option<DateTime<Utc>>,
    #[sqlx_model_hints(timestamptz, default)]
    poll_votes_updated_at: Option<DateTime<Utc>>,
    #[sqlx_model_hints(int4, default)]
    poll_none_votes: i32,
    #[sqlx_model_hints(int4, default)]
    poll_average_votes: i32,
    #[sqlx_model_hints(int4, default)]
    poll_high_votes: i32,
    #[sqlx_model_hints(int4, default)]
    poll_reverse_votes: i32,
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
    referrer_score_override: bool,
    #[sqlx_model_hints(text, default)]
    referrer_score_override_reason: Option<String>,

    #[sqlx_model_hints(boolean, default)]
    holder_score: bool,
    #[sqlx_model_hints(boolean, default)]
    holder_score_override: bool,
    #[sqlx_model_hints(text, default)]
    holder_score_override_reason: Option<String>,

    #[sqlx_model_hints(int4, default)]
    audience_size: i32,
    #[sqlx_model_hints(int4, default)]
    audience_size_override: Option<i32>,
    #[sqlx_model_hints(text, default)]
    audience_size_override_reason: Option<String>,
  },
  belongs_to {
      Handle(handle_id)
  }
}

impl_loggable!(HandleScoring);

impl HandleScoringHub {
    pub async fn create_and_apply(&self, item: Handle) -> AsamiResult<HandleScoring> {
        self.create(item).await?.apply().await
    }

    pub async fn create(&self, item: Handle) -> AsamiResult<HandleScoring> {
        let scoring = self
            .insert(InsertHandleScoring {
                handle_id: item.attrs.id,
            })
            .save()
            .await?;

        let mut update = scoring.clone().update().status(HandleScoringStatus::Ingested);

        match item.x_api_client().await {
            Ok(api) => {
                match self.ingest(api).await {
                    Ok((me, tweets, mentions)) => {
                        update = update 
                            .me_json(Some(me))
                            .tweets_json(Some(tweets))
                            .mentions_json(Some(mentions));
                    }
                    Err(e) => {
                        let _ = scoring.fail("ingestion_error", e).await;
                    }
                };
            }
            Err(e) => {
                let _ = scoring.fail("twitter_client_issues", format!("{e:?}")).await;
            }
        };

        Ok(update.save().await?)
    }
    pub async fn ingest(&self, api: TwitterApi<Oauth2Token>) -> Result<(String, String, String), IngestError> {
        let me = api
            .get_users_me()
            .user_fields([
                UserField::Id,
                UserField::Username,
                UserField::Description,
                UserField::PublicMetrics,
                UserField::Withheld,
                UserField::CreatedAt,
                UserField::Protected,
                UserField::Affiliation,
                UserField::Parody,
                UserField::VerifiedFollowersCount,
                UserField::Verified,
                UserField::VerifiedType,
                UserField::Subscription,
                UserField::SubscriptionType,
            ])
            .send()
            .await
            .map_err(|e| IngestError::new(e, "getting me"))?;

        let me_json = serde_json::to_string(&me).map_err(|e| IngestError::new(e, &me))?;

        let Some(user_id) = me.payload().data().map(|u| u.id) else {
            return Err(IngestError::new("user_data_was_empty", &me));
        };

        let now = Utc::now();

        // The window to score is one month starting a week ago.
        // Se we don't count old tweets, but we also give recent
        // tweets a week to gain impressions.
        let start_time = time::OffsetDateTime::from_unix_timestamp((now - chrono::Duration::days(30)).timestamp())?
            .to_offset(time::UtcOffset::UTC);

        let end_time = time::OffsetDateTime::from_unix_timestamp((now - chrono::Duration::days(0)).timestamp())?
            .to_offset(time::UtcOffset::UTC);

        let tweets = api
            .get_user_tweets(user_id)
            .max_results(100)
            .start_time(start_time)
            .end_time(end_time)
            .tweet_fields([
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
                TweetField::OrganicMetrics,
                TweetField::NonPublicMetrics,
                TweetField::Source,
                TweetField::PossiblySensitive,
                TweetField::ContextAnnotations,
                TweetField::ConversationId,
                TweetField::InReplyToUserId,
                TweetField::ReferencedTweets,
            ])
            .user_fields([
                UserField::Affiliation,
                UserField::ConnectionStatus,
                UserField::Parody,
                UserField::VerifiedFollowersCount,
                UserField::VerifiedType,
                UserField::Subscription,
                UserField::SubscriptionType,
                UserField::CreatedAt,
                UserField::Name,
                UserField::Id,
                UserField::Username,
                UserField::Verified,
                UserField::Withheld,
                UserField::PublicMetrics,
                UserField::Description,
                UserField::PinnedTweetId,
            ])
            .media_fields([
                MediaField::DurationMs,
                MediaField::MediaKey,
                MediaField::Type,
                MediaField::PublicMetrics,
                MediaField::NonPublicMetrics,
                MediaField::OrganicMetrics,
                MediaField::PromotedMetrics,
                MediaField::AltText,
            ])
            .poll_fields([PollField::VotingStatus, PollField::Options, PollField::Id])
            .send()
            .await
            .map_err(|e| IngestError::new(e, &me))?;

        let tweets_json = serde_json::to_string(&tweets).map_err(|e| IngestError::new(e, &me))?;

        let mentions = api
            .get_user_mentions(user_id)
            .max_results(100) // TODO: This can be a configuration if we get throtthled.
            .user_fields([
                UserField::Affiliation,
                UserField::ConnectionStatus,
                UserField::Parody,
                UserField::VerifiedFollowersCount,
                UserField::VerifiedType,
                UserField::Subscription,
                UserField::SubscriptionType,
                UserField::CreatedAt,
                UserField::Name,
                UserField::Id,
                UserField::Username,
                UserField::Verified,
                UserField::Withheld,
                UserField::PublicMetrics,
                UserField::Description,
                UserField::PinnedTweetId,
            ])
            .tweet_fields([
                TweetField::Attachments,
                TweetField::NoteTweet,
                TweetField::AuthorId,
                TweetField::ReplySettings,
                TweetField::PublicMetrics,
                TweetField::Lang,
                TweetField::CreatedAt,
                TweetField::Id,
                TweetField::Source,
                TweetField::Text,
                TweetField::PossiblySensitive,
                TweetField::ContextAnnotations,
                TweetField::ConversationId,
                TweetField::Entities,
                TweetField::Geo,
                TweetField::InReplyToUserId,
                TweetField::ReferencedTweets,
                TweetField::Withheld,
            ])
            .media_fields([
                MediaField::DurationMs,
                MediaField::MediaKey,
                MediaField::Type,
                MediaField::PublicMetrics,
                MediaField::AltText,
            ])
            .poll_fields([PollField::VotingStatus, PollField::Options, PollField::Id])
            .send()
            .await
            .map_err(|e| IngestError::new(e, (&me, &tweets)))?;

        let mentions_json = serde_json::to_string(&mentions).map_err(|e| IngestError::new(e, (&me, &tweets)))?;

        Ok((me_json, tweets_json, mentions_json))
    }
}

impl HandleScoring {
    pub async fn apply(self) -> AsamiResult<Self> {
        let handle = self.handle().await?;

        let Some(me) = self.me_payload()?.and_then(|x| x.into_data() ) else {
            return self.discard_or_apply_empty().await
        };

        let Some(tweets) = self.tweets_payload()?.and_then(|x| x.into_data() ) else {
            return self.discard_or_apply_empty().await
        };

        let Some(mentions) = self.mentions_payload()?.and_then(|x| x.into_data() ) else {
            return self.discard_or_apply_empty().await
        };


        /*

         Online engagement criteria in order of precedence

        
        # Ghost account:
            Has 5 posts or less, none over 30 impressions, regardless of likes and comments.
            Online Engagement is set to None.

        # Inflated audience
            When: follower_count / verified_followers > 70
            If not 1 in 70 followers is verified, the account has a low quality audience.
            Red flag.
            Engagement is set to Average.

        # Followed:
            When verified_follower_count / following_count > 8 and verified_follower_count > 200
            Account with over 200 verified followers where 8 out of 10 didn't need a follow back
            means people want to hear what hey say.
            Contributes 1/3 towards High engagement.

        # Liked:
            70% of their posts have at least 1 like for every 150 impressions, 
            They have at least 3 posts with more than 10 likes. 
            for the first 30k impressions. (If impressions are over 30k we use 30k)
            Contributes 1/3 towards High engagement.

        # Replied:
            Their posts have on average 1 comment for every 600 impressions.
            Each post impression count is capped at 30k for this metric.
            It's a global average.
            Contributes 1/3 towards High engagement.

        # Reposted:
            70% of their posts have at least 1 repost for every 400 impressions, 
            for the first 30k impressions. (If impressions are over 30k we use 30k)
            Contributes 1/3 towards High engagement.

        # Mentioned:
            At least two tweets mentioning them had 500 impressions.
            Contributes 1/3 towards High engagement.

        */

        let mut update = self.update()
            .status(HandleScoringStatus::Applied)
            .post_count(tweets.len().try_into()?);
        
        Ok(update.save().await?)
    }

    pub async fn discard_or_apply_empty(self) -> AsamiResult<Self> {
        /* We retry scoring 3 times, if there are 3 consecutive discarded scorings, we apply it with a 0 value */
        let handle = self.handle().await?;

        let some_recently_applied = handle.handle_scoring_scope().order_by(HandleScoringOrderBy::CreatedAt).desc(true).limit(4).all().await?
            .iter().filter(|s| *s.status() == HandleScoringStatus::Applied)
            .count() > 0;

        if some_recently_applied {
            Ok(self.update().status(HandleScoringStatus::Discarded).save().await?)
        } else {
            self.handle().await?
                .update()
                .current_scoring_id(Some(*self.id()))
                .last_scoring(Some(*self.created_at()))
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

#[derive(Serialize)]
pub struct IngestError {
    error: String,
    context: String,
}

impl IngestError {
    fn new<E: std::fmt::Debug, S: Serialize>(e: E, c: S) -> Self {
        let context = serde_json::to_string(&c).unwrap_or_else(|_| "unserializable_context".to_string());
        Self {
            error: format!("{e:?}"),
            context,
        }
    }
}

impl From<time::error::ComponentRange> for IngestError {
    fn from(e: time::error::ComponentRange) -> IngestError {
        IngestError::new(e, "")
    }
}
