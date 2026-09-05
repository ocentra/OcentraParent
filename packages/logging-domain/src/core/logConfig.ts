import {
  GeneratedDevLogBridge as DevLogBridge,
  GeneratedLogLevel as LogLevel,
  type GeneratedLogLevel as LogLevelValue,
} from '../generated-logging-contracts';
import {
  buildGeneratedParentLogConfig,
  isGeneratedDevOrTestEnvironment,
  isGeneratedLevelAtOrAbove,
  normalizeGeneratedDebugPath,
} from '../parent-log-runtime';
import { resolveBridgeMode, resolveBridgeUrl } from './bridgeLogConfig';

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

const BooleanValues = new Set(['true', '1', 'yes', 'on', 'false', '0', 'no', 'off']);
const LogLevels = new Set<LogLevelValue>([
  LogLevel.Trace,
  LogLevel.Debug,
  LogLevel.Info,
  LogLevel.Warn,
  LogLevel.Error,
]);

function isInvalidBoolean(value: string | undefined): boolean {
  return value != null && !BooleanValues.has(value.trim().toLowerCase());
}

function defaultEnv(): NodeJS.ProcessEnv | Record<string, string | undefined> {
  if (typeof process === 'undefined' || process.env == null) {
    return {};
  }
  return process.env;
}

export function normalizeDebugPath(value: string): string {
  return normalizeGeneratedDebugPath(value);
}

export function isLevelAtOrAbove(level: LogLevelValue, minLevel: LogLevelValue): boolean {
  return isGeneratedLevelAtOrAbove(level, minLevel);
}

export function createParentLogConfig(
  env: NodeJS.ProcessEnv | Record<string, string | undefined> = defaultEnv()
): ParentLogConfig {
  const config = buildGeneratedParentLogConfig(env, DevLogBridge.DefaultUrl);
  const bridgeMode = resolveBridgeMode(env['OCENTRA_PARENT_LOG_BRIDGE_MODE'], config.bridgeMode as BridgeMode);
  const configuredLevel = env['OCENTRA_PARENT_LOG_LEVEL']?.trim().toLowerCase();
  return {
    ...config,
    enabled: isInvalidBoolean(env['OCENTRA_PARENT_LOG_ENABLED']) ? false : config.enabled,
    consoleEnabled: isInvalidBoolean(env['OCENTRA_PARENT_LOG_CONSOLE']) ? false : config.consoleEnabled,
    storeEnabled: isInvalidBoolean(env['OCENTRA_PARENT_LOG_STORE']) ? false : config.storeEnabled,
    minLevel:
      configuredLevel != null && configuredLevel.length > 0 && !LogLevels.has(configuredLevel as LogLevelValue)
        ? LogLevel.Error
        : (config.minLevel as LogLevelValue),
    bridgeMode,
    bridgeUrl: resolveBridgeUrl(bridgeMode, env['OCENTRA_PARENT_LOG_BRIDGE_URL'], config.bridgeUrl),
    skipBridgeHealth: isInvalidBoolean(env['OCENTRA_PARENT_LOG_BRIDGE_SKIP_HEALTH']) ? false : config.skipBridgeHealth,
  };
}

export function isDevOrTestEnvironment(config: ParentLogConfig): boolean {
  return isGeneratedDevOrTestEnvironment(config.nodeEnv, config.testMode);
}
