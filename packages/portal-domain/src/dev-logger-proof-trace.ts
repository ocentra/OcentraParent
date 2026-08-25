import type { GeneratedLogFields as LogFields } from '@ocentra-parent/logging-domain/generated/logging-contracts';
import {
  PortalRunType,
  PortalCompatibilityDefaultConfig,
  PortalTestLogOrigin,
  PortalTestLogScope,
  type PortalLoggerRuntime,
  type PortalLoggerRuntimeConfigContract,
  type PortalProofTraceConfigContract,
  type PortalProofTraceOptionsContract,
} from './dev-logger-contracts';
import { custodiedPortalLogFields } from './dev-logger-custody';

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

const PortalProofTraceScopeByValue: Record<string, PortalTestLogScope> = {
  [PortalTestLogScope.ParentAgent]: PortalTestLogScope.ParentAgent,
  [PortalTestLogScope.ParentPortal]: PortalTestLogScope.ParentPortal,
  [PortalTestLogScope.ParentCloudflare]: PortalTestLogScope.ParentCloudflare,
  [PortalTestLogScope.ParentCodex]: PortalTestLogScope.ParentCodex,
  [PortalTestLogScope.ParentTest]: PortalTestLogScope.ParentTest,
};

const PortalBooleanLookup: Record<string, boolean> = { true: true, '1': true, yes: true, on: true };

interface ResolvedPortalProofTraceOptions {
  readonly proofId: string | null;
  readonly traceStep: string;
  readonly eventType: string;
  readonly action: string | null;
  readonly command: string | null;
  readonly status: string | null;
  readonly expectedNext: string | null;
  readonly artifactRef: string | null;
  readonly causationId: string | null;
  readonly correlationId: string | null;
  readonly scope: string | null;
}

export function resolvePortalProofTraceConfigValue(
  runtime: PortalLoggerRuntime,
  viteEnv: Record<string, string | undefined>
): PortalProofTraceConfigContract {
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

export function buildPortalProofTraceDispatch(
  fields: LogFields,
  proofTrace: PortalProofTraceOptionsContract,
  config: PortalProofTraceConfigContract,
  defaultRunId: string
): { readonly fields: LogFields; readonly runtimeConfig: PortalLoggerRuntimeConfigContract } | null {
  const resolved = resolvePortalProofTraceOptions(config, proofTrace);
  const sanitizedFields = custodiedPortalLogFields(fields);
  if (resolved == null || sanitizedFields == null) {
    return null;
  }
  return {
    fields: mergeProofTraceFields(sanitizedFields, resolved),
    runtimeConfig: {
      runId: resolved.proofId ?? defaultRunId,
      testName: PortalCompatibilityDefaultConfig.testName,
      scope: PortalProofTraceScopeByValue[resolved.scope ?? ''] ?? PortalTestLogScope.ParentPortal,
      runType: PortalRunType.Single,
      origin: PortalTestLogOrigin.Portal,
      environment: PortalCompatibilityDefaultConfig.environment,
      correlationId: resolved.correlationId,
    },
  };
}

function resolvePortalProofTraceOptions(
  config: PortalProofTraceConfigContract,
  proofTrace: PortalProofTraceOptionsContract
): ResolvedPortalProofTraceOptions | null {
  const proofId = proofTrace.proofId ?? config.proofId;
  if (!(config.enabled || proofId != null) || !proofTraceAllowedForPortal(config)) {
    return null;
  }
  return {
    proofId,
    traceStep: proofTrace.traceStep,
    eventType: proofTrace.eventType,
    action: optionalText(proofTrace.action),
    command: optionalText(proofTrace.command),
    status: optionalText(proofTrace.status),
    expectedNext: optionalText(proofTrace.expectedNext),
    artifactRef: optionalText(proofTrace.artifactRef),
    causationId: optionalText(proofTrace.causationId),
    correlationId: optionalText(proofTrace.correlationId),
    scope: optionalText(proofTrace.scope) ?? config.scope,
  };
}

function optionalText(value: string | undefined): string | null {
  return value ?? null;
}

function mergeProofTraceFields(fields: LogFields, proofTrace: ResolvedPortalProofTraceOptions): LogFields {
  return {
    ...fields,
    proofId: proofTrace.proofId,
    traceStep: proofTrace.traceStep,
    eventType: proofTrace.eventType,
    action: proofTrace.action,
    command: proofTrace.command,
    status: proofTrace.status,
    expectedNext: proofTrace.expectedNext,
    artifactRef: proofTrace.artifactRef,
    causationId: proofTrace.causationId,
  };
}

function proofTraceAllowedForPortal(config: PortalProofTraceConfigContract): boolean {
  return config.sources.length === 0 || config.sources.includes(PortalProofTrace.AllowedSourcePortal);
}

function parseBoolean(value: unknown): boolean {
  return value === true || (typeof value === 'string' && PortalBooleanLookup[value.trim().toLowerCase()] === true);
}

function parseList(value: string | null): string[] {
  const normalized = value?.trim() ?? '';
  return normalized.length === 0
    ? []
    : normalized
        .split(',')
        .map((entry) => entry.trim())
        .filter((entry) => entry.length > 0);
}

function firstNonEmptyString(...values: unknown[]): string | null {
  for (const value of values) {
    if (typeof value === 'string') {
      const normalized = value.trim();
      if (normalized.length > 0 && normalized.length <= 4_096) {
        return normalized;
      }
    }
  }
  return null;
}
