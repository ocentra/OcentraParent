import type { D1Database } from '@cloudflare/workers-types';

/** Provider names accepted at the signed webhook boundary. */
export type BillingProvider = 'stripe' | 'razorpay' | 'paypal' | 'apple' | 'google';

export interface ProviderBillingReferenceHints {
  customerId: string | null;
  subscriptionId: string | null;
  invoiceId: string | null;
}

export interface ProviderBillingAuthority {
  accountId: string;
  provider: BillingProvider;
  providerCustomerId: string | null;
  providerSubscriptionId: string | null;
  providerInvoiceId: string | null;
  billingSubject: string;
  parentAccountRef: string;
  familyRef: string;
  billingInvoiceId: string | null;
}

export type ProviderBillingAuthorityResult =
  | { status: 'trusted'; authority: ProviderBillingAuthority }
  | { status: 'not-found' }
  | { status: 'manual-required'; reason: 'account-identity-d1-binding-missing' | 'account-identity-d1-schema-missing' }
  | {
      status: 'rejected';
      reason: 'provider-reference-invalid' | 'provider-reference-mismatch' | 'provider-mapping-inactive';
    };

interface ProviderBillingMappingRow {
  account_id: string;
  provider: BillingProvider;
  provider_customer_id: string | null;
  provider_subscription_id: string | null;
  provider_invoice_id: string | null;
  billing_subject: string;
  parent_account_ref: string;
  family_ref: string;
  billing_invoice_id: string | null;
  status: 'active' | 'revoked';
}

const PROVIDER_MAPPING_SELECT_BY_CUSTOMER_SQL = `
SELECT account_id, provider, provider_customer_id, provider_subscription_id,
       provider_invoice_id, billing_subject, parent_account_ref, family_ref,
       billing_invoice_id, status
FROM ocentra_account_provider_billing_mappings
WHERE provider = ? AND provider_customer_id = ?
  AND EXISTS (SELECT 1 FROM ocentra_account_identities WHERE account_id = ocentra_account_provider_billing_mappings.account_id AND status = 'active')
LIMIT 1
`;

const PROVIDER_MAPPING_SELECT_BY_SUBSCRIPTION_SQL = `
SELECT account_id, provider, provider_customer_id, provider_subscription_id,
       provider_invoice_id, billing_subject, parent_account_ref, family_ref,
       billing_invoice_id, status
FROM ocentra_account_provider_billing_mappings
WHERE provider = ? AND provider_subscription_id = ?
  AND EXISTS (SELECT 1 FROM ocentra_account_identities WHERE account_id = ocentra_account_provider_billing_mappings.account_id AND status = 'active')
LIMIT 1
`;

const PROVIDER_MAPPING_SELECT_BY_INVOICE_SQL = `
SELECT account_id, provider, provider_customer_id, provider_subscription_id,
       provider_invoice_id, billing_subject, parent_account_ref, family_ref,
       billing_invoice_id, status
FROM ocentra_account_provider_billing_mappings
WHERE provider = ? AND provider_invoice_id = ?
  AND EXISTS (SELECT 1 FROM ocentra_account_identities WHERE account_id = ocentra_account_provider_billing_mappings.account_id AND status = 'active')
LIMIT 1
`;

const MAX_REFERENCE_LENGTH = 256;
const CONTROL_CHARACTER_PATTERN = /[\u0000-\u001f\u007f]/;

function normaliseReference(value: string | null): string | null {
  if (value === null) {
    return null;
  }
  const normalised = value.trim();
  return normalised.length > 0 &&
    normalised.length <= MAX_REFERENCE_LENGTH &&
    !CONTROL_CHARACTER_PATTERN.test(normalised)
    ? normalised
    : null;
}

function isProvider(value: string): value is BillingProvider {
  return value === 'stripe' || value === 'razorpay' || value === 'paypal' || value === 'apple' || value === 'google';
}

function isMissingSchemaError(error: unknown): boolean {
  const message = (error instanceof Error ? error.message : String(error)).toLowerCase();
  return (
    message.includes('no such table') &&
    (message.includes('ocentra_account_provider_billing_mappings') || message.includes('ocentra_account_identities'))
  );
}

function sameAuthority(left: ProviderBillingMappingRow, right: ProviderBillingMappingRow): boolean {
  return (
    left.account_id === right.account_id &&
    left.billing_subject === right.billing_subject &&
    left.parent_account_ref === right.parent_account_ref &&
    left.family_ref === right.family_ref
  );
}

function validAuthorityText(value: unknown): value is string {
  return typeof value === 'string' && normaliseReference(value) === value;
}

function validMappingRow(row: ProviderBillingMappingRow): boolean {
  return (
    validAuthorityText(row.account_id) &&
    validAuthorityText(row.billing_subject) &&
    validAuthorityText(row.parent_account_ref) &&
    validAuthorityText(row.family_ref) &&
    (row.provider_customer_id === null || validAuthorityText(row.provider_customer_id)) &&
    (row.provider_subscription_id === null || validAuthorityText(row.provider_subscription_id)) &&
    (row.provider_invoice_id === null || validAuthorityText(row.provider_invoice_id)) &&
    (row.billing_invoice_id === null || validAuthorityText(row.billing_invoice_id)) &&
    (row.status === 'active' || row.status === 'revoked')
  );
}

function toAuthority(row: ProviderBillingMappingRow): ProviderBillingAuthority {
  return {
    accountId: row.account_id,
    provider: row.provider,
    providerCustomerId: row.provider_customer_id,
    providerSubscriptionId: row.provider_subscription_id,
    providerInvoiceId: row.provider_invoice_id,
    billingSubject: row.billing_subject,
    parentAccountRef: row.parent_account_ref,
    familyRef: row.family_ref,
    billingInvoiceId: row.billing_invoice_id,
  };
}

export async function resolveProviderBillingAuthority(
  database: D1Database | undefined,
  provider: string,
  references: ProviderBillingReferenceHints
): Promise<ProviderBillingAuthorityResult> {
  if (!database) {
    return { status: 'manual-required', reason: 'account-identity-d1-binding-missing' };
  }
  if (!isProvider(provider)) {
    return { status: 'rejected', reason: 'provider-reference-invalid' };
  }

  const customerId = normaliseReference(references.customerId);
  const subscriptionId = normaliseReference(references.subscriptionId);
  const invoiceId = normaliseReference(references.invoiceId);
  const suppliedReferences = [customerId, subscriptionId, invoiceId].filter((value): value is string => value !== null);
  if (suppliedReferences.length === 0) {
    return { status: 'not-found' };
  }
  if (
    (references.customerId !== null && customerId === null) ||
    (references.subscriptionId !== null && subscriptionId === null) ||
    (references.invoiceId !== null && invoiceId === null)
  ) {
    return { status: 'rejected', reason: 'provider-reference-invalid' };
  }

  try {
    const rows: ProviderBillingMappingRow[] = [];
    if (customerId) {
      const row = await database
        .prepare(PROVIDER_MAPPING_SELECT_BY_CUSTOMER_SQL)
        .bind(provider, customerId)
        .first<ProviderBillingMappingRow>();
      if (row) rows.push(row);
    }
    if (subscriptionId) {
      const row = await database
        .prepare(PROVIDER_MAPPING_SELECT_BY_SUBSCRIPTION_SQL)
        .bind(provider, subscriptionId)
        .first<ProviderBillingMappingRow>();
      if (row) rows.push(row);
    }
    if (invoiceId) {
      const row = await database
        .prepare(PROVIDER_MAPPING_SELECT_BY_INVOICE_SQL)
        .bind(provider, invoiceId)
        .first<ProviderBillingMappingRow>();
      if (row) rows.push(row);
    }
    if (rows.length === 0) {
      return { status: 'not-found' };
    }
    if (rows.some((row) => !validMappingRow(row))) {
      return { status: 'rejected', reason: 'provider-reference-mismatch' };
    }
    const [first, ...rest] = rows;
    if (first.status !== 'active') {
      return { status: 'rejected', reason: 'provider-mapping-inactive' };
    }
    if (rest.some((row) => row.status !== 'active' || !sameAuthority(first, row))) {
      return { status: 'rejected', reason: 'provider-reference-mismatch' };
    }
    if (
      (invoiceId === null && rows.some((row) => row.provider_invoice_id !== null)) ||
      (invoiceId !== null &&
        rows.some((row) => row.provider_invoice_id !== null && row.provider_invoice_id !== invoiceId))
    ) {
      return { status: 'rejected', reason: 'provider-reference-mismatch' };
    }
    const merged: ProviderBillingMappingRow = {
      ...first,
      provider_customer_id:
        rows.find((row) => row.provider_customer_id === customerId)?.provider_customer_id ?? first.provider_customer_id,
      provider_subscription_id:
        rows.find((row) => row.provider_subscription_id === subscriptionId)?.provider_subscription_id ??
        first.provider_subscription_id,
      provider_invoice_id:
        rows.find((row) => row.provider_invoice_id === invoiceId)?.provider_invoice_id ?? first.provider_invoice_id,
      billing_invoice_id:
        rows.find((row) => row.billing_invoice_id !== null)?.billing_invoice_id ?? first.billing_invoice_id,
    };
    if (
      (customerId !== null && merged.provider_customer_id !== customerId) ||
      (subscriptionId !== null && merged.provider_subscription_id !== subscriptionId) ||
      (invoiceId !== null && (merged.provider_invoice_id !== invoiceId || merged.billing_invoice_id === null))
    ) {
      return { status: 'rejected', reason: 'provider-reference-mismatch' };
    }
    return { status: 'trusted', authority: toAuthority(merged) };
  } catch (error) {
    if (isMissingSchemaError(error)) {
      return { status: 'manual-required', reason: 'account-identity-d1-schema-missing' };
    }
    throw error;
  }
}
