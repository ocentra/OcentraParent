import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentContractSchemaVersion, ParentContractSchemaVersionSchema } from './reference-primitives';
import {
  TrackingAlertIntentSchema,
  TrackingEvidenceTraceSchema,
  TrackingPolicyDecisionSchema,
  TrackingPolicyRuleSchema,
} from './tracking-location-policy';
import {
  TrackingPolicyAuditRefSchema,
  TrackingPolicyReasonCodeSchema,
  TrackingPolicySchemaVersion,
} from './tracking-location-policy-primitives';

const TrackingExpectedPlaceAlertText = Schema.String.pipe(Schema.minLength(1));

export const TrackingExpectedPlaceAlertProofIdSchema = TrackingExpectedPlaceAlertText.pipe(
  Schema.brand('TrackingExpectedPlaceAlertProofId')
);
export const TrackingExpectedPlaceAlertProofReferenceSchema = withParser(
  TrackingExpectedPlaceAlertText.pipe(Schema.brand('TrackingExpectedPlaceAlertProofReference'))
);
const TrackingExpectedPlaceAlertProofIdParserSchema = withParser(TrackingExpectedPlaceAlertProofIdSchema);
const TrackingPolicyAuditRefParserSchema = withParser(TrackingPolicyAuditRefSchema);
export const TrackingExpectedPlaceAlertStateSchema = withParser(
  Schema.Literal('no-alert-expected', 'alert-ready', 'manual-required', 'suppressed-disabled-rule')
);

export const RequiredTrackingExpectedPlaceAlertProofNonClaims = [
  'no-provider-delivery',
  'no-notification-receipt-ingestion',
  'no-parent-notification-ui',
  'no-child-device-delivery',
  'no-platform-adapter-runtime',
  'no-physical-device-proof',
  'no-production-worker',
] as const;

export const TrackingExpectedPlaceAlertProofNonClaimSchema = withParser(
  Schema.Literal(...RequiredTrackingExpectedPlaceAlertProofNonClaims)
);

const TrackingExpectedPlaceAlertRowBaseSchema = Schema.Struct({
  rowId: TrackingExpectedPlaceAlertProofReferenceSchema,
  sourceExpectedPlaceDecisionId: TrackingExpectedPlaceAlertProofReferenceSchema,
  scheduleRefs: Schema.Array(TrackingExpectedPlaceAlertProofReferenceSchema),
  placeRefs: Schema.Array(TrackingExpectedPlaceAlertProofReferenceSchema),
  evidenceRefs: Schema.Array(TrackingEvidenceTraceSchema),
  rule: TrackingPolicyRuleSchema,
  decision: TrackingPolicyDecisionSchema,
  alertIntent: Schema.Union(TrackingAlertIntentSchema, Schema.Null),
  alertPolicyState: TrackingExpectedPlaceAlertStateSchema,
  reasonCodeRefs: Schema.Array(TrackingPolicyReasonCodeSchema),
  auditRefs: Schema.Array(TrackingPolicyAuditRefSchema),
  providerDeliveryClaimed: Schema.Literal(false),
  notificationReceiptClaimed: Schema.Literal(false),
  parentNotificationUiClaimed: Schema.Literal(false),
  childDeviceDeliveryClaimed: Schema.Literal(false),
  platformAdapterRuntimeClaimed: Schema.Literal(false),
  physicalDeviceProofClaimed: Schema.Literal(false),
  productionWorkerClaimed: Schema.Literal(false),
});

export const TrackingExpectedPlaceAlertRowSchema = withParser(
  TrackingExpectedPlaceAlertRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        trackingExpectedPlaceAlertRowIsHonest(row) ||
        'Expected-place alert policy rows need schedule/place/evidence refs and must not claim provider delivery, portal notification UI, platform adapter runtime, or physical-device proof'
    )
  )
);

const TrackingExpectedPlaceAlertReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  proofId: TrackingExpectedPlaceAlertProofIdSchema,
  generatedAt: TrackingExpectedPlaceAlertProofReferenceSchema,
  sourceExpectedPlaceProofRef: TrackingExpectedPlaceAlertProofReferenceSchema,
  sourcePolicyCompilerProofRef: TrackingExpectedPlaceAlertProofReferenceSchema,
  sourceNotificationBoundaryRef: TrackingExpectedPlaceAlertProofReferenceSchema,
  rows: Schema.Array(TrackingExpectedPlaceAlertRowSchema),
  alertReadyCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  manualRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  suppressedCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  noAlertExpectedCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  proofNonClaims: Schema.Array(TrackingExpectedPlaceAlertProofNonClaimSchema),
  providerDeliveryClaimed: Schema.Literal(false),
  notificationReceiptClaimed: Schema.Literal(false),
  parentNotificationUiClaimed: Schema.Literal(false),
  childDeviceDeliveryClaimed: Schema.Literal(false),
  platformAdapterRuntimeClaimed: Schema.Literal(false),
  physicalDeviceProofClaimed: Schema.Literal(false),
  productionWorkerClaimed: Schema.Literal(false),
});

export const TrackingExpectedPlaceAlertReadModelSchema = withParser(
  TrackingExpectedPlaceAlertReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        trackingExpectedPlaceAlertReadModelIsHonest(readModel) ||
        'Expected-place alert policy proof counts must match rows and keep all runtime/product delivery claims false'
    )
  )
);

export type TrackingExpectedPlaceAlertState = Infer<typeof TrackingExpectedPlaceAlertStateSchema>;
export type TrackingExpectedPlaceAlertRow = Infer<typeof TrackingExpectedPlaceAlertRowSchema>;
export type TrackingExpectedPlaceAlertReadModel = Infer<typeof TrackingExpectedPlaceAlertReadModelSchema>;

export type TrackingExpectedPlaceAlertProofOptions = {
  readonly generatedAt: string;
  readonly proofId: string;
  readonly sourceExpectedPlaceProofRef: string;
  readonly sourcePolicyCompilerProofRef: string;
  readonly sourceNotificationBoundaryRef: string;
};

type RowInput = Infer<typeof TrackingExpectedPlaceAlertRowBaseSchema>;
type ReadModelInput = Infer<typeof TrackingExpectedPlaceAlertReadModelBaseSchema>;

export function buildTrackingExpectedPlaceAlertPolicyProof(
  options: TrackingExpectedPlaceAlertProofOptions
): TrackingExpectedPlaceAlertReadModel {
  const rows = [
    row(options, {
      sourceExpectedPlaceDecisionId: 'expected-place-school-arrived',
      expectedPlaceState: 'no-alert-expected',
      ruleAction: 'observe',
      ruleEnabled: true,
      freshEvidence: true,
      decisionAction: 'observe',
      alertSeverity: null,
      reasonCodes: ['expected-place-arrival-confirmed'],
    }),
    row(options, {
      sourceExpectedPlaceDecisionId: 'expected-place-practice-missed',
      expectedPlaceState: 'alert-ready',
      ruleAction: 'notify-parent',
      ruleEnabled: true,
      freshEvidence: true,
      decisionAction: 'notify-parent',
      alertSeverity: 'urgent',
      reasonCodes: ['expected-place-missed-arrival', 'tracking-rule-matched'],
    }),
    row(options, {
      sourceExpectedPlaceDecisionId: 'expected-place-library-stale',
      expectedPlaceState: 'manual-required',
      ruleAction: 'notify-parent',
      ruleEnabled: true,
      freshEvidence: false,
      decisionAction: 'manual-required',
      alertSeverity: null,
      reasonCodes: ['expected-place-evidence-stale', 'manual-proof-required'],
    }),
    row(options, {
      sourceExpectedPlaceDecisionId: 'expected-place-home-disabled-rule',
      expectedPlaceState: 'suppressed-disabled-rule',
      ruleAction: 'notify-parent',
      ruleEnabled: false,
      freshEvidence: true,
      decisionAction: 'no-action',
      alertSeverity: null,
      reasonCodes: ['expected-place-rule-disabled'],
    }),
  ];

  return TrackingExpectedPlaceAlertReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    proofId: TrackingExpectedPlaceAlertProofIdParserSchema.parse(options.proofId),
    generatedAt: TrackingExpectedPlaceAlertProofReferenceSchema.parse(options.generatedAt),
    sourceExpectedPlaceProofRef: TrackingExpectedPlaceAlertProofReferenceSchema.parse(
      options.sourceExpectedPlaceProofRef
    ),
    sourcePolicyCompilerProofRef: TrackingExpectedPlaceAlertProofReferenceSchema.parse(
      options.sourcePolicyCompilerProofRef
    ),
    sourceNotificationBoundaryRef: TrackingExpectedPlaceAlertProofReferenceSchema.parse(
      options.sourceNotificationBoundaryRef
    ),
    rows,
    alertReadyCount: countState(rows, 'alert-ready'),
    manualRequiredCount: countState(rows, 'manual-required'),
    suppressedCount: countState(rows, 'suppressed-disabled-rule'),
    noAlertExpectedCount: countState(rows, 'no-alert-expected'),
    proofNonClaims: RequiredTrackingExpectedPlaceAlertProofNonClaims,
    providerDeliveryClaimed: false,
    notificationReceiptClaimed: false,
    parentNotificationUiClaimed: false,
    childDeviceDeliveryClaimed: false,
    platformAdapterRuntimeClaimed: false,
    physicalDeviceProofClaimed: false,
    productionWorkerClaimed: false,
  });
}

function row(
  options: TrackingExpectedPlaceAlertProofOptions,
  input: {
    readonly sourceExpectedPlaceDecisionId: string;
    readonly expectedPlaceState: TrackingExpectedPlaceAlertState;
    readonly ruleAction: RowInput['rule']['action'];
    readonly ruleEnabled: boolean;
    readonly freshEvidence: boolean;
    readonly decisionAction: RowInput['decision']['action'];
    readonly alertSeverity: RowInput['alertIntent'] extends infer Alert
      ? Alert extends null
        ? never
        : Alert extends { readonly severity: infer Severity }
          ? Severity | null
          : never
      : never;
    readonly reasonCodes: readonly string[];
  }
): TrackingExpectedPlaceAlertRow {
  const rule = trackingExpectedPlaceRuleFor(input);
  const evidenceRefs = [evidence(input.sourceExpectedPlaceDecisionId, input.freshEvidence)];
  const alertId =
    input.alertSeverity === null ? null : `tracking-expected-alert-${input.sourceExpectedPlaceDecisionId}`;
  const decision = trackingExpectedPlaceDecisionFor(options, input, rule, evidenceRefs, alertId);
  const alertIntent = trackingExpectedPlaceAlertFor(options, input, decision, evidenceRefs, alertId);

  return TrackingExpectedPlaceAlertRowSchema.parse({
    rowId: `tracking-expected-alert-row-${input.sourceExpectedPlaceDecisionId}`,
    sourceExpectedPlaceDecisionId: TrackingExpectedPlaceAlertProofReferenceSchema.parse(
      input.sourceExpectedPlaceDecisionId
    ),
    scheduleRefs: [
      TrackingExpectedPlaceAlertProofReferenceSchema.parse(
        `tracking-expected-schedule-${input.sourceExpectedPlaceDecisionId}`
      ),
    ],
    placeRefs: [
      TrackingExpectedPlaceAlertProofReferenceSchema.parse(
        `tracking-expected-place-${input.sourceExpectedPlaceDecisionId}`
      ),
    ],
    evidenceRefs,
    rule,
    decision,
    alertIntent,
    alertPolicyState: input.expectedPlaceState,
    reasonCodeRefs: input.reasonCodes.map((reasonCode) => TrackingPolicyReasonCodeSchema.parse(reasonCode)),
    auditRefs: [
      TrackingPolicyAuditRefParserSchema.parse(
        `tracking-expected-alert-proof-audit-${input.sourceExpectedPlaceDecisionId}`
      ),
    ],
    providerDeliveryClaimed: false,
    notificationReceiptClaimed: false,
    parentNotificationUiClaimed: false,
    childDeviceDeliveryClaimed: false,
    platformAdapterRuntimeClaimed: false,
    physicalDeviceProofClaimed: false,
    productionWorkerClaimed: false,
  });
}

function trackingExpectedPlaceRuleFor(input: Parameters<typeof row>[1]): RowInput['rule'] {
  return TrackingPolicyRuleSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    ruleId: `tracking-expected-alert-rule-${input.sourceExpectedPlaceDecisionId}`,
    familyId: 'family-1',
    childProfileId: 'child-1',
    deviceId: 'parent-device-1',
    policyVersion: 'tracking-policy-expected-place-v1',
    targetKind: 'expected-place',
    action: input.ruleAction,
    enabled: input.ruleEnabled,
    requiresFreshEvidence: true,
    requiresParentConfirmation: false,
    reasonCodes: input.reasonCodes,
    auditRefs: [`tracking-expected-alert-rule-audit-${input.sourceExpectedPlaceDecisionId}`],
  });
}

function trackingExpectedPlaceDecisionFor(
  options: TrackingExpectedPlaceAlertProofOptions,
  input: Parameters<typeof row>[1],
  rule: RowInput['rule'],
  evidenceRefs: RowInput['evidenceRefs'],
  alertId: string | null
): RowInput['decision'] {
  return TrackingPolicyDecisionSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    decisionId: `tracking-expected-alert-decision-${input.sourceExpectedPlaceDecisionId}`,
    decidedAt: options.generatedAt,
    ruleId: rule.ruleId,
    action: input.decisionAction,
    dryRun: true,
    evidenceReferences: evidenceRefs,
    aiAnalysisId: null,
    alertIntentId: alertId,
    reasonCodes: input.reasonCodes,
    auditRefs: [`tracking-expected-alert-decision-audit-${input.sourceExpectedPlaceDecisionId}`],
  });
}

function trackingExpectedPlaceAlertFor(
  options: TrackingExpectedPlaceAlertProofOptions,
  input: Parameters<typeof row>[1],
  decision: RowInput['decision'],
  evidenceRefs: RowInput['evidenceRefs'],
  alertId: string | null
): RowInput['alertIntent'] {
  if (input.alertSeverity === null || alertId === null) return null;
  return TrackingAlertIntentSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    alertId,
    createdAt: options.generatedAt,
    severity: input.alertSeverity,
    policyDecisionId: decision.decisionId,
    evidenceReferences: evidenceRefs,
    sensitiveDetailMode: 'minimal-provider-body',
    notificationStatusRefs: [`tracking-expected-alert-provider-status-${input.sourceExpectedPlaceDecisionId}`],
    acknowledgementId: null,
    reasonCodes: input.reasonCodes,
  });
}

function evidence(sourceExpectedPlaceDecisionId: string, freshEvidence: boolean): RowInput['evidenceRefs'][number] {
  return TrackingEvidenceTraceSchema.parse({
    evidenceReferenceId: `tracking-expected-evidence-${sourceExpectedPlaceDecisionId}`,
    kind: 'journal-event',
    observedAt: freshEvidence ? '2026-06-05T21:50:00.000Z' : '2026-06-05T18:20:00.000Z',
  });
}

function trackingExpectedPlaceAlertRowIsHonest(row: RowInput): boolean {
  if (row.scheduleRefs.length === 0 || row.placeRefs.length === 0 || row.evidenceRefs.length === 0) return false;
  if (trackingExpectedPlaceAlertRowClaimsRuntime(row)) return false;
  if (row.alertPolicyState === 'alert-ready') {
    return row.alertIntent !== null && row.decision.alertIntentId === row.alertIntent.alertId;
  }
  if (row.alertIntent !== null || row.decision.alertIntentId !== null) return false;
  if (row.alertPolicyState === 'manual-required') return row.decision.action === 'manual-required';
  if (row.alertPolicyState === 'suppressed-disabled-rule')
    return !row.rule.enabled && row.decision.action === 'no-action';
  return row.decision.action === 'observe';
}

function trackingExpectedPlaceAlertRowClaimsRuntime(row: RowInput): boolean {
  return (
    row.providerDeliveryClaimed ||
    row.notificationReceiptClaimed ||
    row.parentNotificationUiClaimed ||
    row.childDeviceDeliveryClaimed ||
    row.platformAdapterRuntimeClaimed ||
    row.physicalDeviceProofClaimed ||
    row.productionWorkerClaimed
  );
}

function trackingExpectedPlaceAlertReadModelIsHonest(readModel: ReadModelInput): boolean {
  return (
    readModel.alertReadyCount === countState(readModel.rows, 'alert-ready') &&
    readModel.manualRequiredCount === countState(readModel.rows, 'manual-required') &&
    readModel.suppressedCount === countState(readModel.rows, 'suppressed-disabled-rule') &&
    readModel.noAlertExpectedCount === countState(readModel.rows, 'no-alert-expected') &&
    readModel.proofNonClaims.length === RequiredTrackingExpectedPlaceAlertProofNonClaims.length &&
    !readModel.providerDeliveryClaimed &&
    !readModel.notificationReceiptClaimed &&
    !readModel.parentNotificationUiClaimed &&
    !readModel.childDeviceDeliveryClaimed &&
    !readModel.platformAdapterRuntimeClaimed &&
    !readModel.physicalDeviceProofClaimed &&
    !readModel.productionWorkerClaimed
  );
}

function countState(rows: readonly { readonly alertPolicyState: TrackingExpectedPlaceAlertState }[], state: string) {
  return rows.filter((row) => row.alertPolicyState === state).length;
}
