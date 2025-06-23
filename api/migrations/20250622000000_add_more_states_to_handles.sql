ALTER TYPE handle_status RENAME VALUE 'unverified' TO 'never_connected';
ALTER TABLE handles
ALTER COLUMN status SET DEFAULT 'never_connected'::handle_status;

ALTER TYPE handle_status RENAME VALUE 'verified' TO 'setting_up';
ALTER TYPE handle_status ADD VALUE IF NOT EXISTS 'connecting' AFTER 'setting_up';
ALTER TYPE handle_status ADD VALUE IF NOT EXISTS 'disconnected' AFTER 'active';
ALTER TYPE handle_status ADD VALUE IF NOT EXISTS 'reconnecting' AFTER 'disconnected';
COMMIT;
UPDATE handles SET status = 'disconnected' WHERE x_refresh_token IS NULL AND status = 'active'; 

ALTER TABLE handles ADD COLUMN next_scoring TIMESTAMPTZ;

UPDATE handles SET next_scoring = last_scoring + INTERVAL '20 days' WHERE last_scoring IS NOT NULL;
