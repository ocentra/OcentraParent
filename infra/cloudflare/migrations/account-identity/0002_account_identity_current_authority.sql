-- Account-owned current authority. The provider subject is the only lookup
-- key; household/member/role/device/session/target state is durable here and
-- must never be selected by request fields.
CREATE TABLE IF NOT EXISTS ocentra_account_identity_current_authority (
  provider TEXT NOT NULL CHECK (provider IN ('authjs', 'firebase')),
  provider_subject TEXT NOT NULL,
  mapping_status TEXT NOT NULL CHECK (mapping_status IN ('active', 'revoked')),
  account_id TEXT NOT NULL,
  household_id TEXT NOT NULL,
  member_id TEXT NOT NULL,
  role TEXT NOT NULL CHECK (
    role IN ('parent-owner', 'co-parent-guardian', 'observer', 'child-profile',
             'child-device-agent', 'support-admin')
  ),
  account_state TEXT NOT NULL CHECK (account_state IN ('active', 'suspended', 'disabled')),
  membership_state TEXT NOT NULL CHECK (
    membership_state IN ('invited', 'pending', 'active', 'revoked', 'disabled')
  ),
  device_id TEXT NOT NULL,
  device_trust_state TEXT NOT NULL CHECK (
    device_trust_state IN ('pending', 'trusted', 'revoked', 'reset-required', 'disabled')
  ),
  session_freshness_state TEXT NOT NULL CHECK (
    session_freshness_state IN ('fresh', 'stale', 'expired')
  ),
  session_id TEXT NOT NULL,
  session_generation INTEGER NOT NULL CHECK (
    session_generation > 0 AND session_generation <= 9007199254740991
  ),
  session_expires_at TEXT NOT NULL,
  authority_generation INTEGER NOT NULL CHECK (
    authority_generation > 0 AND authority_generation <= 9007199254740991
  ),
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
  support_receipt_id TEXT,
  support_provider_subject TEXT,
  support_account_id TEXT,
  support_member_id TEXT,
  support_household_id TEXT,
  support_device_id TEXT,
  support_child_profile_id TEXT,
  support_child_device_id TEXT,
  support_scope TEXT CHECK (support_scope IS NULL OR support_scope IN ('read-only', 'household', 'device-control')),
  support_issuer TEXT,
  support_issued_at TEXT,
  support_expires_at TEXT,
  support_revocation_state TEXT CHECK (
    support_revocation_state IS NULL OR support_revocation_state IN ('active', 'revoked')
  ),
  support_audit_identity TEXT,
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL,
  PRIMARY KEY (provider, provider_subject),
  CHECK (
    (support_receipt_id IS NULL AND support_provider_subject IS NULL
     AND support_account_id IS NULL AND support_member_id IS NULL
     AND support_household_id IS NULL AND support_device_id IS NULL
     AND support_child_profile_id IS NULL AND support_child_device_id IS NULL
     AND support_scope IS NULL AND support_issuer IS NULL
     AND support_issued_at IS NULL AND support_expires_at IS NULL
     AND support_revocation_state IS NULL AND support_audit_identity IS NULL)
    OR
    (support_receipt_id IS NOT NULL AND support_provider_subject IS NOT NULL
     AND support_account_id IS NOT NULL AND support_member_id IS NOT NULL
     AND support_household_id IS NOT NULL AND support_device_id IS NOT NULL
     AND support_child_profile_id IS NOT NULL AND support_child_device_id IS NOT NULL
     AND support_scope IS NOT NULL AND support_issuer IS NOT NULL
     AND support_issued_at IS NOT NULL AND support_expires_at IS NOT NULL
     AND support_revocation_state IS NOT NULL AND support_audit_identity IS NOT NULL)
  ),
  CHECK (role <> 'support-admin' OR support_receipt_id IS NOT NULL)
);

CREATE INDEX IF NOT EXISTS idx_ocentra_account_identity_current_authority_account
  ON ocentra_account_identity_current_authority (account_id, household_id);

CREATE INDEX IF NOT EXISTS idx_ocentra_account_identity_current_authority_target
  ON ocentra_account_identity_current_authority (household_id, child_profile_id, child_device_id);
