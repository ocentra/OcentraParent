import { type Infer, Schema, withParser, brandedNonEmptyStringSchema, NonEmptyStringSchema } from './effect';
import { TrackingLocationPolicyReadModelSchema, TrackingPolicySchemaVersion } from './tracking-location-policy';
import { evaluateTrackingAcknowledgementImpact, resolveTrackingChildCheckIn } from './tracking-location-policy-runtime';
import { TrackingPolicyAuditRefSchema } from './tracking-location-policy-primitives';
import type {
  TrackingAcknowledgement,
  TrackingAlertIntent,
  TrackingChildCheckInRequest,
  TrackingChildCheckInResponse,
  TrackingEscalationChain,
  TrackingLocationPolicyReadModel,
  TrackingPolicyDecision,
} from './tracking-location-policy-types';

export const RequiredTrackingEscalationReadinessNonClaims = [
  'no-emergency-auto-contact',
  'no-provider-delivery-execution',
  'no-provider-receipt-ingestion',
  'no-provider-credentials',
  'no-cloud-routing',
  'no-parent-notification-ui',
  'no-child-device-delivery',
  'no-physical-device-proof',
  'no-production-escalation-worker',
  'no-production-quiet-hours-timer',
] as const;

export const TrackingEscalationReadinessNonClaimSchema = withParser(
  Schema.Literal(...RequiredTrackingEscalationReadinessNonClaims)
);

export const TrackingEscalationReadinessIdSchema = brandedNonEmptyStringSchema('TrackingEscalationReadinessId');
export const TrackingEscalationReadinessRowIdSchema = brandedNonEmptyStringSchema('TrackingEscalationReadinessRowId');
export const TrackingEscalationReadinessStateSchema = withParser(
  Schema.Literal(
    'waiting-for-parent',
    'waiting-for-child',
    'second-guardian-required',
    'critical-multi-channel-manual-required',
    'resolved-by-parent-acknowledgement',
    'resolved-by-child-check-in',
    'manual-required',
    'unavailable'
  )
);

const TrackingEscalationReadinessRowBaseSchema = Schema.Struct({
  rowId: TrackingEscalationReadinessRowIdSchema,
  alertId: NonEmptyStringSchema,
  sourceEscalationId: Schema.Union(NonEmptyStringSchema, Schema.Null),
  sourceDecisionId: NonEmptyStringSchema,
  severity: NonEmptyStringSchema,
  readinessState: TrackingEscalationReadinessStateSchema,
  nextActionAt: Schema.Union(NonEmptyStringSchema, Schema.Null),
  evidenceReferenceIds: Schema.Array(NonEmptyStringSchema),
  policyDecisionRefs: Schema.Array(NonEmptyStringSchema),
  acknowledgementRefs: Schema.Array(NonEmptyStringSchema),
  childCheckInRefs: Schema.Array(NonEmptyStringSchema),
  guardianActionRefs: Schema.Array(NonEmptyStringSchema),
  manualProofRequirements: Schema.Array(NonEmptyStringSchema),
  reasonRefs: Schema.Array(TrackingPolicyAuditRefSchema),
  parentAcknowledgementCancelsEscalation: Schema.Boolean,
  childCheckInCancelsEscalation: Schema.Boolean,
  aiScheduledEscalation: Schema.Literal(false),
  emergencyServicesAutoContactClaimed: Schema.Literal(false),
  providerDeliveryClaimed: Schema.Literal(false),
  physicalDeviceProofClaimed: Schema.Literal(false),
});

export const TrackingEscalationReadinessRowSchema = withParser(
  TrackingEscalationReadinessRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        trackingEscalationReadinessRowIsHonest(row) ||
        'Expected tracking escalation readiness rows to cite policy/evidence refs, keep AI and emergency contact non-authoritative, and expose manual-required provider/device gaps'
    )
  )
);

const TrackingEscalationReadinessReadModelBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
  readinessId: TrackingEscalationReadinessIdSchema,
  generatedAt: NonEmptyStringSchema,
  sourceReadModelGeneratedAt: NonEmptyStringSchema,
  sourceContractRefs: Schema.Array(NonEmptyStringSchema),
  rows: Schema.Array(TrackingEscalationReadinessRowSchema),
  waitingCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  resolvedCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  manualRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  aiEscalationBlockedCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  readinessNonClaims: Schema.Array(TrackingEscalationReadinessNonClaimSchema),
  emergencyServicesAutoContactClaimed: Schema.Literal(false),
  providerDeliveryClaimed: Schema.Literal(false),
  providerReceiptIngestionClaimed: Schema.Literal(false),
  providerCredentialsClaimed: Schema.Literal(false),
  cloudRoutingClaimed: Schema.Literal(false),
  parentNotificationUiClaimed: Schema.Literal(false),
  childDeviceDeliveryClaimed: Schema.Literal(false),
  physicalDeviceProofClaimed: Schema.Literal(false),
  productionEscalationWorkerClaimed: Schema.Literal(false),
  productionQuietHoursTimerClaimed: Schema.Literal(false),
  productClaimReady: Schema.Literal(false),
});

export const TrackingEscalationReadinessReadModelSchema = withParser(
  TrackingEscalationReadinessReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        trackingEscalationReadinessReadModelIsHonest(readModel) ||
        'Expected tracking escalation readiness proof to include all non-claims, at least one row, and matching readiness counts'
    )
  )
);

export type TrackingEscalationReadinessRow = Infer<typeof TrackingEscalationReadinessRowSchema>;
export type TrackingEscalationReadinessReadModel = Infer<typeof TrackingEscalationReadinessReadModelSchema>;

type TrackingEscalationReadinessRowInput = Infer<typeof TrackingEscalationReadinessRowBaseSchema>;
type TrackingEscalationReadinessReadModelInput = Infer<typeof TrackingEscalationReadinessReadModelBaseSchema>;
type TrackingEscalationReadinessState = Infer<typeof TrackingEscalationReadinessStateSchema>;
type TrackingPolicyAuditRef = Infer<typeof TrackingPolicyAuditRefSchema>;

const decodeTrackingPolicyAuditRef = Schema.decodeUnknownSync(TrackingPolicyAuditRefSchema);

const TrackingEscalationWaitingStates: readonly TrackingEscalationReadinessState[] = [
  'waiting-for-parent',
  'waiting-for-child',
] as const;

const TrackingEscalationResolvedStates: readonly TrackingEscalationReadinessState[] = [
  'resolved-by-parent-acknowledgement',
  'resolved-by-child-check-in',
] as const;

const TrackingEscalationManualStates: readonly TrackingEscalationReadinessState[] = [
  'second-guardian-required',
  'critical-multi-channel-manual-required',
  'manual-required',
  'unavailable',
] as const;

const TrackingEscalationCriticalAllowedStates: readonly TrackingEscalationReadinessState[] = [
  'critical-multi-channel-manual-required',
  'resolved-by-child-check-in',
] as const;

const TrackingEscalationGuardianActionRefsByState = {
  'waiting-for-parent': [],
  'waiting-for-child': [],
  'second-guardian-required': ['tracking-second-guardian-review'],
  'critical-multi-channel-manual-required': [
    'tracking-critical-multi-channel-review',
    'tracking-critical-parent-call',
  ],
  'resolved-by-parent-acknowledgement': [],
  'resolved-by-child-check-in': [],
  'manual-required': [],
  unavailable: [],
} as const satisfies Record<TrackingEscalationReadinessState, readonly string[]>;

const TrackingEscalationManualProofRequirementsByState = {
  'waiting-for-parent': ['runtime-worker-proof-required', 'provider-delivery-proof-required'],
  'waiting-for-child': ['runtime-worker-proof-required', 'provider-delivery-proof-required'],
  'second-guardian-required': [
    'parent-guardian-configuration-proof-required',
    'provider-delivery-proof-required',
    'parent-notification-ui-proof-required',
    'physical-device-proof-required',
  ],
  'critical-multi-channel-manual-required': [
    'parent-guardian-configuration-proof-required',
    'provider-delivery-proof-required',
    'parent-notification-ui-proof-required',
    'physical-device-proof-required',
  ],
  'resolved-by-parent-acknowledgement': ['runtime-worker-proof-required', 'provider-delivery-proof-required'],
  'resolved-by-child-check-in': ['runtime-worker-proof-required', 'provider-delivery-proof-required'],
  'manual-required': [
    'parent-guardian-configuration-proof-required',
    'provider-delivery-proof-required',
    'parent-notification-ui-proof-required',
    'physical-device-proof-required',
  ],
  unavailable: [
    'parent-guardian-configuration-proof-required',
    'provider-delivery-proof-required',
    'parent-notification-ui-proof-required',
    'physical-device-proof-required',
  ],
} as const satisfies Record<TrackingEscalationReadinessState, readonly string[]>;

const TrackingEscalationReadinessRowFalseClaims = [
  'aiScheduledEscalation',
  'emergencyServicesAutoContactClaimed',
  'providerDeliveryClaimed',
  'physicalDeviceProofClaimed',
] as const;

const TrackingEscalationReadinessReadModelFalseClaims = [
  'emergencyServicesAutoContactClaimed',
  'providerDeliveryClaimed',
  'childDeviceDeliveryClaimed',
  'physicalDeviceProofClaimed',
  'productClaimReady',
] as const;

export type TrackingEscalationReadinessOptions = {
  readonly generatedAt: string;
  readonly readinessId: string;
  readonly sourceContractRefs: readonly string[];
};

export function buildTrackingEscalationReadinessReadModel(
  options: TrackingEscalationReadinessOptions,
  sourceReadModel: TrackingLocationPolicyReadModel
): TrackingEscalationReadinessReadModel {
  const parsed = TrackingLocationPolicyReadModelSchema.parse(sourceReadModel);
  const rows = parsed.alerts.map((alert) => trackingEscalationReadinessRowForAlert(options, parsed, alert));

  return TrackingEscalationReadinessReadModelSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    readinessId: options.readinessId,
    generatedAt: options.generatedAt,
    sourceReadModelGeneratedAt: parsed.generatedAt,
    sourceContractRefs: options.sourceContractRefs,
    rows,
    waitingCount: countRows(rows, ['waiting-for-parent', 'waiting-for-child']),
    resolvedCount: countRows(rows, ['resolved-by-parent-acknowledgement', 'resolved-by-child-check-in']),
    manualRequiredCount: countRows(rows, [
      'second-guardian-required',
      'critical-multi-channel-manual-required',
      'manual-required',
      'unavailable',
    ]),
    aiEscalationBlockedCount: parsed.decisions.filter((decision) => decision.aiAnalysisId !== null).length,
    readinessNonClaims: RequiredTrackingEscalationReadinessNonClaims,
    emergencyServicesAutoContactClaimed: false,
    providerDeliveryClaimed: false,
    providerReceiptIngestionClaimed: false,
    providerCredentialsClaimed: false,
    cloudRoutingClaimed: false,
    parentNotificationUiClaimed: false,
    childDeviceDeliveryClaimed: false,
    physicalDeviceProofClaimed: false,
    productionEscalationWorkerClaimed: false,
    productionQuietHoursTimerClaimed: false,
    productClaimReady: false,
  });
}

function trackingEscalationReadinessRowForAlert(
  options: TrackingEscalationReadinessOptions,
  readModel: TrackingLocationPolicyReadModel,
  alert: TrackingAlertIntent
): TrackingEscalationReadinessRow {
  const acknowledgement = acknowledgementForAlert(readModel, alert);
  const checkInRequest = checkInRequestForAlert(readModel, alert);
  const checkInResponse = checkInRequest === null ? null : checkInResponseForRequest(readModel, checkInRequest);
  const escalation = escalationForAlert(readModel, alert);
  const decision = decisionForAlert(readModel, alert);
  const readinessState = readinessStateForAlert({
    acknowledgement,
    alert,
    checkInRequest,
    checkInResponse,
    escalation,
    generatedAt: options.generatedAt,
  });

  return TrackingEscalationReadinessRowSchema.parse({
    rowId: `tracking-escalation-readiness-${alert.alertId}`,
    alertId: alert.alertId,
    sourceEscalationId: escalation?.escalationId ?? null,
    sourceDecisionId: alert.policyDecisionId,
    severity: alert.severity,
    readinessState,
    nextActionAt: nextActionAtFor(readinessState, escalation),
    evidenceReferenceIds: alert.evidenceReferences.map((evidence) => evidence.evidenceReferenceId),
    policyDecisionRefs: decision === null ? [alert.policyDecisionId] : [decision.decisionId, decision.ruleId],
    acknowledgementRefs: acknowledgement === null ? [] : [acknowledgement.acknowledgementId],
    childCheckInRefs: childCheckInRefs(checkInRequest, checkInResponse),
    guardianActionRefs: guardianActionRefsFor(readinessState, alert),
    manualProofRequirements: manualProofRequirementsFor(readinessState),
    reasonRefs: reasonRefsFor(readinessState, alert, decision),
    parentAcknowledgementCancelsEscalation: readinessState === 'resolved-by-parent-acknowledgement',
    childCheckInCancelsEscalation: readinessState === 'resolved-by-child-check-in',
    aiScheduledEscalation: false,
    emergencyServicesAutoContactClaimed: false,
    providerDeliveryClaimed: false,
    physicalDeviceProofClaimed: false,
  });
}

function readinessStateForAlert(input: {
  readonly acknowledgement: TrackingAcknowledgement | null;
  readonly alert: TrackingAlertIntent;
  readonly checkInRequest: TrackingChildCheckInRequest | null;
  readonly checkInResponse: TrackingChildCheckInResponse | null;
  readonly escalation: TrackingEscalationChain | null;
  readonly generatedAt: string;
}): TrackingEscalationReadinessState {
  return (
    acknowledgementReadinessStateFor(input) ??
    childCheckInReadinessStateFor(input) ??
    severityReadinessStateFor(input.alert, input.escalation)
  );
}

function acknowledgementForAlert(
  readModel: TrackingLocationPolicyReadModel,
  alert: TrackingAlertIntent
): TrackingAcknowledgement | null {
  return (
    readModel.acknowledgements.find(
      (acknowledgement) =>
        acknowledgement.alertId === alert.alertId || acknowledgement.acknowledgementId === alert.acknowledgementId
    ) ?? null
  );
}

function checkInRequestForAlert(
  readModel: TrackingLocationPolicyReadModel,
  alert: TrackingAlertIntent
): TrackingChildCheckInRequest | null {
  return readModel.checkInRequests.find((request) => request.relatedAlertId === alert.alertId) ?? null;
}

function checkInResponseForRequest(
  readModel: TrackingLocationPolicyReadModel,
  request: TrackingChildCheckInRequest
): TrackingChildCheckInResponse | null {
  return readModel.checkInResponses.find((response) => response.checkInId === request.checkInId) ?? null;
}

function escalationForAlert(
  readModel: TrackingLocationPolicyReadModel,
  alert: TrackingAlertIntent
): TrackingEscalationChain | null {
  return readModel.escalations.find((escalation) => escalation.alertId === alert.alertId) ?? null;
}

function decisionForAlert(
  readModel: TrackingLocationPolicyReadModel,
  alert: TrackingAlertIntent
): TrackingPolicyDecision | null {
  return readModel.decisions.find((decision) => decision.decisionId === alert.policyDecisionId) ?? null;
}

function childCheckInRefs(
  request: TrackingChildCheckInRequest | null,
  response: TrackingChildCheckInResponse | null
): readonly string[] {
  if (request === null) {
    return [];
  }
  return response === null
    ? [request.checkInId]
    : [request.checkInId, `tracking-check-in-response-${response.checkInId}`];
}

function nextActionAtFor(
  state: TrackingEscalationReadinessState,
  escalation: TrackingEscalationChain | null
): string | null {
  return TrackingEscalationWaitingStates.includes(state) ? escalation?.nextActionAt ?? null : null;
}

function guardianActionRefsFor(state: TrackingEscalationReadinessState, alert: TrackingAlertIntent): readonly string[] {
  return TrackingEscalationGuardianActionRefsByState[state].map((actionRef) => `${actionRef}-${alert.alertId}`);
}

function manualProofRequirementsFor(state: TrackingEscalationReadinessState): readonly string[] {
  return TrackingEscalationManualProofRequirementsByState[state];
}

function reasonRefsFor(
  state: TrackingEscalationReadinessState,
  alert: TrackingAlertIntent,
  decision: TrackingPolicyDecision | null
): readonly TrackingPolicyAuditRef[] {
  return [
    ...alert.reasonCodes.map((reasonCode) => decodeTrackingPolicyAuditRef(reasonCode)),
    ...(decision?.reasonCodes ?? []).map((reasonCode) => decodeTrackingPolicyAuditRef(reasonCode)),
    decodeTrackingPolicyAuditRef(`tracking-escalation-readiness-${state}`),
  ];
}

function countRows(
  rows: readonly TrackingEscalationReadinessRow[],
  states: readonly TrackingEscalationReadinessState[]
): number {
  return rows.filter((row) => states.includes(row.readinessState)).length;
}

function trackingEscalationReadinessRowIsHonest(row: TrackingEscalationReadinessRowInput): boolean {
  return (
    row.evidenceReferenceIds.length > 0 &&
    row.policyDecisionRefs.length > 0 &&
    row.reasonRefs.length > 0 &&
    TrackingEscalationReadinessRowFalseClaims.every((claim) => row[claim] === false) &&
    criticalRowsStayManual(row)
  );
}

function criticalRowsStayManual(row: TrackingEscalationReadinessRowInput): boolean {
  return row.severity !== 'critical' || TrackingEscalationCriticalAllowedStates.includes(row.readinessState);
}

function trackingEscalationReadinessReadModelIsHonest(readModel: TrackingEscalationReadinessReadModelInput): boolean {
  return (
    readModel.rows.length > 0 &&
    readModel.readinessNonClaims.length === RequiredTrackingEscalationReadinessNonClaims.length &&
    readModel.waitingCount === countRows(readModel.rows, TrackingEscalationWaitingStates) &&
    readModel.resolvedCount === countRows(readModel.rows, TrackingEscalationResolvedStates) &&
    readModel.manualRequiredCount === countRows(readModel.rows, TrackingEscalationManualStates) &&
    TrackingEscalationReadinessReadModelFalseClaims.every((claim) => readModel[claim] === false)
  );
}

function acknowledgementReadinessStateFor(input: {
  readonly acknowledgement: TrackingAcknowledgement | null;
  readonly alert: TrackingAlertIntent;
  readonly generatedAt: string;
}): TrackingEscalationReadinessState | null {
  if (input.acknowledgement === null) {
    return null;
  }

  return evaluateTrackingAcknowledgementImpact({
    acknowledgement: input.acknowledgement,
    alert: input.alert,
    evaluatedAt: input.generatedAt,
  }).suppressesParentAlert
    ? 'resolved-by-parent-acknowledgement'
    : null;
}

function childCheckInReadinessStateFor(input: {
  readonly checkInRequest: TrackingChildCheckInRequest | null;
  readonly checkInResponse: TrackingChildCheckInResponse | null;
  readonly generatedAt: string;
}): TrackingEscalationReadinessState | null {
  if (input.checkInRequest === null) {
    return null;
  }

  const checkInResolution = resolveTrackingChildCheckIn({
    evaluatedAt: input.generatedAt,
    request: input.checkInRequest,
    response: input.checkInResponse,
  });

  return checkInResolution.escalates
    ? null
    : checkInResolution.state === 'answered'
      ? 'resolved-by-child-check-in'
      : 'waiting-for-child';
}

function severityReadinessStateFor(
  alert: TrackingAlertIntent,
  escalation: TrackingEscalationChain | null
): TrackingEscalationReadinessState {
  if (alert.severity === 'critical') {
    return 'critical-multi-channel-manual-required';
  }
  if (alert.severity === 'urgent') {
    return 'second-guardian-required';
  }
  return escalation?.state === 'manual-required' ? 'manual-required' : 'waiting-for-parent';
}
