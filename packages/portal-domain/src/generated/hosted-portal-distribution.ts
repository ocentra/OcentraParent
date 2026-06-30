/* generated from crates/parent-runtime-core/src/hosted_portal_distribution.rs */

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

type GeneratedHostedPortalSearchParams = {
  get(name: string): string | null;
};

const DEFAULT_CACHE_AGE_MINUTES = 4;
const DEFAULT_STALE_CACHE_AGE_MINUTES = 180;
const DEFAULT_PATH_PREFIX = '';

export function generatedResolveHostedPortalDistributionState(
  location: GeneratedHostedPortalLocation,
  env: GeneratedHostedPortalEnv,
  defaultNowMinutes = DEFAULT_CACHE_AGE_MINUTES
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
  const productionClaimBlocked = requestedRelease === 'production' && environment !== null && environment !== 'production';
  const controlsEnabled = routeState === 'matched' && authState === 'authenticated' && cacheState === 'fresh' && !productionClaimBlocked;
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

function shouldRenderHostedPortalDistribution(
  location: GeneratedHostedPortalLocation,
  env: GeneratedHostedPortalEnv
): boolean {
  if (env.VITE_OCENTRA_PARENT_HOSTED_PORTAL_MODE?.trim() === 'hosted') {
    return true;
  }

  return normalizePath(location.pathname) !== '/';
}

function generatedSearchParams(search: string): GeneratedHostedPortalSearchParams {
  const entries = new Map<string, string>();
  const trimmed = search.startsWith('?') ? search.slice(1) : search;
  if (trimmed !== '') {
    for (const segment of trimmed.split('&')) {
      if (segment === '') {
        continue;
      }
      const separatorIndex = segment.indexOf('=');
      const rawKey = separatorIndex >= 0 ? segment.slice(0, separatorIndex) : segment;
      const rawValue = separatorIndex >= 0 ? segment.slice(separatorIndex + 1) : '';
      entries.set(decodeURIComponent(rawKey), decodeURIComponent(rawValue));
    }
  }

  return {
    get(name: string): string | null {
      return entries.get(name) ?? null;
    },
  };
}

function hostedPortalExpectedPaths(prefix: string): readonly string[] {
  return ['preview', 'staging', 'production'].map((environment) => normalizePath(`${prefix}/${environment}`));
}

function generatedEnvironmentForPath(
  pathname: string,
  prefix: string
): GeneratedHostedPortalEnvironment | null {
  const pathMap = new Map<GeneratedHostedPortalEnvironment, string>([
    ['preview', normalizePath(`${prefix}/preview`)],
    ['staging', normalizePath(`${prefix}/staging`)],
    ['production', normalizePath(`${prefix}/production`)],
  ]);

  for (const [environment, expectedPath] of pathMap) {
    if (pathname === expectedPath) {
      return environment;
    }
  }

  return null;
}

function normalizePathPrefix(value: string | undefined): string {
  const trimmed = value?.trim();
  if (!trimmed) {
    return DEFAULT_PATH_PREFIX;
  }
  return normalizePath(trimmed);
}

function normalizePath(value: string): string {
  const trimmed = value.trim();
  if (trimmed === '' || trimmed === '/') {
    return '/';
  }
  const prefixed = trimmed.startsWith('/') ? trimmed : `/${trimmed}`;
  return prefixed.replace(/\/+$/u, '');
}

function generatedResolveRequestedRelease(
  params: GeneratedHostedPortalSearchParams,
  environment: GeneratedHostedPortalEnvironment | null
): GeneratedHostedPortalEnvironment {
  const queryValue = params.get('release');
  if (queryValue === 'preview' || queryValue === 'staging' || queryValue === 'production') {
    return queryValue;
  }
  return environment ?? 'preview';
}

function generatedResolveCacheAgeMinutes(
  params: GeneratedHostedPortalSearchParams,
  cacheState: GeneratedHostedPortalCacheState,
  defaultFreshAgeMinutes: number
): number {
  const value = Number(params.get('cacheAgeMinutes'));
  if (Number.isInteger(value) && value >= 0) {
    return value;
  }
  return cacheState === 'stale' ? DEFAULT_STALE_CACHE_AGE_MINUTES : defaultFreshAgeMinutes;
}

function generatedOpenActionLabel(environment: GeneratedHostedPortalEnvironment): string {
  if (environment === 'preview') {
    return 'Open parent portal preview';
  }
  if (environment === 'staging') {
    return 'Open parent portal staging';
  }
  return 'Open parent portal production';
}
