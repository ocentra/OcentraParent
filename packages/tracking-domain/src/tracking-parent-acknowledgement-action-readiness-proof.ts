import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema,
  NonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { TrackingLocationPolicyReadModelSchema, TrackingPolicySchemaVersion } from './tracking-location-policy';
import { TrackingPolicyAuditRefSchema } from './tracking-location-policy-primitives';
import type {
  TrackingAcknowledgement,
  TrackingAlertIntent,
  TrackingEscalationChain,
  TrackingLocationPolicyReadModel,
  TrackingPolicyDecision,
} from './tracking-location-policy-types';

export const RequiredTrackingParentAcknowledgementActionNonClaims = [
  'no-rendered-portal-acknowledgement-ui',
  'no-live-service-mutation-execution',
  'no-provider-delivery-execution',
  'no-notification-receipt-runtime',
  'no-child-device-runtime',
  'no-physical-device-proof',
  'no-authority-proof',
  'no-production-worker',
  'no-adapter-dispatch',
] as const;

export const TrackingParentAcknowledgementActionNonClaimSchema = withParser(
  Schema.Literal(...RequiredTrackingParentAcknowledgementActionNonClaims)
);

export const TrackingParentAcknowledgementActionReadinessIdSchema = brandedNonEmptyStringSchema('TrackingParentAcknowledgementActionReadinessId');
export const TrackingParentAcknowledgementActionRowIdSchema = brandedNonEmptyStringSchema('TrackingParentAcknowledgementActionRowId');
export const TrackingParentAcknowledgementActionStateSchema = withParser(
  Schema.Literal(
    'acknowledgement-action-ready',
    'acknowledgement-recorded',
    'exception-active',
    'false-alarm-recorded',
    'child-check-in-request-ready',
    'escalation-review-ready',
    'manual-required'
  )
);
export const TrackingParentAcknowledgementActionKindSchema = withParser(
  Schema.Literal(
    'acknowledge-safe',
    'mark-expected',
    'holiday-exception',
    'trip-exception',
    'mark-false-alarm',
    'request-child-check-in',
    'escalate-manual-review'
  )
);

const TrackingParentAcknowledgementActionRowBaseSchema = Schema.Struct({
  rowId: TrackingParentAcknowledgementActionRowIdSchema,
  alertId: NonEmptyStringSchema,
  sourceDecisionId: NonEmptyStringSchema,
  sourceAcknowledgementId: Schema.Union(NonEmptyStringSchema, Schema.Null),
  sourceEscalationId: Schema.Union(NonEmptyStringSchema, Schema.Null),
  severity: NonEmptyStringSchema,
  actionState: TrackingParentAcknowledgementActionStateSchema,
  primaryAction: TrackingParentAcknowledgementActionKindSchema,
  allowedActions: Schema.Array(TrackingParentAcknowledgementActionKindSchema),
  exceptionExpiresAt: Schema.Union(NonEmptyStringSchema, Schema.Null),
  stillAlertForCritical: Schema.Boolean,
  evidenceReferenceIds: Schema.Array(NonEmptyStringSchema),
  policyDecisionRefs: Schema.Array(NonEmptyStringSchema),
  auditRefs: Schema.Array(TrackingPolicyAuditRefSchema),
  uiSurfaceRef: NonEmptyStringSchema,
  manualProofRequirements: Schema.Array(NonEmptyStringSchema),
  renderedPortalAcknowledgementUiClaimed: Schema.Literal(false),
  liveServiceMutationClaimed: Schema.Literal(false),
  providerDeliveryClaimed: Schema.Literal(false),
  childDeviceRuntimeClaimed: Schema.Literal(false),
  physicalDeviceProofClaimed: Schema.Literal(false),
});

export const TrackingParentAcknowledgementActionRowSchema = withParser(
  TrackingParentAcknowledgementActionRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        trackingParentAcknowledgementActionRowIsHonest(row) ||
        'Expected parent acknowledgement action rows to cite evidence/policy/audit refs, keep critical alerts visible, and avoid UI/runtime/device claims'
    )
  )
);

const TrackingParentAcknowledgementActionReadModelBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
  readinessId: TrackingParentAcknowledgementActionReadinessIdSchema,
  generatedAt: NonEmptyStringSchema,
  sourceReadModelGeneratedAt: NonEmptyStringSchema,
  sourceContractRefs: Schema.Array(NonEmptyStringSchema),
  rows: Schema.Array(TrackingParentAcknowledgementActionRowSchema),
  actionReadyCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  recordedCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  manualRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  readinessNonClaims: Schema.Array(TrackingParentAcknowledgementActionNonClaimSchema),
  renderedPortalAcknowledgementUiClaimed: Schema.Literal(false),
  liveServiceMutationClaimed: Schema.Literal(false),
  providerDeliveryClaimed: Schema.Literal(false),
  notificationReceiptRuntimeClaimed: Schema.Literal(false),
  childDeviceRuntimeClaimed: Schema.Literal(false),
  physicalDeviceProofClaimed: Schema.Literal(false),
  authorityProofClaimed: Schema.Literal(false),
  productionWorkerClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  productClaimReady: Schema.Literal(false),
});

export const TrackingParentAcknowledgementActionReadModelSchema = withParser(
  TrackingParentAcknowledgementActionReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        trackingParentAcknowledgementActionReadModelIsHonest(readModel) ||
        'Expected parent acknowledgement action proof to include all non-claims, rows, and matching counts'
    )
  )
);

export type TrackingParentAcknowledgementActionRow = Infer<typeof TrackingParentAcknowledgementActionRowSchema>;
export type TrackingParentAcknowledgementActionReadModel = Infer<
  typeof TrackingParentAcknowledgementActionReadModelSchema
>;

type TrackingParentAcknowledgementActionRowInput = Infer<typeof TrackingParentAcknowledgementActionRowBaseSchema>;
type TrackingParentAcknowledgementActionReadModelInput = Infer<
  typeof TrackingParentAcknowledgementActionReadModelBaseSchema
>;
type TrackingParentAcknowledgementActionState = Infer<typeof TrackingParentAcknowledgementActionStateSchema>;
type TrackingParentAcknowledgementActionKind = Infer<typeof TrackingParentAcknowledgementActionKindSchema>;
type TrackingPolicyAuditRef = Infer<typeof TrackingPolicyAuditRefSchema>;

const decodeTrackingPolicyAuditRef = Schema.decodeUnknownSync(TrackingPolicyAuditRefSchema);

export type TrackingParentAcknowledgementActionReadinessOptions = {
  readonly generatedAt: string;
  readonly readinessId: string;
  readonly sourceContractRefs: readonly string[];
};

export function buildTrackingParentAcknowledgementActionReadModel(
  options: TrackingParentAcknowledgementActionReadinessOptions,
  sourceReadModel: TrackingLocationPolicyReadModel
): TrackingParentAcknowledgementActionReadModel {
  const parsed = TrackingLocationPolicyReadModelSchema.parse(sourceReadModel);
  const rows = parsed.alerts.map((alert) => trackingParentAcknowledgementActionRowForAlert(parsed, alert));

  return TrackingParentAcknowledgementActionReadModelSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    readinessId: options.readinessId,
    generatedAt: options.generatedAt,
    sourceReadModelGeneratedAt: parsed.generatedAt,
    sourceContractRefs: options.sourceContractRefs,
    rows,
    actionReadyCount: countRows(rows, [
      'acknowledgement-action-ready',
      'child-check-in-request-ready',
      'escalation-review-ready',
    ]),
    recordedCount: countRows(rows, ['acknowledgement-recorded', 'exception-active', 'false-alarm-recorded']),
    manualRequiredCount: countRows(rows, ['manual-required']),
    readinessNonClaims: RequiredTrackingParentAcknowledgementActionNonClaims,
    renderedPortalAcknowledgementUiClaimed: false,
    liveServiceMutationClaimed: false,
    providerDeliveryClaimed: false,
    notificationReceiptRuntimeClaimed: false,
    childDeviceRuntimeClaimed: false,
    physicalDeviceProofClaimed: false,
    authorityProofClaimed: false,
    productionWorkerClaimed: false,
    adapterDispatchClaimed: false,
    productClaimReady: false,
  });
}

function trackingParentAcknowledgementActionRowForAlert(
  readModel: TrackingLocationPolicyReadModel,
  alert: TrackingAlertIntent
): TrackingParentAcknowledgementActionRow {
  const acknowledgement = acknowledgementForAlert(readModel, alert);
  const decision = decisionForAlert(readModel, alert);
  const escalation = escalationForAlert(readModel, alert);
  const actionState = actionStateFor(alert, acknowledgement, decision, escalation);
  const primaryAction = primaryActionFor(actionState, acknowledgement, decision);

  return TrackingParentAcknowledgementActionRowSchema.parse({
    rowId: `tracking-parent-acknowledgement-action-${alert.alertId}`,
    alertId: alert.alertId,
    sourceDecisionId: alert.policyDecisionId,
    sourceAcknowledgementId: acknowledgement?.acknowledgementId ?? null,
    sourceEscalationId: escalation?.escalationId ?? null,
    severity: alert.severity,
    actionState,
    primaryAction,
    allowedActions: allowedActionsFor(alert, actionState),
    exceptionExpiresAt: acknowledgement?.expiresAt ?? null,
    stillAlertForCritical: acknowledgement?.stillAlertForCritical ?? alert.severity === 'critical',
    evidenceReferenceIds: alert.evidenceReferences.map((evidence) => evidence.evidenceReferenceId),
    policyDecisionRefs: decision === null ? [alert.policyDecisionId] : [decision.decisionId, decision.ruleId],
    auditRefs: auditRefsFor(alert, acknowledgement, decision, escalation, actionState),
    uiSurfaceRef: `tracking-parent-action-surface-${alert.alertId}`,
    manualProofRequirements: manualProofRequirementsFor(actionState),
    renderedPortalAcknowledgementUiClaimed: false,
    liveServiceMutationClaimed: false,
    providerDeliveryClaimed: false,
    childDeviceRuntimeClaimed: false,
    physicalDeviceProofClaimed: false,
  });
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

function decisionForAlert(
  readModel: TrackingLocationPolicyReadModel,
  alert: TrackingAlertIntent
): TrackingPolicyDecision | null {
  return readModel.decisions.find((decision) => decision.decisionId === alert.policyDecisionId) ?? null;
}

function escalationForAlert(
  readModel: TrackingLocationPolicyReadModel,
  alert: TrackingAlertIntent
): TrackingEscalationChain | null {
  return readModel.escalations.find((escalation) => escalation.alertId === alert.alertId) ?? null;
}

function actionStateFor(
  alert: TrackingAlertIntent,
  acknowledgement: TrackingAcknowledgement | null,
  decision: TrackingPolicyDecision | null,
  escalation: TrackingEscalationChain | null
): TrackingParentAcknowledgementActionState {
  const acknowledgementState = actionStateForAcknowledgement(acknowledgement);
  if (acknowledgementState !== null) {
    return acknowledgementState;
  }
  if (decision?.action === 'ask-child-check-in') {
    return 'child-check-in-request-ready';
  }
  if (alertNeedsManualReview(alert, escalation)) {
    return 'escalation-review-ready';
  }
  return 'acknowledgement-action-ready';
}

function actionStateForAcknowledgement(
  acknowledgement: TrackingAcknowledgement | null
): TrackingParentAcknowledgementActionState | null {
  if (acknowledgement === null) {
    return null;
  }
  if (acknowledgement.state === 'acknowledged-safe') {
    return 'acknowledgement-recorded';
  }
  if (acknowledgement.state === 'false-alarm') {
    return 'false-alarm-recorded';
  }
  if (acknowledgementStateIsException(acknowledgement.state)) {
    return 'exception-active';
  }
  return null;
}

function acknowledgementStateIsException(state: string): boolean {
  return state === 'expected' || state === 'holiday-mode' || state === 'trip-exception';
}

function alertNeedsManualReview(alert: TrackingAlertIntent, escalation: TrackingEscalationChain | null): boolean {
  return alert.severity === 'urgent' || alert.severity === 'critical' || escalation?.state === 'manual-required';
}

function primaryActionFor(
  state: TrackingParentAcknowledgementActionState,
  acknowledgement: TrackingAcknowledgement | null,
  decision: TrackingPolicyDecision | null
): TrackingParentAcknowledgementActionKind {
  if (state === 'child-check-in-request-ready' || decision?.action === 'ask-child-check-in') {
    return 'request-child-check-in';
  }
  if (state === 'escalation-review-ready') {
    return 'escalate-manual-review';
  }
  const acknowledgementAction = primaryActionForAcknowledgement(acknowledgement);
  if (acknowledgementAction !== null) {
    return acknowledgementAction;
  }
  return 'acknowledge-safe';
}

function primaryActionForAcknowledgement(
  acknowledgement: TrackingAcknowledgement | null
): TrackingParentAcknowledgementActionKind | null {
  if (acknowledgement === null) {
    return null;
  }
  if (acknowledgement.state === 'expected') {
    return 'mark-expected';
  }
  if (acknowledgement.state === 'holiday-mode') {
    return 'holiday-exception';
  }
  if (acknowledgement.state === 'trip-exception') {
    return 'trip-exception';
  }
  if (acknowledgement.state === 'false-alarm') {
    return 'mark-false-alarm';
  }
  return null;
}

function allowedActionsFor(
  alert: TrackingAlertIntent,
  state: TrackingParentAcknowledgementActionState
): readonly TrackingParentAcknowledgementActionKind[] {
  if (state === 'escalation-review-ready' || alert.severity === 'critical') {
    return ['acknowledge-safe', 'request-child-check-in', 'escalate-manual-review'];
  }
  return [
    'acknowledge-safe',
    'mark-expected',
    'holiday-exception',
    'trip-exception',
    'mark-false-alarm',
    'request-child-check-in',
  ];
}

function auditRefsFor(
  alert: TrackingAlertIntent,
  acknowledgement: TrackingAcknowledgement | null,
  decision: TrackingPolicyDecision | null,
  escalation: TrackingEscalationChain | null,
  state: TrackingParentAcknowledgementActionState
): readonly TrackingPolicyAuditRef[] {
  return [
    ...alert.reasonCodes.map((reasonCode) => decodeTrackingPolicyAuditRef(reasonCode)),
    ...(acknowledgement?.auditRefs ?? []).map((auditRef) => decodeTrackingPolicyAuditRef(auditRef)),
    ...(decision?.auditRefs ?? []).map((auditRef) => decodeTrackingPolicyAuditRef(auditRef)),
    ...(escalation?.auditRefs ?? []).map((auditRef) => decodeTrackingPolicyAuditRef(auditRef)),
    decodeTrackingPolicyAuditRef(`tracking-parent-action-readiness-${state}`),
  ];
}

function manualProofRequirementsFor(state: TrackingParentAcknowledgementActionState): readonly string[] {
  if (state === 'acknowledgement-recorded' || state === 'exception-active' || state === 'false-alarm-recorded') {
    return ['live-service-mutation-proof-required', 'rendered-portal-acknowledgement-ui-proof-required'];
  }
  return [
    'rendered-portal-acknowledgement-ui-proof-required',
    'live-service-mutation-proof-required',
    'provider-delivery-proof-required',
    'physical-device-proof-required',
  ];
}

function countRows(
  rows: readonly TrackingParentAcknowledgementActionRow[],
  states: readonly TrackingParentAcknowledgementActionState[]
): number {
  return rows.filter((row) => states.includes(row.actionState)).length;
}

function trackingParentAcknowledgementActionRowIsHonest(row: TrackingParentAcknowledgementActionRowInput): boolean {
  return (
    row.evidenceReferenceIds.length > 0 &&
    row.policyDecisionRefs.length > 0 &&
    row.auditRefs.length > 0 &&
    row.allowedActions.length > 0 &&
    (!row.severity.includes('critical') || row.stillAlertForCritical) &&
    row.renderedPortalAcknowledgementUiClaimed === false &&
    row.liveServiceMutationClaimed === false &&
    row.providerDeliveryClaimed === false &&
    row.childDeviceRuntimeClaimed === false &&
    row.physicalDeviceProofClaimed === false
  );
}

function trackingParentAcknowledgementActionReadModelIsHonest(
  readModel: TrackingParentAcknowledgementActionReadModelInput
): boolean {
  return (
    readModel.rows.length > 0 &&
    readModel.readinessNonClaims.length === RequiredTrackingParentAcknowledgementActionNonClaims.length &&
    readModel.actionReadyCount ===
      countRows(readModel.rows, [
        'acknowledgement-action-ready',
        'child-check-in-request-ready',
        'escalation-review-ready',
      ]) &&
    readModel.recordedCount ===
      countRows(readModel.rows, ['acknowledgement-recorded', 'exception-active', 'false-alarm-recorded']) &&
    readModel.manualRequiredCount === countRows(readModel.rows, ['manual-required']) &&
    readModel.renderedPortalAcknowledgementUiClaimed === false &&
    readModel.liveServiceMutationClaimed === false &&
    readModel.providerDeliveryClaimed === false &&
    readModel.childDeviceRuntimeClaimed === false &&
    readModel.physicalDeviceProofClaimed === false &&
    readModel.productClaimReady === false
  );
}

