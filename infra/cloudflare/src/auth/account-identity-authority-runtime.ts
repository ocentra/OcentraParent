import { Logger } from '@ocentra-parent/logging-domain/core/logger';
import { getStackTrace } from '@ocentra-parent/logging-domain/core/stackTrace';
import type { Env } from '../env.js';
import {
  createAccountIdentityAuthorityCaller,
  type AccountIdentityAuthorityCaller,
  type VerifiedProviderAuthorityResult,
} from './account-identity-authority-caller.js';
import type { ProviderVerificationPort } from './verifier.js';
import type { AccountIdentityProvider } from '@ocentra-parent/schema-domain/account-identity-authority';

const log = Logger.instance;
log.register(import.meta.url);

export const ACCOUNT_IDENTITY_AUTHORITY_SOURCE_UNAVAILABLE = 'account-identity-authority-source-unavailable' as const;

export type AccountIdentityAuthorityMutationResult = {
  status: 'manual-required';
  reason: typeof ACCOUNT_IDENTITY_AUTHORITY_SOURCE_UNAVAILABLE;
};

export interface AccountIdentityAuthorityRuntime {
  resolveVerifiedProviderAuthority(
    request: Request,
    providerVerifier: ProviderVerificationPort | undefined
  ): Promise<VerifiedProviderAuthorityResult>;
  createCurrentAuthority(
    provider: AccountIdentityProvider,
    providerSubject: string
  ): Promise<AccountIdentityAuthorityMutationResult>;
  compareAndSwapCurrentAuthority(
    provider: AccountIdentityProvider,
    providerSubject: string,
    expectedAuthorityGeneration: number,
    expectedSessionGeneration: number,
    expectedSessionId: string
  ): Promise<AccountIdentityAuthorityMutationResult>;
  revokeCurrentAuthority(
    provider: AccountIdentityProvider,
    providerSubject: string,
    expectedAuthorityGeneration: number,
    expectedSessionGeneration: number,
    expectedSessionId: string
  ): Promise<AccountIdentityAuthorityMutationResult>;
}

function mutationSourceUnavailable(): AccountIdentityAuthorityMutationResult {
  log.logWarn(
    'account identity authority mutation blocked: Account-owned producer transport is unavailable',
    getStackTrace(),
    {
      owner: 'cloudflare-wp06-account-identity-authority-runtime',
      boundary: 'account-owned-authority-producer',
      result: 'blocked',
      noClaimReason: ACCOUNT_IDENTITY_AUTHORITY_SOURCE_UNAVAILABLE,
      redactionState: 'redacted',
    },
    false
  );
  return { status: 'manual-required', reason: ACCOUNT_IDENTITY_AUTHORITY_SOURCE_UNAVAILABLE };
}

export function createAccountIdentityAuthorityRuntime(
  env: Pick<Env, 'ACCOUNT_IDENTITY_D1'>
): AccountIdentityAuthorityRuntime {
  const caller: AccountIdentityAuthorityCaller = createAccountIdentityAuthorityCaller(env);
  return Object.freeze({
    resolveVerifiedProviderAuthority(
      request: Request,
      providerVerifier: ProviderVerificationPort | undefined
    ): Promise<VerifiedProviderAuthorityResult> {
      return caller.resolveVerifiedProviderAuthority(request, providerVerifier);
    },
    createCurrentAuthority(
      _provider: AccountIdentityProvider,
      _providerSubject: string
    ): Promise<AccountIdentityAuthorityMutationResult> {
      return Promise.resolve(mutationSourceUnavailable());
    },
    compareAndSwapCurrentAuthority(
      _provider: AccountIdentityProvider,
      _providerSubject: string,
      _expectedAuthorityGeneration: number,
      _expectedSessionGeneration: number,
      _expectedSessionId: string
    ): Promise<AccountIdentityAuthorityMutationResult> {
      return Promise.resolve(mutationSourceUnavailable());
    },
    revokeCurrentAuthority(
      _provider: AccountIdentityProvider,
      _providerSubject: string,
      _expectedAuthorityGeneration: number,
      _expectedSessionGeneration: number,
      _expectedSessionId: string
    ): Promise<AccountIdentityAuthorityMutationResult> {
      return Promise.resolve(mutationSourceUnavailable());
    },
  });
}
