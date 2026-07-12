/* generated from crates/parent-runtime-core/src/hosted_portal_distribution.rs */

import {
  generatedSearchParams,
  hostedPortalExpectedPaths,
  shouldRenderHostedPortalDistribution,
  generatedEnvironmentForPath,
  normalizePathPrefix,
  normalizePath,
} from './hosted-portal-distribution.generated-path';
import {
  generatedOpenActionLabel,
  generatedResolveCacheAgeMinutes,
  generatedResolveRequestedRelease,
} from './hosted-portal-distribution.generated-policy';

export type GeneratedHostedPortalEnvironment = 'preview' | 'staging' | 'production';
export type GeneratedHostedPortalAuthState = 'authenticated' | 'missing';
export type GeneratedHostedPortalCacheState = 'fresh' | 'stale';
export type GeneratedHostedPortalRouteState = 'matched' | 'wrong-route';

export type GeneratedHostedPortalLocation = {
  readonly origin: string;
  readonly hash?: string;
  readonly pathname: string;
  readonly search: string;
};

export type GeneratedHostedPortalEnv = {
  readonly [key: string]: unknown;
  readonly VITE_OCENTRA_PARENT_HOSTED_PORTAL_MODE?: string;
  readonly VITE_OCENTRA_PARENT_HOSTED_PORTAL_PATH_PREFIX?: string;
};

export type GeneratedHostedPortalDistributionState = {
  readonly authState: GeneratedHostedPortalAuthState;
  readonly cacheAgeMinutes: number;
  readonly cacheState: GeneratedHostedPortalCacheState;
  readonly controlsEnabled: boolean;
  readonly environment: GeneratedHostedPortalEnvironment;
  readonly expectedPaths: readonly string[];
  readonly openActionLabel: string;
  readonly productionClaimBlocked: boolean;
  readonly requestedPath: string;
  readonly requestedRelease: GeneratedHostedPortalEnvironment;
  readonly routeState: GeneratedHostedPortalRouteState;
};

export function generatedResolveHostedPortalDistributionState(
  location: GeneratedHostedPortalLocation,
  env: GeneratedHostedPortalEnv,
  defaultNowMinutes = 4
): GeneratedHostedPortalDistributionState | null {
  if (!shouldRenderHostedPortalDistribution(location, env)) {
    return null;
  }

  const prefix = normalizePathPrefix(env.VITE_OCENTRA_PARENT_HOSTED_PORTAL_PATH_PREFIX);
  const expectedPaths = hostedPortalExpectedPaths(prefix);
  const requestedPath = normalizePath(location.pathname);
  const params = generatedSearchParams(location.search);
  const environment = generatedEnvironmentForPath(requestedPath, prefix);
  const routeState: GeneratedHostedPortalRouteState = environment === null ? 'wrong-route' : 'matched';
  const authState: GeneratedHostedPortalAuthState = params.get('auth') === 'missing' ? 'missing' : 'authenticated';
  const cacheState: GeneratedHostedPortalCacheState = params.get('cache') === 'stale' ? 'stale' : 'fresh';
  const requestedRelease = generatedResolveRequestedRelease(params, environment);
  const cacheAgeMinutes = generatedResolveCacheAgeMinutes(params, cacheState, defaultNowMinutes);
  const productionClaimBlocked =
    requestedRelease === 'production' && environment !== null && environment !== 'production';
  const controlsEnabled =
    routeState === 'matched' && authState === 'authenticated' && cacheState === 'fresh' && !productionClaimBlocked;
  const currentEnvironment = environment ?? 'preview';

  return {
    authState,
    cacheAgeMinutes,
    cacheState,
    controlsEnabled,
    environment: currentEnvironment,
    expectedPaths,
    openActionLabel: generatedOpenActionLabel(currentEnvironment),
    productionClaimBlocked,
    requestedPath,
    requestedRelease,
    routeState,
  };
}
