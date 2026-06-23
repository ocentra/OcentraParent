import { type Infer, Schema, withParser } from './effect';
import { ActivityTimestampSchema } from './evidence-primitives';
import {
  ScreenChildLocalAnalysisAttemptStateSchema,
  ScreenFamilyAiHubAuditEvidenceIdsSchema,
  ScreenFamilyAiHubCapabilityStateSchema,
  ScreenFamilyAiHubDegradedStatesSchema,
  ScreenFamilyAiHubExecutionStateSchema,
  ScreenFamilyAiHubIdSchema,
  ScreenFamilyAiHubOptionalRuntimeRefSchema,
  ScreenFamilyAiHubOptionalTextSchema,
  ScreenFamilyAiHubRequestedTaskSchema,
  ScreenFamilyAiHubRequiredFalseSchema,
  ScreenFamilyAiHubRequiredTrueSchema,
  ScreenFamilyAiHubRouteIdSchema,
  ScreenFamilyAiHubRouteRefSchema,
  ScreenFamilyAiHubRouteSchemaVersion,
  ScreenFamilyAiHubSupportedTasksSchema,
  ScreenFamilyAiHubTransferModeSchema,
} from './screen-evidence-family-hub-routing-values';
import { ScreenEvidenceQueueJobIdSchema } from './screen-evidence-primitives';
import { ScreenEvidenceCustodyStateSchema, ScreenLocalModelProviderKindSchema } from './screen-evidence-states';

const ScreenFamilyAiHubCapabilityBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(ScreenFamilyAiHubRouteSchemaVersion),
  hubId: ScreenFamilyAiHubIdSchema,
  checkedAt: ActivityTimestampSchema,
  capabilityState: ScreenFamilyAiHubCapabilityStateSchema,
  supportedTasks: ScreenFamilyAiHubSupportedTasksSchema,
  modelRuntimeRef: ScreenFamilyAiHubOptionalRuntimeRefSchema,
  householdRouteRef: Schema.Union(ScreenFamilyAiHubRouteRefSchema, Schema.Null),
  custodyState: ScreenEvidenceCustodyStateSchema,
  noRetention: ScreenFamilyAiHubRequiredTrueSchema,
  localHouseholdOnly: ScreenFamilyAiHubRequiredTrueSchema,
  parentApprovalRequired: ScreenFamilyAiHubRequiredTrueSchema,
  ocentraHostedProcessingAllowed: ScreenFamilyAiHubRequiredFalseSchema,
  rawImageRetentionAllowed: ScreenFamilyAiHubRequiredFalseSchema,
  degradedStates: ScreenFamilyAiHubDegradedStatesSchema,
  unavailableReason: ScreenFamilyAiHubOptionalTextSchema,
});

export const ScreenFamilyAiHubCapabilitySchema = withParser(
  ScreenFamilyAiHubCapabilityBaseSchema.pipe(
    Schema.filter(
      (value) =>
        screenFamilyAiHubCapabilityIsConsistent(value) ||
        'Expected screen family AI hub capability to stay local household, no-retention, and non-Ocentra-hosted'
    )
  )
);

const ScreenChildLocalAnalysisAttemptBaseSchema = Schema.Struct({
  attempted: ScreenFamilyAiHubRequiredTrueSchema,
  providerKind: ScreenLocalModelProviderKindSchema,
  executionState: ScreenChildLocalAnalysisAttemptStateSchema,
  modelRuntimeRef: ScreenFamilyAiHubOptionalRuntimeRefSchema,
  degradedStates: ScreenFamilyAiHubDegradedStatesSchema,
});

type ScreenChildLocalAnalysisAttemptCandidate = Infer<typeof ScreenChildLocalAnalysisAttemptBaseSchema>;

const ScreenChildLocalAnalysisAttemptSchema = withParser(
  ScreenChildLocalAnalysisAttemptBaseSchema.pipe(
    Schema.filter(
      (value) =>
        screenChildLocalAttemptIsConsistent(value) ||
        'Expected child-local screen AI attempt to expose selected runtime or degraded state'
    )
  )
);

const ScreenFamilyAiHubRouteBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(ScreenFamilyAiHubRouteSchemaVersion),
  routeId: ScreenFamilyAiHubRouteIdSchema,
  queueJobId: ScreenEvidenceQueueJobIdSchema,
  routedAt: ActivityTimestampSchema,
  requestedTask: ScreenFamilyAiHubRequestedTaskSchema,
  sourceChildLocalAttempt: ScreenChildLocalAnalysisAttemptSchema,
  capability: ScreenFamilyAiHubCapabilitySchema,
  executionState: ScreenFamilyAiHubExecutionStateSchema,
  selectedRuntimeRef: ScreenFamilyAiHubOptionalRuntimeRefSchema,
  transferMode: ScreenFamilyAiHubTransferModeSchema,
  sourceCustodyState: ScreenEvidenceCustodyStateSchema,
  destinationCustodyState: ScreenEvidenceCustodyStateSchema,
  degradedStates: ScreenFamilyAiHubDegradedStatesSchema,
  auditEvidenceIds: ScreenFamilyAiHubAuditEvidenceIdsSchema,
  parentApprovedFamilyHub: Schema.Boolean,
  localProviderAttempted: ScreenFamilyAiHubRequiredTrueSchema,
  childSafetyLocalFallbackPreserved: ScreenFamilyAiHubRequiredTrueSchema,
  summaryFirst: ScreenFamilyAiHubRequiredTrueSchema,
  redactedOrCroppedInputRequired: ScreenFamilyAiHubRequiredTrueSchema,
  rawFullScreenshotTransferAllowed: ScreenFamilyAiHubRequiredFalseSchema,
  rawImageRetentionAllowed: ScreenFamilyAiHubRequiredFalseSchema,
  remoteProviderSelected: ScreenFamilyAiHubRequiredFalseSchema,
  remoteApiFallbackAllowed: ScreenFamilyAiHubRequiredFalseSchema,
  ocentraHostedProcessingAllowed: ScreenFamilyAiHubRequiredFalseSchema,
  remoteDefaultForBlocking: ScreenFamilyAiHubRequiredFalseSchema,
});

export const ScreenFamilyAiHubRouteSchema = withParser(
  ScreenFamilyAiHubRouteBaseSchema.pipe(
    Schema.filter(
      (value) =>
        screenFamilyAiHubRouteIsConsistent(value) ||
        'Expected screen family AI hub route to be selected only after local-child degradation and never remote-default'
    )
  )
);

export const ScreenFamilyAiHubRouteRequestSchema = withParser(
  Schema.Struct({
    routeId: ScreenFamilyAiHubRouteIdSchema,
    queueJobId: ScreenEvidenceQueueJobIdSchema,
    routedAt: ActivityTimestampSchema,
    requestedTask: ScreenFamilyAiHubRequestedTaskSchema,
    sourceChildLocalAttempt: ScreenChildLocalAnalysisAttemptSchema,
    capability: ScreenFamilyAiHubCapabilitySchema,
    parentApprovedFamilyHub: Schema.Boolean,
    transferMode: ScreenFamilyAiHubTransferModeSchema,
    sourceCustodyState: ScreenEvidenceCustodyStateSchema,
    auditEvidenceIds: ScreenFamilyAiHubAuditEvidenceIdsSchema,
  })
);

export const decodeScreenFamilyAiHubCapability = Schema.decodeUnknownSync(ScreenFamilyAiHubCapabilitySchema);
export const decodeScreenFamilyAiHubRoute = Schema.decodeUnknownSync(ScreenFamilyAiHubRouteSchema);

export type ScreenFamilyAiHubCapability = Infer<typeof ScreenFamilyAiHubCapabilitySchema>;
export type ScreenFamilyAiHubRoute = Infer<typeof ScreenFamilyAiHubRouteSchema>;

function screenFamilyAiHubCapabilityIsConsistent(value: Infer<typeof ScreenFamilyAiHubCapabilityBaseSchema>) {
  if (value.custodyState !== 'live-lan-child-agent') {
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

function screenChildLocalAttemptIsConsistent(value: ScreenChildLocalAnalysisAttemptCandidate) {
  if (value.executionState === 'selected') {
    return value.modelRuntimeRef !== null && value.degradedStates.length === 0;
  }
  return value.modelRuntimeRef === null && value.degradedStates.length > 0;
}

function screenFamilyAiHubRouteIsConsistent(value: Infer<typeof ScreenFamilyAiHubRouteBaseSchema>) {
  if (value.executionState === 'selected') {
    return (
      value.parentApprovedFamilyHub &&
      value.sourceChildLocalAttempt.executionState !== 'selected' &&
      value.capability.capabilityState === 'available' &&
      value.capability.supportedTasks.includes(value.requestedTask) &&
      value.selectedRuntimeRef !== null &&
      value.transferMode !== 'noTransfer' &&
      value.destinationCustodyState === 'live-lan-child-agent' &&
      value.degradedStates.length === 0
    );
  }
  return value.selectedRuntimeRef === null && value.transferMode === 'noTransfer' && value.degradedStates.length > 0;
}
