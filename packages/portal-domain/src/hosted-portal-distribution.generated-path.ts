import type {
  GeneratedHostedPortalEnv,
  GeneratedHostedPortalEnvironment,
  GeneratedHostedPortalLocation,
} from './hosted-portal-distribution.generated';

type GeneratedHostedPortalSearchParams = {
  get(name: string): string | null;
};

const DEFAULT_PATH_PREFIX = '';

export function shouldRenderHostedPortalDistribution(
  location: GeneratedHostedPortalLocation,
  env: GeneratedHostedPortalEnv
): boolean {
  return env.VITE_OCENTRA_PARENT_HOSTED_PORTAL_MODE?.trim() === 'hosted' || normalizePath(location.pathname) !== '/';
}

export function generatedSearchParams(search: string): GeneratedHostedPortalSearchParams {
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

export function hostedPortalExpectedPaths(prefix: string): readonly string[] {
  return ['preview', 'staging', 'production'].map((environment) => normalizePath(`${prefix}/${environment}`));
}

export function generatedEnvironmentForPath(pathname: string, prefix: string): GeneratedHostedPortalEnvironment | null {
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

export function normalizePathPrefix(value: string | undefined): string {
  const trimmed = value?.trim();
  return trimmed ? normalizePath(trimmed) : DEFAULT_PATH_PREFIX;
}

export function normalizePath(value: string): string {
  const trimmed = value.trim();
  if (trimmed === '' || trimmed === '/') {
    return '/';
  }
  const prefixed = trimmed.startsWith('/') ? trimmed : `/${trimmed}`;
  return trimTrailingSolidus(prefixed);
}

function trimTrailingSolidus(value: string): string {
  let end = value.length;
  while (end > 0 && value.charCodeAt(end - 1) === 47) {
    end -= 1;
  }
  return value.slice(0, end);
}
