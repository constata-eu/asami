-- Keep the exact signed transaction for safe reconciliation and rebroadcast.
-- This is public transaction data, not wallet secret material.
ALTER TABLE payout_attempts
  ADD COLUMN raw_transaction BYTEA;
