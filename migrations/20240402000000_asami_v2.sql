CREATE TYPE account_status AS ENUM (
	'managed',
	'claiming',
	'claimed',
	'banned'
);
ALTER TABLE accounts ADD COLUMN status account_status NOT NULL DEFAULT 'managed';
ALTER TABLE accounts ADD COLUMN allows_gasless boolean NOT NULL DEFAULT FALSE;
UPDATE accounts SET status = 'claimed' WHERE addr IS NOT NULL;
ALTER TABLE accounts ADD COLUMN processed_for_legacy_claim boolean NOT NULL DEFAULT false;
UPDATE accounts SET processed_for_legacy_claim = true WHERE addr IS NOT NULL;

ALTER TABLE accounts ADD COLUMN claim_signature VARCHAR;
ALTER TABLE accounts ADD COLUMN claim_session_id VARCHAR REFERENCES sessions(id);
--ALTER TABLE accounts DROP COLUMN unclaimed_asami_tokens;
--ALTER TABLE accounts DROP COLUMN unclaimed_doc_rewards;

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

CREATE TYPE on_chain_job_kind AS ENUM (
	'promote_sub_accounts',
	'admin_legacy_claim_account',
	'admin_claim_balances_free',
	'gasless_claim_balances',
	'reimburse_campaigns',
	'submit_reports',
	'make_collabs',
	'make_sub_account_collabs',
	'claim_fee_pool_share',
	'apply_voted_fee_rate'
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

CREATE TYPE campaign_status AS ENUM (
  'draft',
  'submitted',
	'published'
);

CREATE TABLE campaigns (
  id SERIAL PRIMARY KEY NOT NULL,
  account_id VARCHAR REFERENCES accounts(id) NOT NULL,
	status campaign_status NOT NULL DEFAULT 'draft',
  advertiser_addr VARCHAR NOT NULL,
  campaign_kind campaign_kind NOT NULL,
  briefing_json TEXT NOT NULL,
  briefing_hash VARCHAR NOT NULL,
  budget VARCHAR NOT NULL,
  valid_until timestamp,
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
  status collab_status NOT NULL DEFAULT 'registered',
  dispute_reason VARCHAR,
  reward VARCHAR NOT NULL,
  fee VARCHAR,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX idx_collabs_campaign_id ON collabs(campaign_id);
CREATE INDEX idx_collabs_handle_id ON collabs(handle_id);

CREATE TYPE on_chain_job_status AS ENUM (
  'scheduled',
	'skipped',
  'reverted',
  'submitted',
  'failed',
  'confirmed',
	'settled'
);

CREATE TABLE on_chain_jobs (
  id SERIAL PRIMARY KEY NOT NULL,
	status on_chain_job_status NOT NULL DEFAULT 'scheduled',
  kind on_chain_job_kind NOT NULL,
  tx_hash VARCHAR,
	gas_used VARCHAR,
	nonce VARCHAR,
	block NUMERIC,
	status_line TEXT,
	sleep_until TIMESTAMPTZ NOT NULL DEFAULT now(),
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX idx_on_chain_jobs_kind ON on_chain_jobs(kind);
CREATE INDEX idx_on_chain_jobs_status ON on_chain_jobs(status);
CREATE INDEX idx_on_chain_jobs_tx_hash ON on_chain_jobs(tx_hash);
CREATE INDEX idx_on_chain_jobs_nonce ON on_chain_jobs(nonce);

CREATE TABLE on_chain_job_accounts (
  id SERIAL PRIMARY KEY NOT NULL,
  job_id INTEGER REFERENCES on_chain_jobs(id) NOT NULL,
  account_id VARCHAR REFERENCES accounts(id) NOT NULL
);
CREATE INDEX idx_on_chain_job_accounts_job_id ON on_chain_job_accounts(job_id);
CREATE INDEX idx_on_chain_job_accounts_account_id ON on_chain_job_accounts(account_id);

CREATE TABLE on_chain_job_campaigns (
  id SERIAL PRIMARY KEY NOT NULL,
  job_id INTEGER REFERENCES on_chain_jobs(id) NOT NULL,
  campaign_id INTEGER REFERENCES campaigns(id) NOT NULL
);
CREATE INDEX idx_on_chain_job_campaigns_job_id ON on_chain_job_campaigns(job_id);
CREATE INDEX idx_on_chain_job_campaigns_campaign_id ON on_chain_job_campaigns(campaign_id);

CREATE TABLE on_chain_job_collabs (
  id SERIAL PRIMARY KEY NOT NULL,
  job_id INTEGER REFERENCES on_chain_jobs(id) NOT NULL,
  collab_id INTEGER REFERENCES collabs(id) NOT NULL
);
CREATE INDEX idx_on_chain_job_collabs_job_id ON on_chain_job_collabs(job_id);
CREATE INDEX idx_on_chain_job_collabs_collab_id ON on_chain_job_collabs(collab_id);

CREATE TABLE holders (
  id SERIAL PRIMARY KEY NOT NULL,
  balance VARCHAR NOT NULL,
  address VARCHAR NOT NULL,
  last_fee_pool_share VARCHAR
);

CREATE TABLE on_chain_job_holders (
  id SERIAL PRIMARY KEY NOT NULL,
  job_id INTEGER REFERENCES on_chain_jobs(id) NOT NULL,
  holder_id INTEGER REFERENCES holders(id) NOT NULL
);
CREATE INDEX idx_on_chain_job_holders_job_id ON on_chain_job_holders(job_id);
CREATE INDEX idx_on_chain_job_holders_holder_id ON on_chain_job_holders(holder_id);

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

ALTER TABLE campaign_preferences RENAME TO old_campaign_preferences;

ALTER INDEX idx_campaign_preferences_account_id RENAME TO old_campaign_preferences_account_id;
ALTER INDEX idx_campaign_preferences_campaign_id RENAME TO cold_ampaign_preferences_campaign_id;

CREATE TABLE campaign_preferences (
  id SERIAL PRIMARY KEY NOT NULL,
  account_id VARCHAR REFERENCES accounts(id) NOT NULL,
  campaign_id INTEGER REFERENCES campaigns(id) NOT NULL,
  not_interested_on TIMESTAMPTZ,
  attempted_on TIMESTAMPTZ
);
CREATE INDEX idx_campaign_preferences_account_id ON campaign_preferences(account_id);
CREATE INDEX idx_campaign_preferences_campaign_id ON campaign_preferences(campaign_id);

CREATE TABLE handle_topics (
  id SERIAL PRIMARY KEY NOT NULL,
  handle_id integer REFERENCES handles(id) NOT NULL,
  topic_id integer REFERENCES topics(id) NOT NULL,
  created_at timestamp DEFAULT now() NOT NULL,
  updated_at timestamp
);
CREATE INDEX idx_handle_topics_handle_id ON handle_topics(handle_id);
CREATE INDEX idx_handle_topics_topic_id ON handle_topics(topic_id);

ALTER TABLE ig_campaign_rules RENAME TO old_ig_campaign_rules;
ALTER INDEX idx_ig_campaign_rules_campaign_id RENAME TO old_ig_campaign_rules_campaign_id;

CREATE TABLE ig_campaign_rules (
  id SERIAL PRIMARY KEY NOT NULL,
  campaign_id INTEGER REFERENCES campaigns(id) NOT NULL,
  image BYTEA NOT NULL,
  display_url TEXT NOT NULL,
  image_hash TEXT NOT NULL,
  caption TEXT NOT NULL
);
CREATE INDEX idx_ig_campaign_rules_campaign_id ON ig_campaign_rules(campaign_id);

