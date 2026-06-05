import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentEvidenceReferenceIdSchema, ParentTimestampSchema } from './reference-primitives';
import {
  TrackingAcknowledgementIdSchema,
  TrackingAlertIdSchema,
  TrackingCheckInIdSchema,
  TrackingEscalationIdSchema,
  TrackingPolicyAuditRefSchema,
  TrackingPolicyDecisionIdSchema,
  TrackingPolicyReasonCodeSchema,
  TrackingPolicySchemaVersion,
} from './tracking-location-policy-primitives';
import {
  TrackingAcknowledgementSchema,
  TrackingAlertIntentSchema,
  TrackingChildCheckInRequestSchema,
  TrackingChildCheckInResponseSchema,
  TrackingEscalationChainSchema,
  TrackingEvidenceTraceSchema,
  TrackingLocationAiAnalysisResultSchema,
  TrackingLocationPolicyReadModelSchema,
  TrackingPolicyDecisionSchema,
  TrackingPolicyRuleSchema,
  evaluateTrackingAcknowledgementImpact,
  resolveTrackingChildCheckIn,
} from './tracking-location-policy';

type TrackingPolicyRuleAction = Infer<typeof TrackingPolicyRuleSchema>['action'];
type TrackingPolicyDecisionAction = Infer<typeof TrackingPolicyDecisionSchema>['action'];
type TrackingAlertSeverity = Infer<typeof TrackingAlertIntentSchema>['severity'];
type TrackingCheckInRequestState = Infer<typeof TrackingChildCheckInRequestSchema>['state'];

const RuntimeProofText = Schema.String.pipe(Schema.minLength(1));
type RuntimeProofTextValue = Infer<typeof RuntimeProofText>;

const RuntimeProofVersion = 'tracking-policy-escalation-runtime-proof';
const RuntimeProofGeneratedAt = '2026-06-05T01:30:00.000Z';
const RuntimeProofPolicyVersion = 'tracking-policy-escalation-v1';
const RuntimeProofEvaluatedAt = '2026-06-05T01:12:00.000Z';

const RequiredOutcomeKinds = [
  'ai-analysis-cannot-trigger-alert-directly',
  'parent-acknowledgement-suppresses-warning',
  'critical-alert-remains-visible',
  'safe-child-check-in-resolves',
  'expired-child-check-in-escalates-by-policy',
] as const;

const RuntimeProofNonClaims = [
  'no-provider-delivery-attempted',
  'no-emergency-contact-automation',
  'no-child-device-runtime',
  'no-background-location-claim',
  'no-physical-device-proof',
  'no-ai-final-authority',
] as const;

export const TrackingPolicyEscalationRuntimeProofSchemaVersionSchema = withParser(Schema.Literal(RuntimeProofVersion));
const TrackingPolicyEscalationRuntimeOutcomeKindSchema = withParser(Schema.Literal(...RequiredOutcomeKinds));
const TrackingPolicyEscalationRuntimeStateSchema = withParser(
  Schema.Literal(
    'ai-advisory-only',
    'suppressed-by-acknowledgement',
    'critical-still-alert',
    'check-in-safe',
    'policy-escalated'
  )
);
const TrackingPolicyEscalationRuntimeNonClaimSchema = withParser(Schema.Literal(...RuntimeProofNonClaims));

type TrackingPolicyEscalationRuntimeOutcomeKind = Infer<typeof TrackingPolicyEscalationRuntimeOutcomeKindSchema>;
type TrackingPolicyEscalationRuntimeState = Infer<typeof TrackingPolicyEscalationRuntimeStateSchema>;

const RuntimeOutcomeIdSchema = RuntimeProofText.pipe(Schema.brand('TrackingPolicyEscalationRuntimeOutcomeId'));
const RuntimeProofRefSchema = RuntimeProofText.pipe(Schema.brand('TrackingPolicyEscalationRuntimeProofRef'));
const RuntimeClaimBoundarySchema = RuntimeProofText.pipe(Schema.brand('TrackingPolicyEscalationRuntimeClaimBoundary'));

const TrackingPolicyEscalationRuntimeOutcomeBaseSchema = Schema.Struct({
  schemaVersion: TrackingPolicyEscalationRuntimeProofSchemaVersionSchema,
  outcomeId: RuntimeOutcomeIdSchema,
  outcomeKind: TrackingPolicyEscalationRuntimeOutcomeKindSchema,
  policyDecisionId: Schema.Union(TrackingPolicyDecisionIdSchema, Schema.Null),
  alertId: Schema.Union(TrackingAlertIdSchema, Schema.Null),
  acknowledgementId: Schema.Union(TrackingAcknowledgementIdSchema, Schema.Null),
  checkInId: Schema.Union(TrackingCheckInIdSchema, Schema.Null),
  escalationId: Schema.Union(TrackingEscalationIdSchema, Schema.Null),
  runtimeState: TrackingPolicyEscalationRuntimeStateSchema,
  parentPolicyFinalAuthority: Schema.Boolean,
  aiFinalAuthorityClaimed: Schema.Literal(false),
  aiDirectAlertClaimed: Schema.Literal(false),
  parentAlertSuppressed: Schema.Boolean,
  escalates: Schema.Boolean,
  resolved: Schema.Boolean,
  providerDeliveryAttempted: Schema.Literal(false),
  emergencyContactClaimed: Schema.Literal(false),
  deviceRuntimeClaimed: Schema.Literal(false),
  evidenceReferenceIds: Schema.Array(ParentEvidenceReferenceIdSchema),
  reasonCodes: Schema.Array(TrackingPolicyReasonCodeSchema),
  auditRefs: Schema.Array(TrackingPolicyAuditRefSchema),
  proofRefs: Schema.Array(RuntimeProofRefSchema),
  claimBoundary: RuntimeClaimBoundarySchema,
  evaluatedAt: ParentTimestampSchema,
});

type RuntimeOutcomeCandidate = Infer<typeof TrackingPolicyEscalationRuntimeOutcomeBaseSchema>;

export const TrackingPolicyEscalationRuntimeOutcomeSchema = withParser(
  TrackingPolicyEscalationRuntimeOutcomeBaseSchema.pipe(
    Schema.filter(
      (outcome) =>
        runtimeOutcomeIsHonest(outcome) ||
        'Expected tracking policy escalation runtime outcomes to cite evidence without provider delivery, emergency contact, device runtime, or AI authority claims'
    )
  )
);

const TrackingPolicyEscalationRuntimeProofBaseSchema = Schema.Struct({
  schemaVersion: TrackingPolicyEscalationRuntimeProofSchemaVersionSchema,
  generatedAt: ParentTimestampSchema,
  evaluatedAt: ParentTimestampSchema,
  sourcePolicyVersion: Schema.Literal(RuntimeProofPolicyVersion),
  outcomes: Schema.Array(TrackingPolicyEscalationRuntimeOutcomeSchema),
  nonClaims: Schema.Array(TrackingPolicyEscalationRuntimeNonClaimSchema),
  knownGaps: Schema.Array(RuntimeProofRefSchema),
  productClaimReady: Schema.Literal(false),
  providerDeliveryRuntimeClaimed: Schema.Literal(false),
  emergencyContactRuntimeClaimed: Schema.Literal(false),
  childDeviceRuntimeClaimed: Schema.Literal(false),
});

export type TrackingPolicyEscalationRuntimeOutcome = Infer<typeof TrackingPolicyEscalationRuntimeOutcomeSchema>;
export type TrackingPolicyEscalationRuntimeProof = Infer<typeof TrackingPolicyEscalationRuntimeProofBaseSchema>;

export const TrackingPolicyEscalationRuntimeProofSchema = withParser(
  TrackingPolicyEscalationRuntimeProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        runtimeProofCoversRequiredOutcomes(proof) ||
        'Expected tracking policy escalation runtime proof to cover AI, acknowledgement, check-in, and explicit escalation outcomes while preserving non-claims'
    )
  )
);

export const TrackingPolicyEscalationRuntimeKnownGaps = [
  'Provider delivery is not attempted; notification provider receipt and retry proof remain separate work.',
  'Child-device runtime prompt delivery and physical Android/iOS background behavior remain unproved.',
  'Emergency contact automation is not implemented or claimed; escalation remains parent-policy/manual proof only.',
] as const;

const RuntimeEvidence = TrackingEvidenceTraceSchema.parse({
  evidenceReferenceId: 'tracking-runtime-policy-evidence',
  kind: 'journal-event',
  observedAt: '2026-06-05T01:00:00.000Z',
});

const ProofReadModel = TrackingLocationPolicyReadModelSchema.parse({
  schemaVersion: TrackingPolicySchemaVersion,
  generatedAt: RuntimeProofGeneratedAt,
  rules: [
    runtimeRule('tracking-escalation-notify-rule', 'notify-parent'),
    runtimeRule('tracking-escalation-check-in-rule', 'ask-child-check-in'),
    runtimeRule('tracking-escalation-policy-rule', 'escalate'),
  ],
  decisions: [
    runtimeDecision('tracking-decision-notify', 'tracking-escalation-notify-rule', 'notify-parent'),
    runtimeDecision('tracking-decision-check-in', 'tracking-escalation-check-in-rule', 'ask-child-check-in'),
    runtimeDecision('tracking-decision-escalate', 'tracking-escalation-policy-rule', 'escalate'),
  ],
  acknowledgements: [
    TrackingAcknowledgementSchema.parse({
      schemaVersion: TrackingPolicySchemaVersion,
      acknowledgementId: 'tracking-runtime-ack-safe',
      alertId: 'tracking-runtime-alert-warning',
      state: 'acknowledged-safe',
      acknowledgedAt: '2026-06-05T01:05:00.000Z',
      expiresAt: null,
      stillAlertForCritical: true,
      reasonCodes: ['parent-confirmed-safe'],
      auditRefs: ['tracking-runtime-ack-recorded'],
    }),
  ],
  checkInRequests: [
    runtimeCheckInRequest('tracking-runtime-checkin-safe', 'tracking-runtime-alert-warning', 'sent'),
    runtimeCheckInRequest('tracking-runtime-checkin-expired', 'tracking-runtime-alert-critical', 'sent'),
  ],
  checkInResponses: [
    TrackingChildCheckInResponseSchema.parse({
      schemaVersion: TrackingPolicySchemaVersion,
      checkInId: 'tracking-runtime-checkin-safe',
      respondedAt: '2026-06-05T01:07:00.000Z',
      response: 'safe',
      locationEvidenceReference: RuntimeEvidence,
      auditRefs: ['tracking-runtime-safe-checkin-response'],
    }),
  ],
  aiRoutes: [],
  aiResults: [
    TrackingLocationAiAnalysisResultSchema.parse({
      schemaVersion: TrackingPolicySchemaVersion,
      analysisId: 'tracking-runtime-ai-analysis',
      completedAt: '2026-06-05T01:01:00.000Z',
      riskLevel: 'moderate',
      confidence: 0.72,
      providerRouteId: 'tracking-runtime-metadata-only-route',
      evidenceReferences: [RuntimeEvidence],
      reasonCodes: ['tracking-ai-advisory-only'],
      canTriggerAlertDirectly: false,
      isFinalAuthority: false,
    }),
  ],
  alerts: [
    runtimeAlert('tracking-runtime-alert-warning', 'tracking-decision-notify', 'warning'),
    runtimeAlert('tracking-runtime-alert-critical', 'tracking-decision-escalate', 'critical'),
  ],
  escalations: [
    TrackingEscalationChainSchema.parse({
      schemaVersion: TrackingPolicySchemaVersion,
      escalationId: 'tracking-runtime-escalation-expired-checkin',
      alertId: 'tracking-runtime-alert-critical',
      state: 'waiting-for-parent',
      startedAt: '2026-06-05T01:12:00.000Z',
      nextActionAt: null,
      steps: ['request-parent-review', 'show-manual-escalation'],
      auditRefs: ['tracking-runtime-policy-escalation-created'],
    }),
  ],
  temporaryLiveGrants: [],
  missingDeviceCases: [],
  platformProofRoutes: [],
});

const WarningAlert = TrackingAlertIntentSchema.parse(ProofReadModel.alerts[0]);
const CriticalAlert = TrackingAlertIntentSchema.parse(ProofReadModel.alerts[1]);
const SafeAcknowledgement = TrackingAcknowledgementSchema.parse(ProofReadModel.acknowledgements[0]);
const SafeCheckInRequest = TrackingChildCheckInRequestSchema.parse(ProofReadModel.checkInRequests[0]);
const ExpiredCheckInRequest = TrackingChildCheckInRequestSchema.parse(ProofReadModel.checkInRequests[1]);
const SafeCheckInResponse = TrackingChildCheckInResponseSchema.parse(ProofReadModel.checkInResponses[0]);
const ExpiredEscalation = TrackingEscalationChainSchema.parse(ProofReadModel.escalations[0]);

const WarningAcknowledgementImpact = evaluateTrackingAcknowledgementImpact({
  alert: WarningAlert,
  acknowledgement: SafeAcknowledgement,
  evaluatedAt: RuntimeProofEvaluatedAt,
});
const CriticalAcknowledgementImpact = evaluateTrackingAcknowledgementImpact({
  alert: CriticalAlert,
  acknowledgement: SafeAcknowledgement,
  evaluatedAt: RuntimeProofEvaluatedAt,
});
const SafeCheckInResolution = resolveTrackingChildCheckIn({
  request: SafeCheckInRequest,
  response: SafeCheckInResponse,
  evaluatedAt: RuntimeProofEvaluatedAt,
});
const ExpiredCheckInResolution = resolveTrackingChildCheckIn({
  request: ExpiredCheckInRequest,
  response: null,
  evaluatedAt: RuntimeProofEvaluatedAt,
});

export const TrackingPolicyEscalationRuntimeProofReadModel = TrackingPolicyEscalationRuntimeProofSchema.parse({
  schemaVersion: RuntimeProofVersion,
  generatedAt: RuntimeProofGeneratedAt,
  evaluatedAt: RuntimeProofEvaluatedAt,
  sourcePolicyVersion: RuntimeProofPolicyVersion,
  outcomes: [
    runtimeOutcome('ai-analysis-cannot-trigger-alert-directly', 'ai-advisory-only', {
      policyDecisionId: 'tracking-decision-notify',
      alertId: 'tracking-runtime-alert-warning',
      parentPolicyFinalAuthority: true,
      parentAlertSuppressed: false,
      escalates: false,
      resolved: true,
      reasonCodes: ['tracking-ai-advisory-only'],
      auditRefs: ['tracking-runtime-ai-authority-boundary'],
    }),
    runtimeOutcome(
      'parent-acknowledgement-suppresses-warning',
      acknowledgementRuntimeState(WarningAcknowledgementImpact.state),
      {
        policyDecisionId: 'tracking-decision-notify',
        alertId: WarningAcknowledgementImpact.alertId,
        acknowledgementId: WarningAcknowledgementImpact.acknowledgementId,
        parentPolicyFinalAuthority: true,
        parentAlertSuppressed: WarningAcknowledgementImpact.suppressesParentAlert,
        escalates: false,
        resolved: true,
        reasonCodes: WarningAcknowledgementImpact.reasonCodes,
        auditRefs: ['tracking-runtime-warning-ack-suppressed'],
      }
    ),
    runtimeOutcome('critical-alert-remains-visible', acknowledgementRuntimeState(CriticalAcknowledgementImpact.state), {
      policyDecisionId: 'tracking-decision-escalate',
      alertId: CriticalAcknowledgementImpact.alertId,
      acknowledgementId: CriticalAcknowledgementImpact.acknowledgementId,
      parentPolicyFinalAuthority: true,
      parentAlertSuppressed: CriticalAcknowledgementImpact.suppressesParentAlert,
      escalates: true,
      resolved: false,
      reasonCodes: CriticalAcknowledgementImpact.reasonCodes,
      auditRefs: ['tracking-runtime-critical-not-suppressed'],
    }),
    runtimeOutcome('safe-child-check-in-resolves', 'check-in-safe', {
      policyDecisionId: 'tracking-decision-check-in',
      alertId: 'tracking-runtime-alert-warning',
      checkInId: SafeCheckInResolution.checkInId,
      parentPolicyFinalAuthority: true,
      parentAlertSuppressed: true,
      escalates: SafeCheckInResolution.escalates,
      resolved: true,
      reasonCodes: SafeCheckInResolution.reasonCodes,
      auditRefs: ['tracking-runtime-safe-checkin-resolved'],
    }),
    runtimeOutcome('expired-child-check-in-escalates-by-policy', 'policy-escalated', {
      policyDecisionId: 'tracking-decision-escalate',
      alertId: ExpiredEscalation.alertId,
      checkInId: ExpiredCheckInResolution.checkInId,
      escalationId: ExpiredEscalation.escalationId,
      parentPolicyFinalAuthority: true,
      parentAlertSuppressed: false,
      escalates: ExpiredCheckInResolution.escalates,
      resolved: false,
      reasonCodes: ExpiredCheckInResolution.reasonCodes,
      auditRefs: ['tracking-runtime-expired-checkin-policy-escalation'],
    }),
  ],
  nonClaims: RuntimeProofNonClaims,
  knownGaps: TrackingPolicyEscalationRuntimeKnownGaps,
  productClaimReady: false,
  providerDeliveryRuntimeClaimed: false,
  emergencyContactRuntimeClaimed: false,
  childDeviceRuntimeClaimed: false,
});

export function summarizeTrackingPolicyEscalationRuntimeProof(proof: TrackingPolicyEscalationRuntimeProof) {
  return {
    outcomes: proof.outcomes.length,
    parentPolicyAuthorityRows: proof.outcomes.filter((outcome) => outcome.parentPolicyFinalAuthority).length,
    aiAuthorityRows: proof.outcomes.filter((outcome) => outcome.aiFinalAuthorityClaimed).length,
    providerDeliveryRows: proof.outcomes.filter((outcome) => outcome.providerDeliveryAttempted).length,
    emergencyContactRows: proof.outcomes.filter((outcome) => outcome.emergencyContactClaimed).length,
    deviceRuntimeRows: proof.outcomes.filter((outcome) => outcome.deviceRuntimeClaimed).length,
    escalationRows: proof.outcomes.filter((outcome) => outcome.escalates).length,
    resolvedRows: proof.outcomes.filter((outcome) => outcome.resolved).length,
  } as const;
}

function runtimeRule(ruleId: RuntimeProofTextValue, action: TrackingPolicyRuleAction) {
  return TrackingPolicyRuleSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    ruleId,
    familyId: 'tracking-runtime-family',
    childProfileId: 'tracking-runtime-child',
    deviceId: 'tracking-runtime-parent-device',
    policyVersion: RuntimeProofPolicyVersion,
    targetKind: action === 'ask-child-check-in' ? 'child-check-in' : 'expected-place',
    action,
    enabled: true,
    requiresFreshEvidence: true,
    requiresParentConfirmation: action === 'escalate',
    reasonCodes: [`${ruleId}-reason`],
    auditRefs: [`${ruleId}-audit`],
  });
}

function runtimeDecision(
  decisionId: RuntimeProofTextValue,
  ruleId: RuntimeProofTextValue,
  action: TrackingPolicyDecisionAction
) {
  return TrackingPolicyDecisionSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    decisionId,
    decidedAt: '2026-06-05T01:02:00.000Z',
    ruleId,
    action,
    dryRun: false,
    evidenceReferences: [RuntimeEvidence],
    aiAnalysisId: 'tracking-runtime-ai-analysis',
    alertIntentId: action === 'notify-parent' || action === 'escalate' ? alertIdForAction(action) : null,
    reasonCodes: [`${decisionId}-reason`],
    auditRefs: [`${decisionId}-audit`],
  });
}

function runtimeAlert(
  alertId: RuntimeProofTextValue,
  policyDecisionId: RuntimeProofTextValue,
  severity: TrackingAlertSeverity
) {
  return TrackingAlertIntentSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    alertId,
    createdAt: '2026-06-05T01:03:00.000Z',
    severity,
    policyDecisionId,
    evidenceReferences: [RuntimeEvidence],
    sensitiveDetailMode: 'minimal-provider-body',
    notificationStatusRefs: ['tracking-runtime-local-notification-intent'],
    acknowledgementId: 'tracking-runtime-ack-safe',
    reasonCodes: [`${alertId}-reason`],
  });
}

function runtimeCheckInRequest(
  checkInId: RuntimeProofTextValue,
  relatedAlertId: RuntimeProofTextValue,
  state: TrackingCheckInRequestState
) {
  return TrackingChildCheckInRequestSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    checkInId,
    requestedAt: '2026-06-05T01:04:00.000Z',
    state,
    relatedAlertId,
    includeLocationIfPermitted: true,
    expiresAt: '2026-06-05T01:10:00.000Z',
    evidenceReferences: [RuntimeEvidence],
    auditRefs: [`${checkInId}-audit`],
  });
}

function runtimeOutcome(
  outcomeKind: TrackingPolicyEscalationRuntimeOutcomeKind,
  runtimeState: TrackingPolicyEscalationRuntimeState,
  input: {
    readonly policyDecisionId: RuntimeProofTextValue;
    readonly alertId: RuntimeProofTextValue;
    readonly acknowledgementId?: RuntimeProofTextValue;
    readonly checkInId?: RuntimeProofTextValue;
    readonly escalationId?: RuntimeProofTextValue;
    readonly parentPolicyFinalAuthority: boolean;
    readonly parentAlertSuppressed: boolean;
    readonly escalates: boolean;
    readonly resolved: boolean;
    readonly reasonCodes: ReadonlyArray<RuntimeProofTextValue>;
    readonly auditRefs: ReadonlyArray<RuntimeProofTextValue>;
  }
) {
  return TrackingPolicyEscalationRuntimeOutcomeSchema.parse({
    schemaVersion: RuntimeProofVersion,
    outcomeId: `tracking-runtime-${outcomeKind}`,
    outcomeKind,
    policyDecisionId: input.policyDecisionId,
    alertId: input.alertId,
    acknowledgementId: input.acknowledgementId ?? null,
    checkInId: input.checkInId ?? null,
    escalationId: input.escalationId ?? null,
    runtimeState,
    parentPolicyFinalAuthority: input.parentPolicyFinalAuthority,
    aiFinalAuthorityClaimed: false,
    aiDirectAlertClaimed: false,
    parentAlertSuppressed: input.parentAlertSuppressed,
    escalates: input.escalates,
    resolved: input.resolved,
    providerDeliveryAttempted: false,
    emergencyContactClaimed: false,
    deviceRuntimeClaimed: false,
    evidenceReferenceIds: [RuntimeEvidence.evidenceReferenceId],
    reasonCodes: input.reasonCodes,
    auditRefs: input.auditRefs,
    proofRefs: [
      'docs/plans/tracking-plan/workpacks/25-policy-compiler-for-tracking-rules.md',
      'docs/plans/tracking-plan/workpacks/27-escalation-engine.md',
    ],
    claimBoundary:
      'Local parent-domain runtime proof only; provider delivery, emergency automation, and child-device runtime are not claimed.',
    evaluatedAt: RuntimeProofEvaluatedAt,
  });
}

function runtimeOutcomeIsHonest(outcome: RuntimeOutcomeCandidate): boolean {
  return (
    outcome.evidenceReferenceIds.length > 0 &&
    outcome.proofRefs.length > 0 &&
    outcome.aiFinalAuthorityClaimed === false &&
    outcome.aiDirectAlertClaimed === false &&
    outcome.providerDeliveryAttempted === false &&
    outcome.emergencyContactClaimed === false &&
    outcome.deviceRuntimeClaimed === false &&
    criticalAlertsStayVisible(outcome)
  );
}

function runtimeProofCoversRequiredOutcomes(proof: TrackingPolicyEscalationRuntimeProof): boolean {
  const outcomeKinds = new Set(proof.outcomes.map((outcome) => outcome.outcomeKind));
  return (
    RequiredOutcomeKinds.every((outcomeKind) => outcomeKinds.has(outcomeKind)) &&
    RuntimeProofNonClaims.every((nonClaim) => proof.nonClaims.includes(nonClaim)) &&
    proof.productClaimReady === false &&
    proof.providerDeliveryRuntimeClaimed === false &&
    proof.emergencyContactRuntimeClaimed === false &&
    proof.childDeviceRuntimeClaimed === false
  );
}

function criticalAlertsStayVisible(outcome: RuntimeOutcomeCandidate): boolean {
  return outcome.runtimeState !== 'critical-still-alert' || !outcome.parentAlertSuppressed;
}

function acknowledgementRuntimeState(
  state: ReturnType<typeof evaluateTrackingAcknowledgementImpact>['state']
): TrackingPolicyEscalationRuntimeState {
  if (state === 'critical-still-alert') {
    return 'critical-still-alert';
  }
  if (state === 'suppressed-by-acknowledgement') {
    return 'suppressed-by-acknowledgement';
  }
  return 'ai-advisory-only';
}

function alertIdForAction(action: TrackingPolicyDecisionAction): RuntimeProofTextValue {
  return action === 'escalate' ? 'tracking-runtime-alert-critical' : 'tracking-runtime-alert-warning';
}
