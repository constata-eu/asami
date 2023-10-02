CREATE TYPE site AS ENUM (
  'x',
  'instagram',
  'nostr'
);

CREATE TYPE collab_status AS ENUM (
  'unseen',
  'seen',
  'reverted'
);

CREATE TYPE auth_method_kind AS ENUM (
  'x',
  'google',
  'facebook',
  'nostr',
  'bitcoin_signed_message',
  'ethereum_signed_message',
  'one_time_token'
);

CREATE TABLE accounts (
    id SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR NOT NULL,
    created_at timestamp DEFAULT now() NOT NULL,
    updated_at timestamp
);

CREATE TABLE users (
    id SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR NOT NULL,
    created_at timestamp DEFAULT now() NOT NULL,
    updated_at timestamp
);


CREATE TABLE deletions (
    id SERIAL PRIMARY KEY NOT NULL,
    account_id INTEGER REFERENCES accounts(id),
    user_id INTEGER REFERENCES users(id),
    created_at timestamp DEFAULT now() NOT NULL,
    updated_at timestamp
);
CREATE INDEX idx_deletions_users_account_id ON deletions(account_id);
CREATE INDEX idx_deletions_users_user_id ON deletions(user_id);


CREATE TABLE campaigns (
    id SERIAL PRIMARY KEY NOT NULL,
    account_id INTEGER REFERENCES accounts(id) NOT NULL,
    budget DECIMAL(10,2) NOT NULL,
    site site NOT NULL,
    content VARCHAR NOT NULL,
    created_at timestamp DEFAULT now() NOT NULL,
    updated_at timestamp,
    deletion_id INTEGER REFERENCES deletions(id)
);
CREATE INDEX idx_campaigns_account_id ON campaigns(account_id);
CREATE INDEX idx_campaigns_site ON campaigns(site);

CREATE TABLE topics (
    id SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR NOT NULL,
    created_at timestamp DEFAULT now() NOT NULL,
    updated_at timestamp
);

CREATE TABLE handles (
    id SERIAL PRIMARY KEY NOT NULL,
    account_id INTEGER REFERENCES accounts(id) NOT NULL,
    site site NOT NULL,
    value VARCHAR NOT NULL,
    price DECIMAL(10,2) NOT NULL,
    created_at timestamp DEFAULT now() NOT NULL,
    updated_at timestamp
);
CREATE INDEX idx_collabs_account_id ON handles(account_id);

CREATE TABLE handle_topics (
    id SERIAL PRIMARY KEY NOT NULL,
    handle_id INTEGER REFERENCES handles(id) NOT NULL,
    topic_id INTEGER REFERENCES topics(id) NOT NULL,
    created_at timestamp DEFAULT now() NOT NULL,
    updated_at timestamp
);
CREATE INDEX idx_handle_topics_handle_id ON handle_topics(handle_id);
CREATE INDEX idx_handle_topics_topic_id ON handle_topics(topic_id);

CREATE TABLE collabs (
    id SERIAL PRIMARY KEY NOT NULL,
    campaign_id INTEGER REFERENCES campaigns(id) NOT NULL,
    handle_id INTEGER REFERENCES handles(id) NOT NULL,
    status collab_status,
    created_at timestamp DEFAULT now() NOT NULL,
    updated_at timestamp
);
CREATE INDEX idx_collabs_campaign_id ON collabs(campaign_id);
CREATE INDEX idx_collabs_handle_id ON collabs(handle_id);
CREATE INDEX idx_collabs_status ON collabs(status);

CREATE TABLE campaign_topics (
    id SERIAL PRIMARY KEY NOT NULL,
    campaign_id INTEGER REFERENCES campaigns(id) NOT NULL,
    topic_id INTEGER REFERENCES topics(id) NOT NULL,
    created_at timestamp DEFAULT now() NOT NULL,
    updated_at timestamp
);
CREATE INDEX idx_campaign_topics_campaign_id ON campaign_topics(campaign_id);
CREATE INDEX idx_campaign_topics_topic_id ON campaign_topics(topic_id);

CREATE TABLE auth_methods (
    id SERIAL PRIMARY KEY NOT NULL,
    user_id INTEGER REFERENCES users(id) NOT NULL,
    kind auth_method_kind NOT NULL,
    lookup_key VARCHAR NOT NULL UNIQUE,
    created_at timestamp DEFAULT now() NOT NULL,
    updated_at timestamp
);
CREATE INDEX idx_auth_methods_user_id ON auth_methods(user_id);
CREATE INDEX idx_auth_methods_kind ON auth_methods(kind);
CREATE INDEX idx_auth_methods_lookup_key ON auth_methods(lookup_key);

CREATE TABLE sessions (
    id VARCHAR PRIMARY KEY NOT NULL,
    user_id INTEGER REFERENCES users(id) NOT NULL,
    auth_method_id INTEGER REFERENCES auth_methods(id) NOT NULL,
    pubkey TEXT NOT NULL,
    nonce BIGINT NOT NULL DEFAULT 0,
    created_at timestamp DEFAULT now() NOT NULL,
    updated_at timestamp,
    deletion_id INTEGER REFERENCES deletions(id)
);
CREATE INDEX idx_sessions_user_id ON sessions(user_id);

CREATE TABLE account_users (
    id SERIAL PRIMARY KEY NOT NULL,
    account_id INTEGER REFERENCES accounts(id),
    user_id INTEGER REFERENCES users(id),
    created_at timestamp DEFAULT now() NOT NULL,
    updated_at timestamp
);
CREATE INDEX idx_account_users_account_id ON account_users(account_id);
CREATE INDEX idx_account_users_user_id ON account_users(user_id);

CREATE TABLE one_time_tokens (
    id SERIAL PRIMARY KEY NOT NULL,
    value VARCHAR NOT NULL,
    used boolean NOT NULL DEFAULT FALSE,
    created_at timestamp DEFAULT now() NOT NULL,
    updated_at timestamp DEFAULT now() NOT NULL
);
CREATE INDEX idx_one_time_tokens_usable ON one_time_tokens(value, used);
