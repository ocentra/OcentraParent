import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';
import {
  TrackingAlertIntentSchema,
  TrackingChildCheckInRequestSchema,
  TrackingEscalationChainSchema,
  TrackingEvidenceTraceSchema,
  TrackingLocationAiAnalysisResultSchema,
  TrackingPolicyDecisionSchema,
  TrackingPolicyRuleSchema,
  TrackingTemporaryLiveTrackingGrantSchema,
} from './tracking-location-policy';
import {
  TrackingAlertIdSchema,
  TrackingAlertSeveritySchema,
  TrackingCheckInIdSchema,
  TrackingEscalationIdSchema,
  TrackingLiveTrackingGrantIdSchema,
  TrackingPolicyAuditRefSchema,
  TrackingPolicyDecisionIdSchema,
  TrackingPolicyReasonCodeSchema,
  TrackingPolicySchemaVersion,
} from './tracking-location-policy-primitives';
import type {
  TrackingAlertIntent,
  TrackingChildCheckInRequest,
  TrackingEscalationChain,
  TrackingPolicyDecision,
  TrackingPolicyRule,
  TrackingTemporaryLiveTrackingGrant,
} from './tracking-location-policy-types';

const TrackingPolicyCompilerTextSchema = Schema.String.pipe(Schema.minLength(1));
const TrackingPolicyCompilerDurationSecondsSchema = Schema.Number.pipe(Schema.int(), Schema.nonNegative());

export const TrackingPolicyCompilerRuntimeProofRequestIdSchema = TrackingPolicyCompilerTextSchema.pipe(
  Schema.brand('TrackingPolicyCompilerRuntimeProofRequestId')
);

export const TrackingPolicyCompilerRuntimeProofModeSchema = withParser(Schema.Literal('dry-run', 'active'));

export const TrackingPolicyCompilerRequestedActionSchema = withParser(
  Schema.Literal(
    'observe',
    'notify-parent',
    'ask-child-check-in',
    'request-parent-acknowledgement',
    'start-temporary-live-tracking',
    'escalate',
    'critical-alert',
    'suppress',
    'manual-required'
  )
);

export const TrackingPolicyCompilerFinalActionSourceSchema = withParser(
  Schema.Literal('parent-policy-rule', 'disabled-rule', 'manual-required')
);

export const TrackingPolicyCompilerRuntimeProofRequestSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    requestId: TrackingPolicyCompilerRuntimeProofRequestIdSchema,
    rule: TrackingPolicyRuleSchema,
    requestedAt: ParentTimestampSchema,
    decidedAt: ParentTimestampSchema,
    followUpExpiresAt: ParentTimestampSchema,
    decisionId: TrackingPolicyDecisionIdSchema,
    requestedAction: TrackingPolicyCompilerRequestedActionSchema,
    compilerMode: TrackingPolicyCompilerRuntimeProofModeSchema,
    evidenceReferences: Schema.Array(TrackingEvidenceTraceSchema),
    aiAnalysis: Schema.Union(TrackingLocationAiAnalysisResultSchema, Schema.Null),
    alertId: Schema.Union(TrackingAlertIdSchema, Schema.Null),
    alertSeverity: Schema.Union(TrackingAlertSeveritySchema, Schema.Null),
    checkInId: Schema.Union(TrackingCheckInIdSchema, Schema.Null),
    escalationId: Schema.Union(TrackingEscalationIdSchema, Schema.Null),
    liveTrackingGrantId: Schema.Union(TrackingLiveTrackingGrantIdSchema, Schema.Null),
    liveTrackingDurationSeconds: Schema.Union(TrackingPolicyCompilerDurationSecondsSchema, Schema.Null),
    parentConfirmationReceived: Schema.Boolean,
    freshEvidenceAvailable: Schema.Boolean,
    platformManualRequired: Schema.Boolean,
    reasonCodes: Schema.Array(TrackingPolicyReasonCodeSchema),
    auditRefs: Schema.Array(TrackingPolicyAuditRefSchema),
  })
    .pipe(
      Schema.filter(
        (request) =>
          request.evidenceReferences.length > 0 ||
          request.requestedAction === 'manual-required' ||
          request.requestedAction === 'suppress' ||
          'Tracking policy compiler requests need cited evidence except manual-required or suppress previews'
      )
    )
    .pipe(
      Schema.filter(
        (request) =>
          request.rule.action !== 'ask-child-check-in' ||
          request.checkInId !== null ||
          'Tracking child check-in policy actions need a check-in id'
      )
    )
    .pipe(
      Schema.filter(
        (request) =>
          request.rule.action !== 'start-temporary-live-tracking' ||
          (request.liveTrackingGrantId !== null && request.liveTrackingDurationSeconds !== null) ||
          'Tracking temporary live policy actions need a grant id and duration'
      )
    )
    .pipe(
      Schema.filter(
        (request) =>
          !trackingPolicyCompilerActionNeedsAlert(request.rule.action) ||
          request.alertId !== null ||
          'Tracking alert policy actions need an alert id'
      )
    )
);

export const TrackingPolicyCompilerRuntimeProofResultSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    requestId: TrackingPolicyCompilerRuntimeProofRequestIdSchema,
    requestedAction: TrackingPolicyCompilerRequestedActionSchema,
    finalActionSource: TrackingPolicyCompilerFinalActionSourceSchema,
    decision: TrackingPolicyDecisionSchema,
    alertIntent: Schema.Union(TrackingAlertIntentSchema, Schema.Null),
    childCheckInRequest: Schema.Union(TrackingChildCheckInRequestSchema, Schema.Null),
    escalationChain: Schema.Union(TrackingEscalationChainSchema, Schema.Null),
    temporaryLiveGrant: Schema.Union(TrackingTemporaryLiveTrackingGrantSchema, Schema.Null),
    parentPolicyFinalAuthority: Schema.Literal(true),
    aiFinalAuthority: Schema.Literal(false),
    runtimeEnforcementClaimed: Schema.Literal(false),
    providerDeliveryClaimed: Schema.Literal(false),
    platformAdapterClaimed: Schema.Literal(false),
    physicalDeviceClaimed: Schema.Literal(false),
    productionWorkerClaimed: Schema.Literal(false),
  })
);

export type TrackingPolicyCompilerRuntimeProofRequest = Infer<typeof TrackingPolicyCompilerRuntimeProofRequestSchema>;
export type TrackingPolicyCompilerRuntimeProofResult = Infer<typeof TrackingPolicyCompilerRuntimeProofResultSchema>;

export function compileTrackingPolicyRuntimeProofDecision(
  input: TrackingPolicyCompilerRuntimeProofRequest
): TrackingPolicyCompilerRuntimeProofResult {
  const request = TrackingPolicyCompilerRuntimeProofRequestSchema.parse(input);
  const finalActionSource = finalActionSourceFor(request);
  const action = actionFor(request, finalActionSource);
  const reasonCodes = reasonCodesFor(request, action, finalActionSource);
  const alertIntentId = trackingPolicyCompilerActionNeedsAlert(action) ? request.alertId : null;
  const decision = TrackingPolicyDecisionSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    decisionId: request.decisionId,
    decidedAt: request.decidedAt,
    ruleId: request.rule.ruleId,
    action,
    dryRun: request.compilerMode === 'dry-run',
    evidenceReferences: request.evidenceReferences,
    aiAnalysisId: request.aiAnalysis?.analysisId ?? null,
    alertIntentId,
    reasonCodes,
    auditRefs: request.auditRefs,
  });

  return TrackingPolicyCompilerRuntimeProofResultSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    requestId: request.requestId,
    requestedAction: request.requestedAction,
    finalActionSource,
    decision,
    alertIntent: alertFor(request, decision, action, reasonCodes),
    childCheckInRequest: childCheckInFor(request, action),
    escalationChain: escalationFor(request, action),
    temporaryLiveGrant: liveGrantFor(request, action),
    parentPolicyFinalAuthority: true,
    aiFinalAuthority: false,
    runtimeEnforcementClaimed: false,
    providerDeliveryClaimed: false,
    platformAdapterClaimed: false,
    physicalDeviceClaimed: false,
    productionWorkerClaimed: false,
  });
}

export function trackingPolicyCompilerActionNeedsAlert(action: TrackingPolicyRule['action']): boolean {
  return action === 'notify-parent' || action === 'request-parent-acknowledgement' || action === 'escalate';
}

function actionFor(
  request: TrackingPolicyCompilerRuntimeProofRequest,
  finalActionSource: TrackingPolicyCompilerRuntimeProofResult['finalActionSource']
): TrackingPolicyDecision['action'] {
  if (finalActionSource === 'disabled-rule') return 'no-action';
  if (finalActionSource === 'manual-required') return 'manual-required';
  if (request.requestedAction === 'suppress' && request.rule.action === 'no-action') return 'no-action';
  return request.rule.action;
}

function finalActionSourceFor(
  request: TrackingPolicyCompilerRuntimeProofRequest
): TrackingPolicyCompilerRuntimeProofResult['finalActionSource'] {
  if (!request.rule.enabled) return 'disabled-rule';
  if (
    request.platformManualRequired ||
    (request.rule.requiresFreshEvidence && !request.freshEvidenceAvailable) ||
    (request.rule.requiresParentConfirmation && !request.parentConfirmationReceived)
  ) {
    return 'manual-required';
  }
  return 'parent-policy-rule';
}

function alertFor(
  request: TrackingPolicyCompilerRuntimeProofRequest,
  decision: TrackingPolicyDecision,
  action: TrackingPolicyDecision['action'],
  reasonCodes: readonly TrackingPolicyDecision['reasonCodes'][number][]
): TrackingAlertIntent | null {
  if (!trackingPolicyCompilerActionNeedsAlert(action) || request.alertId === null) return null;
  return TrackingAlertIntentSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    alertId: request.alertId,
    createdAt: request.decidedAt,
    severity: alertSeverityFor(request, action),
    policyDecisionId: decision.decisionId,
    evidenceReferences: request.evidenceReferences,
    sensitiveDetailMode: 'minimal-provider-body',
    notificationStatusRefs: request.auditRefs,
    acknowledgementId: null,
    reasonCodes,
  });
}

function childCheckInFor(
  request: TrackingPolicyCompilerRuntimeProofRequest,
  action: TrackingPolicyDecision['action']
): TrackingChildCheckInRequest | null {
  if (action !== 'ask-child-check-in' || request.checkInId === null) return null;
  return TrackingChildCheckInRequestSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    checkInId: request.checkInId,
    requestedAt: request.decidedAt,
    state: 'sent',
    relatedAlertId: request.alertId,
    includeLocationIfPermitted: true,
    expiresAt: request.followUpExpiresAt,
    evidenceReferences: request.evidenceReferences,
    auditRefs: request.auditRefs,
  });
}

function escalationFor(
  request: TrackingPolicyCompilerRuntimeProofRequest,
  action: TrackingPolicyDecision['action']
): TrackingEscalationChain | null {
  if (action !== 'escalate' || request.alertId === null || request.escalationId === null) return null;
  return TrackingEscalationChainSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    escalationId: request.escalationId,
    alertId: request.alertId,
    state: 'waiting-for-parent',
    startedAt: request.decidedAt,
    nextActionAt: request.followUpExpiresAt,
    steps: ['notify-parent', 'ask-child-check-in', 'manual-review'],
    auditRefs: request.auditRefs,
  });
}

function liveGrantFor(
  request: TrackingPolicyCompilerRuntimeProofRequest,
  action: TrackingPolicyDecision['action']
): TrackingTemporaryLiveTrackingGrant | null {
  if (
    action !== 'start-temporary-live-tracking' ||
    request.liveTrackingGrantId === null ||
    request.liveTrackingDurationSeconds === null
  ) {
    return null;
  }
  return TrackingTemporaryLiveTrackingGrantSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    grantId: request.liveTrackingGrantId,
    state: 'requested',
    requestedAt: request.decidedAt,
    expiresAt: request.followUpExpiresAt,
    durationSeconds: request.liveTrackingDurationSeconds,
    parentApproved: request.parentConfirmationReceived,
    childDisclosureRequired: true,
    auditRefs: request.auditRefs,
  });
}

function alertSeverityFor(
  request: TrackingPolicyCompilerRuntimeProofRequest,
  action: TrackingPolicyDecision['action']
): TrackingAlertIntent['severity'] {
  if (request.requestedAction === 'critical-alert') return 'critical';
  if (action === 'escalate') return 'urgent';
  return request.alertSeverity ?? 'watch';
}

function reasonCodesFor(
  request: TrackingPolicyCompilerRuntimeProofRequest,
  action: TrackingPolicyDecision['action'],
  finalActionSource: TrackingPolicyCompilerRuntimeProofResult['finalActionSource']
) {
  const reasonCodes = [
    ...request.rule.reasonCodes,
    ...request.reasonCodes,
    reasonCode('parent-policy-final-authority'),
    reasonCodeForAction(action),
  ];
  if (finalActionSource === 'disabled-rule') reasonCodes.push(reasonCode('tracking-rule-disabled'));
  if (finalActionSource === 'manual-required') reasonCodes.push(reasonCode('tracking-manual-required'));
  if (candidateDiffersFromAction(request, action)) reasonCodes.push(reasonCode('parent-policy-overrode-candidate'));
  if (request.aiAnalysis !== null) reasonCodes.push(reasonCode('ai-evidence-not-final-authority'));
  return [...new Set(reasonCodes)];
}

function candidateDiffersFromAction(
  request: TrackingPolicyCompilerRuntimeProofRequest,
  action: TrackingPolicyDecision['action']
): boolean {
  if (request.requestedAction === 'critical-alert') return action !== 'notify-parent' && action !== 'escalate';
  if (request.requestedAction === 'suppress') return action !== 'no-action';
  return request.requestedAction !== action;
}

function reasonCodeForAction(action: TrackingPolicyDecision['action']) {
  return reasonCode(`tracking-policy-action-${action}`);
}

function reasonCode(value: string) {
  return TrackingPolicyReasonCodeSchema.parse(value);
}
