import type { Env } from '../env.js';
import {
  createAccountIdentityAuthorityWriter,
  type AccountIdentityAuthorityWriter,
} from '../storage/account-identity-authority-writer.js';
import type { VerifiedAccountIdentityAuthorityCapability } from '../storage/account-identity-authority-store.js';
import type { ProviderVerificationPort, ProviderVerificationResult } from './verifier.js';
import type { AccountIdentityAuthorityIssuerV2CurrentVerification } from './account-identity-authority-issuer-v2.js';

export type VerifiedProviderAuthorityResult =
  | {
      status: 'trusted';
      providerIdentity: Extract<ProviderVerificationResult, { status: 'verified' }>['identity'];
      capability: VerifiedAccountIdentityAuthorityCapability;
    }
  | {
      status: 'not-found';
      providerIdentity: Extract<ProviderVerificationResult, { status: 'verified' }>['identity'];
    }
  | { status: 'provider-unavailable' }
  | {
      status: 'manual-required';
      reason:
        | 'account-identity-d1-authority-missing'
        | 'account-identity-d1-schema-missing'
        | 'account-identity-d1-unavailable';
    }
  | {
      status: 'rejected';
      reason: string;
    };

export interface AccountIdentityAuthorityCaller {
  resolveVerifiedProviderAuthority(
    request: Request,
    providerVerifier: ProviderVerificationPort | undefined
  ): Promise<VerifiedProviderAuthorityResult>;
  verifyAccountIssuerFrame(
    frame: ArrayBuffer | Uint8Array,
    nowMs?: number
  ): Promise<AccountIdentityAuthorityIssuerV2CurrentVerification>;
}

function createCaller(writer: AccountIdentityAuthorityWriter): AccountIdentityAuthorityCaller {
  return {
    async resolveVerifiedProviderAuthority(request, providerVerifier) {
      // This handoff resolves only provider identity against current durable
      // Account state. It does not mint action capability, controller lease,
      // step-up, or target selectors; those owned sources remain fail-closed.
      if (providerVerifier === undefined) {
        return { status: 'provider-unavailable' };
      }

      let verification: ProviderVerificationResult;
      try {
        verification = await providerVerifier.verify(request);
      } catch {
        return { status: 'provider-unavailable' };
      }
      if (verification.status === 'unavailable') {
        return { status: 'provider-unavailable' };
      }
      if (verification.status === 'rejected') {
        return { status: 'rejected', reason: 'provider-credential-rejected' };
      }
      const providerIdentity = verification.identity;

      let authority: Awaited<ReturnType<AccountIdentityAuthorityWriter['readCurrentAuthority']>>;
      try {
        authority = await writer.readCurrentAuthority(providerIdentity.provider, providerIdentity.providerSubject);
      } catch {
        return { status: 'manual-required', reason: 'account-identity-d1-unavailable' };
      }
      switch (authority.status) {
        case 'trusted':
          return { status: 'trusted', providerIdentity, capability: authority.capability };
        case 'not-found':
          return { status: 'not-found', providerIdentity };
        case 'manual-required':
          return authority;
        case 'rejected':
          return authority;
      }
    },
    verifyAccountIssuerFrame(frame, nowMs = Date.now()) {
      return writer.verifyAccountIssuerFrame(frame, nowMs);
    },
  };
}

export function createAccountIdentityAuthorityCaller(
  env: Pick<Env, 'ACCOUNT_IDENTITY_D1'>
): AccountIdentityAuthorityCaller {
  return createCaller(createAccountIdentityAuthorityWriter(env.ACCOUNT_IDENTITY_D1));
}
