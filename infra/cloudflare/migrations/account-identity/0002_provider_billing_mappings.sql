-- Legacy provider mapping shape retained for one migration step only.
-- 0003_provider_billing_mappings_canonical_identity.sql performs the
-- fail-closed conversion to the Account-owned composite identity.
CREATE TABLE IF NOT EXISTS ocentra_account_provider_billing_mappings (
  mapping_id TEXT NOT NULL PRIMARY KEY,
  account_id TEXT NOT NULL,
  provider TEXT NOT NULL CHECK (provider IN ('stripe', 'razorpay', 'paypal', 'apple', 'google')),
  provider_customer_id TEXT,
  provider_subscription_id TEXT,
  provider_invoice_id TEXT,
  billing_subject TEXT NOT NULL,
  parent_account_ref TEXT NOT NULL,
  family_ref TEXT NOT NULL,
  billing_invoice_id TEXT,
  status TEXT NOT NULL CHECK (status IN ('active', 'revoked')),
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL,
  CHECK (provider_customer_id IS NOT NULL OR provider_subscription_id IS NOT NULL OR provider_invoice_id IS NOT NULL)
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_ocentra_provider_billing_customer
  ON ocentra_account_provider_billing_mappings (provider, provider_customer_id)
  WHERE provider_customer_id IS NOT NULL;

CREATE UNIQUE INDEX IF NOT EXISTS idx_ocentra_provider_billing_subscription
  ON ocentra_account_provider_billing_mappings (provider, provider_subscription_id)
  WHERE provider_subscription_id IS NOT NULL;

CREATE UNIQUE INDEX IF NOT EXISTS idx_ocentra_provider_billing_invoice
  ON ocentra_account_provider_billing_mappings (provider, provider_invoice_id)
  WHERE provider_invoice_id IS NOT NULL;
