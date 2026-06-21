import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema,
  NonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import {
  TrackingLocationPolicyReadModelSchema,
  TrackingPolicySchemaVersion,
} from '@ocentra-parent/schema-domain/tracking-location-policy';
import { TrackingPolicyAuditRefSchema } from '@ocentra-parent/schema-domain/tracking-location-policy-primitives';
import type {
  TrackingAlertIntent,
  TrackingChildCheckInRequest,
  TrackingChildCheckInResponse,
  TrackingLocationPolicyReadModel,
} from '@ocentra-parent/schema-domain/tracking-location-policy-types';
import { resolveTrackingChildCheckIn } from './tracking-location-policy-runtime';

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

export const TrackingChildCheckInTimeoutReadinessIdSchema = brandedNonEmptyStringSchema('TrackingChildCheckInTimeoutReadinessId');
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
type TrackingPolicyAuditRef = Infer<typeof TrackingPolicyAuditRefSchema>;

const decodeTrackingPolicyAuditRef = Schema.decodeUnknownSync(TrackingPolicyAuditRefSchema);

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
  if (request.state === 'cancelled') {
    return 'cancelled';
  }
  if (response?.response === 'safe' || response?.response === 'share-location-if-permitted') {
    return 'safe-response-recorded';
  }
  if (response?.response === 'help') {
    return 'help-response-escalation-ready';
  }
  if (response?.response === 'call-parent') {
    return 'call-parent-response-escalation-ready';
  }
  if (runtimeState === 'escalated' || escalates) {
    return 'expired-timeout-escalation-ready';
  }
  return 'waiting-for-child';
}

function escalationStateEscalates(state: TrackingChildCheckInTimeoutState): boolean {
  return (
    state === 'help-response-escalation-ready' ||
    state === 'call-parent-response-escalation-ready' ||
    state === 'expired-timeout-escalation-ready' ||
    state === 'manual-required'
  );
}

function locationSampleStateFor(
  request: TrackingChildCheckInRequest,
  response: TrackingChildCheckInResponse | null
): 'requested-not-yet-attached' | 'attached-from-child-response' | 'not-attached' {
  if (!request.includeLocationIfPermitted) {
    return 'not-attached';
  }
  if (response?.locationEvidenceReference !== null && response !== null) {
    return 'attached-from-child-response';
  }
  return 'requested-not-yet-attached';
}

function alertOutcomeFor(
  state: TrackingChildCheckInTimeoutState
): 'awaiting-child-response' | 'alert-resolved-safe' | 'parent-review-required' {
  if (state === 'safe-response-recorded' || state === 'cancelled') {
    return 'alert-resolved-safe';
  }
  if (escalationStateEscalates(state)) {
    return 'parent-review-required';
  }
  return 'awaiting-child-response';
}

function escalationBasisFor(
  state: TrackingChildCheckInTimeoutState
): 'none' | 'child-help-response' | 'child-call-parent-response' | 'expired-rule-only-timeout' {
  if (state === 'help-response-escalation-ready') {
    return 'child-help-response';
  }
  if (state === 'call-parent-response-escalation-ready') {
    return 'child-call-parent-response';
  }
  if (state === 'expired-timeout-escalation-ready') {
    return 'expired-rule-only-timeout';
  }
  return 'none';
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
  if (escalationStateEscalates(state)) {
    return [
      'child-device-delivery-proof-required',
      'provider-delivery-proof-required',
      'timeout-worker-proof-required',
      'physical-device-proof-required',
    ];
  }
  return ['child-device-runtime-proof-required', 'rendered-child-device-ui-proof-required'];
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
  return (
    row.childDeviceDeliveryRuntimeClaimed === false &&
    row.childDeviceResponseRuntimeClaimed === false &&
    row.renderedChildDeviceUiClaimed === false &&
    row.providerDeliveryClaimed === false &&
    row.liveLocationSampleRuntimeClaimed === false &&
    row.physicalDeviceProofClaimed === false
  );
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
  return (
    readModel.childDeviceDeliveryRuntimeClaimed === false &&
    readModel.renderedChildDeviceUiClaimed === false &&
    readModel.providerDeliveryClaimed === false &&
    readModel.physicalDeviceProofClaimed === false &&
    readModel.productClaimReady === false
  );
}

function locationSampleStateFromRow(
  row: TrackingChildCheckInTimeoutRowInput
): 'requested-not-yet-attached' | 'attached-from-child-response' | 'not-attached' {
  if (!row.includeLocationIfPermitted) {
    return 'not-attached';
  }
  if (row.locationEvidenceReferenceId !== null) {
    return 'attached-from-child-response';
  }
  return 'requested-not-yet-attached';
}

