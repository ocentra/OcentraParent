/* generated from crates/logging-core/src/parent_log_runtime.rs */

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
  return testMode || nodeEnv === 'test' || nodeEnv === 'development';
}
