import type {
  AnalyticsEngineDataset,
  D1Database,
  D1PreparedStatement,
  KVNamespace,
  R2Bucket,
} from '@cloudflare/workers-types';
import { BillingEntitlementSeatCompositionSchema } from '@ocentra-parent/schema-domain/billing-entitlement';
import {
  BillingAuditReferenceSchema,
  BillingEntitlementSnapshotIdSchema,
  BillingEntitlementSourceSchema,
  BillingFailureKindSchema,
  BillingLocalSafetyBehaviorSchema,
  BillingParentResolutionSchema,
  BillingParentVisibleStateSchema,
  BillingPlanActiveStateSchema,
  BillingPlanIdSchema,
  BillingSignatureStateSchema,
  BillingSubscriptionStatusSchema,
  NonNegativeBillingCountSchema,
  PositiveBillingLimitSchema,
} from '@ocentra-parent/schema-domain/billing-entitlement-values';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import { BillingReferralSummarySchema, type BillingReferralSummary } from './generated/billing-contracts.js';
import type { Env } from './env.js';
import {
  buildBillingInvoices,
  buildBillingReferralSummary,
  buildBillingStatusSummary,
  buildEntitlementSnapshot,
  buildLicenseDecision,
  listAdminBillingAccounts,
  listAdminBillingDisputes,
  listAdminBillingInvoices,
  listAdminBillingReferrals,
  listBillingAuditEvents,
  LOCAL_PRICING_PLANS,
  type AdminBillingAccountSummary,
  type AdminBillingDisputeSummary,
  type AdminBillingInvoiceSummary,
  type AdminBillingReferralSummary,
  type BillingAuditEventSummary,
  type BillingCancellationSummary,
  type BillingEntitlementSnapshotSummary,
  type BillingFailureStateSummary,
  type BillingInvoiceSummary,
  type BillingLicenseDecisionSummary,
  type BillingSeatCompositionSummary,
  type BillingStatusSummary,
  type PricingFeatureSummary,
  type PricingPlanSummary,
} from './fixtures.js';

const GENERATED_AT = '2026-06-14T00:00:00.000Z';
const RECONCILIATION_PARENT_ACCOUNT_REF = 'parent-account:billing-reconciliation';
const RECONCILIATION_FAMILY_REF = 'family:billing-reconciliation';
const DEFAULT_BILLING_SUBJECTS = [
  'parent:demo-active',
  'parent:demo-maxed',
  'parent:demo-grace',
  'parent:demo-review',
] as const;

const PRICING_PLANS_KEY = 'billing:pricing-plans';
const AUDIT_EVENTS_KEY = 'billing/audit-events.json';
const TOUCH_KEY_PREFIX = 'billing-touch:';

function normalizeSql(sql: string): string {
  return sql.replace(/\s+/g, ' ').trim();
}

const CREATE_READ_MODEL_SCHEMA_SQL = [
  'CREATE TABLE IF NOT EXISTS billing_status (subject TEXT PRIMARY KEY, payload_json TEXT NOT NULL)',
  'CREATE TABLE IF NOT EXISTS billing_invoices (subject TEXT NOT NULL, invoice_id TEXT PRIMARY KEY, payload_json TEXT NOT NULL)',
  'CREATE TABLE IF NOT EXISTS billing_referrals (subject TEXT PRIMARY KEY, payload_json TEXT NOT NULL)',
  'CREATE TABLE IF NOT EXISTS billing_snapshots (subject TEXT PRIMARY KEY, payload_json TEXT NOT NULL)',
  'CREATE TABLE IF NOT EXISTS billing_admin_accounts (parent_account_ref TEXT PRIMARY KEY, payload_json TEXT NOT NULL)',
  'CREATE TABLE IF NOT EXISTS billing_admin_invoices (invoice_id TEXT PRIMARY KEY, payload_json TEXT NOT NULL)',
  'CREATE TABLE IF NOT EXISTS billing_admin_disputes (dispute_id TEXT PRIMARY KEY, payload_json TEXT NOT NULL)',
  'CREATE TABLE IF NOT EXISTS billing_admin_referrals (referral_code TEXT PRIMARY KEY, payload_json TEXT NOT NULL)',
].join(';\n');

const SELECT_STATUS_ROW_COUNT_SQL = normalizeSql('SELECT COUNT(*) AS row_count FROM billing_status');
const SELECT_STATUS_BY_SUBJECT_SQL = normalizeSql('SELECT payload_json FROM billing_status WHERE subject = ?1 LIMIT 1');
const SELECT_INVOICES_BY_SUBJECT_SQL = normalizeSql(
  'SELECT payload_json FROM billing_invoices WHERE subject = ?1 ORDER BY invoice_id'
);
const SELECT_INVOICE_SUBJECT_SQL = normalizeSql('SELECT subject FROM billing_invoices WHERE invoice_id = ?1 LIMIT 1');
const SELECT_REFERRAL_BY_SUBJECT_SQL = normalizeSql(
  'SELECT payload_json FROM billing_referrals WHERE subject = ?1 LIMIT 1'
);
const SELECT_SNAPSHOT_BY_SUBJECT_SQL = normalizeSql(
  'SELECT payload_json FROM billing_snapshots WHERE subject = ?1 LIMIT 1'
);
const SELECT_ADMIN_ACCOUNTS_SQL = normalizeSql(
  'SELECT payload_json FROM billing_admin_accounts ORDER BY parent_account_ref'
);
const SELECT_ADMIN_INVOICES_SQL = normalizeSql('SELECT payload_json FROM billing_admin_invoices ORDER BY invoice_id');
const SELECT_ADMIN_DISPUTES_SQL = normalizeSql('SELECT payload_json FROM billing_admin_disputes ORDER BY dispute_id');
const SELECT_ADMIN_REFERRALS_SQL = normalizeSql(
  'SELECT payload_json FROM billing_admin_referrals ORDER BY referral_code'
);

const UPSERT_STATUS_SQL = normalizeSql('INSERT OR REPLACE INTO billing_status (subject, payload_json) VALUES (?1, ?2)');
const UPSERT_INVOICE_SQL = normalizeSql(
  'INSERT OR REPLACE INTO billing_invoices (subject, invoice_id, payload_json) VALUES (?1, ?2, ?3)'
);
const UPSERT_REFERRAL_SQL = normalizeSql(
  'INSERT OR REPLACE INTO billing_referrals (subject, payload_json) VALUES (?1, ?2)'
);
const UPSERT_SNAPSHOT_SQL = normalizeSql(
  'INSERT OR REPLACE INTO billing_snapshots (subject, payload_json) VALUES (?1, ?2)'
);
const UPSERT_ADMIN_ACCOUNT_SQL = normalizeSql(
  'INSERT OR REPLACE INTO billing_admin_accounts (parent_account_ref, payload_json) VALUES (?1, ?2)'
);
const UPSERT_ADMIN_INVOICE_SQL = normalizeSql(
  'INSERT OR REPLACE INTO billing_admin_invoices (invoice_id, payload_json) VALUES (?1, ?2)'
);
const UPSERT_ADMIN_DISPUTE_SQL = normalizeSql(
  'INSERT OR REPLACE INTO billing_admin_disputes (dispute_id, payload_json) VALUES (?1, ?2)'
);
const UPSERT_ADMIN_REFERRAL_SQL = normalizeSql(
  'INSERT OR REPLACE INTO billing_admin_referrals (referral_code, payload_json) VALUES (?1, ?2)'
);

const seedReadyByEnv = new WeakMap<Env, Promise<void>>();

interface PayloadJsonRow {
  payload_json: string;
}

interface RowCountRow {
  row_count: number | string;
}

interface InvoiceSubjectRow {
  subject: string;
}

export class BillingReadModelUnavailableError extends Error {
  readonly code = 'billing-read-model-manual-required';

  constructor(readonly scope: string) {
    super(`${this.code}:${scope}`);
    this.name = 'BillingReadModelUnavailableError';
  }
}

type CanonicalBillingSeatComposition = ReturnType<typeof BillingEntitlementSeatCompositionSchema.parse>;

function decodeCanonicalValue<T>(scope: string, decode: () => T): T {
  try {
    return decode();
  } catch (_error) {
    throw new BillingReadModelUnavailableError(`${scope}-invalid`);
  }
}

function payloadRecord(value: unknown, scope: string): Record<string, unknown> {
  if (typeof value !== 'object' || value === null || Array.isArray(value)) {
    throw new BillingReadModelUnavailableError(`${scope}-invalid`);
  }
  return value as Record<string, unknown>;
}

function decodeBillingLimit(value: unknown, scope: string): number {
  return decodeCanonicalValue(scope, () => PositiveBillingLimitSchema.parse(value));
}

function decodeBillingCount(value: unknown, scope: string): number {
  return decodeCanonicalValue(scope, () => NonNegativeBillingCountSchema.parse(value));
}

function decodeNonEmptyString(value: unknown, scope: string): string {
  if (typeof value !== 'string' || value.trim().length === 0) {
    throw new BillingReadModelUnavailableError(`${scope}-invalid`);
  }
  return value;
}

function decodeBoolean(value: unknown, scope: string): boolean {
  if (typeof value !== 'boolean') {
    throw new BillingReadModelUnavailableError(`${scope}-invalid`);
  }
  return value;
}

function decodeLiteral<T extends string>(value: unknown, allowed: ReadonlyArray<T>, scope: string): T {
  const decoded = decodeNonEmptyString(value, scope);
  if (!allowed.includes(decoded as T)) {
    throw new BillingReadModelUnavailableError(`${scope}-invalid`);
  }
  return decoded as T;
}

function decodeCanonicalLiteral<T extends string>(
  value: unknown,
  parse: () => string,
  allowed: ReadonlyArray<T>,
  scope: string
): T {
  const decoded = decodeCanonicalValue(scope, parse);
  if (!allowed.includes(decoded as T)) {
    throw new BillingReadModelUnavailableError(`${scope}-invalid`);
  }
  return decoded as T;
}

function decodeTimestamp(value: unknown, scope: string): string {
  const decoded = decodeCanonicalValue(scope, () => ParentTimestampSchema.parse(value));
  if (!Number.isFinite(Date.parse(decoded))) {
    throw new BillingReadModelUnavailableError(`${scope}-invalid`);
  }
  return decoded;
}

function decodePricingFeatureSummary(value: unknown, scope: string): PricingFeatureSummary {
  const record = payloadRecord(value, scope);
  return {
    code: decodeNonEmptyString(record.code, `${scope}-code`),
    label: decodeNonEmptyString(record.label, `${scope}-label`),
    included: decodeBoolean(record.included, `${scope}-included`),
    safetyCritical: decodeBoolean(record.safetyCritical, `${scope}-safety-critical`),
  };
}

function decodeArray<T>(value: unknown, decode: (entry: unknown, scope: string) => T, scope: string): ReadonlyArray<T> {
  if (!Array.isArray(value)) {
    throw new BillingReadModelUnavailableError(`${scope}-invalid`);
  }
  return value.map((entry, index) => decode(entry, `${scope}-${index}`));
}

function decodeFailureState(value: unknown, scope: string): BillingFailureStateSummary | null {
  if (value === null) {
    return null;
  }
  const record = payloadRecord(value, scope);
  return {
    failureKind: decodeCanonicalValue(scope, () => BillingFailureKindSchema.parse(record.failureKind)),
    parentResolution: decodeCanonicalValue(scope, () => BillingParentResolutionSchema.parse(record.parentResolution)),
    retryAllowed: decodeBoolean(record.retryAllowed, `${scope}-retry-allowed`),
    retryAfter: record.retryAfter === null ? null : decodeTimestamp(record.retryAfter, `${scope}-retry-after`),
  };
}

function decodeStatusReferralSummary(value: unknown, scope: string): BillingStatusSummary['referralSummary'] {
  const record = payloadRecord(value, scope);
  return {
    referralCode: record.referralCode === null ? null : decodeNonEmptyString(record.referralCode, `${scope}-code`),
    availableCredits: decodeBillingCount(record.availableCredits, `${scope}-available-credits`),
    activeReferredParents: decodeBillingCount(record.activeReferredParents, `${scope}-active-referred-parents`),
    pendingInvites: decodeBillingCount(record.pendingInvites, `${scope}-pending-invites`),
    inviteLinkVisible: decodeBoolean(record.inviteLinkVisible, `${scope}-invite-link-visible`),
  };
}

function decodeManualInvoiceState(value: unknown, scope: string): BillingStatusSummary['manualInvoiceState'] {
  const record = payloadRecord(value, scope);
  return {
    visible: decodeBoolean(record.visible, `${scope}-visible`),
    invoiceState:
      record.invoiceState === null
        ? null
        : decodeLiteral(record.invoiceState, ['manual-support-required'] as const, `${scope}-invoice-state`),
  };
}

function decodeBillingSeatComposition(value: unknown, scope: string): BillingSeatCompositionSummary {
  const record = payloadRecord(value, scope);
  const composition = decodeCanonicalValue(scope, () =>
    BillingEntitlementSeatCompositionSchema.parse({
      baseChildDeviceLimit: record.baseIncludedSeats,
      activeReferralCredits: record.activeReferralCredits,
      paidExtraChildDeviceSeats: record.paidExtraSeats,
      effectiveChildDeviceLimit: record.effectiveLimit,
    })
  );

  return {
    baseIncludedSeats: composition.baseChildDeviceLimit,
    activeReferralCredits: composition.activeReferralCredits,
    paidExtraSeats: composition.paidExtraChildDeviceSeats,
    effectiveLimit: composition.effectiveChildDeviceLimit,
    availableDeviceSlots: decodeBillingCount(record.availableDeviceSlots, `${scope}-available-device-slots`),
  };
}

function decodePricingPlan(value: unknown, scope: string): PricingPlanSummary {
  const record = payloadRecord(value, scope);
  return {
    planId: decodeCanonicalValue(scope, () => BillingPlanIdSchema.parse(record.planId)),
    displayName: decodeNonEmptyString(record.displayName, `${scope}-display-name`),
    interval: decodeLiteral(record.interval, ['monthly', 'yearly'] as const, `${scope}-interval`),
    priceCents: decodeBillingCount(record.priceCents, `${scope}-price-cents`),
    currency: decodeLiteral(record.currency, ['USD'] as const, `${scope}-currency`),
    deviceLimit: decodeBillingLimit(record.deviceLimit, `${scope}-device-limit`),
    activeState: decodeCanonicalValue(scope, () => BillingPlanActiveStateSchema.parse(record.activeState)),
    featureSummary: decodeArray(record.featureSummary, decodePricingFeatureSummary, `${scope}-features`),
  };
}

function decodeBillingStatusSummary(value: unknown, scope: string, expectedSubject?: string): BillingStatusSummary {
  const record = payloadRecord(value, scope);
  const subject = decodeNonEmptyString(record.subject, `${scope}-subject`);
  if (expectedSubject !== undefined && subject !== expectedSubject) {
    throw new BillingReadModelUnavailableError(`${scope}-subject-mismatch`);
  }
  const plan = decodePricingPlan(record.plan, `${scope}-plan`);
  const deviceUsage = payloadRecord(record.deviceUsage, `${scope}-device-usage`);
  const seatComposition = decodeBillingSeatComposition(record.seatComposition, `${scope}-seat-composition`);
  const activeDevices = decodeBillingCount(deviceUsage.activeDevices, `${scope}-active-devices`);
  const trustedDevices = decodeBillingCount(deviceUsage.trustedDevices, `${scope}-trusted-devices`);
  const deviceUsageLimit = decodeBillingLimit(deviceUsage.limit, `${scope}-device-usage-limit`);
  const status = decodeLiteral(record.status, ['ok'] as const, `${scope}-status`);
  const environment = decodeNonEmptyString(record.environment, `${scope}-environment`);
  const authAdapterMode = decodeNonEmptyString(record.authAdapterMode, `${scope}-auth-adapter-mode`);
  const parentAccountRef = decodeNonEmptyString(record.parentAccountRef, `${scope}-parent-account-ref`);
  const familyRef = decodeNonEmptyString(record.familyRef, `${scope}-family-ref`);
  const accountStatus = decodeLiteral(
    record.accountStatus,
    ['trialing', 'active', 'grace', 'manual-review', 'unavailable'] as const,
    `${scope}-account-status`
  );
  const subscriptionStatus = decodeCanonicalLiteral(
    record.subscriptionStatus,
    () => BillingSubscriptionStatusSchema.parse(record.subscriptionStatus),
    ['active', 'grace', 'past-due'] as const,
    `${scope}-subscription-status`
  );
  const portalVisibleState = decodeLiteral(
    record.portalVisibleState,
    ['ready', 'degraded', 'stale', 'offline', 'manual-required'] as const,
    `${scope}-portal-visible-state`
  );
  const parentVisibleState = decodeLiteral(
    record.parentVisibleState,
    ['available', 'grace', 'stale', 'unavailable', 'manual-review'] as const,
    `${scope}-parent-visible-state`
  );
  const localSafetyBehavior = decodeCanonicalLiteral(
    record.localSafetyBehavior,
    () => BillingLocalSafetyBehaviorSchema.parse(record.localSafetyBehavior),
    ['unchanged', 'grace-with-local-safety', 'manual-review-with-local-safety'] as const,
    `${scope}-local-safety-behavior`
  );
  const childActivityCustody = decodeLiteral(
    record.childActivityCustody,
    ['not-included'] as const,
    `${scope}-child-activity-custody`
  );
  const evidenceExportAccess = decodeLiteral(
    record.evidenceExportAccess,
    ['retained'] as const,
    `${scope}-evidence-export-access`
  );
  const providerSecretCustody = decodeLiteral(
    record.providerSecretCustody,
    ['not-present'] as const,
    `${scope}-provider-secret-custody`
  );
  const providerMode = decodeLiteral(
    record.providerMode,
    ['stripe-hosted', 'manual-invoice'] as const,
    `${scope}-provider-mode`
  );
  const nextRenewalAt =
    record.nextRenewalAt === null ? null : decodeTimestamp(record.nextRenewalAt, `${scope}-next-renewal-at`);
  const referralSummary = decodeStatusReferralSummary(record.referralSummary, `${scope}-referral-summary`);
  const manualInvoiceState = decodeManualInvoiceState(record.manualInvoiceState, `${scope}-manual-invoice-state`);
  const source = decodeCanonicalLiteral(
    record.source,
    () => BillingEntitlementSourceSchema.parse(record.source),
    ['signed-local-snapshot', 'manual-admin-review'] as const,
    `${scope}-source`
  );
  const failureState = decodeFailureState(record.failureState, `${scope}-failure-state`);
  const warnings = decodeArray(record.warnings, decodeNonEmptyString, `${scope}-warnings`);
  const auditReference = decodeCanonicalValue(scope, () => BillingAuditReferenceSchema.parse(record.auditReference));
  const updatedAt = decodeTimestamp(record.updatedAt, `${scope}-updated-at`);

  if (plan.deviceLimit !== seatComposition.effectiveLimit || deviceUsageLimit !== seatComposition.effectiveLimit) {
    throw new BillingReadModelUnavailableError(`${scope}-limit-mismatch`);
  }
  if (trustedDevices > activeDevices || activeDevices > deviceUsageLimit) {
    throw new BillingReadModelUnavailableError(`${scope}-device-count-mismatch`);
  }
  if (seatComposition.availableDeviceSlots !== Math.max(deviceUsageLimit - activeDevices, 0)) {
    throw new BillingReadModelUnavailableError(`${scope}-available-device-slots-mismatch`);
  }
  const expectedPortalVisibleState = {
    available: 'ready',
    grace: 'degraded',
    stale: 'stale',
    unavailable: 'offline',
    'manual-review': 'manual-required',
  }[parentVisibleState];
  if (portalVisibleState !== expectedPortalVisibleState) {
    throw new BillingReadModelUnavailableError(`${scope}-portal-state-mismatch`);
  }
  if (
    source === 'manual-admin-review' &&
    (accountStatus !== 'manual-review' ||
      parentVisibleState !== 'manual-review' ||
      localSafetyBehavior !== 'manual-review-with-local-safety' ||
      failureState === null)
  ) {
    throw new BillingReadModelUnavailableError(`${scope}-manual-authority-mismatch`);
  }
  if (
    source === 'signed-local-snapshot' &&
    (accountStatus === 'manual-review' ||
      parentVisibleState === 'manual-review' ||
      localSafetyBehavior === 'manual-review-with-local-safety')
  ) {
    throw new BillingReadModelUnavailableError(`${scope}-signed-manual-state`);
  }
  if (subscriptionStatus === 'active' && failureState !== null) {
    throw new BillingReadModelUnavailableError(`${scope}-active-failure-state`);
  }
  if (subscriptionStatus !== 'active' && failureState === null) {
    throw new BillingReadModelUnavailableError(`${scope}-degraded-failure-state-missing`);
  }
  if (failureState?.failureKind === 'validation-failed') {
    throw new BillingReadModelUnavailableError(`${scope}-failed-authority`);
  }

  return {
    status,
    environment,
    authAdapterMode,
    parentAccountRef,
    familyRef,
    subject,
    accountStatus,
    subscriptionStatus,
    portalVisibleState,
    parentVisibleState,
    localSafetyBehavior,
    childActivityCustody,
    evidenceExportAccess,
    providerSecretCustody,
    providerMode,
    nextRenewalAt,
    plan,
    deviceUsage: {
      activeDevices,
      trustedDevices,
      limit: deviceUsageLimit,
    },
    seatComposition,
    referralSummary,
    manualInvoiceState,
    source,
    failureState,
    warnings,
    auditReference,
    updatedAt,
  };
}

function decodeBillingEntitlementSnapshot(
  value: unknown,
  scope: string,
  expectedSubject?: string
): BillingEntitlementSnapshotSummary {
  const record = payloadRecord(value, scope);
  const subject = decodeNonEmptyString(record.subject, `${scope}-subject`);
  if (expectedSubject !== undefined && subject !== expectedSubject) {
    throw new BillingReadModelUnavailableError(`${scope}-subject-mismatch`);
  }
  const deviceLimit = decodeBillingLimit(record.deviceLimit, `${scope}-device-limit`);
  const activeDevices = decodeBillingCount(record.activeDevices, `${scope}-active-devices`);
  const trustedDevices = decodeBillingCount(record.trustedDevices, `${scope}-trusted-devices`);
  const availableDeviceSlots = decodeBillingCount(record.availableDeviceSlots, `${scope}-available-device-slots`);
  if (trustedDevices > activeDevices || activeDevices > deviceLimit) {
    throw new BillingReadModelUnavailableError(`${scope}-device-count-mismatch`);
  }
  if (availableDeviceSlots !== Math.max(deviceLimit - activeDevices, 0)) {
    throw new BillingReadModelUnavailableError(`${scope}-available-device-slots-mismatch`);
  }
  const snapshot: BillingEntitlementSnapshotSummary = {
    snapshotId: decodeCanonicalValue(scope, () => BillingEntitlementSnapshotIdSchema.parse(record.snapshotId)),
    subject,
    parentAccountRef: decodeNonEmptyString(record.parentAccountRef, `${scope}-parent-account-ref`),
    familyRef: decodeNonEmptyString(record.familyRef, `${scope}-family-ref`),
    planId: decodeCanonicalValue(scope, () => BillingPlanIdSchema.parse(record.planId)),
    subscriptionStatus: decodeCanonicalLiteral(
      record.subscriptionStatus,
      () => BillingSubscriptionStatusSchema.parse(record.subscriptionStatus),
      ['active', 'grace', 'past-due'] as const,
      `${scope}-subscription-status`
    ),
    source: decodeCanonicalLiteral(
      record.source,
      () => BillingEntitlementSourceSchema.parse(record.source),
      ['signed-local-snapshot', 'manual-admin-review'] as const,
      `${scope}-source`
    ),
    signatureState: decodeCanonicalLiteral(
      record.signatureState,
      () => BillingSignatureStateSchema.parse(record.signatureState),
      ['signed', 'manual-required'] as const,
      `${scope}-signature-state`
    ),
    signedAt: decodeTimestamp(record.signedAt, `${scope}-signed-at`),
    deviceLimit,
    activeDevices,
    trustedDevices,
    availableDeviceSlots,
    parentVisibleState: decodeCanonicalLiteral(
      record.parentVisibleState,
      () => BillingParentVisibleStateSchema.parse(record.parentVisibleState),
      ['available', 'grace', 'manual-review'] as const,
      `${scope}-parent-visible-state`
    ),
    localSafetyBehavior: decodeCanonicalLiteral(
      record.localSafetyBehavior,
      () => BillingLocalSafetyBehaviorSchema.parse(record.localSafetyBehavior),
      ['unchanged', 'grace-with-local-safety', 'manual-review-with-local-safety'] as const,
      `${scope}-local-safety-behavior`
    ),
    failureState: decodeFailureState(record.failureState, `${scope}-failure-state`),
    auditReference: decodeCanonicalValue(scope, () => BillingAuditReferenceSchema.parse(record.auditReference)),
  };
  const signedAuthority = snapshot.source === 'signed-local-snapshot' && snapshot.signatureState === 'signed';
  const manualAuthority = snapshot.source === 'manual-admin-review' && snapshot.signatureState === 'manual-required';
  if (!signedAuthority && !manualAuthority) {
    throw new BillingReadModelUnavailableError(`${scope}-authority-mismatch`);
  }
  if (
    manualAuthority &&
    (snapshot.parentVisibleState !== 'manual-review' ||
      snapshot.localSafetyBehavior !== 'manual-review-with-local-safety' ||
      snapshot.failureState === null)
  ) {
    throw new BillingReadModelUnavailableError(`${scope}-manual-authority-mismatch`);
  }
  if (
    signedAuthority &&
    (snapshot.parentVisibleState === 'manual-review' ||
      snapshot.localSafetyBehavior === 'manual-review-with-local-safety')
  ) {
    throw new BillingReadModelUnavailableError(`${scope}-signed-manual-state`);
  }
  if (
    (snapshot.subscriptionStatus === 'active' &&
      (snapshot.parentVisibleState !== 'available' || snapshot.localSafetyBehavior !== 'unchanged')) ||
    (snapshot.subscriptionStatus === 'grace' &&
      (snapshot.parentVisibleState !== 'grace' || snapshot.localSafetyBehavior !== 'grace-with-local-safety')) ||
    (snapshot.subscriptionStatus === 'past-due' &&
      signedAuthority &&
      (snapshot.parentVisibleState !== 'grace' || snapshot.localSafetyBehavior !== 'grace-with-local-safety'))
  ) {
    throw new BillingReadModelUnavailableError(`${scope}-state-mismatch`);
  }
  if (snapshot.subscriptionStatus === 'active' && snapshot.failureState !== null) {
    throw new BillingReadModelUnavailableError(`${scope}-active-failure-state`);
  }
  if (snapshot.subscriptionStatus !== 'active' && snapshot.failureState === null) {
    throw new BillingReadModelUnavailableError(`${scope}-degraded-failure-state-missing`);
  }
  if (snapshot.failureState?.failureKind === 'validation-failed') {
    throw new BillingReadModelUnavailableError(`${scope}-failed-authority`);
  }
  return snapshot;
}

function snapshotCanYieldAllowed(
  snapshot: BillingEntitlementSnapshotSummary,
  requestedDeviceAlreadyTrusted: boolean
): boolean {
  const signedActiveAuthority =
    snapshot.subscriptionStatus === 'active' &&
    snapshot.source === 'signed-local-snapshot' &&
    snapshot.signatureState === 'signed' &&
    snapshot.parentVisibleState === 'available' &&
    snapshot.localSafetyBehavior === 'unchanged' &&
    snapshot.failureState === null;
  const signedGraceAuthority =
    requestedDeviceAlreadyTrusted &&
    snapshot.subscriptionStatus === 'grace' &&
    snapshot.source === 'signed-local-snapshot' &&
    snapshot.signatureState === 'signed' &&
    snapshot.parentVisibleState === 'grace' &&
    snapshot.localSafetyBehavior === 'grace-with-local-safety' &&
    snapshot.failureState?.failureKind === 'payment-required';
  return signedActiveAuthority || signedGraceAuthority;
}

function decodeBillingStatePair(
  status: BillingStatusSummary,
  snapshot: BillingEntitlementSnapshotSummary,
  scope: string
): { status: BillingStatusSummary; snapshot: BillingEntitlementSnapshotSummary } {
  const decodedStatus = decodeBillingStatusSummary(status, `${scope}-status`, status.subject);
  const decodedSnapshot = decodeBillingEntitlementSnapshot(snapshot, `${scope}-snapshot`, status.subject);
  if (
    decodedStatus.parentAccountRef !== decodedSnapshot.parentAccountRef ||
    decodedStatus.familyRef !== decodedSnapshot.familyRef ||
    decodedStatus.plan.planId !== decodedSnapshot.planId ||
    decodedStatus.plan.deviceLimit !== decodedSnapshot.deviceLimit ||
    decodedStatus.deviceUsage.activeDevices !== decodedSnapshot.activeDevices ||
    decodedStatus.deviceUsage.trustedDevices !== decodedSnapshot.trustedDevices ||
    decodedStatus.subscriptionStatus !== decodedSnapshot.subscriptionStatus ||
    decodedStatus.source !== decodedSnapshot.source ||
    decodedStatus.parentVisibleState !== decodedSnapshot.parentVisibleState ||
    decodedStatus.localSafetyBehavior !== decodedSnapshot.localSafetyBehavior ||
    JSON.stringify(decodedStatus.failureState) !== JSON.stringify(decodedSnapshot.failureState)
  ) {
    throw new BillingReadModelUnavailableError(`${scope}-mismatch`);
  }
  return { status: decodedStatus, snapshot: decodedSnapshot };
}

function decodeBillingReferralSummary(value: unknown, scope: string): BillingReferralSummary {
  const decoded = decodeCanonicalValue(scope, () => BillingReferralSummarySchema.parse(value));
  return {
    ...decoded,
    availableCredits: decodeBillingCount(decoded.availableCredits, `${scope}-available-credits`),
    activeReferredParents: decodeBillingCount(decoded.activeReferredParents, `${scope}-active-referred-parents`),
    pendingInvites: decodeBillingCount(decoded.pendingInvites, `${scope}-pending-invites`),
  };
}

function remainingBillingSlots(limit: unknown, activeDevices: unknown, scope: string): number {
  const decodedLimit = decodeBillingLimit(limit, `${scope}-limit`);
  const decodedActiveDevices = decodeBillingCount(activeDevices, `${scope}-active-devices`);
  return decodedLimit >= decodedActiveDevices ? decodedLimit - decodedActiveDevices : 0;
}

function incrementBillingCount(value: unknown, scope: string): number {
  const decodedValue = decodeBillingCount(value, `${scope}-current`);
  return decodeBillingCount(decodedValue + 1, scope);
}

function sumBillingCounts(left: unknown, right: unknown, scope: string): number {
  const decodedLeft = decodeBillingCount(left, `${scope}-left`);
  const decodedRight = decodeBillingCount(right, `${scope}-right`);
  return decodeBillingCount(decodedLeft + decodedRight, scope);
}

function targetPlanSeatComposition(
  status: BillingStatusSummary,
  targetPlanDeviceLimit: number,
  scope: string
): CanonicalBillingSeatComposition {
  const baseChildDeviceLimit = decodeBillingCount(status.seatComposition.baseIncludedSeats, `${scope}-base-seats`);
  const activeReferralCredits = decodeBillingCount(
    status.seatComposition.activeReferralCredits,
    `${scope}-referral-credits`
  );
  if (
    targetPlanDeviceLimit < baseChildDeviceLimit ||
    targetPlanDeviceLimit - baseChildDeviceLimit < activeReferralCredits
  ) {
    throw new BillingReadModelUnavailableError(`${scope}-referral-seat-over-limit`);
  }
  const paidExtraChildDeviceSeats = targetPlanDeviceLimit - baseChildDeviceLimit - activeReferralCredits;
  return decodeCanonicalValue(scope, () =>
    BillingEntitlementSeatCompositionSchema.parse({
      baseChildDeviceLimit,
      activeReferralCredits,
      paidExtraChildDeviceSeats,
      effectiveChildDeviceLimit: targetPlanDeviceLimit,
    })
  );
}

function isLocalFixtureEnvironment(env: Env): boolean {
  const authAdapterMode = env.AUTH_ADAPTER_MODE?.trim();
  return (
    (env.ENVIRONMENT === 'local' || env.ENVIRONMENT === 'test' || env.ENVIRONMENT === 'development') &&
    authAdapterMode === 'local-safe-fixture'
  );
}

function requireProductionBinding(env: Env, binding: 'BILLING_D1' | 'BILLING_CONFIG_KV' | 'BILLING_AUDIT_R2'): void {
  if (!isLocalFixtureEnvironment(env) && !env[binding]) {
    throw new BillingReadModelUnavailableError(`${binding.toLowerCase()}-binding-missing`);
  }
}

function requireProductionRecord<T>(env: Env, record: T | null, scope: string): T | null {
  if (record !== null || isLocalFixtureEnvironment(env)) {
    return record;
  }
  throw new BillingReadModelUnavailableError(scope);
}

export interface BillingBindingSeedPatch {
  pricingPlans?: ReadonlyArray<PricingPlanSummary>;
  statusBySubject?: Readonly<Record<string, BillingStatusSummary>>;
  invoicesBySubject?: Readonly<Record<string, ReadonlyArray<BillingInvoiceSummary>>>;
  referralsBySubject?: Readonly<Record<string, BillingReferralSummary>>;
  snapshotsBySubject?: Readonly<Record<string, BillingEntitlementSnapshotSummary>>;
  adminAccounts?: ReadonlyArray<AdminBillingAccountSummary>;
  adminInvoices?: ReadonlyArray<AdminBillingInvoiceSummary>;
  adminDisputes?: ReadonlyArray<AdminBillingDisputeSummary>;
  adminReferrals?: ReadonlyArray<AdminBillingReferralSummary>;
  auditEvents?: ReadonlyArray<BillingAuditEventSummary>;
}

export interface LocalSeedSnapshot extends BillingBindingSeedPatch {
  generatedAt: string;
}

export interface LocalAnalyticsWrite {
  indexes?: ReadonlyArray<string>;
  blobs?: ReadonlyArray<string>;
  doubles?: ReadonlyArray<number>;
}

export interface LocalBillingBindingState {
  applySeedPatch(patch: BillingBindingSeedPatch): Promise<void>;
  replaceSeed(patch: BillingBindingSeedPatch): void;
  getAnalyticsWrites(): ReadonlyArray<LocalAnalyticsWrite>;
  getTouchCount(counterKey: string): number;
}

export type BillingStateMutation =
  | {
      kind: 'hosted-session';
      subject: string;
      requestId: string;
      sessionKind: 'checkout-session-create' | 'billing-portal-session-create';
      auditReference: string;
      actorRole: 'parent' | 'guardian';
    }
  | {
      kind: 'change-plan';
      subject: string;
      requestId: string;
      targetPlanId: string;
      auditReference: string;
    }
  | {
      kind: 'cancel';
      subject: string;
      requestId: string;
      cancellationState: BillingCancellationSummary['cancellationState'];
      auditReference: string;
    }
  | {
      kind: 'referral-invite';
      subject: string;
      requestId: string;
      invitedIdentifier: string;
      referralCode: string;
      auditReference: string;
      actorRole: 'parent' | 'guardian';
    }
  | {
      kind: 'manual-invoice';
      subject: string;
      requestId: string;
      region: string;
      auditReference: string;
      actorRole: 'support' | 'admin';
    }
  | {
      kind: 'admin-refund';
      subject: string;
      requestId: string;
      invoiceId: string;
      refundState: 'refund-requested' | 'refund-settled';
      amountCents: number;
      auditReference: string;
      actorRole: 'support' | 'admin';
    }
  | {
      kind: 'reconciliation';
      subject: string;
      requestId: string;
      auditReference: string;
      actorRole: 'support' | 'admin' | 'system';
    }
  | {
      kind: 'provider-webhook';
      provider: string;
      subject: string;
      eventId: string;
      eventType: string;
      disputeId?: string | null;
      invoiceId?: string | null;
    };

interface LocalBillingD1State {
  statusBySubject: Map<string, BillingStatusSummary>;
  invoicesBySubject: Map<string, ReadonlyArray<BillingInvoiceSummary>>;
  referralsBySubject: Map<string, BillingReferralSummary>;
  snapshotsBySubject: Map<string, BillingEntitlementSnapshotSummary>;
  adminAccounts: ReadonlyArray<AdminBillingAccountSummary>;
  adminInvoices: ReadonlyArray<AdminBillingInvoiceSummary>;
  adminDisputes: ReadonlyArray<AdminBillingDisputeSummary>;
  adminReferrals: ReadonlyArray<AdminBillingReferralSummary>;
}

function cloneJsonValue<T>(value: T): T {
  return JSON.parse(JSON.stringify(value)) as T;
}

function asReadonlyArray<T>(value: ReadonlyArray<T>): ReadonlyArray<T> {
  return cloneJsonValue(value);
}

function mapFromRecord<T>(value: Readonly<Record<string, T>>): Map<string, T> {
  return new Map(Object.entries(value).map(([key, entry]) => [key, cloneJsonValue(entry)]));
}

function withAddedWarning(warnings: ReadonlyArray<string>, nextWarning: string): ReadonlyArray<string> {
  return warnings.includes(nextWarning) ? warnings : [...warnings, nextWarning];
}

function parsePayload<T>(payloadJson: string): T {
  return JSON.parse(payloadJson) as T;
}

function parseUnknownPayload(payloadJson: string, scope: string): unknown {
  try {
    return JSON.parse(payloadJson);
  } catch (_error) {
    throw new BillingReadModelUnavailableError(`${scope}-invalid-json`);
  }
}

function parseUnknownPayloadRow(row: PayloadJsonRow | null, scope: string): unknown | null {
  return row === null ? null : parseUnknownPayload(row.payload_json, scope);
}

function parsePayloadRows<T>(rows: ReadonlyArray<PayloadJsonRow>): ReadonlyArray<T> {
  return rows.map((row) => parsePayload<T>(row.payload_json));
}

function subjectRecord<T>(
  subjects: ReadonlyArray<string>,
  builder: (subject: string) => T
): Readonly<Record<string, T>> {
  return Object.fromEntries(subjects.map((subject) => [subject, builder(subject)]));
}

function replaceByKey<T>(current: ReadonlyArray<T>, nextEntry: T, keyFor: (entry: T) => string): ReadonlyArray<T> {
  const nextKey = keyFor(nextEntry);
  const filtered = current.filter((entry) => keyFor(entry) !== nextKey);
  return [...filtered, cloneJsonValue(nextEntry)];
}

class LocalKvNamespace implements KVNamespace {
  private readonly values = new Map<string, string>();

  getRaw(key: string): string | null {
    return this.values.get(key) ?? null;
  }

  setRaw(key: string, value: string): void {
    this.values.set(key, value);
  }

  async get(key: string, type?: 'text' | 'json'): Promise<unknown> {
    const value = this.values.get(key);
    if (value === undefined) {
      return null;
    }
    if (type === 'json') {
      return JSON.parse(value);
    }
    return value;
  }

  async put(key: string, value: string): Promise<void> {
    this.values.set(key, value);
  }
}

class LocalR2ObjectBody {
  constructor(private readonly value: string) {}

  async text(): Promise<string> {
    return this.value;
  }

  async json<T>(): Promise<T> {
    return JSON.parse(this.value) as T;
  }
}

class LocalR2Bucket implements R2Bucket {
  private readonly objects = new Map<string, string>();

  setRaw(key: string, value: string): void {
    this.objects.set(key, value);
  }

  async get(key: string): Promise<LocalR2ObjectBody | null> {
    const value = this.objects.get(key);
    return value === undefined ? null : new LocalR2ObjectBody(value);
  }

  async put(key: string, value: string): Promise<void> {
    this.objects.set(key, value);
  }
}

class LocalAnalyticsDataset implements AnalyticsEngineDataset {
  private readonly writes: LocalAnalyticsWrite[] = [];

  writeDataPoint(data: LocalAnalyticsWrite): void {
    this.writes.push(cloneJsonValue(data));
  }

  getWrites(): ReadonlyArray<LocalAnalyticsWrite> {
    return this.writes.map((entry) => cloneJsonValue(entry));
  }
}

class LocalD1Statement implements D1PreparedStatement {
  private readonly normalizedQuery: string;

  constructor(
    private readonly state: LocalBillingD1State,
    query: string,
    private readonly values: ReadonlyArray<unknown> = []
  ) {
    this.normalizedQuery = normalizeSql(query);
  }

  bind(...values: ReadonlyArray<unknown>): D1PreparedStatement {
    return new LocalD1Statement(this.state, this.normalizedQuery, values);
  }

  async first<T>(): Promise<T | null> {
    const results = this.resolveSelectedRows<T>();
    return results[0] ?? null;
  }

  async all<T>(): Promise<{ results: ReadonlyArray<T>; success: true }> {
    return {
      results: this.resolveSelectedRows<T>(),
      success: true,
    };
  }

  async run(): Promise<{ results: ReadonlyArray<never>; success: true }> {
    this.executeMutation();
    return {
      results: [],
      success: true,
    };
  }

  private resolveSelectedRows<T>(): ReadonlyArray<T> {
    switch (this.normalizedQuery) {
      case SELECT_STATUS_ROW_COUNT_SQL:
        return [{ row_count: this.state.statusBySubject.size } as T];
      case SELECT_STATUS_BY_SUBJECT_SQL: {
        const subject = String(this.values[0] ?? '');
        const status = this.state.statusBySubject.get(subject);
        return status ? [{ payload_json: JSON.stringify(status) } as T] : [];
      }
      case SELECT_INVOICES_BY_SUBJECT_SQL: {
        const subject = String(this.values[0] ?? '');
        return (this.state.invoicesBySubject.get(subject) ?? []).map(
          (invoice) => ({ payload_json: JSON.stringify(invoice) }) as T
        );
      }
      case SELECT_REFERRAL_BY_SUBJECT_SQL: {
        const subject = String(this.values[0] ?? '');
        const referral = this.state.referralsBySubject.get(subject);
        return referral ? [{ payload_json: JSON.stringify(referral) } as T] : [];
      }
      case SELECT_SNAPSHOT_BY_SUBJECT_SQL: {
        const subject = String(this.values[0] ?? '');
        const snapshot = this.state.snapshotsBySubject.get(subject);
        return snapshot ? [{ payload_json: JSON.stringify(snapshot) } as T] : [];
      }
      case SELECT_ADMIN_ACCOUNTS_SQL:
        return this.state.adminAccounts.map((row) => ({ payload_json: JSON.stringify(row) }) as T);
      case SELECT_ADMIN_INVOICES_SQL:
        return this.state.adminInvoices.map((row) => ({ payload_json: JSON.stringify(row) }) as T);
      case SELECT_ADMIN_DISPUTES_SQL:
        return this.state.adminDisputes.map((row) => ({ payload_json: JSON.stringify(row) }) as T);
      case SELECT_ADMIN_REFERRALS_SQL:
        return this.state.adminReferrals.map((row) => ({ payload_json: JSON.stringify(row) }) as T);
      default:
        return [];
    }
  }

  private executeMutation(): void {
    switch (this.normalizedQuery) {
      case UPSERT_STATUS_SQL: {
        const subject = String(this.values[0] ?? '');
        const payloadJson = String(this.values[1] ?? '{}');
        this.state.statusBySubject.set(
          subject,
          decodeBillingStatusSummary(
            parseUnknownPayload(payloadJson, `billing-status:${subject}`),
            `billing-status:${subject}`,
            subject
          )
        );
        return;
      }
      case UPSERT_INVOICE_SQL: {
        const subject = String(this.values[0] ?? '');
        const payloadJson = String(this.values[2] ?? '{}');
        const invoice = parsePayload<BillingInvoiceSummary>(payloadJson);
        const current = this.state.invoicesBySubject.get(subject) ?? [];
        this.state.invoicesBySubject.set(
          subject,
          replaceByKey(current, invoice, (entry) => entry.invoiceId)
        );
        return;
      }
      case UPSERT_REFERRAL_SQL: {
        const subject = String(this.values[0] ?? '');
        const payloadJson = String(this.values[1] ?? '{}');
        this.state.referralsBySubject.set(
          subject,
          decodeBillingReferralSummary(
            parseUnknownPayload(payloadJson, `billing-referral:${subject}`),
            `billing-referral:${subject}`
          )
        );
        return;
      }
      case UPSERT_SNAPSHOT_SQL: {
        const subject = String(this.values[0] ?? '');
        const payloadJson = String(this.values[1] ?? '{}');
        this.state.snapshotsBySubject.set(
          subject,
          decodeBillingEntitlementSnapshot(
            parseUnknownPayload(payloadJson, `billing-entitlement-snapshot:${subject}`),
            `billing-entitlement-snapshot:${subject}`,
            subject
          )
        );
        return;
      }
      case UPSERT_ADMIN_ACCOUNT_SQL: {
        const payloadJson = String(this.values[1] ?? '{}');
        const nextRow = parsePayload<AdminBillingAccountSummary>(payloadJson);
        this.state.adminAccounts = replaceByKey(this.state.adminAccounts, nextRow, (entry) => entry.parentAccountRef);
        return;
      }
      case UPSERT_ADMIN_INVOICE_SQL: {
        const payloadJson = String(this.values[1] ?? '{}');
        const nextRow = parsePayload<AdminBillingInvoiceSummary>(payloadJson);
        this.state.adminInvoices = replaceByKey(this.state.adminInvoices, nextRow, (entry) => entry.invoiceId);
        return;
      }
      case UPSERT_ADMIN_DISPUTE_SQL: {
        const payloadJson = String(this.values[1] ?? '{}');
        const nextRow = parsePayload<AdminBillingDisputeSummary>(payloadJson);
        this.state.adminDisputes = replaceByKey(this.state.adminDisputes, nextRow, (entry) => entry.disputeId);
        return;
      }
      case UPSERT_ADMIN_REFERRAL_SQL: {
        const payloadJson = String(this.values[1] ?? '{}');
        const nextRow = parsePayload<AdminBillingReferralSummary>(payloadJson);
        this.state.adminReferrals = replaceByKey(this.state.adminReferrals, nextRow, (entry) => entry.referralCode);
        return;
      }
      default:
        return;
    }
  }
}

class LocalBillingD1Database implements D1Database {
  private readonly state: LocalBillingD1State = {
    statusBySubject: new Map<string, BillingStatusSummary>(),
    invoicesBySubject: new Map<string, ReadonlyArray<BillingInvoiceSummary>>(),
    referralsBySubject: new Map<string, BillingReferralSummary>(),
    snapshotsBySubject: new Map<string, BillingEntitlementSnapshotSummary>(),
    adminAccounts: [],
    adminInvoices: [],
    adminDisputes: [],
    adminReferrals: [],
  };

  prepare(query: string): D1PreparedStatement {
    return new LocalD1Statement(this.state, query);
  }

  async batch(
    statements: ReadonlyArray<D1PreparedStatement>
  ): Promise<ReadonlyArray<{ results: ReadonlyArray<unknown>; success: true }>> {
    const backup = {
      statusBySubject: new Map(this.state.statusBySubject),
      invoicesBySubject: new Map(this.state.invoicesBySubject),
      referralsBySubject: new Map(this.state.referralsBySubject),
      snapshotsBySubject: new Map(this.state.snapshotsBySubject),
      adminAccounts: this.state.adminAccounts,
      adminInvoices: this.state.adminInvoices,
      adminDisputes: this.state.adminDisputes,
      adminReferrals: this.state.adminReferrals,
    };
    const results: Array<{ results: ReadonlyArray<unknown>; success: true }> = [];
    try {
      for (const statement of statements) {
        results.push(await statement.run());
      }
      return results;
    } catch (error) {
      this.state.statusBySubject = backup.statusBySubject;
      this.state.invoicesBySubject = backup.invoicesBySubject;
      this.state.referralsBySubject = backup.referralsBySubject;
      this.state.snapshotsBySubject = backup.snapshotsBySubject;
      this.state.adminAccounts = backup.adminAccounts;
      this.state.adminInvoices = backup.adminInvoices;
      this.state.adminDisputes = backup.adminDisputes;
      this.state.adminReferrals = backup.adminReferrals;
      throw error;
    }
  }

  async exec(): Promise<{ count: number; duration: number }> {
    return {
      count: 0,
      duration: 0,
    };
  }

  replaceSeed(patch: BillingBindingSeedPatch): void {
    const decodedStatuses = patch.statusBySubject
      ? new Map(
          Object.entries(patch.statusBySubject).map(([subject, entry]) => [
            subject,
            decodeBillingStatusSummary(entry, `billing-status-seed:${subject}`, subject),
          ])
        )
      : null;
    const decodedSnapshots = patch.snapshotsBySubject
      ? new Map(
          Object.entries(patch.snapshotsBySubject).map(([subject, entry]) => [
            subject,
            decodeBillingEntitlementSnapshot(entry, `billing-entitlement-snapshot-seed:${subject}`, subject),
          ])
        )
      : null;
    if (decodedStatuses && decodedSnapshots) {
      for (const [subject, status] of decodedStatuses) {
        const snapshot = decodedSnapshots.get(subject);
        if (snapshot) {
          decodeBillingStatePair(status, snapshot, `billing-state-seed:${subject}`);
        }
      }
    }
    if (decodedStatuses) {
      this.state.statusBySubject = decodedStatuses;
    }
    if (patch.invoicesBySubject) {
      this.state.invoicesBySubject = mapFromRecord(patch.invoicesBySubject);
    }
    if (patch.referralsBySubject) {
      this.state.referralsBySubject = mapFromRecord(patch.referralsBySubject);
    }
    if (decodedSnapshots) {
      this.state.snapshotsBySubject = decodedSnapshots;
    }
    if (patch.adminAccounts) {
      this.state.adminAccounts = asReadonlyArray(patch.adminAccounts);
    }
    if (patch.adminInvoices) {
      this.state.adminInvoices = asReadonlyArray(patch.adminInvoices);
    }
    if (patch.adminDisputes) {
      this.state.adminDisputes = asReadonlyArray(patch.adminDisputes);
    }
    if (patch.adminReferrals) {
      this.state.adminReferrals = asReadonlyArray(patch.adminReferrals);
    }
  }
}

export function buildDefaultBillingBindingSeed(env: Env): BillingBindingSeedPatch {
  if (!isLocalFixtureEnvironment(env)) {
    throw new BillingReadModelUnavailableError('local-fixture-environment-required');
  }
  const subjects = Array.from(DEFAULT_BILLING_SUBJECTS);
  return {
    pricingPlans: asReadonlyArray(LOCAL_PRICING_PLANS),
    statusBySubject: subjectRecord(subjects, (subject) => buildBillingStatusSummary(subject, env)),
    invoicesBySubject: subjectRecord(subjects, (subject) => buildBillingInvoices(subject)),
    referralsBySubject: subjectRecord(subjects, (subject) => buildBillingReferralSummary(subject)),
    snapshotsBySubject: subjectRecord(subjects, (subject) => buildEntitlementSnapshot(subject)),
    adminAccounts: listAdminBillingAccounts(null),
    adminInvoices: listAdminBillingInvoices(null),
    adminDisputes: listAdminBillingDisputes(null),
    adminReferrals: listAdminBillingReferrals(null),
    auditEvents: listBillingAuditEvents(null),
  };
}

export function buildLocalSeedSnapshot(env: Env): LocalSeedSnapshot {
  return {
    generatedAt: GENERATED_AT,
    ...buildDefaultBillingBindingSeed(env),
  };
}

export function createLocalBillingBindings(): {
  BILLING_D1: D1Database;
  BILLING_CONFIG_KV: KVNamespace;
  BILLING_RATE_LIMIT_KV: KVNamespace;
  BILLING_AUDIT_R2: R2Bucket;
  ANALYTICS: AnalyticsEngineDataset;
  state: LocalBillingBindingState;
} {
  const d1 = new LocalBillingD1Database();
  const configKv = new LocalKvNamespace();
  const rateLimitKv = new LocalKvNamespace();
  const auditR2 = new LocalR2Bucket();
  const analytics = new LocalAnalyticsDataset();

  const replaceSeed = (patch: BillingBindingSeedPatch): void => {
    const pricingPlans = patch.pricingPlans?.map((plan, index) =>
      decodePricingPlan(plan, `billing-pricing-plan-seed-${index}`)
    );
    d1.replaceSeed(patch);
    if (pricingPlans) {
      configKv.setRaw(PRICING_PLANS_KEY, JSON.stringify(pricingPlans));
    }
    if (patch.auditEvents) {
      auditR2.setRaw(AUDIT_EVENTS_KEY, JSON.stringify(patch.auditEvents));
    }
  };

  return {
    BILLING_D1: d1,
    BILLING_CONFIG_KV: configKv,
    BILLING_RATE_LIMIT_KV: rateLimitKv,
    BILLING_AUDIT_R2: auditR2,
    ANALYTICS: analytics,
    state: {
      replaceSeed,
      async applySeedPatch(patch: BillingBindingSeedPatch): Promise<void> {
        replaceSeed(patch);
      },
      getAnalyticsWrites(): ReadonlyArray<LocalAnalyticsWrite> {
        return analytics.getWrites();
      },
      getTouchCount(counterKey: string): number {
        const raw = rateLimitKv.getRaw(`${TOUCH_KEY_PREFIX}${counterKey}`);
        return decodeTelemetryCounter(raw ?? '0', `billing-touch:${counterKey}`);
      },
    },
  };
}

function decodeTelemetryCounter(value: unknown, scope: string): number {
  if (typeof value === 'string' && value.trim().length === 0) {
    throw new BillingReadModelUnavailableError(`${scope}-invalid`);
  }
  const decoded = typeof value === 'string' ? Number(value) : value;
  return decodeBillingCount(decoded, scope);
}

async function incrementTouchCounter(env: Env, counterKey: string): Promise<void> {
  const fullKey = `${TOUCH_KEY_PREFIX}${counterKey}`;
  const current = decodeTelemetryCounter(
    (await env.BILLING_RATE_LIMIT_KV?.get(fullKey)) ?? '0',
    `billing-touch:${counterKey}`
  );
  const next = decodeBillingCount(current + 1, `billing-touch:${counterKey}-next`);
  await env.BILLING_RATE_LIMIT_KV?.put(fullKey, String(next));
}

async function recordBindingRead(env: Env, counterKey: string, subject: string | null): Promise<void> {
  await incrementTouchCounter(env, counterKey);
  env.ANALYTICS?.writeDataPoint({
    indexes: [counterKey],
    blobs: subject ? [subject] : [],
    doubles: [1],
  });
}

async function d1First<T>(
  database: D1Database | undefined,
  query: string,
  ...values: ReadonlyArray<unknown>
): Promise<T | null> {
  if (!database) {
    return null;
  }
  return database
    .prepare(query)
    .bind(...values)
    .first<T>();
}

async function d1All<T>(
  database: D1Database | undefined,
  query: string,
  ...values: ReadonlyArray<unknown>
): Promise<ReadonlyArray<T>> {
  if (!database) {
    return [];
  }
  const result = await database
    .prepare(query)
    .bind(...values)
    .all<T>();
  return result.results;
}

async function upsertBillingStatePair(
  env: Env,
  status: BillingStatusSummary,
  snapshot: BillingEntitlementSnapshotSummary
): Promise<void> {
  const { status: decodedStatus, snapshot: decodedSnapshot } = decodeBillingStatePair(
    status,
    snapshot,
    `billing-state-write:${status.subject}`
  );
  const database = env.BILLING_D1;
  if (!database) {
    throw new BillingReadModelUnavailableError('billing-d1-binding-missing');
  }
  await database.batch([
    database.prepare(UPSERT_STATUS_SQL).bind(decodedStatus.subject, JSON.stringify(decodedStatus)),
    database.prepare(UPSERT_SNAPSHOT_SQL).bind(decodedSnapshot.subject, JSON.stringify(decodedSnapshot)),
  ]);
}

async function upsertAdminBillingAccountSummary(env: Env, account: AdminBillingAccountSummary): Promise<void> {
  await env.BILLING_D1?.prepare(UPSERT_ADMIN_ACCOUNT_SQL).bind(account.parentAccountRef, JSON.stringify(account)).run();
}

async function upsertBillingInvoiceSummary(env: Env, subject: string, invoice: BillingInvoiceSummary): Promise<void> {
  await env.BILLING_D1?.prepare(UPSERT_INVOICE_SQL).bind(subject, invoice.invoiceId, JSON.stringify(invoice)).run();
}

async function upsertAdminBillingInvoiceSummary(env: Env, invoice: AdminBillingInvoiceSummary): Promise<void> {
  await env.BILLING_D1?.prepare(UPSERT_ADMIN_INVOICE_SQL).bind(invoice.invoiceId, JSON.stringify(invoice)).run();
}

async function upsertAdminBillingDisputeSummary(env: Env, dispute: AdminBillingDisputeSummary): Promise<void> {
  await env.BILLING_D1?.prepare(UPSERT_ADMIN_DISPUTE_SQL).bind(dispute.disputeId, JSON.stringify(dispute)).run();
}

async function upsertBillingReferralSummary(env: Env, referral: BillingReferralSummary): Promise<void> {
  await env.BILLING_D1?.prepare(UPSERT_REFERRAL_SQL).bind(referral.subject, JSON.stringify(referral)).run();
}

async function upsertAdminBillingReferralSummary(env: Env, referral: AdminBillingReferralSummary): Promise<void> {
  await env.BILLING_D1?.prepare(UPSERT_ADMIN_REFERRAL_SQL).bind(referral.referralCode, JSON.stringify(referral)).run();
}

async function readStoredAuditEvents(env: Env): Promise<ReadonlyArray<BillingAuditEventSummary>> {
  const object = await env.BILLING_AUDIT_R2?.get(AUDIT_EVENTS_KEY);
  if (!object) {
    return [];
  }
  return ((await object.json<ReadonlyArray<BillingAuditEventSummary>>()) ?? []).map((entry) => cloneJsonValue(entry));
}

async function appendBillingAuditEvent(env: Env, nextEvent: BillingAuditEventSummary): Promise<void> {
  if (!env.BILLING_AUDIT_R2) {
    return;
  }
  const current = await readStoredAuditEvents(env);
  const next = replaceByKey(current, nextEvent, (entry) => entry.eventId);
  await env.BILLING_AUDIT_R2.put(AUDIT_EVENTS_KEY, JSON.stringify(next));
}

async function updateAdminAccountProjection(env: Env, status: BillingStatusSummary): Promise<void> {
  const adminAccounts = await loadAdminBillingAccounts(env, null);
  const current = adminAccounts.find((entry) => entry.parentAccountRef === status.parentAccountRef);
  if (!current) {
    return;
  }

  await upsertAdminBillingAccountSummary(env, {
    ...current,
    parentVisibleState: status.parentVisibleState,
    subscriptionStatus: status.subscriptionStatus,
    planId: status.plan.planId,
    manualRequired: status.accountStatus === 'manual-review',
    failureKind: status.failureState?.failureKind ?? null,
    auditReference: status.auditReference,
    updatedAt: status.updatedAt,
  } as AdminBillingAccountSummary);
}

type ProviderWebhookTransition =
  'activate-subscription' | 'enter-grace' | 'dispute-opened' | 'dispute-lost' | 'dispute-won' | 'ignore';

function providerWebhookTransition(eventType: string): ProviderWebhookTransition {
  if (
    new Set([
      'checkout.session.completed',
      'invoice.paid',
      'customer.subscription.created',
      'customer.subscription.updated',
      'subscription.charged',
      'subscription.activated',
      'BILLING.SUBSCRIPTION.ACTIVATED',
      'SUBSCRIPTION_RENEWED',
      'DID_RENEW',
    ]).has(eventType)
  ) {
    return 'activate-subscription';
  }
  if (
    new Set(['invoice.payment_failed', 'payment_failed', 'subscription_past_due', 'payment.requires_action']).has(
      eventType
    )
  ) {
    return 'enter-grace';
  }
  if (new Set(['charge.dispute.created', 'dispute_open']).has(eventType)) {
    return 'dispute-opened';
  }
  if (new Set(['charge.dispute.funds_withdrawn', 'dispute_lost']).has(eventType)) {
    return 'dispute-lost';
  }
  if (new Set(['charge.dispute.funds_reinstated', 'dispute_won']).has(eventType)) {
    return 'dispute-won';
  }
  return 'ignore';
}

function graceFailureState(): BillingStatusSummary['failureState'] {
  return {
    failureKind: 'payment-required',
    parentResolution: 'payment-update',
    retryAllowed: true,
    retryAfter: '2026-06-15T00:00:00.000Z',
  };
}

function manualReviewFailureState(): BillingStatusSummary['failureState'] {
  return {
    failureKind: 'provider-unavailable',
    parentResolution: 'manual-support-review',
    retryAllowed: false,
    retryAfter: null,
  };
}

function manualInvoiceSummaryId(subject: string, requestId: string): string {
  return `${subject.replaceAll(':', '-')}-manual-invoice-${requestId}`;
}

function referralInviteSummaryId(referralCode: string, requestId: string): string {
  return `${referralCode}-invite-${requestId}`;
}

function hostedSessionAuditEventId(
  sessionKind: 'checkout-session-create' | 'billing-portal-session-create',
  requestId: string
): string {
  return sessionKind === 'checkout-session-create'
    ? `billing-checkout-session:${requestId}`
    : `billing-portal-session:${requestId}`;
}

function hostedSessionAuditEventType(sessionKind: 'checkout-session-create' | 'billing-portal-session-create'): string {
  return sessionKind === 'checkout-session-create'
    ? 'billing.checkout-session.created'
    : 'billing.portal-session.created';
}

async function seedD1Tables(database: D1Database, patch: BillingBindingSeedPatch): Promise<void> {
  const stateStatements: D1PreparedStatement[] = [];
  if (patch.statusBySubject) {
    for (const [subject, row] of Object.entries(patch.statusBySubject)) {
      const decoded = decodeBillingStatusSummary(row, `billing-status-seed:${subject}`, subject);
      stateStatements.push(database.prepare(UPSERT_STATUS_SQL).bind(subject, JSON.stringify(decoded)));
    }
  }
  if (patch.snapshotsBySubject) {
    for (const [subject, snapshot] of Object.entries(patch.snapshotsBySubject)) {
      const decoded = decodeBillingEntitlementSnapshot(
        snapshot,
        `billing-entitlement-snapshot-seed:${subject}`,
        subject
      );
      stateStatements.push(database.prepare(UPSERT_SNAPSHOT_SQL).bind(subject, JSON.stringify(decoded)));
    }
  }
  if (patch.statusBySubject && patch.snapshotsBySubject) {
    for (const [subject, status] of Object.entries(patch.statusBySubject)) {
      const snapshot = patch.snapshotsBySubject[subject];
      if (snapshot) {
        decodeBillingStatePair(status, snapshot, `billing-state-seed:${subject}`);
      }
    }
  }
  if (stateStatements.length > 0) {
    await database.batch(stateStatements);
  }
  if (patch.invoicesBySubject) {
    for (const [subject, invoices] of Object.entries(patch.invoicesBySubject)) {
      for (const invoice of invoices) {
        await database.prepare(UPSERT_INVOICE_SQL).bind(subject, invoice.invoiceId, JSON.stringify(invoice)).run();
      }
    }
  }
  if (patch.referralsBySubject) {
    for (const [subject, referral] of Object.entries(patch.referralsBySubject)) {
      await database.prepare(UPSERT_REFERRAL_SQL).bind(subject, JSON.stringify(referral)).run();
    }
  }
  for (const row of patch.adminAccounts ?? []) {
    await database.prepare(UPSERT_ADMIN_ACCOUNT_SQL).bind(row.parentAccountRef, JSON.stringify(row)).run();
  }
  for (const row of patch.adminInvoices ?? []) {
    await database.prepare(UPSERT_ADMIN_INVOICE_SQL).bind(row.invoiceId, JSON.stringify(row)).run();
  }
  for (const row of patch.adminDisputes ?? []) {
    await database.prepare(UPSERT_ADMIN_DISPUTE_SQL).bind(row.disputeId, JSON.stringify(row)).run();
  }
  for (const row of patch.adminReferrals ?? []) {
    await database.prepare(UPSERT_ADMIN_REFERRAL_SQL).bind(row.referralCode, JSON.stringify(row)).run();
  }
}

async function ensureReadModelSeedOnce(env: Env): Promise<void> {
  if (!isLocalFixtureEnvironment(env)) {
    return;
  }
  const patch = buildDefaultBillingBindingSeed(env);

  if (env.BILLING_D1) {
    await env.BILLING_D1.exec(CREATE_READ_MODEL_SCHEMA_SQL);
    const rowCount = await d1First<RowCountRow>(env.BILLING_D1, SELECT_STATUS_ROW_COUNT_SQL);
    if (Number(rowCount?.row_count ?? 0) === 0) {
      await seedD1Tables(env.BILLING_D1, patch);
    }
  }

  if (env.BILLING_CONFIG_KV) {
    const existingPlans = await env.BILLING_CONFIG_KV.get(PRICING_PLANS_KEY);
    if (existingPlans === null && patch.pricingPlans) {
      const decodedPlans = patch.pricingPlans.map((plan, index) =>
        decodePricingPlan(plan, `billing-pricing-plan-seed-${index}`)
      );
      await env.BILLING_CONFIG_KV.put(PRICING_PLANS_KEY, JSON.stringify(decodedPlans));
    }
  }

  if (env.BILLING_AUDIT_R2) {
    const existingAudit = await env.BILLING_AUDIT_R2.get(AUDIT_EVENTS_KEY);
    if (existingAudit === null && patch.auditEvents) {
      await env.BILLING_AUDIT_R2.put(AUDIT_EVENTS_KEY, JSON.stringify(patch.auditEvents));
    }
  }
}

async function ensureReadModelSeed(env: Env): Promise<void> {
  const existing = seedReadyByEnv.get(env);
  if (existing) {
    await existing;
    return;
  }

  const promise = ensureReadModelSeedOnce(env);
  seedReadyByEnv.set(env, promise);
  try {
    await promise;
  } catch (error) {
    seedReadyByEnv.delete(env);
    throw error;
  }
}

function includesQuery(values: ReadonlyArray<string>, query: string): boolean {
  return values.some((value) => value.toLowerCase().includes(query));
}

export async function loadPricingPlans(env: Env): Promise<ReadonlyArray<PricingPlanSummary>> {
  await ensureReadModelSeed(env);
  requireProductionBinding(env, 'BILLING_CONFIG_KV');
  let plans: unknown;
  try {
    plans = await env.BILLING_CONFIG_KV?.get(PRICING_PLANS_KEY, 'json');
  } catch (_error) {
    throw new BillingReadModelUnavailableError('billing-pricing-plans-invalid-json');
  }
  await recordBindingRead(env, 'pricing-public', null);
  if (Array.isArray(plans)) {
    return plans.map((plan, index) => decodePricingPlan(plan, `billing-pricing-plan-${index}`));
  }
  if (isLocalFixtureEnvironment(env)) {
    return LOCAL_PRICING_PLANS.map((plan, index) => decodePricingPlan(plan, `billing-local-pricing-plan-${index}`));
  }
  throw new BillingReadModelUnavailableError('billing-pricing-plans-missing');
}

export async function loadLocalSeedSummary(env: Env): Promise<{
  generatedAt: string;
  environment: string;
  authAdapterMode: string;
  pricingPlanCount: number;
  adminAccountCount: number;
  referralFixtureCount: number;
  manualReviewAccountCount: number;
} | null> {
  if (!isLocalFixtureEnvironment(env)) {
    return null;
  }
  const pricingPlans = await loadPricingPlans(env);
  const adminAccounts = await loadAdminBillingAccounts(env, null);
  const referrals = await loadAdminBillingReferrals(env, null);
  return {
    generatedAt: GENERATED_AT,
    environment: env.ENVIRONMENT,
    authAdapterMode: env.AUTH_ADAPTER_MODE?.trim() || 'local-safe-fixture',
    pricingPlanCount: pricingPlans.length,
    adminAccountCount: adminAccounts.length,
    referralFixtureCount: referrals.length,
    manualReviewAccountCount: adminAccounts.filter((account) => account.manualRequired).length,
  };
}

export async function loadBillingStatusSummary(env: Env, subject: string): Promise<BillingStatusSummary> {
  await ensureReadModelSeed(env);
  requireProductionBinding(env, 'BILLING_D1');
  const storedPayload = parseUnknownPayloadRow(
    await d1First<PayloadJsonRow>(env.BILLING_D1, SELECT_STATUS_BY_SUBJECT_SQL, subject),
    `billing-status:${subject}`
  );
  await recordBindingRead(env, 'billing-status', subject);
  const stored =
    storedPayload === null ? null : decodeBillingStatusSummary(storedPayload, `billing-status:${subject}`, subject);
  const required = requireProductionRecord(env, stored, `billing-status-row-missing:${subject}`);
  return (
    required ??
    decodeBillingStatusSummary(buildBillingStatusSummary(subject, env), `billing-status:${subject}`, subject)
  );
}

export async function loadBillingInvoices(env: Env, subject: string): Promise<ReadonlyArray<BillingInvoiceSummary>> {
  await ensureReadModelSeed(env);
  requireProductionBinding(env, 'BILLING_D1');
  const stored = parsePayloadRows<BillingInvoiceSummary>(
    await d1All<PayloadJsonRow>(env.BILLING_D1, SELECT_INVOICES_BY_SUBJECT_SQL, subject)
  );
  await recordBindingRead(env, 'billing-invoices', subject);
  return stored.length > 0 || !isLocalFixtureEnvironment(env) ? stored : buildBillingInvoices(subject);
}

export async function findBillingInvoiceSubject(env: Env, invoiceId: string): Promise<string | null> {
  await ensureReadModelSeed(env);
  requireProductionBinding(env, 'BILLING_D1');
  if (!isLocalFixtureEnvironment(env)) {
    const row = await d1First<InvoiceSubjectRow>(env.BILLING_D1, SELECT_INVOICE_SUBJECT_SQL, invoiceId);
    return row?.subject ?? null;
  }
  for (const subject of DEFAULT_BILLING_SUBJECTS) {
    const invoice = (await loadBillingInvoices(env, subject)).find((row) => row.invoiceId === invoiceId);
    if (invoice) {
      return subject;
    }
  }

  return null;
}

export async function loadBillingReferralSummary(env: Env, subject: string): Promise<BillingReferralSummary> {
  await ensureReadModelSeed(env);
  requireProductionBinding(env, 'BILLING_D1');
  const storedPayload = parseUnknownPayloadRow(
    await d1First<PayloadJsonRow>(env.BILLING_D1, SELECT_REFERRAL_BY_SUBJECT_SQL, subject),
    `billing-referral:${subject}`
  );
  await recordBindingRead(env, 'billing-referrals', subject);
  const stored =
    storedPayload === null ? null : decodeBillingReferralSummary(storedPayload, `billing-referral:${subject}`);
  const required = requireProductionRecord(env, stored, `billing-referral-row-missing:${subject}`);
  return required ?? decodeBillingReferralSummary(buildBillingReferralSummary(subject), `billing-referral:${subject}`);
}

export async function loadBillingEntitlementSnapshot(
  env: Env,
  subject: string
): Promise<BillingEntitlementSnapshotSummary> {
  await ensureReadModelSeed(env);
  requireProductionBinding(env, 'BILLING_D1');
  const storedPayload = parseUnknownPayloadRow(
    await d1First<PayloadJsonRow>(env.BILLING_D1, SELECT_SNAPSHOT_BY_SUBJECT_SQL, subject),
    `billing-entitlement-snapshot:${subject}`
  );
  await recordBindingRead(env, 'billing-entitlement-snapshot', subject);
  const stored =
    storedPayload === null
      ? null
      : decodeBillingEntitlementSnapshot(storedPayload, `billing-entitlement-snapshot:${subject}`, subject);
  const required = requireProductionRecord(env, stored, `billing-entitlement-snapshot-row-missing:${subject}`);
  return (
    required ??
    decodeBillingEntitlementSnapshot(
      buildEntitlementSnapshot(subject),
      `billing-entitlement-snapshot:${subject}`,
      subject
    )
  );
}

export async function loadBillingLicenseDecision(
  env: Env,
  subject: string,
  requestId: string,
  deviceId: string,
  requestedNewDevice: boolean
): Promise<BillingLicenseDecisionSummary> {
  const status = await loadBillingStatusSummary(env, subject);
  const snapshot = await loadBillingEntitlementSnapshot(env, subject);
  if (
    snapshot.subject !== status.subject ||
    snapshot.parentAccountRef !== status.parentAccountRef ||
    snapshot.familyRef !== status.familyRef ||
    snapshot.planId !== status.plan.planId ||
    snapshot.deviceLimit !== status.seatComposition.effectiveLimit
  ) {
    throw new BillingReadModelUnavailableError('billing-entitlement-authority-mismatch');
  }
  const requestedDeviceAlreadyTrusted = !requestedNewDevice;
  const atDeviceLimit = snapshot.activeDevices >= snapshot.deviceLimit;

  if (snapshot.parentVisibleState === 'manual-review') {
    return {
      requestId,
      subject,
      deviceId,
      decision: 'manual-review',
      reasonCode: 'manual-review',
      deviceActivationBehavior: 'manual-review-required',
      requestedDeviceAlreadyTrusted,
      planId: snapshot.planId,
      currentActiveDevices: snapshot.activeDevices,
      limit: snapshot.deviceLimit,
      auditReference: `${snapshot.auditReference}:license-check-review`,
    };
  }

  if (requestedNewDevice && atDeviceLimit) {
    return {
      requestId,
      subject,
      deviceId,
      decision: 'denied',
      reasonCode: 'limit-exceeded',
      deviceActivationBehavior: 'deny-new-device',
      requestedDeviceAlreadyTrusted,
      planId: snapshot.planId,
      currentActiveDevices: snapshot.activeDevices,
      limit: snapshot.deviceLimit,
      auditReference: `${snapshot.auditReference}:license-check-denied`,
    };
  }

  if (
    snapshot.subscriptionStatus === 'grace' &&
    requestedNewDevice &&
    snapshot.failureState?.failureKind === 'payment-required'
  ) {
    return {
      requestId,
      subject,
      deviceId,
      decision: 'grace',
      reasonCode: 'payment-required',
      deviceActivationBehavior: 'grace-existing-devices',
      requestedDeviceAlreadyTrusted,
      planId: snapshot.planId,
      currentActiveDevices: snapshot.activeDevices,
      limit: snapshot.deviceLimit,
      auditReference: `${snapshot.auditReference}:license-check-grace`,
    };
  }

  if (!snapshotCanYieldAllowed(snapshot, requestedDeviceAlreadyTrusted)) {
    return {
      requestId,
      subject,
      deviceId,
      decision: 'manual-review',
      reasonCode: 'manual-review',
      deviceActivationBehavior: 'manual-review-required',
      requestedDeviceAlreadyTrusted,
      planId: snapshot.planId,
      currentActiveDevices: snapshot.activeDevices,
      limit: snapshot.deviceLimit,
      auditReference: `${snapshot.auditReference}:license-check-review`,
    };
  }

  if (isLocalFixtureEnvironment(env)) {
    return buildLicenseDecision(subject, requestId, deviceId, requestedNewDevice);
  }

  return {
    requestId,
    subject,
    deviceId,
    decision: 'allowed',
    reasonCode: 'within-plan',
    deviceActivationBehavior: 'allow-new-device',
    requestedDeviceAlreadyTrusted,
    planId: snapshot.planId,
    currentActiveDevices: snapshot.activeDevices,
    limit: snapshot.deviceLimit,
    auditReference: `${snapshot.auditReference}:license-check-allowed`,
  };
}

export async function loadAdminBillingAccounts(
  env: Env,
  query: string | null
): Promise<ReadonlyArray<AdminBillingAccountSummary>> {
  await ensureReadModelSeed(env);
  requireProductionBinding(env, 'BILLING_D1');
  const loweredQuery = query?.trim().toLowerCase() ?? '';
  const stored = parsePayloadRows<AdminBillingAccountSummary>(
    await d1All<PayloadJsonRow>(env.BILLING_D1, SELECT_ADMIN_ACCOUNTS_SQL)
  );
  await recordBindingRead(env, 'admin-billing-accounts', null);
  const rows = stored.length > 0 || !isLocalFixtureEnvironment(env) ? stored : listAdminBillingAccounts(query);
  if (!loweredQuery) {
    return rows;
  }
  return rows.filter((row) =>
    includesQuery([row.parentAccountRef, row.familyRef, row.planId, row.parentVisibleState], loweredQuery)
  );
}

export async function loadAdminBillingInvoices(
  env: Env,
  query: string | null
): Promise<ReadonlyArray<AdminBillingInvoiceSummary>> {
  await ensureReadModelSeed(env);
  requireProductionBinding(env, 'BILLING_D1');
  const loweredQuery = query?.trim().toLowerCase() ?? '';
  const stored = parsePayloadRows<AdminBillingInvoiceSummary>(
    await d1All<PayloadJsonRow>(env.BILLING_D1, SELECT_ADMIN_INVOICES_SQL)
  );
  await recordBindingRead(env, 'admin-billing-invoices', null);
  const rows = stored.length > 0 || !isLocalFixtureEnvironment(env) ? stored : listAdminBillingInvoices(query);
  if (!loweredQuery) {
    return rows;
  }
  return rows.filter((row) =>
    includesQuery([row.invoiceId, row.invoiceNumber, row.parentAccountRef, row.familyRef, row.planId], loweredQuery)
  );
}

export async function loadAdminBillingDisputes(
  env: Env,
  query: string | null
): Promise<ReadonlyArray<AdminBillingDisputeSummary>> {
  await ensureReadModelSeed(env);
  requireProductionBinding(env, 'BILLING_D1');
  const loweredQuery = query?.trim().toLowerCase() ?? '';
  const stored = parsePayloadRows<AdminBillingDisputeSummary>(
    await d1All<PayloadJsonRow>(env.BILLING_D1, SELECT_ADMIN_DISPUTES_SQL)
  );
  await recordBindingRead(env, 'admin-billing-disputes', null);
  const rows = stored.length > 0 || !isLocalFixtureEnvironment(env) ? stored : listAdminBillingDisputes(query);
  if (!loweredQuery) {
    return rows;
  }
  return rows.filter((row) =>
    includesQuery([row.disputeId, row.parentAccountRef, row.familyRef, row.invoiceId, row.disputeState], loweredQuery)
  );
}

export async function loadAdminBillingReferrals(
  env: Env,
  query: string | null
): Promise<ReadonlyArray<AdminBillingReferralSummary>> {
  await ensureReadModelSeed(env);
  requireProductionBinding(env, 'BILLING_D1');
  const loweredQuery = query?.trim().toLowerCase() ?? '';
  const stored = parsePayloadRows<AdminBillingReferralSummary>(
    await d1All<PayloadJsonRow>(env.BILLING_D1, SELECT_ADMIN_REFERRALS_SQL)
  );
  await recordBindingRead(env, 'admin-billing-referrals', null);
  const rows = stored.length > 0 || !isLocalFixtureEnvironment(env) ? stored : listAdminBillingReferrals(query);
  if (!loweredQuery) {
    return rows;
  }
  return rows.filter((row) => includesQuery([row.referralCode, row.ownerSubject, row.abuseReviewState], loweredQuery));
}

export async function loadBillingAuditEvents(
  env: Env,
  query: string | null
): Promise<ReadonlyArray<BillingAuditEventSummary>> {
  await ensureReadModelSeed(env);
  requireProductionBinding(env, 'BILLING_AUDIT_R2');
  const loweredQuery = query?.trim().toLowerCase() ?? '';
  const object = await env.BILLING_AUDIT_R2?.get(AUDIT_EVENTS_KEY);
  const stored = object ? ((await object.json<ReadonlyArray<BillingAuditEventSummary>>()) ?? []) : [];
  await recordBindingRead(env, 'admin-billing-audit', null);
  const rows = stored.length > 0 || !isLocalFixtureEnvironment(env) ? stored : listBillingAuditEvents(query);
  if (!loweredQuery) {
    return rows;
  }
  return rows.filter((row) =>
    includesQuery([row.eventId, row.eventType, row.parentAccountRef, row.familyRef, row.actorRole], loweredQuery)
  );
}

export async function applyBillingStateMutation(env: Env, mutation: BillingStateMutation): Promise<void> {
  await ensureReadModelSeed(env);

  switch (mutation.kind) {
    case 'hosted-session': {
      const status = await loadBillingStatusSummary(env, mutation.subject);
      const updatedAt = new Date().toISOString();
      await appendBillingAuditEvent(env, {
        eventId: hostedSessionAuditEventId(mutation.sessionKind, mutation.requestId),
        eventType: hostedSessionAuditEventType(mutation.sessionKind),
        actorRole: mutation.actorRole,
        parentAccountRef: status.parentAccountRef,
        familyRef: status.familyRef,
        auditReference: mutation.auditReference,
        createdAt: updatedAt,
      } as unknown as BillingAuditEventSummary);
      return;
    }
    case 'change-plan': {
      const status = await loadBillingStatusSummary(env, mutation.subject);
      const snapshot = await loadBillingEntitlementSnapshot(env, mutation.subject);
      const pricingPlans = await loadPricingPlans(env);
      const targetPlan = pricingPlans.find((plan) => plan.planId === mutation.targetPlanId);
      if (!targetPlan) {
        return;
      }

      const updatedAt = new Date().toISOString();
      const targetPlanDeviceLimit = decodeBillingLimit(targetPlan.deviceLimit, 'billing-change-plan-device-limit');
      const targetSeatComposition = targetPlanSeatComposition(
        status,
        targetPlanDeviceLimit,
        'billing-change-plan-seat-composition'
      );
      const availableDeviceSlots = remainingBillingSlots(
        targetPlanDeviceLimit,
        status.deviceUsage.activeDevices,
        'billing-change-plan-available-device-slots'
      );
      const nextStatus = {
        ...status,
        plan: cloneJsonValue(targetPlan),
        deviceUsage: {
          ...status.deviceUsage,
          limit: targetSeatComposition.effectiveChildDeviceLimit,
        },
        seatComposition: {
          ...status.seatComposition,
          paidExtraSeats: targetSeatComposition.paidExtraChildDeviceSeats,
          effectiveLimit: targetSeatComposition.effectiveChildDeviceLimit,
          availableDeviceSlots,
        },
        warnings: withAddedWarning(status.warnings, 'plan-change-pending-provider-sync'),
        auditReference: mutation.auditReference,
        updatedAt,
      } as unknown as BillingStatusSummary;
      const nextSnapshot = {
        ...snapshot,
        planId: targetPlan.planId,
        deviceLimit: targetSeatComposition.effectiveChildDeviceLimit,
        availableDeviceSlots: remainingBillingSlots(
          targetSeatComposition.effectiveChildDeviceLimit,
          snapshot.activeDevices,
          'billing-change-plan-snapshot-available-device-slots'
        ),
        signedAt: updatedAt,
        auditReference: `${mutation.auditReference}:snapshot`,
      } as unknown as BillingEntitlementSnapshotSummary;

      await upsertBillingStatePair(env, nextStatus, nextSnapshot);
      await updateAdminAccountProjection(env, nextStatus);
      await appendBillingAuditEvent(env, {
        eventId: `billing-change-plan:${mutation.requestId}`,
        eventType: 'billing.change-plan.accepted',
        actorRole: 'parent',
        parentAccountRef: nextStatus.parentAccountRef,
        familyRef: nextStatus.familyRef,
        auditReference: mutation.auditReference,
        createdAt: updatedAt,
      } as unknown as BillingAuditEventSummary);
      return;
    }
    case 'cancel': {
      const status = await loadBillingStatusSummary(env, mutation.subject);
      const snapshot = await loadBillingEntitlementSnapshot(env, mutation.subject);
      const updatedAt = new Date().toISOString();
      const cancellationWarning =
        mutation.cancellationState === 'scheduled-period-end'
          ? 'cancellation-scheduled-period-end'
          : mutation.cancellationState === 'already-in-grace'
            ? 'cancellation-confirmed-in-grace'
            : 'cancellation-manual-review-required';

      const nextStatus = {
        ...status,
        accountStatus: mutation.cancellationState === 'manual-review-required' ? 'manual-review' : status.accountStatus,
        subscriptionStatus:
          mutation.cancellationState === 'manual-review-required' ? 'past-due' : status.subscriptionStatus,
        portalVisibleState:
          mutation.cancellationState === 'manual-review-required' ? 'manual-required' : status.portalVisibleState,
        parentVisibleState:
          mutation.cancellationState === 'manual-review-required' ? 'manual-review' : status.parentVisibleState,
        localSafetyBehavior:
          mutation.cancellationState === 'manual-review-required'
            ? 'manual-review-with-local-safety'
            : status.localSafetyBehavior,
        source: mutation.cancellationState === 'manual-review-required' ? 'manual-admin-review' : status.source,
        failureState:
          mutation.cancellationState === 'manual-review-required' ? manualReviewFailureState() : status.failureState,
        warnings: withAddedWarning(status.warnings, cancellationWarning),
        auditReference: mutation.auditReference,
        updatedAt,
      } as unknown as BillingStatusSummary;
      const nextSnapshot = {
        ...snapshot,
        source: mutation.cancellationState === 'manual-review-required' ? 'manual-admin-review' : snapshot.source,
        signatureState:
          mutation.cancellationState === 'manual-review-required' ? 'manual-required' : snapshot.signatureState,
        subscriptionStatus:
          mutation.cancellationState === 'manual-review-required' ? 'past-due' : snapshot.subscriptionStatus,
        parentVisibleState:
          mutation.cancellationState === 'manual-review-required' ? 'manual-review' : snapshot.parentVisibleState,
        localSafetyBehavior:
          mutation.cancellationState === 'manual-review-required'
            ? 'manual-review-with-local-safety'
            : snapshot.localSafetyBehavior,
        failureState:
          mutation.cancellationState === 'manual-review-required' ? manualReviewFailureState() : snapshot.failureState,
        signedAt: updatedAt,
        auditReference: `${mutation.auditReference}:snapshot`,
      } as unknown as BillingEntitlementSnapshotSummary;

      await upsertBillingStatePair(env, nextStatus, nextSnapshot);
      await updateAdminAccountProjection(env, nextStatus);
      await appendBillingAuditEvent(env, {
        eventId: `billing-cancel:${mutation.requestId}`,
        eventType: 'billing.cancel.accepted',
        actorRole: 'parent',
        parentAccountRef: nextStatus.parentAccountRef,
        familyRef: nextStatus.familyRef,
        auditReference: mutation.auditReference,
        createdAt: updatedAt,
      } as unknown as BillingAuditEventSummary);
      return;
    }
    case 'referral-invite': {
      const referral = await loadBillingReferralSummary(env, mutation.subject);
      const adminReferrals = await loadAdminBillingReferrals(env, null);
      const updatedAt = new Date().toISOString();
      const nextReferral = {
        ...referral,
        pendingInvites: incrementBillingCount(referral.pendingInvites, 'billing-referral-pending-invites'),
        invites: [
          {
            inviteId: referralInviteSummaryId(mutation.referralCode, mutation.requestId),
            inviteState: 'invite-created',
            referralCode: mutation.referralCode,
            invitedIdentifier: mutation.invitedIdentifier,
            auditReference: mutation.auditReference,
            updatedAt,
          },
          ...referral.invites,
        ],
        auditReference: `${mutation.auditReference}:summary`,
      } as unknown as BillingReferralSummary;
      const existingAdminReferral = adminReferrals.find((row) => row.referralCode === mutation.referralCode);
      const nextAdminReferral = (existingAdminReferral
        ? {
            ...existingAdminReferral,
            invitedFamilies: incrementBillingCount(
              existingAdminReferral.invitedFamilies,
              'billing-referral-invited-families'
            ),
            auditReference: `${mutation.auditReference}:admin`,
            updatedAt,
          }
        : {
            referralCode: mutation.referralCode,
            ownerSubject: mutation.subject,
            creditedFamilies: nextReferral.availableCredits,
            invitedFamilies: sumBillingCounts(
              nextReferral.activeReferredParents,
              nextReferral.pendingInvites,
              'billing-referral-invited-families'
            ),
            abuseReviewState: 'clear',
            auditReference: `${mutation.auditReference}:admin`,
            updatedAt,
          }) as unknown as AdminBillingReferralSummary;

      await upsertBillingReferralSummary(env, nextReferral);
      await upsertAdminBillingReferralSummary(env, nextAdminReferral);
      await appendBillingAuditEvent(env, {
        eventId: `billing-referral-invite:${mutation.requestId}`,
        eventType: 'billing.referral.invite-created',
        actorRole: mutation.actorRole,
        parentAccountRef: (await loadBillingStatusSummary(env, mutation.subject)).parentAccountRef,
        familyRef: (await loadBillingStatusSummary(env, mutation.subject)).familyRef,
        auditReference: mutation.auditReference,
        createdAt: updatedAt,
      } as unknown as BillingAuditEventSummary);
      return;
    }
    case 'manual-invoice': {
      const status = await loadBillingStatusSummary(env, mutation.subject);
      const snapshot = await loadBillingEntitlementSnapshot(env, mutation.subject);
      const updatedAt = new Date().toISOString();
      const auditReference = `${mutation.auditReference}:state`;
      const manualInvoiceId = manualInvoiceSummaryId(mutation.subject, mutation.requestId);
      const nextInvoice = {
        invoiceId: manualInvoiceId,
        invoiceNumber: `INV-MANUAL-${mutation.requestId.toUpperCase()}`,
        parentAccountRef: status.parentAccountRef,
        familyRef: status.familyRef,
        planId: status.plan.planId,
        currency: 'USD',
        subtotalCents: status.plan.priceCents,
        taxCents: 0,
        totalCents: status.plan.priceCents,
        invoiceVisibility: 'manual-support-required',
        paymentState: 'unpaid',
        provider: 'manual-invoice',
        hostedUrl: null,
        periodStart: updatedAt,
        periodEnd: updatedAt,
        updatedAt,
        auditReference: `${auditReference}:invoice`,
      } as unknown as BillingInvoiceSummary;
      const nextAdminInvoice = {
        ...nextInvoice,
        manualRequired: true,
      } as unknown as AdminBillingInvoiceSummary;
      const nextStatus = {
        ...status,
        accountStatus: 'manual-review',
        subscriptionStatus: 'past-due',
        portalVisibleState: 'manual-required',
        parentVisibleState: 'manual-review',
        localSafetyBehavior: 'manual-review-with-local-safety',
        providerMode: 'manual-invoice',
        nextRenewalAt: null,
        manualInvoiceState: {
          visible: true,
          invoiceState: 'manual-support-required',
        },
        source: 'manual-admin-review',
        failureState: manualReviewFailureState(),
        warnings: withAddedWarning(status.warnings, 'manual-invoice-issued'),
        auditReference,
        updatedAt,
      } as unknown as BillingStatusSummary;
      const nextSnapshot = {
        ...snapshot,
        subscriptionStatus: 'past-due',
        source: 'manual-admin-review',
        signatureState: 'manual-required',
        parentVisibleState: 'manual-review',
        localSafetyBehavior: 'manual-review-with-local-safety',
        failureState: manualReviewFailureState(),
        signedAt: updatedAt,
        auditReference: `${auditReference}:snapshot`,
      } as unknown as BillingEntitlementSnapshotSummary;

      await upsertBillingInvoiceSummary(env, mutation.subject, nextInvoice);
      await upsertAdminBillingInvoiceSummary(env, nextAdminInvoice);
      await upsertBillingStatePair(env, nextStatus, nextSnapshot);
      await updateAdminAccountProjection(env, nextStatus);
      await appendBillingAuditEvent(env, {
        eventId: `billing-manual-invoice:${mutation.requestId}`,
        eventType: 'billing.manual-invoice.created',
        actorRole: mutation.actorRole,
        parentAccountRef: status.parentAccountRef,
        familyRef: status.familyRef,
        auditReference,
        createdAt: updatedAt,
      } as unknown as BillingAuditEventSummary);
      return;
    }
    case 'admin-refund': {
      const status = await loadBillingStatusSummary(env, mutation.subject);
      const snapshot = await loadBillingEntitlementSnapshot(env, mutation.subject);
      const invoices = await loadBillingInvoices(env, mutation.subject);
      const adminInvoices = await loadAdminBillingInvoices(env, null);
      const updatedAt = new Date().toISOString();
      const refundSettled = mutation.refundState === 'refund-settled';
      const auditReference = `${mutation.auditReference}:state`;
      const nextFailureState = refundSettled ? manualReviewFailureState() : status.failureState;

      let matchedInvoice = false;
      for (const invoice of invoices) {
        if (invoice.invoiceId !== mutation.invoiceId) {
          continue;
        }

        matchedInvoice = true;
        await upsertBillingInvoiceSummary(env, mutation.subject, {
          ...invoice,
          paymentState: refundSettled ? 'refunded' : invoice.paymentState,
          auditReference: `${auditReference}:invoice:${invoice.invoiceId}`,
          updatedAt,
        } as unknown as BillingInvoiceSummary);
      }

      for (const invoice of adminInvoices) {
        if (invoice.invoiceId !== mutation.invoiceId) {
          continue;
        }

        await upsertAdminBillingInvoiceSummary(env, {
          ...invoice,
          paymentState: refundSettled ? 'refunded' : invoice.paymentState,
          auditReference: `${auditReference}:admin-invoice:${invoice.invoiceId}`,
          updatedAt,
        } as unknown as AdminBillingInvoiceSummary);
      }

      if (refundSettled && matchedInvoice) {
        const nextStatus: any = {
          ...status,
          accountStatus: 'manual-review',
          subscriptionStatus: 'past-due',
          portalVisibleState: 'manual-required',
          parentVisibleState: 'manual-review',
          localSafetyBehavior: 'manual-review-with-local-safety',
          source: 'manual-admin-review',
          failureState: nextFailureState,
          nextRenewalAt: null,
          warnings: withAddedWarning(
            status.warnings.filter((warning) => warning !== 'provider-webhook-synced'),
            'refund-settled-manual-review'
          ),
          auditReference,
          updatedAt,
        } as unknown as BillingStatusSummary;
        const nextSnapshot: any = {
          ...snapshot,
          subscriptionStatus: 'past-due',
          source: 'manual-admin-review',
          signatureState: 'manual-required',
          parentVisibleState: 'manual-review',
          localSafetyBehavior: 'manual-review-with-local-safety',
          failureState: nextFailureState,
          signedAt: updatedAt,
          auditReference: `${auditReference}:snapshot`,
        } as unknown as BillingEntitlementSnapshotSummary;

        await upsertBillingStatePair(env, nextStatus, nextSnapshot);
        await updateAdminAccountProjection(env, nextStatus);
      }

      await appendBillingAuditEvent(env, {
        eventId: `billing-refund:${mutation.requestId}`,
        eventType: `billing.refund.${mutation.refundState}`,
        actorRole: mutation.actorRole,
        parentAccountRef: status.parentAccountRef,
        familyRef: status.familyRef,
        auditReference,
        createdAt: updatedAt,
      } as unknown as BillingAuditEventSummary);
      return;
    }
    case 'reconciliation': {
      const updatedAt = new Date().toISOString();
      await appendBillingAuditEvent(env, {
        eventId: `billing-reconciliation:${mutation.requestId}`,
        eventType: 'billing.reconciliation.accepted',
        actorRole: mutation.actorRole,
        parentAccountRef: RECONCILIATION_PARENT_ACCOUNT_REF,
        familyRef: RECONCILIATION_FAMILY_REF,
        auditReference: mutation.auditReference,
        createdAt: updatedAt,
      } as unknown as BillingAuditEventSummary);
      return;
    }
    case 'provider-webhook': {
      const transition = providerWebhookTransition(mutation.eventType);
      if (transition === 'ignore') {
        return;
      }

      const status = await loadBillingStatusSummary(env, mutation.subject);
      const snapshot = await loadBillingEntitlementSnapshot(env, mutation.subject);
      const invoices = await loadBillingInvoices(env, mutation.subject);
      const adminInvoices = await loadAdminBillingInvoices(env, null);
      const updatedAt = new Date().toISOString();
      const auditReference = `${status.auditReference}:provider-webhook:${mutation.provider}`;

      if (transition === 'activate-subscription') {
        const nextStatus: any = {
          ...status,
          accountStatus: 'active',
          subscriptionStatus: 'active',
          portalVisibleState: 'ready',
          parentVisibleState: 'available',
          localSafetyBehavior: 'unchanged',
          source: 'signed-local-snapshot',
          failureState: null,
          warnings: withAddedWarning(
            status.warnings.filter(
              (warning) =>
                warning !== 'cancellation-confirmed-in-grace' &&
                warning !== 'cancellation-manual-review-required' &&
                warning !== 'cancellation-scheduled-period-end' &&
                warning !== 'provider-webhook-payment-required' &&
                warning !== 'provider-webhook-dispute-opened' &&
                warning !== 'provider-webhook-dispute-lost'
            ),
            'provider-webhook-synced'
          ),
          auditReference,
          updatedAt,
        } as unknown as BillingStatusSummary;
        const nextSnapshot: any = {
          ...snapshot,
          subscriptionStatus: 'active',
          source: 'signed-local-snapshot',
          signatureState: 'signed',
          parentVisibleState: 'available',
          localSafetyBehavior: 'unchanged',
          failureState: null,
          signedAt: updatedAt,
          auditReference: `${auditReference}:snapshot`,
        } as unknown as BillingEntitlementSnapshotSummary;

        await upsertBillingStatePair(env, nextStatus, nextSnapshot);
        for (const invoice of invoices) {
          const nextInvoice = {
            ...invoice,
            paymentState: invoice.provider === 'manual-invoice' ? invoice.paymentState : 'paid',
            auditReference: `${auditReference}:invoice:${invoice.invoiceId}`,
            updatedAt,
          } as unknown as BillingInvoiceSummary;
          await upsertBillingInvoiceSummary(env, mutation.subject, nextInvoice);
        }
        for (const invoice of adminInvoices.filter((entry) => entry.parentAccountRef === status.parentAccountRef)) {
          const nextInvoice: any = {
            ...invoice,
            paymentState: invoice.provider === 'manual-invoice' ? invoice.paymentState : 'paid',
            manualRequired: false,
            auditReference: `${auditReference}:admin-invoice:${invoice.invoiceId}`,
            updatedAt,
          };
          await upsertAdminBillingInvoiceSummary(env, nextInvoice);
        }
        await updateAdminAccountProjection(env, nextStatus);
      } else if (transition === 'enter-grace') {
        const nextStatus = {
          ...status,
          accountStatus: 'grace',
          subscriptionStatus: 'grace',
          portalVisibleState: 'degraded',
          parentVisibleState: 'grace',
          localSafetyBehavior: 'grace-with-local-safety',
          source: 'signed-local-snapshot',
          failureState: graceFailureState(),
          warnings: withAddedWarning(
            status.warnings.filter(
              (warning) =>
                warning !== 'provider-webhook-synced' &&
                warning !== 'provider-webhook-dispute-opened' &&
                warning !== 'provider-webhook-dispute-lost'
            ),
            'provider-webhook-payment-required'
          ),
          auditReference,
          updatedAt,
        } as unknown as BillingStatusSummary;
        const nextSnapshot = {
          ...snapshot,
          subscriptionStatus: 'grace',
          source: 'signed-local-snapshot',
          signatureState: 'signed',
          parentVisibleState: 'grace',
          localSafetyBehavior: 'grace-with-local-safety',
          failureState: graceFailureState(),
          signedAt: updatedAt,
          auditReference: `${auditReference}:snapshot`,
        } as unknown as BillingEntitlementSnapshotSummary;

        await upsertBillingStatePair(env, nextStatus, nextSnapshot);
        for (const invoice of invoices) {
          const nextInvoice = {
            ...invoice,
            paymentState: invoice.provider === 'manual-invoice' ? invoice.paymentState : 'grace',
            auditReference: `${auditReference}:invoice:${invoice.invoiceId}`,
            updatedAt,
          } as unknown as BillingInvoiceSummary;
          await upsertBillingInvoiceSummary(env, mutation.subject, nextInvoice);
        }
        for (const invoice of adminInvoices.filter((entry) => entry.parentAccountRef === status.parentAccountRef)) {
          const nextInvoice: any = {
            ...invoice,
            paymentState: invoice.provider === 'manual-invoice' ? invoice.paymentState : 'grace',
            manualRequired: invoice.invoiceVisibility === 'manual-support-required',
            auditReference: `${auditReference}:admin-invoice:${invoice.invoiceId}`,
            updatedAt,
          };
          await upsertAdminBillingInvoiceSummary(env, nextInvoice);
        }
        await updateAdminAccountProjection(env, nextStatus);
      } else {
        const disputeId = mutation.disputeId ?? `dispute-${mutation.provider}-${mutation.eventId}`;
        const invoiceId =
          mutation.invoiceId ?? invoices[0]?.invoiceId ?? adminInvoices[0]?.invoiceId ?? 'invoice-unresolved';
        const disputeState =
          transition === 'dispute-opened'
            ? 'dispute-opened'
            : transition === 'dispute-lost'
              ? 'dispute-lost'
              : 'dispute-won';
        const entitlementEffect =
          transition === 'dispute-opened'
            ? 'manual-review-required'
            : transition === 'dispute-lost'
              ? 'revoke-paid-access'
              : 'grace-paid-access';
        const manualRequired = transition !== 'dispute-won';
        const nextDispute = {
          disputeId,
          parentAccountRef: status.parentAccountRef,
          familyRef: status.familyRef,
          invoiceId,
          disputeState,
          entitlementEffect,
          manualRequired,
          auditReference: `${auditReference}:dispute:${disputeId}`,
          updatedAt,
        } as unknown as AdminBillingDisputeSummary;
        await upsertAdminBillingDisputeSummary(env, nextDispute);

        const nextStatus = (transition === 'dispute-won'
          ? {
              ...status,
              accountStatus: 'active',
              subscriptionStatus: 'active',
              portalVisibleState: 'ready',
              parentVisibleState: 'available',
              localSafetyBehavior: 'unchanged',
              source: 'signed-local-snapshot',
              failureState: null,
              warnings: withAddedWarning(
                status.warnings.filter(
                  (warning) =>
                    warning !== 'provider-webhook-dispute-opened' && warning !== 'provider-webhook-dispute-lost'
                ),
                'provider-webhook-synced'
              ),
              auditReference,
              updatedAt,
            }
          : {
              ...status,
              accountStatus: 'manual-review',
              subscriptionStatus: 'past-due',
              portalVisibleState: 'manual-required',
              parentVisibleState: 'manual-review',
              localSafetyBehavior: 'manual-review-with-local-safety',
              source: 'manual-admin-review',
              failureState: manualReviewFailureState(),
              warnings: withAddedWarning(
                status.warnings.filter((warning) => warning !== 'provider-webhook-synced'),
                transition === 'dispute-opened' ? 'provider-webhook-dispute-opened' : 'provider-webhook-dispute-lost'
              ),
              auditReference,
              updatedAt,
            }) as unknown as BillingStatusSummary;
        const nextSnapshot = (transition === 'dispute-won'
          ? {
              ...snapshot,
              subscriptionStatus: 'active',
              source: 'signed-local-snapshot',
              signatureState: 'signed',
              parentVisibleState: 'available',
              localSafetyBehavior: 'unchanged',
              failureState: null,
              signedAt: updatedAt,
              auditReference: `${auditReference}:snapshot`,
            }
          : {
              ...snapshot,
              subscriptionStatus: 'past-due',
              source: 'manual-admin-review',
              signatureState: 'manual-required',
              parentVisibleState: 'manual-review',
              localSafetyBehavior: 'manual-review-with-local-safety',
              failureState: manualReviewFailureState(),
              signedAt: updatedAt,
              auditReference: `${auditReference}:snapshot`,
            }) as unknown as BillingEntitlementSnapshotSummary;

        await upsertBillingStatePair(env, nextStatus, nextSnapshot);
        await updateAdminAccountProjection(env, nextStatus);
      }

      await appendBillingAuditEvent(env, {
        eventId: `billing-webhook:${mutation.provider}:${mutation.eventId}`,
        eventType: `billing.webhook.${mutation.provider}.${mutation.eventType}`,
        actorRole: 'system',
        parentAccountRef: status.parentAccountRef,
        familyRef: status.familyRef,
        auditReference,
        createdAt: updatedAt,
      } as unknown as BillingAuditEventSummary);
      return;
    }
  }
}
