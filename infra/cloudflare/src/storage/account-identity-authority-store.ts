import type { D1Database } from '@cloudflare/workers-types';
import {
  AccountIdentityCurrentMemberDeviceAuthorityHandoffSchema,
  type AccountIdentityCurrentMemberDeviceAuthorityHandoff,
  type AccountIdentityProvider,
} from '@ocentra-parent/schema-domain/account-identity-authority';

const VERIFIED_AUTHORITY_BRAND = Symbol('account-identity-verified-authority');

/**
 * Non-serde capability minted only after the Account-owned D1 row has passed
 * currentness, session, target, and support-receipt validation. The DTO is
 * available only as an explicit evidence JSON string.
 */
export class VerifiedAccountIdentityAuthorityCapability {
  readonly [VERIFIED_AUTHORITY_BRAND] = true;
  readonly #evidenceJson: string;
  readonly #provenance: {
    provider: AccountIdentityProvider;
    providerSubject: string;
    accountId: string;
    householdId: string;
    memberId: string;
    role: AccountIdentityCurrentMemberDeviceAuthorityHandoff['member']['role'];
    deviceId: string;
    childProfileId: string;
    childDeviceId: string;
    sessionId: string;
    sessionGeneration: number;
    authorityGeneration: number;
    supportScope: NonNullable<
      AccountIdentityCurrentMemberDeviceAuthorityHandoff['member']['supportReceipt']
    >['scope'] | null;
    supportIssuer: string | null;
    supportAuditIdentity: string | null;
  };

  private constructor(evidence: AccountIdentityCurrentMemberDeviceAuthorityHandoff) {
    this.#evidenceJson = JSON.stringify(evidence);
    this.#provenance = Object.freeze({
      provider: evidence.mapping.provider,
      providerSubject: evidence.mapping.providerSubject,
      accountId: evidence.member.accountId,
      householdId: evidence.member.householdId,
      memberId: evidence.member.memberId,
      role: evidence.member.role,
      deviceId: evidence.member.deviceId,
      childProfileId: evidence.binding.childProfileId,
      childDeviceId: evidence.binding.childDeviceId,
      sessionId: evidence.member.sessionId,
      sessionGeneration: evidence.member.sessionGeneration,
      authorityGeneration: evidence.member.authorityGeneration,
      supportScope: evidence.member.supportReceipt?.scope ?? null,
      supportIssuer: evidence.member.supportReceipt?.issuer ?? null,
      supportAuditIdentity: evidence.member.supportReceipt?.auditIdentity ?? null,
    });
  }

  get provider(): AccountIdentityProvider {
    return this.#provenance.provider;
  }

  get providerSubject(): string {
    return this.#provenance.providerSubject;
  }

  get accountId(): string {
    return this.#provenance.accountId;
  }

  get householdId(): string {
    return this.#provenance.householdId;
  }

  get memberId(): string {
    return this.#provenance.memberId;
  }

  get role(): AccountIdentityCurrentMemberDeviceAuthorityHandoff['member']['role'] {
    return this.#provenance.role;
  }

  get deviceId(): string {
    return this.#provenance.deviceId;
  }

  get childProfileId(): string {
    return this.#provenance.childProfileId;
  }

  get childDeviceId(): string {
    return this.#provenance.childDeviceId;
  }

  get sessionId(): string {
    return this.#provenance.sessionId;
  }

  get sessionGeneration(): number {
    return this.#provenance.sessionGeneration;
  }

  get authorityGeneration(): number {
    return this.#provenance.authorityGeneration;
  }

  get supportScope(): NonNullable<
    AccountIdentityCurrentMemberDeviceAuthorityHandoff['member']['supportReceipt']
  >['scope'] | null {
    return this.#provenance.supportScope;
  }

  get supportIssuer(): string | null {
    return this.#provenance.supportIssuer;
  }

  get supportAuditIdentity(): string | null {
    return this.#provenance.supportAuditIdentity;
  }

  toEvidenceJson(): string {
    return this.#evidenceJson;
  }

}

function mintVerifiedAuthorityCapability(
  evidence: AccountIdentityCurrentMemberDeviceAuthorityHandoff
): VerifiedAccountIdentityAuthorityCapability {
  return new VerifiedAccountIdentityAuthorityCapability(evidence);
}

export type AccountIdentityAuthorityReadResult =
  | { status: 'trusted'; capability: VerifiedAccountIdentityAuthorityCapability }
  | { status: 'not-found' }
  | {
      status: 'manual-required';
      reason: 'account-identity-d1-authority-missing' | 'account-identity-d1-schema-missing';
    }
  | {
      status: 'rejected';
      reason:
        | 'provider-subject-invalid'
        | 'provider-mapping-inactive'
        | 'provider-mapping-account-mismatch'
        | 'authority-schema-invalid'
        | 'authority-currentness-invalid'
        | 'authority-session-stale'
        | 'authority-session-expired'
        | 'support-receipt-required'
        | 'support-receipt-invalid'
        | 'support-receipt-revoked';
    };

export interface AccountIdentityAuthorityStore {
  readCurrentAuthority(
    provider: AccountIdentityProvider,
    providerSubject: string
  ): Promise<AccountIdentityAuthorityReadResult>;
}

const ACCOUNT_IDENTITY_AUTHORITY_SELECT_SQL = `
SELECT provider, provider_subject, mapping_status,
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
       support_revocation_state, support_audit_identity
FROM ocentra_account_identity_current_authority
WHERE provider = ? AND provider_subject = ?
LIMIT 1
`;

const MAX_IDENTITY_ID_LENGTH = 256;
const CONTROL_CHARACTER_PATTERN = /[\u0000-\u001f\u007f]/;

interface AccountIdentityAuthorityRow {
  provider: AccountIdentityProvider;
  provider_subject: string;
  mapping_status: 'active' | 'revoked';
  account_id: string;
  household_id: string;
  member_id: string;
  role: 'parent-owner' | 'co-parent-guardian' | 'observer' | 'child-profile' | 'child-device-agent' | 'support-admin';
  account_state: 'active' | 'suspended' | 'disabled';
  membership_state: 'invited' | 'pending' | 'active' | 'revoked' | 'disabled';
  device_id: string;
  device_trust_state: 'pending' | 'trusted' | 'revoked' | 'reset-required' | 'disabled';
  session_freshness_state: 'fresh' | 'stale' | 'expired';
  session_id: string;
  session_generation: number;
  session_expires_at: string;
  authority_generation: number;
  child_profile_id: string;
  child_device_id: string;
  pairing_id: string;
  installation_id: string;
  selected_route_id: string;
  pairing_state: 'pending' | 'paired' | 'unpaired';
  install_state: 'pending' | 'installed' | 'failed';
  selected_route: 'local' | 'lan' | 'remote' | 'manual-required';
  lifecycle_state: 'pending' | 'active' | 'suspended' | 'removed';
  revocation_state: 'active' | 'revoked';
  support_receipt_id: string | null;
  support_provider_subject: string | null;
  support_account_id: string | null;
  support_member_id: string | null;
  support_household_id: string | null;
  support_device_id: string | null;
  support_child_profile_id: string | null;
  support_child_device_id: string | null;
  support_scope: 'read-only' | 'household' | 'device-control' | null;
  support_issuer: string | null;
  support_issued_at: string | null;
  support_expires_at: string | null;
  support_revocation_state: 'active' | 'revoked' | null;
  support_audit_identity: string | null;
}

function normaliseIdentityText(value: string): string | null {
  const normalised = value.trim();
  if (
    normalised.length === 0 ||
    normalised.length > MAX_IDENTITY_ID_LENGTH ||
    CONTROL_CHARACTER_PATTERN.test(normalised)
  ) {
    return null;
  }
  return normalised;
}

function isSupportedProvider(value: string): value is AccountIdentityProvider {
  return value === 'authjs' || value === 'firebase';
}

function isMissingAccountIdentitySchemaError(error: unknown): boolean {
  const message = (error instanceof Error ? error.message : String(error)).toLowerCase();
  return message.includes('no such table') && message.includes('ocentra_account_identity_current_authority');
}

function isCurrentTimestamp(value: string, now: number): boolean {
  const timestamp = Date.parse(value);
  return Number.isFinite(timestamp) && timestamp > now;
}

function isValidSupportReceipt(row: AccountIdentityAuthorityRow, now: number): boolean {
  const values = [
    row.support_receipt_id,
    row.support_provider_subject,
    row.support_account_id,
    row.support_member_id,
    row.support_household_id,
    row.support_device_id,
    row.support_child_profile_id,
    row.support_child_device_id,
    row.support_scope,
    row.support_issuer,
    row.support_issued_at,
    row.support_expires_at,
    row.support_revocation_state,
    row.support_audit_identity,
  ];
  const hasReceipt = values.some((value) => value !== null);
  if (!hasReceipt) {
    return row.role !== 'support-admin';
  }
  if (values.some((value) => value === null)) {
    return false;
  }
  return (
    row.support_provider_subject === row.provider_subject &&
    row.support_account_id === row.account_id &&
    row.support_member_id === row.member_id &&
    row.support_household_id === row.household_id &&
    row.support_device_id === row.device_id &&
    row.support_child_profile_id === row.child_profile_id &&
    row.support_child_device_id === row.child_device_id &&
    row.support_revocation_state === 'active' &&
    Number.isFinite(Date.parse(row.support_issued_at!)) &&
    Date.parse(row.support_issued_at!) <= now &&
    Number.isFinite(Date.parse(row.support_expires_at!)) &&
    Date.parse(row.support_expires_at!) > now
  );
}

function toEvidence(row: AccountIdentityAuthorityRow): unknown {
  return {
    schemaVersion: 'v0.1',
    mapping: {
      accountId: row.account_id,
      provider: row.provider,
      providerSubject: row.provider_subject,
      status: row.mapping_status,
    },
    member: {
      accountId: row.account_id,
      householdId: row.household_id,
      memberId: row.member_id,
      role: row.role,
      accountState: row.account_state,
      membershipState: row.membership_state,
      deviceId: row.device_id,
      deviceTrustState: row.device_trust_state,
      sessionFreshnessState: row.session_freshness_state,
      sessionId: row.session_id,
      sessionGeneration: row.session_generation,
      sessionExpiresAt: row.session_expires_at,
      supportReceipt:
        row.support_receipt_id === null
          ? null
          : {
              receiptId: row.support_receipt_id,
              providerSubject: row.support_provider_subject,
              accountId: row.support_account_id,
              memberId: row.support_member_id,
              householdId: row.support_household_id,
              deviceId: row.support_device_id,
              childProfileId: row.support_child_profile_id,
              childDeviceId: row.support_child_device_id,
              scope: row.support_scope,
              issuer: row.support_issuer,
              issuedAt: row.support_issued_at,
              expiresAt: row.support_expires_at,
              revocationState: row.support_revocation_state,
              auditIdentity: row.support_audit_identity,
            },
      authorityGeneration: row.authority_generation,
    },
    binding: {
      accountId: row.account_id,
      householdId: row.household_id,
      childProfileId: row.child_profile_id,
      childDeviceId: row.child_device_id,
      pairingId: row.pairing_id,
      installationId: row.installation_id,
      selectedRouteId: row.selected_route_id,
      pairingState: row.pairing_state,
      installState: row.install_state,
      selectedRoute: row.selected_route,
      lifecycleState: row.lifecycle_state,
      revocationState: row.revocation_state,
      authorityGeneration: row.authority_generation,
    },
  };
}

function validateAuthorityRow(
  row: AccountIdentityAuthorityRow,
  now: number
):
  | { status: 'trusted'; evidence: AccountIdentityCurrentMemberDeviceAuthorityHandoff }
  | { status: 'rejected'; reason: Extract<AccountIdentityAuthorityReadResult, { status: 'rejected' }>['reason'] } {
  if (row.provider_subject.trim() === '' || row.provider_subject.length > MAX_IDENTITY_ID_LENGTH) {
    return { status: 'rejected', reason: 'provider-subject-invalid' };
  }
  if (row.mapping_status !== 'active') {
    return { status: 'rejected', reason: 'provider-mapping-inactive' };
  }
  if (row.account_state !== 'active' || row.membership_state !== 'active') {
    return { status: 'rejected', reason: 'provider-mapping-inactive' };
  }
  if (row.session_freshness_state !== 'fresh') {
    return { status: 'rejected', reason: 'authority-session-stale' };
  }
  if (!isCurrentTimestamp(row.session_expires_at, now)) {
    return { status: 'rejected', reason: 'authority-session-expired' };
  }
  if (!isValidSupportReceipt(row, now)) {
    if (row.support_revocation_state === 'revoked') {
      return { status: 'rejected', reason: 'support-receipt-revoked' };
    }
    return {
      status: 'rejected',
      reason: row.role === 'support-admin' ? 'support-receipt-required' : 'support-receipt-invalid',
    };
  }
  if (row.device_trust_state !== 'trusted') {
    return { status: 'rejected', reason: 'authority-currentness-invalid' };
  }
  if (
    row.pairing_state !== 'paired' ||
    row.install_state !== 'installed' ||
    row.lifecycle_state !== 'active' ||
    row.revocation_state !== 'active'
  ) {
    return { status: 'rejected', reason: 'authority-currentness-invalid' };
  }
  const parsed = AccountIdentityCurrentMemberDeviceAuthorityHandoffSchema.safeParse(toEvidence(row));
  if (!parsed.success) {
    return { status: 'rejected', reason: 'authority-schema-invalid' };
  }
  if (
    parsed.data.mapping.status !== 'active' ||
    parsed.data.mapping.accountId !== parsed.data.member.accountId ||
    parsed.data.member.accountId !== parsed.data.binding.accountId ||
    parsed.data.member.householdId !== parsed.data.binding.householdId ||
    parsed.data.member.authorityGeneration !== parsed.data.binding.authorityGeneration ||
    parsed.data.member.sessionGeneration !== row.session_generation ||
    parsed.data.member.sessionId !== row.session_id ||
    parsed.data.member.authorityGeneration !== row.authority_generation
  ) {
    return { status: 'rejected', reason: 'authority-currentness-invalid' };
  }
  return { status: 'trusted', evidence: parsed.data };
}

export function createAccountIdentityAuthorityStore(database: D1Database | undefined): AccountIdentityAuthorityStore {
  return {
    async readCurrentAuthority(provider, providerSubject) {
      if (database === undefined) {
        return { status: 'manual-required', reason: 'account-identity-d1-authority-missing' };
      }
      const normalisedSubject = normaliseIdentityText(providerSubject);
      if (!isSupportedProvider(provider) || normalisedSubject === null) {
        return { status: 'rejected', reason: 'provider-subject-invalid' };
      }
      try {
        const row = await database
          .prepare(ACCOUNT_IDENTITY_AUTHORITY_SELECT_SQL)
          .bind(provider, normalisedSubject)
          .first<AccountIdentityAuthorityRow>();
        if (row === null) {
          return { status: 'not-found' };
        }
        const validated = validateAuthorityRow(row, Date.now());
        if (validated.status === 'rejected') {
          return validated;
        }
        return {
          status: 'trusted',
          capability: mintVerifiedAuthorityCapability(validated.evidence),
        };
      } catch (error) {
        if (isMissingAccountIdentitySchemaError(error)) {
          return { status: 'manual-required', reason: 'account-identity-d1-schema-missing' };
        }
        throw error;
      }
    },
  };
}
