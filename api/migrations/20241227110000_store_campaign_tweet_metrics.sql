ALTER TABLE campaigns ADD COLUMN impression_count int4 NOT NULL default 0;
ALTER TABLE campaigns ADD COLUMN reply_count int4 NOT NULL default 0;
ALTER TABLE campaigns ADD COLUMN repost_count int4 NOT NULL default 0;
ALTER TABLE campaigns ADD COLUMN like_count int4 NOT NULL default 0;

ALTER TABLE handles ALTER COLUMN score SET DEFAULT '0x0000000000000000000000000000000000000000000000000000000000000000';
ALTER TABLE handles ALTER COLUMN score DROP NOT NULL;
ALTER TABLE handles ALTER COLUMN last_scoring DROP NOT NULL;

ALTER TABLE handles ADD COLUMN avg_impression_count int4 NOT NULL default 0;
ALTER TABLE handles ADD COLUMN avg_reply_count int4 NOT NULL default 0;
ALTER TABLE handles ADD COLUMN avg_repost_count int4 NOT NULL default 0;
ALTER TABLE handles ADD COLUMN avg_like_count int4 NOT NULL default 0;
ALTER TABLE handles ADD COLUMN scored_tweet_count int4 NOT NULL default 0;
ALTER TABLE handles ADD COLUMN legacy_score varchar NOT NULL default '0x0000000000000000000000000000000000000000000000000000000000000000';


UPDATE handles SET legacy_score = score;
UPDATE handles SET last_scoring = NULL;

INSERT INTO topics (name) VALUES ('speaks_english'), ('speaks_spanish');
