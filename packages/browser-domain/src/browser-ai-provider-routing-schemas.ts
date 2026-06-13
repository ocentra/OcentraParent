import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema,
  NonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { ActivityEvidenceIdSchema, ActivityTimestampSchema } from '@ocentra-parent/evidence-domain/primitives';
import { BrowserCustodyLabelSchema } from './browser-schemas';
import {
  BrowserAiModelRuntimePreferenceSchema,
  BrowserAiModelRuntimeRefSchema,
  BrowserAiRequestedTaskSchema,
  type BrowserUrlAiAnalysisInput,
  BrowserUrlAiAnalysisInputSchema,
  BrowserUrlAiAnalysisRequestIdSchema,
} from './browser-ai-analysis-schemas';
const OptionalProviderTextSchema = Schema.Union(NonEmptyStringSchema, Schema.Null);
const OptionalModelRuntimeRefSchema = Schema.Union(BrowserAiModelRuntimeRefSchema, Schema.Null);

export const BrowserAiProviderRouteSchemaVersion = 1;

export const BrowserAiProviderRouteIdSchema = withParser(
  brandedNonEmptyStringSchema('BrowserAiProviderRouteId')
);
export const BrowserAiProviderIdSchema = withParser(
  brandedNonEmptyStringSchema('BrowserAiProviderId')
);

export const BrowserAiProviderKindSchema = withParser(
  Schema.Literal('child-device-local-ai', 'family-ai-hub', 'parent-approved-remote-ai', 'metadata-only', 'no-ai')
);
export const BrowserAiProviderRouteModeSchema = withParser(
  Schema.Literal(
    'local-only',
    'local-then-family-hub',
    'local-then-parent-approved-remote',
    'metadata-only',
    'parent-review-when-unavailable'
  )
);
export const BrowserAiProviderCapabilityStateSchema = withParser(
  Schema.Literal('available', 'disabled-by-parent', 'model-missing', 'provider-unavailable', 'resource-exhausted')
);
export const BrowserAiProviderDegradedStateSchema = withParser(
  Schema.Literal(
    'none',
    'disabled-by-parent',
    'model-missing',
    'provider-unavailable',
    'resource-exhausted',
    'unsupported-task',
    'custody-unsafe',
    'manual-required'
  )
);
export const BrowserAiProviderExecutionStateSchema = withParser(
  Schema.Literal('selected', 'degraded', 'manual-required', 'unavailable')
);

const SupportedTasksSchema = Schema.Array(BrowserAiRequestedTaskSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected at least one browser AI supported task')
);
const AuditEvidenceIdsSchema = Schema.Array(ActivityEvidenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected at least one browser AI provider audit evidence id')
);
const DegradedStatesSchema = Schema.Array(BrowserAiProviderDegradedStateSchema);

const BrowserAiProviderCapabilityBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(BrowserAiProviderRouteSchemaVersion),
  providerId: BrowserAiProviderIdSchema,
  checkedAt: ActivityTimestampSchema,
  providerKind: BrowserAiProviderKindSchema,
  capabilityState: BrowserAiProviderCapabilityStateSchema,
  supportedTasks: SupportedTasksSchema,
  modelRuntimeRef: OptionalModelRuntimeRefSchema,
  custodyLabel: BrowserCustodyLabelSchema,
  noRetention: Schema.Boolean,
  localOnly: Schema.Boolean,
  parentApprovedRemoteEnabled: Schema.Boolean,
  canRunOnChildDevice: Schema.Boolean,
  degradedStates: DegradedStatesSchema,
  unavailableReason: OptionalProviderTextSchema,
});
export const BrowserAiProviderCapabilitySchema = withParser(
  BrowserAiProviderCapabilityBaseSchema.pipe(
    Schema.filter(
      (value) =>
        browserAiProviderCapabilityIsConsistent(value) ||
        'Expected browser AI provider capability to preserve local custody, retention, and availability boundaries'
    )
  )
);

const BrowserAiProviderRouteBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(BrowserAiProviderRouteSchemaVersion),
  routeId: BrowserAiProviderRouteIdSchema,
  requestId: BrowserUrlAiAnalysisRequestIdSchema,
  routedAt: ActivityTimestampSchema,
  routeMode: BrowserAiProviderRouteModeSchema,
  modelRuntimePreference: BrowserAiModelRuntimePreferenceSchema,
  providerKind: BrowserAiProviderKindSchema,
  capability: BrowserAiProviderCapabilitySchema,
  executionState: BrowserAiProviderExecutionStateSchema,
  selectedRuntimeRef: OptionalModelRuntimeRefSchema,
  degradedStates: DegradedStatesSchema,
  auditEvidenceIds: AuditEvidenceIdsSchema,
  dataScopeVisible: Schema.Boolean,
  retentionVisible: Schema.Boolean,
  custodyVisible: Schema.Boolean,
  providerVisible: Schema.Boolean,
  noRetentionVisible: Schema.Boolean,
  parentExplicitRemoteApproval: Schema.Boolean,
  remoteDefaultForBlocking: Schema.Boolean,
  remoteCanOverrideStricterLocalRules: Schema.Boolean,
  remoteOutageDisablesLocalSafety: Schema.Boolean,
});
export const BrowserAiProviderRouteSchema = withParser(
  BrowserAiProviderRouteBaseSchema.pipe(
    Schema.filter(
      (value) =>
        browserAiProviderRouteIsConsistent(value) ||
        'Expected browser AI provider route to be auditable, visible, and non-remote-default'
    )
  )
);

const BrowserAiLocalProviderRouteRequestSchema = withParser(
  Schema.Struct({
    routeId: BrowserAiProviderRouteIdSchema,
    routedAt: ActivityTimestampSchema,
    input: BrowserUrlAiAnalysisInputSchema,
    capability: BrowserAiProviderCapabilitySchema,
    auditEvidenceIds: AuditEvidenceIdsSchema,
  })
);

export const decodeBrowserAiProviderCapability = Schema.decodeUnknownSync(BrowserAiProviderCapabilitySchema);
export const decodeBrowserAiProviderRoute = Schema.decodeUnknownSync(BrowserAiProviderRouteSchema);

export function planBrowserAiLocalProviderRoute(
  request: Infer<typeof BrowserAiLocalProviderRouteRequestSchema>
): BrowserAiProviderRoute {
  const parsed = BrowserAiLocalProviderRouteRequestSchema.parse(request);
  const selected = localProviderCanServe(parsed.input, parsed.capability);
  const degradedStates = selected ? [] : routeDegradedStatesFor(parsed.input, parsed.capability);

  return BrowserAiProviderRouteSchema.parse({
    schemaVersion: BrowserAiProviderRouteSchemaVersion,
    routeId: parsed.routeId,
    requestId: parsed.input.requestId,
    routedAt: parsed.routedAt,
    routeMode: routeModeFor(parsed.input),
    modelRuntimePreference: parsed.input.modelRuntimePreference,
    providerKind: parsed.capability.providerKind,
    capability: parsed.capability,
    executionState: selected ? 'selected' : executionStateFor(parsed.capability),
    selectedRuntimeRef: selected ? parsed.capability.modelRuntimeRef : null,
    degradedStates,
    auditEvidenceIds: parsed.auditEvidenceIds,
    dataScopeVisible: true,
    retentionVisible: true,
    custodyVisible: true,
    providerVisible: true,
    noRetentionVisible: true,
    parentExplicitRemoteApproval: false,
    remoteDefaultForBlocking: false,
    remoteCanOverrideStricterLocalRules: false,
    remoteOutageDisablesLocalSafety: false,
  });
}

export type BrowserAiProviderKind = Infer<typeof BrowserAiProviderKindSchema>;
export type BrowserAiProviderCapability = Infer<typeof BrowserAiProviderCapabilitySchema>;
export type BrowserAiProviderDegradedState = Infer<typeof BrowserAiProviderDegradedStateSchema>;
export type BrowserAiProviderRoute = Infer<typeof BrowserAiProviderRouteSchema>;

function browserAiProviderCapabilityIsConsistent(value: Infer<typeof BrowserAiProviderCapabilityBaseSchema>) {
  if (value.providerKind !== 'child-device-local-ai') {
    return value.parentApprovedRemoteEnabled && value.canRunOnChildDevice === false;
  }
  if (!value.localOnly || value.parentApprovedRemoteEnabled || !value.canRunOnChildDevice || !value.noRetention) {
    return false;
  }
  if (value.capabilityState === 'available') {
    return value.modelRuntimeRef !== null && value.degradedStates.length === 0 && value.unavailableReason === null;
  }
  return value.modelRuntimeRef === null && value.degradedStates.length > 0 && value.unavailableReason !== null;
}

function browserAiProviderRouteIsConsistent(value: Infer<typeof BrowserAiProviderRouteBaseSchema>) {
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

function localProviderCanServe(input: BrowserUrlAiAnalysisInput, capability: BrowserAiProviderCapability) {
  return (
    capability.providerKind === 'child-device-local-ai' &&
    capability.capabilityState === 'available' &&
    capability.modelRuntimeRef !== null &&
    capability.supportedTasks.includes(input.requestedTask) &&
    (input.modelRuntimePreference === 'local-only' || input.modelRuntimePreference === 'local-preferred')
  );
}

function routeDegradedStatesFor(input: BrowserUrlAiAnalysisInput, capability: BrowserAiProviderCapability) {
  if (!capability.supportedTasks.includes(input.requestedTask)) {
    return ['unsupported-task'] as const;
  }
  if (input.modelRuntimePreference === 'manual-required') {
    return ['manual-required'] as const;
  }
  return capability.degradedStates.length > 0 ? capability.degradedStates : (['provider-unavailable'] as const);
}

function routeModeFor(input: BrowserUrlAiAnalysisInput) {
  return input.modelRuntimePreference === 'local-only' ? 'local-only' : 'parent-review-when-unavailable';
}

function executionStateFor(capability: BrowserAiProviderCapability) {
  return capability.capabilityState === 'provider-unavailable' ? 'unavailable' : 'manual-required';
}

function routeVisibilityIsComplete(value: Infer<typeof BrowserAiProviderRouteBaseSchema>) {
  return (
    value.dataScopeVisible &&
    value.retentionVisible &&
    value.custodyVisible &&
    value.providerVisible &&
    value.noRetentionVisible
  );
}

function routeClaimsUnsafeRemoteAuthority(value: Infer<typeof BrowserAiProviderRouteBaseSchema>) {
  return (
    value.remoteDefaultForBlocking || value.remoteCanOverrideStricterLocalRules || value.remoteOutageDisablesLocalSafety
  );
}

