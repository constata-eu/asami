ALTER TABLE campaigns ADD COLUMN x_announcement_id_en VARCHAR, ADD COLUMN x_announcement_id_es VARCHAR;
UPDATE campaigns set x_announcement_id_en = 'skipped', x_announcement_id_es = 'skipped';
