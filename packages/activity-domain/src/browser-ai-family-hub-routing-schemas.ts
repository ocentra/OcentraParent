import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityEvidenceIdSchema, ActivityTimestampSchema } from './primitives';
import { BrowserCustodyLabelSchema } from './browser-schemas';
import {
  BrowserAiModelRuntimeRefSchema,
  BrowserAiRequestedTaskSchema,
  BrowserUrlAiAnalysisInput,
  BrowserUrlAiAnalysisInputSchema,
  BrowserUrlAiAnalysisRequestIdSchema,
} from './browser-ai-analysis-schemas';
import {
  BrowserAiProviderRoute,
  BrowserAiProviderRouteIdSchema,
  BrowserAiProviderRouteSchema,
} from './browser-ai-provider-routing-schemas';

const NonEmptyFamilyHubText = Schema.String.pipe(Schema.minLength(1));
const OptionalFamilyHubTextSchema = Schema.Union(NonEmptyFamilyHubText, Schema.Null);
const OptionalFamilyHubRuntimeRefSchema = Schema.Union(BrowserAiModelRuntimeRefSchema, Schema.Null);

export const BrowserAiFamilyHubRouteSchemaVersion = 1;

export const BrowserAiFamilyHubRouteIdSchema = withParser(
  NonEmptyFamilyHubText.pipe(Schema.brand('BrowserAiFamilyHubRouteId'))
);
export const BrowserAiFamilyHubIdSchema = withParser(NonEmptyFamilyHubText.pipe(Schema.brand('BrowserAiFamilyHubId')));

export const BrowserAiFamilyHubCapabilityStateSchema = withParser(
  Schema.Literal('available', 'disabled-by-parent', 'hub-unavailable', 'lan-proof-missing', 'resource-exhausted')
);
export const BrowserAiFamilyHubDegradedStateSchema = withParser(
  Schema.Literal(
    'none',
    'local-provider-not-exhausted',
    'parent-disabled',
    'hub-unavailable',
    'lan-proof-missing',
    'resource-exhausted',
    'unsupported-task',
    'non-household-route',
    'manual-required'
  )
);
export const BrowserAiFamilyHubExecutionStateSchema = withParser(
  Schema.Literal('selected', 'manual-required', 'unavailable')
);

const SupportedFamilyHubTasksSchema = Schema.Array(BrowserAiRequestedTaskSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected at least one browser AI family hub task')
);
const FamilyHubAuditEvidenceIdsSchema = Schema.Array(ActivityEvidenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected at least one browser AI family hub audit evidence id')
);
const FamilyHubDegradedStatesSchema = Schema.Array(BrowserAiFamilyHubDegradedStateSchema);

const BrowserAiFamilyHubCapabilityBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(BrowserAiFamilyHubRouteSchemaVersion),
  hubId: BrowserAiFamilyHubIdSchema,
  checkedAt: ActivityTimestampSchema,
  capabilityState: BrowserAiFamilyHubCapabilityStateSchema,
  supportedTasks: SupportedFamilyHubTasksSchema,
  modelRuntimeRef: OptionalFamilyHubRuntimeRefSchema,
  householdRouteRef: OptionalFamilyHubTextSchema,
  custodyLabel: BrowserCustodyLabelSchema,
  noRetention: Schema.Boolean,
  localHouseholdOnly: Schema.Boolean,
  parentRemoteApprovalRequired: Schema.Boolean,
  childDeviceCanRunModel: Schema.Boolean,
  degradedStates: FamilyHubDegradedStatesSchema,
  unavailableReason: OptionalFamilyHubTextSchema,
});
export const BrowserAiFamilyHubCapabilitySchema = withParser(
  BrowserAiFamilyHubCapabilityBaseSchema.pipe(
    Schema.filter(
      (value) =>
        browserAiFamilyHubCapabilityIsConsistent(value) ||
        'Expected browser AI family hub capability to preserve local-household, no-retention boundaries'
    )
  )
);

const BrowserAiFamilyHubRouteBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(BrowserAiFamilyHubRouteSchemaVersion),
  routeId: BrowserAiFamilyHubRouteIdSchema,
  requestId: BrowserUrlAiAnalysisRequestIdSchema,
  routedAt: ActivityTimestampSchema,
  sourceLocalProviderRouteId: BrowserAiProviderRouteIdSchema,
  sourceLocalProviderRoute: BrowserAiProviderRouteSchema,
  capability: BrowserAiFamilyHubCapabilitySchema,
  executionState: BrowserAiFamilyHubExecutionStateSchema,
  selectedRuntimeRef: OptionalFamilyHubRuntimeRefSchema,
  degradedStates: FamilyHubDegradedStatesSchema,
  auditEvidenceIds: FamilyHubAuditEvidenceIdsSchema,
  localProviderAttempted: Schema.Boolean,
  parentAllowedFamilyHub: Schema.Boolean,
  dataScopeVisible: Schema.Boolean,
  retentionVisible: Schema.Boolean,
  custodyVisible: Schema.Boolean,
  providerVisible: Schema.Boolean,
  noRetentionVisible: Schema.Boolean,
  remoteProviderSelected: Schema.Boolean,
  remoteDefaultForBlocking: Schema.Boolean,
});
export const BrowserAiFamilyHubRouteSchema = withParser(
  BrowserAiFamilyHubRouteBaseSchema.pipe(
    Schema.filter(
      (value) =>
        browserAiFamilyHubRouteIsConsistent(value) ||
        'Expected browser AI family hub route to be local-household fallback only'
    )
  )
);

const BrowserAiFamilyHubRouteRequestSchema = withParser(
  Schema.Struct({
    routeId: BrowserAiFamilyHubRouteIdSchema,
    routedAt: ActivityTimestampSchema,
    input: BrowserUrlAiAnalysisInputSchema,
    sourceLocalProviderRoute: BrowserAiProviderRouteSchema,
    capability: BrowserAiFamilyHubCapabilitySchema,
    parentAllowedFamilyHub: Schema.Boolean,
    auditEvidenceIds: FamilyHubAuditEvidenceIdsSchema,
  })
);

export const decodeBrowserAiFamilyHubCapability = Schema.decodeUnknownSync(BrowserAiFamilyHubCapabilitySchema);
export const decodeBrowserAiFamilyHubRoute = Schema.decodeUnknownSync(BrowserAiFamilyHubRouteSchema);

export function planBrowserAiFamilyHubRoute(
  request: Infer<typeof BrowserAiFamilyHubRouteRequestSchema>
): BrowserAiFamilyHubRoute {
  const parsed = BrowserAiFamilyHubRouteRequestSchema.parse(request);
  const selected = familyHubCanServe(parsed.input, parsed.sourceLocalProviderRoute, parsed.capability, parsed);
  const degradedStates = selected ? [] : familyHubDegradedStatesFor(parsed);

  return BrowserAiFamilyHubRouteSchema.parse({
    schemaVersion: BrowserAiFamilyHubRouteSchemaVersion,
    routeId: parsed.routeId,
    requestId: parsed.input.requestId,
    routedAt: parsed.routedAt,
    sourceLocalProviderRouteId: parsed.sourceLocalProviderRoute.routeId,
    sourceLocalProviderRoute: parsed.sourceLocalProviderRoute,
    capability: parsed.capability,
    executionState: selected ? 'selected' : familyHubExecutionStateFor(parsed.capability),
    selectedRuntimeRef: selected ? parsed.capability.modelRuntimeRef : null,
    degradedStates,
    auditEvidenceIds: parsed.auditEvidenceIds,
    localProviderAttempted: true,
    parentAllowedFamilyHub: parsed.parentAllowedFamilyHub,
    dataScopeVisible: true,
    retentionVisible: true,
    custodyVisible: true,
    providerVisible: true,
    noRetentionVisible: true,
    remoteProviderSelected: false,
    remoteDefaultForBlocking: false,
  });
}

export type BrowserAiFamilyHubCapability = Infer<typeof BrowserAiFamilyHubCapabilitySchema>;
export type BrowserAiFamilyHubDegradedState = Infer<typeof BrowserAiFamilyHubDegradedStateSchema>;
export type BrowserAiFamilyHubRoute = Infer<typeof BrowserAiFamilyHubRouteSchema>;

function browserAiFamilyHubCapabilityIsConsistent(value: Infer<typeof BrowserAiFamilyHubCapabilityBaseSchema>) {
  if (
    !value.localHouseholdOnly ||
    !value.noRetention ||
    value.parentRemoteApprovalRequired ||
    value.childDeviceCanRunModel ||
    value.custodyLabel !== 'local-network-child-agent'
  ) {
    return false;
  }
  if (value.capabilityState === 'available') {
    return (
      value.modelRuntimeRef !== null &&
      value.householdRouteRef !== null &&
      value.degradedStates.length === 0 &&
      value.unavailableReason === null
    );
  }
  return value.modelRuntimeRef === null && value.degradedStates.length > 0 && value.unavailableReason !== null;
}

function browserAiFamilyHubRouteIsConsistent(value: Infer<typeof BrowserAiFamilyHubRouteBaseSchema>) {
  if (!familyHubRouteVisibilityIsComplete(value) || value.remoteProviderSelected || value.remoteDefaultForBlocking) {
    return false;
  }
  if (!value.localProviderAttempted || value.sourceLocalProviderRoute.executionState === 'selected') {
    return value.executionState !== 'selected' && value.selectedRuntimeRef === null;
  }
  if (value.executionState === 'selected') {
    return value.capability.capabilityState === 'available' && value.selectedRuntimeRef !== null;
  }
  return value.selectedRuntimeRef === null && value.degradedStates.length > 0;
}

function familyHubCanServe(
  input: BrowserUrlAiAnalysisInput,
  localRoute: BrowserAiProviderRoute,
  capability: BrowserAiFamilyHubCapability,
  request: Infer<typeof BrowserAiFamilyHubRouteRequestSchema>
) {
  return (
    request.parentAllowedFamilyHub &&
    input.modelRuntimePreference === 'local-preferred' &&
    localRoute.executionState !== 'selected' &&
    capability.capabilityState === 'available' &&
    capability.supportedTasks.includes(input.requestedTask)
  );
}

function familyHubDegradedStatesFor(request: Infer<typeof BrowserAiFamilyHubRouteRequestSchema>) {
  if (!request.parentAllowedFamilyHub) {
    return ['parent-disabled'] as const;
  }
  if (request.sourceLocalProviderRoute.executionState === 'selected') {
    return ['local-provider-not-exhausted'] as const;
  }
  if (!request.capability.supportedTasks.includes(request.input.requestedTask)) {
    return ['unsupported-task'] as const;
  }
  return request.capability.degradedStates.length > 0
    ? request.capability.degradedStates
    : (['manual-required'] as const);
}

function familyHubExecutionStateFor(capability: BrowserAiFamilyHubCapability) {
  return capability.capabilityState === 'hub-unavailable' ? 'unavailable' : 'manual-required';
}

function familyHubRouteVisibilityIsComplete(value: Infer<typeof BrowserAiFamilyHubRouteBaseSchema>) {
  return (
    value.dataScopeVisible &&
    value.retentionVisible &&
    value.custodyVisible &&
    value.providerVisible &&
    value.noRetentionVisible
  );
}
