import {
  NonEmptyStringSchema,
  Schema,
  withParser,
} from './effect';
import {
  ChildProfileIdSchema,
  FamilyIdSchema,
  ParentDeviceIdSchema,
  ParentEvidenceReferenceIdSchema,
  ParentEvidenceReferenceKindSchema,
  ParentPolicyVersionSchema,
  ParentTimestampSchema,
} from './family-reference-primitives';
import {
  TrackingAcknowledgementIdSchema,
  TrackingAcknowledgementStateSchema,
  TrackingAiAnalysisIdSchema,
  TrackingAiLocationRiskLevelSchema,
  TrackingAiProviderModeSchema,
  TrackingAlertIdSchema,
  TrackingAlertSeveritySchema,
  TrackingCheckInIdSchema,
  TrackingCheckInRequestStateSchema,
  TrackingCheckInResponseKindSchema,
  TrackingEscalationIdSchema,
  TrackingEscalationStateSchema,
  TrackingLiveTrackingGrantIdSchema,
  TrackingLiveTrackingGrantStateSchema,
  TrackingMissingDeviceCaseIdSchema,
  TrackingMissingDeviceStateSchema,
  TrackingPolicyActionSchema,
  TrackingPolicyAuditRefSchema,
  TrackingPolicyDecisionIdSchema,
  TrackingPolicyReasonCodeSchema,
  TrackingPolicyRuleIdSchema,
  TrackingPolicySchemaVersion,
  TrackingPolicyTargetKindSchema,
  TrackingPlatformProofRouteStateSchema,
  TrackingProviderCapabilityStateSchema,
  TrackingProviderRouteIdSchema,
} from './tracking-location-policy-primitives';
import { TrackingPlatformProofRouteSchema } from './tracking-location-policy-platform-proof';
import type {
  TrackingAcknowledgement,
  TrackingAiProviderRoute,
  TrackingAlertIntent,
  TrackingChildCheckInRequest,
  TrackingChildCheckInResponse,
  TrackingEscalationChain,
  TrackingEvidenceTrace,
  TrackingLocationAiAnalysisInput,
  TrackingLocationAiAnalysisResult,
  TrackingLocationPolicyReadModel,
  TrackingMissingDeviceCase,
  TrackingPlatformProofRoute,
  TrackingPolicyDecision,
  TrackingPolicyRule,
  TrackingTemporaryLiveTrackingGrant,
} from './tracking-location-policy-types';

const TrackingPolicyConfidenceSchema = Schema.Number.pipe(Schema.between(0, 1));
const TrackingPolicyDurationSecondsSchema = Schema.Number.pipe(Schema.int(), Schema.nonNegative());

export const TrackingEvidenceTraceSchema = withParser(
  Schema.Struct({
    evidenceReferenceId: ParentEvidenceReferenceIdSchema,
    kind: ParentEvidenceReferenceKindSchema,
    observedAt: ParentTimestampSchema,
  })
);

export const TrackingPolicyRuleSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    ruleId: TrackingPolicyRuleIdSchema,
    familyId: FamilyIdSchema,
    childProfileId: ChildProfileIdSchema,
    deviceId: ParentDeviceIdSchema,
    policyVersion: ParentPolicyVersionSchema,
    targetKind: TrackingPolicyTargetKindSchema,
    action: TrackingPolicyActionSchema,
    enabled: Schema.Boolean,
    requiresFreshEvidence: Schema.Boolean,
    requiresParentConfirmation: Schema.Boolean,
    reasonCodes: Schema.Array(TrackingPolicyReasonCodeSchema),
    auditRefs: Schema.Array(TrackingPolicyAuditRefSchema),
  })
);

export const TrackingPolicyDecisionSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    decisionId: TrackingPolicyDecisionIdSchema,
    decidedAt: ParentTimestampSchema,
    ruleId: TrackingPolicyRuleIdSchema,
    action: TrackingPolicyActionSchema,
    dryRun: Schema.Boolean,
    evidenceReferences: Schema.Array(TrackingEvidenceTraceSchema),
    aiAnalysisId: Schema.Union(TrackingAiAnalysisIdSchema, Schema.Null),
    alertIntentId: Schema.Union(TrackingAlertIdSchema, Schema.Null),
    reasonCodes: Schema.Array(TrackingPolicyReasonCodeSchema),
    auditRefs: Schema.Array(TrackingPolicyAuditRefSchema),
  }).pipe(
    Schema.filter(
      (decision) =>
        decision.evidenceReferences.length > 0 ||
        decision.action === 'manual-required' ||
        decision.action === 'no-action' ||
        'Tracking policy actions need cited evidence unless they are manual-required or no-action'
    )
  )
);

export const TrackingAcknowledgementSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    acknowledgementId: TrackingAcknowledgementIdSchema,
    alertId: TrackingAlertIdSchema,
    state: TrackingAcknowledgementStateSchema,
    acknowledgedAt: Schema.Union(ParentTimestampSchema, Schema.Null),
    expiresAt: Schema.Union(ParentTimestampSchema, Schema.Null),
    stillAlertForCritical: Schema.Boolean,
    reasonCodes: Schema.Array(TrackingPolicyReasonCodeSchema),
    auditRefs: Schema.Array(TrackingPolicyAuditRefSchema),
  }).pipe(
    Schema.filter(
      (acknowledgement) =>
        trackingAcknowledgementPreservesCriticalAlerts(acknowledgement) ||
        'Tracking acknowledgement and exception states must keep critical alerts visible'
    )
  )
);

export const TrackingChildCheckInRequestSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    checkInId: TrackingCheckInIdSchema,
    requestedAt: ParentTimestampSchema,
    state: TrackingCheckInRequestStateSchema,
    relatedAlertId: Schema.Union(TrackingAlertIdSchema, Schema.Null),
    includeLocationIfPermitted: Schema.Boolean,
    expiresAt: ParentTimestampSchema,
    evidenceReferences: Schema.Array(TrackingEvidenceTraceSchema),
    auditRefs: Schema.Array(TrackingPolicyAuditRefSchema),
  })
);

export const TrackingChildCheckInResponseSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    checkInId: TrackingCheckInIdSchema,
    respondedAt: ParentTimestampSchema,
    response: TrackingCheckInResponseKindSchema,
    locationEvidenceReference: Schema.Union(TrackingEvidenceTraceSchema, Schema.Null),
    auditRefs: Schema.Array(TrackingPolicyAuditRefSchema),
  })
);

export const TrackingLocationAiAnalysisInputSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    analysisId: TrackingAiAnalysisIdSchema,
    requestedAt: ParentTimestampSchema,
    evidenceReferences: Schema.Array(TrackingEvidenceTraceSchema),
    policyVersion: ParentPolicyVersionSchema,
    providerRouteId: TrackingProviderRouteIdSchema,
  }).pipe(
    Schema.filter(
      (input) => input.evidenceReferences.length > 0 || 'Tracking AI analysis inputs need cited location evidence'
    )
  )
);

export const TrackingLocationAiAnalysisResultSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    analysisId: TrackingAiAnalysisIdSchema,
    completedAt: ParentTimestampSchema,
    riskLevel: TrackingAiLocationRiskLevelSchema,
    confidence: TrackingPolicyConfidenceSchema,
    providerRouteId: TrackingProviderRouteIdSchema,
    evidenceReferences: Schema.Array(TrackingEvidenceTraceSchema),
    reasonCodes: Schema.Array(TrackingPolicyReasonCodeSchema),
    canTriggerAlertDirectly: Schema.Literal(false),
    isFinalAuthority: Schema.Literal(false),
  })
);

export const TrackingAiProviderRouteSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    providerRouteId: TrackingProviderRouteIdSchema,
    mode: TrackingAiProviderModeSchema,
    capabilityState: TrackingProviderCapabilityStateSchema,
    remoteDataAllowed: Schema.Boolean,
    unavailableReason: Schema.Union(TrackingPolicyReasonCodeSchema, Schema.Null),
    auditRefs: Schema.Array(TrackingPolicyAuditRefSchema),
  }).pipe(
    Schema.filter(
      (route) =>
        route.mode === 'parent-approved-remote' ||
        !route.remoteDataAllowed ||
        'Tracking remote data can be allowed only for parent-approved remote provider routes'
    )
  )
);

export const TrackingAlertIntentSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    alertId: TrackingAlertIdSchema,
    createdAt: ParentTimestampSchema,
    severity: TrackingAlertSeveritySchema,
    policyDecisionId: TrackingPolicyDecisionIdSchema,
    evidenceReferences: Schema.Array(TrackingEvidenceTraceSchema),
    sensitiveDetailMode: Schema.Literal('minimal-provider-body', 'authenticated-drill-in-only'),
    notificationStatusRefs: Schema.Array(TrackingPolicyAuditRefSchema),
    acknowledgementId: Schema.Union(TrackingAcknowledgementIdSchema, Schema.Null),
    reasonCodes: Schema.Array(TrackingPolicyReasonCodeSchema),
  })
);

export const TrackingEscalationChainSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    escalationId: TrackingEscalationIdSchema,
    alertId: TrackingAlertIdSchema,
    state: TrackingEscalationStateSchema,
    startedAt: ParentTimestampSchema,
    nextActionAt: Schema.Union(ParentTimestampSchema, Schema.Null),
    steps: Schema.Array(NonEmptyStringSchema),
    auditRefs: Schema.Array(TrackingPolicyAuditRefSchema),
  })
);

export const TrackingTemporaryLiveTrackingGrantSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    grantId: TrackingLiveTrackingGrantIdSchema,
    state: TrackingLiveTrackingGrantStateSchema,
    requestedAt: ParentTimestampSchema,
    expiresAt: ParentTimestampSchema,
    durationSeconds: TrackingPolicyDurationSecondsSchema,
    parentApproved: Schema.Boolean,
    childDisclosureRequired: Schema.Boolean,
    auditRefs: Schema.Array(TrackingPolicyAuditRefSchema),
  }).pipe(
    Schema.filter(
      (grant) =>
        grant.state !== 'active' ||
        (grant.parentApproved && grant.childDisclosureRequired) ||
        'Active temporary live tracking grants need parent approval and child disclosure'
    )
  )
);

export const TrackingMissingDeviceCaseSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    caseId: TrackingMissingDeviceCaseIdSchema,
    openedAt: ParentTimestampSchema,
    state: TrackingMissingDeviceStateSchema,
    lastKnownEvidence: Schema.Union(TrackingEvidenceTraceSchema, Schema.Null),
    deviceStatusEvidence: Schema.Union(TrackingEvidenceTraceSchema, Schema.Null),
    contactActionRefs: Schema.Array(TrackingPolicyAuditRefSchema),
    reasonCodes: Schema.Array(TrackingPolicyReasonCodeSchema),
  })
);

export const TrackingLocationPolicyReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    generatedAt: ParentTimestampSchema,
    rules: Schema.Array(TrackingPolicyRuleSchema),
    decisions: Schema.Array(TrackingPolicyDecisionSchema),
    acknowledgements: Schema.Array(TrackingAcknowledgementSchema),
    checkInRequests: Schema.Array(TrackingChildCheckInRequestSchema),
    checkInResponses: Schema.Array(TrackingChildCheckInResponseSchema),
    aiRoutes: Schema.Array(TrackingAiProviderRouteSchema),
    aiResults: Schema.Array(TrackingLocationAiAnalysisResultSchema),
    alerts: Schema.Array(TrackingAlertIntentSchema),
    escalations: Schema.Array(TrackingEscalationChainSchema),
    temporaryLiveGrants: Schema.Array(TrackingTemporaryLiveTrackingGrantSchema),
    missingDeviceCases: Schema.Array(TrackingMissingDeviceCaseSchema),
    platformProofRoutes: Schema.Array(TrackingPlatformProofRouteSchema),
  })
);

export {
  TrackingPolicySchemaVersion,
  TrackingPolicyRuleIdSchema,
  TrackingPolicyDecisionIdSchema,
  TrackingAlertIdSchema,
  TrackingAcknowledgementIdSchema,
  TrackingCheckInIdSchema,
  TrackingAiAnalysisIdSchema,
  TrackingProviderRouteIdSchema,
  TrackingEscalationIdSchema,
  TrackingLiveTrackingGrantIdSchema,
  TrackingMissingDeviceCaseIdSchema,
  TrackingPolicyReasonCodeSchema,
  TrackingPolicyAuditRefSchema,
  TrackingPolicyTargetKindSchema,
  TrackingPolicyActionSchema,
  TrackingAlertSeveritySchema,
  TrackingAcknowledgementStateSchema,
  TrackingCheckInRequestStateSchema,
  TrackingCheckInResponseKindSchema,
  TrackingAiLocationRiskLevelSchema,
  TrackingAiProviderModeSchema,
  TrackingProviderCapabilityStateSchema,
  TrackingEscalationStateSchema,
  TrackingLiveTrackingGrantStateSchema,
  TrackingMissingDeviceStateSchema,
  TrackingPlatformProofRouteStateSchema,
  TrackingPlatformProofRouteSchema,
};

export type {
  TrackingEvidenceTrace,
  TrackingPolicyRule,
  TrackingPolicyDecision,
  TrackingAcknowledgement,
  TrackingChildCheckInRequest,
  TrackingChildCheckInResponse,
  TrackingLocationAiAnalysisInput,
  TrackingLocationAiAnalysisResult,
  TrackingAiProviderRoute,
  TrackingAlertIntent,
  TrackingEscalationChain,
  TrackingTemporaryLiveTrackingGrant,
  TrackingMissingDeviceCase,
  TrackingPlatformProofRoute,
  TrackingLocationPolicyReadModel,
};

function trackingAcknowledgementPreservesCriticalAlerts(acknowledgement: {
  readonly state: string;
  readonly stillAlertForCritical: boolean;
}) {
  if (
    acknowledgement.state === 'acknowledged-safe' ||
    acknowledgement.state === 'expected' ||
    acknowledgement.state === 'holiday-mode' ||
    acknowledgement.state === 'trip-exception' ||
    acknowledgement.state === 'false-alarm'
  ) {
    return acknowledgement.stillAlertForCritical;
  }

  return true;
}
