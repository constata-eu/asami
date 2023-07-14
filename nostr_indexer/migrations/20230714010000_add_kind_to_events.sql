ALTER TABLE events ADD COLUMN  kind INTEGER;
UPDATE events SET kind = (payload::json->>kind)::integer;
CREATE INDEX events_kind ON events (kind);
ALTER TABLE events ALTER COLUMN kind SET NOT NULL;
