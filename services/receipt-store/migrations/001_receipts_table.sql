-- Receipt hash lookup table for hash chain verification
CREATE TABLE IF NOT EXISTS receipts (
    receipt_id UUID PRIMARY KEY,
    receipt_hash VARCHAR(64) NOT NULL,
    timestamp TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_receipts_timestamp ON receipts(timestamp DESC);
CREATE INDEX idx_receipts_hash ON receipts(receipt_hash);

