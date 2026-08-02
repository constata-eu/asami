CREATE TYPE series_name AS ENUM (
	'asami_doc_price',
	'asami_supply',
	'asami_assigned_tokens',
	'asami_issuance_rate',
    'asami_fee_pool'
);

CREATE TABLE value_series (
  id SERIAL PRIMARY KEY NOT NULL,
  name series_name NOT NULL,
  value VARCHAR NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX latest_series_entry ON value_series(name, created_at DESC);

INSERT INTO value_series (name, value) VALUES
    ('asami_doc_price',       '0x00000000000000000000000000000000000000000000000000086a5d8840d976'),
    ('asami_supply',          '0x000000000000000000000000000000000000000000027b72c2d12a4b2cbd1b72'),
    ('asami_assigned_tokens', '0x00000000000000000000000000000000000000000002ee4afec5005a9a06206f'),
    ('asami_issuance_rate',   '0x000000000000000000000000000000000000000000000267165ca88aba45eeca'),
    ('asami_fee_pool',        '0x000000000000000000000000000000000000000000000002a700977c68c116cb');

ALTER TABLE holders
ADD COLUMN estimated_total_doc_claimed VARCHAR NOT NULL DEFAULT  '0x0000000000000000000000000000000000000000000000000000000000000000',
ADD COLUMN force_hydrate BOOLEAN NOT NULL DEFAULT FALSE;

UPDATE holders SET force_hydrate = true;
