import type { D1Database } from '@cloudflare/workers-types';

/** External identity providers supported by the narrow account mapping boundary. */
export type AccountIdentityProvider = 'authjs' | 'firebase';
/** Lifecycle state for a provider-subject mapping. */
export type AccountIdentityStatus = 'active' | 'revoked';

/** Minimal account identity data retained by Cloudflare custody. */
export interface AccountIdentityRecord {
  accountId: string;
  provider: AccountIdentityProvider;
  providerSubject: string;
  status: AccountIdentityStatus;
  createdAt: number;
  updatedAt: number;
}

/** Verified input required to persist a provider-subject mapping. */
export interface AccountIdentityUpsertInput {
  accountId: string;
  provider: AccountIdentityProvider;
  providerSubject: string;
  status: AccountIdentityStatus;
  nowMs: number;
}

/** Validation failures returned before any database write. */
export type AccountIdentityInvalidReason =
  | 'account-id-invalid'
  | 'provider-subject-invalid'
  | 'provider-unsupported'
  | 'status-invalid'
  | 'timestamp-invalid';

/** Explicit outcomes for an account identity persistence attempt. */
export type AccountIdentityUpsertResult =
  | { status: 'persisted'; record: AccountIdentityRecord }
  | { status: 'manual-required'; reason: 'account-identity-d1-binding-missing' }
  | { status: 'conflict'; reason: 'provider-subject-already-linked' }
  | { status: 'invalid-input'; reason: AccountIdentityInvalidReason };

/** Explicit outcomes for an account identity lookup. */
export type AccountIdentityLookupResult =
  | { status: 'found'; record: AccountIdentityRecord }
  | { status: 'not-found' }
  | { status: 'manual-required'; reason: 'account-identity-d1-binding-missing' }
  | { status: 'invalid-input'; reason: 'provider-unsupported' | 'provider-subject-invalid' };

type AccountIdentityLookupInvalidReason = 'provider-unsupported' | 'provider-subject-invalid';

/** D1-backed operations for the provider-subject mapping boundary. */
export interface AccountIdentityStore {
  get(provider: AccountIdentityProvider, providerSubject: string): Promise<AccountIdentityLookupResult>;
  upsert(input: AccountIdentityUpsertInput): Promise<AccountIdentityUpsertResult>;
}

/** Schema owned by the account identity persistence adapter. */
export const ACCOUNT_IDENTITY_SCHEMA_SQL = `
CREATE TABLE IF NOT EXISTS ocentra_account_identities (
  account_id TEXT NOT NULL,
  provider TEXT NOT NULL CHECK (provider IN ('authjs', 'firebase')),
  provider_subject TEXT NOT NULL,
  status TEXT NOT NULL CHECK (status IN ('active', 'revoked')),
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL,
  PRIMARY KEY (provider, provider_subject)
);
`;

const ACCOUNT_IDENTITY_SELECT_SQL = `
SELECT account_id, provider, provider_subject, status, created_at, updated_at
FROM ocentra_account_identities
WHERE provider = ? AND provider_subject = ?
LIMIT 1
`;

const ACCOUNT_IDENTITY_INSERT_SQL = `
INSERT OR IGNORE INTO ocentra_account_identities
  (account_id, provider, provider_subject, status, created_at, updated_at)
VALUES (?, ?, ?, ?, ?, ?)
`;

const ACCOUNT_IDENTITY_UPDATE_SQL = `
UPDATE ocentra_account_identities
SET status = ?, updated_at = ?
WHERE provider = ? AND provider_subject = ?
`;

const MAX_ACCOUNT_ID_LENGTH = 128;
const MAX_PROVIDER_SUBJECT_LENGTH = 256;
const CONTROL_CHARACTER_PATTERN = /[\u0000-\u001f\u007f]/;

interface AccountIdentityRow {
  account_id: string;
  provider: AccountIdentityProvider;
  provider_subject: string;
  status: AccountIdentityStatus;
  created_at: number;
  updated_at: number;
}

function normaliseBoundedText(value: string, maxLength: number): string | null {
  const normalised = value.trim();
  if (normalised.length === 0 || normalised.length > maxLength || CONTROL_CHARACTER_PATTERN.test(normalised)) {
    return null;
  }
  return normalised;
}

function isProvider(value: string): value is AccountIdentityProvider {
  return value === 'authjs' || value === 'firebase';
}

function isStatus(value: string): value is AccountIdentityStatus {
  return value === 'active' || value === 'revoked';
}

function toRecord(row: AccountIdentityRow): AccountIdentityRecord {
  return {
    accountId: row.account_id,
    provider: row.provider,
    providerSubject: row.provider_subject,
    status: row.status,
    createdAt: row.created_at,
    updatedAt: row.updated_at,
  };
}

function validateUpsertInput(input: AccountIdentityUpsertInput): AccountIdentityInvalidReason | null {
  if (normaliseBoundedText(input.accountId, MAX_ACCOUNT_ID_LENGTH) === null) {
    return 'account-id-invalid';
  }
  if (!isProvider(input.provider)) {
    return 'provider-unsupported';
  }
  if (normaliseBoundedText(input.providerSubject, MAX_PROVIDER_SUBJECT_LENGTH) === null) {
    return 'provider-subject-invalid';
  }
  if (!isStatus(input.status)) {
    return 'status-invalid';
  }
  if (!Number.isSafeInteger(input.nowMs) || input.nowMs < 0) {
    return 'timestamp-invalid';
  }
  return null;
}

function validateLookupInput(provider: string, providerSubject: string): AccountIdentityLookupInvalidReason | null {
  if (!isProvider(provider)) {
    return 'provider-unsupported';
  }
  if (normaliseBoundedText(providerSubject, MAX_PROVIDER_SUBJECT_LENGTH) === null) {
    return 'provider-subject-invalid';
  }
  return null;
}

/** Ensure the narrow mapping table exists before an adapter operation. */
export async function initializeAccountIdentitySchema(database: D1Database): Promise<void> {
  await database.exec(ACCOUNT_IDENTITY_SCHEMA_SQL);
}

/** Create an adapter that fails closed when the optional D1 binding is absent. */
export function createAccountIdentityStore(database: D1Database | undefined): AccountIdentityStore {
  async function read(
    database: D1Database,
    provider: AccountIdentityProvider,
    providerSubject: string
  ): Promise<AccountIdentityRecord | null> {
    const row = await database
      .prepare(ACCOUNT_IDENTITY_SELECT_SQL)
      .bind(provider, providerSubject)
      .first<AccountIdentityRow>();
    return row === null ? null : toRecord(row);
  }

  return {
    async get(provider, providerSubject) {
      if (database === undefined) {
        return { status: 'manual-required', reason: 'account-identity-d1-binding-missing' };
      }
      const invalidReason = validateLookupInput(provider, providerSubject);
      if (invalidReason !== null) {
        return { status: 'invalid-input', reason: invalidReason };
      }
      const normalisedSubject = normaliseBoundedText(providerSubject, MAX_PROVIDER_SUBJECT_LENGTH)!;
      await initializeAccountIdentitySchema(database);
      const record = await read(database, provider, normalisedSubject);
      return record === null ? { status: 'not-found' } : { status: 'found', record };
    },

    async upsert(input) {
      if (database === undefined) {
        return { status: 'manual-required', reason: 'account-identity-d1-binding-missing' };
      }
      const invalidReason = validateUpsertInput(input);
      if (invalidReason !== null) {
        return { status: 'invalid-input', reason: invalidReason };
      }
      const accountId = normaliseBoundedText(input.accountId, MAX_ACCOUNT_ID_LENGTH)!;
      const providerSubject = normaliseBoundedText(input.providerSubject, MAX_PROVIDER_SUBJECT_LENGTH)!;
      await initializeAccountIdentitySchema(database);
      const existing = await read(database, input.provider, providerSubject);
      if (existing !== null && existing.accountId !== accountId) {
        return { status: 'conflict', reason: 'provider-subject-already-linked' };
      }
      if (existing !== null) {
        await database
          .prepare(ACCOUNT_IDENTITY_UPDATE_SQL)
          .bind(input.status, input.nowMs, input.provider, providerSubject)
          .run();
      } else {
        await database
          .prepare(ACCOUNT_IDENTITY_INSERT_SQL)
          .bind(accountId, input.provider, providerSubject, input.status, input.nowMs, input.nowMs)
          .run();
      }
      const persisted = await read(database, input.provider, providerSubject);
      if (persisted === null) {
        throw new Error('account identity row unavailable after persistence attempt');
      }
      if (persisted.accountId !== accountId) {
        return { status: 'conflict', reason: 'provider-subject-already-linked' };
      }
      return { status: 'persisted', record: persisted };
    },
  };
}
