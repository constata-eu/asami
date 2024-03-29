CREATE TYPE site AS ENUM (
  'x',
  'instagram',
  'linkedin',
  'facebook',
  'tiktok',
  'youtube',
  'nostr',
  'bluesky',
  'reserved1',
  'reserved2',
  'reserved3',
  'reserved4',
  'reserved5',
  'reserved6'
);

CREATE TYPE auth_method_kind AS ENUM (
  'x',
  'facebook',
  'eip712',
  'one_time_token'
);

CREATE TYPE handle_request_status AS ENUM (
  'unverified',
  'verified',
  'appraised',
  'submitted',
  'failed',
  'done'
);

CREATE TYPE campaign_request_status AS ENUM (
  'received',
  'paid',
  'approved',
  'submitted',
  'failed',
  'done'
);

CREATE TYPE generic_request_status AS ENUM (
  'received',
  'submitted',
  'failed',
  'done'
);

CREATE TABLE on_chain_txs (
  id SERIAL PRIMARY KEY NOT NULL,
  success boolean NOT NULL default false,
  function_name VARCHAR NOT NULL,
  tx_hash VARCHAR,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX idx_on_chain_txs_function_name ON on_chain_txs(function_name);
CREATE INDEX idx_on_chain_txs_success ON on_chain_txs(success);
CREATE INDEX idx_on_chain_txs_tx_hash ON on_chain_txs(tx_hash);

CREATE TABLE synced_events (
  id SERIAL PRIMARY KEY NOT NULL,
  address VARCHAR NOT NULL,
  block_number DECIMAL NOT NULL,
  block_hash VARCHAR NOT NULL,
  tx_hash VARCHAR NOT NULL,
  tx_index DECIMAL NOT NULL,
  log_index VARCHAR NOT NULL,
  data TEXT NOT NULL,
  created_at timestamp DEFAULT now() NOT NULL
);
CREATE INDEX idx_synced_events_tx_hash ON synced_events(tx_hash);
CREATE INDEX idx_synced_events_block_number ON synced_events(block_number);
CREATE INDEX idx_synced_events_log_index ON synced_events(log_index);
CREATE INDEX idx_synced_events_block_and_log_index ON synced_events(block_number, log_index);
CREATE INDEX idx_synced_events_block_hash ON synced_events(block_hash);
ALTER TABLE synced_events ADD CONSTRAINT synced_events_unique UNIQUE (block_number, log_index);

CREATE FUNCTION to_u256(a BIGINT)
RETURNS VARCHAR AS
$$
BEGIN
    RETURN concat('0x', lpad(to_hex(a), 64, '0'));
END;
$$ LANGUAGE plpgsql;

CREATE FUNCTION to_wei(a BIGINT)
RETURNS VARCHAR AS
$$
BEGIN
    RETURN to_u256((a * 1e18)::INT8);
END;
$$ LANGUAGE plpgsql;

CREATE SEQUENCE next_account_seq;
CREATE TABLE accounts (
    id VARCHAR PRIMARY KEY UNIQUE NOT NULL DEFAULT to_u256(nextval('next_account_seq')),
    name VARCHAR,
    addr VARCHAR,
    unclaimed_asami_tokens VARCHAR NOT NULL DEFAULT to_u256(0),
    unclaimed_doc_rewards VARCHAR NOT NULL DEFAULT to_u256(0),
    created_at timestamp DEFAULT now() NOT NULL,
    updated_at timestamp
);

CREATE INDEX idx_accounts_addr ON accounts(addr);

CREATE TABLE users (
    id SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR NOT NULL,
    created_at timestamp DEFAULT now() NOT NULL,
    updated_at timestamp
);

CREATE TABLE deletions (
    id SERIAL PRIMARY KEY NOT NULL,
    account_id VARCHAR REFERENCES accounts(id),
    user_id INTEGER REFERENCES users(id),
    created_at timestamp DEFAULT now() NOT NULL,
    updated_at timestamp
);
CREATE INDEX idx_deletions_users_account_id ON deletions(account_id);
CREATE INDEX idx_deletions_users_user_id ON deletions(user_id);

CREATE TABLE topics (
    id VARCHAR UNIQUE NOT NULL,
    name VARCHAR NOT NULL,
    created_at timestamp DEFAULT now() NOT NULL,
    updated_at timestamp
);

CREATE TABLE topic_requests (
    id SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR NOT NULL, 
    status generic_request_status NOT NULL DEFAULT 'received',
    on_chain_tx_id INTEGER REFERENCES on_chain_txs(id),
    created_at timestamp DEFAULT now() NOT NULL,
    updated_at timestamp
);

CREATE TABLE handles (
  id VARCHAR UNIQUE NOT NULL,
  account_id VARCHAR REFERENCES accounts(id) NOT NULL,
  site site NOT NULL,
  username VARCHAR,
  user_id VARCHAR,
  price VARCHAR NOT NULL,
  score VARCHAR NOT NULL,
  created_at timestamp DEFAULT now() NOT NULL,
  updated_at timestamp
);
CREATE INDEX idx_handles_account_id ON handles(account_id);
CREATE INDEX idx_handles_site ON handles(site);
CREATE INDEX idx_handles_username ON handles(username);
CREATE INDEX idx_handles_user_id ON handles(user_id);

CREATE TABLE handle_requests (
    id SERIAL PRIMARY KEY NOT NULL,
    account_id VARCHAR REFERENCES accounts(id) NOT NULL,
    site site NOT NULL,
    username VARCHAR NOT NULL,
    user_id VARCHAR,
    price VARCHAR,
    score VARCHAR,
    status handle_request_status NOT NULL DEFAULT 'unverified',
    on_chain_tx_id INTEGER REFERENCES on_chain_txs(id),
    created_at timestamp DEFAULT now() NOT NULL,
    updated_at timestamp
);
CREATE INDEX idx_handle_requests_account_id ON handle_requests(account_id);
CREATE INDEX idx_handle_requests_site ON handle_requests(site);
CREATE INDEX idx_handle_requests_status ON handle_requests(status);
CREATE INDEX idx_handle_requests_username ON handle_requests(username);
CREATE INDEX idx_handle_requests_user_id ON handle_requests(user_id);

CREATE TABLE handle_request_topics (
    id SERIAL PRIMARY KEY NOT NULL,
    handle_request_id INTEGER REFERENCES handle_requests(id) NOT NULL,
    topic_id VARCHAR REFERENCES topics(id) NOT NULL,
    created_at timestamp DEFAULT now() NOT NULL,
    updated_at timestamp
);
CREATE INDEX idx_handle_request_topics_handle_id ON handle_request_topics(handle_request_id);
CREATE INDEX idx_handle_request_topics_topic_id ON handle_request_topics(topic_id);


CREATE TABLE handle_topics (
  id SERIAL PRIMARY KEY NOT NULL,
  handle_id VARCHAR REFERENCES handles(id) NOT NULL,
  topic_id VARCHAR REFERENCES topics(id) NOT NULL,
  created_at timestamp DEFAULT now() NOT NULL,
  updated_at timestamp
);
CREATE INDEX idx_handle_topics_handle_id ON handle_topics(handle_id);
CREATE INDEX idx_handle_topics_topic_id ON handle_topics(topic_id);

CREATE TABLE set_score_and_topics_requests (
  id SERIAL PRIMARY KEY NOT NULL,
  account_id VARCHAR REFERENCES accounts(id) NOT NULL,
  handle_id VARCHAR REFERENCES handles(id) NOT NULL,
  score VARCHAR NOT NULL,
  status generic_request_status NOT NULL DEFAULT 'received',
  on_chain_tx_id INTEGER REFERENCES on_chain_txs(id),
  created_at timestamp DEFAULT now() NOT NULL,
  updated_at timestamp
);
CREATE INDEX idx_set_score_and_topics_requests_handle_id ON set_score_and_topics_requests(handle_id);
CREATE INDEX idx_set_score_and_topics_requests_status ON set_score_and_topics_requests(status);

CREATE TABLE set_score_and_topics_request_topics (
  id SERIAL PRIMARY KEY NOT NULL,
  set_score_and_topics_request_id INTEGER REFERENCES set_score_and_topics_requests(id) NOT NULL,
  topic_id VARCHAR REFERENCES topics(id) NOT NULL,
  created_at timestamp DEFAULT now() NOT NULL,
  updated_at timestamp
);
CREATE INDEX idx_set_score_and_topics_request_topics_handle_id ON set_score_and_topics_request_topics(set_score_and_topics_request_id);
CREATE INDEX idx_set_score_and_topics_request_topics_topic_id ON set_score_and_topics_request_topics(topic_id);

CREATE TABLE set_price_requests (
  id SERIAL PRIMARY KEY NOT NULL,
  account_id VARCHAR REFERENCES accounts(id) NOT NULL,
  handle_id VARCHAR REFERENCES handles(id) NOT NULL,
  price VARCHAR NOT NULL,
  status generic_request_status NOT NULL DEFAULT 'received',
  on_chain_tx_id INTEGER REFERENCES on_chain_txs(id),
  created_at timestamp DEFAULT now() NOT NULL,
  updated_at timestamp
);
CREATE INDEX idx_set_price_requests_handle_id ON set_price_requests(handle_id);
CREATE INDEX idx_set_price_requests_status ON set_price_requests(status);
CREATE INDEX idx_set_price_requests_on_chain_tx_id ON set_price_requests(on_chain_tx_id);

CREATE TABLE campaigns (
  id VARCHAR UNIQUE NOT NULL,
  account_id VARCHAR REFERENCES accounts(id) NOT NULL,
  site site NOT NULL,
  budget VARCHAR NOT NULL,
  remaining VARCHAR NOT NULL,
  content_id VARCHAR NOT NULL,
  price_score_ratio VARCHAR NOT NULL,
  valid_until timestamp NOT NULL,
  finished BOOLEAN NOT NULL DEFAULT FALSE,
  created_at timestamp DEFAULT now() NOT NULL,
  tx_hash VARCHAR NOT NULL,
  updated_at timestamp,
  funded_by_admin boolean NOT NULL
);
CREATE INDEX idx_campaigns_account_id ON campaigns(account_id);
CREATE INDEX idx_campaigns_site ON campaigns(site);
CREATE INDEX idx_campaigns_finished ON campaigns(finished);

CREATE TABLE collabs (
  id VARCHAR UNIQUE NOT NULL,
  handle_id VARCHAR REFERENCES handles(id) NOT NULL,
  member_id VARCHAR REFERENCES accounts(id) NOT NULL,
  campaign_id VARCHAR REFERENCES campaigns(id) NOT NULL,
  advertiser_id VARCHAR REFERENCES accounts(id) NOT NULL,
  gross VARCHAR NOT NULL,
  fee VARCHAR NOT NULL,
  proof VARCHAR,
  created_at VARCHAR NOT NULL
);
CREATE INDEX idx_collabs_campaign_id ON collabs(campaign_id);
CREATE INDEX idx_collabs_handle_id ON collabs(handle_id);

CREATE TABLE collab_requests (
  id SERIAL PRIMARY KEY NOT NULL,
  campaign_id VARCHAR REFERENCES campaigns(id) NOT NULL,
  handle_id VARCHAR REFERENCES handles(id) NOT NULL,
  status generic_request_status NOT NULL DEFAULT 'received',
  on_chain_tx_id INTEGER REFERENCES on_chain_txs(id),
  created_at timestamp DEFAULT now() NOT NULL,
  updated_at timestamp
);
CREATE INDEX idx_collab_requests_campaign_id ON collab_requests(campaign_id);
CREATE INDEX idx_collab_requests_on_chain_tx_id ON collab_requests(on_chain_tx_id);
CREATE INDEX idx_collab_requests_handle_id ON collab_requests(handle_id);

CREATE TABLE campaign_topics (
  id SERIAL PRIMARY KEY NOT NULL,
  campaign_id VARCHAR REFERENCES campaigns(id) NOT NULL,
  topic_id VARCHAR REFERENCES topics(id) NOT NULL,
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
    account_id VARCHAR REFERENCES accounts(id) NOT NULL,
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
    account_id VARCHAR REFERENCES accounts(id),
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
    x_handle_verification_checkpoint BIGINT NOT NULL DEFAULT 0,
    suggested_price_per_point VARCHAR NOT NULL DEFAULT to_u256(1e16::INT8),
    last_synced_block DECIMAL NOT NULL DEFAULT 0
);

CREATE TABLE campaign_requests (
    id SERIAL PRIMARY KEY NOT NULL,
    account_id VARCHAR REFERENCES accounts(id) NOT NULL,
    handle_id VARCHAR REFERENCES handles(id),
    collab_id VARCHAR REFERENCES collabs(id),
    budget VARCHAR NOT NULL,
    site site NOT NULL,
    content_id VARCHAR NOT NULL,
    price_score_ratio VARCHAR NOT NULL,
    valid_until timestamp NOT NULL,
    status campaign_request_status NOT NULL DEFAULT 'received',
    approval_id INTEGER REFERENCES on_chain_txs(id),
    on_chain_tx_id INTEGER REFERENCES on_chain_txs(id),
    created_at timestamp DEFAULT now() NOT NULL,
    updated_at timestamp
);
CREATE INDEX idx_campaign_requests_account_id ON campaign_requests(account_id);
CREATE INDEX idx_campaign_requests_approval_id ON campaign_requests(approval_id);
CREATE INDEX idx_campaign_requests_on_chain_tx_id ON campaign_requests(on_chain_tx_id);
CREATE INDEX idx_campaign_requests_site ON campaign_requests(site);

CREATE TABLE campaign_request_topics (
    id SERIAL PRIMARY KEY NOT NULL,
    campaign_request_id INTEGER REFERENCES campaign_requests(id) NOT NULL,
    topic_id VARCHAR REFERENCES topics(id) NOT NULL,
    created_at timestamp DEFAULT now() NOT NULL,
    updated_at timestamp
);
CREATE INDEX idx_campaign_request_topics_campaign_id ON campaign_request_topics(campaign_request_id);
CREATE INDEX idx_campaign_request_topics_topic_id ON campaign_request_topics(topic_id);

CREATE TABLE claim_account_requests (
    id SERIAL PRIMARY KEY NOT NULL,
    account_id VARCHAR REFERENCES accounts(id) NOT NULL,
    addr VARCHAR NOT NULL,
    signature VARCHAR NOT NULL,
    session_id VARCHAR REFERENCES sessions(id) NOT NULL,
    status generic_request_status NOT NULL DEFAULT 'received',
    on_chain_tx_id INTEGER REFERENCES on_chain_txs(id)
);
CREATE INDEX idx_claim_account_request_account_id ON claim_account_requests(account_id);
CREATE INDEX idx_claim_account_request_on_chain_tx_id ON claim_account_requests(on_chain_tx_id);
CREATE INDEX idx_claim_account_request_status ON claim_account_requests(status);

CREATE TABLE campaign_preferences (
  id SERIAL PRIMARY KEY NOT NULL,
  account_id VARCHAR REFERENCES accounts(id) NOT NULL,
  campaign_id VARCHAR REFERENCES campaigns(id) NOT NULL,
  not_interested_on TIMESTAMPTZ,
  attempted_on TIMESTAMPTZ
);
CREATE INDEX idx_campaign_preferences_account_id ON campaign_preferences(account_id);
CREATE INDEX idx_campaign_preferences_campaign_id ON campaign_preferences(campaign_id);

CREATE TABLE ig_campaign_rules (
  id SERIAL PRIMARY KEY NOT NULL,
  campaign_id VARCHAR REFERENCES campaigns(id) NOT NULL,
  image BYTEA NOT NULL,
  display_url TEXT NOT NULL,
  image_hash TEXT NOT NULL,
  caption TEXT NOT NULL
);
CREATE INDEX idx_ig_campaign_rules_campaign_id ON ig_campaign_rules(campaign_id);

CREATE TYPE ig_crawl_status AS ENUM (
  'scheduled',
  'submitted',
  'responded',
  'cancelled'
);

CREATE TABLE ig_crawls (
  id SERIAL PRIMARY KEY NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  status ig_crawl_status NOT NULL DEFAULT 'scheduled',
  input TEXT NOT NULL,
  apify_id VARCHAR,
  processed_for_campaign_rules BOOLEAN NOT NULL DEFAULT false,
  processed_for_handle_requests BOOLEAN NOT NULL DEFAULT false,
  processed_for_collabs BOOLEAN NOT NULL DEFAULT false,
  log_text TEXT NOT NULL DEFAULT ''
);
CREATE INDEX idx_ig_crawls_status ON ig_crawls(status);
CREATE INDEX idx_ig_crawls_apify_id ON ig_crawls(apify_id);

CREATE TABLE ig_crawl_results (
  id SERIAL PRIMARY KEY NOT NULL,
  crawl_id INTEGER REFERENCES ig_crawls(id),
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  json_string TEXT NOT NULL,
  processed_for_campaign_rules BOOLEAN NOT NULL DEFAULT false,
  processed_for_handle_requests BOOLEAN NOT NULL DEFAULT false,
  processed_for_collabs BOOLEAN NOT NULL DEFAULT false,
  log_text TEXT NOT NULL DEFAULT ''
);
CREATE INDEX idx_ig_crawl_results_crawl_id ON ig_crawl_results(crawl_id);

CREATE TYPE audit_log_severity AS ENUM (
  'trace',
  'debug',
  'info',
  'warn',
  'fail'
);
CREATE TABLE audit_log_entries (
  id SERIAL PRIMARY KEY NOT NULL,
  severity audit_log_severity NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  kind VARCHAR NOT NULL,
  subkind VARCHAR NOT NULL,
  context TEXT NOT NULL,
  loggable_type VARCHAR,
  loggable_id VARCHAR
);
CREATE INDEX idx_audit_log_entries_severity ON audit_log_entries(severity);
CREATE INDEX idx_audit_log_entries_kind ON audit_log_entries(kind);
CREATE INDEX idx_audit_log_entries_loggable ON audit_log_entries(loggable_type, loggable_id);

