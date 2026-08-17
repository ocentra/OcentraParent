CREATE TABLE IF NOT EXISTS ocentra_account_provider_billing_mappings (
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

CREATE UNIQUE INDEX IF NOT EXISTS idx_ocentra_provider_billing_identity_provider
  ON ocentra_account_provider_billing_mappings (identity_provider, identity_provider_subject, provider);

CREATE UNIQUE INDEX IF NOT EXISTS idx_ocentra_provider_billing_customer
  ON ocentra_account_provider_billing_mappings (provider, provider_customer_id)
  WHERE provider_customer_id IS NOT NULL;

CREATE UNIQUE INDEX IF NOT EXISTS idx_ocentra_provider_billing_subscription
  ON ocentra_account_provider_billing_mappings (provider, provider_subscription_id)
  WHERE provider_subscription_id IS NOT NULL;

CREATE UNIQUE INDEX IF NOT EXISTS idx_ocentra_provider_billing_invoice
  ON ocentra_account_provider_billing_mappings (provider, provider_invoice_id)
  WHERE provider_invoice_id IS NOT NULL;
