import type { GeneratedHostedPortalEnvironment } from './hosted-portal-distribution.generated';

const DEFAULT_STALE_CACHE_AGE_MINUTES = 180;

const GENERATED_HOSTED_PORTAL_RELEASE_LABELS: Record<GeneratedHostedPortalEnvironment, string> = {
  preview: 'Open parent portal preview',
  staging: 'Open parent portal staging',
  production: 'Open parent portal production',
};

const GENERATED_HOSTED_PORTAL_RELEASE_LOOKUP: Record<string, GeneratedHostedPortalEnvironment | undefined> = {
  preview: 'preview',
  staging: 'staging',
  production: 'production',
};

export function generatedResolveRequestedRelease(
  params: { get(name: string): string | null },
  environment: GeneratedHostedPortalEnvironment | null
): GeneratedHostedPortalEnvironment {
  const queryValue = params.get('release');
  const requestedRelease = queryValue === null ? null : (GENERATED_HOSTED_PORTAL_RELEASE_LOOKUP[queryValue] ?? null);
  return requestedRelease ?? environment ?? 'preview';
}

export function generatedResolveCacheAgeMinutes(
  params: { get(name: string): string | null },
  cacheState: 'fresh' | 'stale',
  defaultFreshAgeMinutes: number
): number {
  const value = Number(params.get('cacheAgeMinutes'));
  if (Number.isInteger(value) && value >= 0) {
    return value;
  }
  return cacheState === 'stale' ? DEFAULT_STALE_CACHE_AGE_MINUTES : defaultFreshAgeMinutes;
}

export function generatedOpenActionLabel(environment: GeneratedHostedPortalEnvironment): string {
  return GENERATED_HOSTED_PORTAL_RELEASE_LABELS[environment];
}
