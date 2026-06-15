import type { LogLevel as LogLevelValue } from '../contracts';
import {
  createParentLogConfig,
  isDevOrTestEnvironment,
  isLevelAtOrAbove,
  normalizeDebugPath,
  type ParentLogConfig,
} from './logConfig';

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

function normalizeSource(source: string | null): string | null {
  if (source == null || source.trim().length === 0) {
    return null;
  }
  return source.trim().toLowerCase();
}

function matchesDebugSelection(
  config: ParentLogConfig,
  source: string | null,
  context?: ParentLogDecisionContext
): boolean {
  const normalizedSource = normalizeSource(source);
  if (
    normalizedSource != null &&
    (config.debugSources.includes(normalizedSource) ||
      context?.requestDebugSources?.some((entry) => entry.trim().toLowerCase() === normalizedSource) === true)
  ) {
    return true;
  }

  if (context?.filePath != null && context.filePath.trim().length > 0) {
    const normalizedFile = normalizeDebugPath(context.filePath);
    if (config.debugFiles.some((entry) => normalizedFile.includes(entry))) {
      return true;
    }
  }

  if (context?.runId != null && context.runId.trim().length > 0) {
    return config.debugRuns.includes(context.runId.trim());
  }

  return false;
}

export class ParentLogDecisionProvider implements LogDecisionProvider {
  constructor(private readonly config: ParentLogConfig) {}

  shouldLog(source: string | null, level: LogLevelValue, context?: ParentLogDecisionContext): boolean {
    return this.shouldLogToConsole(source, level, context) || this.shouldStoreLog(source, level, context);
  }

  shouldLogToConsole(source: string | null, level: LogLevelValue, context?: ParentLogDecisionContext): boolean {
    if (!this.config.consoleEnabled) {
      return false;
    }

    if (level === 'error' || level === 'warn') {
      return true;
    }

    if (!this.config.enabled) {
      return false;
    }

    if (matchesDebugSelection(this.config, source, context)) {
      return true;
    }

    return isDevOrTestEnvironment(this.config) && isLevelAtOrAbove(level, this.config.minLevel);
  }

  shouldStoreLog(source: string | null, level: LogLevelValue, context?: ParentLogDecisionContext): boolean {
    if (level === 'error' || level === 'warn') {
      return true;
    }

    if (!this.config.storeEnabled || !this.config.enabled) {
      return false;
    }

    if (matchesDebugSelection(this.config, source, context)) {
      return true;
    }

    return isDevOrTestEnvironment(this.config) && isLevelAtOrAbove(level, this.config.minLevel);
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
