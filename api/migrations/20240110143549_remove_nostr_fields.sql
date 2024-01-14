-- Add migration script here
ALTER TABLE handle_requests DROP COLUMN nostr_affine_x;
ALTER TABLE handle_requests DROP COLUMN nostr_affine_y;
ALTER TABLE handles DROP COLUMN nostr_affine_x;
ALTER TABLE handles DROP COLUMN nostr_affine_y;
ALTER TABLE accounts DROP COLUMN nostr_self_managed;
ALTER TABLE accounts DROP COLUMN nostr_abuse_proven;
