import {
  GeneratedDevLogBridge as DevLogBridge,
  GeneratedDevLogEndpoint as DevLogEndpoint,
  GeneratedDevLogHttp as DevLogHttp,
  GeneratedDevLogIdPrefix as DevLogIdPrefix,
  GeneratedLogLevel as LogLevel,
  type GeneratedLogMessage as LogMessage,
  GeneratedLogEntryIdSchema,
  GeneratedLogSource as LogSource,
  type GeneratedDevLogEntry as DevLogEntry,
  type GeneratedLogFields as LogFields,
  GeneratedLogTimestampSchema,
  type GeneratedStackTrace as StackTrace,
} from '@ocentra-parent/logging-domain/generated/logging-contracts';
import { Schema } from 'effect';
import { Logger } from '@ocentra-parent/logging-domain/core/logger';

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
const PortalTestLogScope = {
  ParentAgent: 'parent-agent',
  ParentPortal: 'parent-portal',
  ParentCloudflare: 'parent-cloudflare',
  ParentCodex: 'parent-codex',
  ParentTest: 'parent-test',
} as const;
const PortalRunType = {
  Single: 'single',
} as const;
const PortalTestLogOrigin = {
  Portal: 'portal',
} as const;
const PortalProofTraceScopeByValue: Record<string, PortalTestLogScope> = {
  [PortalTestLogScope.ParentAgent]: PortalTestLogScope.ParentAgent,
  [PortalTestLogScope.ParentPortal]: PortalTestLogScope.ParentPortal,
  [PortalTestLogScope.ParentCloudflare]: PortalTestLogScope.ParentCloudflare,
  [PortalTestLogScope.ParentCodex]: PortalTestLogScope.ParentCodex,
  [PortalTestLogScope.ParentTest]: PortalTestLogScope.ParentTest,
};
const PortalBooleanLookup: Record<string, boolean> = {
  true: true,
  '1': true,
  yes: true,
  on: true,
};
const PortalCompatibilityDefaultConfig = {
  runId: '',
  testName: PortalDevLogBridge.PortalTestName,
  scope: PortalTestLogScope.ParentPortal,
  runType: PortalRunType.Single,
  origin: PortalTestLogOrigin.Portal,
  environment: PortalDevLogBridge.PortalEnvironment,
  correlationId: null,
} as const;

type PortalTestLogScope = (typeof PortalTestLogScope)[keyof typeof PortalTestLogScope];
type PortalRunType = (typeof PortalRunType)[keyof typeof PortalRunType];
type PortalTestLogOrigin = (typeof PortalTestLogOrigin)[keyof typeof PortalTestLogOrigin];

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
  readonly scope: PortalTestLogScope;
  readonly runType: PortalRunType;
  readonly origin: PortalTestLogOrigin;
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
  return (
    firstNonEmptyString(getPortalEnv()[PortalDevLogBridge.EnvironmentUrl], runtime[PortalDevLogBridge.GlobalUrlKey]) ??
    PortalDevLogBridge.DefaultUrl
  );
}

export function resolvePortalProofTraceConfig(
  runtime: PortalLoggerRuntime = globalThis as PortalLoggerRuntime
): PortalProofTraceConfig {
  const viteEnv = getPortalEnv();
  const proofId = firstNonEmptyString(viteEnv[PortalProofTrace.IdEnv], runtime[PortalProofTrace.IdGlobalKey]);
  const enabled =
    parseBoolean(viteEnv[PortalProofTrace.EnabledEnv]) ||
    parseBoolean(runtime[PortalProofTrace.EnabledGlobalKey]) ||
    proofId !== null;

  return {
    enabled,
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
  return (
    (await sendPortalBridgeMessage(message, fields, null, context)) ||
    (await sendPortalCompatibilityLog(message, fields, context.runtime))
  );
}

export async function sendPortalProofTraceLogWithContext(
  message: LogMessage,
  proofTrace: PortalProofTraceOptions,
  fields: LogFields = {},
  context: PortalLoggerDispatchContext
): Promise<boolean> {
  const dispatch = buildPortalProofTraceDispatch(fields, proofTrace, context.runtime);
  return (
    dispatch !== null &&
    ((await sendPortalBridgeMessage(message, dispatch.fields, dispatch.runtimeConfig, context)) ||
      (await sendPortalCompatibilityLog(message, dispatch.fields, context.runtime)))
  );
}

function buildPortalProofTraceDispatch(
  fields: LogFields,
  proofTrace: PortalProofTraceOptions,
  runtime: PortalLoggerRuntime
): { readonly fields: LogFields; readonly runtimeConfig: PortalLoggerRuntimeConfig } | null {
  const config = resolvePortalProofTraceConfig(runtime);
  const resolvedProofTrace = resolvePortalProofTraceOptions(config, proofTrace);
  return resolvedProofTrace === null
    ? null
    : {
        fields: mergeProofTraceFields(fields, resolvedProofTrace),
        runtimeConfig: {
          runId: resolvedProofTrace.proofId ?? PORTAL_DEV_LOG_RUN_ID,
          testName: PortalDevLogBridge.PortalTestName,
          scope: resolveProofTraceScope(resolvedProofTrace.scope),
          runType: PortalRunType.Single,
          origin: PortalTestLogOrigin.Portal,
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
  const allowed = (config.enabled || effectiveProofId !== null) && proofTraceAllowedForPortal(config);
  return allowed
    ? {
        ...proofTrace,
        ...(effectiveProofId === null ? {} : { proofId: effectiveProofId }),
        ...resolvePortalProofTraceScopeOverride(proofTrace.scope, config.scope),
      }
    : null;
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

function resolveProofTraceScope(scope: string | undefined): PortalTestLogScope {
  return PortalProofTraceScopeByValue[scope ?? ''] ?? PortalTestLogScope.ParentPortal;
}

function proofTraceAllowedForPortal(config: PortalProofTraceConfig): boolean {
  return config.sources.length === 0 || config.sources.includes(PortalProofTrace.AllowedSourcePortal);
}

function parseBoolean(value: unknown): boolean {
  return typeof value === 'string' ? PortalBooleanLookup[value.trim().toLowerCase()] === true : value === true;
}

function parseList(value: string | null): string[] {
  const normalized = value?.trim();
  return normalized === undefined || normalized.length === 0
    ? []
    : normalized
        .split(',')
        .map((entry) => entry.trim())
        .filter((entry) => entry.length > 0);
}

function firstNonEmptyString(...values: unknown[]): string | null {
  return values.map(readTrimmedString).find((value) => value !== null) ?? null;
}

function readTrimmedString(value: unknown): string | null {
  return typeof value === 'string' ? normalizedNonEmptyString(value) : null;
}

function normalizedNonEmptyString(value: string): string | null {
  const normalized = value.trim();
  return normalized.length === 0 ? null : normalized;
}

function getPortalEnv(): Record<string, string | undefined> {
  const env = (import.meta as PortalImportMetaWithEnv).env;
  return env ?? {};
}

function createPortalLogToken(): string {
  const runtimeCrypto = (globalThis as { readonly crypto?: { readonly randomUUID?: () => string } }).crypto;
  return runtimeCrypto?.randomUUID?.() ?? `portal-dev-log-${Date.now()}-${Math.random().toString(16).slice(2)}`;
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
  portalLogger.logInfo(message, context.stackTrace, fields, true);
  return portalLogger.flush().then(
    () => true,
    () => false
  );
}

async function sendPortalCompatibilityLog(
  message: LogMessage,
  fields: LogFields,
  runtime: PortalLoggerRuntime
): Promise<boolean> {
  const endpoint = resolvePortalCompatibilityUrl(runtime);
  const fetchFn = (globalThis as { readonly fetch?: PortalFetch }).fetch;
  return endpoint === null || fetchFn === undefined
    ? false
    : fetchFn(endpoint, {
        method: DevLogHttp.MethodPost,
        headers: {
          [DevLogHttp.HeaderContentType]: DevLogHttp.ContentTypeJson,
        },
        body: JSON.stringify(createPortalCompatibilityEntry(message, fields)),
        credentials: DevLogHttp.CredentialsSameOrigin,
      }).then(
        (response) => response.ok,
        () => false
      );
}

function resolvePortalCompatibilityUrl(runtime: PortalLoggerRuntime): string | null {
  const origin = readTrimmedString((runtime['location'] as { readonly origin?: unknown } | undefined)?.origin);
  return origin === null ? null : `${trimTrailingSolidus(origin)}${DevLogEndpoint.Write}`;
}

function trimTrailingSolidus(value: string): string {
  let end = value.length;
  while (end > 0 && value.charCodeAt(end - 1) === 47) {
    end -= 1;
  }
  return value.slice(0, end);
}

function buildPortalLoggerConfiguration(
  endpoint: string,
  runtimeConfig: PortalLoggerRuntimeConfig | null
): {
  readonly bridgeEndpoint: string;
  readonly runId: string;
  readonly testName: string;
  readonly scope: PortalTestLogScope;
  readonly runType: PortalRunType;
  readonly origin: PortalTestLogOrigin;
  readonly environment: string;
  readonly correlationId: string | null;
} {
  const resolvedConfig = runtimeConfig ?? PortalCompatibilityDefaultConfig;
  return {
    bridgeEndpoint: endpoint,
    runId: runtimeConfig?.runId ?? PORTAL_DEV_LOG_RUN_ID,
    testName: resolvedConfig.testName,
    scope: resolvedConfig.scope,
    runType: resolvedConfig.runType,
    origin: resolvedConfig.origin,
    environment: resolvedConfig.environment,
    correlationId: resolvedConfig.correlationId,
  };
}

function resolvePortalProofTraceScopeOverride(
  proofTraceScope: string | undefined,
  configScope: string | null
): Partial<Pick<PortalProofTraceOptions, 'scope'>> {
  return proofTraceScope != null ? { scope: proofTraceScope } : configScope === null ? {} : { scope: configScope };
}

function createPortalCompatibilityEntry(message: LogMessage, fields: LogFields): DevLogEntry {
  return {
    schemaVersion: 1,
    id: Schema.decodeUnknownSync(GeneratedLogEntryIdSchema)(`${DevLogIdPrefix.Portal}${createPortalLogToken()}`),
    timestamp: Schema.decodeUnknownSync(GeneratedLogTimestampSchema)(new Date().toISOString()),
    level: LogLevel.Info,
    source: LogSource.Portal,
    message,
    fields,
  };
}
