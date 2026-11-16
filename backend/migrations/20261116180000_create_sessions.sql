CREATE TABLE sessions (
    token TEXT PRIMARY KEY,
    account_id UUID NOT NULL REFERENCES accounts (id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
