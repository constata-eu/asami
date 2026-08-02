CREATE TYPE on_chain_tx_status AS ENUM (
  'created',
  'reverted',
  'submitted',
  'failure',
  'success'
);

ALTER TABLE on_chain_txs ADD COLUMN status on_chain_tx_status NOT NULL DEFAULT 'created';
UPDATE on_chain_txs SET status = 'success' where success;
UPDATE on_chain_txs SET status = 'failure' where not success;
ALTER TABLE on_chain_txs DROP COLUMN success;
