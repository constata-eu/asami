ALTER TABLE one_time_tokens ADD COLUMN lookup_key VARCHAR NOT NULL;
ALTER TABLE one_time_tokens ADD COLUMN user_id INTEGER;
ALTER TABLE one_time_tokens ADD COLUMN expires_at TIMESTAMPTZ NOT NULL DEFAULT now() + INTERVAL '3 DAYS';
ALTER TABLE one_time_tokens ADD COLUMN sent_at TIMESTAMPTZ;

CREATE INDEX idx_one_time_tokens_sent_at ON one_time_tokens(sent_at);

