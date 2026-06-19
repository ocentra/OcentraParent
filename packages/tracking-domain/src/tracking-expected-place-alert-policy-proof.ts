import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  TrackingAlertIntentSchema,
  TrackingEvidenceTraceSchema,
  TrackingLocationPolicyReadModelSchema,
  TrackingPolicyDecisionSchema,
  TrackingPolicyRuleSchema,
  type TrackingAlertIntent,
  type TrackingLocationPolicyReadModel,
  type TrackingPolicyDecision,
  type TrackingPolicyRule,
} from './tracking-location-policy';
import {
  TrackingPolicyAuditRefSchema,
  TrackingPolicyReasonCodeSchema,
  TrackingPolicySchemaVersion,
} from './tracking-location-policy-primitives';

export const TrackingExpectedPlaceAlertPolicyRowStatus = {
  AlertPolicyReady: 'alert-policy-ready',
  CheckInPolicyReady: 'check-in-policy-ready',
  SuppressedNoAction: 'suppressed-no-action',
  ManualRequired: 'manual-required',
} as const;

export const RequiredTrackingExpectedPlaceAlertPolicyNonClaims = [
  'no-rendered-parent-ui',
  'no-alert-delivery-runtime',
  'no-provider-delivery',
  'no-notification-receipt-runtime',
  'no-child-device-runtime',
  'no-physical-device-proof',
  'no-authority-proof',
  'no-production-worker',
  'no-adapter-dispatch',
] as const;

export const TrackingExpectedPlaceAlertPolicyProofRefSchema = brandedNonEmptyStringSchema('TrackingExpectedPlaceAlertPolicyProofRef');
export const TrackingExpectedPlaceAlertPolicyRowIdSchema = brandedNonEmptyStringSchema('TrackingExpectedPlaceAlertPolicyRowId');
export const TrackingExpectedPlaceAlertPolicyRowStatusSchema = withParser(
  Schema.Literal(...Object.values(TrackingExpectedPlaceAlertPolicyRowStatus))
);
export const TrackingExpectedPlaceAlertPolicyNonClaimSchema = withParser(
  Schema.Literal(...RequiredTrackingExpectedPlaceAlertPolicyNonClaims)
);

const TrackingExpectedPlaceAlertPolicyRowBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
  rowId: TrackingExpectedPlaceAlertPolicyRowIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceRule: TrackingPolicyRuleSchema,
  sourceDecision: TrackingPolicyDecisionSchema,
  sourceAlert: Schema.Union(TrackingAlertIntentSchema, Schema.Null),
  status: TrackingExpectedPlaceAlertPolicyRowStatusSchema,
  uiSurfaceRef: TrackingExpectedPlaceAlertPolicyProofRefSchema,
  evidenceReferences: Schema.Array(TrackingEvidenceTraceSchema),
  reasonCodes: Schema.Array(TrackingPolicyReasonCodeSchema),
  auditRefs: Schema.Array(TrackingPolicyAuditRefSchema),
  manualProofRequirements: Schema.Array(TrackingExpectedPlaceAlertPolicyProofRefSchema),
  renderedParentUiClaimed: Schema.Literal(false),
  alertDeliveryRuntimeClaimed: Schema.Literal(false),
  providerDeliveryClaimed: Schema.Literal(false),
  notificationReceiptRuntimeClaimed: Schema.Literal(false),
  childDeviceRuntimeClaimed: Schema.Literal(false),
  physicalDeviceProofClaimed: Schema.Literal(false),
  authorityProofClaimed: Schema.Literal(false),
  productionWorkerClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
});

export const TrackingExpectedPlaceAlertPolicyRowSchema = withParser(
  TrackingExpectedPlaceAlertPolicyRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        expectedPlaceAlertPolicyRowIsHonest(row) ||
        'Expected-place alert policy rows need expected-place rule refs, evidence refs, reason refs, and false runtime/UI claims'
    )
  )
);

export const TrackingExpectedPlaceAlertPolicyProofSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    proofMode: Schema.Literal('tracking-expected-place-alert-policy-proof'),
    generatedAt: ParentTimestampSchema,
    sourceReadModelRef: TrackingExpectedPlaceAlertPolicyProofRefSchema,
    sourceProofRefs: Schema.Array(TrackingExpectedPlaceAlertPolicyProofRefSchema),
    rows: Schema.Array(TrackingExpectedPlaceAlertPolicyRowSchema),
    alertPolicyReadyCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
    checkInPolicyReadyCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
    suppressedNoActionCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
    manualRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
    proofNonClaims: Schema.Array(TrackingExpectedPlaceAlertPolicyNonClaimSchema),
    renderedParentUiClaimed: Schema.Literal(false),
    alertDeliveryRuntimeClaimed: Schema.Literal(false),
    providerDeliveryClaimed: Schema.Literal(false),
    notificationReceiptRuntimeClaimed: Schema.Literal(false),
    childDeviceRuntimeClaimed: Schema.Literal(false),
    physicalDeviceProofClaimed: Schema.Literal(false),
    authorityProofClaimed: Schema.Literal(false),
    productionWorkerClaimed: Schema.Literal(false),
    adapterDispatchClaimed: Schema.Literal(false),
  }).pipe(
    Schema.filter(
      (proof) => proof.rows.length > 0 || 'Expected-place alert policy proof needs at least one expected-place row'
    )
  )
);

export type TrackingExpectedPlaceAlertPolicyRow = Infer<typeof TrackingExpectedPlaceAlertPolicyRowSchema>;
export type TrackingExpectedPlaceAlertPolicyProof = Infer<typeof TrackingExpectedPlaceAlertPolicyProofSchema>;
type TrackingExpectedPlaceAlertPolicyRowInput = Infer<typeof TrackingExpectedPlaceAlertPolicyRowBaseSchema>;

export function buildTrackingExpectedPlaceAlertPolicyProof(input: {
  readonly generatedAt: string;
  readonly sourceReadModelRef: string;
  readonly sourceProofRefs: readonly string[];
  readonly readModel: TrackingLocationPolicyReadModel;
}): TrackingExpectedPlaceAlertPolicyProof {
  const readModel = TrackingLocationPolicyReadModelSchema.parse(input.readModel);
  const rulesById = new Map(readModel.rules.map((rule) => [rule.ruleId, rule]));
  const alertsById = new Map(readModel.alerts.map((alert) => [alert.alertId, alert]));
  const expectedDecisions = readModel.decisions.filter(
    (decision) => rulesById.get(decision.ruleId)?.targetKind === 'expected-place'
  );
  const rows = expectedDecisions.map((decision) =>
    expectedPlaceAlertPolicyRow(
      input.generatedAt,
      requiredRule(rulesById, decision),
      decision,
      alertFor(alertsById, decision)
    )
  );

  return TrackingExpectedPlaceAlertPolicyProofSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    proofMode: 'tracking-expected-place-alert-policy-proof',
    generatedAt: input.generatedAt,
    sourceReadModelRef: input.sourceReadModelRef,
    sourceProofRefs: input.sourceProofRefs,
    rows,
    alertPolicyReadyCount: countRows(rows, TrackingExpectedPlaceAlertPolicyRowStatus.AlertPolicyReady),
    checkInPolicyReadyCount: countRows(rows, TrackingExpectedPlaceAlertPolicyRowStatus.CheckInPolicyReady),
    suppressedNoActionCount: countRows(rows, TrackingExpectedPlaceAlertPolicyRowStatus.SuppressedNoAction),
    manualRequiredCount: countRows(rows, TrackingExpectedPlaceAlertPolicyRowStatus.ManualRequired),
    proofNonClaims: RequiredTrackingExpectedPlaceAlertPolicyNonClaims,
    renderedParentUiClaimed: false,
    alertDeliveryRuntimeClaimed: false,
    providerDeliveryClaimed: false,
    notificationReceiptRuntimeClaimed: false,
    childDeviceRuntimeClaimed: false,
    physicalDeviceProofClaimed: false,
    authorityProofClaimed: false,
    productionWorkerClaimed: false,
    adapterDispatchClaimed: false,
  });
}

function expectedPlaceAlertPolicyRow(
  generatedAt: string,
  rule: TrackingPolicyRule,
  decision: TrackingPolicyDecision,
  alert: TrackingAlertIntent | null
): TrackingExpectedPlaceAlertPolicyRow {
  const status = statusFor(decision);
  return TrackingExpectedPlaceAlertPolicyRowSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    rowId: `tracking-expected-place-alert-policy-${decision.decisionId}`,
    generatedAt,
    sourceRule: rule,
    sourceDecision: decision,
    sourceAlert: alert,
    status,
    uiSurfaceRef: `tracking-expected-place-ui-readiness-${decision.decisionId}`,
    evidenceReferences: decision.evidenceReferences,
    reasonCodes: [...rule.reasonCodes, ...decision.reasonCodes, ...(alert?.reasonCodes ?? [])],
    auditRefs: [...rule.auditRefs, ...decision.auditRefs],
    manualProofRequirements: manualProofRequirementsFor(status, decision),
    renderedParentUiClaimed: false,
    alertDeliveryRuntimeClaimed: false,
    providerDeliveryClaimed: false,
    notificationReceiptRuntimeClaimed: false,
    childDeviceRuntimeClaimed: false,
    physicalDeviceProofClaimed: false,
    authorityProofClaimed: false,
    productionWorkerClaimed: false,
    adapterDispatchClaimed: false,
  });
}

function statusFor(decision: TrackingPolicyDecision): TrackingExpectedPlaceAlertPolicyRow['status'] {
  if (decision.action === 'notify-parent') {
    return TrackingExpectedPlaceAlertPolicyRowStatus.AlertPolicyReady;
  }
  if (decision.action === 'ask-child-check-in' || decision.action === 'request-parent-acknowledgement') {
    return TrackingExpectedPlaceAlertPolicyRowStatus.CheckInPolicyReady;
  }
  return decision.action === 'no-action'
    ? TrackingExpectedPlaceAlertPolicyRowStatus.SuppressedNoAction
    : TrackingExpectedPlaceAlertPolicyRowStatus.ManualRequired;
}

function manualProofRequirementsFor(
  status: TrackingExpectedPlaceAlertPolicyRow['status'],
  decision: TrackingPolicyDecision
): readonly string[] {
  return status === TrackingExpectedPlaceAlertPolicyRowStatus.ManualRequired
    ? [`tracking-expected-place-manual-proof-${decision.decisionId}`]
    : [];
}

function expectedPlaceAlertPolicyRowIsHonest(row: TrackingExpectedPlaceAlertPolicyRowInput): boolean {
  return (
    row.sourceRule.targetKind === 'expected-place' &&
    row.evidenceReferences.length > 0 &&
    row.reasonCodes.length > 0 &&
    row.auditRefs.length > 0 &&
    rowClaimsStayFalse(row)
  );
}

function rowClaimsStayFalse(row: TrackingExpectedPlaceAlertPolicyRowInput): boolean {
  return [
    row.renderedParentUiClaimed,
    row.alertDeliveryRuntimeClaimed,
    row.providerDeliveryClaimed,
    row.notificationReceiptRuntimeClaimed,
    row.childDeviceRuntimeClaimed,
    row.physicalDeviceProofClaimed,
    row.authorityProofClaimed,
    row.productionWorkerClaimed,
    row.adapterDispatchClaimed,
  ].every((claim) => claim === false);
}

function requiredRule(
  rulesById: ReadonlyMap<TrackingPolicyRule['ruleId'], TrackingPolicyRule>,
  decision: TrackingPolicyDecision
): TrackingPolicyRule {
  const rule = rulesById.get(decision.ruleId);
  if (rule === undefined) {
    throw new Error(`Missing expected-place policy rule for ${decision.decisionId}`);
  }
  return rule;
}

function alertFor(
  alertsById: ReadonlyMap<TrackingAlertIntent['alertId'], TrackingAlertIntent>,
  decision: TrackingPolicyDecision
): TrackingAlertIntent | null {
  return decision.alertIntentId === null ? null : (alertsById.get(decision.alertIntentId) ?? null);
}

function countRows(
  rows: readonly TrackingExpectedPlaceAlertPolicyRow[],
  status: TrackingExpectedPlaceAlertPolicyRow['status']
) {
  return rows.filter((row) => row.status === status).length;
}

