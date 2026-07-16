import {
  GeneratedDevLogBridge as DevLogBridge,
  type GeneratedLogLevel as LogLevelValue,
} from '../generated-logging-contracts';
import {
  buildGeneratedParentLogConfig,
  isGeneratedDevOrTestEnvironment,
  isGeneratedLevelAtOrAbove,
  normalizeGeneratedDebugPath,
} from '../parent-log-runtime';

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
  return {
    ...config,
    minLevel: config.minLevel as LogLevelValue,
    bridgeMode: config.bridgeMode as BridgeMode,
  };
}

export function isDevOrTestEnvironment(config: ParentLogConfig): boolean {
  return isGeneratedDevOrTestEnvironment(config.nodeEnv, config.testMode);
}
