import {
  TrackingAlertIntentSchema,
  TrackingChildCheckInRequestSchema,
  TrackingEscalationChainSchema,
  TrackingPolicyDecisionSchema,
  TrackingTemporaryLiveTrackingGrantSchema,
} from '@ocentra-parent/schema-domain/tracking-location-policy';
import {
  TrackingPolicyReasonCodeSchema,
  TrackingPolicySchemaVersion,
} from '@ocentra-parent/schema-domain/tracking-location-policy-primitives';
import type {
  TrackingAlertIntent,
  TrackingChildCheckInRequest,
  TrackingEscalationChain,
  TrackingPolicyDecision,
  TrackingPolicyRule,
  TrackingTemporaryLiveTrackingGrant,
} from '@ocentra-parent/schema-domain/tracking-location-policy-types';
import {
  TrackingPolicyCompilerRuntimeProofRequestSchema,
  TrackingPolicyCompilerRuntimeProofResultSchema,
  type TrackingPolicyCompilerRuntimeProofRequest,
  type TrackingPolicyCompilerRuntimeProofResult,
} from '@ocentra-parent/schema-domain/tracking-policy-compiler-runtime-proof';

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
