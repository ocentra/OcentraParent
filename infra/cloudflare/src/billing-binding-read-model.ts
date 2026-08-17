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
import {
  BillingReferralSummarySchema,
  BillingReferralInviteResultSchema,
  BillingSupportAdminAccountSummarySchema,
  BillingSupportAdminAuditEventSummarySchema,
  BillingSupportAdminDisputeSummarySchema,
  BillingSupportAdminInvoiceSummarySchema,
  BillingSupportAdminReferralSummarySchema,
  BillingSupportAdminRefundResultSchema,
  BillingSupportAdminReconciliationSummarySchema,
  type BillingReferralSummary,
  type BillingReferralInviteResult,
  type BillingSupportAdminReconciliationSummary,
  type BillingSupportAdminRefundResult,
} from './generated/billing-contracts.js';
import type { Env } from './env.js';
import {
  buildBillingInvoices,
  buildBillingReferralSummary,
  buildBillingStatusSummary,
  buildEntitlementSnapshot,
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
export const BILLING_AUDIT_OWNER_NAME = 'billing-audit-owner';
export const BILLING_AUDIT_APPEND_PATH = '/internal/billing-audit/append';
const MAX_BILLING_OUTBOX_DRAIN_ROWS = 20;
const MAX_BILLING_OUTBOX_ATTEMPTS = 5;

const REFUND_LEDGER_TOTAL_GUARD_SQL =
  "CREATE TRIGGER IF NOT EXISTS billing_refund_ledger_total_guard BEFORE INSERT ON billing_refund_ledger BEGIN SELECT RAISE(ABORT, 'billing-refund-ledger-total-exceeded') WHERE EXISTS (SELECT 1 FROM billing_refund_ledger WHERE invoice_id = NEW.invoice_id AND invoice_total_cents != NEW.invoice_total_cents) OR COALESCE((SELECT SUM(amount_cents) FROM billing_refund_ledger WHERE invoice_id = NEW.invoice_id), 0) + NEW.amount_cents > NEW.invoice_total_cents; END";

const BILLING_MUTATION_AUTHORITY_GUARD_SQL =
  "CREATE TRIGGER IF NOT EXISTS billing_mutation_authority_guard BEFORE INSERT ON billing_mutation_outbox BEGIN SELECT RAISE(ABORT, 'billing-mutation-authority-cas-failed') WHERE NEW.authority_subject IS NOT NULL AND (NEW.authority_version IS NULL OR NEW.authority_token IS NULL OR json_extract(NEW.mutation_json, '$.subject') IS NOT NEW.authority_subject OR NOT EXISTS (SELECT 1 FROM billing_subject_versions WHERE subject = NEW.authority_subject AND version = NEW.authority_version AND last_mutation_token = NEW.authority_token)); END";

const BILLING_PROVIDER_RECEIPT_STATE_GUARD_SQL =
  "CREATE TRIGGER IF NOT EXISTS billing_provider_receipt_state_guard BEFORE UPDATE OF processing_state, queue_state, state_version ON billing_provider_event_receipts BEGIN SELECT RAISE(ABORT, 'billing-provider-event-state-regression') WHERE (OLD.queue_state IN ('delivered', 'manual-required', 'dead-letter') AND NEW.queue_state <> OLD.queue_state) OR (OLD.processing_state IN ('applied', 'ignored', 'manual-required', 'dead-letter') AND NEW.processing_state <> OLD.processing_state) OR NEW.state_version <> OLD.state_version + 1; END";

const BILLING_PROVIDER_CURSOR_GUARD_SQL =
  "CREATE TRIGGER IF NOT EXISTS billing_provider_cursor_guard BEFORE INSERT ON billing_mutation_outbox BEGIN SELECT RAISE(ABORT, 'billing-provider-event-cursor-cas-failed') WHERE json_extract(NEW.mutation_json, '$.kind') = 'provider-webhook' AND (json_extract(NEW.mutation_json, '$.providerCursorExpectedVersion') IS NULL OR NOT EXISTS (SELECT 1 FROM billing_provider_event_cursors WHERE provider = json_extract(NEW.mutation_json, '$.provider') AND billing_subject = json_extract(NEW.mutation_json, '$.subject') AND last_event_id = json_extract(NEW.mutation_json, '$.eventId') AND state_version = json_extract(NEW.mutation_json, '$.providerCursorExpectedVersion') + 1)); END";

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
  "CREATE TABLE IF NOT EXISTS billing_provider_event_receipts (provider TEXT NOT NULL, event_id TEXT NOT NULL, event_fingerprint TEXT NOT NULL, event_type TEXT NOT NULL, provider_occurred_at TEXT, provider_sequence INTEGER CHECK (provider_sequence IS NULL OR provider_sequence BETWEEN 0 AND 4294967295), state_version INTEGER NOT NULL DEFAULT 0 CHECK (state_version BETWEEN 0 AND 4294967295), account_id TEXT, provider_customer_id TEXT, provider_subscription_id TEXT, provider_invoice_id TEXT, billing_subject TEXT, parent_account_ref TEXT, family_ref TEXT, billing_invoice_id TEXT, processing_state TEXT NOT NULL CHECK (processing_state IN ('received', 'ignored', 'queued', 'applied', 'manual-required', 'dead-letter')), queue_state TEXT NOT NULL CHECK (queue_state IN ('pending', 'queued', 'delivered', 'manual-required', 'dead-letter')), queue_attempt_count INTEGER NOT NULL DEFAULT 0 CHECK (queue_attempt_count BETWEEN 0 AND 4294967295), last_queue_attempt_at TEXT, last_error TEXT, created_at TEXT NOT NULL, updated_at TEXT NOT NULL, PRIMARY KEY (provider, event_id))",
  'CREATE TABLE IF NOT EXISTS billing_provider_event_cursors (provider TEXT NOT NULL, billing_subject TEXT NOT NULL, last_occurred_at TEXT, last_sequence INTEGER CHECK (last_sequence IS NULL OR last_sequence BETWEEN 0 AND 4294967295), last_event_id TEXT NOT NULL, state_version INTEGER NOT NULL DEFAULT 0 CHECK (state_version BETWEEN 0 AND 4294967295), updated_at TEXT NOT NULL, PRIMARY KEY (provider, billing_subject))',
  "CREATE TABLE IF NOT EXISTS billing_refund_ledger (invoice_id TEXT NOT NULL, mutation_key TEXT NOT NULL, subject TEXT NOT NULL, amount_cents INTEGER NOT NULL CHECK (amount_cents >= 0), invoice_total_cents INTEGER NOT NULL CHECK (invoice_total_cents >= 0), refund_state TEXT NOT NULL CHECK (refund_state IN ('refund-requested', 'refund-settled')), audit_reference TEXT NOT NULL, created_at TEXT NOT NULL, PRIMARY KEY (invoice_id, mutation_key))",
  'CREATE TABLE IF NOT EXISTS billing_subject_versions (subject TEXT PRIMARY KEY, version INTEGER NOT NULL CHECK (version >= 0), last_mutation_token TEXT, updated_at TEXT NOT NULL)',
  "CREATE TABLE IF NOT EXISTS billing_mutation_outbox (request_key TEXT PRIMARY KEY, authority_subject TEXT, authority_version INTEGER CHECK (authority_version IS NULL OR authority_version >= 1), authority_token TEXT, mutation_kind TEXT NOT NULL, mutation_json TEXT NOT NULL, audit_state TEXT NOT NULL CHECK (audit_state IN ('pending', 'delivered')), audit_event_json TEXT NOT NULL, attempt_count INTEGER NOT NULL DEFAULT 0 CHECK (attempt_count >= 0), last_attempt_at TEXT, last_error TEXT, lease_token TEXT, lease_expires_at TEXT, created_at TEXT NOT NULL, updated_at TEXT NOT NULL)",
  REFUND_LEDGER_TOTAL_GUARD_SQL,
  BILLING_PROVIDER_RECEIPT_STATE_GUARD_SQL,
  BILLING_PROVIDER_CURSOR_GUARD_SQL,
].join(';\n');

const CREATE_MUTATION_SCHEMA_SQL = [
  "CREATE TABLE IF NOT EXISTS billing_refund_ledger (invoice_id TEXT NOT NULL, mutation_key TEXT NOT NULL, subject TEXT NOT NULL, amount_cents INTEGER NOT NULL CHECK (amount_cents >= 0), invoice_total_cents INTEGER NOT NULL CHECK (invoice_total_cents >= 0), refund_state TEXT NOT NULL CHECK (refund_state IN ('refund-requested', 'refund-settled')), audit_reference TEXT NOT NULL, created_at TEXT NOT NULL, PRIMARY KEY (invoice_id, mutation_key))",
  'CREATE TABLE IF NOT EXISTS billing_subject_versions (subject TEXT PRIMARY KEY, version INTEGER NOT NULL CHECK (version >= 0), last_mutation_token TEXT, updated_at TEXT NOT NULL)',
  "CREATE TABLE IF NOT EXISTS billing_mutation_outbox (request_key TEXT PRIMARY KEY, authority_subject TEXT, authority_version INTEGER CHECK (authority_version IS NULL OR authority_version >= 1), authority_token TEXT, mutation_kind TEXT NOT NULL, mutation_json TEXT NOT NULL, audit_state TEXT NOT NULL CHECK (audit_state IN ('pending', 'delivered')), audit_event_json TEXT NOT NULL, attempt_count INTEGER NOT NULL DEFAULT 0 CHECK (attempt_count >= 0), last_attempt_at TEXT, last_error TEXT, lease_token TEXT, lease_expires_at TEXT, created_at TEXT NOT NULL, updated_at TEXT NOT NULL)",
  REFUND_LEDGER_TOTAL_GUARD_SQL,
].join(';\n');

const SELECT_STATUS_ROW_COUNT_SQL = normalizeSql('SELECT COUNT(*) AS row_count FROM billing_status');
const SELECT_STATUS_BY_SUBJECT_SQL = normalizeSql('SELECT payload_json FROM billing_status WHERE subject = ?1 LIMIT 1');
const SELECT_ALL_STATUS_SQL = normalizeSql('SELECT payload_json FROM billing_status ORDER BY subject');
const SELECT_INVOICES_BY_SUBJECT_SQL = normalizeSql(
  'SELECT payload_json FROM billing_invoices WHERE subject = ?1 ORDER BY invoice_id'
);
const SELECT_INVOICE_BY_ID_SQL = normalizeSql(
  'SELECT payload_json FROM billing_invoices WHERE invoice_id = ?1 LIMIT 1'
);
const SELECT_INVOICE_SUBJECT_SQL = normalizeSql('SELECT subject FROM billing_invoices WHERE invoice_id = ?1 LIMIT 1');
const SELECT_REFUND_LEDGER_SUMMARY_SQL = normalizeSql(
  'SELECT COALESCE(SUM(amount_cents), 0) AS applied_amount_cents, (SELECT refund_state FROM billing_refund_ledger AS latest WHERE latest.invoice_id = ?1 ORDER BY created_at DESC, mutation_key DESC LIMIT 1) AS final_refund_state FROM billing_refund_ledger WHERE invoice_id = ?1'
);
const SELECT_SUBJECT_VERSION_SQL = normalizeSql(
  'SELECT subject, version, last_mutation_token, updated_at FROM billing_subject_versions WHERE subject = ?1 LIMIT 1'
);
const SELECT_MUTATION_OUTBOX_SQL = normalizeSql(
  'SELECT request_key, authority_subject, authority_version, authority_token, mutation_kind, mutation_json, audit_state, audit_event_json, attempt_count, last_attempt_at, last_error, lease_token, lease_expires_at, created_at, updated_at FROM billing_mutation_outbox WHERE request_key = ?1 LIMIT 1'
);
const SELECT_PENDING_MUTATION_OUTBOX_SQL = normalizeSql(
  "SELECT request_key, authority_subject, authority_version, authority_token, mutation_kind, mutation_json, audit_state, audit_event_json, attempt_count, last_attempt_at, last_error, lease_token, lease_expires_at, created_at, updated_at FROM billing_mutation_outbox WHERE audit_state = 'pending' AND attempt_count < ?2 AND (lease_expires_at IS NULL OR lease_expires_at <= ?3) ORDER BY created_at, request_key LIMIT ?1"
);
const SELECT_MANUAL_REVIEW_MUTATION_OUTBOX_SQL = normalizeSql(
  "SELECT COUNT(*) AS manual_review_count FROM billing_mutation_outbox WHERE audit_state = 'pending' AND attempt_count >= ?1"
);
const SELECT_MUTATION_OUTBOX_COLUMNS_SQL = 'PRAGMA table_info(billing_mutation_outbox)';
const SELECT_PROVIDER_EVENT_RECEIPT_COLUMNS_SQL = 'PRAGMA table_info(billing_provider_event_receipts)';
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
const SELECT_PROVIDER_EVENT_RECEIPT_SQL = normalizeSql(
  'SELECT provider, event_id, event_fingerprint, event_type, provider_occurred_at, provider_sequence, state_version, account_id, provider_customer_id, provider_subscription_id, provider_invoice_id, billing_subject, parent_account_ref, family_ref, billing_invoice_id, processing_state, queue_state, queue_attempt_count, last_queue_attempt_at, last_error, created_at, updated_at FROM billing_provider_event_receipts WHERE provider = ?1 AND event_id = ?2 LIMIT 1'
);
const INSERT_PROVIDER_EVENT_RECEIPT_SQL = normalizeSql(
  "INSERT INTO billing_provider_event_receipts (provider, event_id, event_fingerprint, event_type, provider_occurred_at, provider_sequence, state_version, account_id, provider_customer_id, provider_subscription_id, provider_invoice_id, billing_subject, parent_account_ref, family_ref, billing_invoice_id, processing_state, queue_state, queue_attempt_count, last_queue_attempt_at, last_error, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, 0, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, 'pending', 0, NULL, NULL, ?16, ?16)"
);
const UPDATE_PROVIDER_EVENT_RECEIPT_QUEUE_SQL = normalizeSql(
  "UPDATE billing_provider_event_receipts SET processing_state = ?6, queue_state = ?7, queue_attempt_count = queue_attempt_count + ?8, state_version = state_version + 1, last_queue_attempt_at = ?9, last_error = ?10, updated_at = ?9 WHERE provider = ?1 AND event_id = ?2 AND state_version = ?3 AND processing_state = ?4 AND queue_state = ?5 AND queue_attempt_count BETWEEN 0 AND 4294967294 AND state_version BETWEEN 0 AND 4294967294 AND NOT (queue_state IN ('delivered', 'manual-required', 'dead-letter') AND ?7 <> queue_state) AND NOT (processing_state IN ('applied', 'ignored', 'manual-required', 'dead-letter') AND ?6 <> processing_state) AND (?7 <> 'queued' OR queue_state = 'pending') AND (?7 <> 'delivered' OR queue_state IN ('queued', 'pending'))"
);
const SELECT_PROVIDER_EVENT_CURSOR_SQL = normalizeSql(
  'SELECT provider, billing_subject, last_occurred_at, last_sequence, last_event_id, state_version, updated_at FROM billing_provider_event_cursors WHERE provider = ?1 AND billing_subject = ?2 LIMIT 1'
);
const UPSERT_PROVIDER_EVENT_CURSOR_SQL = normalizeSql(
  'INSERT INTO billing_provider_event_cursors (provider, billing_subject, last_occurred_at, last_sequence, last_event_id, state_version, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, 1, ?6) ON CONFLICT(provider, billing_subject) DO UPDATE SET last_occurred_at = excluded.last_occurred_at, last_sequence = excluded.last_sequence, last_event_id = excluded.last_event_id, state_version = state_version + 1, updated_at = excluded.updated_at WHERE state_version = ?7 AND state_version BETWEEN 0 AND 4294967294'
);
const SELECT_PENDING_MUTATION_OUTBOX_COUNT_SQL = normalizeSql(
  "SELECT COUNT(*) AS row_count FROM billing_mutation_outbox WHERE audit_state = 'pending' AND attempt_count < ?1"
);
const SELECT_MANUAL_REVIEW_PROVIDER_RECEIPT_COUNT_SQL = normalizeSql(
  "SELECT COUNT(*) AS row_count FROM billing_provider_event_receipts WHERE queue_state IN ('manual-required', 'dead-letter')"
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
const INSERT_SUBJECT_VERSION_SQL = normalizeSql(
  'INSERT OR IGNORE INTO billing_subject_versions (subject, version, last_mutation_token, updated_at) VALUES (?1, 0, NULL, ?2)'
);
const ADVANCE_SUBJECT_VERSION_SQL = normalizeSql(
  'UPDATE billing_subject_versions SET version = version + 1, last_mutation_token = ?3, updated_at = ?4 WHERE subject = ?1 AND version = ?2'
);
const INSERT_REFUND_LEDGER_SQL = normalizeSql(
  'INSERT INTO billing_refund_ledger (invoice_id, mutation_key, subject, amount_cents, invoice_total_cents, refund_state, audit_reference, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)'
);
const INSERT_MUTATION_OUTBOX_SQL = normalizeSql(
  'INSERT INTO billing_mutation_outbox (request_key, authority_subject, authority_version, authority_token, mutation_kind, mutation_json, audit_state, audit_event_json, attempt_count, last_attempt_at, last_error, lease_token, lease_expires_at, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, 0, NULL, NULL, NULL, NULL, ?9, ?10)'
);
const MARK_MUTATION_OUTBOX_ATTEMPT_SQL = normalizeSql(
  "UPDATE billing_mutation_outbox SET attempt_count = attempt_count + 1, last_attempt_at = ?2, last_error = NULL, lease_token = ?3, lease_expires_at = ?4, updated_at = ?2 WHERE request_key = ?1 AND audit_state = 'pending' AND attempt_count < ?5 AND (lease_expires_at IS NULL OR lease_expires_at <= ?2)"
);
const MARK_MUTATION_OUTBOX_FAILURE_SQL = normalizeSql(
  "UPDATE billing_mutation_outbox SET last_error = ?2, lease_token = NULL, lease_expires_at = NULL, updated_at = ?3 WHERE request_key = ?1 AND audit_state = 'pending' AND lease_token = ?4"
);
const MARK_MUTATION_AUDIT_DELIVERED_SQL = normalizeSql(
  "UPDATE billing_mutation_outbox SET audit_state = 'delivered', last_error = NULL, lease_token = NULL, lease_expires_at = NULL, updated_at = ?2 WHERE request_key = ?1 AND audit_state = 'pending' AND lease_token = ?3"
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

interface RefundLedgerSummaryRow {
  applied_amount_cents: number | string;
  final_refund_state: string | null;
}

interface SubjectVersionRow {
  subject: string;
  version: number | string;
  last_mutation_token: string | null;
  updated_at: string;
}

interface MutationOutboxRow {
  request_key: string;
  authority_subject: string | null;
  authority_version: number | string | null;
  authority_token: string | null;
  mutation_kind: string;
  mutation_json: string;
  audit_state: string;
  audit_event_json: string;
  attempt_count: number | string;
  last_attempt_at: string | null;
  last_error: string | null;
  lease_token: string | null;
  lease_expires_at: string | null;
  created_at: string;
  updated_at: string;
}

interface MutationOutboxColumnRow {
  name: string;
}

interface ManualReviewOutboxCountRow {
  manual_review_count: number | string;
}

interface ProviderEventReceiptRow {
  provider: string;
  event_id: string;
  event_fingerprint: string;
  event_type: string;
  provider_occurred_at: string | null;
  provider_sequence: number | string | null;
  state_version: number | string;
  account_id: string | null;
  provider_customer_id: string | null;
  provider_subscription_id: string | null;
  provider_invoice_id: string | null;
  billing_subject: string | null;
  parent_account_ref: string | null;
  family_ref: string | null;
  billing_invoice_id: string | null;
  processing_state: string;
  queue_state: string;
  queue_attempt_count: number | string;
  last_queue_attempt_at: string | null;
  last_error: string | null;
  created_at: string;
  updated_at: string;
}

interface ProviderEventCursorRow {
  provider: string;
  billing_subject: string;
  last_occurred_at: string | null;
  last_sequence: number | string | null;
  last_event_id: string;
  state_version: number | string;
  updated_at: string;
}

export interface ProviderEventReceipt {
  provider: string;
  eventId: string;
  eventFingerprint: string;
  eventType: string;
  providerOccurredAt: string | null;
  providerSequence: number | null;
  stateVersion: number;
  accountId: string | null;
  providerCustomerId: string | null;
  providerSubscriptionId: string | null;
  providerInvoiceId: string | null;
  billingSubject: string | null;
  parentAccountRef: string | null;
  familyRef: string | null;
  billingInvoiceId: string | null;
  processingState: 'received' | 'ignored' | 'queued' | 'applied' | 'manual-required' | 'dead-letter';
  queueState: 'pending' | 'queued' | 'delivered' | 'manual-required' | 'dead-letter';
  queueAttemptCount: number;
  lastQueueAttemptAt: string | null;
  lastError: string | null;
  createdAt: string;
  updatedAt: string;
}

function providerQueueStateRank(state: ProviderEventReceipt['queueState']): number {
  switch (state) {
    case 'pending':
      return 0;
    case 'queued':
      return 1;
    case 'delivered':
    case 'manual-required':
    case 'dead-letter':
      return 2;
  }
}

function providerProcessingStateTerminal(state: ProviderEventReceipt['processingState']): boolean {
  return state === 'applied' || state === 'ignored' || state === 'manual-required' || state === 'dead-letter';
}

function assertProviderEventReceiptTransition(
  current: ProviderEventReceipt,
  nextQueueState: ProviderEventReceipt['queueState'],
  nextProcessingState: ProviderEventReceipt['processingState']
): void {
  if (
    current.queueState !== nextQueueState &&
    providerQueueStateRank(nextQueueState) < providerQueueStateRank(current.queueState)
  ) {
    throw new BillingReadModelUnavailableError('billing-provider-event-queue-state-regression');
  }
  if (current.queueState !== nextQueueState && providerQueueStateRank(current.queueState) === 2) {
    throw new BillingReadModelUnavailableError('billing-provider-event-queue-state-terminal');
  }
  if (providerProcessingStateTerminal(current.processingState) && current.processingState !== nextProcessingState) {
    throw new BillingReadModelUnavailableError('billing-provider-event-processing-state-terminal');
  }
  if (nextQueueState === 'queued' && current.queueState !== 'pending') {
    throw new BillingReadModelUnavailableError('billing-provider-event-queue-state-transition-invalid');
  }
  if (nextQueueState === 'delivered' && current.queueState !== 'queued' && current.queueState !== 'pending') {
    throw new BillingReadModelUnavailableError('billing-provider-event-queue-state-transition-invalid');
  }
  if (nextQueueState === 'delivered' && nextProcessingState !== 'applied' && nextProcessingState !== 'ignored') {
    throw new BillingReadModelUnavailableError('billing-provider-event-delivery-state-mismatch');
  }
  if (
    (nextQueueState === 'manual-required' || nextQueueState === 'dead-letter') &&
    nextProcessingState !== nextQueueState &&
    nextProcessingState !== 'ignored' &&
    nextProcessingState !== 'applied' &&
    nextProcessingState !== 'manual-required'
  ) {
    throw new BillingReadModelUnavailableError('billing-provider-event-terminal-state-mismatch');
  }
  if (
    (nextProcessingState === 'applied' || nextProcessingState === 'ignored') &&
    nextQueueState !== 'delivered' &&
    nextQueueState !== 'manual-required' &&
    nextQueueState !== 'dead-letter'
  ) {
    throw new BillingReadModelUnavailableError('billing-provider-event-processing-state-mismatch');
  }
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

function decodeNullableTimestamp(value: unknown, scope: string): string | null {
  return value === null || value === undefined ? null : decodeTimestamp(value, scope);
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

function decodeBillingInvoiceSummary(value: unknown, scope: string): BillingInvoiceSummary {
  const record = payloadRecord(value, scope);
  const generated = decodeCanonicalValue(scope, () =>
    BillingSupportAdminInvoiceSummarySchema.parse({
      ...record,
      manualRequired: Object.prototype.hasOwnProperty.call(record, 'manualRequired') ? record.manualRequired : false,
    })
  );
  const subtotalCents = decodeBillingCount(generated.subtotalCents, `${scope}-subtotal-cents`);
  const taxCents = decodeBillingCount(generated.taxCents, `${scope}-tax-cents`);
  const totalCents = decodeBillingCount(generated.totalCents, `${scope}-total-cents`);
  if (decodeBillingCount(subtotalCents + taxCents, `${scope}-subtotal-tax-total`) !== totalCents) {
    throw new BillingReadModelUnavailableError(`${scope}-total-mismatch`);
  }

  return {
    invoiceId: decodeNonEmptyString(generated.invoiceId, `${scope}-invoice-id`),
    invoiceNumber: decodeNonEmptyString(generated.invoiceNumber, `${scope}-invoice-number`),
    parentAccountRef: decodeNonEmptyString(generated.parentAccountRef, `${scope}-parent-account-ref`),
    familyRef: decodeNonEmptyString(generated.familyRef, `${scope}-family-ref`),
    planId: decodeCanonicalValue(scope, () => BillingPlanIdSchema.parse(generated.planId)),
    currency: decodeLiteral(generated.currency, ['USD'] as const, `${scope}-currency`),
    subtotalCents,
    taxCents,
    totalCents,
    invoiceVisibility: decodeLiteral(
      generated.invoiceVisibility,
      ['customer-portal-hosted', 'manual-support-required'] as const,
      `${scope}-invoice-visibility`
    ),
    paymentState: decodeLiteral(
      generated.paymentState,
      ['paid', 'grace', 'unpaid', 'refunded'] as const,
      `${scope}-payment-state`
    ),
    provider: decodeLiteral(generated.provider, ['stripe', 'manual-invoice'] as const, `${scope}-provider`),
    hostedUrl: generated.hostedUrl === null ? null : decodeNonEmptyString(generated.hostedUrl, `${scope}-hosted-url`),
    periodStart: decodeTimestamp(generated.periodStart, `${scope}-period-start`),
    periodEnd: decodeTimestamp(generated.periodEnd, `${scope}-period-end`),
    updatedAt: decodeTimestamp(generated.updatedAt, `${scope}-updated-at`),
    auditReference: decodeCanonicalValue(scope, () => BillingAuditReferenceSchema.parse(generated.auditReference)),
  };
}

function decodeAdminBillingInvoiceSummary(value: unknown, scope: string): AdminBillingInvoiceSummary {
  const generated = decodeCanonicalValue(scope, () => BillingSupportAdminInvoiceSummarySchema.parse(value));
  const invoice = decodeBillingInvoiceSummary({ ...generated, manualRequired: false }, scope);
  return {
    ...invoice,
    manualRequired: decodeBoolean(generated.manualRequired, `${scope}-manual-required`),
  };
}

function decodeAdminBillingAccountSummary(value: unknown, scope: string): AdminBillingAccountSummary {
  return decodeCanonicalValue(scope, () => BillingSupportAdminAccountSummarySchema.parse(value));
}

function decodeAdminBillingDisputeSummary(value: unknown, scope: string): AdminBillingDisputeSummary {
  return decodeCanonicalValue(scope, () => BillingSupportAdminDisputeSummarySchema.parse(value));
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
  if (source === 'signed-local-snapshot') {
    throw new BillingReadModelUnavailableError(`${scope}-verifier-authority-unavailable`);
  }
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
  const manualAuthority = snapshot.source === 'manual-admin-review' && snapshot.signatureState === 'manual-required';
  if (snapshot.source === 'signed-local-snapshot' || snapshot.signatureState === 'signed') {
    throw new BillingReadModelUnavailableError(`${scope}-verifier-authority-unavailable`);
  }
  if (!manualAuthority) {
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
    (snapshot.subscriptionStatus === 'active' &&
      (snapshot.parentVisibleState !== 'available' || snapshot.localSafetyBehavior !== 'unchanged')) ||
    (snapshot.subscriptionStatus === 'grace' &&
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

type BillingAuthorityState = 'manual-review';

function correlateBillingAuthority(
  status: BillingStatusSummary,
  snapshot: BillingEntitlementSnapshotSummary,
  scope: string
): BillingAuthorityState {
  if (
    status.subject !== snapshot.subject ||
    status.parentAccountRef !== snapshot.parentAccountRef ||
    status.familyRef !== snapshot.familyRef ||
    status.plan.planId !== snapshot.planId ||
    status.plan.deviceLimit !== snapshot.deviceLimit ||
    status.deviceUsage.activeDevices !== snapshot.activeDevices ||
    status.deviceUsage.trustedDevices !== snapshot.trustedDevices ||
    status.subscriptionStatus !== snapshot.subscriptionStatus ||
    status.source !== snapshot.source ||
    status.parentVisibleState !== snapshot.parentVisibleState ||
    status.localSafetyBehavior !== snapshot.localSafetyBehavior ||
    JSON.stringify(status.failureState) !== JSON.stringify(snapshot.failureState)
  ) {
    throw new BillingReadModelUnavailableError(`${scope}-mismatch`);
  }

  return 'manual-review';
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
  if (
    decodedStatus.accountStatus !== 'manual-review' ||
    decodedStatus.source !== 'manual-admin-review' ||
    decodedSnapshot.source !== 'manual-admin-review' ||
    decodedSnapshot.signatureState !== 'manual-required'
  ) {
    throw new BillingReadModelUnavailableError(`${scope}-verifier-authority-unavailable`);
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
      actorSubject: string;
      requestId: string;
      invoiceId: string;
      currency: string;
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
      providerOccurredAt: string | null;
      providerSequence: number | null;
      providerCursorExpectedVersion: number;
      disputeId?: string | null;
      invoiceId?: string | null;
      parentAccountRef?: string | null;
      familyRef?: string | null;
    };

interface BillingRefundLedgerEntry {
  invoiceId: string;
  mutationKey: string;
  subject: string;
  amountCents: number;
  invoiceTotalCents: number;
  refundState: 'refund-requested' | 'refund-settled';
  auditReference: string;
  createdAt: string;
}

type MutationAuditState = 'pending' | 'delivered';

interface LocalMutationOutboxEntry {
  requestKey: string;
  authoritySubject: string | null;
  authorityVersion: number | null;
  authorityToken: string | null;
  mutationKind: BillingStateMutation['kind'];
  mutationJson: string;
  auditState: MutationAuditState;
  auditJson: string;
  attemptCount: number;
  lastAttemptAt: string | null;
  lastError: string | null;
  leaseToken: string | null;
  leaseExpiresAt: string | null;
  createdAt: string;
  updatedAt: string;
}

interface LocalBillingD1State {
  statusBySubject: Map<string, BillingStatusSummary>;
  invoicesBySubject: Map<string, ReadonlyArray<BillingInvoiceSummary>>;
  referralsBySubject: Map<string, BillingReferralSummary>;
  snapshotsBySubject: Map<string, BillingEntitlementSnapshotSummary>;
  adminAccounts: ReadonlyArray<AdminBillingAccountSummary>;
  adminInvoices: ReadonlyArray<AdminBillingInvoiceSummary>;
  adminDisputes: ReadonlyArray<AdminBillingDisputeSummary>;
  adminReferrals: ReadonlyArray<AdminBillingReferralSummary>;
  refundLedgerByInvoice: Map<string, ReadonlyArray<BillingRefundLedgerEntry>>;
  providerEventReceipts: Map<string, ProviderEventReceipt>;
  providerEventCursors: Map<
    string,
    {
      provider: string;
      billingSubject: string;
      lastOccurredAt: string | null;
      lastSequence: number | null;
      lastEventId: string;
      stateVersion: number;
      updatedAt: string;
    }
  >;
  subjectVersions: Map<string, { version: number; lastMutationToken: string | null; updatedAt: string }>;
  mutationOutbox: Map<string, LocalMutationOutboxEntry>;
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

  async run(): Promise<{ results: ReadonlyArray<never>; success: true; meta: { changes: number } }> {
    this.executeMutation();
    return {
      results: [],
      success: true,
      meta: { changes: 1 },
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
      case SELECT_ALL_STATUS_SQL:
        return Array.from(this.state.statusBySubject.values()).map(
          (status) =>
            ({
              payload_json: JSON.stringify(status),
            }) as T
        );
      case SELECT_INVOICES_BY_SUBJECT_SQL: {
        const subject = String(this.values[0] ?? '');
        return (this.state.invoicesBySubject.get(subject) ?? []).map(
          (invoice) => ({ payload_json: JSON.stringify(invoice) }) as T
        );
      }
      case SELECT_INVOICE_BY_ID_SQL: {
        const invoiceId = String(this.values[0] ?? '');
        for (const invoices of this.state.invoicesBySubject.values()) {
          const invoice = invoices.find((entry) => entry.invoiceId === invoiceId);
          if (invoice) {
            return [{ payload_json: JSON.stringify(invoice) } as T];
          }
        }
        return [];
      }
      case SELECT_REFUND_LEDGER_SUMMARY_SQL: {
        const invoiceId = String(this.values[0] ?? '');
        const entries = this.state.refundLedgerByInvoice.get(invoiceId) ?? [];
        const appliedAmount = entries.reduce((total, entry) => total + entry.amountCents, 0);
        return [
          {
            applied_amount_cents: appliedAmount,
            final_refund_state: entries[entries.length - 1]?.refundState ?? null,
          } as T,
        ];
      }
      case SELECT_SUBJECT_VERSION_SQL: {
        const subject = String(this.values[0] ?? '');
        const version = this.state.subjectVersions.get(subject);
        return version
          ? [
              {
                subject,
                version: version.version,
                last_mutation_token: version.lastMutationToken,
                updated_at: version.updatedAt,
              } as T,
            ]
          : [];
      }
      case SELECT_MUTATION_OUTBOX_SQL:
      case SELECT_PENDING_MUTATION_OUTBOX_SQL: {
        const requestKey = String(this.values[0] ?? '');
        const entries =
          this.normalizedQuery === SELECT_PENDING_MUTATION_OUTBOX_SQL
            ? Array.from(this.state.mutationOutbox.values())
                .filter(
                  (entry) =>
                    entry.auditState === 'pending' &&
                    entry.attemptCount < Number(this.values[1] ?? MAX_BILLING_OUTBOX_ATTEMPTS) &&
                    (entry.leaseExpiresAt === null || entry.leaseExpiresAt <= String(this.values[2] ?? ''))
                )
                .sort((left, right) =>
                  `${left.createdAt}:${left.requestKey}`.localeCompare(`${right.createdAt}:${right.requestKey}`)
                )
                .slice(0, Math.max(0, Number(this.values[0] ?? 0)))
            : requestKey.length > 0 && this.state.mutationOutbox.has(requestKey)
              ? [this.state.mutationOutbox.get(requestKey)!]
              : [];
        return entries.map(
          (entry) =>
            ({
              request_key: entry.requestKey,
              authority_subject: entry.authoritySubject,
              authority_version: entry.authorityVersion,
              authority_token: entry.authorityToken,
              mutation_kind: entry.mutationKind,
              mutation_json: entry.mutationJson,
              audit_state: entry.auditState,
              audit_event_json: entry.auditJson,
              attempt_count: entry.attemptCount,
              last_attempt_at: entry.lastAttemptAt,
              last_error: entry.lastError,
              lease_token: entry.leaseToken,
              lease_expires_at: entry.leaseExpiresAt,
              created_at: entry.createdAt,
              updated_at: entry.updatedAt,
            }) as T
        );
      }
      case SELECT_MANUAL_REVIEW_MUTATION_OUTBOX_SQL: {
        const maxAttempts = Number(this.values[0] ?? MAX_BILLING_OUTBOX_ATTEMPTS);
        return [
          {
            manual_review_count: Array.from(this.state.mutationOutbox.values()).filter(
              (entry) => entry.auditState === 'pending' && entry.attemptCount >= maxAttempts
            ).length,
          } as T,
        ];
      }
      case SELECT_PENDING_MUTATION_OUTBOX_COUNT_SQL:
        return [
          {
            row_count: Array.from(this.state.mutationOutbox.values()).filter(
              (entry) =>
                entry.auditState === 'pending' &&
                entry.attemptCount < Number(this.values[0] ?? MAX_BILLING_OUTBOX_ATTEMPTS)
            ).length,
          } as T,
        ];
      case SELECT_MANUAL_REVIEW_PROVIDER_RECEIPT_COUNT_SQL:
        return [
          {
            row_count: Array.from(this.state.providerEventReceipts.values()).filter(
              (entry) => entry.queueState === 'manual-required' || entry.queueState === 'dead-letter'
            ).length,
          } as T,
        ];
      case SELECT_PROVIDER_EVENT_RECEIPT_SQL: {
        const key = `${String(this.values[0] ?? '')}:${String(this.values[1] ?? '')}`;
        const receipt = this.state.providerEventReceipts.get(key);
        if (!receipt) {
          return [];
        }
        return [
          {
            provider: receipt.provider,
            event_id: receipt.eventId,
            event_fingerprint: receipt.eventFingerprint,
            event_type: receipt.eventType,
            provider_occurred_at: receipt.providerOccurredAt,
            provider_sequence: receipt.providerSequence,
            state_version: receipt.stateVersion,
            account_id: receipt.accountId,
            provider_customer_id: receipt.providerCustomerId,
            provider_subscription_id: receipt.providerSubscriptionId,
            provider_invoice_id: receipt.providerInvoiceId,
            billing_subject: receipt.billingSubject,
            parent_account_ref: receipt.parentAccountRef,
            family_ref: receipt.familyRef,
            billing_invoice_id: receipt.billingInvoiceId,
            processing_state: receipt.processingState,
            queue_state: receipt.queueState,
            queue_attempt_count: receipt.queueAttemptCount,
            last_queue_attempt_at: receipt.lastQueueAttemptAt,
            last_error: receipt.lastError,
            created_at: receipt.createdAt,
            updated_at: receipt.updatedAt,
          } as T,
        ];
      }
      case SELECT_PROVIDER_EVENT_CURSOR_SQL: {
        const key = `${String(this.values[0] ?? '')}:${String(this.values[1] ?? '')}`;
        const cursor = this.state.providerEventCursors.get(key);
        return cursor
          ? [
              {
                provider: cursor.provider,
                billing_subject: cursor.billingSubject,
                last_occurred_at: cursor.lastOccurredAt,
                last_sequence: cursor.lastSequence,
                last_event_id: cursor.lastEventId,
                state_version: cursor.stateVersion,
                updated_at: cursor.updatedAt,
              } as T,
            ]
          : [];
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
      case INSERT_SUBJECT_VERSION_SQL: {
        const subject = decodeNonEmptyString(this.values[0], 'billing-subject-version-subject');
        const updatedAt = decodeTimestamp(this.values[1], 'billing-subject-version-updated-at');
        if (!this.state.subjectVersions.has(subject)) {
          this.state.subjectVersions.set(subject, { version: 0, lastMutationToken: null, updatedAt });
        }
        return;
      }
      case ADVANCE_SUBJECT_VERSION_SQL: {
        const subject = decodeNonEmptyString(this.values[0], 'billing-subject-version-subject');
        const expectedVersion = decodeBillingCount(this.values[1], 'billing-subject-version-expected');
        const mutationToken = decodeNonEmptyString(this.values[2], 'billing-subject-version-token');
        const updatedAt = decodeTimestamp(this.values[3], 'billing-subject-version-updated-at');
        const current = this.state.subjectVersions.get(subject);
        if (!current || current.version !== expectedVersion) {
          throw new BillingReadModelUnavailableError('billing-mutation-authority-cas-failed');
        }
        this.state.subjectVersions.set(subject, {
          version: incrementBillingCount(current.version, 'billing-subject-version-next'),
          lastMutationToken: mutationToken,
          updatedAt,
        });
        return;
      }
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
        const invoice = decodeBillingInvoiceSummary(
          parseUnknownPayload(payloadJson, `billing-invoice:${subject}`),
          `billing-invoice:${subject}`
        );
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
        const nextRow = decodeAdminBillingAccountSummary(
          parseUnknownPayload(payloadJson, 'billing-admin-account'),
          'billing-admin-account'
        );
        this.state.adminAccounts = replaceByKey(this.state.adminAccounts, nextRow, (entry) => entry.parentAccountRef);
        return;
      }
      case UPSERT_ADMIN_INVOICE_SQL: {
        const payloadJson = String(this.values[1] ?? '{}');
        const nextRow = decodeAdminBillingInvoiceSummary(
          parseUnknownPayload(payloadJson, 'billing-admin-invoice'),
          'billing-admin-invoice'
        );
        this.state.adminInvoices = replaceByKey(this.state.adminInvoices, nextRow, (entry) => entry.invoiceId);
        return;
      }
      case UPSERT_ADMIN_DISPUTE_SQL: {
        const payloadJson = String(this.values[1] ?? '{}');
        const nextRow = decodeAdminBillingDisputeSummary(
          parseUnknownPayload(payloadJson, 'billing-admin-dispute'),
          'billing-admin-dispute'
        );
        this.state.adminDisputes = replaceByKey(this.state.adminDisputes, nextRow, (entry) => entry.disputeId);
        return;
      }
      case UPSERT_ADMIN_REFERRAL_SQL: {
        const payloadJson = String(this.values[1] ?? '{}');
        const nextRow = parsePayload<AdminBillingReferralSummary>(payloadJson);
        this.state.adminReferrals = replaceByKey(this.state.adminReferrals, nextRow, (entry) => entry.referralCode);
        return;
      }
      case INSERT_REFUND_LEDGER_SQL: {
        const invoiceId = decodeNonEmptyString(this.values[0], 'billing-refund-ledger-invoice-id');
        const mutationKey = decodeNonEmptyString(this.values[1], 'billing-refund-ledger-mutation-key');
        const subject = decodeNonEmptyString(this.values[2], 'billing-refund-ledger-subject');
        const amountCents = decodeBillingCount(this.values[3], 'billing-refund-ledger-amount-cents');
        const invoiceTotalCents = decodeBillingCount(this.values[4], 'billing-refund-ledger-invoice-total-cents');
        const refundState = decodeLiteral(
          this.values[5],
          ['refund-requested', 'refund-settled'] as const,
          'billing-refund-ledger-state'
        );
        const auditReference = decodeCanonicalValue('billing-refund-ledger-audit-reference', () =>
          BillingAuditReferenceSchema.parse(this.values[6])
        );
        const createdAt = decodeTimestamp(this.values[7], 'billing-refund-ledger-created-at');
        const current = this.state.refundLedgerByInvoice.get(invoiceId) ?? [];
        if (current.some((entry) => entry.mutationKey === mutationKey)) {
          throw new BillingReadModelUnavailableError('billing-refund-ledger-duplicate');
        }
        if (current.some((entry) => entry.invoiceTotalCents !== invoiceTotalCents)) {
          throw new BillingReadModelUnavailableError('billing-refund-ledger-total-mismatch');
        }
        const cumulativeAmountCents = decodeBillingCount(
          current.reduce((total, entry) => total + entry.amountCents, 0) + amountCents,
          'billing-refund-ledger-cumulative-amount-cents'
        );
        if (cumulativeAmountCents > invoiceTotalCents) {
          throw new BillingReadModelUnavailableError('billing-refund-ledger-total-exceeded');
        }
        this.state.refundLedgerByInvoice.set(invoiceId, [
          ...current,
          { invoiceId, mutationKey, subject, amountCents, invoiceTotalCents, refundState, auditReference, createdAt },
        ]);
        return;
      }
      case INSERT_PROVIDER_EVENT_RECEIPT_SQL: {
        const provider = decodeNonEmptyString(this.values[0], 'billing-provider-event-provider');
        const eventId = decodeNonEmptyString(this.values[1], 'billing-provider-event-id');
        const key = `${provider}:${eventId}`;
        if (this.state.providerEventReceipts.has(key)) {
          throw new BillingReadModelUnavailableError('billing-provider-event-receipt-duplicate');
        }
        const processingState = decodeLiteral(
          this.values[14],
          ['received', 'ignored', 'queued', 'applied', 'manual-required', 'dead-letter'] as const,
          'billing-provider-event-processing-state'
        );
        const createdAt = decodeTimestamp(this.values[15], 'billing-provider-event-created');
        this.state.providerEventReceipts.set(key, {
          provider,
          eventId,
          eventFingerprint: decodeNonEmptyString(this.values[2], 'billing-provider-event-fingerprint'),
          eventType: decodeNonEmptyString(this.values[3], 'billing-provider-event-type'),
          providerOccurredAt:
            this.values[4] === null ? null : decodeTimestamp(this.values[4], 'billing-provider-event-occurred-at'),
          providerSequence:
            this.values[5] === null ? null : decodeBillingCount(this.values[5], 'billing-provider-event-sequence'),
          stateVersion: 0,
          accountId:
            this.values[6] === null ? null : decodeNonEmptyString(this.values[6], 'billing-provider-event-account'),
          providerCustomerId:
            this.values[7] === null ? null : decodeNonEmptyString(this.values[7], 'billing-provider-event-customer'),
          providerSubscriptionId:
            this.values[8] === null
              ? null
              : decodeNonEmptyString(this.values[8], 'billing-provider-event-subscription'),
          providerInvoiceId:
            this.values[9] === null ? null : decodeNonEmptyString(this.values[9], 'billing-provider-event-invoice'),
          billingSubject:
            this.values[10] === null ? null : decodeNonEmptyString(this.values[10], 'billing-provider-event-subject'),
          parentAccountRef:
            this.values[11] === null
              ? null
              : decodeNonEmptyString(this.values[11], 'billing-provider-event-parent-account'),
          familyRef:
            this.values[12] === null ? null : decodeNonEmptyString(this.values[12], 'billing-provider-event-family'),
          billingInvoiceId:
            this.values[13] === null
              ? null
              : decodeNonEmptyString(this.values[13], 'billing-provider-event-billing-invoice'),
          processingState,
          queueState: 'pending',
          queueAttemptCount: 0,
          lastQueueAttemptAt: null,
          lastError: null,
          createdAt,
          updatedAt: createdAt,
        });
        return;
      }
      case UPDATE_PROVIDER_EVENT_RECEIPT_QUEUE_SQL: {
        const provider = decodeNonEmptyString(this.values[0], 'billing-provider-event-provider');
        const eventId = decodeNonEmptyString(this.values[1], 'billing-provider-event-id');
        const key = `${provider}:${eventId}`;
        const current = this.state.providerEventReceipts.get(key);
        if (!current) {
          throw new BillingReadModelUnavailableError('billing-provider-event-receipt-missing');
        }
        const expectedStateVersion = decodeBillingCount(
          this.values[2],
          'billing-provider-event-expected-state-version'
        );
        const expectedProcessingState = decodeLiteral(
          this.values[3],
          ['received', 'ignored', 'queued', 'applied', 'manual-required', 'dead-letter'] as const,
          'billing-provider-event-expected-processing-state'
        );
        const expectedQueueState = decodeLiteral(
          this.values[4],
          ['pending', 'queued', 'delivered', 'manual-required', 'dead-letter'] as const,
          'billing-provider-event-expected-queue-state'
        );
        if (
          current.stateVersion !== expectedStateVersion ||
          current.processingState !== expectedProcessingState ||
          current.queueState !== expectedQueueState
        ) {
          throw new BillingReadModelUnavailableError('billing-provider-event-state-cas-failed');
        }
        const nextProcessingState = decodeLiteral(
          this.values[5],
          ['received', 'ignored', 'queued', 'applied', 'manual-required', 'dead-letter'] as const,
          'billing-provider-event-processing-state'
        );
        const nextQueueState = decodeLiteral(
          this.values[6],
          ['pending', 'queued', 'delivered', 'manual-required', 'dead-letter'] as const,
          'billing-provider-event-queue-state'
        );
        assertProviderEventReceiptTransition(current, nextQueueState, nextProcessingState);
        const updatedAt = decodeTimestamp(this.values[8], 'billing-provider-event-updated');
        this.state.providerEventReceipts.set(key, {
          ...current,
          processingState: nextProcessingState,
          queueState: nextQueueState,
          queueAttemptCount: decodeBillingCount(
            current.queueAttemptCount + Number(this.values[7] ?? 0),
            'billing-provider-event-attempts'
          ),
          stateVersion: incrementBillingCount(current.stateVersion, 'billing-provider-event-state-version'),
          lastQueueAttemptAt: updatedAt,
          lastError:
            this.values[9] === null ? null : decodeNonEmptyString(this.values[9], 'billing-provider-event-error'),
          updatedAt,
        });
        return;
      }
      case UPSERT_PROVIDER_EVENT_CURSOR_SQL: {
        const provider = decodeNonEmptyString(this.values[0], 'billing-provider-event-cursor-provider');
        const billingSubject = decodeNonEmptyString(this.values[1], 'billing-provider-event-cursor-subject');
        const key = `${provider}:${billingSubject}`;
        const current = this.state.providerEventCursors.get(key);
        const expectedVersion = decodeBillingCount(this.values[6], 'billing-provider-event-cursor-expected-version');
        if (current && current.stateVersion !== expectedVersion) {
          throw new BillingReadModelUnavailableError('billing-provider-event-cursor-cas-failed');
        }
        if (!current && expectedVersion !== 0) {
          throw new BillingReadModelUnavailableError('billing-provider-event-cursor-cas-failed');
        }
        const updatedAt = decodeTimestamp(this.values[5], 'billing-provider-event-cursor-updated');
        this.state.providerEventCursors.set(key, {
          provider,
          billingSubject,
          lastOccurredAt:
            this.values[2] === null
              ? null
              : decodeTimestamp(this.values[2], 'billing-provider-event-cursor-occurred-at'),
          lastSequence:
            this.values[3] === null
              ? null
              : decodeBillingCount(this.values[3], 'billing-provider-event-cursor-sequence'),
          lastEventId: decodeNonEmptyString(this.values[4], 'billing-provider-event-cursor-event-id'),
          stateVersion: incrementBillingCount(expectedVersion, 'billing-provider-event-cursor-next-version'),
          updatedAt,
        });
        return;
      }
      case INSERT_MUTATION_OUTBOX_SQL: {
        const requestKey = decodeNonEmptyString(this.values[0], 'billing-mutation-outbox-request-key');
        const authoritySubject = decodeNonEmptyString(this.values[1], 'billing-mutation-outbox-authority-subject');
        const authorityVersion = decodeBillingCount(this.values[2], 'billing-mutation-outbox-authority-version');
        if (authorityVersion < 1) {
          throw new BillingReadModelUnavailableError('billing-mutation-outbox-authority-version-invalid');
        }
        const authorityToken = decodeNonEmptyString(this.values[3], 'billing-mutation-outbox-authority-token');
        const currentAuthority = this.state.subjectVersions.get(authoritySubject);
        if (
          !currentAuthority ||
          currentAuthority.version !== authorityVersion ||
          currentAuthority.lastMutationToken !== authorityToken
        ) {
          throw new BillingReadModelUnavailableError('billing-mutation-authority-cas-failed');
        }
        const mutationKind = decodeNonEmptyString(this.values[4], 'billing-mutation-outbox-kind');
        const mutationJson = decodeNonEmptyString(this.values[5], 'billing-mutation-outbox-mutation');
        const mutationSubject = decodeNonEmptyString(
          payloadRecord(
            parseUnknownPayload(mutationJson, 'billing-mutation-outbox-mutation'),
            'billing-mutation-outbox-mutation'
          ).subject,
          'billing-mutation-outbox-mutation-subject'
        );
        if (mutationSubject !== authoritySubject) {
          throw new BillingReadModelUnavailableError('billing-mutation-authority-subject-mismatch');
        }
        if (mutationKind === 'provider-webhook') {
          const providerMutation = payloadRecord(
            parseUnknownPayload(mutationJson, 'billing-provider-event-mutation'),
            'billing-provider-event-mutation'
          );
          const provider = decodeNonEmptyString(providerMutation.provider, 'billing-provider-event-provider');
          const eventId = decodeNonEmptyString(providerMutation.eventId, 'billing-provider-event-id');
          const expectedCursorVersion = decodeBillingCount(
            providerMutation.providerCursorExpectedVersion,
            'billing-provider-event-cursor-expected-version'
          );
          const cursor = this.state.providerEventCursors.get(`${provider}:${authoritySubject}`);
          if (!cursor || cursor.stateVersion !== expectedCursorVersion + 1 || cursor.lastEventId !== eventId) {
            throw new BillingReadModelUnavailableError('billing-provider-event-cursor-cas-failed');
          }
        }
        const auditState = decodeLiteral(
          this.values[6],
          ['pending', 'delivered'] as const,
          'billing-mutation-outbox-audit-state'
        );
        const auditJson = decodeNonEmptyString(this.values[7], 'billing-mutation-outbox-audit-event');
        const createdAt = decodeTimestamp(this.values[8], 'billing-mutation-outbox-created-at');
        const updatedAt = decodeTimestamp(this.values[9], 'billing-mutation-outbox-updated-at');
        decodeCanonicalValue('billing-mutation-outbox-audit-event', () =>
          BillingSupportAdminAuditEventSummarySchema.parse(
            parseUnknownPayload(auditJson, 'billing-mutation-outbox-audit-event')
          )
        );
        if (this.state.mutationOutbox.has(requestKey)) {
          throw new BillingReadModelUnavailableError('billing-mutation-outbox-duplicate');
        }
        this.state.mutationOutbox.set(requestKey, {
          requestKey,
          authoritySubject,
          authorityVersion,
          authorityToken,
          mutationKind: mutationKind as BillingStateMutation['kind'],
          mutationJson,
          auditState,
          auditJson,
          attemptCount: 0,
          lastAttemptAt: null,
          lastError: null,
          leaseToken: null,
          leaseExpiresAt: null,
          createdAt,
          updatedAt,
        });
        return;
      }
      case MARK_MUTATION_AUDIT_DELIVERED_SQL: {
        const requestKey = decodeNonEmptyString(this.values[0], 'billing-mutation-outbox-request-key');
        const updatedAt = decodeTimestamp(this.values[1], 'billing-mutation-outbox-updated-at');
        const leaseToken = decodeNonEmptyString(this.values[2], 'billing-mutation-outbox-lease-token');
        const current = this.state.mutationOutbox.get(requestKey);
        if (!current) {
          throw new BillingReadModelUnavailableError('billing-mutation-outbox-missing');
        }
        if (current.auditState !== 'pending') {
          throw new BillingReadModelUnavailableError('billing-mutation-outbox-not-pending');
        }
        if (current.leaseToken !== leaseToken) {
          throw new BillingReadModelUnavailableError('billing-mutation-outbox-lease-conflict');
        }
        this.state.mutationOutbox.set(requestKey, {
          ...current,
          auditState: 'delivered',
          leaseToken: null,
          leaseExpiresAt: null,
          updatedAt,
        });
        return;
      }
      case MARK_MUTATION_OUTBOX_ATTEMPT_SQL: {
        const requestKey = decodeNonEmptyString(this.values[0], 'billing-mutation-outbox-request-key');
        const attemptedAt = decodeTimestamp(this.values[1], 'billing-mutation-outbox-attempted-at');
        const leaseToken = decodeNonEmptyString(this.values[2], 'billing-mutation-outbox-lease-token');
        const leaseExpiresAt = decodeTimestamp(this.values[3], 'billing-mutation-outbox-lease-expires-at');
        const maxAttempts = decodeBillingCount(this.values[4], 'billing-mutation-outbox-max-attempts');
        const current = this.state.mutationOutbox.get(requestKey);
        if (!current || current.auditState !== 'pending') {
          throw new BillingReadModelUnavailableError('billing-mutation-outbox-not-pending');
        }
        if (current.attemptCount >= maxAttempts) {
          throw new BillingReadModelUnavailableError(`billing-mutation-outbox-manual-review:${requestKey}`);
        }
        if (current.leaseExpiresAt !== null && current.leaseExpiresAt > attemptedAt) {
          throw new BillingReadModelUnavailableError('billing-mutation-outbox-lease-conflict');
        }
        this.state.mutationOutbox.set(requestKey, {
          ...current,
          attemptCount: decodeBillingCount(current.attemptCount + 1, 'billing-mutation-outbox-attempt-count'),
          lastAttemptAt: attemptedAt,
          lastError: null,
          leaseToken,
          leaseExpiresAt,
          updatedAt: attemptedAt,
        });
        return;
      }
      case MARK_MUTATION_OUTBOX_FAILURE_SQL: {
        const requestKey = decodeNonEmptyString(this.values[0], 'billing-mutation-outbox-request-key');
        const lastError = decodeNonEmptyString(this.values[1], 'billing-mutation-outbox-last-error');
        const updatedAt = decodeTimestamp(this.values[2], 'billing-mutation-outbox-updated-at');
        const leaseToken = decodeNonEmptyString(this.values[3], 'billing-mutation-outbox-lease-token');
        const current = this.state.mutationOutbox.get(requestKey);
        if (!current || current.auditState !== 'pending') {
          throw new BillingReadModelUnavailableError('billing-mutation-outbox-not-pending');
        }
        if (current.leaseToken !== leaseToken) {
          throw new BillingReadModelUnavailableError('billing-mutation-outbox-lease-conflict');
        }
        this.state.mutationOutbox.set(requestKey, {
          ...current,
          lastError,
          leaseToken: null,
          leaseExpiresAt: null,
          updatedAt,
        });
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
    refundLedgerByInvoice: new Map<string, ReadonlyArray<BillingRefundLedgerEntry>>(),
    providerEventReceipts: new Map<string, ProviderEventReceipt>(),
    providerEventCursors: new Map(),
    subjectVersions: new Map<string, { version: number; lastMutationToken: string | null; updatedAt: string }>(),
    mutationOutbox: new Map<string, LocalMutationOutboxEntry>(),
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
      refundLedgerByInvoice: new Map(this.state.refundLedgerByInvoice),
      providerEventReceipts: new Map(this.state.providerEventReceipts),
      providerEventCursors: new Map(this.state.providerEventCursors),
      subjectVersions: new Map(this.state.subjectVersions),
      mutationOutbox: new Map(this.state.mutationOutbox),
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
      this.state.refundLedgerByInvoice = backup.refundLedgerByInvoice;
      this.state.providerEventReceipts = backup.providerEventReceipts;
      this.state.providerEventCursors = backup.providerEventCursors;
      this.state.subjectVersions = backup.subjectVersions;
      this.state.mutationOutbox = backup.mutationOutbox;
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
      this.state.invoicesBySubject = new Map(
        Object.entries(patch.invoicesBySubject).map(([subject, invoices]) => [
          subject,
          invoices.map((invoice, index) =>
            decodeBillingInvoiceSummary(invoice, `billing-invoice-seed:${subject}-${index}`)
          ),
        ])
      );
    }
    if (patch.referralsBySubject) {
      this.state.referralsBySubject = mapFromRecord(patch.referralsBySubject);
    }
    if (decodedSnapshots) {
      this.state.snapshotsBySubject = decodedSnapshots;
    }
    if (patch.adminAccounts) {
      this.state.adminAccounts = patch.adminAccounts.map((account, index) =>
        decodeAdminBillingAccountSummary(account, `billing-admin-account-seed-${index}`)
      );
    }
    if (patch.adminInvoices) {
      this.state.adminInvoices = patch.adminInvoices.map((invoice, index) =>
        decodeAdminBillingInvoiceSummary(invoice, `billing-admin-invoice-seed-${index}`)
      );
    }
    if (patch.adminDisputes) {
      this.state.adminDisputes = patch.adminDisputes.map((dispute, index) =>
        decodeAdminBillingDisputeSummary(dispute, `billing-admin-dispute-seed-${index}`)
      );
    }
    if (patch.adminReferrals) {
      this.state.adminReferrals = asReadonlyArray(patch.adminReferrals);
    }
    this.state.refundLedgerByInvoice = new Map();
    this.state.subjectVersions = new Map();
    this.state.mutationOutbox = new Map();
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

async function ensureMutationSchema(env: Env): Promise<void> {
  const database = requireBillingD1Database(env);
  await database.exec(CREATE_MUTATION_SCHEMA_SQL);
  const columns = new Set(
    (await database.prepare(SELECT_MUTATION_OUTBOX_COLUMNS_SQL).all<MutationOutboxColumnRow>()).results.map(
      (row) => row.name
    )
  );
  const migrations: ReadonlyArray<readonly [string, string]> = [
    ['audit_event_json', "ALTER TABLE billing_mutation_outbox ADD COLUMN audit_event_json TEXT NOT NULL DEFAULT '{}'"],
    ['attempt_count', 'ALTER TABLE billing_mutation_outbox ADD COLUMN attempt_count INTEGER NOT NULL DEFAULT 0'],
    ['last_attempt_at', 'ALTER TABLE billing_mutation_outbox ADD COLUMN last_attempt_at TEXT'],
    ['last_error', 'ALTER TABLE billing_mutation_outbox ADD COLUMN last_error TEXT'],
    ['lease_token', 'ALTER TABLE billing_mutation_outbox ADD COLUMN lease_token TEXT'],
    ['lease_expires_at', 'ALTER TABLE billing_mutation_outbox ADD COLUMN lease_expires_at TEXT'],
    ['authority_subject', 'ALTER TABLE billing_mutation_outbox ADD COLUMN authority_subject TEXT'],
    ['authority_version', 'ALTER TABLE billing_mutation_outbox ADD COLUMN authority_version INTEGER'],
    ['authority_token', 'ALTER TABLE billing_mutation_outbox ADD COLUMN authority_token TEXT'],
  ];
  for (const [column, statement] of migrations) {
    if (!columns.has(column)) {
      try {
        await database.exec(statement);
      } catch (error) {
        const currentColumns = new Set(
          (await database.prepare(SELECT_MUTATION_OUTBOX_COLUMNS_SQL).all<MutationOutboxColumnRow>()).results.map(
            (row) => row.name
          )
        );
        if (!currentColumns.has(column)) {
          throw error;
        }
      }
    }
  }
  await database.exec(BILLING_MUTATION_AUTHORITY_GUARD_SQL);
  await database.exec(BILLING_PROVIDER_CURSOR_GUARD_SQL);
}

async function ensureProviderEventReceiptSchema(database: D1Database): Promise<void> {
  const columns = new Set(
    (await database.prepare(SELECT_PROVIDER_EVENT_RECEIPT_COLUMNS_SQL).all<MutationOutboxColumnRow>()).results.map(
      (row) => row.name
    )
  );
  const migrations: ReadonlyArray<readonly [string, string]> = [
    ['provider_occurred_at', 'ALTER TABLE billing_provider_event_receipts ADD COLUMN provider_occurred_at TEXT'],
    [
      'provider_sequence',
      'ALTER TABLE billing_provider_event_receipts ADD COLUMN provider_sequence INTEGER CHECK (provider_sequence IS NULL OR provider_sequence BETWEEN 0 AND 4294967295)',
    ],
    [
      'state_version',
      'ALTER TABLE billing_provider_event_receipts ADD COLUMN state_version INTEGER NOT NULL DEFAULT 0 CHECK (state_version BETWEEN 0 AND 4294967295)',
    ],
  ];
  for (const [column, statement] of migrations) {
    if (!columns.has(column)) {
      try {
        await database.exec(statement);
      } catch (error) {
        const currentColumns = new Set(
          (
            await database.prepare(SELECT_PROVIDER_EVENT_RECEIPT_COLUMNS_SQL).all<MutationOutboxColumnRow>()
          ).results.map((row) => row.name)
        );
        if (!currentColumns.has(column)) {
          throw error;
        }
      }
    }
  }
  await database.exec(
    'CREATE TABLE IF NOT EXISTS billing_provider_event_cursors (provider TEXT NOT NULL, billing_subject TEXT NOT NULL, last_occurred_at TEXT, last_sequence INTEGER CHECK (last_sequence IS NULL OR last_sequence BETWEEN 0 AND 4294967295), last_event_id TEXT NOT NULL, state_version INTEGER NOT NULL DEFAULT 0 CHECK (state_version BETWEEN 0 AND 4294967295), updated_at TEXT NOT NULL, PRIMARY KEY (provider, billing_subject))'
  );
  await database.exec(BILLING_PROVIDER_RECEIPT_STATE_GUARD_SQL);
}

function decodeSubjectVersionRow(
  row: SubjectVersionRow | null,
  scope: string
): {
  subject: string;
  version: number;
  lastMutationToken: string | null;
  updatedAt: string;
} {
  if (!row) {
    throw new BillingReadModelUnavailableError(`${scope}-missing`);
  }
  const subject = decodeNonEmptyString(row.subject, `${scope}-subject`);
  const version = decodeBillingCount(row.version, `${scope}-version`);
  const lastMutationToken =
    row.last_mutation_token === null || row.last_mutation_token === undefined
      ? null
      : decodeNonEmptyString(row.last_mutation_token, `${scope}-token`);
  if ((version === 0 && lastMutationToken !== null) || (version > 0 && lastMutationToken === null)) {
    throw new BillingReadModelUnavailableError(`${scope}-inconsistent`);
  }
  return {
    subject,
    version,
    lastMutationToken,
    updatedAt: decodeTimestamp(row.updated_at, `${scope}-updated-at`),
  };
}

async function ensureBillingSubjectVersion(
  env: Env,
  subject: string
): Promise<{
  subject: string;
  version: number;
  lastMutationToken: string | null;
  updatedAt: string;
}> {
  const decodedSubject = decodeNonEmptyString(subject, 'billing-subject-version-subject');
  const database = requireBillingD1Database(env);
  await database.prepare(INSERT_SUBJECT_VERSION_SQL).bind(decodedSubject, new Date().toISOString()).run();
  return decodeSubjectVersionRow(
    await d1First<SubjectVersionRow>(database, SELECT_SUBJECT_VERSION_SQL, decodedSubject),
    `billing-subject-version:${decodedSubject}`
  );
}

function mutationReplayKey(mutation: BillingStateMutation): string {
  switch (mutation.kind) {
    case 'provider-webhook':
      return `provider-webhook:${mutation.provider}:${mutation.eventId}`;
    case 'reconciliation':
      return `reconciliation:${mutation.subject}:${mutation.requestId}`;
    case 'admin-refund':
      return `admin-refund:${mutation.actorSubject}:${mutation.requestId}`;
    default:
      return `${mutation.kind}:${mutation.subject}:${mutation.requestId}`;
  }
}

function stableMutationValue(value: unknown): unknown {
  if (Array.isArray(value)) {
    return value.map((entry) => stableMutationValue(entry));
  }
  if (typeof value !== 'object' || value === null) {
    return value;
  }
  return Object.fromEntries(
    Object.entries(value as Record<string, unknown>)
      .sort(([left], [right]) => (left < right ? -1 : left > right ? 1 : 0))
      .map(([key, entry]) => [key, stableMutationValue(entry)])
  );
}

function mutationFingerprint(mutation: BillingStateMutation): string {
  if (mutation.kind === 'provider-webhook') {
    const identity = Object.fromEntries(
      Object.entries(mutation).filter(([key]) => key !== 'providerCursorExpectedVersion')
    );
    return JSON.stringify(stableMutationValue(identity));
  }
  return JSON.stringify(stableMutationValue(mutation));
}

function canonicalMutationEventId(mutation: BillingStateMutation): string {
  switch (mutation.kind) {
    case 'hosted-session':
      return `billing-hosted-session:${mutation.subject}:${mutation.sessionKind}:${mutation.requestId}`;
    case 'change-plan':
      return `billing-change-plan:${mutation.subject}:${mutation.requestId}`;
    case 'cancel':
      return `billing-cancel:${mutation.subject}:${mutation.requestId}`;
    case 'referral-invite':
      return `billing-referral-invite:${mutation.subject}:${mutation.requestId}`;
    case 'manual-invoice':
      return `billing-manual-invoice:${mutation.subject}:${mutation.requestId}`;
    case 'admin-refund':
      return `billing-admin-refund:${mutation.subject}:${mutation.invoiceId}:${mutation.actorSubject}:${mutation.requestId}`;
    case 'reconciliation':
      return `billing-reconciliation:${mutation.subject}:${mutation.requestId}`;
    case 'provider-webhook':
      return `billing-provider-webhook:${mutation.subject}:${mutation.provider}:${mutation.eventId}`;
  }
  throw new BillingReadModelUnavailableError(`billing-event-id-unhandled:${mutation.kind}`);
}

function billingMutationOutboxStatement(
  env: Env,
  mutation: BillingStateMutation,
  auditEvent: BillingAuditEventSummary,
  authorityVersion: number,
  authorityToken: string
): D1PreparedStatement {
  const now = new Date().toISOString();
  const key = mutationReplayKey(mutation);
  return requireBillingD1Database(env)
    .prepare(INSERT_MUTATION_OUTBOX_SQL)
    .bind(
      key,
      decodeNonEmptyString(mutation.subject, 'billing-mutation-outbox-authority-subject'),
      decodeBillingCount(authorityVersion, 'billing-mutation-outbox-authority-version'),
      decodeNonEmptyString(authorityToken, 'billing-mutation-outbox-authority-token'),
      mutation.kind,
      mutationFingerprint(mutation),
      'pending',
      JSON.stringify(auditEvent),
      now,
      now
    );
}

function decodeMutationOutboxRow(row: MutationOutboxRow): MutationOutboxRow {
  decodeNonEmptyString(row.request_key, 'billing-mutation-outbox-request-key');
  const authoritySubject = row.authority_subject ?? null;
  const authorityVersion = row.authority_version ?? null;
  const authorityToken = row.authority_token ?? null;
  const authorityFieldsMissing = authoritySubject === null && authorityVersion === null && authorityToken === null;
  const authorityFieldsPartial = authoritySubject === null || authorityVersion === null || authorityToken === null;
  if (!authorityFieldsMissing && authorityFieldsPartial) {
    throw new BillingReadModelUnavailableError('billing-mutation-outbox-authority-incomplete');
  }
  if (!authorityFieldsMissing) {
    decodeNonEmptyString(authoritySubject, 'billing-mutation-outbox-authority-subject');
    if (decodeBillingCount(authorityVersion, 'billing-mutation-outbox-authority-version') < 1) {
      throw new BillingReadModelUnavailableError('billing-mutation-outbox-authority-version-invalid');
    }
    decodeNonEmptyString(authorityToken, 'billing-mutation-outbox-authority-token');
  }
  decodeNonEmptyString(row.mutation_kind, 'billing-mutation-outbox-kind');
  const mutationJson = decodeNonEmptyString(row.mutation_json, 'billing-mutation-outbox-mutation');
  if (!authorityFieldsMissing) {
    const mutationSubject = decodeNonEmptyString(
      payloadRecord(
        parseUnknownPayload(mutationJson, 'billing-mutation-outbox-mutation'),
        'billing-mutation-outbox-mutation'
      ).subject,
      'billing-mutation-outbox-mutation-subject'
    );
    if (mutationSubject !== authoritySubject) {
      throw new BillingReadModelUnavailableError('billing-mutation-authority-subject-mismatch');
    }
  }
  decodeLiteral(row.audit_state, ['pending', 'delivered'] as const, 'billing-mutation-outbox-audit-state');
  decodeCanonicalValue('billing-mutation-outbox-audit-event', () =>
    BillingSupportAdminAuditEventSummarySchema.parse(
      parseUnknownPayload(row.audit_event_json, 'billing-mutation-outbox-audit-event')
    )
  );
  decodeBillingCount(row.attempt_count, 'billing-mutation-outbox-attempt-count');
  decodeNullableTimestamp(row.last_attempt_at, 'billing-mutation-outbox-last-attempt-at');
  if (row.last_error !== null && row.last_error !== undefined) {
    decodeNonEmptyString(row.last_error, 'billing-mutation-outbox-last-error');
  }
  if (row.lease_token !== null && row.lease_token !== undefined) {
    decodeNonEmptyString(row.lease_token, 'billing-mutation-outbox-lease-token');
  }
  decodeNullableTimestamp(row.lease_expires_at, 'billing-mutation-outbox-lease-expires-at');
  decodeTimestamp(row.created_at, 'billing-mutation-outbox-created-at');
  decodeTimestamp(row.updated_at, 'billing-mutation-outbox-updated-at');
  return row;
}

function decodeProviderEventReceiptRow(row: ProviderEventReceiptRow): ProviderEventReceipt {
  const provider = decodeLiteral(
    row.provider,
    ['stripe', 'razorpay', 'paypal', 'apple', 'google'] as const,
    'billing-provider-event-provider'
  );
  const eventId = decodeNonEmptyString(row.event_id, 'billing-provider-event-id');
  const eventFingerprint = decodeNonEmptyString(row.event_fingerprint, `billing-provider-event-fingerprint:${eventId}`);
  const eventType = decodeNonEmptyString(row.event_type, `billing-provider-event-type:${eventId}`);
  const providerOccurredAt =
    row.provider_occurred_at === null
      ? null
      : decodeTimestamp(row.provider_occurred_at, `billing-provider-event-occurred-at:${eventId}`);
  const providerSequence =
    row.provider_sequence === null
      ? null
      : decodeBillingCount(row.provider_sequence, `billing-provider-event-sequence:${eventId}`);
  const stateVersion = decodeBillingCount(row.state_version, `billing-provider-event-state-version:${eventId}`);
  const processingState = decodeLiteral(
    row.processing_state,
    ['received', 'ignored', 'queued', 'applied', 'manual-required', 'dead-letter'] as const,
    `billing-provider-event-processing-state:${eventId}`
  );
  const queueState = decodeLiteral(
    row.queue_state,
    ['pending', 'queued', 'delivered', 'manual-required', 'dead-letter'] as const,
    `billing-provider-event-queue-state:${eventId}`
  );
  const optionalText = (value: string | null, scope: string): string | null =>
    value === null ? null : decodeNonEmptyString(value, scope);
  return {
    provider,
    eventId,
    eventFingerprint,
    eventType,
    providerOccurredAt,
    providerSequence,
    stateVersion,
    accountId: optionalText(row.account_id, `billing-provider-event-account:${eventId}`),
    providerCustomerId: optionalText(row.provider_customer_id, `billing-provider-event-customer:${eventId}`),
    providerSubscriptionId: optionalText(
      row.provider_subscription_id,
      `billing-provider-event-subscription:${eventId}`
    ),
    providerInvoiceId: optionalText(row.provider_invoice_id, `billing-provider-event-invoice:${eventId}`),
    billingSubject: optionalText(row.billing_subject, `billing-provider-event-subject:${eventId}`),
    parentAccountRef: optionalText(row.parent_account_ref, `billing-provider-event-parent-account:${eventId}`),
    familyRef: optionalText(row.family_ref, `billing-provider-event-family:${eventId}`),
    billingInvoiceId: optionalText(row.billing_invoice_id, `billing-provider-event-billing-invoice:${eventId}`),
    processingState,
    queueState,
    queueAttemptCount: decodeBillingCount(row.queue_attempt_count, `billing-provider-event-attempts:${eventId}`),
    lastQueueAttemptAt: decodeNullableTimestamp(
      row.last_queue_attempt_at,
      `billing-provider-event-last-attempt:${eventId}`
    ),
    lastError:
      row.last_error === null ? null : decodeNonEmptyString(row.last_error, `billing-provider-event-error:${eventId}`),
    createdAt: decodeTimestamp(row.created_at, `billing-provider-event-created:${eventId}`),
    updatedAt: decodeTimestamp(row.updated_at, `billing-provider-event-updated:${eventId}`),
  };
}

export interface ProviderEventCursor {
  provider: string;
  billingSubject: string;
  lastOccurredAt: string | null;
  lastSequence: number | null;
  lastEventId: string;
  stateVersion: number;
  updatedAt: string;
}

function decodeProviderEventCursorRow(row: ProviderEventCursorRow): ProviderEventCursor {
  return {
    provider: decodeLiteral(
      row.provider,
      ['stripe', 'razorpay', 'paypal', 'apple', 'google'] as const,
      'billing-provider-event-cursor-provider'
    ),
    billingSubject: decodeNonEmptyString(row.billing_subject, 'billing-provider-event-cursor-subject'),
    lastOccurredAt:
      row.last_occurred_at === null
        ? null
        : decodeTimestamp(row.last_occurred_at, 'billing-provider-event-cursor-occurred-at'),
    lastSequence:
      row.last_sequence === null
        ? null
        : decodeBillingCount(row.last_sequence, 'billing-provider-event-cursor-sequence'),
    lastEventId: decodeNonEmptyString(row.last_event_id, 'billing-provider-event-cursor-event-id'),
    stateVersion: decodeBillingCount(row.state_version, 'billing-provider-event-cursor-state-version'),
    updatedAt: decodeTimestamp(row.updated_at, 'billing-provider-event-cursor-updated'),
  };
}

export async function loadBillingProviderEventCursor(
  env: Env,
  provider: string,
  billingSubject: string
): Promise<ProviderEventCursor | null> {
  await ensureReadModelSeed(env);
  const row = await d1First<ProviderEventCursorRow>(
    requireBillingD1Database(env),
    SELECT_PROVIDER_EVENT_CURSOR_SQL,
    provider,
    billingSubject
  );
  return row ? decodeProviderEventCursorRow(row) : null;
}

function providerEventOrderIsStale(
  cursor: ProviderEventCursor,
  receipt: Pick<ProviderEventReceipt, 'eventId' | 'providerOccurredAt' | 'providerSequence'>
): boolean {
  if (receipt.providerSequence !== null && cursor.lastSequence !== null) {
    if (receipt.providerSequence < cursor.lastSequence) {
      return true;
    }
    if (receipt.providerSequence === cursor.lastSequence && receipt.eventId !== cursor.lastEventId) {
      return true;
    }
  } else if (cursor.lastSequence !== null && receipt.providerSequence === null) {
    return true;
  }
  if (receipt.providerOccurredAt !== null && cursor.lastOccurredAt !== null) {
    if (receipt.providerOccurredAt < cursor.lastOccurredAt) {
      return true;
    }
    if (receipt.providerOccurredAt === cursor.lastOccurredAt && receipt.eventId !== cursor.lastEventId) {
      return true;
    }
  } else if (cursor.lastOccurredAt !== null && receipt.providerOccurredAt === null) {
    return true;
  }
  return false;
}

function providerEventCursorStatement(
  env: Env,
  receipt: ProviderEventReceipt,
  expectedVersion: number
): D1PreparedStatement {
  if (receipt.billingSubject === null) {
    throw new BillingReadModelUnavailableError(`billing-provider-event-cursor-subject-missing:${receipt.eventId}`);
  }
  if (receipt.providerOccurredAt === null && receipt.providerSequence === null) {
    throw new BillingReadModelUnavailableError(`billing-provider-event-order-metadata-missing:${receipt.eventId}`);
  }
  return requireBillingD1Database(env)
    .prepare(UPSERT_PROVIDER_EVENT_CURSOR_SQL)
    .bind(
      receipt.provider,
      receipt.billingSubject,
      receipt.providerOccurredAt,
      receipt.providerSequence,
      receipt.eventId,
      new Date().toISOString(),
      expectedVersion
    );
}

export interface RegisterBillingProviderEventInput {
  provider: string;
  eventId: string;
  eventFingerprint: string;
  eventType: string;
  providerOccurredAt: string | null;
  providerSequence: number | null;
  accountId: string | null;
  providerCustomerId: string | null;
  providerSubscriptionId: string | null;
  providerInvoiceId: string | null;
  billingSubject: string | null;
  parentAccountRef: string | null;
  familyRef: string | null;
  billingInvoiceId: string | null;
  processingState: ProviderEventReceipt['processingState'];
}

export type RegisterBillingProviderEventResult =
  | { status: 'created'; receipt: ProviderEventReceipt }
  | { status: 'replay'; receipt: ProviderEventReceipt }
  | { status: 'conflict'; receipt: ProviderEventReceipt };

function optionalBindingText(value: string | null, scope: string): string | null {
  return value === null ? null : decodeNonEmptyString(value, scope);
}

function providerReceiptAuthorityMatches(
  receipt: ProviderEventReceipt,
  input: RegisterBillingProviderEventInput,
  eventType: string,
  providerOccurredAt: string | null,
  providerSequence: number | null
): boolean {
  return (
    receipt.provider === input.provider &&
    receipt.eventId === input.eventId &&
    receipt.eventType === eventType &&
    receipt.providerOccurredAt === providerOccurredAt &&
    receipt.providerSequence === providerSequence &&
    receipt.accountId === input.accountId &&
    receipt.providerCustomerId === input.providerCustomerId &&
    receipt.providerSubscriptionId === input.providerSubscriptionId &&
    receipt.providerInvoiceId === input.providerInvoiceId &&
    receipt.billingSubject === input.billingSubject &&
    receipt.parentAccountRef === input.parentAccountRef &&
    receipt.familyRef === input.familyRef &&
    receipt.billingInvoiceId === input.billingInvoiceId
  );
}

export async function registerBillingProviderEvent(
  env: Env,
  input: RegisterBillingProviderEventInput
): Promise<RegisterBillingProviderEventResult> {
  await ensureReadModelSeed(env);
  const database = requireBillingD1Database(env);
  const provider = decodeLiteral(
    input.provider,
    ['stripe', 'razorpay', 'paypal', 'apple', 'google'] as const,
    'billing-provider-event-provider'
  );
  const eventId = decodeNonEmptyString(input.eventId, 'billing-provider-event-id');
  const eventFingerprint = decodeNonEmptyString(
    input.eventFingerprint,
    `billing-provider-event-fingerprint:${eventId}`
  );
  const eventType = decodeNonEmptyString(input.eventType, `billing-provider-event-type:${eventId}`);
  const providerOccurredAt =
    input.providerOccurredAt === null
      ? null
      : decodeTimestamp(input.providerOccurredAt, `billing-provider-event-occurred-at:${eventId}`);
  const providerSequence =
    input.providerSequence === null
      ? null
      : decodeBillingCount(input.providerSequence, `billing-provider-event-sequence:${eventId}`);
  const processingState = decodeLiteral(
    input.processingState,
    ['received', 'ignored', 'manual-required'] as const,
    `billing-provider-event-processing-state:${eventId}`
  );
  const existing = await d1First<ProviderEventReceiptRow>(
    database,
    SELECT_PROVIDER_EVENT_RECEIPT_SQL,
    provider,
    eventId
  );
  if (existing) {
    const receipt = decodeProviderEventReceiptRow(existing);
    const authorityMatches = providerReceiptAuthorityMatches(
      receipt,
      input,
      eventType,
      providerOccurredAt,
      providerSequence
    );
    return {
      status: receipt.eventFingerprint === eventFingerprint && authorityMatches ? 'replay' : 'conflict',
      receipt,
    };
  }

  const createdAt = new Date().toISOString();
  try {
    await database
      .prepare(INSERT_PROVIDER_EVENT_RECEIPT_SQL)
      .bind(
        provider,
        eventId,
        eventFingerprint,
        eventType,
        providerOccurredAt,
        providerSequence,
        optionalBindingText(input.accountId, `billing-provider-event-account:${eventId}`),
        optionalBindingText(input.providerCustomerId, `billing-provider-event-customer:${eventId}`),
        optionalBindingText(input.providerSubscriptionId, `billing-provider-event-subscription:${eventId}`),
        optionalBindingText(input.providerInvoiceId, `billing-provider-event-invoice:${eventId}`),
        optionalBindingText(input.billingSubject, `billing-provider-event-subject:${eventId}`),
        optionalBindingText(input.parentAccountRef, `billing-provider-event-parent-account:${eventId}`),
        optionalBindingText(input.familyRef, `billing-provider-event-family:${eventId}`),
        optionalBindingText(input.billingInvoiceId, `billing-provider-event-billing-invoice:${eventId}`),
        processingState,
        createdAt
      )
      .run();
  } catch (error) {
    const raced = await d1First<ProviderEventReceiptRow>(
      database,
      SELECT_PROVIDER_EVENT_RECEIPT_SQL,
      provider,
      eventId
    );
    if (!raced) {
      throw error;
    }
  }
  const persisted = await d1First<ProviderEventReceiptRow>(
    database,
    SELECT_PROVIDER_EVENT_RECEIPT_SQL,
    provider,
    eventId
  );
  if (!persisted) {
    throw new BillingReadModelUnavailableError(`billing-provider-event-receipt-missing:${provider}:${eventId}`);
  }
  const receipt = decodeProviderEventReceiptRow(persisted);
  const authorityMatches = providerReceiptAuthorityMatches(
    receipt,
    input,
    eventType,
    providerOccurredAt,
    providerSequence
  );
  return {
    status: receipt.eventFingerprint === eventFingerprint && authorityMatches ? 'created' : 'conflict',
    receipt,
  };
}

export async function loadBillingProviderEventReceipt(
  env: Env,
  provider: string,
  eventId: string
): Promise<ProviderEventReceipt | null> {
  await ensureReadModelSeed(env);
  const row = await d1First<ProviderEventReceiptRow>(
    requireBillingD1Database(env),
    SELECT_PROVIDER_EVENT_RECEIPT_SQL,
    provider,
    eventId
  );
  return row ? decodeProviderEventReceiptRow(row) : null;
}

export async function markBillingProviderEventQueue(
  env: Env,
  provider: string,
  eventId: string,
  queueState: ProviderEventReceipt['queueState'],
  processingState: ProviderEventReceipt['processingState'],
  error: string | null,
  expected?: Pick<ProviderEventReceipt, 'stateVersion' | 'processingState' | 'queueState'>
): Promise<ProviderEventReceipt> {
  await ensureReadModelSeed(env);
  const database = requireBillingD1Database(env);
  const now = new Date().toISOString();
  const boundedError =
    error === null ? null : decodeNonEmptyString(error, `billing-provider-event-error:${eventId}`).slice(0, 240);
  const currentRow = await d1First<ProviderEventReceiptRow>(
    database,
    SELECT_PROVIDER_EVENT_RECEIPT_SQL,
    provider,
    eventId
  );
  if (!currentRow) {
    throw new BillingReadModelUnavailableError(`billing-provider-event-receipt-missing:${provider}:${eventId}`);
  }
  const current = decodeProviderEventReceiptRow(currentRow);
  const expectedState = expected ?? current;
  if (
    current.stateVersion !== expectedState.stateVersion ||
    current.processingState !== expectedState.processingState ||
    current.queueState !== expectedState.queueState
  ) {
    throw new BillingReadModelUnavailableError(`billing-provider-event-state-cas-failed:${provider}:${eventId}`);
  }
  if (current.queueState === queueState && current.processingState === processingState) {
    return current;
  }
  assertProviderEventReceiptTransition(current, queueState, processingState);
  const update = await database
    .prepare(UPDATE_PROVIDER_EVENT_RECEIPT_QUEUE_SQL)
    .bind(
      provider,
      eventId,
      current.stateVersion,
      current.processingState,
      current.queueState,
      processingState,
      queueState,
      1,
      now,
      boundedError
    )
    .run();
  if ((update.meta?.changes ?? 0) !== 1) {
    throw new BillingReadModelUnavailableError(`billing-provider-event-state-cas-failed:${provider}:${eventId}`);
  }
  const row = await d1First<ProviderEventReceiptRow>(database, SELECT_PROVIDER_EVENT_RECEIPT_SQL, provider, eventId);
  if (!row) {
    throw new BillingReadModelUnavailableError(`billing-provider-event-receipt-missing:${provider}:${eventId}`);
  }
  return decodeProviderEventReceiptRow(row);
}

async function loadMutationOutbox(env: Env, mutation: BillingStateMutation): Promise<MutationOutboxRow | null> {
  const row = await d1First<MutationOutboxRow>(
    requireBillingD1Database(env),
    SELECT_MUTATION_OUTBOX_SQL,
    mutationReplayKey(mutation)
  );
  if (!row) {
    return null;
  }
  return decodeMutationOutboxRow(row);
}

type RefundLedgerState = 'none' | 'refund-requested' | 'refund-settled';

async function loadRefundLedgerSummary(
  env: Env,
  invoiceId: string
): Promise<{ appliedAmountCents: number; finalRefundState: RefundLedgerState }> {
  await ensureMutationSchema(env);
  const row = await d1First<RefundLedgerSummaryRow>(
    requireBillingD1Database(env),
    SELECT_REFUND_LEDGER_SUMMARY_SQL,
    invoiceId
  );
  const finalRefundState =
    row?.final_refund_state === null || row?.final_refund_state === undefined
      ? 'none'
      : decodeLiteral(
          row.final_refund_state,
          ['refund-requested', 'refund-settled'] as const,
          `billing-refund-ledger-state:${invoiceId}`
        );
  return {
    appliedAmountCents: decodeBillingCount(row?.applied_amount_cents ?? 0, `billing-refund-ledger-total:${invoiceId}`),
    finalRefundState,
  };
}

export async function loadAppliedRefundAmount(env: Env, invoiceId: string): Promise<number> {
  return (await loadRefundLedgerSummary(env, invoiceId)).appliedAmountCents;
}

function requireBillingD1Database(env: Env): D1Database {
  const database = env.BILLING_D1;
  if (!database) {
    throw new BillingReadModelUnavailableError('billing-d1-binding-missing');
  }
  return database;
}

async function commitBillingD1Batch(env: Env, statements: ReadonlyArray<D1PreparedStatement>): Promise<void> {
  if (statements.length === 0) {
    return;
  }
  await requireBillingD1Database(env).batch(statements);
}

function billingStatePairStatements(
  env: Env,
  status: BillingStatusSummary,
  snapshot: BillingEntitlementSnapshotSummary
): ReadonlyArray<D1PreparedStatement> {
  const { status: decodedStatus, snapshot: decodedSnapshot } = decodeBillingStatePair(
    status,
    snapshot,
    `billing-state-write:${status.subject}`
  );
  const database = requireBillingD1Database(env);
  return [
    database.prepare(UPSERT_STATUS_SQL).bind(decodedStatus.subject, JSON.stringify(decodedStatus)),
    database.prepare(UPSERT_SNAPSHOT_SQL).bind(decodedSnapshot.subject, JSON.stringify(decodedSnapshot)),
  ];
}

function billingAdminAccountStatement(env: Env, account: AdminBillingAccountSummary): D1PreparedStatement {
  const decoded = decodeAdminBillingAccountSummary(account, 'billing-admin-account-write');
  return requireBillingD1Database(env)
    .prepare(UPSERT_ADMIN_ACCOUNT_SQL)
    .bind(decoded.parentAccountRef, JSON.stringify(decoded));
}

function billingInvoiceStatement(env: Env, subject: string, invoice: BillingInvoiceSummary): D1PreparedStatement {
  const decoded = decodeBillingInvoiceSummary(invoice, `billing-invoice-write:${subject}`);
  return requireBillingD1Database(env)
    .prepare(UPSERT_INVOICE_SQL)
    .bind(subject, decoded.invoiceId, JSON.stringify(decoded));
}

function billingAdminInvoiceStatement(env: Env, invoice: AdminBillingInvoiceSummary): D1PreparedStatement {
  const decoded = decodeAdminBillingInvoiceSummary(invoice, `billing-admin-invoice-write:${invoice.invoiceId}`);
  return requireBillingD1Database(env)
    .prepare(UPSERT_ADMIN_INVOICE_SQL)
    .bind(decoded.invoiceId, JSON.stringify(decoded));
}

function billingAdminDisputeStatement(env: Env, dispute: AdminBillingDisputeSummary): D1PreparedStatement {
  const decoded = decodeAdminBillingDisputeSummary(dispute, `billing-admin-dispute-write:${dispute.disputeId}`);
  return requireBillingD1Database(env)
    .prepare(UPSERT_ADMIN_DISPUTE_SQL)
    .bind(decoded.disputeId, JSON.stringify(decoded));
}

function billingRefundLedgerStatement(
  env: Env,
  mutation: Extract<BillingStateMutation, { kind: 'admin-refund' }>,
  invoiceTotalCents: number,
  createdAt: string
): D1PreparedStatement {
  return requireBillingD1Database(env)
    .prepare(INSERT_REFUND_LEDGER_SQL)
    .bind(
      mutation.invoiceId,
      mutationReplayKey(mutation),
      mutation.subject,
      decodeBillingCount(mutation.amountCents, 'billing-refund-ledger-amount-cents'),
      decodeBillingCount(invoiceTotalCents, 'billing-refund-ledger-invoice-total-cents'),
      mutation.refundState,
      decodeCanonicalValue('billing-refund-ledger-audit-reference', () =>
        BillingAuditReferenceSchema.parse(mutation.auditReference)
      ),
      decodeTimestamp(createdAt, 'billing-refund-ledger-created-at')
    );
}

async function commitBillingMutationD1Batch(
  env: Env,
  mutation: BillingStateMutation,
  statements: ReadonlyArray<D1PreparedStatement>,
  auditEvent: BillingAuditEventSummary
): Promise<void> {
  await ensureMutationSchema(env);
  const currentAuthority = await ensureBillingSubjectVersion(env, mutation.subject);
  const authorityVersion = incrementBillingCount(
    currentAuthority.version,
    `billing-subject-version-next:${mutation.subject}`
  );
  const authorityToken = crypto.randomUUID();
  const database = requireBillingD1Database(env);
  const authorityAdvance = database
    .prepare(ADVANCE_SUBJECT_VERSION_SQL)
    .bind(mutation.subject, currentAuthority.version, authorityToken, new Date().toISOString());
  const providerCursorStatements: ReadonlyArray<D1PreparedStatement> =
    mutation.kind === 'provider-webhook'
      ? await (async () => {
          const receipt = await loadBillingProviderEventReceipt(env, mutation.provider, mutation.eventId);
          if (!receipt || receipt.billingSubject !== mutation.subject) {
            throw new BillingReadModelUnavailableError(
              `billing-provider-event-receipt-authority-mismatch:${mutation.eventId}`
            );
          }
          if (
            receipt.providerOccurredAt !== mutation.providerOccurredAt ||
            receipt.providerSequence !== mutation.providerSequence
          ) {
            throw new BillingReadModelUnavailableError(
              `billing-provider-event-order-metadata-mismatch:${mutation.eventId}`
            );
          }
          const cursor = await loadBillingProviderEventCursor(env, mutation.provider, mutation.subject);
          const expectedCursorVersion = cursor?.stateVersion ?? 0;
          if (expectedCursorVersion !== mutation.providerCursorExpectedVersion) {
            throw new BillingReadModelUnavailableError(`billing-provider-event-cursor-cas-failed:${mutation.eventId}`);
          }
          if (cursor && providerEventOrderIsStale(cursor, receipt)) {
            throw new BillingReadModelUnavailableError(`billing-provider-event-stale:${mutation.eventId}`);
          }
          return [providerEventCursorStatement(env, receipt, expectedCursorVersion)];
        })()
      : [];
  await commitBillingD1Batch(env, [
    authorityAdvance,
    ...providerCursorStatements,
    ...statements,
    billingMutationOutboxStatement(env, mutation, auditEvent, authorityVersion, authorityToken),
  ]);
}

function billingReferralStatement(env: Env, referral: BillingReferralSummary): D1PreparedStatement {
  const decoded = decodeBillingReferralSummary(referral, `billing-referral-write:${referral.subject}`);
  return requireBillingD1Database(env).prepare(UPSERT_REFERRAL_SQL).bind(decoded.subject, JSON.stringify(decoded));
}

function billingAdminReferralStatement(env: Env, referral: AdminBillingReferralSummary): D1PreparedStatement {
  return requireBillingD1Database(env)
    .prepare(UPSERT_ADMIN_REFERRAL_SQL)
    .bind(referral.referralCode, JSON.stringify(referral));
}

async function readStoredAuditEvents(env: Env): Promise<ReadonlyArray<BillingAuditEventSummary>> {
  const object = await env.BILLING_AUDIT_R2?.get(AUDIT_EVENTS_KEY);
  if (!object) {
    return [];
  }
  return ((await object.json<ReadonlyArray<BillingAuditEventSummary>>()) ?? []).map((entry) => cloneJsonValue(entry));
}

export async function appendBillingAuditEventAtOwner(env: Env, value: unknown): Promise<void> {
  const nextEvent = decodeCanonicalValue('billing-audit-event', () =>
    BillingSupportAdminAuditEventSummarySchema.parse(value)
  );
  if (!env.BILLING_AUDIT_R2) {
    throw new BillingReadModelUnavailableError('billing-audit-r2-binding-missing');
  }
  const current = await readStoredAuditEvents(env);
  const next = replaceByKey(current, nextEvent, (entry) => entry.eventId);
  await env.BILLING_AUDIT_R2.put(AUDIT_EVENTS_KEY, JSON.stringify(next));
}

async function appendBillingAuditEvent(env: Env, nextEvent: BillingAuditEventSummary): Promise<void> {
  const namespace = env.BILLING_DO;
  if (!namespace) {
    if (isLocalFixtureEnvironment(env)) {
      await appendBillingAuditEventAtOwner(env, nextEvent);
      return;
    }
    throw new BillingReadModelUnavailableError('billing-audit-owner-binding-missing');
  }
  const response = await namespace.get(namespace.idFromName(BILLING_AUDIT_OWNER_NAME)).fetch(
    new Request(`https://billing-owner.local${BILLING_AUDIT_APPEND_PATH}`, {
      method: 'POST',
      headers: {
        'content-type': 'application/json',
      },
      body: JSON.stringify(nextEvent),
    })
  );
  if (!response.ok) {
    throw new BillingReadModelUnavailableError(`billing-audit-owner-${response.status}`);
  }
}

async function billingAuditEventForMutation(
  env: Env,
  mutation: BillingStateMutation
): Promise<BillingAuditEventSummary> {
  const createdAt = new Date().toISOString();
  switch (mutation.kind) {
    case 'hosted-session': {
      const status = await loadBillingStatusSummary(env, mutation.subject);
      return {
        eventId: canonicalMutationEventId(mutation),
        eventType: hostedSessionAuditEventType(mutation.sessionKind),
        actorRole: mutation.actorRole,
        parentAccountRef: status.parentAccountRef,
        familyRef: status.familyRef,
        auditReference: mutation.auditReference,
        createdAt,
      } as BillingAuditEventSummary;
    }
    case 'change-plan': {
      const status = await loadBillingStatusSummary(env, mutation.subject);
      return {
        eventId: canonicalMutationEventId(mutation),
        eventType: 'billing.change-plan.accepted',
        actorRole: 'parent',
        parentAccountRef: status.parentAccountRef,
        familyRef: status.familyRef,
        auditReference: mutation.auditReference,
        createdAt,
      } as BillingAuditEventSummary;
    }
    case 'cancel': {
      const status = await loadBillingStatusSummary(env, mutation.subject);
      return {
        eventId: canonicalMutationEventId(mutation),
        eventType: 'billing.cancel.accepted',
        actorRole: 'parent',
        parentAccountRef: status.parentAccountRef,
        familyRef: status.familyRef,
        auditReference: mutation.auditReference,
        createdAt,
      } as BillingAuditEventSummary;
    }
    case 'referral-invite': {
      const status = await loadBillingStatusSummary(env, mutation.subject);
      return {
        eventId: canonicalMutationEventId(mutation),
        eventType: 'billing.referral.invite-created',
        actorRole: mutation.actorRole,
        parentAccountRef: status.parentAccountRef,
        familyRef: status.familyRef,
        auditReference: mutation.auditReference,
        createdAt,
      } as BillingAuditEventSummary;
    }
    case 'manual-invoice': {
      const status = await loadBillingStatusSummary(env, mutation.subject);
      return {
        eventId: canonicalMutationEventId(mutation),
        eventType: 'billing.manual-invoice.created',
        actorRole: mutation.actorRole,
        parentAccountRef: status.parentAccountRef,
        familyRef: status.familyRef,
        auditReference: `${mutation.auditReference}:state`,
        createdAt,
      } as BillingAuditEventSummary;
    }
    case 'admin-refund': {
      const status = await loadBillingStatusSummary(env, mutation.subject);
      return {
        eventId: canonicalMutationEventId(mutation),
        eventType: `billing.refund.${mutation.refundState}`,
        actorRole: mutation.actorRole,
        parentAccountRef: status.parentAccountRef,
        familyRef: status.familyRef,
        auditReference: `${mutation.auditReference}:state`,
        createdAt,
      } as BillingAuditEventSummary;
    }
    case 'reconciliation':
      return {
        eventId: canonicalMutationEventId(mutation),
        eventType: 'billing.reconciliation.accepted',
        actorRole: mutation.actorRole,
        parentAccountRef: RECONCILIATION_PARENT_ACCOUNT_REF,
        familyRef: RECONCILIATION_FAMILY_REF,
        auditReference: mutation.auditReference,
        createdAt,
      } as BillingAuditEventSummary;
    case 'provider-webhook': {
      const status = await loadBillingStatusSummary(env, mutation.subject);
      return {
        eventId: canonicalMutationEventId(mutation),
        eventType: `billing.webhook.${mutation.provider}.${mutation.eventType}`,
        actorRole: 'system',
        parentAccountRef: status.parentAccountRef,
        familyRef: status.familyRef,
        auditReference: `${status.auditReference}:provider-webhook:${mutation.provider}`,
        createdAt,
      } as BillingAuditEventSummary;
    }
  }
  throw new BillingReadModelUnavailableError(`billing-audit-event-unhandled:${mutation.kind}`);
}

async function completeBillingMutation(env: Env, mutation: BillingStateMutation): Promise<void> {
  const outbox = await loadMutationOutbox(env, mutation);
  if (!outbox) {
    throw new BillingReadModelUnavailableError(`billing-mutation-outbox-missing:${mutationReplayKey(mutation)}`);
  }
  if (outbox.mutation_kind !== mutation.kind || outbox.mutation_json !== mutationFingerprint(mutation)) {
    throw new BillingReadModelUnavailableError(`billing-mutation-outbox-conflict:${mutationReplayKey(mutation)}`);
  }
  await deliverBillingMutationOutbox(env, outbox);
}

function billingOutboxErrorMessage(error: unknown): string {
  const message = error instanceof Error ? error.message : String(error);
  return message.replace(/\s+/gu, ' ').trim().slice(0, 240) || 'billing-audit-delivery-failed';
}

async function deliverBillingMutationOutbox(env: Env, outbox: MutationOutboxRow): Promise<boolean> {
  if (outbox.audit_state === 'delivered') {
    return true;
  }
  const attemptCount = decodeBillingCount(
    outbox.attempt_count,
    `billing-mutation-outbox-attempt-count:${outbox.request_key}`
  );
  if (attemptCount >= MAX_BILLING_OUTBOX_ATTEMPTS) {
    throw new BillingReadModelUnavailableError(`billing-mutation-outbox-manual-review:${outbox.request_key}`);
  }
  const database = requireBillingD1Database(env);
  const attemptedAt = new Date().toISOString();
  const leaseToken = crypto.randomUUID();
  const leaseExpiresAt = new Date(Date.now() + 60_000).toISOString();
  const claim = await database
    .prepare(MARK_MUTATION_OUTBOX_ATTEMPT_SQL)
    .bind(outbox.request_key, attemptedAt, leaseToken, leaseExpiresAt, MAX_BILLING_OUTBOX_ATTEMPTS)
    .run();
  if ((claim.meta?.changes ?? 0) !== 1) {
    return false;
  }
  try {
    const auditEvent = decodeCanonicalValue('billing-mutation-outbox-audit-event', () =>
      BillingSupportAdminAuditEventSummarySchema.parse(
        parseUnknownPayload(outbox.audit_event_json, 'billing-mutation-outbox-audit-event')
      )
    );
    await appendBillingAuditEvent(env, auditEvent);
    const delivered = await database
      .prepare(MARK_MUTATION_AUDIT_DELIVERED_SQL)
      .bind(outbox.request_key, new Date().toISOString(), leaseToken)
      .run();
    return (delivered.meta?.changes ?? 0) === 1;
  } catch (error) {
    const failedAt = new Date().toISOString();
    await database
      .prepare(MARK_MUTATION_OUTBOX_FAILURE_SQL)
      .bind(outbox.request_key, billingOutboxErrorMessage(error), failedAt, leaseToken)
      .run();
    throw error;
  }
}

export interface BillingMutationOutboxDrainSummary {
  scanned: number;
  delivered: number;
  failed: number;
  manualReview: number;
}

export async function drainPendingBillingMutationOutbox(env: Env): Promise<BillingMutationOutboxDrainSummary> {
  await ensureMutationSchema(env);
  const rows = await d1All<MutationOutboxRow>(
    requireBillingD1Database(env),
    SELECT_PENDING_MUTATION_OUTBOX_SQL,
    MAX_BILLING_OUTBOX_DRAIN_ROWS,
    MAX_BILLING_OUTBOX_ATTEMPTS,
    new Date().toISOString()
  );
  const manualReviewRow = await d1First<ManualReviewOutboxCountRow>(
    requireBillingD1Database(env),
    SELECT_MANUAL_REVIEW_MUTATION_OUTBOX_SQL,
    MAX_BILLING_OUTBOX_ATTEMPTS
  );
  const summary: BillingMutationOutboxDrainSummary = {
    scanned: rows.length,
    delivered: 0,
    failed: 0,
    manualReview: decodeBillingCount(
      manualReviewRow?.manual_review_count ?? 0,
      'billing-mutation-outbox-manual-review-count'
    ),
  };
  for (const row of rows) {
    try {
      const outbox = decodeMutationOutboxRow(row);
      const attemptCount = decodeBillingCount(
        outbox.attempt_count,
        `billing-mutation-outbox-attempt-count:${outbox.request_key}`
      );
      if (attemptCount >= MAX_BILLING_OUTBOX_ATTEMPTS) {
        summary.manualReview += 1;
        continue;
      }
      if (await deliverBillingMutationOutbox(env, outbox)) {
        summary.delivered += 1;
      }
    } catch (_error) {
      summary.failed += 1;
    }
  }
  env.ANALYTICS?.writeDataPoint({
    indexes: ['billing-mutation-outbox-drain'],
    doubles: [summary.scanned, summary.delivered, summary.failed, summary.manualReview],
  });
  return summary;
}

async function replayKnownBillingMutation(env: Env, mutation: BillingStateMutation): Promise<boolean> {
  await ensureMutationSchema(env);
  const outbox = await loadMutationOutbox(env, mutation);
  if (!outbox) {
    return false;
  }
  const storedMutation = parseUnknownPayload(outbox.mutation_json, 'billing-mutation-outbox-replay-mutation');
  if (JSON.stringify(stableMutationValue(storedMutation)) !== mutationFingerprint(mutation)) {
    throw new BillingReadModelUnavailableError(
      `billing-mutation-outbox-fingerprint-mismatch:${mutationReplayKey(mutation)}`
    );
  }
  await completeBillingMutation(env, mutation);
  return true;
}

async function adminAccountProjectionStatement(
  env: Env,
  status: BillingStatusSummary
): Promise<D1PreparedStatement | null> {
  const adminAccounts = await loadAdminBillingAccounts(env, null);
  const current = adminAccounts.find(
    (entry) => entry.parentAccountRef === status.parentAccountRef && entry.familyRef === status.familyRef
  );
  if (!current) {
    return null;
  }

  return billingAdminAccountStatement(env, {
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

export function isIgnoredProviderWebhookEvent(eventType: string): boolean {
  return providerWebhookTransition(eventType) === 'ignore';
}

function hasManualReviewAuthority(status: BillingStatusSummary, snapshot: BillingEntitlementSnapshotSummary): boolean {
  return (
    status.accountStatus === 'manual-review' ||
    status.portalVisibleState === 'manual-required' ||
    status.parentVisibleState === 'manual-review' ||
    status.localSafetyBehavior === 'manual-review-with-local-safety' ||
    status.source === 'manual-admin-review' ||
    status.failureState?.retryAllowed === false ||
    snapshot.signatureState === 'manual-required' ||
    snapshot.parentVisibleState === 'manual-review' ||
    snapshot.localSafetyBehavior === 'manual-review-with-local-safety' ||
    snapshot.source === 'manual-admin-review' ||
    snapshot.failureState?.retryAllowed === false
  );
}

async function billingTerminalStateBlocker(
  env: Env,
  status: BillingStatusSummary,
  snapshot: BillingEntitlementSnapshotSummary,
  invoices: ReadonlyArray<BillingInvoiceSummary>,
  adminInvoices: ReadonlyArray<AdminBillingInvoiceSummary>,
  authoritativeInvoiceId: string | null = null
): Promise<string | null> {
  const scopedAdminInvoices = adminInvoices.filter(
    (invoice) => invoice.parentAccountRef === status.parentAccountRef && invoice.familyRef === status.familyRef
  );
  let settledRefund = false;
  const invoiceIds = new Set([
    ...invoices.map((invoice) => invoice.invoiceId),
    ...scopedAdminInvoices.map((invoice) => invoice.invoiceId),
  ]);
  if (authoritativeInvoiceId !== null) {
    invoiceIds.add(decodeNonEmptyString(authoritativeInvoiceId, 'billing-provider-authoritative-invoice'));
  }
  for (const invoiceId of invoiceIds) {
    const ledger = await loadRefundLedgerSummary(env, invoiceId);
    const relatedInvoices = [
      ...invoices.filter((invoice) => invoice.invoiceId === invoiceId),
      ...scopedAdminInvoices.filter((invoice) => invoice.invoiceId === invoiceId),
    ];
    const hasRefundedInvoice = relatedInvoices.some((entry) => entry.paymentState === 'refunded');
    const hasUnrefundedInvoice = relatedInvoices.some((entry) => entry.paymentState !== 'refunded');
    if (ledger.finalRefundState === 'refund-settled') {
      if (!hasRefundedInvoice || hasUnrefundedInvoice) {
        throw new BillingReadModelUnavailableError(`billing-refund-final-state-mismatch:${invoiceId}`);
      }
      settledRefund = true;
    } else if (hasRefundedInvoice) {
      throw new BillingReadModelUnavailableError(`billing-refund-final-state-missing:${invoiceId}`);
    }
  }

  if (settledRefund) {
    return 'billing-refund-settled-manual-review';
  }
  if (hasManualReviewAuthority(status, snapshot)) {
    return 'billing-manual-review-authority';
  }
  return null;
}

async function assertBillingMutationNotTerminal(env: Env, mutation: BillingStateMutation): Promise<void> {
  if (
    mutation.kind !== 'change-plan' &&
    (mutation.kind !== 'cancel' || mutation.cancellationState === 'manual-review-required')
  ) {
    return;
  }
  const paired = decodeBillingStatePair(
    await loadBillingStatusSummary(env, mutation.subject),
    await loadBillingEntitlementSnapshot(env, mutation.subject),
    `billing-mutation-terminal-state:${mutation.subject}`
  );
  const blocker = await billingTerminalStateBlocker(
    env,
    paired.status,
    paired.snapshot,
    await loadBillingInvoices(env, mutation.subject),
    await loadAdminBillingInvoices(env, null)
  );
  if (blocker) {
    throw new BillingReadModelUnavailableError(`${blocker}:${mutation.kind}`);
  }
}

function manualReviewFailureState(): BillingStatusSummary['failureState'] {
  return {
    failureKind: 'provider-unavailable',
    parentResolution: 'manual-support-review',
    retryAllowed: false,
    retryAfter: null,
  };
}

function providerManualReviewStatePair(
  status: BillingStatusSummary,
  snapshot: BillingEntitlementSnapshotSummary,
  auditReference: string,
  updatedAt: string,
  warning: string
): { status: BillingStatusSummary; snapshot: BillingEntitlementSnapshotSummary } {
  const failureState = manualReviewFailureState();
  return {
    status: {
      ...status,
      accountStatus: 'manual-review',
      subscriptionStatus: 'past-due',
      portalVisibleState: 'manual-required',
      parentVisibleState: 'manual-review',
      localSafetyBehavior: 'manual-review-with-local-safety',
      source: 'manual-admin-review',
      failureState,
      nextRenewalAt: null,
      warnings: withAddedWarning(status.warnings, warning),
      auditReference,
      updatedAt,
    } as unknown as BillingStatusSummary,
    snapshot: {
      ...snapshot,
      subscriptionStatus: 'past-due',
      source: 'manual-admin-review',
      signatureState: 'manual-required',
      parentVisibleState: 'manual-review',
      localSafetyBehavior: 'manual-review-with-local-safety',
      failureState,
      signedAt: updatedAt,
      auditReference: `${auditReference}:snapshot`,
    } as unknown as BillingEntitlementSnapshotSummary,
  };
}

function manualInvoiceSummaryId(subject: string, requestId: string): string {
  return `${subject.replaceAll(':', '-')}-manual-invoice-${requestId}`;
}

function referralInviteSummaryId(referralCode: string, requestId: string): string {
  return `${referralCode}-invite-${requestId}`;
}

function hostedSessionAuditEventType(sessionKind: 'checkout-session-create' | 'billing-portal-session-create'): string {
  return sessionKind === 'checkout-session-create'
    ? 'billing.checkout-session.created'
    : 'billing.portal-session.created';
}

async function seedD1Tables(database: D1Database, patch: BillingBindingSeedPatch): Promise<void> {
  const statements: D1PreparedStatement[] = [];
  if (patch.statusBySubject) {
    for (const [subject, row] of Object.entries(patch.statusBySubject)) {
      const decoded = decodeBillingStatusSummary(row, `billing-status-seed:${subject}`, subject);
      statements.push(database.prepare(UPSERT_STATUS_SQL).bind(subject, JSON.stringify(decoded)));
    }
  }
  if (patch.snapshotsBySubject) {
    for (const [subject, snapshot] of Object.entries(patch.snapshotsBySubject)) {
      const decoded = decodeBillingEntitlementSnapshot(
        snapshot,
        `billing-entitlement-snapshot-seed:${subject}`,
        subject
      );
      statements.push(database.prepare(UPSERT_SNAPSHOT_SQL).bind(subject, JSON.stringify(decoded)));
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
  if (patch.invoicesBySubject) {
    for (const [subject, invoices] of Object.entries(patch.invoicesBySubject)) {
      for (const [index, invoice] of invoices.entries()) {
        const decoded = decodeBillingInvoiceSummary(invoice, `billing-invoice-seed:${subject}-${index}`);
        statements.push(database.prepare(UPSERT_INVOICE_SQL).bind(subject, decoded.invoiceId, JSON.stringify(decoded)));
      }
    }
  }
  if (patch.referralsBySubject) {
    for (const [subject, referral] of Object.entries(patch.referralsBySubject)) {
      const decoded = decodeBillingReferralSummary(referral, `billing-referral-seed:${subject}`);
      statements.push(database.prepare(UPSERT_REFERRAL_SQL).bind(subject, JSON.stringify(decoded)));
    }
  }
  for (const [index, row] of (patch.adminAccounts ?? []).entries()) {
    const decoded = decodeAdminBillingAccountSummary(row, `billing-admin-account-seed-${index}`);
    statements.push(database.prepare(UPSERT_ADMIN_ACCOUNT_SQL).bind(decoded.parentAccountRef, JSON.stringify(decoded)));
  }
  for (const [index, row] of (patch.adminInvoices ?? []).entries()) {
    const decoded = decodeAdminBillingInvoiceSummary(row, `billing-admin-invoice-seed-${index}`);
    statements.push(database.prepare(UPSERT_ADMIN_INVOICE_SQL).bind(decoded.invoiceId, JSON.stringify(decoded)));
  }
  for (const [index, row] of (patch.adminDisputes ?? []).entries()) {
    const decoded = decodeAdminBillingDisputeSummary(row, `billing-admin-dispute-seed-${index}`);
    statements.push(database.prepare(UPSERT_ADMIN_DISPUTE_SQL).bind(decoded.disputeId, JSON.stringify(decoded)));
  }
  for (const row of patch.adminReferrals ?? []) {
    const decoded = decodeCanonicalValue('billing-admin-referral-seed', () =>
      BillingSupportAdminReferralSummarySchema.parse(row)
    );
    statements.push(database.prepare(UPSERT_ADMIN_REFERRAL_SQL).bind(decoded.referralCode, JSON.stringify(decoded)));
  }
  if (statements.length > 0) {
    await database.batch(statements);
  }
}

async function ensureReadModelSeedOnce(env: Env): Promise<void> {
  if (env.BILLING_D1) {
    await env.BILLING_D1.exec(CREATE_READ_MODEL_SCHEMA_SQL);
    if (!isLocalFixtureEnvironment(env)) {
      await ensureProviderEventReceiptSchema(env.BILLING_D1);
    }
  }
  if (!isLocalFixtureEnvironment(env)) {
    return;
  }
  const patch = buildDefaultBillingBindingSeed(env);

  if (env.BILLING_D1) {
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
  const stored = (await d1All<PayloadJsonRow>(env.BILLING_D1, SELECT_INVOICES_BY_SUBJECT_SQL, subject)).map(
    (row, index) =>
      decodeBillingInvoiceSummary(
        parseUnknownPayload(row.payload_json, `billing-invoice:${subject}`),
        `billing-invoice:${subject}-${index}`
      )
  );
  await recordBindingRead(env, 'billing-invoices', subject);
  return stored.length > 0 || !isLocalFixtureEnvironment(env)
    ? stored
    : buildBillingInvoices(subject).map((invoice, index) =>
        decodeBillingInvoiceSummary(invoice, `billing-local-invoice:${subject}-${index}`)
      );
}

export async function loadBillingInvoiceById(env: Env, invoiceId: string): Promise<BillingInvoiceSummary | null> {
  await ensureReadModelSeed(env);
  requireProductionBinding(env, 'BILLING_D1');
  const stored = parseUnknownPayloadRow(
    await d1First<PayloadJsonRow>(env.BILLING_D1, SELECT_INVOICE_BY_ID_SQL, invoiceId),
    `billing-invoice:${invoiceId}`
  );
  await recordBindingRead(env, 'billing-invoice', invoiceId);
  if (stored !== null) {
    return decodeBillingInvoiceSummary(stored, `billing-invoice:${invoiceId}`);
  }
  if (!isLocalFixtureEnvironment(env)) {
    return null;
  }
  for (const subject of DEFAULT_BILLING_SUBJECTS) {
    const invoice = (await loadBillingInvoices(env, subject)).find((row) => row.invoiceId === invoiceId);
    if (invoice) {
      return invoice;
    }
  }
  return null;
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

export async function buildBillingReferralInviteResultFromD1(
  env: Env,
  subject: string,
  requestId: string,
  invitee: string
): Promise<BillingReferralInviteResult> {
  const referral = await loadBillingReferralSummary(env, subject);
  const normalizedInvitee = decodeNonEmptyString(invitee, 'billing-referral-invitee').trim().toLowerCase();
  const referralCode = decodeNonEmptyString(referral.referralCode, `billing-referral-code:${subject}`);
  const auditReference = decodeNonEmptyString(referral.auditReference, `billing-referral-audit:${subject}`);
  if (normalizedInvitee === subject.trim().toLowerCase()) {
    return BillingReferralInviteResultSchema.parse({
      requestId,
      status: 'rejected',
      inviteState: 'fraud-review',
      referralCode,
      rejectionReason: 'self-referral-rejected',
      auditReference: `${auditReference}:invite-self-rejected`,
    });
  }
  return BillingReferralInviteResultSchema.parse({
    requestId,
    status: 'accepted',
    inviteState: 'invite-created',
    referralCode,
    rejectionReason: null,
    auditReference: `${auditReference}:invite-created`,
  });
}

export interface BillingManualInvoiceResult {
  requestId: string;
  status: 'accepted';
  invoiceState: 'manual-support-required';
  region: string;
  provider: 'manual-invoice';
  auditReference: string;
}

export async function buildManualInvoiceResultFromD1(
  env: Env,
  subject: string,
  requestId: string,
  region: string
): Promise<BillingManualInvoiceResult> {
  const status = await loadBillingStatusSummary(env, subject);
  const normalizedRegion = decodeNonEmptyString(region, 'billing-manual-invoice-region').trim();
  return {
    requestId,
    status: 'accepted',
    invoiceState: 'manual-support-required',
    region: normalizedRegion,
    provider: 'manual-invoice',
    auditReference: `${status.auditReference}:manual-invoice`,
  };
}

export async function buildReconciliationSummaryFromD1(
  env: Env,
  requestId: string
): Promise<BillingSupportAdminReconciliationSummary> {
  await ensureReadModelSeed(env);
  await ensureMutationSchema(env);
  const database = requireBillingD1Database(env);
  const statuses = (await d1All<PayloadJsonRow>(database, SELECT_ALL_STATUS_SQL)).map((row, index) =>
    decodeBillingStatusSummary(
      parseUnknownPayload(row.payload_json, `billing-reconciliation-status-${index}`),
      `billing-reconciliation-status-${index}`
    )
  );
  const retryBacklog = await d1First<RowCountRow>(
    database,
    SELECT_PENDING_MUTATION_OUTBOX_COUNT_SQL,
    MAX_BILLING_OUTBOX_ATTEMPTS
  );
  const manualReviewOutbox = await d1First<RowCountRow>(
    database,
    SELECT_MANUAL_REVIEW_MUTATION_OUTBOX_SQL,
    MAX_BILLING_OUTBOX_ATTEMPTS
  );
  const providerManualReview = await d1First<RowCountRow>(database, SELECT_MANUAL_REVIEW_PROVIDER_RECEIPT_COUNT_SQL);
  return BillingSupportAdminReconciliationSummarySchema.parse({
    requestId: decodeNonEmptyString(requestId, 'billing-reconciliation-request-id'),
    status: 'accepted',
    queued: true,
    driftFamiliesVisible: statuses.filter(
      (status) => status.accountStatus !== 'active' || status.parentVisibleState !== 'available'
    ).length,
    retryBacklogVisible: decodeBillingCount(retryBacklog?.row_count ?? 0, 'billing-reconciliation-retry-backlog'),
    deadLetterVisible:
      decodeBillingCount(manualReviewOutbox?.row_count ?? 0, 'billing-reconciliation-dead-letter-outbox') +
      decodeBillingCount(providerManualReview?.row_count ?? 0, 'billing-reconciliation-dead-letter-provider'),
    auditReference: `audit:billing:reconciliation:${requestId}`,
  });
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
  const paired = decodeBillingStatePair(
    await loadBillingStatusSummary(env, subject),
    await loadBillingEntitlementSnapshot(env, subject),
    `billing-license-authority:${subject}`
  );
  const status = paired.status;
  const snapshot = paired.snapshot;
  correlateBillingAuthority(status, snapshot, `billing-license-authority:${subject}`);
  const requestedDeviceAlreadyTrusted = !requestedNewDevice;
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
  const stored = (await d1All<PayloadJsonRow>(env.BILLING_D1, SELECT_ADMIN_INVOICES_SQL)).map((row, index) =>
    decodeAdminBillingInvoiceSummary(
      parseUnknownPayload(row.payload_json, 'billing-admin-invoice'),
      `billing-admin-invoice-${index}`
    )
  );
  await recordBindingRead(env, 'admin-billing-invoices', null);
  const rows =
    stored.length > 0 || !isLocalFixtureEnvironment(env)
      ? stored
      : listAdminBillingInvoices(query).map((invoice, index) =>
          decodeAdminBillingInvoiceSummary(invoice, `billing-local-admin-invoice-${index}`)
        );
  if (!loweredQuery) {
    return rows;
  }
  return rows.filter((row) =>
    includesQuery([row.invoiceId, row.invoiceNumber, row.parentAccountRef, row.familyRef, row.planId], loweredQuery)
  );
}

export function buildBillingRefundResult(
  requestId: string,
  invoice: BillingInvoiceSummary | null,
  amountCents: number | null,
  appliedAmountCents = 0
): BillingSupportAdminRefundResult {
  if (!invoice) {
    return BillingSupportAdminRefundResultSchema.parse({
      requestId,
      status: 'rejected',
      invoiceId: null,
      refundState: 'manual-review-required',
      amountCents: null,
      auditReference: 'audit:refund:rejected',
      rejectionReason: 'invoice-not-found',
    });
  }

  const decodedAppliedAmount = decodeBillingCount(appliedAmountCents, 'billing-refund-applied-amount');
  if (decodedAppliedAmount > invoice.totalCents) {
    return BillingSupportAdminRefundResultSchema.parse({
      requestId,
      status: 'rejected',
      invoiceId: invoice.invoiceId,
      refundState: 'manual-review-required',
      amountCents: null,
      auditReference: `${invoice.auditReference}:refund:ledger-invalid`,
      rejectionReason: null,
    });
  }
  const remainingAmount = invoice.totalCents - decodedAppliedAmount;
  const decodedAmount = decodeBillingCount(amountCents ?? remainingAmount, 'billing-refund-amount');
  const cumulativeAmount = decodeBillingCount(decodedAppliedAmount + decodedAmount, 'billing-refund-cumulative-amount');
  if (decodedAmount === 0 || cumulativeAmount > invoice.totalCents) {
    return BillingSupportAdminRefundResultSchema.parse({
      requestId,
      status: 'rejected',
      invoiceId: invoice.invoiceId,
      refundState: 'manual-review-required',
      amountCents: null,
      auditReference: `${invoice.auditReference}:refund:manual-review`,
      rejectionReason: null,
    });
  }

  return BillingSupportAdminRefundResultSchema.parse({
    requestId,
    status: 'accepted',
    invoiceId: invoice.invoiceId,
    refundState: cumulativeAmount < invoice.totalCents ? 'refund-requested' : 'refund-settled',
    amountCents: decodedAmount,
    auditReference: `${invoice.auditReference}:refund:${decodedAmount}:cumulative:${cumulativeAmount}`,
    rejectionReason: null,
  });
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
  if (await replayKnownBillingMutation(env, mutation)) {
    return;
  }
  await assertBillingMutationNotTerminal(env, mutation);

  switch (mutation.kind) {
    case 'hosted-session': {
      const auditEvent = await billingAuditEventForMutation(env, mutation);
      await commitBillingMutationD1Batch(env, mutation, [], auditEvent);
      await completeBillingMutation(env, mutation);
      return;
    }
    case 'change-plan': {
      const status = await loadBillingStatusSummary(env, mutation.subject);
      const snapshot = await loadBillingEntitlementSnapshot(env, mutation.subject);
      const pricingPlans = (await loadPricingPlans(env)).filter((plan) => plan.activeState === 'active');
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

      const changePlanStatements = [...billingStatePairStatements(env, nextStatus, nextSnapshot)];
      const changePlanProjection = await adminAccountProjectionStatement(env, nextStatus);
      if (changePlanProjection) {
        changePlanStatements.push(changePlanProjection);
      }
      const auditEvent = await billingAuditEventForMutation(env, mutation);
      await commitBillingMutationD1Batch(env, mutation, changePlanStatements, auditEvent);
      await completeBillingMutation(env, mutation);
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

      const cancellationStatements = [...billingStatePairStatements(env, nextStatus, nextSnapshot)];
      const cancellationProjection = await adminAccountProjectionStatement(env, nextStatus);
      if (cancellationProjection) {
        cancellationStatements.push(cancellationProjection);
      }
      const auditEvent = await billingAuditEventForMutation(env, mutation);
      await commitBillingMutationD1Batch(env, mutation, cancellationStatements, auditEvent);
      await completeBillingMutation(env, mutation);
      return;
    }
    case 'referral-invite': {
      const referral = await loadBillingReferralSummary(env, mutation.subject);
      if (referral.referralCode !== mutation.referralCode) {
        throw new BillingReadModelUnavailableError(`billing-referral-code-subject-mismatch:${mutation.referralCode}`);
      }
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
      if (existingAdminReferral && existingAdminReferral.ownerSubject !== mutation.subject) {
        throw new BillingReadModelUnavailableError(`billing-referral-owner-mismatch:${mutation.referralCode}`);
      }
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

      const auditEvent = await billingAuditEventForMutation(env, mutation);
      await commitBillingMutationD1Batch(
        env,
        mutation,
        [billingReferralStatement(env, nextReferral), billingAdminReferralStatement(env, nextAdminReferral)],
        auditEvent
      );
      await completeBillingMutation(env, mutation);
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

      const manualInvoiceStatements = [
        ...billingStatePairStatements(env, nextStatus, nextSnapshot),
        billingInvoiceStatement(env, mutation.subject, nextInvoice),
        billingAdminInvoiceStatement(env, nextAdminInvoice),
      ];
      const manualInvoiceProjection = await adminAccountProjectionStatement(env, nextStatus);
      if (manualInvoiceProjection) {
        manualInvoiceStatements.push(manualInvoiceProjection);
      }
      const auditEvent = await billingAuditEventForMutation(env, mutation);
      await commitBillingMutationD1Batch(env, mutation, manualInvoiceStatements, auditEvent);
      await completeBillingMutation(env, mutation);
      return;
    }
    case 'admin-refund': {
      const paired = decodeBillingStatePair(
        await loadBillingStatusSummary(env, mutation.subject),
        await loadBillingEntitlementSnapshot(env, mutation.subject),
        `billing-refund-authority:${mutation.subject}`
      );
      const status = paired.status;
      const snapshot = paired.snapshot;
      const invoices = await loadBillingInvoices(env, mutation.subject);
      const adminInvoices = await loadAdminBillingInvoices(env, null);
      const matchedInvoice = invoices.find((invoice) => invoice.invoiceId === mutation.invoiceId);
      if (!matchedInvoice) {
        throw new BillingReadModelUnavailableError(`billing-refund-invoice-missing:${mutation.invoiceId}`);
      }
      if (mutation.currency !== matchedInvoice.currency) {
        throw new BillingReadModelUnavailableError(`billing-refund-currency-mismatch:${mutation.invoiceId}`);
      }
      const refundLedger = await loadRefundLedgerSummary(env, mutation.invoiceId);
      const appliedAmountCents = refundLedger.appliedAmountCents;
      if (appliedAmountCents > matchedInvoice.totalCents) {
        throw new BillingReadModelUnavailableError(`billing-refund-ledger-exceeds-invoice:${mutation.invoiceId}`);
      }
      if (
        (refundLedger.finalRefundState === 'refund-settled' && appliedAmountCents < matchedInvoice.totalCents) ||
        (refundLedger.finalRefundState === 'refund-requested' && appliedAmountCents >= matchedInvoice.totalCents) ||
        (refundLedger.finalRefundState === 'refund-settled' && matchedInvoice.paymentState !== 'refunded') ||
        (refundLedger.finalRefundState === 'refund-requested' && matchedInvoice.paymentState === 'refunded')
      ) {
        throw new BillingReadModelUnavailableError(`billing-refund-ledger-state-mismatch:${mutation.invoiceId}`);
      }
      const cumulativeAmountCents = decodeBillingCount(
        appliedAmountCents + mutation.amountCents,
        `billing-refund-cumulative-amount:${mutation.invoiceId}`
      );
      if (mutation.amountCents === 0 || cumulativeAmountCents > matchedInvoice.totalCents) {
        throw new BillingReadModelUnavailableError(`billing-refund-amount-exceeds-invoice:${mutation.invoiceId}`);
      }
      const expectedRefundState =
        cumulativeAmountCents < matchedInvoice.totalCents ? 'refund-requested' : 'refund-settled';
      if (mutation.refundState !== expectedRefundState) {
        throw new BillingReadModelUnavailableError(`billing-refund-state-mismatch:${mutation.invoiceId}`);
      }
      const updatedAt = new Date().toISOString();
      const refundSettled = mutation.refundState === 'refund-settled';
      const auditReference = `${mutation.auditReference}:state`;
      const nextFailureState = refundSettled ? manualReviewFailureState() : status.failureState;
      const refundAmountAudit = `amount-cents:${mutation.amountCents}:cumulative-cents:${cumulativeAmountCents}`;
      const refundStatements = [
        billingRefundLedgerStatement(env, mutation, matchedInvoice.totalCents, updatedAt),
        billingInvoiceStatement(env, mutation.subject, {
          ...matchedInvoice,
          paymentState: refundSettled ? 'refunded' : matchedInvoice.paymentState,
          auditReference: `${auditReference}:invoice:${matchedInvoice.invoiceId}:${refundAmountAudit}`,
          updatedAt,
        }),
      ];
      const matchedAdminInvoice = adminInvoices.find((invoice) => invoice.invoiceId === mutation.invoiceId);
      if (matchedAdminInvoice) {
        refundStatements.push(
          billingAdminInvoiceStatement(env, {
            ...matchedAdminInvoice,
            paymentState: refundSettled ? 'refunded' : matchedAdminInvoice.paymentState,
            auditReference: `${auditReference}:admin-invoice:${matchedAdminInvoice.invoiceId}:${refundAmountAudit}`,
            updatedAt,
          })
        );
      }

      if (refundSettled) {
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

        refundStatements.push(...billingStatePairStatements(env, nextStatus, nextSnapshot));
        const refundProjection = await adminAccountProjectionStatement(env, nextStatus);
        if (refundProjection) {
          refundStatements.push(refundProjection);
        }
      }

      const auditEvent = await billingAuditEventForMutation(env, mutation);
      await commitBillingMutationD1Batch(env, mutation, refundStatements, auditEvent);
      await completeBillingMutation(env, mutation);
      return;
    }
    case 'reconciliation': {
      const auditEvent = await billingAuditEventForMutation(env, mutation);
      await commitBillingMutationD1Batch(env, mutation, [], auditEvent);
      await completeBillingMutation(env, mutation);
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
      if (
        (mutation.parentAccountRef !== null &&
          mutation.parentAccountRef !== undefined &&
          mutation.parentAccountRef !== status.parentAccountRef) ||
        (mutation.familyRef !== null && mutation.familyRef !== undefined && mutation.familyRef !== status.familyRef)
      ) {
        throw new BillingReadModelUnavailableError(`billing-provider-authority-mismatch:${mutation.eventId}`);
      }
      if (
        mutation.invoiceId !== null &&
        mutation.invoiceId !== undefined &&
        !invoices.some((invoice) => invoice.invoiceId === mutation.invoiceId) &&
        !adminInvoices.some(
          (invoice) =>
            invoice.invoiceId === mutation.invoiceId &&
            invoice.parentAccountRef === status.parentAccountRef &&
            invoice.familyRef === status.familyRef
        )
      ) {
        throw new BillingReadModelUnavailableError(`billing-provider-invoice-subject-mismatch:${mutation.invoiceId}`);
      }
      const updatedAt = new Date().toISOString();
      const auditReference = `${status.auditReference}:provider-webhook:${mutation.provider}`;
      const auditEvent = await billingAuditEventForMutation(env, mutation);
      const recoveryBlocker =
        transition === 'activate-subscription' || transition === 'enter-grace' || transition === 'dispute-won'
          ? await billingTerminalStateBlocker(
              env,
              status,
              snapshot,
              invoices,
              adminInvoices,
              mutation.invoiceId ?? null
            )
          : null;
      if (recoveryBlocker) {
        env.ANALYTICS?.writeDataPoint({
          indexes: ['billing-provider-recovery-blocked'],
          blobs: [recoveryBlocker],
          doubles: [1],
        });
        await commitBillingMutationD1Batch(env, mutation, [], auditEvent);
        await completeBillingMutation(env, mutation);
        return;
      }

      if (transition === 'activate-subscription') {
        const { status: nextStatus, snapshot: nextSnapshot } = providerManualReviewStatePair(
          status,
          snapshot,
          auditReference,
          updatedAt,
          'provider-webhook-verifier-required'
        );

        const activationStatements = [...billingStatePairStatements(env, nextStatus, nextSnapshot)];
        for (const invoice of invoices) {
          const nextInvoice = {
            ...invoice,
            paymentState: invoice.provider === 'manual-invoice' ? invoice.paymentState : 'paid',
            auditReference: `${auditReference}:invoice:${invoice.invoiceId}`,
            updatedAt,
          } as unknown as BillingInvoiceSummary;
          activationStatements.push(billingInvoiceStatement(env, mutation.subject, nextInvoice));
        }
        for (const invoice of adminInvoices.filter(
          (entry) => entry.parentAccountRef === status.parentAccountRef && entry.familyRef === status.familyRef
        )) {
          const nextInvoice: any = {
            ...invoice,
            paymentState: invoice.provider === 'manual-invoice' ? invoice.paymentState : 'paid',
            manualRequired: true,
            auditReference: `${auditReference}:admin-invoice:${invoice.invoiceId}`,
            updatedAt,
          };
          activationStatements.push(billingAdminInvoiceStatement(env, nextInvoice));
        }
        const activationProjection = await adminAccountProjectionStatement(env, nextStatus);
        if (activationProjection) {
          activationStatements.push(activationProjection);
        }
        await commitBillingMutationD1Batch(env, mutation, activationStatements, auditEvent);
      } else if (transition === 'enter-grace') {
        const { status: nextStatus, snapshot: nextSnapshot } = providerManualReviewStatePair(
          status,
          snapshot,
          auditReference,
          updatedAt,
          'provider-webhook-verifier-required'
        );

        const graceStatements = [...billingStatePairStatements(env, nextStatus, nextSnapshot)];
        for (const invoice of invoices) {
          const nextInvoice = {
            ...invoice,
            paymentState: invoice.provider === 'manual-invoice' ? invoice.paymentState : 'grace',
            auditReference: `${auditReference}:invoice:${invoice.invoiceId}`,
            updatedAt,
          } as unknown as BillingInvoiceSummary;
          graceStatements.push(billingInvoiceStatement(env, mutation.subject, nextInvoice));
        }
        for (const invoice of adminInvoices.filter(
          (entry) => entry.parentAccountRef === status.parentAccountRef && entry.familyRef === status.familyRef
        )) {
          const nextInvoice: any = {
            ...invoice,
            paymentState: invoice.provider === 'manual-invoice' ? invoice.paymentState : 'grace',
            manualRequired: invoice.invoiceVisibility === 'manual-support-required',
            auditReference: `${auditReference}:admin-invoice:${invoice.invoiceId}`,
            updatedAt,
          };
          graceStatements.push(billingAdminInvoiceStatement(env, nextInvoice));
        }
        const graceProjection = await adminAccountProjectionStatement(env, nextStatus);
        if (graceProjection) {
          graceStatements.push(graceProjection);
        }
        await commitBillingMutationD1Batch(env, mutation, graceStatements, auditEvent);
      } else {
        if (transition.startsWith('dispute-') && !mutation.invoiceId) {
          throw new BillingReadModelUnavailableError(`billing-provider-dispute-invoice-unresolved:${mutation.eventId}`);
        }
        const disputeId = mutation.disputeId ?? `dispute-${mutation.provider}-${mutation.eventId}`;
        const invoiceId = mutation.invoiceId;
        if (!invoiceId) {
          throw new BillingReadModelUnavailableError(`billing-provider-invoice-unresolved:${mutation.eventId}`);
        }
        if (
          !invoices.some((invoice) => invoice.invoiceId === invoiceId) &&
          !adminInvoices.some(
            (invoice) =>
              invoice.invoiceId === invoiceId &&
              invoice.parentAccountRef === status.parentAccountRef &&
              invoice.familyRef === status.familyRef
          )
        ) {
          throw new BillingReadModelUnavailableError(`billing-provider-invoice-subject-mismatch:${invoiceId}`);
        }
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
              : 'manual-review-required';
        const manualRequired = true;
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
        const disputeStatements = [billingAdminDisputeStatement(env, nextDispute)];

        const providerStatePair = providerManualReviewStatePair(
          status,
          snapshot,
          auditReference,
          updatedAt,
          transition === 'dispute-opened'
            ? 'provider-webhook-dispute-opened'
            : transition === 'dispute-lost'
              ? 'provider-webhook-dispute-lost'
              : 'provider-webhook-verifier-required'
        );
        const nextStatus = providerStatePair.status;
        const nextSnapshot = providerStatePair.snapshot;

        disputeStatements.push(...billingStatePairStatements(env, nextStatus, nextSnapshot));
        const disputeProjection = await adminAccountProjectionStatement(env, nextStatus);
        if (disputeProjection) {
          disputeStatements.push(disputeProjection);
        }
        await commitBillingMutationD1Batch(env, mutation, disputeStatements, auditEvent);
      }

      await completeBillingMutation(env, mutation);
      return;
    }
  }
}
