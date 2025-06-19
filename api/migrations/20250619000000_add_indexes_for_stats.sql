CREATE INDEX accounts_created_at ON accounts(created_at);
CREATE INDEX handles_created_at ON handles(created_at);
CREATE INDEX handles_status ON handles(status);
CREATE INDEX campaigns_status ON campaigns(status);
CREATE INDEX campaigns_created_at ON campaigns(created_at);
CREATE INDEX campaigns_published ON campaigns(created_at) WHERE status = 'published';
CREATE INDEX collabs_status ON collabs(status);
CREATE INDEX collabs_created_at ON collabs(created_at);
CREATE INDEX collabs_cleared ON collabs(created_at) WHERE status = 'cleared';

CREATE OR REPLACE FUNCTION u256_hex_to_numeric(hex TEXT)
RETURNS numeric AS $$
DECLARE
    hex_clean TEXT;
    hex_bytes BYTEA;
    reversed_bytes BYTEA := '';
    i INTEGER;
    byte_val INTEGER;
    byte_hex TEXT;
    result NUMERIC := 0;
BEGIN
    -- Remove 0x prefix and pad to even length
    hex_clean := ltrim(hex, '0x');
    IF length(hex_clean) % 2 <> 0 THEN
        hex_clean := '0' || hex_clean;
    END IF;

    -- Convert hex to bytea
    hex_bytes := decode(hex_clean, 'hex');

    -- Reverse the byte order manually
    FOR i IN reverse length(hex_bytes)..1 LOOP
        byte_val := get_byte(hex_bytes, i - 1);
        -- Convert to 2-digit hex, then decode into single byte
        byte_hex := lpad(to_hex(byte_val), 2, '0');
        reversed_bytes := reversed_bytes || decode(byte_hex, 'hex');
    END LOOP;

    -- Convert reversed bytes into numeric
    FOR i IN 0..length(reversed_bytes) - 1 LOOP
        result := result + get_byte(reversed_bytes, i) * power(256::numeric, i);
    END LOOP;

    RETURN result;
END;
$$ LANGUAGE plpgsql IMMUTABLE;
