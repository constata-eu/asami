ALTER TABLE on_chain_txs ADD COLUMN gas_used VARCHAR;
ALTER TABLE on_chain_txs ADD COLUMN nonce VARCHAR;
ALTER TABLE on_chain_txs ADD COLUMN message VARCHAR;

ALTER TABLE handles RENAME TO old_handles;
ALTER INDEX idx_handles_account_id RENAME TO old_handles_account_id;
ALTER INDEX idx_handles_site RENAME TO old_handles_site;
ALTER INDEX idx_handles_username RENAME TO  old_handles_username;
ALTER INDEX idx_handles_user_id RENAME TO old_handles_user_id;

ALTER TABLE handle_topics RENAME TO old_handle_topics;
ALTER INDEX idx_handle_topics_handle_id RENAME TO old_handle_topics_handle_id;
ALTER INDEX idx_handle_topics_topic_id RENAME TO old_handle_topics_topic_id;

/*
Rebuild handles
Handles that were already synced and validated should be kept.
But now they don't go to the smart contract, so anything verified is OK.
*/
ALTER TABLE handle_requests RENAME TO handles;
ALTER INDEX idx_handle_requests_account_id RENAME TO idx_handle_account_id;
ALTER INDEX idx_handle_requests_site RENAME TO idx_handle_site;
ALTER INDEX idx_handle_requests_status RENAME TO idx_handle_status;
ALTER INDEX idx_handle_requests_username RENAME TO idx_handle_username;
ALTER INDEX idx_handle_requests_user_id RENAME TO idx_handle_user_id;

ALTER TABLE handle_request_topics RENAME TO handle_topics;
ALTER INDEX idx_handle_request_topics_handle_id RENAME TO idx_handle_topics_handle_id;
ALTER INDEX idx_handle_request_topics_topic_id RENAME TO idx_handle_topics_topic_id;
ALTER TABLE handle_topics RENAME COLUMN handle_request_id TO handle_id;

ALTER TABLE handles RENAME COLUMN on_chain_tx_id TO old_on_chain_tx_id;
ALTER TABLE handles RENAME COLUMN price TO old_price;
ALTER TABLE handles RENAME COLUMN status TO old_status;
ALTER TABLE handles ALTER COLUMN old_status DROP NOT NULL;

CREATE TYPE handle_status AS ENUM (
  'unverified',
  'verified',
  'active',
  'inactive'
);

ALTER TABLE handles ADD COLUMN status handle_status;
ALTER TABLE handles ADD COLUMN last_scoring TIMESTAMPTZ NOT NULL DEFAULT now();
ALTER TABLE handles ADD COLUMN inactive_reason VARCHAR;

UPDATE handles SET status = 'active' WHERE old_status IN ('submitted', 'appraised', 'done', 'failed');
UPDATE handles SET status = 'verified' WHERE old_status = 'verified';
UPDATE handles SET status = 'unverified' WHERE status IS NULL;
UPDATE handles SET score = to_u256(2490) WHERE score > to_u256(2490);
ALTER TABLE handles ALTER COLUMN status SET DEFAULT 'unverified', ALTER COLUMN status SET NOT NULL;

ALTER TABLE campaigns RENAME TO old_campaigns;
ALTER INDEX idx_campaigns_account_id RENAME TO old_campaigns_account_id;
ALTER INDEX idx_campaigns_site RENAME TO old_campaigns_site;;
ALTER INDEX idx_campaigns_finished RENAME TO old_campaigns_finished;

ALTER TABLE campaign_topics RENAME TO old_campaign_topics;
ALTER INDEX idx_campaign_topics_campaign_id RENAME TO old_campaign_topics_campaign_id;
ALTER INDEX idx_campaign_topics_topic_id RENAME TO old_campaign_topics_topic_id;

CREATE TYPE campaign_kind AS ENUM (
  'x_repost',
  'ig_clone_post'
);

CREATE TABLE campaigns (
  id SERIAL PRIMARY KEY NOT NULL,
  account_id VARCHAR REFERENCES accounts(id) NOT NULL,
  campaign_kind campaign_kind NOT NULL,
  briefing_json TEXT NOT NULL,
  briefing_hash VARCHAR NOT NULL,
  budget VARCHAR NOT NULL,
  valid_until timestamp NOT NULL,
  report_hash VARCHAR,
  created_at TIMESTAMPTZ DEFAULT now() NOT NULL
);
CREATE INDEX idx_campaigns_account_id ON campaigns(account_id);
CREATE INDEX idx_campaigns_campaign_kind ON campaigns(campaign_kind);
CREATE INDEX idx_campaigns_budget ON campaigns(budget);
CREATE INDEX idx_campaigns_valid_until ON campaigns(valid_until);

-- Collabs rebuild
ALTER TABLE collabs RENAME TO old_collabs;
ALTER INDEX idx_collabs_handle_id RENAME TO old_collabs_handle_id;
ALTER INDEX idx_collabs_campaign_id RENAME TO old_collabs_campaign_id;

ALTER TABLE topics RENAME TO old_topics;
ALTER TABLE topic_requests RENAME TO old_topic_requests;

CREATE TYPE collab_status AS ENUM (
  'registered',
  'cleared',
  'failed',
  'disputed'
);

CREATE TABLE collabs (
  id SERIAL PRIMARY KEY NOT NULL,
  member_id VARCHAR REFERENCES accounts(id) NOT NULL,
  advertiser_id VARCHAR REFERENCES accounts(id) NOT NULL,
  campaign_id INTEGER REFERENCES campaigns(id) NOT NULL,
  collab_trigger_unique_id VARCHAR NOT NULL,
  handle_id INTEGER REFERENCES handles(id) NOT NULL,
  status collab_status NOT NULL,
  dispute_reason VARCHAR NOT NULL,
  reward VARCHAR NOT NULL,
  created_at TIMESTAMPTZ NOT NULL
);
CREATE INDEX idx_collabs_campaign_id ON collabs(campaign_id);
CREATE INDEX idx_collabs_handle_id ON collabs(handle_id);

CREATE TABLE on_chain_jobs (
  id SERIAL PRIMARY KEY NOT NULL,
  success boolean NOT NULL default false,
  function_name VARCHAR NOT NULL,
  tx_hash VARCHAR,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX idx_on_chain_jobs_function_name ON on_chain_jobs(function_name);
CREATE INDEX idx_on_chain_jobs_success ON on_chain_jobs(success);
CREATE INDEX idx_on_chain_jobs_tx_hash ON on_chain_jobs(tx_hash);

CREATE TABLE on_chain_jobs_campaigns (
  id SERIAL PRIMARY KEY NOT NULL,
  job_id INTEGER REFERENCES on_chain_jobs(id) NOT NULL,
  campaign_id INTEGER REFERENCES campaigns(id) NOT NULL
);

CREATE TABLE on_chain_jobs_collabs (
  id SERIAL PRIMARY KEY NOT NULL,
  job_id INTEGER REFERENCES on_chain_jobs(id) NOT NULL,
  collab_id INTEGER REFERENCES collabs(id) NOT NULL
);

CREATE TABLE topics (
  id SERIAL PRIMARY KEY NOT NULL,
  name VARCHAR NOT NULL,
  created_at timestamp DEFAULT now() NOT NULL
);

CREATE TABLE campaign_topics (
  id SERIAL PRIMARY KEY NOT NULL,
  campaign_id INTEGER REFERENCES campaigns(id) NOT NULL,
  topic_id INTEGER REFERENCES topics(id) NOT NULL,
  created_at timestamp DEFAULT now() NOT NULL,
  updated_at timestamp
);
CREATE INDEX idx_campaign_topics_campaign_id ON campaign_topics(campaign_id);
CREATE INDEX idx_campaign_topics_topic_id ON campaign_topics(topic_id);

