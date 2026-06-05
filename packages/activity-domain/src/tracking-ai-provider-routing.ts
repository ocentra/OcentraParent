import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityDeviceIdSchema, ActivityEvidenceIdSchema, ActivityTimestampSchema } from './primitives';
import { TrackingCustodyLabelSchema, TrackingReasonCodeSchema } from './tracking-primitives';

const TrackingAiRouteText = Schema.String.pipe(Schema.minLength(1));
const OptionalRouteTextSchema = Schema.Union(TrackingAiRouteText, Schema.Null);
const OptionalModelRuntimeRefSchema = OptionalRouteTextSchema;

export const TrackingAiProviderRouteSchemaVersion = 1;

export const TrackingAiProviderRouteIdSchema = withParser(
  TrackingAiRouteText.pipe(Schema.brand('TrackingAiProviderRouteId'))
);
export const TrackingAiProviderIdSchema = withParser(TrackingAiRouteText.pipe(Schema.brand('TrackingAiProviderId')));
export const TrackingAiChildProfileRefSchema = withParser(
  TrackingAiRouteText.pipe(Schema.brand('TrackingAiChildProfileRef'))
);
export const TrackingAiParentRuleRefSchema = withParser(
  TrackingAiRouteText.pipe(Schema.brand('TrackingAiParentRuleRef'))
);
export const TrackingAiPolicyVersionRefSchema = withParser(
  TrackingAiRouteText.pipe(Schema.brand('TrackingAiPolicyVersionRef'))
);

export const TrackingAiProviderKindSchema = withParser(
  Schema.Literal(
    'child-device-local-ai',
    'parent-device-local-ai',
    'family-ai-hub',
    'parent-approved-remote-ai',
    'metadata-only',
    'no-ai'
  )
);
export const TrackingAiProviderRouteModeSchema = withParser(
  Schema.Literal(
    'child-local-default',
    'parent-local-review',
    'family-hub-local-lan',
    'parent-approved-remote',
    'metadata-only',
    'no-ai',
    'manual-required'
  )
);
export const TrackingAiProviderCapabilityStateSchema = withParser(
  Schema.Literal(
    'available',
    'disabled-by-parent',
    'model-missing',
    'provider-unavailable',
    'resource-exhausted',
    'unsupported-platform',
    'manual-required'
  )
);
export const TrackingAiProviderDegradedStateSchema = withParser(
  Schema.Literal(
    'disabled-by-parent',
    'model-missing',
    'provider-unavailable',
    'resource-exhausted',
    'unsupported-task',
    'custody-unsafe',
    'parent-approval-missing',
    'remote-disabled',
    'manual-required',
    'platform-unproved',
    'metadata-only',
    'no-ai'
  )
);
export const TrackingAiProviderExecutionStateSchema = withParser(
  Schema.Literal('selected', 'degraded', 'manual-required', 'unavailable', 'metadata-only', 'no-ai')
);
export const TrackingAiRequestedTaskSchema = withParser(
  Schema.Literal('location-safety', 'expected-place-safety', 'nearby-place-context', 'geofence-risk', 'parent-summary')
);
export const TrackingAiModelRuntimePreferenceSchema = withParser(
  Schema.Literal(
    'child-local-required',
    'local-preferred',
    'parent-approved-remote-allowed',
    'metadata-only',
    'no-ai',
    'manual-required'
  )
);

const SupportedTasksSchema = Schema.Array(TrackingAiRequestedTaskSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected at least one tracking AI provider supported task')
);
const EvidenceIdsSchema = Schema.Array(ActivityEvidenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected tracking AI provider route to cite evidence ids')
);
const ParentRuleRefsSchema = Schema.Array(TrackingAiParentRuleRefSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected tracking AI provider route to cite parent rule refs')
);
const DegradedStatesSchema = Schema.Array(TrackingAiProviderDegradedStateSchema);

const TrackingAiProviderCapabilityBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(TrackingAiProviderRouteSchemaVersion),
  providerId: TrackingAiProviderIdSchema,
  checkedAt: ActivityTimestampSchema,
  providerKind: TrackingAiProviderKindSchema,
  capabilityState: TrackingAiProviderCapabilityStateSchema,
  supportedTasks: SupportedTasksSchema,
  modelRuntimeRef: OptionalModelRuntimeRefSchema,
  custodyLabel: TrackingCustodyLabelSchema,
  noRetention: Schema.Boolean,
  localOnly: Schema.Boolean,
  parentApprovedRemoteEnabled: Schema.Boolean,
  canRunOnChildDevice: Schema.Boolean,
  canRunOnParentDevice: Schema.Boolean,
  familyHubLanOnly: Schema.Boolean,
  degradedStates: DegradedStatesSchema,
  unavailableReason: OptionalRouteTextSchema,
});

export const TrackingAiProviderCapabilitySchema = withParser(
  TrackingAiProviderCapabilityBaseSchema.pipe(
    Schema.filter(
      (value) =>
        trackingAiProviderCapabilityIsConsistent(value) ||
        'Expected tracking AI provider capability to preserve custody, retention, and availability boundaries'
    )
  )
);

const TrackingAiProviderRouteBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(TrackingAiProviderRouteSchemaVersion),
  routeId: TrackingAiProviderRouteIdSchema,
  routedAt: ActivityTimestampSchema,
  requestedTask: TrackingAiRequestedTaskSchema,
  modelRuntimePreference: TrackingAiModelRuntimePreferenceSchema,
  providerKind: TrackingAiProviderKindSchema,
  capability: TrackingAiProviderCapabilitySchema,
  executionState: TrackingAiProviderExecutionStateSchema,
  selectedRuntimeRef: OptionalModelRuntimeRefSchema,
  degradedStates: DegradedStatesSchema,
  deviceId: ActivityDeviceIdSchema,
  childProfileRef: TrackingAiChildProfileRefSchema,
  policyVersionRef: TrackingAiPolicyVersionRefSchema,
  evidenceIds: EvidenceIdsSchema,
  parentRuleRefs: ParentRuleRefsSchema,
  custodyLabel: TrackingCustodyLabelSchema,
  reasonCodes: Schema.Array(TrackingReasonCodeSchema),
  dataScopeVisible: Schema.Boolean,
  retentionVisible: Schema.Boolean,
  custodyVisible: Schema.Boolean,
  providerVisible: Schema.Boolean,
  noRetentionVisible: Schema.Boolean,
  parentExplicitRemoteApproval: Schema.Boolean,
  remoteDefaultForBlocking: Schema.Boolean,
  remoteCanOverrideStricterLocalRules: Schema.Boolean,
  remoteOutageDisablesLocalSafety: Schema.Boolean,
  assistantCanWritePolicy: Schema.Boolean,
  aiCanTriggerAlertDirectly: Schema.Boolean,
  aiIsFinalAuthority: Schema.Boolean,
  metadataOnly: Schema.Boolean,
  noAi: Schema.Boolean,
  manualRequired: Schema.Boolean,
});

export const TrackingAiProviderRouteSchema = withParser(
  TrackingAiProviderRouteBaseSchema.pipe(
    Schema.filter(
      (value) =>
        trackingAiProviderRouteIsConsistent(value) ||
        'Expected tracking AI provider route to stay visible, auditable, and unable to claim AI authority'
    )
  )
);

const TrackingAiProviderRouteRequestSchema = withParser(
  Schema.Struct({
    routeId: TrackingAiProviderRouteIdSchema,
    routedAt: ActivityTimestampSchema,
    requestedTask: TrackingAiRequestedTaskSchema,
    modelRuntimePreference: TrackingAiModelRuntimePreferenceSchema,
    deviceId: ActivityDeviceIdSchema,
    childProfileRef: TrackingAiChildProfileRefSchema,
    policyVersionRef: TrackingAiPolicyVersionRefSchema,
    evidenceIds: EvidenceIdsSchema,
    parentRuleRefs: ParentRuleRefsSchema,
    capability: TrackingAiProviderCapabilitySchema,
    parentExplicitRemoteApproval: Schema.Boolean,
    reasonCodes: Schema.Array(TrackingReasonCodeSchema),
  })
);

export const decodeTrackingAiProviderCapability = Schema.decodeUnknownSync(TrackingAiProviderCapabilitySchema);
export const decodeTrackingAiProviderRoute = Schema.decodeUnknownSync(TrackingAiProviderRouteSchema);

export function planTrackingAiProviderRoute(
  request: Infer<typeof TrackingAiProviderRouteRequestSchema>
): TrackingAiProviderRoute {
  const parsed = TrackingAiProviderRouteRequestSchema.parse(request);
  const selected = trackingAiProviderCanServe(parsed);
  const executionState = selected ? 'selected' : executionStateFor(parsed);
  const degradedStates = selected ? [] : routeDegradedStatesFor(parsed);

  return TrackingAiProviderRouteSchema.parse({
    schemaVersion: TrackingAiProviderRouteSchemaVersion,
    routeId: parsed.routeId,
    routedAt: parsed.routedAt,
    requestedTask: parsed.requestedTask,
    modelRuntimePreference: parsed.modelRuntimePreference,
    providerKind: parsed.capability.providerKind,
    capability: parsed.capability,
    executionState,
    selectedRuntimeRef: selected ? parsed.capability.modelRuntimeRef : null,
    degradedStates,
    deviceId: parsed.deviceId,
    childProfileRef: parsed.childProfileRef,
    policyVersionRef: parsed.policyVersionRef,
    evidenceIds: parsed.evidenceIds,
    parentRuleRefs: parsed.parentRuleRefs,
    custodyLabel: parsed.capability.custodyLabel,
    reasonCodes: parsed.reasonCodes,
    dataScopeVisible: true,
    retentionVisible: true,
    custodyVisible: true,
    providerVisible: true,
    noRetentionVisible: true,
    parentExplicitRemoteApproval: parsed.parentExplicitRemoteApproval,
    remoteDefaultForBlocking: false,
    remoteCanOverrideStricterLocalRules: false,
    remoteOutageDisablesLocalSafety: false,
    assistantCanWritePolicy: false,
    aiCanTriggerAlertDirectly: false,
    aiIsFinalAuthority: false,
    metadataOnly: executionState === 'metadata-only',
    noAi: executionState === 'no-ai',
    manualRequired: executionState === 'manual-required',
  });
}

export type TrackingAiProviderCapability = Infer<typeof TrackingAiProviderCapabilitySchema>;
export type TrackingAiProviderDegradedState = Infer<typeof TrackingAiProviderDegradedStateSchema>;
export type TrackingAiProviderKind = Infer<typeof TrackingAiProviderKindSchema>;
export type TrackingAiProviderRoute = Infer<typeof TrackingAiProviderRouteSchema>;

function trackingAiProviderCapabilityIsConsistent(value: Infer<typeof TrackingAiProviderCapabilityBaseSchema>) {
  if (!providerCustodyIsSafe(value)) {
    return false;
  }
  if (providerKindIsRuntime(value.providerKind)) {
    return runtimeProviderCapabilityIsConsistent(value);
  }
  return nonRuntimeProviderCapabilityIsConsistent(value);
}

function providerCustodyIsSafe(value: Infer<typeof TrackingAiProviderCapabilityBaseSchema>) {
  if (!value.noRetention) {
    return false;
  }
  if (value.providerKind === 'parent-approved-remote-ai') {
    return value.custodyLabel === 'parent-approved-cloud' && value.parentApprovedRemoteEnabled;
  }
  if (value.providerKind === 'family-ai-hub') {
    return value.custodyLabel === 'live-lan-child-agent' && value.familyHubLanOnly;
  }
  return value.custodyLabel === 'child-device-local' || value.custodyLabel === 'parent-device-cache';
}

function runtimeProviderCapabilityIsConsistent(value: Infer<typeof TrackingAiProviderCapabilityBaseSchema>) {
  if (value.capabilityState === 'available') {
    return value.modelRuntimeRef !== null && value.degradedStates.length === 0 && value.unavailableReason === null;
  }
  return value.modelRuntimeRef === null && value.degradedStates.length > 0 && value.unavailableReason !== null;
}

function nonRuntimeProviderCapabilityIsConsistent(value: Infer<typeof TrackingAiProviderCapabilityBaseSchema>) {
  return (
    value.modelRuntimeRef === null &&
    value.degradedStates.length > 0 &&
    value.unavailableReason !== null &&
    !value.parentApprovedRemoteEnabled &&
    !value.canRunOnChildDevice &&
    !value.canRunOnParentDevice
  );
}

function trackingAiProviderRouteIsConsistent(value: Infer<typeof TrackingAiProviderRouteBaseSchema>) {
  if (!routeVisibilityIsComplete(value) || routeClaimsUnsafeAuthority(value)) {
    return false;
  }
  if (value.providerKind !== value.capability.providerKind || value.custodyLabel !== value.capability.custodyLabel) {
    return false;
  }
  if (value.executionState === 'selected') {
    return selectedRouteIsConsistent(value);
  }
  return degradedRouteIsConsistent(value);
}

function selectedRouteIsConsistent(value: Infer<typeof TrackingAiProviderRouteBaseSchema>) {
  return (
    value.capability.capabilityState === 'available' &&
    value.selectedRuntimeRef !== null &&
    value.degradedStates.length === 0 &&
    !value.metadataOnly &&
    !value.noAi &&
    !value.manualRequired
  );
}

function degradedRouteIsConsistent(value: Infer<typeof TrackingAiProviderRouteBaseSchema>) {
  return (
    value.selectedRuntimeRef === null &&
    value.degradedStates.length > 0 &&
    value.metadataOnly === (value.executionState === 'metadata-only') &&
    value.noAi === (value.executionState === 'no-ai') &&
    value.manualRequired === (value.executionState === 'manual-required')
  );
}

function trackingAiProviderCanServe(request: Infer<typeof TrackingAiProviderRouteRequestSchema>) {
  return (
    request.capability.capabilityState === 'available' &&
    request.capability.modelRuntimeRef !== null &&
    request.capability.supportedTasks.includes(request.requestedTask) &&
    providerKindMatchesPreference(request) &&
    remoteApprovalIsSafe(request)
  );
}

function providerKindMatchesPreference(request: Infer<typeof TrackingAiProviderRouteRequestSchema>) {
  if (request.capability.providerKind === 'child-device-local-ai') {
    return (
      request.modelRuntimePreference === 'child-local-required' || request.modelRuntimePreference === 'local-preferred'
    );
  }
  if (request.capability.providerKind === 'parent-device-local-ai') {
    return request.modelRuntimePreference === 'local-preferred';
  }
  if (request.capability.providerKind === 'family-ai-hub') {
    return request.modelRuntimePreference === 'local-preferred';
  }
  if (request.capability.providerKind === 'parent-approved-remote-ai') {
    return request.modelRuntimePreference === 'parent-approved-remote-allowed';
  }
  return false;
}

function remoteApprovalIsSafe(request: Infer<typeof TrackingAiProviderRouteRequestSchema>) {
  return request.capability.providerKind !== 'parent-approved-remote-ai' || request.parentExplicitRemoteApproval;
}

function routeDegradedStatesFor(request: Infer<typeof TrackingAiProviderRouteRequestSchema>) {
  if (!request.capability.supportedTasks.includes(request.requestedTask)) {
    return ['unsupported-task'] as const;
  }
  if (request.modelRuntimePreference === 'metadata-only' || request.capability.providerKind === 'metadata-only') {
    return ['metadata-only'] as const;
  }
  if (request.modelRuntimePreference === 'no-ai' || request.capability.providerKind === 'no-ai') {
    return ['no-ai'] as const;
  }
  if (request.modelRuntimePreference === 'manual-required') {
    return ['manual-required'] as const;
  }
  if (request.capability.providerKind === 'parent-approved-remote-ai' && !request.parentExplicitRemoteApproval) {
    return ['parent-approval-missing'] as const;
  }
  return request.capability.degradedStates.length > 0
    ? request.capability.degradedStates
    : (['provider-unavailable'] as const);
}

function executionStateFor(request: Infer<typeof TrackingAiProviderRouteRequestSchema>) {
  if (request.modelRuntimePreference === 'metadata-only' || request.capability.providerKind === 'metadata-only') {
    return 'metadata-only';
  }
  if (request.modelRuntimePreference === 'no-ai' || request.capability.providerKind === 'no-ai') {
    return 'no-ai';
  }
  if (request.capability.capabilityState === 'provider-unavailable') {
    return 'unavailable';
  }
  return 'manual-required';
}

function routeVisibilityIsComplete(value: Infer<typeof TrackingAiProviderRouteBaseSchema>) {
  return (
    value.dataScopeVisible &&
    value.retentionVisible &&
    value.custodyVisible &&
    value.providerVisible &&
    value.noRetentionVisible
  );
}

function routeClaimsUnsafeAuthority(value: Infer<typeof TrackingAiProviderRouteBaseSchema>) {
  return (
    value.remoteDefaultForBlocking ||
    value.remoteCanOverrideStricterLocalRules ||
    value.remoteOutageDisablesLocalSafety ||
    value.assistantCanWritePolicy ||
    value.aiCanTriggerAlertDirectly ||
    value.aiIsFinalAuthority
  );
}

function providerKindIsRuntime(value: TrackingAiProviderKind) {
  return value !== 'metadata-only' && value !== 'no-ai';
}
