import type { LogLevel as LogLevelValue } from '@ocentra-parent/schema-domain/logging-contracts';
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

function matchesConfiguredDebugSource(config: ParentLogConfig, normalizedSource: string): boolean {
  return config.debugSources.includes(normalizedSource);
}

function matchesRequestedDebugSource(normalizedSource: string, context?: ParentLogDecisionContext): boolean {
  return context?.requestDebugSources?.some((entry) => entry.trim().toLowerCase() === normalizedSource) === true;
}

function matchesDebugFile(config: ParentLogConfig, filePath?: string | null): boolean {
  if (filePath == null || filePath.trim().length === 0) {
    return false;
  }
  const normalizedFile = normalizeDebugPath(filePath);
  return config.debugFiles.some((entry) => normalizedFile.includes(entry));
}

function matchesDebugRun(config: ParentLogConfig, runId?: string | null): boolean {
  if (runId == null || runId.trim().length === 0) {
    return false;
  }
  return config.debugRuns.includes(runId.trim());
}

function matchesDebugSelection(
  config: ParentLogConfig,
  source: string | null,
  context?: ParentLogDecisionContext
): boolean {
  const normalizedSource = normalizeSource(source);
  if (
    normalizedSource != null &&
    (matchesConfiguredDebugSource(config, normalizedSource) || matchesRequestedDebugSource(normalizedSource, context))
  ) {
    return true;
  }

  if (matchesDebugFile(config, context?.filePath)) {
    return true;
  }

  return matchesDebugRun(config, context?.runId);
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
