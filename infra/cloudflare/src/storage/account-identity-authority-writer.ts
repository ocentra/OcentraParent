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
const SERVER_OWNED_AUTHORITY = Symbol('server-owned-account-authority');
const AUTHORITY_TABLE = 'ocentra_account_identity_current_authority';

/**
 * Only an Account-owned adapter may construct this value.  The provider caller
 * intentionally cannot manufacture a family, device, role, session, or
 * currentness decision from request data.
 */
export interface ServerOwnedAccountIdentityAuthority {
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
        | 'account-identity-d1-unavailable';
    }
  | {
      status: 'rejected';
      reason: 'authority-source-invalid' | 'authority-generation-invalid' | 'provider-subject-invalid';
    };

export type AccountIdentityAuthorityRevokeResult =
  | { status: 'revoked'; authorityGeneration: number }
  | { status: 'conflict'; reason: 'authority-generation-conflict' }
  | {
      status: 'manual-required';
      reason:
        | 'account-identity-d1-authority-missing'
        | 'account-identity-d1-schema-missing'
        | 'account-identity-d1-unavailable';
    }
  | { status: 'rejected'; reason: 'provider-subject-invalid' | 'authority-generation-invalid' };

export interface AccountIdentityAuthorityWriter {
  readCurrentAuthority(
    provider: AccountIdentityProvider,
    providerSubject: string
  ): Promise<AccountIdentityAuthorityReadResult>;
  createCurrentAuthority(source: ServerOwnedAccountIdentityAuthority): Promise<AccountIdentityAuthorityWriteResult>;
  compareAndSwapCurrentAuthority(
    source: ServerOwnedAccountIdentityAuthority,
    expectedAuthorityGeneration: number
  ): Promise<AccountIdentityAuthorityWriteResult>;
  revokeCurrentAuthority(
    provider: AccountIdentityProvider,
    providerSubject: string,
    expectedAuthorityGeneration: number,
    nowMs?: number
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
  AND mapping_status = 'active' AND revocation_state = 'active'
`;

const REVOKE_AUTHORITY_SQL = `
UPDATE ${AUTHORITY_TABLE}
SET mapping_status = 'revoked', session_freshness_state = 'expired',
    lifecycle_state = 'removed', revocation_state = 'revoked',
    authority_generation = authority_generation + 1, updated_at = ?
WHERE provider = ? AND provider_subject = ? AND authority_generation = ?
  AND mapping_status = 'active' AND revocation_state = 'active'
  AND authority_generation < ?
`;

function isMissingAuthoritySchemaError(error: unknown): boolean {
  const message = (error instanceof Error ? error.message : String(error)).toLowerCase();
  return message.includes('no such table') && message.includes(AUTHORITY_TABLE);
}

function isValidGeneration(value: number): boolean {
  return Number.isSafeInteger(value) && value > 0 && value <= MAX_SAFE_GENERATION;
}

function isValidProviderSubject(provider: string, providerSubject: string): providerSubject is string {
  return (provider === 'authjs' || provider === 'firebase') && providerSubject.trim().length > 0;
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

function validateSource(source: ServerOwnedAccountIdentityAuthority): AuthorityHandoff | null {
  if (source === null || typeof source !== 'object' || source[SERVER_OWNED_AUTHORITY] !== true) {
    return null;
  }
  const parsed = AccountIdentityCurrentMemberDeviceAuthorityHandoffSchema.safeParse(source.handoff);
  if (!parsed.success || parsed.data.mapping.status !== 'active') {
    return null;
  }
  if (
    parsed.data.mapping.accountId !== parsed.data.member.accountId ||
    parsed.data.member.accountId !== parsed.data.binding.accountId ||
    parsed.data.member.householdId !== parsed.data.binding.householdId ||
    parsed.data.member.authorityGeneration !== parsed.data.binding.authorityGeneration ||
    !isValidGeneration(parsed.data.member.authorityGeneration) ||
    !isValidGeneration(parsed.data.member.sessionGeneration)
  ) {
    return null;
  }
  return parsed.data;
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

    async createCurrentAuthority(source) {
      if (database === undefined) {
        return { status: 'manual-required', reason: 'account-identity-d1-authority-missing' };
      }
      const handoff = validateSource(source);
      if (handoff === null) {
        return { status: 'rejected', reason: 'authority-source-invalid' };
      }
      try {
        const outcome = await runMutation(
          database,
          database.prepare(INSERT_AUTHORITY_SQL).bind(...handoffValues(handoff, Date.now()))
        );
        return outcome === 'written'
          ? { status: 'written', authorityGeneration: handoff.member.authorityGeneration }
          : { status: 'conflict', reason: 'provider-subject-already-linked' };
      } catch (error) {
        if (isMissingAuthoritySchemaError(error)) {
          return { status: 'manual-required', reason: 'account-identity-d1-schema-missing' };
        }
        return { status: 'manual-required', reason: 'account-identity-d1-unavailable' };
      }
    },

    async compareAndSwapCurrentAuthority(source, expectedAuthorityGeneration) {
      if (database === undefined) {
        return { status: 'manual-required', reason: 'account-identity-d1-authority-missing' };
      }
      const handoff = validateSource(source);
      if (handoff === null) {
        return { status: 'rejected', reason: 'authority-source-invalid' };
      }
      if (
        !isValidGeneration(expectedAuthorityGeneration) ||
        handoff.member.authorityGeneration !== expectedAuthorityGeneration + 1 ||
        !isValidGeneration(handoff.member.authorityGeneration)
      ) {
        return { status: 'rejected', reason: 'authority-generation-invalid' };
      }
      try {
        const values = handoffValues(handoff, Date.now());
        const provider = values[0];
        const providerSubject = values[1];
        const updateValues = [...values.slice(2, -2), values[values.length - 1]];
        const outcome = await runMutation(
          database,
          database
            .prepare(UPDATE_AUTHORITY_SQL)
            .bind(...updateValues, provider, providerSubject, expectedAuthorityGeneration)
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

    async revokeCurrentAuthority(provider, providerSubject, expectedAuthorityGeneration, nowMs = Date.now()) {
      if (database === undefined) {
        return { status: 'manual-required', reason: 'account-identity-d1-authority-missing' };
      }
      if (!isValidProviderSubject(provider, providerSubject)) {
        return { status: 'rejected', reason: 'provider-subject-invalid' };
      }
      if (!isValidGeneration(expectedAuthorityGeneration) || expectedAuthorityGeneration >= MAX_SAFE_GENERATION) {
        return { status: 'rejected', reason: 'authority-generation-invalid' };
      }
      try {
        const outcome = await runMutation(
          database,
          database
            .prepare(REVOKE_AUTHORITY_SQL)
            .bind(nowMs, provider, providerSubject.trim(), expectedAuthorityGeneration, MAX_SAFE_GENERATION)
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
