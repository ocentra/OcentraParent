import type { D1Database } from '@cloudflare/workers-types';
import { Logger } from '@ocentra-parent/logging-domain/core/logger';
import { getStackTrace } from '@ocentra-parent/logging-domain/core/stackTrace';
import {
  ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_KEY_ID_PREFIX,
  ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE,
  ACCOUNT_ISSUER_MAX_FIELD_BYTES,
  deriveAccountIdentityAuthorityProducerV2ServiceBindingId,
  isAccountIdentityAuthorityProducerV2Digest,
  isAccountIdentityAuthorityProducerV2KeyId,
  isAccountIdentityAuthorityProducerV2ReceiptId,
  isAccountIdentityAuthorityProducerV2ServiceBindingId,
  isAccountIdentityAuthorityProducerV2Text,
} from '../auth/account-identity-authority-producer-v2-contract.js';
import { deriveAccountIdentityAuthorityProducerV2KeyId } from '../auth/account-identity-authority-issuer-v2.js';

const log = Logger.instance;
log.register(import.meta.url);

const SCHEMA_NAME = 'account_identity_issuer_v2';
const SCHEMA_VERSION = 9;
const MAX_SAFE_GENERATION = Number.MAX_SAFE_INTEGER;
const MISSING_SCHEMA_REASON = 'account-identity-issuer-v2-schema-missing' as const;
const UNAVAILABLE_REASON = 'account-identity-issuer-v2-unavailable' as const;
const MILLIS_UTC_PATTERN = /^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{3}Z$/u;

export interface AccountIdentityAuthorityIssuerV2Verifier {
  readonly service: typeof ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE;
  readonly serviceBindingId: string;
  readonly accountId: string;
  readonly householdId: string;
  readonly provider: 'authjs' | 'firebase';
  readonly providerSubject: string;
  readonly keyId: string;
  readonly keyGeneration: number;
  readonly enrollmentGeneration: number;
  readonly authorityGeneration: number;
  readonly sessionGeneration: number;
  readonly publicKey: Uint8Array;
  readonly status: 'active' | 'revoked';
}

export interface AccountIdentityAuthorityIssuerV2InboundReceipt {
  readonly receiptId: string;
  readonly operation: 'IssueCurrentAuthority' | 'AcknowledgeReceipt';
  readonly accountId: string;
  readonly householdId: string;
  readonly provider: 'authjs' | 'firebase';
  readonly providerSubject: string;
  readonly serviceBindingId: string;
  readonly correlationId: string;
  readonly idempotencyKey: string;
  readonly payloadDigest: string;
  readonly authorityPayloadDigest: string;
  readonly keyId: string;
  readonly keyGeneration: number;
  readonly enrollmentGeneration: number;
  readonly authorityGeneration: number;
  readonly sessionGeneration: number;
  readonly issuedAt: string;
  readonly expiresAt: string;
  readonly wireDigest: string;
}

export type AccountIdentityAuthorityIssuerV2CurrentVerifierResult =
  | { readonly status: 'current'; readonly verifier: AccountIdentityAuthorityIssuerV2Verifier }
  | { readonly status: 'not-found' }
  | { readonly status: 'manual-required'; readonly reason: typeof MISSING_SCHEMA_REASON | typeof UNAVAILABLE_REASON };

export type AccountIdentityAuthorityIssuerV2CasResult =
  | { readonly status: 'updated'; readonly verifier: AccountIdentityAuthorityIssuerV2Verifier }
  | { readonly status: 'conflict' }
  | { readonly status: 'manual-required'; readonly reason: typeof MISSING_SCHEMA_REASON | typeof UNAVAILABLE_REASON };

export type AccountIdentityAuthorityIssuerV2EnrollmentResult =
  | { readonly status: 'enrolled'; readonly verifier: AccountIdentityAuthorityIssuerV2Verifier }
  | { readonly status: 'conflict' }
  | { readonly status: 'manual-required'; readonly reason: typeof MISSING_SCHEMA_REASON | typeof UNAVAILABLE_REASON };

export type AccountIdentityAuthorityIssuerV2RevocationResult =
  | { readonly status: 'revoked' }
  | { readonly status: 'conflict' }
  | { readonly status: 'manual-required'; readonly reason: typeof MISSING_SCHEMA_REASON | typeof UNAVAILABLE_REASON };

export type AccountIdentityAuthorityIssuerV2ReceiptResult =
  | { readonly status: 'recorded' }
  | { readonly status: 'duplicate' }
  | { readonly status: 'currentness-mismatch' }
  | { readonly status: 'conflict' }
  | { readonly status: 'manual-required'; readonly reason: typeof MISSING_SCHEMA_REASON | typeof UNAVAILABLE_REASON };

export interface AccountIdentityAuthorityIssuerV2Store {
  readCurrentVerifier(serviceBindingId: string): Promise<AccountIdentityAuthorityIssuerV2CurrentVerifierResult>;
  compareAndSwapCurrentVerifier(
    expected: AccountIdentityAuthorityIssuerV2Verifier,
    replacement: AccountIdentityAuthorityIssuerV2Verifier
  ): Promise<AccountIdentityAuthorityIssuerV2CasResult>;
  enrollCurrentVerifier(
    verifier: AccountIdentityAuthorityIssuerV2Verifier
  ): Promise<AccountIdentityAuthorityIssuerV2EnrollmentResult>;
  revokeCurrentVerifier(
    expected: AccountIdentityAuthorityIssuerV2Verifier
  ): Promise<AccountIdentityAuthorityIssuerV2RevocationResult>;
  recordInboundReceipt(
    receipt: AccountIdentityAuthorityIssuerV2InboundReceipt,
    expected: AccountIdentityAuthorityIssuerV2Verifier
  ): Promise<AccountIdentityAuthorityIssuerV2ReceiptResult>;
}

interface CurrentVerifierRow {
  service: string;
  service_binding_id: string;
  account_id: string;
  household_id: string;
  provider: 'authjs' | 'firebase';
  provider_subject: string;
  key_id: string;
  key_generation: number;
  enrollment_generation: number;
  authority_generation: number;
  session_generation: number;
  public_key: unknown;
  status: 'active' | 'revoked';
}

interface InboundReceiptRow {
  receipt_id: string;
  operation: 'IssueCurrentAuthority' | 'AcknowledgeReceipt';
  account_id: string;
  household_id: string;
  provider: 'authjs' | 'firebase';
  provider_subject: string;
  service_binding_id: string;
  correlation_id: string;
  idempotency_key: string;
  payload_digest: string;
  authority_payload_digest: string;
  key_id: string;
  key_generation: number;
  enrollment_generation: number;
  authority_generation: number;
  session_generation: number;
  issued_at: string;
  expires_at: string;
  wire_digest: string;
}

const SELECT_CURRENT_VERIFIER_SQL = `
SELECT service, service_binding_id, account_id, household_id, provider, provider_subject,
       key_id, key_generation, enrollment_generation, authority_generation, session_generation, public_key, status
FROM ocentra_account_identity_issuer_v2_currentness
WHERE service_binding_id = ? AND service = ?
LIMIT 1
`;

const UPDATE_CURRENT_VERIFIER_SQL = `
UPDATE ocentra_account_identity_issuer_v2_currentness
SET account_id = ?, household_id = ?, provider = ?, provider_subject = ?,
    key_id = ?, key_generation = ?, enrollment_generation = ?, authority_generation = ?, session_generation = ?,
    public_key = ?, status = 'active', updated_at = ?
WHERE service_binding_id = ? AND service = ?
  AND account_id = ? AND household_id = ? AND provider = ? AND provider_subject = ?
  AND key_id = ? AND key_generation = ? AND enrollment_generation = ?
  AND authority_generation = ? AND session_generation = ? AND public_key = ?
  AND ? >= key_generation AND ? >= enrollment_generation
  AND ? >= authority_generation AND ? >= session_generation
  AND (? > key_generation OR ? > enrollment_generation OR ? > authority_generation OR ? > session_generation)
  AND status = 'active'
`;

const INSERT_CURRENT_VERIFIER_SQL = `
INSERT INTO ocentra_account_identity_issuer_v2_currentness (
  service_binding_id, account_id, household_id, provider, provider_subject, service,
  key_id, key_generation, enrollment_generation, authority_generation, session_generation,
  public_key, status, created_at, updated_at
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 'active', ?, ?)
`;

const REVOKE_CURRENT_VERIFIER_SQL = `
UPDATE ocentra_account_identity_issuer_v2_currentness
SET status = 'revoked', updated_at = ?
WHERE service_binding_id = ? AND service = ?
  AND account_id = ? AND household_id = ? AND provider = ? AND provider_subject = ?
  AND key_id = ? AND key_generation = ? AND enrollment_generation = ?
  AND authority_generation = ? AND session_generation = ? AND public_key = ?
  AND status = 'active'
`;

const GUARD_CURRENT_VERIFIER_SQL = `
UPDATE ocentra_account_identity_issuer_v2_currentness
SET updated_at = updated_at
WHERE service_binding_id = ? AND service = ?
  AND account_id = ? AND household_id = ? AND provider = ? AND provider_subject = ?
  AND key_id = ? AND key_generation = ? AND enrollment_generation = ?
  AND authority_generation = ? AND session_generation = ? AND public_key = ?
  AND status = 'active'
RETURNING service_binding_id
`;

const INSERT_INBOUND_RECEIPT_SQL = `
INSERT INTO ocentra_account_identity_issuer_v2_inbound_receipts (
  receipt_id, operation, account_id, household_id, provider, provider_subject,
  service, service_binding_id, correlation_id, idempotency_key, payload_digest,
  authority_payload_digest, key_id, key_generation, enrollment_generation,
  authority_generation, session_generation, issued_at, expires_at, wire_digest, receipt_state, recorded_at
) SELECT ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 'accepted', ?
FROM ocentra_account_identity_issuer_v2_currentness
WHERE service_binding_id = ? AND service = ?
  AND account_id = ? AND household_id = ? AND provider = ? AND provider_subject = ?
  AND key_id = ? AND key_generation = ? AND enrollment_generation = ?
  AND authority_generation = ? AND session_generation = ? AND public_key = ?
  AND status = 'active'
RETURNING receipt_id
`;

const SELECT_INBOUND_BY_IDEMPOTENCY_SQL = `
SELECT receipt_id, operation, account_id, household_id, provider, provider_subject,
       service_binding_id, correlation_id, idempotency_key, payload_digest,
       authority_payload_digest, key_id, key_generation, authority_generation,
       enrollment_generation, session_generation, issued_at, expires_at, wire_digest
FROM ocentra_account_identity_issuer_v2_inbound_receipts
WHERE service = ? AND service_binding_id = ? AND operation = ? AND idempotency_key = ?
LIMIT 1
`;

const SELECT_INBOUND_BY_RECEIPT_SQL = `
SELECT receipt_id, operation, account_id, household_id, provider, provider_subject,
       service_binding_id, correlation_id, idempotency_key, payload_digest,
       authority_payload_digest, key_id, key_generation, authority_generation,
       enrollment_generation, session_generation, issued_at, expires_at, wire_digest
FROM ocentra_account_identity_issuer_v2_inbound_receipts
WHERE service = ? AND service_binding_id = ? AND operation = ? AND receipt_id = ?
LIMIT 1
`;

export function createAccountIdentityAuthorityIssuerV2Store(
  database: D1Database | undefined
): AccountIdentityAuthorityIssuerV2Store {
  return {
    async readCurrentVerifier(serviceBindingId) {
      if (database === undefined) return manualRequired(MISSING_SCHEMA_REASON);
      if (
        !isValidText(serviceBindingId, ACCOUNT_ISSUER_MAX_FIELD_BYTES) ||
        !isAccountIdentityAuthorityProducerV2ServiceBindingId(serviceBindingId)
      ) {
        return { status: 'not-found' };
      }
      const schema = await ensureSchema(database);
      if (schema !== 'ready') return manualRequired(schema === 'missing' ? MISSING_SCHEMA_REASON : UNAVAILABLE_REASON);
      try {
        const row = await database
          .prepare(SELECT_CURRENT_VERIFIER_SQL)
          .bind(serviceBindingId, ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE)
          .first<CurrentVerifierRow>();
        if (row === null) return { status: 'not-found' };
        const verifier = parseVerifierRow(row);
        if (verifier === null || !(await isValidVerifier(verifier, true))) return manualRequired(UNAVAILABLE_REASON);
        if (verifier.status === 'revoked') return { status: 'not-found' };
        return { status: 'current', verifier };
      } catch (error) {
        return manualRequired(isMissingTableError(error) ? MISSING_SCHEMA_REASON : UNAVAILABLE_REASON);
      }
    },

    async compareAndSwapCurrentVerifier(expected, replacement) {
      if (database === undefined) return manualRequired(MISSING_SCHEMA_REASON);
      if (!(await isValidVerifier(expected)) || !(await isValidVerifier(replacement))) return { status: 'conflict' };
      if (
        expected.serviceBindingId !== replacement.serviceBindingId ||
        expected.accountId !== replacement.accountId ||
        expected.householdId !== replacement.householdId ||
        expected.provider !== replacement.provider ||
        expected.providerSubject !== replacement.providerSubject ||
        replacement.keyGeneration < expected.keyGeneration ||
        replacement.enrollmentGeneration < expected.enrollmentGeneration ||
        replacement.authorityGeneration < expected.authorityGeneration ||
        replacement.sessionGeneration < expected.sessionGeneration ||
        (replacement.keyGeneration === expected.keyGeneration &&
          replacement.enrollmentGeneration === expected.enrollmentGeneration &&
          replacement.authorityGeneration === expected.authorityGeneration &&
          replacement.sessionGeneration === expected.sessionGeneration)
      ) {
        return { status: 'conflict' };
      }
      const schema = await ensureSchema(database);
      if (schema !== 'ready') return manualRequired(schema === 'missing' ? MISSING_SCHEMA_REASON : UNAVAILABLE_REASON);
      try {
        const result = await database
          .prepare(UPDATE_CURRENT_VERIFIER_SQL)
          .bind(
            replacement.accountId,
            replacement.householdId,
            replacement.provider,
            replacement.providerSubject,
            replacement.keyId,
            replacement.keyGeneration,
            replacement.enrollmentGeneration,
            replacement.authorityGeneration,
            replacement.sessionGeneration,
            replacement.publicKey,
            new Date().toISOString(),
            expected.serviceBindingId,
            ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE,
            expected.accountId,
            expected.householdId,
            expected.provider,
            expected.providerSubject,
            expected.keyId,
            expected.keyGeneration,
            expected.enrollmentGeneration,
            expected.authorityGeneration,
            expected.sessionGeneration,
            expected.publicKey,
            replacement.keyGeneration,
            replacement.enrollmentGeneration,
            replacement.authorityGeneration,
            replacement.sessionGeneration,
            replacement.keyGeneration,
            replacement.enrollmentGeneration,
            replacement.authorityGeneration,
            replacement.sessionGeneration
          )
          .run();
        if (resultChanges(result) !== 1) return { status: 'conflict' };
        return { status: 'updated', verifier: replacement };
      } catch (error) {
        if (isUniqueConstraintError(error)) return { status: 'conflict' };
        return manualRequired(isMissingTableError(error) ? MISSING_SCHEMA_REASON : UNAVAILABLE_REASON);
      }
    },

    async enrollCurrentVerifier(verifier) {
      if (database === undefined) return manualRequired(MISSING_SCHEMA_REASON);
      if (!(await isValidVerifier(verifier))) return { status: 'conflict' };
      const schema = await ensureSchema(database);
      if (schema !== 'ready') return manualRequired(schema === 'missing' ? MISSING_SCHEMA_REASON : UNAVAILABLE_REASON);
      const now = new Date().toISOString();
      try {
        const result = await database
          .prepare(INSERT_CURRENT_VERIFIER_SQL)
          .bind(
            verifier.serviceBindingId,
            verifier.accountId,
            verifier.householdId,
            verifier.provider,
            verifier.providerSubject,
            ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE,
            verifier.keyId,
            verifier.keyGeneration,
            verifier.enrollmentGeneration,
            verifier.authorityGeneration,
            verifier.sessionGeneration,
            verifier.publicKey,
            now,
            now
          )
          .run();
        return resultChanges(result) === 1 ? { status: 'enrolled', verifier } : { status: 'conflict' };
      } catch (error) {
        if (isUniqueConstraintError(error)) return { status: 'conflict' };
        return manualRequired(isMissingTableError(error) ? MISSING_SCHEMA_REASON : UNAVAILABLE_REASON);
      }
    },

    async revokeCurrentVerifier(expected) {
      if (database === undefined) return manualRequired(MISSING_SCHEMA_REASON);
      if (!(await isValidVerifier(expected))) return { status: 'conflict' };
      const schema = await ensureSchema(database);
      if (schema !== 'ready') return manualRequired(schema === 'missing' ? MISSING_SCHEMA_REASON : UNAVAILABLE_REASON);
      try {
        const result = await database
          .prepare(REVOKE_CURRENT_VERIFIER_SQL)
          .bind(
            new Date().toISOString(),
            expected.serviceBindingId,
            ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE,
            expected.accountId,
            expected.householdId,
            expected.provider,
            expected.providerSubject,
            expected.keyId,
            expected.keyGeneration,
            expected.enrollmentGeneration,
            expected.authorityGeneration,
            expected.sessionGeneration,
            expected.publicKey
          )
          .run();
        return resultChanges(result) === 1 ? { status: 'revoked' } : { status: 'conflict' };
      } catch (error) {
        return manualRequired(isMissingTableError(error) ? MISSING_SCHEMA_REASON : UNAVAILABLE_REASON);
      }
    },

    async recordInboundReceipt(receipt, expected) {
      if (database === undefined) return manualRequired(MISSING_SCHEMA_REASON);
      if (!(await isValidVerifier(expected)) || !isValidInboundReceipt(receipt)) return { status: 'conflict' };
      if (!matchesReceiptVerifier(receipt, expected)) return { status: 'currentness-mismatch' };
      const schema = await ensureSchema(database);
      if (schema !== 'ready') return manualRequired(schema === 'missing' ? MISSING_SCHEMA_REASON : UNAVAILABLE_REASON);
      const recordedAt = new Date().toISOString();
      try {
        const results = await database.batch([
          database.prepare(GUARD_CURRENT_VERIFIER_SQL).bind(...verifierBindingValues(expected)),
          database.prepare(INSERT_INBOUND_RECEIPT_SQL).bind(...inboundInsertValues(receipt, expected, recordedAt)),
        ]);
        if (resultChanges(results[0]) !== 1 || resultChanges(results[1]) !== 1) {
          return { status: 'currentness-mismatch' };
        }
        return { status: 'recorded' };
      } catch (error) {
        if (isMissingTableError(error)) return manualRequired(MISSING_SCHEMA_REASON);
        if (!isUniqueConstraintError(error)) return manualRequired(UNAVAILABLE_REASON);
        try {
          const existing = await readExistingInboundWithCurrentness(database, receipt, expected);
          if (existing.status === 'currentness-mismatch') return existing;
          if (existing.row === null) return { status: 'conflict' };
          return sameInboundReceipt(existing.row, receipt) ? { status: 'duplicate' } : { status: 'conflict' };
        } catch (readError) {
          return manualRequired(isMissingTableError(readError) ? MISSING_SCHEMA_REASON : UNAVAILABLE_REASON);
        }
      }
    },
  };
}

function parseVerifierRow(row: CurrentVerifierRow): AccountIdentityAuthorityIssuerV2Verifier | null {
  const publicKey = readPublicKey(row.public_key);
  if (
    row.service !== ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE ||
    !isValidText(row.service_binding_id, ACCOUNT_ISSUER_MAX_FIELD_BYTES) ||
    !isAccountIdentityAuthorityProducerV2ServiceBindingId(row.service_binding_id) ||
    !isValidText(row.account_id, ACCOUNT_ISSUER_MAX_FIELD_BYTES) ||
    !isValidText(row.household_id, ACCOUNT_ISSUER_MAX_FIELD_BYTES) ||
    (row.provider !== 'authjs' && row.provider !== 'firebase') ||
    !isValidText(row.provider_subject, ACCOUNT_ISSUER_MAX_FIELD_BYTES) ||
    !isAccountIdentityAuthorityProducerV2KeyId(row.key_id) ||
    !isValidGeneration(row.key_generation) ||
    !isValidGeneration(row.enrollment_generation) ||
    !isValidGeneration(row.authority_generation) ||
    !isValidGeneration(row.session_generation) ||
    publicKey === null ||
    (row.status !== 'active' && row.status !== 'revoked')
  ) {
    return null;
  }
  return {
    service: ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE,
    serviceBindingId: row.service_binding_id,
    accountId: row.account_id,
    householdId: row.household_id,
    provider: row.provider,
    providerSubject: row.provider_subject,
    keyId: row.key_id,
    keyGeneration: row.key_generation,
    enrollmentGeneration: row.enrollment_generation,
    authorityGeneration: row.authority_generation,
    sessionGeneration: row.session_generation,
    publicKey,
    status: row.status,
  };
}

async function isValidVerifier(
  value: AccountIdentityAuthorityIssuerV2Verifier,
  allowRevoked = false
): Promise<boolean> {
  if (typeof value !== 'object' || value === null || !(value.publicKey instanceof Uint8Array)) return false;
  if (
    !(
      value.service === ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE &&
      isValidText(value.serviceBindingId, ACCOUNT_ISSUER_MAX_FIELD_BYTES) &&
      isAccountIdentityAuthorityProducerV2ServiceBindingId(value.serviceBindingId) &&
      isValidText(value.accountId, ACCOUNT_ISSUER_MAX_FIELD_BYTES) &&
      isValidText(value.householdId, ACCOUNT_ISSUER_MAX_FIELD_BYTES) &&
      (value.provider === 'authjs' || value.provider === 'firebase') &&
      isValidText(value.providerSubject, ACCOUNT_ISSUER_MAX_FIELD_BYTES) &&
      isAccountIdentityAuthorityProducerV2KeyId(value.keyId) &&
      value.keyId.startsWith(ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_KEY_ID_PREFIX) &&
      isValidGeneration(value.keyGeneration) &&
      isValidGeneration(value.enrollmentGeneration) &&
      isValidGeneration(value.authorityGeneration) &&
      isValidGeneration(value.sessionGeneration) &&
      value.publicKey.byteLength === 65 &&
      value.publicKey[0] === 0x04 &&
      value.publicKey.slice(1).some((byte) => byte !== 0)
    )
  ) {
    return false;
  }
  if (value.status !== 'active' && (!allowRevoked || value.status !== 'revoked')) return false;
  try {
    const [expectedKeyId, expectedServiceBindingId] = await Promise.all([
      deriveAccountIdentityAuthorityProducerV2KeyId(value.publicKey),
      deriveAccountIdentityAuthorityProducerV2ServiceBindingId(value.service, value.accountId, value.householdId),
    ]);
    return expectedKeyId === value.keyId && expectedServiceBindingId === value.serviceBindingId;
  } catch {
    return false;
  }
}

function isValidInboundReceipt(value: AccountIdentityAuthorityIssuerV2InboundReceipt): boolean {
  const issuedAt = parseTimestamp(value.issuedAt);
  const expiresAt = parseTimestamp(value.expiresAt);
  return (
    isAccountIdentityAuthorityProducerV2ReceiptId(value.receiptId) &&
    (value.operation === 'IssueCurrentAuthority' || value.operation === 'AcknowledgeReceipt') &&
    isValidText(value.accountId, ACCOUNT_ISSUER_MAX_FIELD_BYTES) &&
    isValidText(value.householdId, ACCOUNT_ISSUER_MAX_FIELD_BYTES) &&
    (value.provider === 'authjs' || value.provider === 'firebase') &&
    isValidText(value.providerSubject, ACCOUNT_ISSUER_MAX_FIELD_BYTES) &&
    isValidText(value.serviceBindingId, ACCOUNT_ISSUER_MAX_FIELD_BYTES) &&
    isAccountIdentityAuthorityProducerV2ServiceBindingId(value.serviceBindingId) &&
    isValidText(value.correlationId, ACCOUNT_ISSUER_MAX_FIELD_BYTES) &&
    isValidText(value.idempotencyKey, ACCOUNT_ISSUER_MAX_FIELD_BYTES) &&
    isAccountIdentityAuthorityProducerV2Digest(value.payloadDigest) &&
    isAccountIdentityAuthorityProducerV2Digest(value.authorityPayloadDigest) &&
    isAccountIdentityAuthorityProducerV2KeyId(value.keyId) &&
    isValidGeneration(value.keyGeneration) &&
    isValidGeneration(value.enrollmentGeneration) &&
    isValidGeneration(value.authorityGeneration) &&
    isValidGeneration(value.sessionGeneration) &&
    issuedAt !== null &&
    expiresAt !== null &&
    expiresAt > issuedAt &&
    isAccountIdentityAuthorityProducerV2Digest(value.wireDigest)
  );
}

function matchesReceiptVerifier(
  receipt: AccountIdentityAuthorityIssuerV2InboundReceipt,
  verifier: AccountIdentityAuthorityIssuerV2Verifier
): boolean {
  return (
    receipt.serviceBindingId === verifier.serviceBindingId &&
    receipt.accountId === verifier.accountId &&
    receipt.householdId === verifier.householdId &&
    receipt.provider === verifier.provider &&
    receipt.providerSubject === verifier.providerSubject &&
    receipt.keyId === verifier.keyId &&
    receipt.keyGeneration === verifier.keyGeneration &&
    receipt.enrollmentGeneration === verifier.enrollmentGeneration &&
    receipt.authorityGeneration === verifier.authorityGeneration &&
    receipt.sessionGeneration === verifier.sessionGeneration
  );
}

function verifierBindingValues(verifier: AccountIdentityAuthorityIssuerV2Verifier): ReadonlyArray<unknown> {
  return [
    verifier.serviceBindingId,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE,
    verifier.accountId,
    verifier.householdId,
    verifier.provider,
    verifier.providerSubject,
    verifier.keyId,
    verifier.keyGeneration,
    verifier.enrollmentGeneration,
    verifier.authorityGeneration,
    verifier.sessionGeneration,
    verifier.publicKey,
  ];
}

function inboundInsertValues(
  receipt: AccountIdentityAuthorityIssuerV2InboundReceipt,
  verifier: AccountIdentityAuthorityIssuerV2Verifier,
  recordedAt: string
): ReadonlyArray<unknown> {
  return [
    receipt.receiptId,
    receipt.operation,
    receipt.accountId,
    receipt.householdId,
    receipt.provider,
    receipt.providerSubject,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE,
    receipt.serviceBindingId,
    receipt.correlationId,
    receipt.idempotencyKey,
    receipt.payloadDigest,
    receipt.authorityPayloadDigest,
    receipt.keyId,
    receipt.keyGeneration,
    receipt.enrollmentGeneration,
    receipt.authorityGeneration,
    receipt.sessionGeneration,
    receipt.issuedAt,
    receipt.expiresAt,
    receipt.wireDigest,
    recordedAt,
    ...verifierBindingValues(verifier),
  ];
}

function resultChanges(value: unknown): number {
  if (typeof value !== 'object' || value === null) return 0;
  if ('results' in value) {
    const results = (value as { results?: unknown }).results;
    if (Array.isArray(results)) return results.length;
  }
  if ('meta' in value) {
    const meta = (value as { meta?: unknown }).meta;
    if (typeof meta === 'object' && meta !== null && 'changes' in meta) {
      const changes = (meta as { changes?: unknown }).changes;
      return typeof changes === 'number' ? changes : 0;
    }
  }
  return 0;
}

async function readExistingInboundWithCurrentness(
  database: D1Database,
  receipt: AccountIdentityAuthorityIssuerV2InboundReceipt,
  expected: AccountIdentityAuthorityIssuerV2Verifier
): Promise<
  { readonly status: 'current'; readonly row: InboundReceiptRow | null } | { readonly status: 'currentness-mismatch' }
> {
  const results = await database.batch([
    database.prepare(GUARD_CURRENT_VERIFIER_SQL).bind(...verifierBindingValues(expected)),
    database
      .prepare(SELECT_INBOUND_BY_IDEMPOTENCY_SQL)
      .bind(
        ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE,
        receipt.serviceBindingId,
        receipt.operation,
        receipt.idempotencyKey
      ),
    database
      .prepare(SELECT_INBOUND_BY_RECEIPT_SQL)
      .bind(
        ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE,
        receipt.serviceBindingId,
        receipt.operation,
        receipt.receiptId
      ),
  ]);
  if (resultChanges(results[0]) !== 1) return { status: 'currentness-mismatch' };
  return {
    status: 'current',
    row: firstBatchRow(results[1]) ?? firstBatchRow(results[2]),
  };
}

function firstBatchRow(value: unknown): InboundReceiptRow | null {
  if (typeof value !== 'object' || value === null || !('results' in value)) return null;
  const results = (value as { results?: unknown }).results;
  if (!Array.isArray(results) || results.length === 0 || typeof results[0] !== 'object' || results[0] === null) {
    return null;
  }
  return results[0] as InboundReceiptRow;
}

function sameInboundReceipt(row: InboundReceiptRow, value: AccountIdentityAuthorityIssuerV2InboundReceipt): boolean {
  return (
    row.receipt_id === value.receiptId &&
    row.operation === value.operation &&
    row.account_id === value.accountId &&
    row.household_id === value.householdId &&
    row.provider === value.provider &&
    row.provider_subject === value.providerSubject &&
    row.service_binding_id === value.serviceBindingId &&
    row.correlation_id === value.correlationId &&
    row.idempotency_key === value.idempotencyKey &&
    row.payload_digest === value.payloadDigest &&
    row.authority_payload_digest === value.authorityPayloadDigest &&
    row.key_id === value.keyId &&
    row.key_generation === value.keyGeneration &&
    row.enrollment_generation === value.enrollmentGeneration &&
    row.authority_generation === value.authorityGeneration &&
    row.session_generation === value.sessionGeneration &&
    row.issued_at === value.issuedAt &&
    row.expires_at === value.expiresAt &&
    row.wire_digest === value.wireDigest
  );
}

async function ensureSchema(database: D1Database): Promise<'ready' | 'missing' | 'unavailable'> {
  try {
    const row = await database
      .prepare('SELECT schema_version FROM ocentra_account_identity_issuer_v2_schema WHERE schema_name = ? LIMIT 1')
      .bind(SCHEMA_NAME)
      .first<{ schema_version: number }>();
    return row?.schema_version === SCHEMA_VERSION ? 'ready' : 'missing';
  } catch (error) {
    log.logWarn(
      'account issuer v2 D1 schema unavailable',
      getStackTrace(),
      {
        owner: 'cloudflare-wp05-account-identity-authority-issuer-v2',
        boundary: 'account-issuer-v2-d1-schema',
        result: 'blocked',
        noClaimReason: isMissingTableError(error) ? MISSING_SCHEMA_REASON : UNAVAILABLE_REASON,
        redactionState: 'redacted',
      },
      false
    );
    return isMissingTableError(error) ? 'missing' : 'unavailable';
  }
}

function readPublicKey(value: unknown): Uint8Array | null {
  try {
    if (value instanceof Uint8Array) {
      const copy = new Uint8Array(value.byteLength);
      copy.set(value);
      return copy.byteLength === 65 && copy[0] === 0x04 && copy.slice(1).some((byte) => byte !== 0) ? copy : null;
    }
    if (value instanceof ArrayBuffer) {
      const copy = new Uint8Array(value.slice(0));
      return copy.byteLength === 65 && copy[0] === 0x04 && copy.slice(1).some((byte) => byte !== 0) ? copy : null;
    }
  } catch {
    return null;
  }
  return null;
}

function isValidText(value: string, maxBytes: number): boolean {
  return typeof value === 'string' && value.length <= maxBytes && isAccountIdentityAuthorityProducerV2Text(value);
}

function isValidGeneration(value: unknown): value is number {
  return typeof value === 'number' && Number.isSafeInteger(value) && value > 0 && value <= MAX_SAFE_GENERATION;
}

function parseTimestamp(value: string): number | null {
  if (typeof value !== 'string' || !MILLIS_UTC_PATTERN.test(value)) return null;
  const timestamp = Date.parse(value);
  return Number.isSafeInteger(timestamp) && timestamp >= 0 && new Date(timestamp).toISOString() === value
    ? timestamp
    : null;
}

function isMissingTableError(error: unknown): boolean {
  const message = (error instanceof Error ? error.message : String(error)).toLowerCase();
  return message.includes('no such table') && message.includes('account_identity_issuer_v2');
}

function isUniqueConstraintError(error: unknown): boolean {
  const message = (error instanceof Error ? error.message : String(error)).toLowerCase();
  return message.includes('unique constraint') || message.includes('primary key');
}

function manualRequired(reason: typeof MISSING_SCHEMA_REASON | typeof UNAVAILABLE_REASON): {
  readonly status: 'manual-required';
  readonly reason: typeof MISSING_SCHEMA_REASON | typeof UNAVAILABLE_REASON;
} {
  return { status: 'manual-required', reason };
}
