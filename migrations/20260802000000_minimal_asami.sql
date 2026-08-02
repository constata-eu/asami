-- Minimal Asami schema. All legacy tables and data are retained.
DELETE FROM audit_log_entries;

ALTER TABLE audit_log_entries ADD COLUMN IF NOT EXISTS message TEXT NOT NULL DEFAULT '';
ALTER TABLE audit_log_entries ADD COLUMN IF NOT EXISTS chain_id BIGINT;
ALTER TABLE audit_log_entries ADD COLUMN IF NOT EXISTS entity_type VARCHAR;
ALTER TABLE audit_log_entries ADD COLUMN IF NOT EXISTS entity_id VARCHAR;

CREATE TABLE asami_indexer_states (
  chain_id BIGINT NOT NULL,
  stream VARCHAR NOT NULL,
  next_block BIGINT NOT NULL,
  last_block_hash VARCHAR,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  PRIMARY KEY (chain_id, stream)
);

CREATE TABLE member_update_events (
  id BIGSERIAL PRIMARY KEY,
  chain_id BIGINT NOT NULL,
  member_id NUMERIC(78,0) NOT NULL,
  account VARCHAR(42) NOT NULL,
  score NUMERIC(78,0) NOT NULL,
  name TEXT NOT NULL,
  block_number BIGINT NOT NULL,
  block_hash VARCHAR(66) NOT NULL,
  transaction_hash VARCHAR(66) NOT NULL,
  transaction_index BIGINT NOT NULL,
  log_index BIGINT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  UNIQUE(chain_id, transaction_hash, log_index)
);

CREATE TABLE asset_config_update_events (
  id BIGSERIAL PRIMARY KEY,
  chain_id BIGINT NOT NULL,
  asset_address VARCHAR(42) NOT NULL,
  enabled BOOLEAN NOT NULL,
  minimum_amount NUMERIC(78,0) NOT NULL,
  block_number BIGINT NOT NULL,
  block_hash VARCHAR(66) NOT NULL,
  transaction_hash VARCHAR(66) NOT NULL,
  transaction_index BIGINT NOT NULL,
  log_index BIGINT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  UNIQUE(chain_id, transaction_hash, log_index)
);

CREATE TABLE sender_name_update_events (
  id BIGSERIAL PRIMARY KEY,
  chain_id BIGINT NOT NULL,
  account VARCHAR(42) NOT NULL,
  name TEXT NOT NULL,
  block_number BIGINT NOT NULL,
  block_hash VARCHAR(66) NOT NULL,
  transaction_hash VARCHAR(66) NOT NULL,
  transaction_index BIGINT NOT NULL,
  log_index BIGINT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  UNIQUE(chain_id, transaction_hash, log_index)
);

CREATE TABLE native_receipt_events (
  id BIGSERIAL PRIMARY KEY,
  chain_id BIGINT NOT NULL,
  sender VARCHAR(42) NOT NULL,
  amount NUMERIC(78,0) NOT NULL,
  block_number BIGINT NOT NULL,
  block_hash VARCHAR(66) NOT NULL,
  transaction_hash VARCHAR(66) NOT NULL,
  transaction_index BIGINT NOT NULL,
  log_index BIGINT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  UNIQUE(chain_id, transaction_hash, log_index)
);

CREATE TABLE payment_sent_events (
  id BIGSERIAL PRIMARY KEY,
  chain_id BIGINT NOT NULL,
  asset_address VARCHAR(42) NOT NULL,
  recipient VARCHAR(42) NOT NULL,
  amount NUMERIC(78,0) NOT NULL,
  block_number BIGINT NOT NULL,
  block_hash VARCHAR(66) NOT NULL,
  transaction_hash VARCHAR(66) NOT NULL,
  transaction_index BIGINT NOT NULL,
  log_index BIGINT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  UNIQUE(chain_id, transaction_hash, log_index)
);

CREATE TABLE incoming_transfers (
  id BIGSERIAL PRIMARY KEY,
  chain_id BIGINT NOT NULL,
  asset_address VARCHAR(42) NOT NULL,
  sender VARCHAR(42) NOT NULL,
  recipient VARCHAR(42) NOT NULL,
  amount NUMERIC(78,0) NOT NULL,
  transaction_hash VARCHAR(66) NOT NULL,
  log_index BIGINT NOT NULL,
  block_number BIGINT NOT NULL,
  block_hash VARCHAR(66) NOT NULL,
  transaction_index BIGINT NOT NULL,
  disbursement_id BIGINT,
  processing_status VARCHAR NOT NULL DEFAULT 'unprocessed',
  ineligibility_reason VARCHAR,
  first_evaluated_at TIMESTAMPTZ,
  last_evaluated_at TIMESTAMPTZ,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  UNIQUE(chain_id, transaction_hash, log_index)
);

CREATE TABLE disbursements (
  id BIGSERIAL PRIMARY KEY,
  incoming_transfer_id BIGINT NOT NULL UNIQUE REFERENCES incoming_transfers(id),
  chain_id BIGINT NOT NULL,
  asset_address VARCHAR(42) NOT NULL,
  received_amount NUMERIC(78,0) NOT NULL,
  distributable_amount NUMERIC(78,0) NOT NULL,
  retained_amount NUMERIC(78,0) NOT NULL,
  member_snapshot_block BIGINT NOT NULL,
  total_active_score NUMERIC(78,0) NOT NULL,
  sender VARCHAR(42) NOT NULL,
  status VARCHAR NOT NULL,
  failure_reason VARCHAR,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  completed_at TIMESTAMPTZ
);

ALTER TABLE incoming_transfers
  ADD CONSTRAINT incoming_transfers_disbursement_fk
  FOREIGN KEY (disbursement_id) REFERENCES disbursements(id);

CREATE TABLE payouts (
  id BIGSERIAL PRIMARY KEY,
  disbursement_id BIGINT NOT NULL REFERENCES disbursements(id),
  member_id NUMERIC(78,0) NOT NULL,
  recipient VARCHAR(42) NOT NULL,
  member_name TEXT NOT NULL,
  member_score NUMERIC(78,0) NOT NULL,
  asset_address VARCHAR(42) NOT NULL,
  amount NUMERIC(78,0) NOT NULL,
  status VARCHAR NOT NULL,
  attempt_count INTEGER NOT NULL DEFAULT 0,
  successful_transaction_hash VARCHAR(66),
  successful_block_number BIGINT,
  successful_receipt_recorded_at TIMESTAMPTZ,
  last_error VARCHAR,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  completed_at TIMESTAMPTZ,
  UNIQUE(disbursement_id, member_id)
);

CREATE TABLE payout_attempts (
  id BIGSERIAL PRIMARY KEY,
  payout_id BIGINT NOT NULL REFERENCES payouts(id),
  attempt_number INTEGER NOT NULL,
  from_address VARCHAR(42) NOT NULL,
  nonce NUMERIC(78,0) NOT NULL,
  transaction_hash VARCHAR(66) NOT NULL,
  submitted_at TIMESTAMPTZ,
  receipt_status BIGINT,
  receipt_block_number BIGINT,
  gas_limit NUMERIC(78,0),
  gas_used NUMERIC(78,0),
  effective_gas_price NUMERIC(78,0),
  error_type VARCHAR,
  error_message VARCHAR,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  resolved_at TIMESTAMPTZ,
  UNIQUE(payout_id, attempt_number),
  UNIQUE(transaction_hash)
);

CREATE TABLE tweet_queue (
  id BIGSERIAL PRIMARY KEY,
  kind VARCHAR NOT NULL,
  language VARCHAR(2) NOT NULL CHECK (language IN ('en', 'es')),
  account_key VARCHAR NOT NULL,
  payload_json JSONB NOT NULL,
  status VARCHAR NOT NULL DEFAULT 'pending',
  attempt_count INTEGER NOT NULL DEFAULT 0,
  rendered_text TEXT,
  x_post_id VARCHAR,
  last_error VARCHAR,
  not_before TIMESTAMPTZ NOT NULL DEFAULT now(),
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  sent_at TIMESTAMPTZ
);

CREATE TABLE seen_token_alerts (
  chain_id BIGINT NOT NULL,
  token_address VARCHAR(42) NOT NULL,
  first_detected_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  english_tweet_queue_id BIGINT REFERENCES tweet_queue(id),
  spanish_tweet_queue_id BIGINT REFERENCES tweet_queue(id),
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  PRIMARY KEY(chain_id, token_address)
);

CREATE TABLE notification_deduplications (
  dedupe_key VARCHAR PRIMARY KEY,
  expires_at TIMESTAMPTZ NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE x_accounts (
  account_key VARCHAR PRIMARY KEY,
  username VARCHAR NOT NULL,
  access_token_nonce BYTEA NOT NULL,
  access_token_ciphertext BYTEA NOT NULL,
  refresh_token_nonce BYTEA NOT NULL,
  refresh_token_ciphertext BYTEA NOT NULL,
  expires_at TIMESTAMPTZ NOT NULL,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE x_oauth_states (
  state VARCHAR PRIMARY KEY,
  account_key VARCHAR NOT NULL,
  verifier_nonce BYTEA NOT NULL,
  verifier_ciphertext BYTEA NOT NULL,
  expires_at TIMESTAMPTZ NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE VIEW current_members AS
SELECT DISTINCT ON (chain_id, member_id)
  chain_id, member_id, account, score, name,
  block_number, block_hash, transaction_hash, transaction_index, log_index
FROM member_update_events
ORDER BY chain_id, member_id, block_number DESC, transaction_index DESC, log_index DESC;

CREATE VIEW current_asset_configs AS
SELECT DISTINCT ON (chain_id, asset_address)
  chain_id, asset_address, enabled, minimum_amount,
  block_number, block_hash, transaction_hash, transaction_index, log_index
FROM asset_config_update_events
ORDER BY chain_id, asset_address, block_number DESC, transaction_index DESC, log_index DESC;

CREATE VIEW current_sender_names AS
SELECT DISTINCT ON (chain_id, account)
  chain_id, account, name,
  block_number, block_hash, transaction_hash, transaction_index, log_index
FROM sender_name_update_events
ORDER BY chain_id, account, block_number DESC, transaction_index DESC, log_index DESC;

CREATE VIEW unconfigured_seen_tokens AS
SELECT
  incoming.chain_id,
  incoming.asset_address AS token_address,
  MIN(incoming.block_number) AS first_seen_block,
  (ARRAY_AGG(incoming.transaction_hash ORDER BY incoming.block_number, incoming.transaction_index, incoming.log_index))[1] AS first_seen_transaction_hash,
  MIN(incoming.created_at) AS first_seen_at,
  COUNT(*) AS transfer_count,
  SUM(incoming.amount) AS total_observed_amount
FROM incoming_transfers incoming
WHERE incoming.asset_address <> '0x0000000000000000000000000000000000000000'
  AND NOT EXISTS (
    SELECT 1 FROM asset_config_update_events configured
    WHERE configured.chain_id = incoming.chain_id
      AND configured.asset_address = incoming.asset_address
  )
GROUP BY incoming.chain_id, incoming.asset_address;

CREATE INDEX incoming_transfers_candidates
  ON incoming_transfers(processing_status, id DESC) WHERE disbursement_id IS NULL;
CREATE INDEX payouts_processing_order ON payouts(status, disbursement_id, member_id);
CREATE UNIQUE INDEX tweet_queue_dedupe
  ON tweet_queue(kind, language, account_key, (payload_json->>'dedupe_key'));
CREATE INDEX tweet_queue_processing ON tweet_queue(status, not_before, id);
CREATE INDEX audit_log_entries_cursor ON audit_log_entries(id DESC);
