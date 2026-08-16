CREATE TABLE IF NOT EXISTS ocentra_account_identities (
  account_id TEXT NOT NULL,
  provider TEXT NOT NULL CHECK (provider IN ('authjs', 'firebase')),
  provider_subject TEXT NOT NULL,
  status TEXT NOT NULL CHECK (status IN ('active', 'revoked')),
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL,
  PRIMARY KEY (provider, provider_subject)
);

CREATE INDEX IF NOT EXISTS idx_ocentra_account_identities_account_id
  ON ocentra_account_identities (account_id);
