type BrowserAiProviderCapabilityShape = {
  readonly providerKind: string;
  readonly capabilityState: string;
  readonly supportedTasks: readonly string[];
  readonly modelRuntimeRef: string | null;
  readonly noRetention: boolean;
  readonly localOnly: boolean;
  readonly parentApprovedRemoteEnabled: boolean;
  readonly canRunOnChildDevice: boolean;
  readonly degradedStates: readonly string[];
  readonly unavailableReason: string | null;
};

type BrowserAiProviderRouteShape = {
  readonly providerKind: string;
  readonly capability: BrowserAiProviderCapabilityShape;
  readonly executionState: string;
  readonly selectedRuntimeRef: string | null;
  readonly degradedStates: readonly string[];
  readonly dataScopeVisible: boolean;
  readonly retentionVisible: boolean;
  readonly custodyVisible: boolean;
  readonly providerVisible: boolean;
  readonly noRetentionVisible: boolean;
  readonly remoteDefaultForBlocking: boolean;
  readonly remoteCanOverrideStricterLocalRules: boolean;
  readonly remoteOutageDisablesLocalSafety: boolean;
};

type BrowserAiProviderRouteRequestShape = {
  readonly routeId: string;
  readonly routedAt: unknown;
  readonly input: {
    readonly requestId: string;
    readonly requestedTask: string;
    readonly modelRuntimePreference: string;
  };
  readonly capability: BrowserAiProviderCapabilityShape;
  readonly auditEvidenceIds: readonly unknown[];
};

export function browserAiProviderCapabilityIsConsistent(value: BrowserAiProviderCapabilityShape) {
  return value.providerKind !== 'child-device-local-ai'
    ? value.parentApprovedRemoteEnabled && value.canRunOnChildDevice === false
    : value.localOnly &&
        !value.parentApprovedRemoteEnabled &&
        value.canRunOnChildDevice &&
        value.noRetention &&
        (value.capabilityState === 'available'
          ? value.modelRuntimeRef !== null && value.degradedStates.length === 0 && value.unavailableReason === null
          : value.modelRuntimeRef === null && value.degradedStates.length > 0 && value.unavailableReason !== null);
}

export function browserAiProviderRouteIsConsistent(value: BrowserAiProviderRouteShape) {
  return (
    routeVisibilityIsComplete(value) &&
    !routeClaimsUnsafeRemoteAuthority(value) &&
    value.providerKind === value.capability.providerKind &&
    (value.executionState === 'selected'
      ? value.capability.capabilityState === 'available' &&
          value.selectedRuntimeRef !== null &&
          value.degradedStates.length === 0
      : value.selectedRuntimeRef === null && value.degradedStates.length > 0)
  );
}

export function buildBrowserAiLocalProviderRoute(request: BrowserAiProviderRouteRequestShape) {
  const selected = localProviderCanServe(request.input, request.capability);

  return {
    schemaVersion: 1,
    routeId: request.routeId,
    requestId: request.input.requestId,
    routedAt: request.routedAt,
    routeMode: request.input.modelRuntimePreference === 'local-only' ? 'local-only' : 'parent-review-when-unavailable',
    modelRuntimePreference: request.input.modelRuntimePreference,
    providerKind: request.capability.providerKind,
    capability: request.capability,
    executionState: selected
      ? 'selected'
      : request.capability.capabilityState === 'provider-unavailable'
        ? 'unavailable'
        : 'manual-required',
    selectedRuntimeRef: selected ? request.capability.modelRuntimeRef : null,
    degradedStates: selected
      ? []
      : !request.capability.supportedTasks.includes(request.input.requestedTask)
        ? ['unsupported-task']
        : request.input.modelRuntimePreference === 'manual-required'
          ? ['manual-required']
          : request.capability.degradedStates.length > 0
            ? request.capability.degradedStates
            : ['provider-unavailable'],
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
  } as const;
}

function localProviderCanServe(
  input: BrowserAiProviderRouteRequestShape['input'],
  capability: BrowserAiProviderCapabilityShape
) {
  return (
    capability.providerKind === 'child-device-local-ai' &&
    capability.capabilityState === 'available' &&
    capability.modelRuntimeRef !== null &&
    capability.supportedTasks.includes(input.requestedTask) &&
    (input.modelRuntimePreference === 'local-only' || input.modelRuntimePreference === 'local-preferred')
  );
}

function routeVisibilityIsComplete(value: BrowserAiProviderRouteShape) {
  return (
    value.dataScopeVisible &&
    value.retentionVisible &&
    value.custodyVisible &&
    value.providerVisible &&
    value.noRetentionVisible
  );
}

function routeClaimsUnsafeRemoteAuthority(value: BrowserAiProviderRouteShape) {
  return (
    value.remoteDefaultForBlocking || value.remoteCanOverrideStricterLocalRules || value.remoteOutageDisablesLocalSafety
  );
}
