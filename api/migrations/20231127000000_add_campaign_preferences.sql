ALTER TABLE campaign_requests ADD COLUMN campaign_id VARCHAR REFERENCES campaigns(id);
ALTER TABLE handle_requests ADD COLUMN handle_id VARCHAR REFERENCES handles(id);
ALTER TABLE collab_requests ADD COLUMN collab_id VARCHAR REFERENCES collabs(id);

ALTER TABLE sessions ADD COLUMN account_id VARCHAR REFERENCES accounts(id);
UPDATE sessions SET account_id = to_u256(user_id);
ALTER TABLE sessions ALTER COLUMN account_id SET NOT NULL;

CREATE TABLE campaign_preferences (
  id SERIAL PRIMARY KEY NOT NULL,
  account_id VARCHAR REFERENCES accounts(id) NOT NULL,
  campaign_id VARCHAR REFERENCES campaigns(id) NOT NULL,
  not_interested_on TIMESTAMPTZ,
  attempted_on TIMESTAMPTZ
);
CREATE INDEX idx_campaign_preferences_account_id ON campaign_preferences(account_id);
CREATE INDEX idx_campaign_preferences_campaign_id ON campaign_preferences(campaign_id);

