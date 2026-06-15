import {
  DevLogBridge,
  DevLogField,
  DevLogMessage,
  type LogFields,
  type LogMessage,
} from '@ocentra-parent/logging-domain/contracts';
import { Logger } from '@ocentra-parent/logging-domain/core/logger';
import { getStackTrace } from '@ocentra-parent/logging-domain/core/stackTrace';
import { RunType, TestLogOrigin, TestLogScope } from '@ocentra-parent/logging-domain/test-log/types';

export { DevLogField, DevLogMessage };

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
};

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

interface PortalProofTraceConfig {
  readonly enabled: boolean;
  readonly proofId: string | null;
  readonly scope: string | null;
  readonly sources: readonly string[];
  readonly level: string | null;
}

const portalLogger = Logger.instance;

export function writePortalDevLog(message: LogMessage, fields: LogFields = {}): void {
  void sendPortalDevLog(message, fields);
}

export function writePortalProofTraceLog(
  message: LogMessage,
  proofTrace: PortalProofTraceOptions,
  fields: LogFields = {}
): void {
  void sendPortalProofTraceLog(message, proofTrace, fields);
}

export async function sendPortalDevLog(message: LogMessage, fields: LogFields = {}, endpoint = resolvePortalDevLogBridgeUrl()): Promise<boolean> {
  if (endpoint.length === 0) {
    return false;
  }
  return sendPortalLoggerMessage(message, fields, null, endpoint, getStackTrace());
}

export function resolvePortalDevLogBridgeUrl(runtime: Record<string, unknown> = globalThis as Record<string, unknown>): string {
  const fromEnv = getPortalEnv()[PortalDevLogBridge.EnvironmentUrl];
  if (typeof fromEnv === 'string' && fromEnv.trim().length > 0) {
    return fromEnv.trim();
  }

  const fromGlobal = runtime[PortalDevLogBridge.GlobalUrlKey];
  if (typeof fromGlobal === 'string' && fromGlobal.trim().length > 0) {
    return fromGlobal.trim();
  }

  return PortalDevLogBridge.DefaultUrl;
}

export async function sendPortalProofTraceLog(
  message: LogMessage,
  proofTrace: PortalProofTraceOptions,
  fields: LogFields = {},
  endpoint = resolvePortalDevLogBridgeUrl()
): Promise<boolean> {
  if (endpoint.length === 0) {
    return false;
  }

  const config = resolvePortalProofTraceConfig();
  const effectiveProofId = proofTrace.proofId ?? config.proofId;
  if ((!config.enabled && effectiveProofId == null) || !proofTraceAllowedForPortal(config)) {
    return false;
  }

  const entry = createPortalBridgeEntry(message, fields, {
    ...proofTrace,
    ...(effectiveProofId != null ? { proofId: effectiveProofId } : {}),
    ...(proofTrace.scope != null
      ? { scope: proofTrace.scope }
      : config.scope != null
        ? { scope: config.scope }
        : {}),
  });
  return sendPortalLoggerMessage(
    message,
    entry.dataFields,
    entry.runtimeConfig,
    endpoint,
    getStackTrace()
  );
}

export function resolvePortalProofTraceConfig(
  runtime: Record<string, unknown> = globalThis as Record<string, unknown>
): PortalProofTraceConfig {
  const viteEnv = getPortalEnv();
  const envEnabled = parseBoolean(viteEnv[PortalProofTrace.EnabledEnv]);
  const globalEnabled = parseBoolean(runtime[PortalProofTrace.EnabledGlobalKey]);
  const proofId = firstNonEmptyString(
    viteEnv[PortalProofTrace.IdEnv],
    runtime[PortalProofTrace.IdGlobalKey]
  );
  const scope = firstNonEmptyString(
    viteEnv[PortalProofTrace.ScopeEnv],
    runtime[PortalProofTrace.ScopeGlobalKey]
  );
  return {
    enabled: envEnabled || globalEnabled || proofId != null,
    proofId,
    scope,
    sources: parseList(
      firstNonEmptyString(
        viteEnv[PortalProofTrace.SourcesEnv],
        runtime[PortalProofTrace.SourcesGlobalKey]
      )
    ),
    level: firstNonEmptyString(
      viteEnv[PortalProofTrace.LevelEnv],
      runtime[PortalProofTrace.LevelGlobalKey]
    ),
  };
}

function createPortalBridgeEntry(
  message: LogMessage,
  fields: LogFields,
  proofTrace: PortalProofTraceOptions | null
) {
  return {
    dataFields: mergeProofTraceFields(fields, proofTrace),
    runtimeConfig: {
      runId: proofTrace?.proofId ?? PORTAL_DEV_LOG_RUN_ID,
      testName: PortalDevLogBridge.PortalTestName,
      scope: resolveProofTraceScope(proofTrace?.scope),
      runType: RunType.Single,
      origin: TestLogOrigin.Portal,
      environment: PortalDevLogBridge.PortalEnvironment,
      correlationId: proofTrace?.correlationId ?? null,
    },
  };
}

function mergeProofTraceFields(fields: LogFields, proofTrace: PortalProofTraceOptions | null): LogFields {
  if (proofTrace == null) {
    return fields;
  }

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

function resolveProofTraceScope(scope: string | undefined) {
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
  return config.sources.length === 0 || config.sources.includes('portal');
}

function parseBoolean(value: unknown): boolean {
  if (typeof value !== 'string') {
    return value === true;
  }
  const normalized = value.trim().toLowerCase();
  return normalized === 'true' || normalized === '1' || normalized === 'yes' || normalized === 'on';
}

function parseList(value: string | null): string[] {
  if (value == null || value.trim().length === 0) {
    return [];
  }
  return value
    .split(',')
    .map((entry) => entry.trim())
    .filter((entry) => entry.length > 0);
}

function firstNonEmptyString(...values: unknown[]): string | null {
  for (const value of values) {
    if (typeof value === 'string' && value.trim().length > 0) {
      return value.trim();
    }
  }
  return null;
}

function getPortalEnv(): Record<string, string | undefined> {
  const env = import.meta.env;
  return env != null ? env : {};
}

async function sendPortalLoggerMessage(
  message: LogMessage,
  fields: LogFields,
  runtimeConfig: {
    readonly runId: string;
    readonly testName: string;
    readonly scope: TestLogScope;
    readonly runType: RunType;
    readonly origin: TestLogOrigin;
    readonly environment: string;
    readonly correlationId: string | null;
  } | null,
  endpoint: string,
  stackTrace: ReturnType<typeof getStackTrace>
): Promise<boolean> {
  portalLogger.register(import.meta.url);
  portalLogger.configure({
    bridgeEndpoint: endpoint,
    runId: runtimeConfig?.runId ?? PORTAL_DEV_LOG_RUN_ID,
    testName: runtimeConfig?.testName ?? PortalDevLogBridge.PortalTestName,
    scope: runtimeConfig?.scope ?? TestLogScope.ParentPortal,
    runType: runtimeConfig?.runType ?? RunType.Single,
    origin: runtimeConfig?.origin ?? TestLogOrigin.Portal,
    environment: runtimeConfig?.environment ?? PortalDevLogBridge.PortalEnvironment,
    correlationId: runtimeConfig?.correlationId ?? null,
  });

  try {
    portalLogger.logInfo(message, stackTrace, fields, true);
    await portalLogger.flush();
    return true;
  } catch {
    return false;
  }
}

const PORTAL_DEV_LOG_RUN_ID = `portal-dev-${globalThis.crypto?.randomUUID?.() ?? Date.now()}`;
