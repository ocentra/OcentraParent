import type { D1Database } from '@cloudflare/workers-types';
import {
  AccountIdentityAuthorityHandoffSchema,
  type AccountIdentityAuthorityHandoff,
  type AccountIdentityProvider,
} from '@ocentra-parent/schema-domain/account-identity-authority';

export interface AccountIdentityBindingLookup {
  provider: AccountIdentityProvider;
  providerSubject: string;
  householdId: string;
  childProfileId: string;
  childDeviceId: string;
}

export type AccountIdentityAuthorityReadResult =
  | { status: 'trusted'; handoff: AccountIdentityAuthorityHandoff }
  | { status: 'not-found' }
  | {
      status: 'manual-required';
      reason: 'account-identity-d1-binding-missing' | 'account-identity-d1-schema-missing';
    }
  | {
      status: 'rejected';
      reason:
        | 'provider-mapping-inactive'
        | 'provider-mapping-account-mismatch'
        | 'binding-schema-invalid'
        | 'binding-pairing-incomplete'
        | 'binding-install-incomplete'
        | 'binding-lifecycle-inactive'
        | 'binding-revoked';
    };

export interface AccountIdentityAuthorityStore {
  readCurrentBinding(lookup: AccountIdentityBindingLookup): Promise<AccountIdentityAuthorityReadResult>;
}

const ACCOUNT_IDENTITY_MAPPING_SELECT_SQL = `
SELECT account_id, provider, provider_subject, status
FROM ocentra_account_identities
WHERE provider = ? AND provider_subject = ?
LIMIT 1
`;

const ACCOUNT_IDENTITY_BINDING_SELECT_SQL = `
SELECT account_id, household_id, child_profile_id, child_device_id,
       pairing_id, installation_id, selected_route_id, pairing_state,
       install_state, selected_route, lifecycle_state, revocation_state,
       authority_generation
FROM ocentra_account_identity_bindings
WHERE household_id = ? AND child_profile_id = ? AND child_device_id = ?
LIMIT 1
`;

const MAX_IDENTITY_ID_LENGTH = 256;
const CONTROL_CHARACTER_PATTERN = /[\u0000-\u001f\u007f]/;

interface AccountIdentityMappingRow {
  account_id: string;
  provider: AccountIdentityProvider;
  provider_subject: string;
  status: 'active' | 'revoked';
}

interface AccountIdentityBindingRow {
  account_id: string;
  household_id: string;
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
  authority_generation: number;
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
  return (
    message.includes('no such table') &&
    (message.includes('ocentra_account_identities') || message.includes('ocentra_account_identity_bindings'))
  );
}

function invalidLookup(lookup: AccountIdentityBindingLookup): boolean {
  return (
    !isSupportedProvider(lookup.provider) ||
    normaliseIdentityText(lookup.providerSubject) === null ||
    normaliseIdentityText(lookup.householdId) === null ||
    normaliseIdentityText(lookup.childProfileId) === null ||
    normaliseIdentityText(lookup.childDeviceId) === null
  );
}

function toHandoff(mapping: AccountIdentityMappingRow, binding: AccountIdentityBindingRow): unknown {
  return {
    schemaVersion: 'v0.7',
    mapping: {
      accountId: mapping.account_id,
      provider: mapping.provider,
      providerSubject: mapping.provider_subject,
      status: mapping.status,
    },
    binding: {
      accountId: binding.account_id,
      householdId: binding.household_id,
      childProfileId: binding.child_profile_id,
      childDeviceId: binding.child_device_id,
      pairingId: binding.pairing_id,
      installationId: binding.installation_id,
      selectedRouteId: binding.selected_route_id,
      pairingState: binding.pairing_state,
      installState: binding.install_state,
      selectedRoute: binding.selected_route,
      lifecycleState: binding.lifecycle_state,
      revocationState: binding.revocation_state,
      authorityGeneration: binding.authority_generation,
    },
  };
}

export function createAccountIdentityAuthorityStore(database: D1Database | undefined): AccountIdentityAuthorityStore {
  return {
    async readCurrentBinding(lookup) {
      if (database === undefined) {
        return { status: 'manual-required', reason: 'account-identity-d1-binding-missing' };
      }
      if (invalidLookup(lookup)) {
        return { status: 'rejected', reason: 'binding-schema-invalid' };
      }

      const providerSubject = normaliseIdentityText(lookup.providerSubject)!;
      const householdId = normaliseIdentityText(lookup.householdId)!;
      const childProfileId = normaliseIdentityText(lookup.childProfileId)!;
      const childDeviceId = normaliseIdentityText(lookup.childDeviceId)!;

      try {
        const mapping = await database
          .prepare(ACCOUNT_IDENTITY_MAPPING_SELECT_SQL)
          .bind(lookup.provider, providerSubject)
          .first<AccountIdentityMappingRow>();
        if (mapping === null) {
          return { status: 'not-found' };
        }
        const binding = await database
          .prepare(ACCOUNT_IDENTITY_BINDING_SELECT_SQL)
          .bind(householdId, childProfileId, childDeviceId)
          .first<AccountIdentityBindingRow>();
        if (binding === null) {
          return { status: 'not-found' };
        }

        const parsed = AccountIdentityAuthorityHandoffSchema.safeParse(toHandoff(mapping, binding));
        if (!parsed.success) {
          return { status: 'rejected', reason: 'binding-schema-invalid' };
        }
        if (parsed.data.mapping.status !== 'active') {
          return { status: 'rejected', reason: 'provider-mapping-inactive' };
        }
        if (parsed.data.mapping.accountId !== parsed.data.binding.accountId) {
          return { status: 'rejected', reason: 'provider-mapping-account-mismatch' };
        }
        if (parsed.data.binding.pairingState !== 'paired') {
          return { status: 'rejected', reason: 'binding-pairing-incomplete' };
        }
        if (parsed.data.binding.installState !== 'installed') {
          return { status: 'rejected', reason: 'binding-install-incomplete' };
        }
        if (parsed.data.binding.lifecycleState !== 'active') {
          return { status: 'rejected', reason: 'binding-lifecycle-inactive' };
        }
        if (parsed.data.binding.revocationState !== 'active') {
          return { status: 'rejected', reason: 'binding-revoked' };
        }
        return { status: 'trusted', handoff: parsed.data };
      } catch (error) {
        if (isMissingAccountIdentitySchemaError(error)) {
          return { status: 'manual-required', reason: 'account-identity-d1-schema-missing' };
        }
        throw error;
      }
    },
  };
}
