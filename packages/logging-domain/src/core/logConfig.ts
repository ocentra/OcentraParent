import path from 'node:path';
import { DevLogBridge, LogLevel, type LogLevel as LogLevelValue } from '../contracts';

export type BridgeMode = 'local' | 'tunnel' | 'disabled';

export interface ParentLogConfig {
  readonly enabled: boolean;
  readonly consoleEnabled: boolean;
  readonly storeEnabled: boolean;
  readonly minLevel: LogLevelValue;
  readonly nodeEnv: string;
  readonly testMode: boolean;
  readonly debugSources: readonly string[];
  readonly debugFiles: readonly string[];
  readonly debugRuns: readonly string[];
  readonly bridgeMode: BridgeMode;
  readonly bridgeUrl: string | null;
  readonly skipBridgeHealth: boolean;
}

const LEVEL_ORDER: readonly LogLevelValue[] = [
  LogLevel.Trace,
  LogLevel.Debug,
  LogLevel.Info,
  LogLevel.Warn,
  LogLevel.Error,
];

function defaultEnv(): NodeJS.ProcessEnv | Record<string, string | undefined> {
  if (typeof process === 'undefined' || process.env == null) {
    return {};
  }
  return process.env;
}

function parseBoolean(value: string | undefined, fallback: boolean): boolean {
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

function parseList(value: string | undefined): string[] {
  if (value == null || value.trim().length === 0) {
    return [];
  }

  return value
    .split(',')
    .map((entry) => entry.trim())
    .filter((entry) => entry.length > 0);
}

function parseLevel(value: string | undefined, fallback: LogLevelValue = LogLevel.Info): LogLevelValue {
  if (value == null || value.trim().length === 0) {
    return fallback;
  }

  const normalized = value.trim().toLowerCase();
  const match = LEVEL_ORDER.find((level) => level === normalized);
  return match ?? fallback;
}

function parseBridgeMode(value: string | undefined): BridgeMode {
  const normalized = value?.trim().toLowerCase();
  if (normalized === 'tunnel') {
    return 'tunnel';
  }
  if (normalized === 'disabled') {
    return 'disabled';
  }
  return 'local';
}

export function normalizeDebugPath(value: string): string {
  return path.normalize(value).replace(/\\/g, '/').toLowerCase();
}

export function levelWeight(level: LogLevelValue): number {
  const weight = LEVEL_ORDER.indexOf(level);
  return weight >= 0 ? weight : LEVEL_ORDER.indexOf(LogLevel.Info);
}

export function isLevelAtOrAbove(level: LogLevelValue, minLevel: LogLevelValue): boolean {
  return levelWeight(level) >= levelWeight(minLevel);
}

export function createParentLogConfig(
  env: NodeJS.ProcessEnv | Record<string, string | undefined> = defaultEnv()
): ParentLogConfig {
  const nodeEnv = env['NODE_ENV']?.trim().toLowerCase() ?? 'development';
  const testMode =
    parseBoolean(env['OCENTRA_PARENT_TEST_MODE'], false) ||
    parseBoolean(env['VITEST'], false) ||
    nodeEnv === 'test';
  const bridgeMode = parseBridgeMode(env['OCENTRA_PARENT_LOG_BRIDGE_MODE']);
  const configuredBridgeUrl = env['OCENTRA_PARENT_LOG_BRIDGE_URL']?.trim();

  return {
    enabled: parseBoolean(env['OCENTRA_PARENT_LOG_ENABLED'], true),
    consoleEnabled: parseBoolean(env['OCENTRA_PARENT_LOG_CONSOLE'], true),
    storeEnabled: parseBoolean(env['OCENTRA_PARENT_LOG_STORE'], true),
    minLevel: parseLevel(env['OCENTRA_PARENT_LOG_LEVEL'], LogLevel.Info),
    nodeEnv,
    testMode,
    debugSources: parseList(env['OCENTRA_PARENT_DEBUG_SOURCES']),
    debugFiles: parseList(env['OCENTRA_PARENT_DEBUG_FILES']).map(normalizeDebugPath),
    debugRuns: parseList(env['OCENTRA_PARENT_DEBUG_RUNS']),
    bridgeMode,
    bridgeUrl:
      bridgeMode === 'disabled'
        ? null
        : configuredBridgeUrl != null && configuredBridgeUrl.length > 0
          ? configuredBridgeUrl
          : DevLogBridge.DefaultUrl,
    skipBridgeHealth: parseBoolean(env['OCENTRA_PARENT_LOG_BRIDGE_SKIP_HEALTH'], false),
  };
}

export function isDevOrTestEnvironment(config: ParentLogConfig): boolean {
  return config.testMode || config.nodeEnv !== 'production';
}
