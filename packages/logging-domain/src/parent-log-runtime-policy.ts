/* generated from crates/logging-core/src/parent_log_runtime.rs */

import { isGeneratedDevOrTestEnvironment, isGeneratedLevelAtOrAbove } from './parent-log-runtime-level';

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
