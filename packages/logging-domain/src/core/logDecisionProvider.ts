import type { GeneratedLogLevel as LogLevelValue } from '../generated-logging-contracts';
import { createParentLogConfig, isDevOrTestEnvironment, type ParentLogConfig } from './logConfig';
import {
  matchesGeneratedDebugSelection,
  shouldGeneratedLogToConsole,
  shouldGeneratedStoreLog,
} from '../parent-log-runtime';

export interface ParentLogDecisionContext {
  readonly filePath?: string | null;
  readonly runId?: string | null;
  readonly requestDebugSources?: readonly string[];
}

export interface LogDecisionProvider {
  shouldLog(source: string | null, level: LogLevelValue, context?: ParentLogDecisionContext): boolean;
  shouldLogToConsole(source: string | null, level: LogLevelValue, context?: ParentLogDecisionContext): boolean;
  shouldStoreLog(source: string | null, level: LogLevelValue, context?: ParentLogDecisionContext): boolean;
  isDevOrTestEnvironment(): boolean;
  getConfig(): ParentLogConfig;
}

function matchesDebugSelection(
  config: ParentLogConfig,
  source: string | null,
  context?: ParentLogDecisionContext
): boolean {
  return matchesGeneratedDebugSelection(
    config.debugSources,
    config.debugFiles,
    config.debugRuns,
    source,
    context?.filePath,
    context?.runId,
    context?.requestDebugSources
  );
}

export class ParentLogDecisionProvider implements LogDecisionProvider {
  constructor(private readonly config: ParentLogConfig) {}

  shouldLog(source: string | null, level: LogLevelValue, context?: ParentLogDecisionContext): boolean {
    return this.shouldLogToConsole(source, level, context) || this.shouldStoreLog(source, level, context);
  }

  shouldLogToConsole(source: string | null, level: LogLevelValue, context?: ParentLogDecisionContext): boolean {
    return shouldGeneratedLogToConsole(
      this.config.enabled,
      this.config.consoleEnabled,
      this.config.nodeEnv,
      this.config.testMode,
      level,
      this.config.minLevel,
      matchesDebugSelection(this.config, source, context)
    );
  }

  shouldStoreLog(source: string | null, level: LogLevelValue, context?: ParentLogDecisionContext): boolean {
    return shouldGeneratedStoreLog(
      this.config.enabled,
      this.config.storeEnabled,
      this.config.nodeEnv,
      this.config.testMode,
      level,
      this.config.minLevel,
      matchesDebugSelection(this.config, source, context)
    );
  }

  isDevOrTestEnvironment(): boolean {
    return isDevOrTestEnvironment(this.config);
  }

  getConfig(): ParentLogConfig {
    return this.config;
  }
}

export function createParentLogDecisionProvider(
  env?: NodeJS.ProcessEnv | Record<string, string | undefined>
): ParentLogDecisionProvider {
  return new ParentLogDecisionProvider(createParentLogConfig(env));
}
