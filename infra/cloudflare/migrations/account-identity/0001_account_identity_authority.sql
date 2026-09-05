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

CREATE TABLE IF NOT EXISTS ocentra_account_identity_bindings (
  account_id TEXT NOT NULL,
  household_id TEXT NOT NULL,
  child_profile_id TEXT NOT NULL,
  child_device_id TEXT NOT NULL,
  pairing_id TEXT NOT NULL,
  installation_id TEXT NOT NULL,
  selected_route_id TEXT NOT NULL,
  pairing_state TEXT NOT NULL CHECK (pairing_state IN ('pending', 'paired', 'unpaired')),
  install_state TEXT NOT NULL CHECK (install_state IN ('pending', 'installed', 'failed')),
  selected_route TEXT NOT NULL CHECK (selected_route IN ('local', 'lan', 'remote', 'manual-required')),
  lifecycle_state TEXT NOT NULL CHECK (lifecycle_state IN ('pending', 'active', 'suspended', 'removed')),
  revocation_state TEXT NOT NULL CHECK (revocation_state IN ('active', 'revoked')),
  authority_generation INTEGER NOT NULL CHECK (
    authority_generation > 0 AND authority_generation <= 9007199254740991
  ),
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL,
  PRIMARY KEY (household_id, child_profile_id, child_device_id)
);

CREATE INDEX IF NOT EXISTS idx_ocentra_account_identity_bindings_account_id
  ON ocentra_account_identity_bindings (account_id);
