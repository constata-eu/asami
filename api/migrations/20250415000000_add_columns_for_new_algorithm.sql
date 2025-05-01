CREATE TYPE engagement_score AS ENUM (
    'none',
    'average',
    'high'
);
CREATE TYPE poll_score AS ENUM (
    'none',
    'average',
    'high',
    'reverse'
);
CREATE TYPE operational_status AS ENUM (
    'banned',
    'shadowbanned',
    'normal',
    'enhanced'
);
CREATE TYPE handle_scoring_status AS ENUM (
    'pending',
    'ingested',
    'applied',
    'discarded'
);

CREATE TABLE handle_scorings (
    id SERIAL PRIMARY KEY NOT NULL,
    handle_id INTEGER REFERENCES handles(id) NOT NULL,
    status handle_scoring_status NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    me_json TEXT,
    tweets_json TEXT,
    mentions_json TEXT,
    reposts_json TEXT,
    poll_json TEXT,

    post_count INT4 NOT NULL DEFAULT 0,
    impression_count INT4 NOT NULL DEFAULT 0,

    repost_fatigue BOOLEAN NOT NULL DEFAULT FALSE,
    ghost_account BOOLEAN NOT NULL DEFAULT FALSE,
    indeterminate_audience BOOLEAN NOT NULL DEFAULT FALSE,
    followed BOOLEAN NOT NULL DEFAULT FALSE,
    liked BOOLEAN NOT NULL DEFAULT FALSE,
    replied BOOLEAN NOT NULL DEFAULT FALSE,
    reposted BOOLEAN NOT NULL DEFAULT FALSE,
    mentioned BOOLEAN NOT NULL DEFAULT FALSE,

    online_engagement_score engagement_score NOT NULL DEFAULT 'none',
    online_engagement_override engagement_score,
    online_engagement_override_reason TEXT,

    offline_engagement_score engagement_score NOT NULL DEFAULT 'none',
    offline_engagement_description TEXT,

    poll_id VARCHAR,
    poll_score poll_score,
    poll_override poll_score,
    poll_override_reason TEXT,

    operational_status_score operational_status NOT NULL DEFAULT 'normal',
    operational_status_override operational_status,
    operational_status_override_reason TEXT,

    referrer_score boolean NOT NULL DEFAULT false,
    referrer_score_override boolean,
    referrer_score_override_reason TEXT,

    holder_score boolean NOT NULL DEFAULT false,
    holder_score_override boolean,
    holder_score_override_reason TEXT,

    authority INT4 NOT NULL DEFAULT 0,

    audience_size INT4 NOT NULL DEFAULT 0,
    audience_size_override INT4,
    audience_size_override_reason TEXT,

    score VARCHAR
);

CREATE INDEX idx_handle_scorings_handle_id ON handle_scorings(handle_id);
CREATE INDEX idx_handle_scorings_status ON handle_scorings(status);

DELETE FROM handles WHERE user_id IS NULL;

ALTER TABLE handles

ALTER COLUMN user_id SET NOT NULL,

ALTER COLUMN last_scoring SET DEFAULT NULL,

DROP COLUMN avg_impression_count,
DROP COLUMN avg_reply_count,
DROP COLUMN avg_repost_count,
DROP COLUMN avg_like_count,
DROP COLUMN scored_tweet_count,

-- The refresh token is needed to obtain offline data.
-- If it's not there, we need to ask users for it.
ADD COLUMN x_refresh_token TEXT,

ADD COLUMN current_scoring_id INT4,

-- The online engagement is always calculated, but we may override it 
-- manually if we discover algorithm errors or if there are data errors.
-- We may also see artificially high numbers.
ADD COLUMN online_engagement_override engagement_score,
ADD COLUMN online_engagement_override_reason TEXT,

-- This is always set manually.
-- A human readable description should be added explaining the reasoning.
ADD COLUMN offline_engagement_score engagement_score NOT NULL DEFAULT 'none',
ADD COLUMN offline_engagement_description TEXT,

-- The poll they've run to check their authority.
ADD COLUMN poll_id VARCHAR,
ADD COLUMN poll_ends_at TIMESTAMPTZ,
ADD COLUMN poll_override poll_score,
ADD COLUMN poll_override_reason TEXT,

ADD COLUMN operational_status_override operational_status,
ADD COLUMN operational_status_override_reason TEXT,

-- If the account referred a number of accounts with real engagement they will a bonus.
ADD COLUMN referrer_score_override boolean,
ADD COLUMN referrer_score_override_reason TEXT,

-- Accounts that hold on to their asami tokens get this score.
ADD COLUMN holder_score_override boolean,
ADD COLUMN holder_score_override_reason TEXT,

-- Audience size is the average impression count.
-- Unless we find some wrong-doing and need to override it manually.
ADD COLUMN audience_size INT4 NOT NULL DEFAULT 0,
ADD COLUMN audience_size_override INT4,
ADD COLUMN audience_size_override_reason TEXT,
ADD COLUMN audience_size_override_date TIMESTAMPTZ;

CREATE INDEX idx_handles_current_scoring_id ON handles(current_scoring_id);
