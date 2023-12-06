CREATE TABLE ig_campaign_rules (
  id SERIAL PRIMARY KEY NOT NULL,
  campaign_id VARCHAR REFERENCES campaigns(id) NOT NULL,
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
