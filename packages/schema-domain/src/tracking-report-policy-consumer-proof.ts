import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from './effect';
import { ParentTimestampSchema } from './family-reference-primitives';
import { TrackingEvidenceTraceSchema, TrackingPolicyDecisionSchema } from './tracking-location-policy';
import {
  TrackingPolicyAuditRefSchema,
  TrackingPolicyReasonCodeSchema,
  TrackingPolicySchemaVersion,
} from './tracking-location-policy-primitives';

export const TrackingReportPolicyConsumerRowIdSchema = brandedNonEmptyStringSchema('TrackingReportPolicyConsumerRowId');

export const TrackingReportPolicyConsumerProofRefSchema = brandedNonEmptyStringSchema('TrackingReportPolicyConsumerProofRef');

export const TrackingReportPolicyConsumerJournalRefSchema = brandedNonEmptyStringSchema('TrackingReportPolicyConsumerJournalRef');

export const TrackingReportPolicyConsumerReadModelRowRefSchema = brandedNonEmptyStringSchema('TrackingReportPolicyConsumerReadModelRowRef');

export const TrackingReportPolicyConsumerKindSchema = withParser(
  Schema.Literal('parent-report-summary', 'policy-evidence-drill-in', 'retention-audit-export')
);

export const TrackingReportPolicyConsumerStateSchema = withParser(Schema.Literal('consumer-ready', 'manual-required'));

export const TrackingReportPolicyConsumerProofTierSchema = withParser(
  Schema.Literal('P1_FIXTURE_SIMULATION', 'P2_HOSTED_CI', 'P3_LOCAL_DEV_MACHINE')
);

export const TrackingReportPolicyConsumerRowSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    rowId: TrackingReportPolicyConsumerRowIdSchema,
    consumerKind: TrackingReportPolicyConsumerKindSchema,
    readinessState: TrackingReportPolicyConsumerStateSchema,
    requiredProofTier: TrackingReportPolicyConsumerProofTierSchema,
    currentProofTier: TrackingReportPolicyConsumerProofTierSchema,
    generatedAt: ParentTimestampSchema,
    sourceProofRefs: Schema.Array(TrackingReportPolicyConsumerProofRefSchema),
    productSurfaceSummaryRefs: Schema.Array(TrackingReportPolicyConsumerProofRefSchema),
    reportSurfaceRefs: Schema.Array(TrackingReportPolicyConsumerProofRefSchema),
    storedJournalRefs: Schema.Array(TrackingReportPolicyConsumerJournalRefSchema),
    storedReadModelRowRefs: Schema.Array(TrackingReportPolicyConsumerReadModelRowRefSchema),
    evidenceReferences: Schema.Array(TrackingEvidenceTraceSchema),
    policyDecision: Schema.Union(TrackingPolicyDecisionSchema, Schema.Null),
    reasonCodes: Schema.Array(TrackingPolicyReasonCodeSchema),
    auditRefs: Schema.Array(TrackingPolicyAuditRefSchema),
    reportConsumerClaimed: Schema.Literal(true),
    portalUiClaimed: Schema.Literal(false),
    childDeviceDeliveryClaimed: Schema.Literal(false),
    providerDeliveryClaimed: Schema.Literal(false),
    notificationReceiptClaimed: Schema.Literal(false),
    physicalDeviceClaimed: Schema.Literal(false),
    authorityClaimed: Schema.Literal(false),
    productClaimReady: Schema.Literal(false),
  })
    .pipe(
      Schema.filter(
        (row) => row.sourceProofRefs.length > 0 || 'Tracking report/policy consumers need source proof refs'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          row.productSurfaceSummaryRefs.length > 0 ||
          'Tracking report/policy consumers need product-surface summary refs'
      )
    )
    .pipe(
      Schema.filter((row) => row.evidenceReferences.length > 0 || 'Tracking report/policy consumers need evidence refs')
    )
    .pipe(
      Schema.filter(
        (row) => row.storedJournalRefs.length > 0 || 'Tracking report/policy consumers need stored journal refs'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          row.storedReadModelRowRefs.length > 0 || 'Tracking report/policy consumers need stored read-model row refs'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          row.consumerKind !== 'policy-evidence-drill-in' ||
          row.policyDecision !== null ||
          'Tracking policy drill-in consumers need a policy decision'
      )
    )
);

export const TrackingReportPolicyConsumerProofSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    proofMode: Schema.Literal('tracking-report-policy-consumer-proof'),
    generatedAt: ParentTimestampSchema,
    rows: Schema.Array(TrackingReportPolicyConsumerRowSchema),
    productClaims: Schema.Struct({
      productClaimReady: Schema.Literal(false),
      portalUiClaimed: Schema.Literal(false),
      childDeviceDeliveryClaimed: Schema.Literal(false),
      providerDeliveryClaimed: Schema.Literal(false),
      notificationReceiptClaimed: Schema.Literal(false),
      physicalDeviceClaimed: Schema.Literal(false),
      authorityClaimed: Schema.Literal(false),
    }),
  }).pipe(
    Schema.filter((proof) => proof.rows.length >= 3 || 'Tracking report/policy consumer proof needs all consumer rows')
  )
);

export type TrackingReportPolicyConsumerKind = Infer<typeof TrackingReportPolicyConsumerKindSchema>;
export type TrackingReportPolicyConsumerRow = Infer<typeof TrackingReportPolicyConsumerRowSchema>;
export type TrackingReportPolicyConsumerProof = Infer<typeof TrackingReportPolicyConsumerProofSchema>;
type TrackingReportPolicyEvidence = Infer<typeof TrackingEvidenceTraceSchema>;
type TrackingReportPolicyDecision = Infer<typeof TrackingPolicyDecisionSchema>;

export function buildTrackingReportPolicyConsumerProof(generatedAt: string): TrackingReportPolicyConsumerProof {
  const timestamp = generatedAt;
  const reportEvidence = evidence('tracking-report-policy-evidence-summary', 'query-store-summary', timestamp);
  const policyEvidence = evidence('tracking-report-policy-evidence-decision', 'policy-decision', timestamp);
  const retentionEvidence = evidence('tracking-report-policy-evidence-retention', 'query-store-summary', timestamp);
  const policyDecision = TrackingPolicyDecisionSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    decisionId: 'tracking-report-policy-decision-drill-in',
    decidedAt: timestamp,
    ruleId: 'tracking-report-policy-rule-drill-in',
    action: 'notify-parent',
    dryRun: true,
    evidenceReferences: [policyEvidence],
    aiAnalysisId: null,
    alertIntentId: 'tracking-report-policy-alert-drill-in',
    reasonCodes: ['tracking-report-policy-consumer-ready'],
    auditRefs: ['tracking-report-policy-audit-drill-in'],
  });

  return TrackingReportPolicyConsumerProofSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    proofMode: 'tracking-report-policy-consumer-proof',
    generatedAt: timestamp,
    rows: [
      row({
        rowId: 'tracking-report-policy-row-report-summary',
        consumerKind: 'parent-report-summary',
        generatedAt: timestamp,
        evidenceReferences: [reportEvidence],
        storedJournalRefs: ['tracking-journal-row-report-summary'],
        storedReadModelRowRefs: ['tracking-read-model-row-report-summary'],
        policyDecision: null,
        reasonCodes: ['tracking-product-surface-summary-consumed'],
        auditRefs: ['tracking-report-policy-audit-report-summary'],
        reportSurfaceRefs: ['parent-report-location-summary-row'],
      }),
      row({
        rowId: 'tracking-report-policy-row-policy-drill-in',
        consumerKind: 'policy-evidence-drill-in',
        generatedAt: timestamp,
        evidenceReferences: [policyEvidence],
        storedJournalRefs: ['tracking-journal-row-policy-drill-in'],
        storedReadModelRowRefs: ['tracking-read-model-row-policy-drill-in'],
        policyDecision,
        reasonCodes: ['tracking-policy-decision-drill-in-consumed'],
        auditRefs: ['tracking-report-policy-audit-policy-drill-in'],
        reportSurfaceRefs: ['parent-policy-evidence-drill-in-row'],
      }),
      row({
        rowId: 'tracking-report-policy-row-retention-export',
        consumerKind: 'retention-audit-export',
        generatedAt: timestamp,
        evidenceReferences: [retentionEvidence],
        storedJournalRefs: ['tracking-journal-row-retention-export'],
        storedReadModelRowRefs: ['tracking-read-model-row-retention-export'],
        policyDecision: null,
        reasonCodes: ['tracking-retention-tombstone-summary-consumed'],
        auditRefs: ['tracking-report-policy-audit-retention-export'],
        reportSurfaceRefs: ['parent-retention-audit-export-row'],
      }),
    ],
    productClaims: {
      productClaimReady: false,
      portalUiClaimed: false,
      childDeviceDeliveryClaimed: false,
      providerDeliveryClaimed: false,
      notificationReceiptClaimed: false,
      physicalDeviceClaimed: false,
      authorityClaimed: false,
    },
  });
}

function row(input: {
  readonly rowId: string;
  readonly consumerKind: TrackingReportPolicyConsumerKind;
  readonly generatedAt: string;
  readonly evidenceReferences: readonly TrackingReportPolicyEvidence[];
  readonly storedJournalRefs: readonly string[];
  readonly storedReadModelRowRefs: readonly string[];
  readonly policyDecision: TrackingReportPolicyDecision | null;
  readonly reasonCodes: readonly string[];
  readonly auditRefs: readonly string[];
  readonly reportSurfaceRefs: readonly string[];
}): TrackingReportPolicyConsumerRow {
  return TrackingReportPolicyConsumerRowSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    rowId: input.rowId,
    consumerKind: input.consumerKind,
    readinessState: 'consumer-ready',
    requiredProofTier: 'P2_HOSTED_CI',
    currentProofTier: 'P2_HOSTED_CI',
    generatedAt: input.generatedAt,
    sourceProofRefs: [
      'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/18-service-read-model-proof.json',
      'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/21-product-surface-summary-proof.json',
    ],
    productSurfaceSummaryRefs: [
      'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/21-product-surface-summary-proof.json',
    ],
    reportSurfaceRefs: input.reportSurfaceRefs,
    storedJournalRefs: input.storedJournalRefs,
    storedReadModelRowRefs: input.storedReadModelRowRefs,
    evidenceReferences: input.evidenceReferences,
    policyDecision: input.policyDecision,
    reasonCodes: input.reasonCodes,
    auditRefs: input.auditRefs,
    reportConsumerClaimed: true,
    portalUiClaimed: false,
    childDeviceDeliveryClaimed: false,
    providerDeliveryClaimed: false,
    notificationReceiptClaimed: false,
    physicalDeviceClaimed: false,
    authorityClaimed: false,
    productClaimReady: false,
  });
}

function evidence(
  evidenceReferenceId: string,
  kind: TrackingReportPolicyEvidence['kind'],
  observedAt: string
): TrackingReportPolicyEvidence {
  return TrackingEvidenceTraceSchema.parse({
    evidenceReferenceId,
    kind,
    observedAt,
  });
}

