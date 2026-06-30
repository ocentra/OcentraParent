/* generated from crates/logging-core/src/parent_log_runtime.rs */

export function parseGeneratedBoolean(value: string | undefined, fallback: boolean): boolean {
  if (value == null) {
    return fallback;
  }
  const normalized = value.trim().toLowerCase();
  if (normalized === 'true' || normalized === '1' || normalized === 'yes' || normalized === 'on') {
    return true;
  }
  if (normalized === 'false' || normalized === '0' || normalized === 'no' || normalized === 'off') {
    return false;
  }
  return fallback;
}

export function parseGeneratedList(value: string | undefined): string[] {
  if (value == null || value.trim().length === 0) {
    return [];
  }
  return value.split(',').map((entry) => entry.trim()).filter((entry) => entry.length > 0);
}

export function parseGeneratedLevel(value: string | undefined, fallback = 'info'): string {
  if (value == null || value.trim().length === 0) {
    return fallback;
  }
  const normalized = value.trim().toLowerCase();
  return ['trace', 'debug', 'info', 'warn', 'error'].includes(normalized) ? normalized : fallback;
}

export function parseGeneratedBridgeMode(value: string | undefined): 'local' | 'tunnel' | 'disabled' {
  const normalized = value?.trim().toLowerCase();
  if (normalized === 'tunnel') {
    return 'tunnel';
  }
  if (normalized === 'disabled') {
    return 'disabled';
  }
  return 'local';
}

export function normalizeGeneratedDebugPath(value: string): string {
  return value.replace(/\\/g, '/').toLowerCase();
}

function generatedLevelWeight(level: string): number {
  switch (level) {
    case 'trace':
      return 0;
    case 'debug':
      return 1;
    case 'warn':
      return 3;
    case 'error':
      return 4;
    case 'info':
    default:
      return 2;
  }
}

export function isGeneratedLevelAtOrAbove(level: string, minLevel: string): boolean {
  return generatedLevelWeight(level) >= generatedLevelWeight(minLevel);
}

export function isGeneratedDevOrTestEnvironment(nodeEnv: string, testMode: boolean): boolean {
  return testMode || nodeEnv === 'test';
}

function normalizeGeneratedSource(source: string | null | undefined): string | null {
  if (source == null || source.trim().length === 0) {
    return null;
  }
  return source.trim().toLowerCase();
}

export function matchesGeneratedDebugSelection(
  debugSources: readonly string[],
  debugFiles: readonly string[],
  debugRuns: readonly string[],
  source: string | null | undefined,
  filePath?: string | null,
  runId?: string | null,
  requestDebugSources?: readonly string[]
): boolean {
  const normalizedSource = normalizeGeneratedSource(source);
  if (
    normalizedSource != null &&
    (debugSources.includes(normalizedSource) ||
      requestDebugSources?.some((entry) => entry.trim().toLowerCase() === normalizedSource) === true)
  ) {
    return true;
  }

  if (filePath != null && filePath.trim().length > 0) {
    const normalizedFile = normalizeGeneratedDebugPath(filePath);
    if (debugFiles.some((entry) => normalizedFile.includes(entry))) {
      return true;
    }
  }

  if (runId != null && runId.trim().length > 0 && debugRuns.includes(runId.trim())) {
    return true;
  }

  return false;
}

export function shouldGeneratedLogToConsole(
  enabled: boolean,
  consoleEnabled: boolean,
  nodeEnv: string,
  testMode: boolean,
  level: string,
  minLevel: string,
  debugSelected: boolean
): boolean {
  if (!consoleEnabled) {
    return false;
  }
  if (level === 'error' || level === 'warn') {
    return true;
  }
  if (!enabled) {
    return false;
  }
  if (debugSelected) {
    return true;
  }
  return isGeneratedDevOrTestEnvironment(nodeEnv, testMode) && isGeneratedLevelAtOrAbove(level, minLevel);
}

export function shouldGeneratedStoreLog(
  enabled: boolean,
  storeEnabled: boolean,
  nodeEnv: string,
  testMode: boolean,
  level: string,
  minLevel: string,
  debugSelected: boolean
): boolean {
  if (level === 'error' || level === 'warn') {
    return true;
  }
  if (!enabled || !storeEnabled) {
    return false;
  }
  if (debugSelected) {
    return true;
  }
  return isGeneratedDevOrTestEnvironment(nodeEnv, testMode) && isGeneratedLevelAtOrAbove(level, minLevel);
}

export function buildGeneratedParentLogConfig(
  env: NodeJS.ProcessEnv | Record<string, string | undefined>,
  defaultBridgeUrl: string
) {
  const nodeEnv = env['NODE_ENV']?.trim().toLowerCase() ?? 'development';
  const testMode =
    parseGeneratedBoolean(env['OCENTRA_PARENT_TEST_MODE'], false) ||
    parseGeneratedBoolean(env['VITEST'], false) ||
    nodeEnv === 'test';
  const bridgeMode = parseGeneratedBridgeMode(env['OCENTRA_PARENT_LOG_BRIDGE_MODE']);
  const configuredBridgeUrl = env['OCENTRA_PARENT_LOG_BRIDGE_URL']?.trim();

  return {
    enabled: parseGeneratedBoolean(env['OCENTRA_PARENT_LOG_ENABLED'], true),
    consoleEnabled: parseGeneratedBoolean(env['OCENTRA_PARENT_LOG_CONSOLE'], true),
    storeEnabled: parseGeneratedBoolean(env['OCENTRA_PARENT_LOG_STORE'], true),
    minLevel: parseGeneratedLevel(env['OCENTRA_PARENT_LOG_LEVEL'], 'info'),
    nodeEnv,
    testMode,
    debugSources: parseGeneratedList(env['OCENTRA_PARENT_DEBUG_SOURCES']),
    debugFiles: parseGeneratedList(env['OCENTRA_PARENT_DEBUG_FILES']).map(normalizeGeneratedDebugPath),
    debugRuns: parseGeneratedList(env['OCENTRA_PARENT_DEBUG_RUNS']),
    bridgeMode,
    bridgeUrl:
      bridgeMode === 'disabled'
        ? null
        : configuredBridgeUrl != null && configuredBridgeUrl.length > 0
          ? configuredBridgeUrl
          : defaultBridgeUrl,
    skipBridgeHealth: parseGeneratedBoolean(env['OCENTRA_PARENT_LOG_BRIDGE_SKIP_HEALTH'], false),
  };
}

export function normalizeGeneratedBridgeEndpoint(endpoint: string): string {
  return endpoint.endsWith('/') ? endpoint.slice(0, -1) : endpoint;
}

export function resolveGeneratedBridgeRoute(method: string, pathname: string): 'health' | 'run-info' | 'run-started' | 'logs' | 'flush' | 'not-found' {
  switch (pathname) {
    case '/__health__':
      return method === 'GET' ? 'health' : 'not-found';
    case '/__run_info__':
      return method === 'GET' ? 'run-info' : 'not-found';
    case '/__run_started__':
      return method === 'POST' ? 'run-started' : 'not-found';
    case '/__logs__':
      return method === 'POST' ? 'logs' : 'not-found';
    case '/__flush__':
      return method === 'GET' || method === 'POST' ? 'flush' : 'not-found';
    default:
      return 'not-found';
  }
}

export function generatedStaleRunInfoWarning(runId: string | null, startedAt: number | null, now = Date.now()): string | null {
  if (runId == null || startedAt == null) {
    return null;
  }
  return now - startedAt > 5 * 60 * 1000 ? 'previous run info was stale and has been replaced' : null;
}

export function generatedHasRunInfoConflict(
  runInfo: { readonly runId: string | null; readonly scope: string | null },
  entries: readonly { readonly runId?: string | null; readonly consumer: string | null }[]
): boolean {
  return runInfo.runId != null &&
    runInfo.scope != null &&
    entries.some((entry) => entry.consumer === runInfo.scope && entry.runId !== runInfo.runId);
}

export function buildGeneratedRunStartedPayload(payload: {
  readonly runId: string;
  readonly runType?: string;
  readonly suiteType?: string | null;
  readonly scope?: string | null;
  readonly filePath?: string | null;
  readonly wipeAll?: boolean;
}) {
  return {
    runId: payload.runId,
    runType: payload.runType ?? 'single',
    suiteType: payload.suiteType ?? null,
    scope: payload.scope ?? null,
    filePath: payload.filePath ?? null,
    wipeAll: payload.wipeAll ?? false,
  };
}
