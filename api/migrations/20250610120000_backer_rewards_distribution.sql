ALTER TABLE indexer_states ADD COLUMN last_rewards_indexed_block NUMERIC NOT NULL DEFAULT 0;
UPDATE indexer_states SET last_rewards_indexed_block = 7317000;

CREATE TABLE backers (
    id SERIAL PRIMARY KEY NOT NULL,
    address VARCHAR NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX backers_address ON backers(address);

CREATE TABLE backer_disbursements (
    id SERIAL PRIMARY KEY NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    rif_claimed NUMERIC NOT NULL,
    btc_claimed NUMERIC NOT NULL,

    rif_usd_rate NUMERIC NOT NULL,
    btc_usd_rate NUMERIC NOT NULL,
    asami_issuance_rate NUMERIC NOT NULL,

    tx_hash VARCHAR NOT NULL
);

CREATE TABLE backer_payouts (
    id SERIAL PRIMARY KEY NOT NULL,
    backer_id INT REFERENCES backers(id),
    asami_amount NUMERIC NOT NULL,
    disbursement_id INT REFERENCES backer_disbursements(id) NOT NULL,
    paid BOOLEAN NOT NULL DEFAULT FALSE, 
    tx_hash VARCHAR,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX backer_payout_disbursement ON backer_payouts(disbursement_id);
CREATE INDEX backer_payout_backer_id ON backer_payouts(backer_id);

CREATE TABLE backer_stakes (
    id SERIAL PRIMARY KEY NOT NULL,
    backer_id INT REFERENCES backers(id),
    stake NUMERIC NOT NULL,
    disbursement_id INT REFERENCES backer_disbursements(id),
    date TIMESTAMPTZ NOT NULL
);
    
CREATE INDEX backer_stake_disbursement ON backer_stakes(disbursement_id);
CREATE INDEX backer_stake_backer_id ON backer_stakes(backer_id);
