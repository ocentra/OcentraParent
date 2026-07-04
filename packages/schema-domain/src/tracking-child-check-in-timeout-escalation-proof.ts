import { type Infer, Schema, withParser, brandedNonEmptyStringSchema, NonEmptyStringSchema } from './effect';
import { TrackingLocationPolicyReadModelSchema, TrackingPolicySchemaVersion } from './tracking-location-policy';
import { resolveTrackingChildCheckIn } from './tracking-location-policy-runtime';
import { TrackingPolicyAuditRefSchema } from './tracking-location-policy-primitives';
import type {
  TrackingAlertIntent,
  TrackingChildCheckInRequest,
  TrackingChildCheckInResponse,
  TrackingLocationPolicyReadModel,
} from './tracking-location-policy-types';

export const RequiredTrackingChildCheckInTimeoutNonClaims = [
  'no-child-device-delivery-runtime',
  'no-child-device-response-runtime',
  'no-rendered-child-device-ui',
  'no-provider-delivery-execution',
  'no-notification-receipt-runtime',
  'no-live-location-sample-runtime',
  'no-physical-device-proof',
  'no-authority-proof',
  'no-production-timeout-worker',
  'no-adapter-dispatch',
] as const;

export const TrackingChildCheckInTimeoutNonClaimSchema = withParser(
  Schema.Literal(...RequiredTrackingChildCheckInTimeoutNonClaims)
);

export const TrackingChildCheckInTimeoutReadinessIdSchema = brandedNonEmptyStringSchema(
  'TrackingChildCheckInTimeoutReadinessId'
);
export const TrackingChildCheckInTimeoutRowIdSchema = brandedNonEmptyStringSchema('TrackingChildCheckInTimeoutRowId');
export const TrackingChildCheckInTimeoutStateSchema = withParser(
  Schema.Literal(
    'waiting-for-child',
    'safe-response-recorded',
    'help-response-escalation-ready',
    'call-parent-response-escalation-ready',
    'expired-timeout-escalation-ready',
    'cancelled',
    'manual-required'
  )
);
export const TrackingChildCheckInTimeoutAlertOutcomeSchema = withParser(
  Schema.Literal('awaiting-child-response', 'alert-resolved-safe', 'parent-review-required')
);
export const TrackingChildCheckInTimeoutLocationSampleStateSchema = withParser(
  Schema.Literal('requested-not-yet-attached', 'attached-from-child-response', 'not-attached')
);
export const TrackingChildCheckInTimeoutAuditCoverageStateSchema = withParser(
  Schema.Literal('prompt-audited', 'prompt-and-response-audited')
);
export const TrackingChildCheckInTimeoutEscalationBasisSchema = withParser(
  Schema.Literal('none', 'child-help-response', 'child-call-parent-response', 'expired-rule-only-timeout')
);

const TrackingChildCheckInTimeoutRowBaseSchema = Schema.Struct({
  rowId: TrackingChildCheckInTimeoutRowIdSchema,
  checkInId: NonEmptyStringSchema,
  relatedAlertId: Schema.Union(NonEmptyStringSchema, Schema.Null),
  sourceResponseKind: Schema.Union(NonEmptyStringSchema, Schema.Null),
  timeoutAt: NonEmptyStringSchema,
  evaluatedAt: NonEmptyStringSchema,
  resolutionState: TrackingChildCheckInTimeoutStateSchema,
  escalates: Schema.Boolean,
  includeLocationIfPermitted: Schema.Boolean,
  locationEvidenceReferenceId: Schema.Union(NonEmptyStringSchema, Schema.Null),
  locationSampleState: TrackingChildCheckInTimeoutLocationSampleStateSchema,
  auditCoverageState: TrackingChildCheckInTimeoutAuditCoverageStateSchema,
  alertOutcome: TrackingChildCheckInTimeoutAlertOutcomeSchema,
  escalationBasis: TrackingChildCheckInTimeoutEscalationBasisSchema,
  evidenceReferenceIds: Schema.Array(NonEmptyStringSchema),
  policyDecisionRefs: Schema.Array(NonEmptyStringSchema),
  auditRefs: Schema.Array(TrackingPolicyAuditRefSchema),
  parentActionRefs: Schema.Array(NonEmptyStringSchema),
  manualProofRequirements: Schema.Array(NonEmptyStringSchema),
  childDeviceDeliveryRuntimeClaimed: Schema.Literal(false),
  childDeviceResponseRuntimeClaimed: Schema.Literal(false),
  renderedChildDeviceUiClaimed: Schema.Literal(false),
  providerDeliveryClaimed: Schema.Literal(false),
  liveLocationSampleRuntimeClaimed: Schema.Literal(false),
  physicalDeviceProofClaimed: Schema.Literal(false),
});

export const TrackingChildCheckInTimeoutRowSchema = withParser(
  TrackingChildCheckInTimeoutRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        trackingChildCheckInTimeoutRowIsHonest(row) ||
        'Expected child check-in timeout rows to cite evidence/audit refs, escalate only from response/timeout states, and avoid child-runtime/device claims'
    )
  )
);

const TrackingChildCheckInTimeoutReadModelBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
  readinessId: TrackingChildCheckInTimeoutReadinessIdSchema,
  generatedAt: NonEmptyStringSchema,
  sourceReadModelGeneratedAt: NonEmptyStringSchema,
  sourceContractRefs: Schema.Array(NonEmptyStringSchema),
  rows: Schema.Array(TrackingChildCheckInTimeoutRowSchema),
  waitingCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  resolvedCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  escalationReadyCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  locationSampleRequestedCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  attachedLocationSampleCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  auditedPromptCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  auditedResponseCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  ruleOnlyEscalationCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  safeAlertOutcomeCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  readinessNonClaims: Schema.Array(TrackingChildCheckInTimeoutNonClaimSchema),
  childDeviceDeliveryRuntimeClaimed: Schema.Literal(false),
  childDeviceResponseRuntimeClaimed: Schema.Literal(false),
  renderedChildDeviceUiClaimed: Schema.Literal(false),
  providerDeliveryClaimed: Schema.Literal(false),
  notificationReceiptRuntimeClaimed: Schema.Literal(false),
  liveLocationSampleRuntimeClaimed: Schema.Literal(false),
  physicalDeviceProofClaimed: Schema.Literal(false),
  authorityProofClaimed: Schema.Literal(false),
  productionTimeoutWorkerClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  productClaimReady: Schema.Literal(false),
});

export const TrackingChildCheckInTimeoutReadModelSchema = withParser(
  TrackingChildCheckInTimeoutReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        trackingChildCheckInTimeoutReadModelIsHonest(readModel) ||
        'Expected child check-in timeout proof to include all non-claims, rows, and matching counts'
    )
  )
);

export type TrackingChildCheckInTimeoutRow = Infer<typeof TrackingChildCheckInTimeoutRowSchema>;
export type TrackingChildCheckInTimeoutReadModel = Infer<typeof TrackingChildCheckInTimeoutReadModelSchema>;

type TrackingChildCheckInTimeoutRowInput = Infer<typeof TrackingChildCheckInTimeoutRowBaseSchema>;
type TrackingChildCheckInTimeoutReadModelInput = Infer<typeof TrackingChildCheckInTimeoutReadModelBaseSchema>;
type TrackingChildCheckInTimeoutState = Infer<typeof TrackingChildCheckInTimeoutStateSchema>;
type TrackingChildCheckInTimeoutResponseKind = TrackingChildCheckInResponse['response'];
type TrackingPolicyAuditRef = Infer<typeof TrackingPolicyAuditRefSchema>;

const decodeTrackingPolicyAuditRef = Schema.decodeUnknownSync(TrackingPolicyAuditRefSchema);

const TrackingChildCheckInTimeoutEscalationReadyStates = [
  'help-response-escalation-ready',
  'call-parent-response-escalation-ready',
  'expired-timeout-escalation-ready',
  'manual-required',
] as const satisfies readonly TrackingChildCheckInTimeoutState[];

const TrackingChildCheckInTimeoutResponseStateMap = {
  safe: 'safe-response-recorded',
  'share-location-if-permitted': 'safe-response-recorded',
  help: 'help-response-escalation-ready',
  'call-parent': 'call-parent-response-escalation-ready',
} as const satisfies Partial<Record<TrackingChildCheckInTimeoutResponseKind, TrackingChildCheckInTimeoutState>>;

const TrackingChildCheckInTimeoutAlertOutcomeByState = {
  'waiting-for-child': 'awaiting-child-response',
  'safe-response-recorded': 'alert-resolved-safe',
  'help-response-escalation-ready': 'parent-review-required',
  'call-parent-response-escalation-ready': 'parent-review-required',
  'expired-timeout-escalation-ready': 'parent-review-required',
  cancelled: 'alert-resolved-safe',
  'manual-required': 'parent-review-required',
} as const satisfies Record<
  TrackingChildCheckInTimeoutState,
  'awaiting-child-response' | 'alert-resolved-safe' | 'parent-review-required'
>;

const TrackingChildCheckInTimeoutEscalationBasisByState = {
  'waiting-for-child': 'none',
  'safe-response-recorded': 'none',
  'help-response-escalation-ready': 'child-help-response',
  'call-parent-response-escalation-ready': 'child-call-parent-response',
  'expired-timeout-escalation-ready': 'expired-rule-only-timeout',
  cancelled: 'none',
  'manual-required': 'none',
} as const satisfies Record<
  TrackingChildCheckInTimeoutState,
  'none' | 'child-help-response' | 'child-call-parent-response' | 'expired-rule-only-timeout'
>;

const TrackingChildCheckInTimeoutManualProofRequirementsByState = {
  'waiting-for-child': [
    'child-device-runtime-proof-required',
    'rendered-child-device-ui-proof-required',
  ],
  'safe-response-recorded': [
    'child-device-runtime-proof-required',
    'rendered-child-device-ui-proof-required',
  ],
  'help-response-escalation-ready': [
    'child-device-delivery-proof-required',
    'provider-delivery-proof-required',
    'timeout-worker-proof-required',
    'physical-device-proof-required',
  ],
  'call-parent-response-escalation-ready': [
    'child-device-delivery-proof-required',
    'provider-delivery-proof-required',
    'timeout-worker-proof-required',
    'physical-device-proof-required',
  ],
  'expired-timeout-escalation-ready': [
    'child-device-delivery-proof-required',
    'provider-delivery-proof-required',
    'timeout-worker-proof-required',
    'physical-device-proof-required',
  ],
  cancelled: ['child-device-runtime-proof-required', 'rendered-child-device-ui-proof-required'],
  'manual-required': [
    'child-device-delivery-proof-required',
    'provider-delivery-proof-required',
    'timeout-worker-proof-required',
    'physical-device-proof-required',
  ],
} as const satisfies Record<TrackingChildCheckInTimeoutState, readonly string[]>;

const TrackingChildCheckInTimeoutRowFalseClaims = [
  'childDeviceDeliveryRuntimeClaimed',
  'childDeviceResponseRuntimeClaimed',
  'renderedChildDeviceUiClaimed',
  'providerDeliveryClaimed',
  'liveLocationSampleRuntimeClaimed',
  'physicalDeviceProofClaimed',
] as const;

const TrackingChildCheckInTimeoutReadModelFalseClaims = [
  'childDeviceDeliveryRuntimeClaimed',
  'renderedChildDeviceUiClaimed',
  'providerDeliveryClaimed',
  'physicalDeviceProofClaimed',
  'productClaimReady',
] as const;

export type TrackingChildCheckInTimeoutReadinessOptions = {
  readonly generatedAt: string;
  readonly readinessId: string;
  readonly sourceContractRefs: readonly string[];
};

export function buildTrackingChildCheckInTimeoutReadModel(
  options: TrackingChildCheckInTimeoutReadinessOptions,
  sourceReadModel: TrackingLocationPolicyReadModel
): TrackingChildCheckInTimeoutReadModel {
  const parsed = TrackingLocationPolicyReadModelSchema.parse(sourceReadModel);
  const rows = parsed.checkInRequests.map((request) => childCheckInTimeoutRowForRequest(options, parsed, request));

  return TrackingChildCheckInTimeoutReadModelSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    readinessId: options.readinessId,
    generatedAt: options.generatedAt,
    sourceReadModelGeneratedAt: parsed.generatedAt,
    sourceContractRefs: options.sourceContractRefs,
    rows,
    waitingCount: countRows(rows, ['waiting-for-child']),
    resolvedCount: countRows(rows, ['safe-response-recorded', 'cancelled']),
    escalationReadyCount: countRows(rows, [
      'help-response-escalation-ready',
      'call-parent-response-escalation-ready',
      'expired-timeout-escalation-ready',
      'manual-required',
    ]),
    locationSampleRequestedCount: rows.filter((row) => row.includeLocationIfPermitted).length,
    attachedLocationSampleCount: rows.filter((row) => row.locationSampleState === 'attached-from-child-response')
      .length,
    auditedPromptCount: rows.filter((row) => row.auditCoverageState === 'prompt-audited').length,
    auditedResponseCount: rows.filter((row) => row.auditCoverageState === 'prompt-and-response-audited').length,
    ruleOnlyEscalationCount: rows.filter((row) => row.escalationBasis === 'expired-rule-only-timeout').length,
    safeAlertOutcomeCount: rows.filter((row) => row.alertOutcome === 'alert-resolved-safe').length,
    readinessNonClaims: RequiredTrackingChildCheckInTimeoutNonClaims,
    childDeviceDeliveryRuntimeClaimed: false,
    childDeviceResponseRuntimeClaimed: false,
    renderedChildDeviceUiClaimed: false,
    providerDeliveryClaimed: false,
    notificationReceiptRuntimeClaimed: false,
    liveLocationSampleRuntimeClaimed: false,
    physicalDeviceProofClaimed: false,
    authorityProofClaimed: false,
    productionTimeoutWorkerClaimed: false,
    adapterDispatchClaimed: false,
    productClaimReady: false,
  });
}

function childCheckInTimeoutRowForRequest(
  options: TrackingChildCheckInTimeoutReadinessOptions,
  readModel: TrackingLocationPolicyReadModel,
  request: TrackingChildCheckInRequest
): TrackingChildCheckInTimeoutRow {
  const response = responseForRequest(readModel, request);
  const alert = alertForRequest(readModel, request);
  const resolution = resolveTrackingChildCheckIn({
    evaluatedAt: options.generatedAt,
    request,
    response,
  });
  const resolutionState = resolutionStateFor(request, response, resolution.escalates, resolution.state);

  return TrackingChildCheckInTimeoutRowSchema.parse({
    rowId: `tracking-child-check-in-timeout-${request.checkInId}`,
    checkInId: request.checkInId,
    relatedAlertId: request.relatedAlertId,
    sourceResponseKind: response?.response ?? null,
    timeoutAt: request.expiresAt,
    evaluatedAt: options.generatedAt,
    resolutionState,
    escalates: escalationStateEscalates(resolutionState),
    includeLocationIfPermitted: request.includeLocationIfPermitted,
    locationEvidenceReferenceId: response?.locationEvidenceReference?.evidenceReferenceId ?? null,
    locationSampleState: locationSampleStateFor(request, response),
    auditCoverageState: response === null ? 'prompt-audited' : 'prompt-and-response-audited',
    alertOutcome: alertOutcomeFor(resolutionState),
    escalationBasis: escalationBasisFor(resolutionState),
    evidenceReferenceIds: evidenceReferenceIdsFor(request, response, alert),
    policyDecisionRefs: alert === null ? [] : [alert.policyDecisionId],
    auditRefs: auditRefsFor(request, response, resolution.reasonCodes, resolutionState),
    parentActionRefs: parentActionRefsFor(request, resolutionState),
    manualProofRequirements: manualProofRequirementsFor(resolutionState),
    childDeviceDeliveryRuntimeClaimed: false,
    childDeviceResponseRuntimeClaimed: false,
    renderedChildDeviceUiClaimed: false,
    providerDeliveryClaimed: false,
    liveLocationSampleRuntimeClaimed: false,
    physicalDeviceProofClaimed: false,
  });
}

function responseForRequest(
  readModel: TrackingLocationPolicyReadModel,
  request: TrackingChildCheckInRequest
): TrackingChildCheckInResponse | null {
  return readModel.checkInResponses.find((response) => response.checkInId === request.checkInId) ?? null;
}

function alertForRequest(
  readModel: TrackingLocationPolicyReadModel,
  request: TrackingChildCheckInRequest
): TrackingAlertIntent | null {
  if (request.relatedAlertId === null) {
    return null;
  }
  return readModel.alerts.find((alert) => alert.alertId === request.relatedAlertId) ?? null;
}

function resolutionStateFor(
  request: TrackingChildCheckInRequest,
  response: TrackingChildCheckInResponse | null,
  escalates: boolean,
  runtimeState: string
): TrackingChildCheckInTimeoutState {
  const responseState = response === null ? undefined : TrackingChildCheckInTimeoutResponseStateMap[response.response];
  if (request.state === 'cancelled') {
    return 'cancelled';
  }
  if (responseState !== undefined) {
    return responseState;
  }
  if (runtimeState === 'escalated' || escalates) {
    return 'expired-timeout-escalation-ready';
  }
  return 'waiting-for-child';
}

function escalationStateEscalates(state: TrackingChildCheckInTimeoutState): boolean {
  return TrackingChildCheckInTimeoutEscalationReadyStates.includes(state);
}

function locationSampleStateFor(
  request: TrackingChildCheckInRequest,
  response: TrackingChildCheckInResponse | null
): 'requested-not-yet-attached' | 'attached-from-child-response' | 'not-attached' {
  return locationSampleStateFrom(request.includeLocationIfPermitted, response?.locationEvidenceReference !== null);
}

function alertOutcomeFor(
  state: TrackingChildCheckInTimeoutState
): 'awaiting-child-response' | 'alert-resolved-safe' | 'parent-review-required' {
  return TrackingChildCheckInTimeoutAlertOutcomeByState[state];
}

function escalationBasisFor(
  state: TrackingChildCheckInTimeoutState
): 'none' | 'child-help-response' | 'child-call-parent-response' | 'expired-rule-only-timeout' {
  return TrackingChildCheckInTimeoutEscalationBasisByState[state];
}

function evidenceReferenceIdsFor(
  request: TrackingChildCheckInRequest,
  response: TrackingChildCheckInResponse | null,
  alert: TrackingAlertIntent | null
): readonly string[] {
  return [
    ...request.evidenceReferences.map((evidence) => evidence.evidenceReferenceId),
    ...(response?.locationEvidenceReference === null || response === null
      ? []
      : [response.locationEvidenceReference.evidenceReferenceId]),
    ...(alert?.evidenceReferences ?? []).map((evidence) => evidence.evidenceReferenceId),
  ];
}

function auditRefsFor(
  request: TrackingChildCheckInRequest,
  response: TrackingChildCheckInResponse | null,
  runtimeReasonCodes: readonly string[],
  state: TrackingChildCheckInTimeoutState
): readonly TrackingPolicyAuditRef[] {
  return [
    ...request.auditRefs.map((auditRef) => decodeTrackingPolicyAuditRef(auditRef)),
    ...(response?.auditRefs ?? []).map((auditRef) => decodeTrackingPolicyAuditRef(auditRef)),
    ...runtimeReasonCodes.map((reasonCode) => decodeTrackingPolicyAuditRef(reasonCode)),
    decodeTrackingPolicyAuditRef(`tracking-child-check-in-timeout-${state}`),
  ];
}

function parentActionRefsFor(
  request: TrackingChildCheckInRequest,
  state: TrackingChildCheckInTimeoutState
): readonly string[] {
  if (escalationStateEscalates(state)) {
    return [`tracking-parent-review-child-check-in-${request.checkInId}`];
  }
  return [];
}

function manualProofRequirementsFor(state: TrackingChildCheckInTimeoutState): readonly string[] {
  return TrackingChildCheckInTimeoutManualProofRequirementsByState[state];
}

function countRows(
  rows: readonly TrackingChildCheckInTimeoutRow[],
  states: readonly TrackingChildCheckInTimeoutState[]
): number {
  return rows.filter((row) => states.includes(row.resolutionState)).length;
}

function trackingChildCheckInTimeoutRowIsHonest(row: TrackingChildCheckInTimeoutRowInput): boolean {
  return (
    trackingChildCheckInTimeoutRowRefsAreHonest(row) &&
    trackingChildCheckInTimeoutRowDerivedFieldsAreHonest(row) &&
    trackingChildCheckInTimeoutRowNonClaimsAreHonest(row)
  );
}

function trackingChildCheckInTimeoutRowRefsAreHonest(row: TrackingChildCheckInTimeoutRowInput): boolean {
  return row.evidenceReferenceIds.length > 0 && row.auditRefs.length > 0 && row.manualProofRequirements.length > 0;
}

function trackingChildCheckInTimeoutRowDerivedFieldsAreHonest(row: TrackingChildCheckInTimeoutRowInput): boolean {
  return (
    row.escalates === escalationStateEscalates(row.resolutionState) &&
    row.locationSampleState === locationSampleStateFromRow(row) &&
    row.auditCoverageState === (row.sourceResponseKind === null ? 'prompt-audited' : 'prompt-and-response-audited') &&
    row.alertOutcome === alertOutcomeFor(row.resolutionState) &&
    row.escalationBasis === escalationBasisFor(row.resolutionState)
  );
}

function trackingChildCheckInTimeoutRowNonClaimsAreHonest(row: TrackingChildCheckInTimeoutRowInput): boolean {
  return TrackingChildCheckInTimeoutRowFalseClaims.every((claim) => row[claim] === false);
}

function trackingChildCheckInTimeoutReadModelIsHonest(readModel: TrackingChildCheckInTimeoutReadModelInput): boolean {
  return (
    trackingChildCheckInTimeoutReadModelRowsAreHonest(readModel) &&
    trackingChildCheckInTimeoutReadModelCountsAreHonest(readModel) &&
    trackingChildCheckInTimeoutReadModelNonClaimsAreHonest(readModel)
  );
}

function trackingChildCheckInTimeoutReadModelRowsAreHonest(
  readModel: TrackingChildCheckInTimeoutReadModelInput
): boolean {
  return (
    readModel.rows.length > 0 &&
    readModel.readinessNonClaims.length === RequiredTrackingChildCheckInTimeoutNonClaims.length
  );
}

function trackingChildCheckInTimeoutReadModelCountsAreHonest(
  readModel: TrackingChildCheckInTimeoutReadModelInput
): boolean {
  return (
    readModel.waitingCount === countRows(readModel.rows, ['waiting-for-child']) &&
    readModel.resolvedCount === countRows(readModel.rows, ['safe-response-recorded', 'cancelled']) &&
    readModel.escalationReadyCount ===
      countRows(readModel.rows, [
        'help-response-escalation-ready',
        'call-parent-response-escalation-ready',
        'expired-timeout-escalation-ready',
        'manual-required',
      ]) &&
    readModel.locationSampleRequestedCount === readModel.rows.filter((row) => row.includeLocationIfPermitted).length &&
    readModel.attachedLocationSampleCount ===
      readModel.rows.filter((row) => row.locationSampleState === 'attached-from-child-response').length &&
    readModel.auditedPromptCount ===
      readModel.rows.filter((row) => row.auditCoverageState === 'prompt-audited').length &&
    readModel.auditedResponseCount ===
      readModel.rows.filter((row) => row.auditCoverageState === 'prompt-and-response-audited').length &&
    readModel.ruleOnlyEscalationCount ===
      readModel.rows.filter((row) => row.escalationBasis === 'expired-rule-only-timeout').length &&
    readModel.safeAlertOutcomeCount ===
      readModel.rows.filter((row) => row.alertOutcome === 'alert-resolved-safe').length
  );
}

function trackingChildCheckInTimeoutReadModelNonClaimsAreHonest(
  readModel: TrackingChildCheckInTimeoutReadModelInput
): boolean {
  return TrackingChildCheckInTimeoutReadModelFalseClaims.every((claim) => readModel[claim] === false);
}

function locationSampleStateFrom(
  includeLocationIfPermitted: boolean,
  hasLocationEvidence: boolean
): 'requested-not-yet-attached' | 'attached-from-child-response' | 'not-attached' {
  if (!includeLocationIfPermitted) {
    return 'not-attached';
  }
  if (hasLocationEvidence) {
    return 'attached-from-child-response';
  }
  return 'requested-not-yet-attached';
}

function locationSampleStateFromRow(
  row: TrackingChildCheckInTimeoutRowInput
): 'requested-not-yet-attached' | 'attached-from-child-response' | 'not-attached' {
  return locationSampleStateFrom(row.includeLocationIfPermitted, row.locationEvidenceReferenceId !== null);
}
