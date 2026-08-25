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
  identityProvider: 'authjs' | 'firebase';
  identityProviderSubject: string;
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
  identity_provider: 'authjs' | 'firebase';
  identity_provider_subject: string;
  account_id: string;
  household_id: string;
  mapping_status: 'active' | 'revoked';
  account_state: 'active' | 'suspended' | 'disabled';
  membership_state: 'invited' | 'pending' | 'active' | 'revoked' | 'disabled';
  lifecycle_state: 'pending' | 'active' | 'suspended' | 'removed';
  revocation_state: 'active' | 'revoked';
  provider: BillingProvider;
  provider_customer_id: string | null;
  provider_subscription_id: string | null;
  provider_invoice_id: string | null;
  billing_invoice_id: string | null;
  status: 'active' | 'revoked';
}

const PROVIDER_MAPPING_SELECT_BY_CUSTOMER_SQL = `
SELECT mapping.identity_provider, mapping.identity_provider_subject,
       authority.account_id, authority.household_id, authority.mapping_status,
       authority.account_state, authority.membership_state,
       authority.lifecycle_state, authority.revocation_state,
       mapping.provider, mapping.provider_customer_id,
       mapping.provider_subscription_id, mapping.provider_invoice_id,
       mapping.billing_invoice_id, mapping.status
FROM ocentra_account_provider_billing_mappings AS mapping
JOIN ocentra_account_identity_current_authority AS authority
  ON authority.provider = mapping.identity_provider
 AND authority.provider_subject = mapping.identity_provider_subject
WHERE mapping.provider = ? AND mapping.provider_customer_id = ?
LIMIT 1
`;

const PROVIDER_MAPPING_SELECT_BY_SUBSCRIPTION_SQL = `
SELECT mapping.identity_provider, mapping.identity_provider_subject,
       authority.account_id, authority.household_id, authority.mapping_status,
       authority.account_state, authority.membership_state,
       authority.lifecycle_state, authority.revocation_state,
       mapping.provider, mapping.provider_customer_id,
       mapping.provider_subscription_id, mapping.provider_invoice_id,
       mapping.billing_invoice_id, mapping.status
FROM ocentra_account_provider_billing_mappings AS mapping
JOIN ocentra_account_identity_current_authority AS authority
  ON authority.provider = mapping.identity_provider
 AND authority.provider_subject = mapping.identity_provider_subject
WHERE mapping.provider = ? AND mapping.provider_subscription_id = ?
LIMIT 1
`;

const PROVIDER_MAPPING_SELECT_BY_INVOICE_SQL = `
SELECT mapping.identity_provider, mapping.identity_provider_subject,
       authority.account_id, authority.household_id, authority.mapping_status,
       authority.account_state, authority.membership_state,
       authority.lifecycle_state, authority.revocation_state,
       mapping.provider, mapping.provider_customer_id,
       mapping.provider_subscription_id, mapping.provider_invoice_id,
       mapping.billing_invoice_id, mapping.status
FROM ocentra_account_provider_billing_mappings AS mapping
JOIN ocentra_account_identity_current_authority AS authority
  ON authority.provider = mapping.identity_provider
 AND authority.provider_subject = mapping.identity_provider_subject
WHERE mapping.provider = ? AND mapping.provider_invoice_id = ?
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
    (message.includes('no such table') || message.includes('no such column')) &&
    (message.includes('ocentra_account_provider_billing_mappings') ||
      message.includes('ocentra_account_identities') ||
      message.includes('ocentra_account_identity_current_authority'))
  );
}

function sameAuthority(left: ProviderBillingMappingRow, right: ProviderBillingMappingRow): boolean {
  return (
    left.identity_provider === right.identity_provider &&
    left.identity_provider_subject === right.identity_provider_subject &&
    left.account_id === right.account_id &&
    left.household_id === right.household_id
  );
}

function validAuthorityText(value: unknown): value is string {
  return typeof value === 'string' && normaliseReference(value) === value;
}

function validMappingRow(row: ProviderBillingMappingRow): boolean {
  return (
    (row.identity_provider === 'authjs' || row.identity_provider === 'firebase') &&
    validAuthorityText(row.identity_provider_subject) &&
    validAuthorityText(row.account_id) &&
    validAuthorityText(row.household_id) &&
    (row.mapping_status === 'active' || row.mapping_status === 'revoked') &&
    (row.account_state === 'active' || row.account_state === 'suspended' || row.account_state === 'disabled') &&
    (row.membership_state === 'invited' ||
      row.membership_state === 'pending' ||
      row.membership_state === 'active' ||
      row.membership_state === 'revoked' ||
      row.membership_state === 'disabled') &&
    (row.lifecycle_state === 'pending' ||
      row.lifecycle_state === 'active' ||
      row.lifecycle_state === 'suspended' ||
      row.lifecycle_state === 'removed') &&
    (row.revocation_state === 'active' || row.revocation_state === 'revoked') &&
    isProvider(row.provider) &&
    (row.provider_customer_id !== null || row.provider_subscription_id !== null || row.provider_invoice_id !== null) &&
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
    identityProvider: row.identity_provider,
    identityProviderSubject: row.identity_provider_subject,
    providerCustomerId: row.provider_customer_id,
    providerSubscriptionId: row.provider_subscription_id,
    providerInvoiceId: row.provider_invoice_id,
    billingSubject: `${row.identity_provider}:${row.identity_provider_subject}`,
    parentAccountRef: row.account_id,
    familyRef: row.household_id,
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
    if (
      first.status !== 'active' ||
      first.mapping_status !== 'active' ||
      first.account_state !== 'active' ||
      first.membership_state !== 'active' ||
      first.lifecycle_state !== 'active' ||
      first.revocation_state !== 'active'
    ) {
      return { status: 'rejected', reason: 'provider-mapping-inactive' };
    }
    if (
      rest.some(
        (row) =>
          row.status !== 'active' ||
          row.mapping_status !== 'active' ||
          row.account_state !== 'active' ||
          row.membership_state !== 'active' ||
          row.lifecycle_state !== 'active' ||
          row.revocation_state !== 'active' ||
          !sameAuthority(first, row)
      )
    ) {
      return { status: 'rejected', reason: 'provider-reference-mismatch' };
    }
    if (
      invoiceId !== null &&
      rows.some((row) => row.provider_invoice_id !== null && row.provider_invoice_id !== invoiceId)
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
