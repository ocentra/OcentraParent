import { type BrowserAiProviderCapability } from './browser-ai-provider-routing-capability';

export interface BrowserAiProviderRouteRequest {
  readonly routeId: string;
  readonly routedAt: unknown;
  readonly input: {
    readonly requestId: string;
    readonly modelRuntimePreference: string;
    readonly requestedTask: string;
  };
  readonly capability: BrowserAiProviderCapability;
  readonly auditEvidenceIds: readonly unknown[];
}

export interface BrowserAiProviderRoute {
  readonly schemaVersion: number;
  readonly routeId: string;
  readonly requestId: string;
  readonly routedAt: unknown;
  readonly routeMode: string;
  readonly modelRuntimePreference: string;
  readonly providerKind: string;
  readonly capability: BrowserAiProviderCapability;
  readonly executionState: string;
  readonly selectedRuntimeRef: string | null;
  readonly degradedStates: readonly string[];
  readonly auditEvidenceIds: readonly unknown[];
  readonly dataScopeVisible: boolean;
  readonly retentionVisible: boolean;
  readonly custodyVisible: boolean;
  readonly providerVisible: boolean;
  readonly noRetentionVisible: boolean;
  readonly parentExplicitRemoteApproval: boolean;
  readonly remoteDefaultForBlocking: boolean;
  readonly remoteCanOverrideStricterLocalRules: boolean;
  readonly remoteOutageDisablesLocalSafety: boolean;
}

export function browserAiProviderRouteIsConsistent(value: BrowserAiProviderRoute) {
  if (!routeVisibilityIsComplete(value) || routeClaimsUnsafeRemoteAuthority(value)) {
    return false;
  }
  if (value.providerKind !== value.capability.providerKind) {
    return false;
  }
  if (value.executionState === 'selected') {
    return (
      value.capability.capabilityState === 'available' &&
      value.selectedRuntimeRef !== null &&
      value.degradedStates.length === 0
    );
  }
  return value.selectedRuntimeRef === null && value.degradedStates.length > 0;
}

export function planBrowserAiLocalProviderRoute(request: BrowserAiProviderRouteRequest): BrowserAiProviderRoute {
  const selected = localProviderCanServe(request.input, request.capability);
  const degradedStates = selected ? [] : routeDegradedStatesFor(request.input, request.capability);

  return {
    schemaVersion: 1,
    routeId: request.routeId,
    requestId: request.input.requestId,
    routedAt: request.routedAt,
    routeMode: routeModeFor(request.input),
    modelRuntimePreference: request.input.modelRuntimePreference,
    providerKind: request.capability.providerKind,
    capability: request.capability,
    executionState: selected ? 'selected' : executionStateFor(request.capability),
    selectedRuntimeRef: selected ? request.capability.modelRuntimeRef : null,
    degradedStates,
    auditEvidenceIds: request.auditEvidenceIds,
    dataScopeVisible: true,
    retentionVisible: true,
    custodyVisible: true,
    providerVisible: true,
    noRetentionVisible: true,
    parentExplicitRemoteApproval: false,
    remoteDefaultForBlocking: false,
    remoteCanOverrideStricterLocalRules: false,
    remoteOutageDisablesLocalSafety: false,
  };
}

function localProviderCanServe(input: BrowserAiProviderRouteRequest['input'], capability: BrowserAiProviderCapability) {
  return (
    capability.providerKind === 'child-device-local-ai' &&
    capability.capabilityState === 'available' &&
    capability.modelRuntimeRef !== null &&
    capability.supportedTasks.includes(input.requestedTask) &&
    (input.modelRuntimePreference === 'local-only' || input.modelRuntimePreference === 'local-preferred')
  );
}

function routeDegradedStatesFor(input: BrowserAiProviderRouteRequest['input'], capability: BrowserAiProviderCapability) {
  if (!capability.supportedTasks.includes(input.requestedTask)) {
    return ['unsupported-task'] as const;
  }
  if (input.modelRuntimePreference === 'manual-required') {
    return ['manual-required'] as const;
  }
  return capability.degradedStates.length > 0 ? capability.degradedStates : (['provider-unavailable'] as const);
}

function routeModeFor(input: BrowserAiProviderRouteRequest['input']) {
  return input.modelRuntimePreference === 'local-only' ? 'local-only' : 'parent-review-when-unavailable';
}

function executionStateFor(capability: BrowserAiProviderCapability) {
  return capability.capabilityState === 'provider-unavailable' ? 'unavailable' : 'manual-required';
}

function routeVisibilityIsComplete(value: BrowserAiProviderRoute) {
  return (
    value.dataScopeVisible &&
    value.retentionVisible &&
    value.custodyVisible &&
    value.providerVisible &&
    value.noRetentionVisible
  );
}

function routeClaimsUnsafeRemoteAuthority(value: BrowserAiProviderRoute) {
  return (
    value.remoteDefaultForBlocking || value.remoteCanOverrideStricterLocalRules || value.remoteOutageDisablesLocalSafety
  );
}
