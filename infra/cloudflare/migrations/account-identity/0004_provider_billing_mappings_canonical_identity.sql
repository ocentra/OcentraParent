-- Forward migration for databases that already applied the legacy 0003
-- provider mapping shape. Account migration 0002 must run first so the
-- current-authority table is available as the only identity owner.
--
-- Every legacy row must resolve to exactly one current Account authority with
-- the same account, household, and canonical provider-subject key. Ambiguous,
-- stale, or cross-owner rows abort the migration instead of being dropped or
-- rebound to request-controlled identity data.
CREATE TABLE ocentra_account_provider_billing_mappings_canonical (
  mapping_id TEXT NOT NULL PRIMARY KEY,
  identity_provider TEXT NOT NULL CHECK (identity_provider IN ('authjs', 'firebase')),
  identity_provider_subject TEXT NOT NULL,
  provider TEXT NOT NULL CHECK (provider IN ('stripe', 'razorpay', 'paypal', 'apple', 'google')),
  provider_customer_id TEXT,
  provider_subscription_id TEXT,
  provider_invoice_id TEXT,
  billing_invoice_id TEXT,
  status TEXT NOT NULL CHECK (status IN ('active', 'revoked')),
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL,
  CHECK (provider_customer_id IS NOT NULL OR provider_subscription_id IS NOT NULL OR provider_invoice_id IS NOT NULL),
  FOREIGN KEY (identity_provider, identity_provider_subject)
    REFERENCES ocentra_account_identity_current_authority (provider, provider_subject)
);

CREATE TABLE ocentra_account_provider_billing_migration_guard (
  guard_id INTEGER PRIMARY KEY CHECK (guard_id = 0)
);

INSERT INTO ocentra_account_provider_billing_migration_guard (guard_id)
SELECT 1
WHERE EXISTS (
  SELECT legacy.mapping_id
  FROM ocentra_account_provider_billing_mappings AS legacy
  LEFT JOIN ocentra_account_identity_current_authority AS authority
    ON authority.account_id = legacy.account_id
   AND authority.household_id = legacy.family_ref
   AND legacy.parent_account_ref = authority.account_id
   AND legacy.billing_subject = authority.provider || ':' || authority.provider_subject
  GROUP BY legacy.mapping_id
  HAVING COUNT(authority.provider_subject) <> 1
);

INSERT INTO ocentra_account_provider_billing_mappings_canonical (
  mapping_id,
  identity_provider,
  identity_provider_subject,
  provider,
  provider_customer_id,
  provider_subscription_id,
  provider_invoice_id,
  billing_invoice_id,
  status,
  created_at,
  updated_at
)
SELECT
  legacy.mapping_id,
  authority.provider,
  authority.provider_subject,
  legacy.provider,
  legacy.provider_customer_id,
  legacy.provider_subscription_id,
  legacy.provider_invoice_id,
  legacy.billing_invoice_id,
  legacy.status,
  legacy.created_at,
  legacy.updated_at
FROM ocentra_account_provider_billing_mappings AS legacy
JOIN ocentra_account_identity_current_authority AS authority
  ON authority.account_id = legacy.account_id
 AND authority.household_id = legacy.family_ref
 AND legacy.parent_account_ref = authority.account_id
 AND legacy.billing_subject = authority.provider || ':' || authority.provider_subject;

DROP TABLE ocentra_account_provider_billing_migration_guard;
DROP TABLE ocentra_account_provider_billing_mappings;
ALTER TABLE ocentra_account_provider_billing_mappings_canonical
  RENAME TO ocentra_account_provider_billing_mappings;

CREATE UNIQUE INDEX idx_ocentra_provider_billing_identity_provider
  ON ocentra_account_provider_billing_mappings (identity_provider, identity_provider_subject, provider);

CREATE UNIQUE INDEX idx_ocentra_provider_billing_customer
  ON ocentra_account_provider_billing_mappings (provider, provider_customer_id)
  WHERE provider_customer_id IS NOT NULL;

CREATE UNIQUE INDEX idx_ocentra_provider_billing_subscription
  ON ocentra_account_provider_billing_mappings (provider, provider_subscription_id)
  WHERE provider_subscription_id IS NOT NULL;

CREATE UNIQUE INDEX idx_ocentra_provider_billing_invoice
  ON ocentra_account_provider_billing_mappings (provider, provider_invoice_id)
  WHERE provider_invoice_id IS NOT NULL;
