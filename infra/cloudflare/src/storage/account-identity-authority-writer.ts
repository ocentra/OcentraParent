import type { D1Database } from '@cloudflare/workers-types';
import {
  AccountIdentityCurrentMemberDeviceAuthorityHandoffSchema,
  type AccountIdentityCurrentMemberDeviceAuthorityHandoff,
  type AccountIdentityProvider,
} from '@ocentra-parent/schema-domain/account-identity-authority';
import {
  createAccountIdentityAuthorityStore,
  type AccountIdentityAuthorityReadResult,
} from './account-identity-authority-store.js';

const MAX_SAFE_GENERATION = Number.MAX_SAFE_INTEGER;
const MAX_IDENTITY_TEXT_LENGTH = 256;
const CONTROL_CHARACTER_PATTERN = /[\u0000-\u001f\u007f]/;
const RFC3339_TIMESTAMP_PATTERN = /^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d{1,9})?(?:Z|[+-]\d{2}:\d{2})$/;
const SERVER_OWNED_AUTHORITY = Symbol('server-owned-account-authority');
const ACCOUNT_OWNED_PRODUCER = Symbol('account-owned-authority-producer');
const AUTHORITY_TABLE = 'ocentra_account_identity_current_authority';

/**
 * The Account plan supplies this producer from its durable, sealed authority
 * service. It receives only the provider identity already verified by the
 * external provider adapter; request fields are never passed to it.
 */
export interface AccountOwnedAuthorityProducer {
  readonly [ACCOUNT_OWNED_PRODUCER]: true;
  resolveCurrentAuthority(
    provider: AccountIdentityProvider,
    providerSubject: string
  ): Promise<AccountIdentityCurrentMemberDeviceAuthorityHandoff | null>;
}

/**
 * Bind the Account-owned producer to the Cloudflare storage seam. The private
 * brand prevents a request or serialized handoff from being passed as the
 * producer; only this explicit owner composition boundary can create it.
 */
export function createAccountOwnedAuthorityProducer(
  resolveCurrentAuthority: AccountOwnedAuthorityProducer['resolveCurrentAuthority']
): AccountOwnedAuthorityProducer {
  return Object.freeze({
    [ACCOUNT_OWNED_PRODUCER]: true as const,
    resolveCurrentAuthority,
  });
}

interface ServerOwnedAccountIdentityAuthority {
  readonly handoff: AccountIdentityCurrentMemberDeviceAuthorityHandoff;
  readonly [SERVER_OWNED_AUTHORITY]: true;
}

export type AccountIdentityAuthorityWriteResult =
  | { status: 'written'; authorityGeneration: number }
  | { status: 'conflict'; reason: 'authority-generation-conflict' | 'provider-subject-already-linked' }
  | {
      status: 'manual-required';
      reason:
        | 'account-identity-d1-authority-missing'
        | 'account-identity-d1-schema-missing'
        | 'account-identity-d1-unavailable'
        | 'account-identity-authority-source-unavailable';
    }
  | {
      status: 'rejected';
      reason:
        | 'authority-source-invalid'
        | 'authority-currentness-invalid'
        | 'authority-generation-invalid'
        | 'provider-subject-invalid';
    };

export type AccountIdentityAuthorityRevokeResult =
  | { status: 'revoked'; authorityGeneration: number }
  | { status: 'conflict'; reason: 'authority-generation-conflict' }
  | {
      status: 'manual-required';
      reason:
        | 'account-identity-d1-authority-missing'
        | 'account-identity-d1-schema-missing'
        | 'account-identity-d1-unavailable'
        | 'account-identity-authority-source-unavailable';
    }
  | { status: 'rejected'; reason: 'provider-subject-invalid' | 'authority-generation-invalid' };

export interface AccountIdentityAuthorityWriter {
  readCurrentAuthority(
    provider: AccountIdentityProvider,
    providerSubject: string
  ): Promise<AccountIdentityAuthorityReadResult>;
  createCurrentAuthority(
    producer: AccountOwnedAuthorityProducer,
    provider: AccountIdentityProvider,
    providerSubject: string
  ): Promise<AccountIdentityAuthorityWriteResult>;
  compareAndSwapCurrentAuthority(
    producer: AccountOwnedAuthorityProducer,
    provider: AccountIdentityProvider,
    providerSubject: string,
    expectedAuthorityGeneration: number,
    expectedSessionGeneration: number,
    expectedSessionId: string
  ): Promise<AccountIdentityAuthorityWriteResult>;
  revokeCurrentAuthority(
    provider: AccountIdentityProvider,
    providerSubject: string,
    expectedAuthorityGeneration: number,
    expectedSessionGeneration: number,
    expectedSessionId: string
  ): Promise<AccountIdentityAuthorityRevokeResult>;
}

type AuthorityHandoff = AccountIdentityCurrentMemberDeviceAuthorityHandoff;

const INSERT_AUTHORITY_SQL = `
INSERT INTO ${AUTHORITY_TABLE} (
  provider, provider_subject, mapping_status,
  account_id, household_id, member_id, role, account_state, membership_state,
  device_id, device_trust_state, session_freshness_state,
  session_id, session_generation, session_expires_at,
  authority_generation,
  child_profile_id, child_device_id, pairing_id, installation_id,
  selected_route_id, pairing_state, install_state, selected_route,
  lifecycle_state, revocation_state,
  support_receipt_id, support_provider_subject, support_account_id,
  support_member_id, support_household_id, support_device_id,
  support_child_profile_id, support_child_device_id, support_scope,
  support_issuer, support_issued_at, support_expires_at,
  support_revocation_state, support_audit_identity,
  created_at, updated_at
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
`;

const UPDATE_AUTHORITY_SQL = `
UPDATE ${AUTHORITY_TABLE}
SET mapping_status = ?, account_id = ?, household_id = ?, member_id = ?, role = ?,
    account_state = ?, membership_state = ?, device_id = ?, device_trust_state = ?,
    session_freshness_state = ?, session_id = ?, session_generation = ?, session_expires_at = ?,
    authority_generation = ?, child_profile_id = ?, child_device_id = ?, pairing_id = ?,
    installation_id = ?, selected_route_id = ?, pairing_state = ?, install_state = ?,
    selected_route = ?, lifecycle_state = ?, revocation_state = ?, support_receipt_id = ?,
    support_provider_subject = ?, support_account_id = ?, support_member_id = ?,
    support_household_id = ?, support_device_id = ?, support_child_profile_id = ?,
    support_child_device_id = ?, support_scope = ?, support_issuer = ?, support_issued_at = ?,
    support_expires_at = ?, support_revocation_state = ?, support_audit_identity = ?, updated_at = ?
WHERE provider = ? AND provider_subject = ? AND authority_generation = ?
  AND session_generation = ? AND session_id = ? AND session_id <> ?
  AND mapping_status = 'active' AND revocation_state = 'active'
`;

const REVOKE_AUTHORITY_SQL = `
UPDATE ${AUTHORITY_TABLE}
SET mapping_status = 'revoked', session_freshness_state = 'expired',
    lifecycle_state = 'removed', revocation_state = 'revoked',
    authority_generation = authority_generation + 1, updated_at = ?
WHERE provider = ? AND provider_subject = ? AND authority_generation = ?
  AND session_generation = ? AND session_id = ?
  AND mapping_status = 'active' AND revocation_state = 'active'
  AND authority_generation < ? AND session_generation < ?
`;

function isMissingAuthoritySchemaError(error: unknown): boolean {
  const message = (error instanceof Error ? error.message : String(error)).toLowerCase();
  return message.includes('no such table') && message.includes(AUTHORITY_TABLE);
}

function isDuplicateAuthorityKeyError(error: unknown): boolean {
  const message = (error instanceof Error ? error.message : String(error)).toLowerCase();
  const table = AUTHORITY_TABLE.toLowerCase().replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
  return new RegExp(
    `unique constraint failed:\\s+(?:main\\.)?${table}\\.provider,\\s+(?:main\\.)?${table}\\.provider_subject`
  ).test(message);
}

function isValidGeneration(value: number): boolean {
  return Number.isSafeInteger(value) && value > 0 && value <= MAX_SAFE_GENERATION;
}

function isValidAuthorityText(value: string): boolean {
  const normalised = value.trim();
  return (
    normalised.length > 0 &&
    normalised.length <= MAX_IDENTITY_TEXT_LENGTH &&
    !CONTROL_CHARACTER_PATTERN.test(normalised)
  );
}

function isValidProviderSubject(provider: string, providerSubject: string): providerSubject is string {
  return (provider === 'authjs' || provider === 'firebase') && isValidAuthorityText(providerSubject);
}

function supportValues(handoff: AuthorityHandoff): ReadonlyArray<string | null> {
  const receipt = handoff.member.supportReceipt;
  return receipt === null
    ? [null, null, null, null, null, null, null, null, null, null, null, null, null, null]
    : [
        receipt.receiptId,
        receipt.providerSubject,
        receipt.accountId,
        receipt.memberId,
        receipt.householdId,
        receipt.deviceId,
        receipt.childProfileId,
        receipt.childDeviceId,
        receipt.scope,
        receipt.issuer,
        receipt.issuedAt,
        receipt.expiresAt,
        receipt.revocationState,
        receipt.auditIdentity,
      ];
}

function handoffValues(handoff: AuthorityHandoff, nowMs: number): ReadonlyArray<string | number | null> {
  const member = handoff.member;
  const binding = handoff.binding;
  return [
    handoff.mapping.provider,
    handoff.mapping.providerSubject,
    handoff.mapping.status,
    member.accountId,
    member.householdId,
    member.memberId,
    member.role,
    member.accountState,
    member.membershipState,
    member.deviceId,
    member.deviceTrustState,
    member.sessionFreshnessState,
    member.sessionId,
    member.sessionGeneration,
    member.sessionExpiresAt,
    member.authorityGeneration,
    binding.childProfileId,
    binding.childDeviceId,
    binding.pairingId,
    binding.installationId,
    binding.selectedRouteId,
    binding.pairingState,
    binding.installState,
    binding.selectedRoute,
    binding.lifecycleState,
    binding.revocationState,
    ...supportValues(handoff),
    nowMs,
    nowMs,
  ];
}

async function resolveServerOwnedAuthority(
  producer: AccountOwnedAuthorityProducer,
  provider: AccountIdentityProvider,
  providerSubject: string
): Promise<ServerOwnedAccountIdentityAuthority | null> {
  if (producer === null || typeof producer !== 'object' || producer[ACCOUNT_OWNED_PRODUCER] !== true) {
    return null;
  }
  const handoff = await producer.resolveCurrentAuthority(provider, providerSubject);
  if (handoff === null) {
    return null;
  }
  return {
    handoff,
    [SERVER_OWNED_AUTHORITY]: true,
  };
}

function isCurrentTimestamp(value: string, nowMs: number): boolean {
  const timestamp = parseTrustedTimestamp(value);
  return timestamp !== null && timestamp > nowMs;
}

function parseTrustedTimestamp(value: string): number | null {
  if (!RFC3339_TIMESTAMP_PATTERN.test(value)) {
    return null;
  }
  const timestamp = Date.parse(value);
  return Number.isFinite(timestamp) ? timestamp : null;
}

function isValidSupportReceipt(handoff: AuthorityHandoff, providerSubject: string, nowMs: number): boolean {
  const receipt = handoff.member.supportReceipt;
  if (receipt === null) {
    return handoff.member.role !== 'support-admin';
  }
  const issuedAt = parseTrustedTimestamp(receipt.issuedAt);
  const expiresAt = parseTrustedTimestamp(receipt.expiresAt);
  return (
    receipt.providerSubject === providerSubject &&
    receipt.accountId === handoff.member.accountId &&
    receipt.memberId === handoff.member.memberId &&
    receipt.householdId === handoff.member.householdId &&
    receipt.deviceId === handoff.member.deviceId &&
    receipt.childProfileId === handoff.binding.childProfileId &&
    receipt.childDeviceId === handoff.binding.childDeviceId &&
    receipt.revocationState === 'active' &&
    issuedAt !== null &&
    issuedAt <= nowMs &&
    expiresAt !== null &&
    expiresAt > nowMs
  );
}

function validateSource(
  source: ServerOwnedAccountIdentityAuthority,
  provider: AccountIdentityProvider,
  providerSubject: string,
  nowMs: number
):
  | { status: 'valid'; handoff: AuthorityHandoff }
  | { status: 'rejected'; reason: 'authority-source-invalid' | 'authority-currentness-invalid' } {
  if (source === null || typeof source !== 'object' || source[SERVER_OWNED_AUTHORITY] !== true) {
    return { status: 'rejected', reason: 'authority-source-invalid' };
  }
  const parsed = AccountIdentityCurrentMemberDeviceAuthorityHandoffSchema.safeParse(source.handoff);
  if (!parsed.success) {
    return { status: 'rejected', reason: 'authority-source-invalid' };
  }
  if (
    parsed.data.mapping.provider !== provider ||
    parsed.data.mapping.providerSubject !== providerSubject ||
    parsed.data.mapping.status !== 'active' ||
    parsed.data.mapping.accountId !== parsed.data.member.accountId ||
    parsed.data.member.accountId !== parsed.data.binding.accountId ||
    parsed.data.member.householdId !== parsed.data.binding.householdId ||
    parsed.data.member.authorityGeneration !== parsed.data.binding.authorityGeneration ||
    !isValidGeneration(parsed.data.member.authorityGeneration) ||
    !isValidGeneration(parsed.data.member.sessionGeneration) ||
    parsed.data.member.accountState !== 'active' ||
    parsed.data.member.membershipState !== 'active' ||
    parsed.data.member.deviceTrustState !== 'trusted' ||
    parsed.data.member.sessionFreshnessState !== 'fresh' ||
    !isCurrentTimestamp(parsed.data.member.sessionExpiresAt, nowMs) ||
    parsed.data.binding.pairingState !== 'paired' ||
    parsed.data.binding.installState !== 'installed' ||
    parsed.data.binding.lifecycleState !== 'active' ||
    parsed.data.binding.revocationState !== 'active' ||
    !isValidSupportReceipt(parsed.data, providerSubject, nowMs)
  ) {
    return { status: 'rejected', reason: 'authority-currentness-invalid' };
  }
  return { status: 'valid', handoff: parsed.data };
}

type ValidatedAuthoritySource =
  | { status: 'valid'; handoff: AuthorityHandoff }
  | { status: 'rejected'; reason: 'authority-source-invalid' | 'authority-currentness-invalid' }
  | { status: 'manual-required'; reason: 'account-identity-authority-source-unavailable' };

async function resolveValidatedSource(
  producer: AccountOwnedAuthorityProducer,
  provider: AccountIdentityProvider,
  providerSubject: string,
  nowMs: number
): Promise<ValidatedAuthoritySource> {
  let source: ServerOwnedAccountIdentityAuthority | null;
  try {
    source = await resolveServerOwnedAuthority(producer, provider, providerSubject);
  } catch {
    return { status: 'manual-required', reason: 'account-identity-authority-source-unavailable' };
  }
  if (source === null) {
    return { status: 'manual-required', reason: 'account-identity-authority-source-unavailable' };
  }
  return validateSource(source, provider, providerSubject, nowMs);
}

async function runMutation(
  database: D1Database,
  statement: ReturnType<D1Database['prepare']>
): Promise<'written' | 'conflict'> {
  const result = await statement.run();
  return result.meta.changes === 1 ? 'written' : 'conflict';
}

export function createAccountIdentityAuthorityWriter(database: D1Database | undefined): AccountIdentityAuthorityWriter {
  const readStore = createAccountIdentityAuthorityStore(database);
  return {
    async readCurrentAuthority(provider, providerSubject) {
      try {
        return await readStore.readCurrentAuthority(provider, providerSubject);
      } catch {
        return { status: 'manual-required', reason: 'account-identity-d1-unavailable' };
      }
    },

    async createCurrentAuthority(producer, provider, providerSubject) {
      if (database === undefined) {
        return { status: 'manual-required', reason: 'account-identity-d1-authority-missing' };
      }
      if (!isValidProviderSubject(provider, providerSubject)) {
        return { status: 'rejected', reason: 'provider-subject-invalid' };
      }
      const nowMs = Date.now();
      const source = await resolveValidatedSource(producer, provider, providerSubject, nowMs);
      if (source.status === 'manual-required') {
        return source;
      }
      if (source.status === 'rejected') {
        return source;
      }
      const handoff = source.handoff;
      if (handoff.member.authorityGeneration !== 1 || handoff.member.sessionGeneration !== 1) {
        return { status: 'rejected', reason: 'authority-generation-invalid' };
      }
      try {
        const outcome = await runMutation(
          database,
          database.prepare(INSERT_AUTHORITY_SQL).bind(...handoffValues(handoff, nowMs))
        );
        return outcome === 'written'
          ? { status: 'written', authorityGeneration: handoff.member.authorityGeneration }
          : { status: 'conflict', reason: 'provider-subject-already-linked' };
      } catch (error) {
        if (isDuplicateAuthorityKeyError(error)) {
          return { status: 'conflict', reason: 'provider-subject-already-linked' };
        }
        if (isMissingAuthoritySchemaError(error)) {
          return { status: 'manual-required', reason: 'account-identity-d1-schema-missing' };
        }
        return { status: 'manual-required', reason: 'account-identity-d1-unavailable' };
      }
    },

    async compareAndSwapCurrentAuthority(
      producer,
      provider,
      providerSubject,
      expectedAuthorityGeneration,
      expectedSessionGeneration,
      expectedSessionId
    ) {
      if (database === undefined) {
        return { status: 'manual-required', reason: 'account-identity-d1-authority-missing' };
      }
      if (!isValidProviderSubject(provider, providerSubject)) {
        return { status: 'rejected', reason: 'provider-subject-invalid' };
      }
      if (
        !isValidGeneration(expectedAuthorityGeneration) ||
        expectedAuthorityGeneration >= MAX_SAFE_GENERATION ||
        !isValidGeneration(expectedSessionGeneration) ||
        expectedSessionGeneration >= MAX_SAFE_GENERATION ||
        !isValidAuthorityText(expectedSessionId)
      ) {
        return { status: 'rejected', reason: 'authority-generation-invalid' };
      }
      const nowMs = Date.now();
      const source = await resolveValidatedSource(producer, provider, providerSubject, nowMs);
      if (source.status === 'manual-required') {
        return source;
      }
      if (source.status === 'rejected') {
        return source;
      }
      const handoff = source.handoff;
      if (
        handoff.member.authorityGeneration !== expectedAuthorityGeneration + 1 ||
        handoff.member.sessionGeneration !== expectedSessionGeneration + 1
      ) {
        return { status: 'rejected', reason: 'authority-generation-invalid' };
      }
      if (handoff.member.sessionId === expectedSessionId) {
        return { status: 'rejected', reason: 'authority-currentness-invalid' };
      }
      try {
        const values = handoffValues(handoff, nowMs);
        const provider = values[0];
        const providerSubject = values[1];
        const updateValues = [...values.slice(2, -2), values[values.length - 1]];
        const outcome = await runMutation(
          database,
          database
            .prepare(UPDATE_AUTHORITY_SQL)
            .bind(
              ...updateValues,
              provider,
              providerSubject,
              expectedAuthorityGeneration,
              expectedSessionGeneration,
              expectedSessionId,
              handoff.member.sessionId
            )
        );
        return outcome === 'written'
          ? { status: 'written', authorityGeneration: handoff.member.authorityGeneration }
          : { status: 'conflict', reason: 'authority-generation-conflict' };
      } catch (error) {
        if (isMissingAuthoritySchemaError(error)) {
          return { status: 'manual-required', reason: 'account-identity-d1-schema-missing' };
        }
        return { status: 'manual-required', reason: 'account-identity-d1-unavailable' };
      }
    },

    async revokeCurrentAuthority(
      provider,
      providerSubject,
      expectedAuthorityGeneration,
      expectedSessionGeneration,
      expectedSessionId
    ) {
      if (database === undefined) {
        return { status: 'manual-required', reason: 'account-identity-d1-authority-missing' };
      }
      if (!isValidProviderSubject(provider, providerSubject)) {
        return { status: 'rejected', reason: 'provider-subject-invalid' };
      }
      if (
        !isValidGeneration(expectedAuthorityGeneration) ||
        expectedAuthorityGeneration >= MAX_SAFE_GENERATION ||
        !isValidGeneration(expectedSessionGeneration) ||
        expectedSessionGeneration >= MAX_SAFE_GENERATION ||
        !isValidAuthorityText(expectedSessionId)
      ) {
        return { status: 'rejected', reason: 'authority-generation-invalid' };
      }
      const nowMs = Date.now();
      try {
        const outcome = await runMutation(
          database,
          database
            .prepare(REVOKE_AUTHORITY_SQL)
            .bind(
              nowMs,
              provider,
              providerSubject.trim(),
              expectedAuthorityGeneration,
              expectedSessionGeneration,
              expectedSessionId,
              MAX_SAFE_GENERATION,
              MAX_SAFE_GENERATION
            )
        );
        return outcome === 'written'
          ? { status: 'revoked', authorityGeneration: expectedAuthorityGeneration + 1 }
          : { status: 'conflict', reason: 'authority-generation-conflict' };
      } catch (error) {
        if (isMissingAuthoritySchemaError(error)) {
          return { status: 'manual-required', reason: 'account-identity-d1-schema-missing' };
        }
        return { status: 'manual-required', reason: 'account-identity-d1-unavailable' };
      }
    },
  };
}
