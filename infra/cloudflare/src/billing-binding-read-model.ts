import type {
  AnalyticsEngineDataset,
  D1Database,
  D1PreparedStatement,
  KVNamespace,
  R2Bucket,
} from '@cloudflare/workers-types';
import type { BillingReferralSummary } from './generated/billing-contracts.js';
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
  type BillingInvoiceSummary,
  type BillingLicenseDecisionSummary,
  type BillingStatusSummary,
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

export function isLocalFixtureEnvironment(environment: string): boolean {
  return environment === 'local' || environment === 'test';
}

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
const INSERT_MISSING_STATUS_SQL = normalizeSql(
  'INSERT OR IGNORE INTO billing_status (subject, payload_json) VALUES (?1, ?2)'
);
const INSERT_MISSING_INVOICE_SQL = normalizeSql(
  'INSERT OR IGNORE INTO billing_invoices (subject, invoice_id, payload_json) VALUES (?1, ?2, ?3)'
);
const INSERT_MISSING_REFERRAL_SQL = normalizeSql(
  'INSERT OR IGNORE INTO billing_referrals (subject, payload_json) VALUES (?1, ?2)'
);
const INSERT_MISSING_SNAPSHOT_SQL = normalizeSql(
  'INSERT OR IGNORE INTO billing_snapshots (subject, payload_json) VALUES (?1, ?2)'
);
const INSERT_MISSING_ADMIN_ACCOUNT_SQL = normalizeSql(
  'INSERT OR IGNORE INTO billing_admin_accounts (parent_account_ref, payload_json) VALUES (?1, ?2)'
);
const INSERT_MISSING_ADMIN_INVOICE_SQL = normalizeSql(
  'INSERT OR IGNORE INTO billing_admin_invoices (invoice_id, payload_json) VALUES (?1, ?2)'
);
const INSERT_MISSING_ADMIN_DISPUTE_SQL = normalizeSql(
  'INSERT OR IGNORE INTO billing_admin_disputes (dispute_id, payload_json) VALUES (?1, ?2)'
);
const INSERT_MISSING_ADMIN_REFERRAL_SQL = normalizeSql(
  'INSERT OR IGNORE INTO billing_admin_referrals (referral_code, payload_json) VALUES (?1, ?2)'
);

const seedReadyByEnv = new WeakMap<Env, Promise<void>>();

interface PayloadJsonRow {
  payload_json: string;
}

interface RowCountRow {
  row_count: number | string;
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

function parsePayloadRow<T>(row: PayloadJsonRow | null): T | null {
  return row === null ? null : parsePayload<T>(row.payload_json);
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
      case UPSERT_STATUS_SQL:
      case INSERT_MISSING_STATUS_SQL: {
        const subject = String(this.values[0] ?? '');
        const payloadJson = String(this.values[1] ?? '{}');
        if (this.normalizedQuery === UPSERT_STATUS_SQL || !this.state.statusBySubject.has(subject)) {
          this.state.statusBySubject.set(subject, parsePayload<BillingStatusSummary>(payloadJson));
        }
        return;
      }
      case UPSERT_INVOICE_SQL:
      case INSERT_MISSING_INVOICE_SQL: {
        const subject = String(this.values[0] ?? '');
        const payloadJson = String(this.values[2] ?? '{}');
        const invoice = parsePayload<BillingInvoiceSummary>(payloadJson);
        const current = this.state.invoicesBySubject.get(subject) ?? [];
        if (
          this.normalizedQuery === UPSERT_INVOICE_SQL ||
          !current.some((entry) => entry.invoiceId === invoice.invoiceId)
        ) {
          this.state.invoicesBySubject.set(
            subject,
            replaceByKey(current, invoice, (entry) => entry.invoiceId)
          );
        }
        return;
      }
      case UPSERT_REFERRAL_SQL:
      case INSERT_MISSING_REFERRAL_SQL: {
        const subject = String(this.values[0] ?? '');
        const payloadJson = String(this.values[1] ?? '{}');
        if (this.normalizedQuery === UPSERT_REFERRAL_SQL || !this.state.referralsBySubject.has(subject)) {
          this.state.referralsBySubject.set(subject, parsePayload<BillingReferralSummary>(payloadJson));
        }
        return;
      }
      case UPSERT_SNAPSHOT_SQL:
      case INSERT_MISSING_SNAPSHOT_SQL: {
        const subject = String(this.values[0] ?? '');
        const payloadJson = String(this.values[1] ?? '{}');
        if (this.normalizedQuery === UPSERT_SNAPSHOT_SQL || !this.state.snapshotsBySubject.has(subject)) {
          this.state.snapshotsBySubject.set(subject, parsePayload<BillingEntitlementSnapshotSummary>(payloadJson));
        }
        return;
      }
      case UPSERT_ADMIN_ACCOUNT_SQL:
      case INSERT_MISSING_ADMIN_ACCOUNT_SQL: {
        const payloadJson = String(this.values[1] ?? '{}');
        const nextRow = parsePayload<AdminBillingAccountSummary>(payloadJson);
        if (
          this.normalizedQuery === UPSERT_ADMIN_ACCOUNT_SQL ||
          !this.state.adminAccounts.some((entry) => entry.parentAccountRef === nextRow.parentAccountRef)
        ) {
          this.state.adminAccounts = replaceByKey(this.state.adminAccounts, nextRow, (entry) => entry.parentAccountRef);
        }
        return;
      }
      case UPSERT_ADMIN_INVOICE_SQL:
      case INSERT_MISSING_ADMIN_INVOICE_SQL: {
        const payloadJson = String(this.values[1] ?? '{}');
        const nextRow = parsePayload<AdminBillingInvoiceSummary>(payloadJson);
        if (
          this.normalizedQuery === UPSERT_ADMIN_INVOICE_SQL ||
          !this.state.adminInvoices.some((entry) => entry.invoiceId === nextRow.invoiceId)
        ) {
          this.state.adminInvoices = replaceByKey(this.state.adminInvoices, nextRow, (entry) => entry.invoiceId);
        }
        return;
      }
      case UPSERT_ADMIN_DISPUTE_SQL:
      case INSERT_MISSING_ADMIN_DISPUTE_SQL: {
        const payloadJson = String(this.values[1] ?? '{}');
        const nextRow = parsePayload<AdminBillingDisputeSummary>(payloadJson);
        if (
          this.normalizedQuery === UPSERT_ADMIN_DISPUTE_SQL ||
          !this.state.adminDisputes.some((entry) => entry.disputeId === nextRow.disputeId)
        ) {
          this.state.adminDisputes = replaceByKey(this.state.adminDisputes, nextRow, (entry) => entry.disputeId);
        }
        return;
      }
      case UPSERT_ADMIN_REFERRAL_SQL:
      case INSERT_MISSING_ADMIN_REFERRAL_SQL: {
        const payloadJson = String(this.values[1] ?? '{}');
        const nextRow = parsePayload<AdminBillingReferralSummary>(payloadJson);
        if (
          this.normalizedQuery === UPSERT_ADMIN_REFERRAL_SQL ||
          !this.state.adminReferrals.some((entry) => entry.referralCode === nextRow.referralCode)
        ) {
          this.state.adminReferrals = replaceByKey(this.state.adminReferrals, nextRow, (entry) => entry.referralCode);
        }
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
    return Promise.all(statements.map((statement) => statement.run()));
  }

  async exec(): Promise<{ count: number; duration: number }> {
    return {
      count: 0,
      duration: 0,
    };
  }

  replaceSeed(patch: BillingBindingSeedPatch): void {
    if (patch.statusBySubject) {
      this.state.statusBySubject = mapFromRecord(patch.statusBySubject);
    }
    if (patch.invoicesBySubject) {
      this.state.invoicesBySubject = mapFromRecord(patch.invoicesBySubject);
    }
    if (patch.referralsBySubject) {
      this.state.referralsBySubject = mapFromRecord(patch.referralsBySubject);
    }
    if (patch.snapshotsBySubject) {
      this.state.snapshotsBySubject = mapFromRecord(patch.snapshotsBySubject);
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
    d1.replaceSeed(patch);
    if (patch.pricingPlans) {
      configKv.setRaw(PRICING_PLANS_KEY, JSON.stringify(patch.pricingPlans));
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
        return raw === null ? 0 : Number(raw);
      },
    },
  };
}

async function incrementTouchCounter(env: Env, counterKey: string): Promise<void> {
  const fullKey = `${TOUCH_KEY_PREFIX}${counterKey}`;
  const current = Number((await env.BILLING_RATE_LIMIT_KV?.get(fullKey)) ?? '0');
  await env.BILLING_RATE_LIMIT_KV?.put(fullKey, String(current + 1));
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

async function upsertBillingStatusSummary(env: Env, status: BillingStatusSummary): Promise<void> {
  await env.BILLING_D1?.prepare(UPSERT_STATUS_SQL).bind(status.subject, JSON.stringify(status)).run();
}

async function upsertBillingSnapshotSummary(env: Env, snapshot: BillingEntitlementSnapshotSummary): Promise<void> {
  await env.BILLING_D1?.prepare(UPSERT_SNAPSHOT_SQL).bind(snapshot.subject, JSON.stringify(snapshot)).run();
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
  | 'activate-subscription'
  | 'enter-grace'
  | 'dispute-opened'
  | 'dispute-lost'
  | 'dispute-won'
  | 'ignore';

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
  if (patch.statusBySubject) {
    for (const [subject, row] of Object.entries(patch.statusBySubject)) {
      await database.prepare(INSERT_MISSING_STATUS_SQL).bind(subject, JSON.stringify(row)).run();
    }
  }
  if (patch.invoicesBySubject) {
    for (const [subject, invoices] of Object.entries(patch.invoicesBySubject)) {
      for (const invoice of invoices) {
        await database
          .prepare(INSERT_MISSING_INVOICE_SQL)
          .bind(subject, invoice.invoiceId, JSON.stringify(invoice))
          .run();
      }
    }
  }
  if (patch.referralsBySubject) {
    for (const [subject, referral] of Object.entries(patch.referralsBySubject)) {
      await database.prepare(INSERT_MISSING_REFERRAL_SQL).bind(subject, JSON.stringify(referral)).run();
    }
  }
  if (patch.snapshotsBySubject) {
    for (const [subject, snapshot] of Object.entries(patch.snapshotsBySubject)) {
      await database.prepare(INSERT_MISSING_SNAPSHOT_SQL).bind(subject, JSON.stringify(snapshot)).run();
    }
  }
  for (const row of patch.adminAccounts ?? []) {
    await database.prepare(INSERT_MISSING_ADMIN_ACCOUNT_SQL).bind(row.parentAccountRef, JSON.stringify(row)).run();
  }
  for (const row of patch.adminInvoices ?? []) {
    await database.prepare(INSERT_MISSING_ADMIN_INVOICE_SQL).bind(row.invoiceId, JSON.stringify(row)).run();
  }
  for (const row of patch.adminDisputes ?? []) {
    await database.prepare(INSERT_MISSING_ADMIN_DISPUTE_SQL).bind(row.disputeId, JSON.stringify(row)).run();
  }
  for (const row of patch.adminReferrals ?? []) {
    await database.prepare(INSERT_MISSING_ADMIN_REFERRAL_SQL).bind(row.referralCode, JSON.stringify(row)).run();
  }
}

async function ensureReadModelSeedOnce(env: Env): Promise<void> {
  if (env.BILLING_D1) {
    await env.BILLING_D1.exec(CREATE_READ_MODEL_SCHEMA_SQL);
  }
  if (!isLocalFixtureEnvironment(env.ENVIRONMENT)) {
    return;
  }
  const patch = buildDefaultBillingBindingSeed(env);

  if (env.BILLING_D1) {
    await seedD1Tables(env.BILLING_D1, patch);
  }

  if (env.BILLING_CONFIG_KV) {
    const existingPlans = await env.BILLING_CONFIG_KV.get(PRICING_PLANS_KEY);
    if (existingPlans === null && patch.pricingPlans) {
      await env.BILLING_CONFIG_KV.put(PRICING_PLANS_KEY, JSON.stringify(patch.pricingPlans));
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
  const plans = await env.BILLING_CONFIG_KV?.get(PRICING_PLANS_KEY, 'json');
  await recordBindingRead(env, 'pricing-public', null);
  return Array.isArray(plans) ? (plans as ReadonlyArray<PricingPlanSummary>) : LOCAL_PRICING_PLANS;
}

export async function loadLocalSeedSummary(env: Env): Promise<{
  generatedAt: string;
  environment: string;
  authAdapterMode: string;
  pricingPlanCount: number;
  adminAccountCount: number;
  referralFixtureCount: number;
  manualReviewAccountCount: number;
  persistence: {
    d1StatusRows: number;
    d1AdminAccountRows: number;
    d1ReferralRows: number;
    kvPricingPlanRows: number;
    r2AuditEventRows: number;
  };
  fixtureValidation: { statusFixturesValid: boolean };
}> {
  const pricingPlans = await loadPricingPlans(env);
  const adminAccounts = await loadAdminBillingAccounts(env, null);
  const referrals = await loadAdminBillingReferrals(env, null);
  const [statusRowCount, storedAdminAccounts, storedReferrals, storedPricingPlans, storedAuditEvents] =
    await Promise.all([
      d1First<RowCountRow>(env.BILLING_D1, SELECT_STATUS_ROW_COUNT_SQL),
      d1All<PayloadJsonRow>(env.BILLING_D1, SELECT_ADMIN_ACCOUNTS_SQL),
      d1All<PayloadJsonRow>(env.BILLING_D1, SELECT_ADMIN_REFERRALS_SQL),
      env.BILLING_CONFIG_KV?.get(PRICING_PLANS_KEY, 'json'),
      readStoredAuditEvents(env),
    ]);
  const expectedSubjects = Object.keys(buildDefaultBillingBindingSeed(env).statusBySubject ?? {});
  const statusFixturesValid = (await Promise.all(
    expectedSubjects.map(async (subject) => {
      const row = await d1First<PayloadJsonRow>(env.BILLING_D1, SELECT_STATUS_BY_SUBJECT_SQL, subject);
      try {
        const payload = JSON.parse(row?.payload_json ?? '{}') as Record<string, unknown>;
        return payload.subject === subject && typeof payload.parentAccountRef === 'string';
      } catch {
        return false;
      }
    })
  )).every(Boolean);
  return {
    generatedAt: GENERATED_AT,
    environment: env.ENVIRONMENT,
    authAdapterMode: env.AUTH_ADAPTER_MODE?.trim() || 'local-safe-fixture',
    pricingPlanCount: pricingPlans.length,
    adminAccountCount: adminAccounts.length,
    referralFixtureCount: referrals.length,
    manualReviewAccountCount: adminAccounts.filter((account) => account.manualRequired).length,
    persistence: {
      d1StatusRows: Number(statusRowCount?.row_count ?? 0),
      d1AdminAccountRows: storedAdminAccounts.length,
      d1ReferralRows: storedReferrals.length,
      kvPricingPlanRows: Array.isArray(storedPricingPlans) ? storedPricingPlans.length : 0,
      r2AuditEventRows: storedAuditEvents.length,
    },
    fixtureValidation: { statusFixturesValid },
  };
}

export async function loadBillingStatusSummary(env: Env, subject: string): Promise<BillingStatusSummary> {
  await ensureReadModelSeed(env);
  const stored = parsePayloadRow<BillingStatusSummary>(
    await d1First<PayloadJsonRow>(env.BILLING_D1, SELECT_STATUS_BY_SUBJECT_SQL, subject)
  );
  await recordBindingRead(env, 'billing-status', subject);
  return stored ?? buildBillingStatusSummary(subject, env);
}

export async function loadBillingInvoices(env: Env, subject: string): Promise<ReadonlyArray<BillingInvoiceSummary>> {
  await ensureReadModelSeed(env);
  const stored = parsePayloadRows<BillingInvoiceSummary>(
    await d1All<PayloadJsonRow>(env.BILLING_D1, SELECT_INVOICES_BY_SUBJECT_SQL, subject)
  );
  await recordBindingRead(env, 'billing-invoices', subject);
  return stored.length > 0 ? stored : buildBillingInvoices(subject);
}

export async function findBillingInvoiceSubject(env: Env, invoiceId: string): Promise<string | null> {
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
  const stored = parsePayloadRow<BillingReferralSummary>(
    await d1First<PayloadJsonRow>(env.BILLING_D1, SELECT_REFERRAL_BY_SUBJECT_SQL, subject)
  );
  await recordBindingRead(env, 'billing-referrals', subject);
  return stored ?? buildBillingReferralSummary(subject);
}

export async function loadBillingEntitlementSnapshot(
  env: Env,
  subject: string
): Promise<BillingEntitlementSnapshotSummary> {
  await ensureReadModelSeed(env);
  const stored = parsePayloadRow<BillingEntitlementSnapshotSummary>(
    await d1First<PayloadJsonRow>(env.BILLING_D1, SELECT_SNAPSHOT_BY_SUBJECT_SQL, subject)
  );
  await recordBindingRead(env, 'billing-entitlement-snapshot', subject);
  return stored ?? buildEntitlementSnapshot(subject);
}

export async function loadBillingLicenseDecision(
  env: Env,
  subject: string,
  requestId: string,
  deviceId: string,
  requestedNewDevice: boolean
): Promise<BillingLicenseDecisionSummary> {
  const snapshot = await loadBillingEntitlementSnapshot(env, subject);
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

  if (snapshot.subscriptionStatus === 'grace' && requestedNewDevice) {
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

  return buildLicenseDecision(subject, requestId, deviceId, requestedNewDevice);
}

export async function loadAdminBillingAccounts(
  env: Env,
  query: string | null
): Promise<ReadonlyArray<AdminBillingAccountSummary>> {
  await ensureReadModelSeed(env);
  const loweredQuery = query?.trim().toLowerCase() ?? '';
  const stored = parsePayloadRows<AdminBillingAccountSummary>(
    await d1All<PayloadJsonRow>(env.BILLING_D1, SELECT_ADMIN_ACCOUNTS_SQL)
  );
  await recordBindingRead(env, 'admin-billing-accounts', null);
  const rows = stored.length > 0 ? stored : listAdminBillingAccounts(query);
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
  const loweredQuery = query?.trim().toLowerCase() ?? '';
  const stored = parsePayloadRows<AdminBillingInvoiceSummary>(
    await d1All<PayloadJsonRow>(env.BILLING_D1, SELECT_ADMIN_INVOICES_SQL)
  );
  await recordBindingRead(env, 'admin-billing-invoices', null);
  const rows = stored.length > 0 ? stored : listAdminBillingInvoices(query);
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
  const loweredQuery = query?.trim().toLowerCase() ?? '';
  const stored = parsePayloadRows<AdminBillingDisputeSummary>(
    await d1All<PayloadJsonRow>(env.BILLING_D1, SELECT_ADMIN_DISPUTES_SQL)
  );
  await recordBindingRead(env, 'admin-billing-disputes', null);
  const rows = stored.length > 0 ? stored : listAdminBillingDisputes(query);
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
  const loweredQuery = query?.trim().toLowerCase() ?? '';
  const stored = parsePayloadRows<AdminBillingReferralSummary>(
    await d1All<PayloadJsonRow>(env.BILLING_D1, SELECT_ADMIN_REFERRALS_SQL)
  );
  await recordBindingRead(env, 'admin-billing-referrals', null);
  const rows = stored.length > 0 ? stored : listAdminBillingReferrals(query);
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
  const loweredQuery = query?.trim().toLowerCase() ?? '';
  const object = await env.BILLING_AUDIT_R2?.get(AUDIT_EVENTS_KEY);
  const stored = object ? ((await object.json<ReadonlyArray<BillingAuditEventSummary>>()) ?? []) : [];
  await recordBindingRead(env, 'admin-billing-audit', null);
  const rows = stored.length > 0 ? stored : listBillingAuditEvents(query);
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
      const activeReferralCredits = status.seatComposition.activeReferralCredits;
      const baseIncludedSeats = status.seatComposition.baseIncludedSeats;
      const paidExtraSeats = Math.max(targetPlan.deviceLimit - baseIncludedSeats - activeReferralCredits, 0);
      const availableDeviceSlots = Math.max(targetPlan.deviceLimit - status.deviceUsage.activeDevices, 0);
      const nextStatus = {
        ...status,
        plan: cloneJsonValue(targetPlan),
        deviceUsage: {
          ...status.deviceUsage,
          limit: targetPlan.deviceLimit,
        },
        seatComposition: {
          ...status.seatComposition,
          paidExtraSeats,
          effectiveLimit: baseIncludedSeats + activeReferralCredits + paidExtraSeats,
          availableDeviceSlots,
        },
        warnings: withAddedWarning(status.warnings, 'plan-change-pending-provider-sync'),
        auditReference: mutation.auditReference,
        updatedAt,
      } as unknown as BillingStatusSummary;
      const nextSnapshot = {
        ...snapshot,
        planId: targetPlan.planId,
        deviceLimit: targetPlan.deviceLimit,
        availableDeviceSlots: Math.max(targetPlan.deviceLimit - snapshot.activeDevices, 0),
        signedAt: updatedAt,
        auditReference: `${mutation.auditReference}:snapshot`,
      } as unknown as BillingEntitlementSnapshotSummary;

      await upsertBillingStatusSummary(env, nextStatus);
      await upsertBillingSnapshotSummary(env, nextSnapshot);
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
        signedAt: updatedAt,
        auditReference: `${mutation.auditReference}:snapshot`,
      } as unknown as BillingEntitlementSnapshotSummary;

      await upsertBillingStatusSummary(env, nextStatus);
      await upsertBillingSnapshotSummary(env, nextSnapshot);
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
        pendingInvites: referral.pendingInvites + 1,
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
            invitedFamilies: existingAdminReferral.invitedFamilies + 1,
            auditReference: `${mutation.auditReference}:admin`,
            updatedAt,
          }
        : {
            referralCode: mutation.referralCode,
            ownerSubject: mutation.subject,
            creditedFamilies: nextReferral.availableCredits,
            invitedFamilies: nextReferral.activeReferredParents + nextReferral.pendingInvites,
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
      await upsertBillingStatusSummary(env, nextStatus);
      await upsertBillingSnapshotSummary(env, nextSnapshot);
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

        await upsertBillingStatusSummary(env, nextStatus);
        await upsertBillingSnapshotSummary(env, nextSnapshot);
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

        await upsertBillingStatusSummary(env, nextStatus);
        await upsertBillingSnapshotSummary(env, nextSnapshot);
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

        await upsertBillingStatusSummary(env, nextStatus);
        await upsertBillingSnapshotSummary(env, nextSnapshot);
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

        await upsertBillingStatusSummary(env, nextStatus);
        await upsertBillingSnapshotSummary(env, nextSnapshot);
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
