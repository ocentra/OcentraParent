import {
  GeneratedDevLogBridge as DevLogBridge,
  type GeneratedStackTrace as StackTrace,
} from '@ocentra-parent/logging-domain/generated/logging-contracts';

export const PortalTestLogScope = {
  ParentAgent: 'parent-agent',
  ParentPortal: 'parent-portal',
  ParentCloudflare: 'parent-cloudflare',
  ParentCodex: 'parent-codex',
  ParentTest: 'parent-test',
} as const;

export const PortalRunType = {
  Single: 'single',
} as const;

export const PortalTestLogOrigin = {
  Portal: 'portal',
} as const;

export type PortalTestLogScope = (typeof PortalTestLogScope)[keyof typeof PortalTestLogScope];
export type PortalRunType = (typeof PortalRunType)[keyof typeof PortalRunType];
export type PortalTestLogOrigin = (typeof PortalTestLogOrigin)[keyof typeof PortalTestLogOrigin];
export type PortalLoggerRuntime = Record<string, unknown>;

export interface PortalProofTraceOptionsContract {
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

export interface PortalProofTraceConfigContract {
  readonly enabled: boolean;
  readonly proofId: string | null;
  readonly scope: string | null;
  readonly sources: readonly string[];
  readonly level: string | null;
}

export interface PortalLoggerRuntimeConfigContract {
  readonly runId: string;
  readonly testName: string;
  readonly scope: PortalTestLogScope;
  readonly runType: PortalRunType;
  readonly origin: PortalTestLogOrigin;
  readonly environment: string;
  readonly correlationId: string | null;
}

export interface PortalLoggerDispatchContextContract {
  readonly endpoint: string;
  readonly runtime: PortalLoggerRuntime;
  readonly stackTrace: StackTrace;
  readonly moduleUrl: string;
}

export const PortalCompatibilityDefaultConfig: PortalLoggerRuntimeConfigContract = {
  runId: '',
  testName: DevLogBridge.PortalTestName,
  scope: PortalTestLogScope.ParentPortal,
  runType: PortalRunType.Single,
  origin: PortalTestLogOrigin.Portal,
  environment: DevLogBridge.PortalEnvironment,
  correlationId: null,
};

export function resolvePortalCompatibilityUrl(runtime: PortalLoggerRuntime, route: string): string | null {
  const location = runtime['location'] as { readonly origin?: unknown } | undefined;
  const origin = typeof location?.origin === 'string' ? location.origin.trim() : '';
  return origin.length === 0 ? null : `${trimTrailingSolidus(origin)}${route}`;
}

function trimTrailingSolidus(value: string): string {
  let end = value.length;
  while (end > 0 && value.charCodeAt(end - 1) === 47) {
    end -= 1;
  }
  return value.slice(0, end);
}
