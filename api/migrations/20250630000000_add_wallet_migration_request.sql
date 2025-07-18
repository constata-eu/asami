CREATE TYPE account_merge_status AS ENUM (
	'pending',
	'done',
    'abandoned'
);

CREATE TABLE account_merges (
  id SERIAL PRIMARY KEY NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  destination_id VARCHAR REFERENCES accounts(id) NOT NULL,
  source_id VARCHAR REFERENCES accounts(id),
  status account_merge_status NOT NULL DEFAULT 'pending',
  code VARCHAR
);

CREATE INDEX account_merges_code on account_merges(code);
CREATE INDEX account_merges_source on account_merges(source_id);
CREATE INDEX account_merges_destination on account_merges(destination_id);

ALTER TABLE sessions ADD COLUMN logged_out_at TIMESTAMPTZ;
