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
  if (input.acknowledgement !== null) {
    const acknowledgementImpact = evaluateTrackingAcknowledgementImpact({
      acknowledgement: input.acknowledgement,
      alert: input.alert,
      evaluatedAt: input.generatedAt,
    });
    if (acknowledgementImpact.suppressesParentAlert) {
      return 'resolved-by-parent-acknowledgement';
    }
  }

  if (input.checkInRequest !== null) {
    const checkInResolution = resolveTrackingChildCheckIn({
      evaluatedAt: input.generatedAt,
      request: input.checkInRequest,
      response: input.checkInResponse,
    });
    if (!checkInResolution.escalates && checkInResolution.state === 'answered') {
      return 'resolved-by-child-check-in';
    }
    if (!checkInResolution.escalates) {
      return 'waiting-for-child';
    }
  }

  if (input.alert.severity === 'critical') {
    return 'critical-multi-channel-manual-required';
  }
  if (input.alert.severity === 'urgent') {
    return 'second-guardian-required';
  }
  if (input.escalation?.state === 'manual-required') {
    return 'manual-required';
  }

  return 'waiting-for-parent';
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
  if (state === 'waiting-for-parent' || state === 'waiting-for-child') {
    return escalation?.nextActionAt ?? null;
  }
  return null;
}

function guardianActionRefsFor(state: TrackingEscalationReadinessState, alert: TrackingAlertIntent): readonly string[] {
  if (state === 'second-guardian-required') {
    return [`tracking-second-guardian-review-${alert.alertId}`];
  }
  if (state === 'critical-multi-channel-manual-required') {
    return [
      `tracking-critical-multi-channel-review-${alert.alertId}`,
      `tracking-critical-parent-call-${alert.alertId}`,
    ];
  }
  return [];
}

function manualProofRequirementsFor(state: TrackingEscalationReadinessState): readonly string[] {
  if (
    state === 'second-guardian-required' ||
    state === 'critical-multi-channel-manual-required' ||
    state === 'manual-required' ||
    state === 'unavailable'
  ) {
    return [
      'parent-guardian-configuration-proof-required',
      'provider-delivery-proof-required',
      'parent-notification-ui-proof-required',
      'physical-device-proof-required',
    ];
  }
  return ['runtime-worker-proof-required', 'provider-delivery-proof-required'];
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
    row.aiScheduledEscalation === false &&
    row.emergencyServicesAutoContactClaimed === false &&
    row.providerDeliveryClaimed === false &&
    row.physicalDeviceProofClaimed === false &&
    criticalRowsStayManual(row)
  );
}

function criticalRowsStayManual(row: TrackingEscalationReadinessRowInput): boolean {
  if (row.severity !== 'critical') {
    return true;
  }
  return (
    row.readinessState === 'critical-multi-channel-manual-required' ||
    row.readinessState === 'resolved-by-child-check-in'
  );
}

function trackingEscalationReadinessReadModelIsHonest(readModel: TrackingEscalationReadinessReadModelInput): boolean {
  return (
    readModel.rows.length > 0 &&
    readModel.readinessNonClaims.length === RequiredTrackingEscalationReadinessNonClaims.length &&
    readModel.waitingCount === countRows(readModel.rows, ['waiting-for-parent', 'waiting-for-child']) &&
    readModel.resolvedCount ===
      countRows(readModel.rows, ['resolved-by-parent-acknowledgement', 'resolved-by-child-check-in']) &&
    readModel.manualRequiredCount ===
      countRows(readModel.rows, [
        'second-guardian-required',
        'critical-multi-channel-manual-required',
        'manual-required',
        'unavailable',
      ]) &&
    readModel.emergencyServicesAutoContactClaimed === false &&
    readModel.providerDeliveryClaimed === false &&
    readModel.childDeviceDeliveryClaimed === false &&
    readModel.physicalDeviceProofClaimed === false &&
    readModel.productClaimReady === false
  );
}
