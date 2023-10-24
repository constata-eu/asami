CREATE TYPE site AS ENUM (
  'x',
  'instagram',
  'nostr'
);

CREATE TYPE auth_method_kind AS ENUM (
  'x',
  'instagram',
  'nostr',
  'eip712',
  'one_time_token',
  'google'
);

CREATE TYPE handle_status AS ENUM (
  'unverified',
  'verified',
  'appraised',
  'submitted',
  'active'
);

CREATE TYPE account_status AS ENUM (
  'managed',
  'claiming',
  'claimed'
);

CREATE TYPE campaign_request_status AS ENUM (
  'received',
  'paid',
  'submitted',
  'active'
);

CREATE TYPE collab_request_status AS ENUM (
  'received',
  'submitted',
  'active'
);

CREATE TABLE accounts (
    id SERIAL PRIMARY KEY NOT NULL,
    status account_status NOT NULL DEFAULT 'managed',
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
    fixed_id VARCHAR,
    price DECIMAL,
    tx_hash VARCHAR,
    status handle_status NOT NULL DEFAULT 'unverified',
    verification_message_id VARCHAR,
    score DECIMAL,
    created_at timestamp DEFAULT now() NOT NULL,
    updated_at timestamp
);
CREATE INDEX idx_handles_account_id ON handles(account_id);
CREATE INDEX idx_handles_site ON handles(site);
CREATE INDEX idx_handles_status ON handles(status);
ALTER TABLE handles ADD CONSTRAINT handles_unique  UNIQUE (site, value);

CREATE TABLE handle_topics (
    id SERIAL PRIMARY KEY NOT NULL,
    handle_id INTEGER REFERENCES handles(id) NOT NULL,
    topic_id INTEGER REFERENCES topics(id) NOT NULL,
    created_at timestamp DEFAULT now() NOT NULL,
    updated_at timestamp
);
CREATE INDEX idx_handle_topics_handle_id ON handle_topics(handle_id);
CREATE INDEX idx_handle_topics_topic_id ON handle_topics(topic_id);

CREATE TABLE campaigns (
    id SERIAL PRIMARY KEY NOT NULL,
    on_chain_id DECIMAL NOT NULL,
    account_id INTEGER REFERENCES accounts(id) NOT NULL,
    site site NOT NULL,
    budget DECIMAL NOT NULL,
    remaining DECIMAL NOT NULL,
    content_id VARCHAR NOT NULL,
    created_at timestamp DEFAULT now() NOT NULL,
    updated_at timestamp,
    tx_hash VARCHAR NOT NULL,
    block_number DECIMAL NOT NULL,
    log_index DECIMAL NOT NULL,
    finished BOOLEAN NOT NULL DEFAULT FALSE
);
CREATE INDEX idx_campaigns_account_id ON campaigns(account_id);
CREATE INDEX idx_campaigns_site ON campaigns(site);
CREATE INDEX idx_campaigns_finished ON campaigns(finished);
ALTER TABLE campaigns ADD CONSTRAINT campaigns_unique UNIQUE (block_number, log_index);

CREATE TABLE collab_requests (
    id SERIAL PRIMARY KEY NOT NULL,
    campaign_id INTEGER REFERENCES campaigns(id) NOT NULL,
    handle_id INTEGER REFERENCES handles(id) NOT NULL,
    status collab_request_status NOT NULL DEFAULT 'received',
    created_at timestamp DEFAULT now() NOT NULL,
    updated_at timestamp
);
CREATE INDEX idx_collab_requests_campaign_id ON collab_requests(campaign_id);
CREATE INDEX idx_collab_requests_handle_id ON collab_requests(handle_id);

CREATE TABLE collabs (
    id SERIAL PRIMARY KEY NOT NULL,
    campaign_id INTEGER REFERENCES campaigns(id) NOT NULL,
    handle_id INTEGER REFERENCES handles(id) NOT NULL,
    reward DECIMAL NOT NULL,
    created_at timestamp DEFAULT now() NOT NULL,
    updated_at timestamp
);
CREATE INDEX idx_collabs_campaign_id ON collabs(campaign_id);
CREATE INDEX idx_collabs_handle_id ON collabs(handle_id);

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

CREATE TABLE indexer_states (
    id SERIAL PRIMARY KEY NOT NULL,
    x_handle_verification_checkpoint INT8 NOT NULL DEFAULT 0,
    suggested_price_per_point DECIMAL NOT NULL DEFAULT 0.001,
    last_synced_block INT8 NOT NULL DEFAULT 0
);

CREATE TABLE campaign_requests (
    id SERIAL PRIMARY KEY NOT NULL,
    account_id INTEGER REFERENCES accounts(id) NOT NULL,
    budget DECIMAL(10,2) NOT NULL,
    site site NOT NULL,
    content_id VARCHAR NOT NULL,
    status campaign_request_status NOT NULL DEFAULT 'received',
    tx_hash VARCHAR,
    created_at timestamp DEFAULT now() NOT NULL,
    updated_at timestamp
);
CREATE INDEX idx_campaign_requestcs_account_id ON campaign_requests(account_id);
CREATE INDEX idx_campaign_requests_site ON campaign_requests(site);
