#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HostedPortalEnvironment {
    Preview,
    Staging,
    Production,
}

impl HostedPortalEnvironment {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Preview => "preview",
            Self::Staging => "staging",
            Self::Production => "production",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HostedPortalAuthState {
    Authenticated,
    Missing,
}

impl HostedPortalAuthState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Authenticated => "authenticated",
            Self::Missing => "missing",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HostedPortalCacheState {
    Fresh,
    Stale,
}

impl HostedPortalCacheState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::Stale => "stale",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HostedPortalRouteState {
    Matched,
    WrongRoute,
}

impl HostedPortalRouteState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Matched => "matched",
            Self::WrongRoute => "wrong-route",
        }
    }
}

pub fn hosted_portal_distribution_typescript() -> String {
    let mut output = hosted_portal_distribution_typescript_types();
    output.push_str(
        "export function generatedResolveHostedPortalDistributionState(\n",
    );
    output.push_str("  location: GeneratedHostedPortalLocation,\n");
    output.push_str("  env: GeneratedHostedPortalEnv,\n");
    output.push_str("  defaultNowMinutes = DEFAULT_CACHE_AGE_MINUTES\n");
    output.push_str("): GeneratedHostedPortalDistributionState | null {\n");
    output.push_str("  if (!shouldRenderHostedPortalDistribution(location, env)) {\n");
    output.push_str("    return null;\n");
    output.push_str("  }\n\n");
    output.push_str(
        "  const prefix = normalizePathPrefix(env.VITE_OCENTRA_PARENT_HOSTED_PORTAL_PATH_PREFIX);\n",
    );
    output.push_str("  const expectedPaths = hostedPortalExpectedPaths(prefix);\n");
    output.push_str("  const requestedPath = normalizePath(location.pathname);\n");
    output.push_str("  const params = generatedSearchParams(location.search);\n");
    output.push_str("  const environment = generatedEnvironmentForPath(requestedPath, prefix);\n");
    output.push_str(
        "  const routeState: GeneratedHostedPortalRouteState = environment === null ? 'wrong-route' : 'matched';\n",
    );
    output.push_str("  const authState: GeneratedHostedPortalAuthState = params.get('auth') === 'missing' ? 'missing' : 'authenticated';\n");
    output.push_str("  const cacheState: GeneratedHostedPortalCacheState = params.get('cache') === 'stale' ? 'stale' : 'fresh';\n");
    output.push_str(
        "  const requestedRelease = generatedResolveRequestedRelease(params, environment);\n",
    );
    output.push_str(
        "  const cacheAgeMinutes = generatedResolveCacheAgeMinutes(params, cacheState, defaultNowMinutes);\n",
    );
    output.push_str("  const productionClaimBlocked = requestedRelease === 'production' && environment !== null && environment !== 'production';\n");
    output.push_str("  const controlsEnabled = routeState === 'matched' && authState === 'authenticated' && cacheState === 'fresh' && !productionClaimBlocked;\n");
    output.push_str("  const currentEnvironment = environment ?? 'preview';\n\n");
    output.push_str("  return {\n");
    output.push_str("    authState,\n");
    output.push_str("    cacheAgeMinutes,\n");
    output.push_str("    cacheState,\n");
    output.push_str("    controlsEnabled,\n");
    output.push_str("    environment: currentEnvironment,\n");
    output.push_str("    expectedPaths,\n");
    output.push_str(
        "    openActionLabel: generatedOpenActionLabel(currentEnvironment),\n",
    );
    output.push_str("    productionClaimBlocked,\n");
    output.push_str("    requestedPath,\n");
    output.push_str("    requestedRelease,\n");
    output.push_str("    routeState,\n");
    output.push_str("  };\n");
    output.push_str("}\n\n");
    output.push_str(&hosted_portal_distribution_runtime_typescript());
    output.push_str(&hosted_portal_distribution_resolution_typescript());
    output
}

fn hosted_portal_distribution_typescript_types() -> String {
    let mut output = String::from(
        "/* generated from crates/parent-runtime-core/src/hosted_portal_distribution.rs */\n\n",
    );
    output.push_str(
        "export type GeneratedHostedPortalEnvironment = 'preview' | 'staging' | 'production';\n",
    );
    output.push_str(
        "export type GeneratedHostedPortalAuthState = 'authenticated' | 'missing';\n",
    );
    output.push_str("export type GeneratedHostedPortalCacheState = 'fresh' | 'stale';\n");
    output.push_str(
        "export type GeneratedHostedPortalRouteState = 'matched' | 'wrong-route';\n\n",
    );
    output.push_str("export type GeneratedHostedPortalLocation = {\n");
    output.push_str("  readonly origin: string;\n");
    output.push_str("  readonly hash?: string;\n");
    output.push_str("  readonly pathname: string;\n");
    output.push_str("  readonly search: string;\n");
    output.push_str("};\n\n");
    output.push_str("export type GeneratedHostedPortalEnv = {\n");
    output.push_str("  readonly [key: string]: unknown;\n");
    output.push_str("  readonly VITE_OCENTRA_PARENT_HOSTED_PORTAL_MODE?: string;\n");
    output.push_str("  readonly VITE_OCENTRA_PARENT_HOSTED_PORTAL_PATH_PREFIX?: string;\n");
    output.push_str("};\n\n");
    output.push_str("export type GeneratedHostedPortalDistributionState = {\n");
    output.push_str("  readonly authState: GeneratedHostedPortalAuthState;\n");
    output.push_str("  readonly cacheAgeMinutes: number;\n");
    output.push_str("  readonly cacheState: GeneratedHostedPortalCacheState;\n");
    output.push_str("  readonly controlsEnabled: boolean;\n");
    output.push_str("  readonly environment: GeneratedHostedPortalEnvironment;\n");
    output.push_str("  readonly expectedPaths: readonly string[];\n");
    output.push_str("  readonly openActionLabel: string;\n");
    output.push_str("  readonly productionClaimBlocked: boolean;\n");
    output.push_str("  readonly requestedPath: string;\n");
    output.push_str("  readonly requestedRelease: GeneratedHostedPortalEnvironment;\n");
    output.push_str("  readonly routeState: GeneratedHostedPortalRouteState;\n");
    output.push_str("};\n\n");
    output.push_str("type GeneratedHostedPortalSearchParams = {\n");
    output.push_str("  get(name: string): string | null;\n");
    output.push_str("};\n\n");
    output.push_str("const DEFAULT_CACHE_AGE_MINUTES = 4;\n");
    output.push_str("const DEFAULT_STALE_CACHE_AGE_MINUTES = 180;\n");
    output.push_str("const DEFAULT_PATH_PREFIX = '';\n\n");
    output
}

fn hosted_portal_distribution_runtime_typescript() -> String {
    let mut output = String::new();
    output.push_str(
        "function shouldRenderHostedPortalDistribution(\n  location: GeneratedHostedPortalLocation,\n  env: GeneratedHostedPortalEnv\n): boolean {\n",
    );
    output.push_str(
        "  if (env.VITE_OCENTRA_PARENT_HOSTED_PORTAL_MODE?.trim() === 'hosted') {\n",
    );
    output.push_str("    return true;\n");
    output.push_str("  }\n\n");
    output.push_str("  return normalizePath(location.pathname) !== '/';\n");
    output.push_str("}\n\n");
    output.push_str(
        "function generatedSearchParams(search: string): GeneratedHostedPortalSearchParams {\n",
    );
    output.push_str("  const entries = new Map<string, string>();\n");
    output.push_str("  const trimmed = search.startsWith('?') ? search.slice(1) : search;\n");
    output.push_str("  if (trimmed !== '') {\n");
    output.push_str("    for (const segment of trimmed.split('&')) {\n");
    output.push_str("      if (segment === '') {\n");
    output.push_str("        continue;\n");
    output.push_str("      }\n");
    output.push_str("      const separatorIndex = segment.indexOf('=');\n");
    output.push_str("      const rawKey = separatorIndex >= 0 ? segment.slice(0, separatorIndex) : segment;\n");
    output.push_str("      const rawValue = separatorIndex >= 0 ? segment.slice(separatorIndex + 1) : '';\n");
    output.push_str("      entries.set(decodeURIComponent(rawKey), decodeURIComponent(rawValue));\n");
    output.push_str("    }\n");
    output.push_str("  }\n\n");
    output.push_str("  return {\n");
    output.push_str("    get(name: string): string | null {\n");
    output.push_str("      return entries.get(name) ?? null;\n");
    output.push_str("    },\n");
    output.push_str("  };\n");
    output.push_str("}\n\n");
    output.push_str(
        "function hostedPortalExpectedPaths(prefix: string): readonly string[] {\n",
    );
    output.push_str(
        "  return ['preview', 'staging', 'production'].map((environment) => normalizePath(`${prefix}/${environment}`));\n",
    );
    output.push_str("}\n\n");
    output.push_str(
        "function generatedEnvironmentForPath(\n  pathname: string,\n  prefix: string\n): GeneratedHostedPortalEnvironment | null {\n",
    );
    output.push_str("  const pathMap = new Map<GeneratedHostedPortalEnvironment, string>([\n");
    output.push_str("    ['preview', normalizePath(`${prefix}/preview`)],\n");
    output.push_str("    ['staging', normalizePath(`${prefix}/staging`)],\n");
    output.push_str("    ['production', normalizePath(`${prefix}/production`)],\n");
    output.push_str("  ]);\n\n");
    output.push_str("  for (const [environment, expectedPath] of pathMap) {\n");
    output.push_str("    if (pathname === expectedPath) {\n");
    output.push_str("      return environment;\n");
    output.push_str("    }\n");
    output.push_str("  }\n\n");
    output.push_str("  return null;\n");
    output.push_str("}\n\n");
    output.push_str("function normalizePathPrefix(value: string | undefined): string {\n");
    output.push_str("  const trimmed = value?.trim();\n");
    output.push_str("  if (!trimmed) {\n");
    output.push_str("    return DEFAULT_PATH_PREFIX;\n");
    output.push_str("  }\n");
    output.push_str("  return normalizePath(trimmed);\n");
    output.push_str("}\n\n");
    output.push_str("function normalizePath(value: string): string {\n");
    output.push_str("  const trimmed = value.trim();\n");
    output.push_str("  if (trimmed === '' || trimmed === '/') {\n");
    output.push_str("    return '/';\n");
    output.push_str("  }\n");
    output.push_str("  const prefixed = trimmed.startsWith('/') ? trimmed : `/${trimmed}`;\n");
    output.push_str("  return prefixed.replace(/\\/+$/u, '');\n");
    output.push_str("}\n");
    output
}

fn hosted_portal_distribution_resolution_typescript() -> String {
    let mut output = String::new();
    output.push_str(
        "function generatedResolveRequestedRelease(\n  params: GeneratedHostedPortalSearchParams,\n  environment: GeneratedHostedPortalEnvironment | null\n): GeneratedHostedPortalEnvironment {\n",
    );
    output.push_str("  const queryValue = params.get('release');\n");
    output.push_str(
        "  if (queryValue === 'preview' || queryValue === 'staging' || queryValue === 'production') {\n",
    );
    output.push_str("    return queryValue;\n");
    output.push_str("  }\n");
    output.push_str("  return environment ?? 'preview';\n");
    output.push_str("}\n\n");
    output.push_str(
        "function generatedResolveCacheAgeMinutes(\n  params: GeneratedHostedPortalSearchParams,\n  cacheState: GeneratedHostedPortalCacheState,\n  defaultFreshAgeMinutes: number\n): number {\n",
    );
    output.push_str("  const value = Number(params.get('cacheAgeMinutes'));\n");
    output.push_str("  if (Number.isInteger(value) && value >= 0) {\n");
    output.push_str("    return value;\n");
    output.push_str("  }\n");
    output.push_str(
        "  return cacheState === 'stale' ? DEFAULT_STALE_CACHE_AGE_MINUTES : defaultFreshAgeMinutes;\n",
    );
    output.push_str("}\n\n");
    output.push_str(
        "function generatedOpenActionLabel(environment: GeneratedHostedPortalEnvironment): string {\n",
    );
    output.push_str("  if (environment === 'preview') {\n");
    output.push_str("    return 'Open parent portal preview';\n");
    output.push_str("  }\n");
    output.push_str("  if (environment === 'staging') {\n");
    output.push_str("    return 'Open parent portal staging';\n");
    output.push_str("  }\n");
    output.push_str("  return 'Open parent portal production';\n");
    output.push_str("}\n");
    output
}
