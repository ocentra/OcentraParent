import {
  LogLevel,
  LoggerRuntimeDefaults,
  LoggerRuntimeEnvironment,
  type LogLevel as LogLevelValue,
  type StackTrace,
} from '@ocentra-parent/schema-domain/logging-contracts';
import {
  RunType,
  TestLogOrigin,
  TestLogScope,
  parseRunTypeOrDefault,
  parseSuiteTypeOrNull,
  parseTestLogScopeOrDefault,
  type RunType as RunTypeValue,
  type TestLogOrigin as TestLogOriginValue,
  type TestLogScope as TestLogScopeValue,
  type TestSuiteType,
} from '@ocentra-parent/schema-domain/test-log/types';
import { createParentLogDecisionProvider } from './logDecisionProvider';
import { resolveBridgeEndpoint, sendToBridge } from '../transport/bridgeTransport';
import type { BridgeEntry } from '@ocentra-parent/schema-domain/transport/bridgeLogPayload';
import { parseStackTrace, type StackFrame } from './stackTraceParser';

export interface LoggerRuntimeConfig {
  readonly bridgeEndpoint?: string | null;
  readonly runId?: string | null;
  readonly testName?: string | null;
  readonly scope?: TestLogScopeValue | null;
  readonly runType?: RunTypeValue | null;
  readonly suiteType?: TestSuiteType | null;
  readonly origin?: TestLogOriginValue | null;
  readonly environment?: string | null;
  readonly correlationId?: string | null;
  readonly skipHealthCheck?: boolean;
}

interface LoggerRegistration {
  readonly moduleName: string;
  readonly file: string | null;
  readonly filePath: string | null;
  readonly absoluteFilePath: string;
}

interface ResolvedRuntimeConfig {
  readonly bridgeEndpoint: string | null;
  readonly runId: string;
  readonly testName: string;
  readonly scope: TestLogScopeValue;
  readonly runType: RunTypeValue;
  readonly suiteType: TestSuiteType | null;
  readonly origin: TestLogOriginValue | null;
  readonly environment: string | null;
  readonly correlationId: string | null;
  readonly skipHealthCheck: boolean;
}

interface ResolvedLogLocation {
  readonly registration: LoggerRegistration | null;
  readonly matchedFrame: StackFrame | null;
  readonly moduleName: string;
  readonly file: string | null;
  readonly filePath: string | null;
}

function readEnv(name: string): string | undefined {
  if (typeof process === 'undefined' || process.env == null) {
    return undefined;
  }
  const value = process.env[name];
  return value != null && value.trim().length > 0 ? value.trim() : undefined;
}

function normalizePath(value: string): string {
  return value.replace(/\\/g, '/');
}

function toFilePath(moduleUrl: string): string {
  if (moduleUrl.startsWith('file://')) {
    const url = new URL(moduleUrl);
    return normalizePath(decodeURIComponent(url.pathname).replace(/^\/([A-Za-z]:)/, '$1'));
  }
  return normalizePath(moduleUrl);
}

function toRelativePath(filePath: string): string {
  const normalized = normalizePath(filePath);
  if (typeof process === 'undefined') {
    return normalized;
  }
  const cwd = normalizePath(process.cwd());
  if (normalized.toLowerCase().startsWith(`${cwd.toLowerCase()}/`)) {
    return normalized.slice(cwd.length + 1);
  }
  if (normalized.toLowerCase() === cwd.toLowerCase()) {
    return '.';
  }
  return normalized;
}

function fileNameFromPath(filePath: string): string {
  const normalized = normalizePath(filePath);
  const lastSlash = normalized.lastIndexOf('/');
  return lastSlash >= 0 ? normalized.slice(lastSlash + 1) : normalized;
}

function moduleNameFromPath(filePath: string): string {
  const fileName = fileNameFromPath(filePath).replace(/\.[^.]+$/, '');
  return fileName
    .split(/[^a-zA-Z0-9]+/)
    .filter((segment) => segment.length > 0)
    .map((segment) => segment.slice(0, 1).toUpperCase() + segment.slice(1))
    .join('');
}

function resolveOrigin(value: string | null | undefined): TestLogOriginValue | null {
  switch (value) {
    case TestLogOrigin.AgentService:
      return TestLogOrigin.AgentService;
    case TestLogOrigin.Portal:
      return TestLogOrigin.Portal;
    case TestLogOrigin.Worker:
      return TestLogOrigin.Worker;
    case TestLogOrigin.Codex:
      return TestLogOrigin.Codex;
    case TestLogOrigin.Test:
      return TestLogOrigin.Test;
    default:
      return null;
  }
}

function resolveContext(moduleName: string, frame: StackFrame | null): string {
  if (frame?.functionName != null && frame.functionName.trim().length > 0) {
    return frame.functionName.includes('.') ? frame.functionName : `${moduleName}.${frame.functionName}`;
  }
  return `${moduleName}.${LoggerRuntimeDefaults.ModuleContextSuffix}`;
}

function resolveSource(moduleName: string, frame: StackFrame | null): string {
  if (frame?.functionName != null && frame.functionName.includes('.')) {
    return frame.functionName.split('.')[0] ?? moduleName;
  }
  return moduleName;
}

export class Logger {
  static readonly instance = new Logger();

  private readonly registrations = new Map<string, LoggerRegistration>();
  private readonly logQueue: BridgeEntry[] = [];
  private runtimeConfig: Partial<LoggerRuntimeConfig> = {};
  private runSequence = 0;
  private generatedRunId: string | null = null;

  configure(config: Partial<LoggerRuntimeConfig>): void {
    this.runtimeConfig = {
      ...this.runtimeConfig,
      ...config,
    };
  }

  reset(): void {
    this.registrations.clear();
    this.logQueue.length = 0;
    this.runtimeConfig = {};
    this.runSequence = 0;
    this.generatedRunId = null;
  }

  register(moduleUrl: string): void {
    const absolutePath = toFilePath(moduleUrl);
    const relativePath = toRelativePath(absolutePath);
    const registration: LoggerRegistration = {
      moduleName: moduleNameFromPath(relativePath),
      file: fileNameFromPath(relativePath),
      filePath: relativePath,
      absoluteFilePath: normalizePath(absolutePath),
    };
    this.registrations.set(registration.absoluteFilePath.toLowerCase(), registration);
  }

  logInfo(message: string, stackTrace: StackTrace, data?: unknown, enabled = true): void {
    if (!enabled) {
      return;
    }
    this.log(LogLevel.Info, message, stackTrace, data);
  }

  logWarn(message: string, stackTrace: StackTrace, data?: unknown, enabled = true): void {
    if (!enabled) {
      return;
    }
    this.log(LogLevel.Warn, message, stackTrace, data);
  }

  logError(message: string, stackTrace: StackTrace, data?: unknown): void {
    this.log(LogLevel.Error, message, stackTrace, data);
  }

  logDebug(message: string, stackTrace: StackTrace, data?: unknown, enabled = false): void {
    if (!enabled) {
      return;
    }
    this.log(LogLevel.Debug, message, stackTrace, data);
  }

  async flushLogQueue(): Promise<void> {
    if (this.logQueue.length === 0) {
      return;
    }

    const runtime = this.resolveRuntimeConfig();
    if (runtime.bridgeEndpoint == null || runtime.bridgeEndpoint.length === 0) {
      return;
    }

    const entries = this.logQueue.splice(0, this.logQueue.length);
    await sendToBridge(entries, runtime.bridgeEndpoint, {
      skipHealthCheck: runtime.skipHealthCheck,
    });
  }

  async flush(): Promise<void> {
    await this.flushLogQueue();
  }

  private log(level: LogLevelValue, message: string, stackTrace: StackTrace, data?: unknown): void {
    const frames = parseStackTrace(stackTrace);
    const location = this.resolveLogLocation(frames);
    if (!this.shouldStoreLog(level, location)) {
      return;
    }

    const runtime = this.resolveRuntimeConfig();
    this.logQueue.push(this.buildBridgeEntry(level, message, stackTrace, data, runtime, location));
  }

  private findRegistration(frames: readonly StackFrame[]): LoggerRegistration | null {
    for (const frame of frames) {
      if (frame.filePath == null) {
        continue;
      }
      const registration = this.registrations.get(normalizePath(frame.filePath).toLowerCase());
      if (registration != null) {
        return registration;
      }
    }
    return null;
  }

  private findMatchedFrame(frames: readonly StackFrame[], registration: LoggerRegistration | null): StackFrame | null {
    if (registration?.filePath != null) {
      for (const frame of frames) {
        if (frame.filePath === registration.absoluteFilePath) {
          return frame;
        }
      }
    }
    return frames.find((frame) => frame.filePath != null) ?? null;
  }

  private resolveLogLocation(frames: readonly StackFrame[]): ResolvedLogLocation {
    const registration = this.findRegistration(frames);
    const matchedFrame = this.findMatchedFrame(frames, registration);
    return {
      registration,
      matchedFrame,
      moduleName: registration?.moduleName ?? LoggerRuntimeDefaults.UnknownModule,
      file: registration?.file ?? matchedFrame?.file ?? null,
      filePath: registration?.filePath ?? matchedFrame?.filePath ?? null,
    };
  }

  private shouldStoreLog(level: LogLevelValue, location: ResolvedLogLocation): boolean {
    return createParentLogDecisionProvider().shouldStoreLog(location.moduleName, level, {
      filePath: location.filePath,
    });
  }

  private buildBridgeEntry(
    level: LogLevelValue,
    message: string,
    stackTrace: StackTrace,
    data: unknown,
    runtime: ResolvedRuntimeConfig,
    location: ResolvedLogLocation
  ): BridgeEntry {
    return {
      testName: runtime.testName,
      runId: runtime.runId,
      runType: runtime.runType,
      consumer: runtime.scope,
      log: {
        log_timestamp: Date.now(),
        level,
        source: resolveSource(location.moduleName, location.matchedFrame),
        context: resolveContext(location.moduleName, location.matchedFrame),
        message,
        data: this.serializeData(data),
        file: location.file,
        file_path: location.filePath,
        line: location.matchedFrame?.line ?? null,
        column: location.matchedFrame?.column ?? null,
        correlation_id: this.resolveCorrelationId(runtime),
        tags: [],
        stack: this.resolveStack(level, stackTrace),
        suite_type: runtime.suiteType,
        origin: runtime.origin,
        environment: runtime.environment,
      },
    };
  }

  private serializeData(data: unknown): string | null {
    return data == null ? null : JSON.stringify(data);
  }

  private resolveCorrelationId(runtime: ResolvedRuntimeConfig): string {
    return runtime.correlationId ?? globalThis.crypto?.randomUUID?.() ?? `${Date.now()}`;
  }

  private resolveStack(level: LogLevelValue, stackTrace: StackTrace): string | null {
    return level === LogLevel.Warn || level === LogLevel.Error ? String(stackTrace) : null;
  }

  private resolveRuntimeConfig(): ResolvedRuntimeConfig {
    return {
      bridgeEndpoint: this.runtimeConfig.bridgeEndpoint ?? resolveBridgeEndpoint(),
      runId: this.resolveRunId(),
      testName: this.resolveTestName(),
      scope: this.resolveScope(),
      runType: this.resolveRunType(),
      suiteType: this.resolveSuiteType(),
      origin: this.resolveOrigin(),
      environment: this.resolveEnvironment(),
      correlationId: this.runtimeConfig.correlationId ?? null,
      skipHealthCheck: this.runtimeConfig.skipHealthCheck ?? false,
    };
  }

  private resolveRunId(): string {
    return this.runtimeConfig.runId ?? readEnv(LoggerRuntimeEnvironment.RunId) ?? this.ensureGeneratedRunId();
  }

  private ensureGeneratedRunId(): string {
    if (this.generatedRunId == null) {
      this.runSequence += 1;
      this.generatedRunId = `${LoggerRuntimeDefaults.GeneratedRunIdPrefix}${this.runSequence}`;
    }
    return this.generatedRunId;
  }

  private resolveTestName(): string {
    return this.runtimeConfig.testName ?? readEnv(LoggerRuntimeEnvironment.TestName) ?? LoggerRuntimeDefaults.TestName;
  }

  private resolveScope(): TestLogScopeValue {
    return parseTestLogScopeOrDefault(
      this.runtimeConfig.scope ?? readEnv(LoggerRuntimeEnvironment.Scope),
      TestLogScope.ParentTest
    );
  }

  private resolveRunType(): RunTypeValue {
    return parseRunTypeOrDefault(
      this.runtimeConfig.runType ?? readEnv(LoggerRuntimeEnvironment.RunType),
      RunType.Single
    );
  }

  private resolveSuiteType(): TestSuiteType | null {
    return parseSuiteTypeOrNull(this.runtimeConfig.suiteType ?? readEnv(LoggerRuntimeEnvironment.SuiteType));
  }

  private resolveOrigin(): TestLogOriginValue | null {
    return resolveOrigin(this.runtimeConfig.origin ?? readEnv(LoggerRuntimeEnvironment.Origin));
  }

  private resolveEnvironment(): string | null {
    return this.runtimeConfig.environment ?? readEnv(LoggerRuntimeEnvironment.Environment) ?? null;
  }
}
