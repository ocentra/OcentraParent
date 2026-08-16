import type { D1Database } from '@cloudflare/workers-types';
import {
  ACCOUNT_AUTH_ADAPTER_MANUAL_REQUIRED_BLOCKER,
} from './auth/verifier.js';
import {
  AccountIdentityAuthorityHandoffSchema,
  AccountIdentityAuthoritySchemaVersion,
  type AccountIdentityAuthorityHandoff,
  type AccountIdentityProviderSubjectMapping,
} from '@ocentra-parent/schema-domain/account-identity-authority';
import {
  createAccountIdentityStore,
  type AccountIdentityStore,
  type AccountIdentityUpsertResult,
} from './storage/account-identity-store.js';

/**
 * Opaque input reserved for a runtime-owned provider verifier. The weak set is
 * intentionally module-private and has no issuer while provider verification
 * is manual-required, so plain objects cannot authorize a D1 write.
 */
const runtimeVerifiedProviderMappings = new WeakSet<object>();
export type RuntimeVerifiedProviderMapping = {
  readonly mapping: AccountIdentityProviderSubjectMapping;
};

export type AccountIdentityD1AdapterResult =
  | AccountIdentityUpsertResult
  | { status: 'manual-required'; reason: typeof ACCOUNT_AUTH_ADAPTER_MANUAL_REQUIRED_BLOCKER }
  | {
      status: 'invalid-handoff';
      reason:
        | 'schema-version-mismatch'
        | 'mapping-account-id-mismatch'
        | 'mapping-provider-mismatch'
        | 'mapping-subject-mismatch'
        | 'mapping-status-mismatch'
        | 'authority-account-id-mismatch'
        | 'handoff-schema-invalid';
    };

type AccountIdentityMappingMismatch =
  | 'mapping-account-id-mismatch'
  | 'mapping-provider-mismatch'
  | 'mapping-subject-mismatch'
  | 'mapping-status-mismatch';

function isRuntimeVerifiedProviderMapping(
  value: RuntimeVerifiedProviderMapping | null
): value is RuntimeVerifiedProviderMapping {
  return value !== null && typeof value === 'object' && runtimeVerifiedProviderMappings.has(value);
}

export interface AccountIdentityD1Adapter {
  /**
   * Persist only a verified provider-subject mapping. The optional authority
   * snapshot is deliberately validated but never written to Cloudflare D1.
   */
  persistVerifiedProviderMapping(
    verifiedMapping: RuntimeVerifiedProviderMapping | null,
    handoff: AccountIdentityAuthorityHandoff,
    nowMs: number
  ): Promise<AccountIdentityD1AdapterResult>;
}

function mappingMatches(
  verifiedMapping: AccountIdentityProviderSubjectMapping,
  handoffMapping: AccountIdentityProviderSubjectMapping
): AccountIdentityMappingMismatch | null {
  if (verifiedMapping.accountId !== handoffMapping.accountId) {
    return 'mapping-account-id-mismatch';
  }
  if (verifiedMapping.provider !== handoffMapping.provider) {
    return 'mapping-provider-mismatch';
  }
  if (verifiedMapping.providerSubject !== handoffMapping.providerSubject) {
    return 'mapping-subject-mismatch';
  }
  if (verifiedMapping.status !== handoffMapping.status) {
    return 'mapping-status-mismatch';
  }
  return null;
}

function persistMapping(
  store: AccountIdentityStore,
  handoff: AccountIdentityAuthorityHandoff,
  nowMs: number
): Promise<AccountIdentityD1AdapterResult> {
  return store.upsert({
    accountId: handoff.mapping.accountId,
    provider: handoff.mapping.provider,
    providerSubject: handoff.mapping.providerSubject,
    status: handoff.mapping.status,
    nowMs,
  });
}

/**
 * Cloudflare's narrow D1 consumer. This adapter is not registered in the
 * public route manifest until a concrete runtime-owned provider verifier exists.
 */
export function createAccountIdentityD1Adapter(database: D1Database | undefined): AccountIdentityD1Adapter {
  const store = createAccountIdentityStore(database);

  return {
    async persistVerifiedProviderMapping(verifiedMapping, handoff, nowMs) {
      if (!isRuntimeVerifiedProviderMapping(verifiedMapping)) {
        return { status: 'manual-required', reason: ACCOUNT_AUTH_ADAPTER_MANUAL_REQUIRED_BLOCKER };
      }

      const parsedHandoff = AccountIdentityAuthorityHandoffSchema.safeParse(handoff);
      if (!parsedHandoff.success) {
        return { status: 'invalid-handoff', reason: 'handoff-schema-invalid' };
      }
      const validatedHandoff = parsedHandoff.data;

      if (validatedHandoff.schemaVersion !== AccountIdentityAuthoritySchemaVersion.V0_6) {
        return { status: 'invalid-handoff', reason: 'schema-version-mismatch' };
      }
      if (validatedHandoff.authority !== null && validatedHandoff.authority.accountId !== validatedHandoff.mapping.accountId) {
        return { status: 'invalid-handoff', reason: 'authority-account-id-mismatch' };
      }

      const mismatch = mappingMatches(verifiedMapping.mapping, validatedHandoff.mapping);
      if (mismatch !== null) {
        return { status: 'invalid-handoff', reason: mismatch };
      }

      return persistMapping(store, validatedHandoff, nowMs);
    },
  };
}
