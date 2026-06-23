import { type Infer, Schema, withParser, NonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { ActivityEvidenceIdSchema, ActivityTimestampSchema } from '@ocentra-parent/schema-domain/evidence-primitives';
import {
  BrowserAiModelRuntimeRefSchema,
  BrowserAiRequestedTaskSchema,
  type BrowserUrlAiAnalysisInput,
  BrowserUrlAiAnalysisInputSchema,
  BrowserUrlAiAnalysisRequestIdSchema,
} from './browser-ai-analysis-schemas';
import {
  BrowserAiRemoteApprovalIdSchema,
  BrowserAiRemoteCapabilityStateSchema,
  BrowserAiRemoteDataScopeSchema,
  BrowserAiRemoteDegradedStateSchema,
  BrowserAiRemoteExecutionStateSchema,
  BrowserAiRemoteProviderIdSchema,
  BrowserAiRemoteRetentionModeSchema,
  BrowserAiRemoteRouteIdSchema,
} from '@ocentra-parent/schema-domain/browser-ai-remote-boundary-values';
const OptionalRemoteAiTextSchema = Schema.Union(NonEmptyStringSchema, Schema.Null);
const OptionalRemoteRuntimeRefSchema = Schema.Union(BrowserAiModelRuntimeRefSchema, Schema.Null);

export const BrowserAiRemoteBoundarySchemaVersion = 1;

const RemoteSupportedTasksSchema = Schema.Array(BrowserAiRequestedTaskSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected at least one remote AI supported task')
);
const RemoteDataScopesSchema = Schema.Array(BrowserAiRemoteDataScopeSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected at least one remote AI data scope')
);
const RemoteAuditEvidenceIdsSchema = Schema.Array(ActivityEvidenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected at least one remote AI audit evidence id')
);
const RemoteDegradedStatesSchema = Schema.Array(BrowserAiRemoteDegradedStateSchema);

const BrowserAiRemoteApprovalBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(BrowserAiRemoteBoundarySchemaVersion),
  approvalId: BrowserAiRemoteApprovalIdSchema,
  approvedAt: ActivityTimestampSchema,
  approvedByParentRef: NonEmptyStringSchema,
  providerId: BrowserAiRemoteProviderIdSchema,
  allowedTasks: RemoteSupportedTasksSchema,
  allowedDataScopes: RemoteDataScopesSchema,
  retentionMode: BrowserAiRemoteRetentionModeSchema,
  expiresAt: ActivityTimestampSchema,
  parentCanRevoke: Schema.Boolean,
  rawBrowserStateAllowed: Schema.Boolean,
  rawPageBodyAllowed: Schema.Boolean,
  transcriptTextAllowed: Schema.Boolean,
  screenshotAllowed: Schema.Boolean,
});
export const BrowserAiRemoteApprovalSchema = withParser(
  BrowserAiRemoteApprovalBaseSchema.pipe(
    Schema.filter(
      (value) =>
        browserAiRemoteApprovalIsConsistent(value) ||
        'Expected remote AI approval to be parent-owned, no-retention, and structured-scope only'
    )
  )
);

const BrowserAiRemoteCapabilityBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(BrowserAiRemoteBoundarySchemaVersion),
  providerId: BrowserAiRemoteProviderIdSchema,
  checkedAt: ActivityTimestampSchema,
  capabilityState: BrowserAiRemoteCapabilityStateSchema,
  supportedTasks: RemoteSupportedTasksSchema,
  modelRuntimeRef: OptionalRemoteRuntimeRefSchema,
  approval: Schema.Union(BrowserAiRemoteApprovalSchema, Schema.Null),
  retentionMode: BrowserAiRemoteRetentionModeSchema,
  allowedDataScopes: RemoteDataScopesSchema,
  dataScopeVisible: Schema.Boolean,
  retentionVisible: Schema.Boolean,
  providerVisible: Schema.Boolean,
  noRetentionVisible: Schema.Boolean,
  degradedStates: RemoteDegradedStatesSchema,
  unavailableReason: OptionalRemoteAiTextSchema,
});
export const BrowserAiRemoteCapabilitySchema = withParser(
  BrowserAiRemoteCapabilityBaseSchema.pipe(
    Schema.filter(
      (value) =>
        browserAiRemoteCapabilityIsConsistent(value) ||
        'Expected remote AI capability to require approval, visible no-retention, and explicit unavailable states'
    )
  )
);

const BrowserAiRemoteRouteBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(BrowserAiRemoteBoundarySchemaVersion),
  routeId: BrowserAiRemoteRouteIdSchema,
  requestId: BrowserUrlAiAnalysisRequestIdSchema,
  routedAt: ActivityTimestampSchema,
  capability: BrowserAiRemoteCapabilitySchema,
  executionState: BrowserAiRemoteExecutionStateSchema,
  selectedRuntimeRef: OptionalRemoteRuntimeRefSchema,
  degradedStates: RemoteDegradedStatesSchema,
  auditEvidenceIds: RemoteAuditEvidenceIdsSchema,
  parentExplicitRemoteApproval: Schema.Boolean,
  localSafetyFallbackAvailable: Schema.Boolean,
  remoteDefaultForBlocking: Schema.Boolean,
  remoteCanOverrideStricterLocalRules: Schema.Boolean,
  remoteOutageDisablesLocalSafety: Schema.Boolean,
});
export const BrowserAiRemoteRouteSchema = withParser(
  BrowserAiRemoteRouteBaseSchema.pipe(
    Schema.filter(
      (value) =>
        browserAiRemoteRouteIsConsistent(value) ||
        'Expected remote AI route to be parent-approved and unable to override local safety'
    )
  )
);

const BrowserAiRemoteRouteRequestSchema = withParser(
  Schema.Struct({
    routeId: BrowserAiRemoteRouteIdSchema,
    routedAt: ActivityTimestampSchema,
    input: BrowserUrlAiAnalysisInputSchema,
    capability: BrowserAiRemoteCapabilitySchema,
    parentExplicitRemoteApproval: Schema.Boolean,
    localSafetyFallbackAvailable: Schema.Boolean,
    auditEvidenceIds: RemoteAuditEvidenceIdsSchema,
  })
);

export const decodeBrowserAiRemoteApproval = Schema.decodeUnknownSync(BrowserAiRemoteApprovalSchema);
export const decodeBrowserAiRemoteCapability = Schema.decodeUnknownSync(BrowserAiRemoteCapabilitySchema);
export const decodeBrowserAiRemoteRoute = Schema.decodeUnknownSync(BrowserAiRemoteRouteSchema);

export function planBrowserAiRemoteRoute(
  request: Infer<typeof BrowserAiRemoteRouteRequestSchema>
): BrowserAiRemoteRoute {
  const parsed = BrowserAiRemoteRouteRequestSchema.parse(request);
  const selected = remoteProviderCanServe(parsed.input, parsed.capability, parsed);
  const degradedStates = selected ? [] : remoteDegradedStatesFor(parsed);

  return BrowserAiRemoteRouteSchema.parse({
    schemaVersion: BrowserAiRemoteBoundarySchemaVersion,
    routeId: parsed.routeId,
    requestId: parsed.input.requestId,
    routedAt: parsed.routedAt,
    capability: parsed.capability,
    executionState: selected ? 'selected' : remoteExecutionStateFor(parsed.capability),
    selectedRuntimeRef: selected ? parsed.capability.modelRuntimeRef : null,
    degradedStates,
    auditEvidenceIds: parsed.auditEvidenceIds,
    parentExplicitRemoteApproval: parsed.parentExplicitRemoteApproval,
    localSafetyFallbackAvailable: parsed.localSafetyFallbackAvailable,
    remoteDefaultForBlocking: false,
    remoteCanOverrideStricterLocalRules: false,
    remoteOutageDisablesLocalSafety: false,
  });
}

export type BrowserAiRemoteApproval = Infer<typeof BrowserAiRemoteApprovalSchema>;
export type BrowserAiRemoteCapability = Infer<typeof BrowserAiRemoteCapabilitySchema>;
export type BrowserAiRemoteDegradedState = Infer<typeof BrowserAiRemoteDegradedStateSchema>;
export type BrowserAiRemoteRoute = Infer<typeof BrowserAiRemoteRouteSchema>;

function browserAiRemoteApprovalIsConsistent(value: Infer<typeof BrowserAiRemoteApprovalBaseSchema>) {
  return (
    value.parentCanRevoke &&
    value.retentionMode === 'no-retention' &&
    !value.rawBrowserStateAllowed &&
    !value.rawPageBodyAllowed &&
    !value.transcriptTextAllowed &&
    !value.screenshotAllowed
  );
}

function browserAiRemoteCapabilityIsConsistent(value: Infer<typeof BrowserAiRemoteCapabilityBaseSchema>) {
  if (!remoteVisibilityIsComplete(value)) {
    return false;
  }
  if (value.capabilityState === 'available') {
    return (
      value.approval !== null &&
      value.modelRuntimeRef !== null &&
      value.retentionMode === 'no-retention' &&
      value.degradedStates.length === 0
    );
  }
  return value.modelRuntimeRef === null && value.degradedStates.length > 0 && value.unavailableReason !== null;
}

function browserAiRemoteRouteIsConsistent(value: Infer<typeof BrowserAiRemoteRouteBaseSchema>) {
  if (!value.localSafetyFallbackAvailable || remoteRouteClaimsUnsafeAuthority(value)) {
    return false;
  }
  if (value.executionState === 'selected') {
    return (
      value.parentExplicitRemoteApproval &&
      value.capability.capabilityState === 'available' &&
      value.selectedRuntimeRef !== null &&
      value.degradedStates.length === 0
    );
  }
  return value.selectedRuntimeRef === null && value.degradedStates.length > 0;
}

function remoteProviderCanServe(
  input: BrowserUrlAiAnalysisInput,
  capability: BrowserAiRemoteCapability,
  request: Infer<typeof BrowserAiRemoteRouteRequestSchema>
) {
  return (
    input.modelRuntimePreference === 'parent-approved-remote' &&
    request.parentExplicitRemoteApproval &&
    request.localSafetyFallbackAvailable &&
    capability.capabilityState === 'available' &&
    capability.supportedTasks.includes(input.requestedTask) &&
    capability.approval !== null &&
    capability.approval.allowedTasks.includes(input.requestedTask)
  );
}

function remoteDegradedStatesFor(request: Infer<typeof BrowserAiRemoteRouteRequestSchema>) {
  if (!request.parentExplicitRemoteApproval) {
    return ['parent-approval-missing'] as const;
  }
  if (!request.localSafetyFallbackAvailable) {
    return ['local-safety-fallback-missing'] as const;
  }
  if (!request.capability.supportedTasks.includes(request.input.requestedTask)) {
    return ['unsupported-task'] as const;
  }
  return request.capability.degradedStates.length > 0
    ? request.capability.degradedStates
    : (['manual-required'] as const);
}

function remoteExecutionStateFor(capability: BrowserAiRemoteCapability) {
  return capability.capabilityState === 'provider-unavailable' ? 'unavailable' : 'manual-required';
}

function remoteVisibilityIsComplete(value: Infer<typeof BrowserAiRemoteCapabilityBaseSchema>) {
  return value.dataScopeVisible && value.retentionVisible && value.providerVisible && value.noRetentionVisible;
}

function remoteRouteClaimsUnsafeAuthority(value: Infer<typeof BrowserAiRemoteRouteBaseSchema>) {
  return (
    value.remoteDefaultForBlocking || value.remoteCanOverrideStricterLocalRules || value.remoteOutageDisablesLocalSafety
  );
}
