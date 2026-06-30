const DEFAULT_BRIDGE_MODE: &str = "local";
const DEFAULT_NODE_ENV: &str = "development";
const STALE_RUN_INFO_WARNING: &str = "previous run info was stale and has been replaced";

pub fn parse_boolean(value: Option<&str>, fallback: bool) -> bool {
    let Some(value) = value.map(str::trim) else {
        return fallback;
    };
    let normalized = value.to_ascii_lowercase();
    match normalized.as_str() {
        "true" | "1" | "yes" | "on" => true,
        "false" | "0" | "no" | "off" => false,
        _ => fallback,
    }
}

pub fn parse_list(value: Option<&str>) -> Vec<String> {
    value
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|value| {
            value
                .split(',')
                .map(str::trim)
                .filter(|entry| !entry.is_empty())
                .map(ToOwned::to_owned)
                .collect()
        })
        .unwrap_or_default()
}

pub fn parse_level(value: Option<&str>, fallback: &str) -> String {
    let Some(value) = value.map(str::trim).filter(|value| !value.is_empty()) else {
        return fallback.to_owned();
    };

    match value.to_ascii_lowercase().as_str() {
        "trace" | "debug" | "info" | "warn" | "error" => value.to_ascii_lowercase(),
        _ => fallback.to_owned(),
    }
}

pub fn parse_bridge_mode(value: Option<&str>) -> &'static str {
    match value.map(str::trim).map(str::to_ascii_lowercase) {
        Some(value) if value == "tunnel" => "tunnel",
        Some(value) if value == "disabled" => "disabled",
        _ => DEFAULT_BRIDGE_MODE,
    }
}

pub fn normalize_debug_path(value: &str) -> String {
    value.replace('\\', "/").to_ascii_lowercase()
}

pub fn level_weight(level: &str) -> usize {
    match level {
        "trace" => 0,
        "debug" => 1,
        "info" => 2,
        "warn" => 3,
        "error" => 4,
        _ => 2,
    }
}

pub fn is_level_at_or_above(level: &str, min_level: &str) -> bool {
    level_weight(level) >= level_weight(min_level)
}

pub fn is_dev_or_test_environment(
    node_env: Option<&str>,
    test_mode: bool,
    vitest_mode: bool,
) -> bool {
    test_mode
        || vitest_mode
        || node_env
            .unwrap_or(DEFAULT_NODE_ENV)
            .trim()
            .eq_ignore_ascii_case("test")
}

fn normalize_source(source: Option<&str>) -> Option<String> {
    source
        .map(str::trim)
        .filter(|source| !source.is_empty())
        .map(str::to_ascii_lowercase)
}

pub fn matches_debug_selection(
    debug_sources: &[String],
    debug_files: &[String],
    debug_runs: &[String],
    source: Option<&str>,
    file_path: Option<&str>,
    run_id: Option<&str>,
    request_debug_sources: &[String],
) -> bool {
    if let Some(normalized_source) = normalize_source(source) {
        if debug_sources
            .iter()
            .any(|entry| entry == &normalized_source)
            || request_debug_sources
                .iter()
                .map(|entry| entry.trim().to_ascii_lowercase())
                .any(|entry| entry == normalized_source)
        {
            return true;
        }
    }

    if let Some(file_path) = file_path.map(str::trim).filter(|file| !file.is_empty()) {
        let normalized_file = normalize_debug_path(file_path);
        if debug_files
            .iter()
            .any(|entry| normalized_file.contains(entry))
        {
            return true;
        }
    }

    if let Some(run_id) = run_id.map(str::trim).filter(|run_id| !run_id.is_empty()) {
        if debug_runs.iter().any(|entry| entry == run_id) {
            return true;
        }
    }

    false
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ParentLogRuntimeDecisionInput<'a> {
    pub enabled: bool,
    pub sink_enabled: bool,
    pub node_env: Option<&'a str>,
    pub test_mode: bool,
    pub vitest_mode: bool,
    pub level: &'a str,
    pub min_level: &'a str,
    pub debug_selected: bool,
}

pub fn should_log_to_console(input: &ParentLogRuntimeDecisionInput<'_>) -> bool {
    if !input.sink_enabled {
        return false;
    }
    if input.level == "error" || input.level == "warn" {
        return true;
    }
    if !input.enabled {
        return false;
    }
    if input.debug_selected {
        return true;
    }
    is_dev_or_test_environment(input.node_env, input.test_mode, input.vitest_mode)
        && is_level_at_or_above(input.level, input.min_level)
}

pub fn should_store_log(input: &ParentLogRuntimeDecisionInput<'_>) -> bool {
    if input.level == "error" || input.level == "warn" {
        return true;
    }
    if !input.enabled || !input.sink_enabled {
        return false;
    }
    if input.debug_selected {
        return true;
    }
    is_dev_or_test_environment(input.node_env, input.test_mode, input.vitest_mode)
        && is_level_at_or_above(input.level, input.min_level)
}

pub fn normalize_bridge_endpoint(endpoint: &str) -> String {
    endpoint.trim_end_matches('/').to_owned()
}

pub fn resolve_bridge_route(method: &str, pathname: &str) -> &'static str {
    match (method, pathname) {
        ("GET", "/__health__") => "health",
        ("GET", "/__run_info__") => "run-info",
        ("POST", "/__run_started__") => "run-started",
        ("POST", "/__logs__") => "logs",
        ("GET", "/__flush__") | ("POST", "/__flush__") => "flush",
        _ => "not-found",
    }
}

pub fn stale_run_info_warning(
    run_id: Option<&str>,
    started_at: Option<u64>,
    now: u64,
) -> Option<String> {
    if run_id.is_none() || started_at.is_none() {
        return None;
    }
    if now.saturating_sub(started_at.unwrap_or_default()) > 5 * 60 * 1000 {
        return Some(STALE_RUN_INFO_WARNING.to_owned());
    }
    None
}

pub fn has_stale_run_info_conflict(
    active_run_id: Option<&str>,
    active_scope: Option<&str>,
    entries: &[(Option<&str>, &str)],
) -> bool {
    let Some(active_run_id) = active_run_id else {
        return false;
    };
    let Some(active_scope) = active_scope else {
        return false;
    };

    entries.iter().any(|(entry_run_id, entry_consumer)| {
        *entry_consumer == active_scope && entry_run_id.unwrap_or("") != active_run_id
    })
}

pub fn parent_log_runtime_typescript() -> &'static str {
    PARENT_LOG_RUNTIME_TYPESCRIPT
}

const PARENT_LOG_RUNTIME_TYPESCRIPT: &str = r#"/* generated from crates/logging-core/src/parent_log_runtime.rs */

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
"#;
