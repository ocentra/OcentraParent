import {
  GeneratedLoggerRuntimeDefaults as LoggerRuntimeDefaults,
  GeneratedLoggerRuntimeEnvironment as LoggerRuntimeEnvironment,
  GeneratedLogLevel as LogLevel,
  type GeneratedLogLevel as LogLevelValue,
  type GeneratedStackTrace as StackTrace,
} from '../generated-logging-contracts';
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
} from '../test-log/types';
import { createParentLogDecisionProvider } from './logDecisionProvider';
import { createParentLogConfig } from './logConfig';
import {
  BridgeLogQueue,
  type AmbiguousBridgeDeliveryResolution,
  type BridgeQueueDeliveryState,
} from './bridgeLogQueue';
import { serializeStructuredLogDataForCustody } from './structuredLogCustody';
import type { BridgeEntry } from '../transport/bridgeLogPayload';
import { parseStackTrace, type StackFrame } from './stackTraceParser';
import {
  fileNameFromGeneratedPath,
  moduleNameFromGeneratedPath,
  normalizeGeneratedStackPath,
  resolveGeneratedLoggerContext,
  resolveGeneratedLoggerSource,
} from '../stack-trace-runtime';

const TestLogOriginLookup: Readonly<Record<string, TestLogOriginValue>> = {
  [TestLogOrigin.AgentService]: TestLogOrigin.AgentService,
  [TestLogOrigin.Portal]: TestLogOrigin.Portal,
  [TestLogOrigin.Worker]: TestLogOrigin.Worker,
  [TestLogOrigin.Codex]: TestLogOrigin.Codex,
  [TestLogOrigin.Test]: TestLogOrigin.Test,
};

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
  const value = typeof process === 'undefined' ? undefined : process.env?.[name]?.trim();
  return value != null && value.length > 0 ? value : undefined;
}

function toFilePath(moduleUrl: string): string {
  if (moduleUrl.startsWith('file://')) {
    const url = new URL(moduleUrl);
    return normalizeGeneratedStackPath(decodeURIComponent(url.pathname).replace(/^\/([A-Za-z]:)/, '$1'));
  }
  return normalizeGeneratedStackPath(moduleUrl);
}

function toRelativePath(filePath: string): string {
  const normalized = normalizeGeneratedStackPath(filePath);
  if (typeof process === 'undefined') {
    return normalized;
  }
  const cwd = normalizeGeneratedStackPath(process.cwd());
  if (normalized.toLowerCase().startsWith(`${cwd.toLowerCase()}/`)) {
    return normalized.slice(cwd.length + 1);
  }
  if (normalized.toLowerCase() === cwd.toLowerCase()) {
    return '.';
  }
  return normalized;
}

function resolveOrigin(value: string | null | undefined): TestLogOriginValue | null {
  return TestLogOriginLookup[value ?? ''] ?? null;
}

export class Logger {
  static readonly instance = new Logger();

  private readonly registrations = new Map<string, LoggerRegistration>();
  private readonly bridgeQueue = new BridgeLogQueue(() => {
    const runtime = this.resolveRuntimeConfig();
    return { endpoint: runtime.bridgeEndpoint, skipHealthCheck: runtime.skipHealthCheck };
  });
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
    this.bridgeQueue.reset();
    this.runtimeConfig = {};
    this.runSequence = 0;
    this.generatedRunId = null;
  }

  register(moduleUrl: string): void {
    const absolutePath = toFilePath(moduleUrl);
    const relativePath = toRelativePath(absolutePath);
    const registration: LoggerRegistration = {
      moduleName: moduleNameFromGeneratedPath(relativePath),
      file: fileNameFromGeneratedPath(relativePath),
      filePath: relativePath,
      absoluteFilePath: normalizeGeneratedStackPath(absolutePath),
    };
    this.registrations.set(registration.absoluteFilePath.toLowerCase(), registration);
  }

  logInfo(message: string, stackTrace: StackTrace, data?: unknown, enabled = true): void {
    this.logIfEnabled(enabled, LogLevel.Info, message, stackTrace, data);
  }

  logWarn(message: string, stackTrace: StackTrace, data?: unknown, enabled = true): void {
    this.logIfEnabled(enabled, LogLevel.Warn, message, stackTrace, data);
  }

  logError(message: string, stackTrace: StackTrace, data?: unknown): void {
    this.log(LogLevel.Error, message, stackTrace, data);
  }

  logDebug(message: string, stackTrace: StackTrace, data?: unknown, enabled = false): void {
    this.logIfEnabled(enabled, LogLevel.Debug, message, stackTrace, data);
  }

  async flushLogQueue(): Promise<void> {
    await this.bridgeQueue.flush();
  }

  async flush(): Promise<void> {
    await this.flushLogQueue();
  }

  logQueueDeliveryState(): BridgeQueueDeliveryState {
    return this.bridgeQueue.deliveryState();
  }

  resolveAmbiguousLogDelivery(resolution: AmbiguousBridgeDeliveryResolution): void {
    this.bridgeQueue.resolveAmbiguousDelivery(resolution);
  }

  private log(level: LogLevelValue, message: string, stackTrace: StackTrace, data?: unknown): void {
    const frames = parseStackTrace(stackTrace);
    const location = this.resolveLogLocation(frames);
    const runtime = this.resolveRuntimeConfig();
    if (this.shouldStoreLog(level, location, runtime.runId)) {
      this.bridgeQueue.enqueue(this.buildBridgeEntry(level, message, stackTrace, data, runtime, location));
    }
  }

  private findRegistration(frames: readonly StackFrame[]): LoggerRegistration | null {
    const frame = frames.find(
      (candidate) =>
        candidate.filePath != null &&
        this.registrations.has(normalizeGeneratedStackPath(candidate.filePath).toLowerCase())
    );
    return frame?.filePath == null
      ? null
      : (this.registrations.get(normalizeGeneratedStackPath(frame.filePath).toLowerCase()) ?? null);
  }

  private findMatchedFrame(frames: readonly StackFrame[], registration: LoggerRegistration | null): StackFrame | null {
    const matchedFrame =
      registration?.filePath == null
        ? null
        : (frames.find((frame) => frame.filePath === registration.absoluteFilePath) ?? null);
    return matchedFrame ?? frames.find((frame) => frame.filePath != null) ?? null;
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

  private shouldStoreLog(level: LogLevelValue, location: ResolvedLogLocation, runId: string): boolean {
    return createParentLogDecisionProvider().shouldStoreLog(location.moduleName, level, {
      filePath: location.filePath,
      runId,
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
        source: resolveGeneratedLoggerSource(location.moduleName, location.matchedFrame),
        context: resolveGeneratedLoggerContext(
          location.moduleName,
          location.matchedFrame,
          LoggerRuntimeDefaults.ModuleContextSuffix
        ),
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
    return serializeStructuredLogDataForCustody(data);
  }

  private logIfEnabled(
    enabled: boolean,
    level: LogLevelValue,
    message: string,
    stackTrace: StackTrace,
    data?: unknown
  ): void {
    if (!enabled) {
      return;
    }
    this.log(level, message, stackTrace, data);
  }

  private resolveCorrelationId(runtime: ResolvedRuntimeConfig): string {
    return runtime.correlationId ?? globalThis.crypto?.randomUUID?.() ?? `${Date.now()}`;
  }

  private resolveStack(level: LogLevelValue, stackTrace: StackTrace): string | null {
    return level === LogLevel.Warn || level === LogLevel.Error ? String(stackTrace) : null;
  }

  private resolveRuntimeConfig(): ResolvedRuntimeConfig {
    const bridgeConfig = createParentLogConfig();
    return {
      bridgeEndpoint: this.runtimeConfig.bridgeEndpoint ?? bridgeConfig.bridgeUrl,
      runId: this.resolveRunId(),
      testName: this.resolveTestName(),
      scope: this.resolveScope(),
      runType: this.resolveRunType(),
      suiteType: this.resolveSuiteType(),
      origin: this.resolveOrigin(),
      environment: this.resolveEnvironment(),
      correlationId: this.runtimeConfig.correlationId ?? null,
      skipHealthCheck: this.runtimeConfig.skipHealthCheck ?? bridgeConfig.skipBridgeHealth,
    };
  }

  private resolveRunId(): string {
    return this.runtimeConfig.runId ?? readEnv(LoggerRuntimeEnvironment.RunId) ?? this.ensureGeneratedRunId();
  }

  private ensureGeneratedRunId(): string {
    this.generatedRunId ??= `${LoggerRuntimeDefaults.GeneratedRunIdPrefix}${++this.runSequence}`;
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
