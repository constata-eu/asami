CREATE TYPE language AS ENUM (
  'en',
  'es'
);

ALTER TABLE one_time_tokens ADD COLUMN lookup_key VARCHAR NOT NULL;
ALTER TABLE one_time_tokens ADD COLUMN expires_at TIMESTAMPTZ NOT NULL DEFAULT now() + INTERVAL '3 DAYS';
ALTER TABLE one_time_tokens ADD COLUMN sent_at TIMESTAMPTZ;
ALTER TABLE one_time_tokens ADD COLUMN email TEXT;
ALTER TABLE one_time_tokens ADD COLUMN lang language;

-- The user ID can be used to link the email to an existing account instead, but it's not used.
ALTER TABLE one_time_tokens ADD COLUMN user_id INTEGER;

CREATE INDEX idx_one_time_tokens_sent_at ON one_time_tokens(sent_at);

