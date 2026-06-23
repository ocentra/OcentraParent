import {
  DevLogBridge,
  DevLogEndpoint,
  DevLogHttp,
  DevLogIdPrefix,
  LogLevel,
  LogSource,
  decodeLogEntryId,
  decodeLogTimestamp,
  type DevLogEntry,
  type LogFields,
  type LogMessage,
  type StackTrace,
} from '@ocentra-parent/schema-domain/logging-contracts';
import { Logger } from '@ocentra-parent/logging-domain/core/logger';
import { RunType, TestLogOrigin, TestLogScope } from '@ocentra-parent/schema-domain/test-log/types';

const PortalDevLogBridge = DevLogBridge;
const PortalProofTrace = {
  EnabledEnv: 'VITE_OCENTRA_PARENT_PROOF_TRACE',
  IdEnv: 'VITE_OCENTRA_PARENT_PROOF_TRACE_ID',
  ScopeEnv: 'VITE_OCENTRA_PARENT_PROOF_TRACE_SCOPE',
  SourcesEnv: 'VITE_OCENTRA_PARENT_PROOF_TRACE_SOURCES',
  LevelEnv: 'VITE_OCENTRA_PARENT_PROOF_TRACE_LEVEL',
  EnabledGlobalKey: '__OCENTRA_PARENT_PROOF_TRACE',
  IdGlobalKey: '__OCENTRA_PARENT_PROOF_TRACE_ID',
  ScopeGlobalKey: '__OCENTRA_PARENT_PROOF_TRACE_SCOPE',
  SourcesGlobalKey: '__OCENTRA_PARENT_PROOF_TRACE_SOURCES',
  LevelGlobalKey: '__OCENTRA_PARENT_PROOF_TRACE_LEVEL',
  AllowedSourcePortal: 'portal',
} as const;

export interface PortalProofTraceOptions {
  readonly proofId?: string;
  readonly traceStep: string;
  readonly eventType: string;
  readonly action?: string;
  readonly command?: string;
  readonly status?: string;
  readonly expectedNext?: string;
  readonly artifactRef?: string;
  readonly causationId?: string;
  readonly correlationId?: string;
  readonly scope?: string;
}

export interface PortalProofTraceConfig {
  readonly enabled: boolean;
  readonly proofId: string | null;
  readonly scope: string | null;
  readonly sources: readonly string[];
  readonly level: string | null;
}

export interface PortalLoggerRuntimeConfig {
  readonly runId: string;
  readonly testName: string;
  readonly scope: TestLogScope;
  readonly runType: RunType;
  readonly origin: TestLogOrigin;
  readonly environment: string;
  readonly correlationId: string | null;
}

export interface PortalLoggerDispatchContext {
  readonly endpoint: string;
  readonly runtime: PortalLoggerRuntime;
  readonly stackTrace: StackTrace;
  readonly moduleUrl: string;
}

export type PortalLoggerRuntime = Record<string, unknown>;
type PortalImportMetaWithEnv = ImportMeta & { readonly env?: Record<string, string | undefined> };
type PortalFetch = (
  input: string,
  init: {
    readonly method: string;
    readonly headers: Record<string, string>;
    readonly body: string;
    readonly credentials: string;
  }
) => Promise<{ readonly ok: boolean }>;

const portalLogger = Logger.instance;
const PORTAL_DEV_LOG_RUN_ID = `portal-dev-${createPortalLogToken()}`;

export function resolvePortalDevLogBridgeUrl(runtime: PortalLoggerRuntime = globalThis as PortalLoggerRuntime): string {
  const fromEnv = readTrimmedString(getPortalEnv()[PortalDevLogBridge.EnvironmentUrl]);
  if (fromEnv !== null) {
    return fromEnv;
  }

  const fromGlobal = readTrimmedString(runtime[PortalDevLogBridge.GlobalUrlKey]);
  if (fromGlobal !== null) {
    return fromGlobal;
  }

  return PortalDevLogBridge.DefaultUrl;
}

export function resolvePortalProofTraceConfig(
  runtime: PortalLoggerRuntime = globalThis as PortalLoggerRuntime
): PortalProofTraceConfig {
  const viteEnv = getPortalEnv();
  const proofId = firstNonEmptyString(viteEnv[PortalProofTrace.IdEnv], runtime[PortalProofTrace.IdGlobalKey]);
  return {
    enabled:
      parseBoolean(viteEnv[PortalProofTrace.EnabledEnv]) ||
      parseBoolean(runtime[PortalProofTrace.EnabledGlobalKey]) ||
      proofId !== null,
    proofId,
    scope: firstNonEmptyString(viteEnv[PortalProofTrace.ScopeEnv], runtime[PortalProofTrace.ScopeGlobalKey]),
    sources: parseList(
      firstNonEmptyString(viteEnv[PortalProofTrace.SourcesEnv], runtime[PortalProofTrace.SourcesGlobalKey])
    ),
    level: firstNonEmptyString(viteEnv[PortalProofTrace.LevelEnv], runtime[PortalProofTrace.LevelGlobalKey]),
  };
}

export async function sendPortalDevLogWithContext(
  message: LogMessage,
  fields: LogFields = {},
  context: PortalLoggerDispatchContext
): Promise<boolean> {
  if (await sendPortalBridgeMessage(message, fields, null, context)) {
    return true;
  }
  return sendPortalCompatibilityLog(message, fields, context.runtime);
}

export async function sendPortalProofTraceLogWithContext(
  message: LogMessage,
  proofTrace: PortalProofTraceOptions,
  fields: LogFields = {},
  context: PortalLoggerDispatchContext
): Promise<boolean> {
  const dispatch = buildPortalProofTraceDispatch(fields, proofTrace, context.runtime);
  if (dispatch === null) {
    return false;
  }
  if (await sendPortalBridgeMessage(message, dispatch.fields, dispatch.runtimeConfig, context)) {
    return true;
  }
  return sendPortalCompatibilityLog(message, dispatch.fields, context.runtime);
}

function buildPortalProofTraceDispatch(
  fields: LogFields,
  proofTrace: PortalProofTraceOptions,
  runtime: PortalLoggerRuntime
): { readonly fields: LogFields; readonly runtimeConfig: PortalLoggerRuntimeConfig } | null {
  const config = resolvePortalProofTraceConfig(runtime);
  const resolvedProofTrace = resolvePortalProofTraceOptions(config, proofTrace);
  if (resolvedProofTrace === null) {
    return null;
  }
  return {
    fields: mergeProofTraceFields(fields, resolvedProofTrace),
    runtimeConfig: {
      runId: resolvedProofTrace.proofId ?? PORTAL_DEV_LOG_RUN_ID,
      testName: PortalDevLogBridge.PortalTestName,
      scope: resolveProofTraceScope(resolvedProofTrace.scope),
      runType: RunType.Single,
      origin: TestLogOrigin.Portal,
      environment: PortalDevLogBridge.PortalEnvironment,
      correlationId: resolvedProofTrace.correlationId ?? null,
    },
  };
}

function resolvePortalProofTraceOptions(
  config: PortalProofTraceConfig,
  proofTrace: PortalProofTraceOptions
): PortalProofTraceOptions | null {
  const effectiveProofId = proofTrace.proofId ?? config.proofId;
  if ((!config.enabled && effectiveProofId === null) || !proofTraceAllowedForPortal(config)) {
    return null;
  }

  return {
    ...proofTrace,
    ...(effectiveProofId === null ? {} : { proofId: effectiveProofId }),
    ...(proofTrace.scope != null ? { scope: proofTrace.scope } : config.scope === null ? {} : { scope: config.scope }),
  };
}

function mergeProofTraceFields(fields: LogFields, proofTrace: PortalProofTraceOptions): LogFields {
  return {
    ...fields,
    proofId: proofTrace.proofId ?? null,
    traceStep: proofTrace.traceStep,
    eventType: proofTrace.eventType,
    action: proofTrace.action ?? null,
    command: proofTrace.command ?? null,
    status: proofTrace.status ?? null,
    expectedNext: proofTrace.expectedNext ?? null,
    artifactRef: proofTrace.artifactRef ?? null,
    causationId: proofTrace.causationId ?? null,
  };
}

function resolveProofTraceScope(scope: string | undefined): TestLogScope {
  switch (scope) {
    case TestLogScope.ParentAgent:
      return TestLogScope.ParentAgent;
    case TestLogScope.ParentCloudflare:
      return TestLogScope.ParentCloudflare;
    case TestLogScope.ParentCodex:
      return TestLogScope.ParentCodex;
    case TestLogScope.ParentTest:
      return TestLogScope.ParentTest;
    case TestLogScope.ParentPortal:
    default:
      return TestLogScope.ParentPortal;
  }
}

function proofTraceAllowedForPortal(config: PortalProofTraceConfig): boolean {
  return config.sources.length === 0 || config.sources.includes(PortalProofTrace.AllowedSourcePortal);
}

function parseBoolean(value: unknown): boolean {
  if (typeof value !== 'string') {
    return value === true;
  }
  const normalized = value.trim().toLowerCase();
  return normalized === 'true' || normalized === '1' || normalized === 'yes' || normalized === 'on';
}

function parseList(value: string | null): string[] {
  if (value === null || value.trim().length === 0) {
    return [];
  }
  return value
    .split(',')
    .map((entry) => entry.trim())
    .filter((entry) => entry.length > 0);
}

function firstNonEmptyString(...values: unknown[]): string | null {
  for (const value of values) {
    const normalized = readTrimmedString(value);
    if (normalized !== null) {
      return normalized;
    }
  }
  return null;
}

function readTrimmedString(value: unknown): string | null {
  if (typeof value !== 'string') {
    return null;
  }
  const normalized = value.trim();
  return normalized.length === 0 ? null : normalized;
}

function getPortalEnv(): Record<string, string | undefined> {
  const env = (import.meta as PortalImportMetaWithEnv).env;
  return env ?? {};
}

function createPortalLogToken(): string {
  const runtimeCrypto = (globalThis as { readonly crypto?: { readonly randomUUID?: () => string } }).crypto;
  if (runtimeCrypto?.randomUUID != null) {
    return runtimeCrypto.randomUUID();
  }
  return `portal-dev-log-${Date.now()}-${Math.random().toString(16).slice(2)}`;
}

async function sendPortalBridgeMessage(
  message: LogMessage,
  fields: LogFields,
  runtimeConfig: PortalLoggerRuntimeConfig | null,
  context: PortalLoggerDispatchContext
): Promise<boolean> {
  if (context.endpoint.length === 0) {
    return false;
  }

  portalLogger.register(context.moduleUrl);
  portalLogger.configure(buildPortalLoggerConfiguration(context.endpoint, runtimeConfig));

  try {
    portalLogger.logInfo(message, context.stackTrace, fields, true);
    await portalLogger.flush();
    return true;
  } catch {
    return false;
  }
}

async function sendPortalCompatibilityLog(
  message: LogMessage,
  fields: LogFields,
  runtime: PortalLoggerRuntime
): Promise<boolean> {
  const endpoint = resolvePortalCompatibilityUrl(runtime);
  if (endpoint === null) {
    return false;
  }

  try {
    const fetchFn = (globalThis as { readonly fetch?: PortalFetch }).fetch;
    if (fetchFn === undefined) {
      return false;
    }
    const response = await fetchFn(endpoint, {
      method: DevLogHttp.MethodPost,
      headers: {
        [DevLogHttp.HeaderContentType]: DevLogHttp.ContentTypeJson,
      },
      body: JSON.stringify(createPortalCompatibilityEntry(message, fields)),
      credentials: DevLogHttp.CredentialsSameOrigin,
    });
    return response.ok;
  } catch {
    return false;
  }
}

function resolvePortalCompatibilityUrl(runtime: PortalLoggerRuntime): string | null {
  const location = runtime['location'];
  if (location === null || typeof location !== 'object') {
    return null;
  }
  const origin = readTrimmedString((location as { readonly origin?: unknown }).origin);
  if (origin === null) {
    return null;
  }
  return `${trimTrailingSolidus(origin)}${DevLogEndpoint.Write}`;
}

function trimTrailingSolidus(value: string): string {
  let end = value.length;
  while (end > 0 && value.charCodeAt(end - 1) === 47) {
    end -= 1;
  }
  return end === value.length ? value : value.slice(0, end);
}

function buildPortalLoggerConfiguration(
  endpoint: string,
  runtimeConfig: PortalLoggerRuntimeConfig | null
): {
  readonly bridgeEndpoint: string;
  readonly runId: string;
  readonly testName: string;
  readonly scope: TestLogScope;
  readonly runType: RunType;
  readonly origin: TestLogOrigin;
  readonly environment: string;
  readonly correlationId: string | null;
} {
  if (runtimeConfig === null) {
    return {
      bridgeEndpoint: endpoint,
      runId: PORTAL_DEV_LOG_RUN_ID,
      testName: PortalDevLogBridge.PortalTestName,
      scope: TestLogScope.ParentPortal,
      runType: RunType.Single,
      origin: TestLogOrigin.Portal,
      environment: PortalDevLogBridge.PortalEnvironment,
      correlationId: null,
    };
  }

  return {
    bridgeEndpoint: endpoint,
    runId: runtimeConfig.runId,
    testName: runtimeConfig.testName,
    scope: runtimeConfig.scope,
    runType: runtimeConfig.runType,
    origin: runtimeConfig.origin,
    environment: runtimeConfig.environment,
    correlationId: runtimeConfig.correlationId,
  };
}

function createPortalCompatibilityEntry(message: LogMessage, fields: LogFields): DevLogEntry {
  return {
    schemaVersion: 1,
    id: decodeLogEntryId(`${DevLogIdPrefix.Portal}${createPortalLogToken()}`),
    timestamp: decodeLogTimestamp(new Date().toISOString()),
    level: LogLevel.Info,
    source: LogSource.Portal,
    message,
    fields,
  };
}
