/* generated from crates/logging-core/src/parent_log_runtime.rs */

import {
  parseGeneratedBoolean,
  parseGeneratedBridgeMode,
  parseGeneratedLevel,
  parseGeneratedList,
} from './parent-log-runtime-parsing';

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
    debugFiles: parseGeneratedList(env['OCENTRA_PARENT_DEBUG_FILES']).map((entry) =>
      entry.replace(/\\/g, '/').toLowerCase()
    ),
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
