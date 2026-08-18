import type { Env } from '../env.js';
import {
  createAccountIdentityAuthorityWriter,
  type AccountIdentityAuthorityRevokeResult,
  type AccountIdentityAuthorityWriteResult,
  type AccountOwnedAuthorityProducer,
  type AccountIdentityAuthorityWriter,
} from '../storage/account-identity-authority-writer.js';
import type {
  AccountIdentityProvider,
  ProviderVerificationPort,
  VerifiedAccountIdentityAuthorityCapability,
  VerifiedProviderIdentity,
} from './verifier.js';

export type VerifiedProviderAuthorityResult =
  | {
      status: 'trusted';
      providerIdentity: VerifiedProviderIdentity;
      capability: VerifiedAccountIdentityAuthorityCapability;
    }
  | { status: 'not-found'; providerIdentity: VerifiedProviderIdentity }
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

function createCaller(writer: AccountIdentityAuthorityWriter): AccountIdentityAuthorityCaller {
  return {
    async resolveVerifiedProviderAuthority(request, providerVerifier) {
      // This handoff resolves only provider identity against current durable
      // Account state. It does not mint action capability, controller lease,
      // step-up, or target selectors; those owned sources remain fail-closed.
      if (providerVerifier === undefined) {
        return { status: 'provider-unavailable' };
      }

      let providerIdentity: VerifiedProviderIdentity | null;
      try {
        providerIdentity = await providerVerifier.verify(request);
      } catch {
        providerIdentity = null;
      }
      if (providerIdentity === null) {
        return { status: 'provider-unavailable' };
      }

      const authority = await writer.readCurrentAuthority(providerIdentity.provider, providerIdentity.providerSubject);
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

    createCurrentAuthority(producer, provider, providerSubject) {
      return writer.createCurrentAuthority(producer, provider, providerSubject);
    },

    compareAndSwapCurrentAuthority(
      producer,
      provider,
      providerSubject,
      expectedAuthorityGeneration,
      expectedSessionGeneration,
      expectedSessionId
    ) {
      return writer.compareAndSwapCurrentAuthority(
        producer,
        provider,
        providerSubject,
        expectedAuthorityGeneration,
        expectedSessionGeneration,
        expectedSessionId
      );
    },

    revokeCurrentAuthority(
      provider,
      providerSubject,
      expectedAuthorityGeneration,
      expectedSessionGeneration,
      expectedSessionId
    ) {
      return writer.revokeCurrentAuthority(
        provider,
        providerSubject,
        expectedAuthorityGeneration,
        expectedSessionGeneration,
        expectedSessionId
      );
    },
  };
}

export function createAccountIdentityAuthorityCaller(
  env: Pick<Env, 'ACCOUNT_IDENTITY_D1'>
): AccountIdentityAuthorityCaller {
  return createCaller(createAccountIdentityAuthorityWriter(env.ACCOUNT_IDENTITY_D1));
}
