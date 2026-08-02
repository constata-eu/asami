CREATE TYPE acceptance_status AS ENUM (
    'guestlist',
    'active',
    'inactive'
);

ALTER TYPE campaign_kind ADD VALUE IF NOT EXISTS 'community_giveaway';

ALTER TABLE accounts
    ADD COLUMN acceptance_status acceptance_status NOT NULL DEFAULT 'active',
    ADD COLUMN invited_by_account_id VARCHAR REFERENCES accounts(id),
    ADD COLUMN invites_per_month INT4 NOT NULL DEFAULT 0,
    ADD COLUMN available_invites INT4 NOT NULL DEFAULT 0,
    ADD COLUMN invite_quota_last_replenished_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    ADD COLUMN profile_pic TEXT,
    DROP COLUMN stripe_customer_id;

UPDATE accounts
SET acceptance_status = CASE
    WHEN status = 'banned' THEN 'inactive'::acceptance_status
    ELSE 'active'::acceptance_status
END;

CREATE INDEX idx_accounts_acceptance_status ON accounts(acceptance_status);
CREATE INDEX idx_accounts_invited_by_account_id ON accounts(invited_by_account_id);

CREATE TABLE invitations (
    id SERIAL PRIMARY KEY NOT NULL,
    inviter_account_id VARCHAR REFERENCES accounts(id) NOT NULL,
    account_id VARCHAR REFERENCES accounts(id) NOT NULL,
    token VARCHAR NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    expires_at TIMESTAMPTZ,
    accepted_at TIMESTAMPTZ,
    revoked_at TIMESTAMPTZ
);

CREATE INDEX idx_invitations_inviter_account_id ON invitations(inviter_account_id);
CREATE INDEX idx_invitations_account_id ON invitations(account_id);
CREATE INDEX idx_invitations_token ON invitations(token);

CREATE TYPE waitlist_application_status AS ENUM (
    'pending',
    'reviewed',
    'accepted',
    'rejected'
);

CREATE TABLE waitlist_applications (
    id SERIAL PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    email TEXT NOT NULL,
    link_1 TEXT NOT NULL,
    link_2 TEXT NOT NULL,
    link_3 TEXT,
    note TEXT NOT NULL,
    invitation_id INT4 REFERENCES invitations(id),
    status waitlist_application_status NOT NULL DEFAULT 'pending',
    review_notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    reviewed_at TIMESTAMPTZ
);

CREATE INDEX idx_waitlist_applications_status ON waitlist_applications(status);
CREATE INDEX idx_waitlist_applications_created_at ON waitlist_applications(created_at DESC);

ALTER TABLE community_members
    ALTER COLUMN rewards SET DEFAULT to_u256(0),
    ALTER COLUMN collabs SET DEFAULT 0,
    ALTER COLUMN first_collab_date DROP NOT NULL,
    ALTER COLUMN last_collab_date DROP NOT NULL,
    ADD COLUMN priority BOOLEAN NOT NULL DEFAULT FALSE,
    ADD COLUMN alignment INT4,
    ADD COLUMN trust INT4,
    ADD COLUMN contribution INT4,
    ADD COLUMN advocacy INT4,
    ADD COLUMN judgment INT4,
    ADD COLUMN rootstock_joined_at TIMESTAMPTZ,
    ADD COLUMN comment TEXT,
    ADD COLUMN created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    ADD COLUMN updated_at TIMESTAMPTZ NOT NULL DEFAULT now();

UPDATE community_members
SET
    created_at = COALESCE(first_collab_date, now()),
    updated_at = now();

ALTER TABLE campaigns
    ADD COLUMN kind campaign_kind NOT NULL DEFAULT 'x_repost',
    ADD COLUMN collabs_gathered_at TIMESTAMPTZ,
    ADD COLUMN rewards_assigned_at TIMESTAMPTZ,
    DROP COLUMN price_per_point,
    DROP COLUMN min_individual_reward,
    DROP COLUMN managed_by_admin,
    DROP COLUMN managed_unit_amount,
    DROP COLUMN stripe_session_url,
    DROP COLUMN stripe_session_id;

CREATE INDEX idx_campaigns_kind ON campaigns(kind);
CREATE INDEX idx_campaigns_collabs_gathered_at ON campaigns(collabs_gathered_at);
CREATE INDEX idx_campaigns_rewards_assigned_at ON campaigns(rewards_assigned_at);

ALTER TABLE collabs
    ALTER COLUMN handle_id DROP NOT NULL;

ALTER TYPE collab_status ADD VALUE IF NOT EXISTS 'pending';

ALTER TABLE handles
    DROP COLUMN next_scoring,
    DROP COLUMN score,
    DROP COLUMN last_scoring,
    DROP COLUMN current_scoring_id,
    DROP COLUMN online_engagement_override,
    DROP COLUMN online_engagement_override_reason,
    DROP COLUMN offline_engagement_score,
    DROP COLUMN offline_engagement_description,
    DROP COLUMN poll_id,
    DROP COLUMN poll_ends_at,
    DROP COLUMN poll_override,
    DROP COLUMN poll_override_reason,
    DROP COLUMN operational_status_override,
    DROP COLUMN operational_status_override_reason,
    DROP COLUMN referrer_score_override,
    DROP COLUMN referrer_score_override_reason,
    DROP COLUMN holder_score_override,
    DROP COLUMN holder_score_override_reason,
    DROP COLUMN audience_size_override,
    DROP COLUMN audience_size_override_reason,
    ADD COLUMN profile_image_url TEXT NOT NULL DEFAULT '';

DROP TABLE IF EXISTS set_score_and_topics_request_topics;
DROP TABLE IF EXISTS set_score_and_topics_requests;
DROP TABLE IF EXISTS handle_scorings;

DROP TYPE IF EXISTS handle_scoring_status;
DROP TYPE IF EXISTS engagement_score;
DROP TYPE IF EXISTS poll_score;
DROP TYPE IF EXISTS operational_status;

CREATE TABLE score_card_skips (
    id SERIAL PRIMARY KEY NOT NULL,
    scorer_account_id VARCHAR REFERENCES accounts(id) NOT NULL,
    target_account_id VARCHAR REFERENCES accounts(id) NOT NULL,
    reason TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_score_card_skips_scorer_account_id ON score_card_skips(scorer_account_id);
CREATE INDEX idx_score_card_skips_target_account_id ON score_card_skips(target_account_id);
CREATE INDEX idx_score_card_skips_pair_created_at ON score_card_skips(scorer_account_id, target_account_id, created_at DESC);

CREATE TABLE weekly_score_picks (
    id SERIAL PRIMARY KEY NOT NULL,
    target_account_id VARCHAR REFERENCES accounts(id) NOT NULL,
    created_by_account_id VARCHAR REFERENCES accounts(id),
    starts_at TIMESTAMPTZ NOT NULL,
    ends_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_weekly_score_picks_target_account_id ON weekly_score_picks(target_account_id);
CREATE INDEX idx_weekly_score_picks_window ON weekly_score_picks(starts_at, ends_at);
