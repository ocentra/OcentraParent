import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from './effect';
import { ParentTimestampSchema } from './family-reference-primitives';
import { TrackingEvidenceTraceSchema } from './tracking-location-policy';
import {
  TrackingPolicyAuditRefSchema,
  TrackingPolicyReasonCodeSchema,
  TrackingPolicySchemaVersion,
} from './tracking-location-policy-primitives';

export const TrackingFamilyDashboardRollupRowIdSchema = brandedNonEmptyStringSchema(
  'TrackingFamilyDashboardRollupRowId'
);

export const TrackingFamilyDashboardRollupProofRefSchema = brandedNonEmptyStringSchema(
  'TrackingFamilyDashboardRollupProofRef'
);

export const TrackingFamilyDashboardRollupKindSchema = withParser(
  Schema.Literal('family-active-summary', 'child-attention-summary', 'retention-audit-summary')
);

export const TrackingFamilyDashboardRollupStateSchema = withParser(Schema.Literal('rollup-ready', 'manual-required'));

export const TrackingFamilyDashboardRollupProofTierSchema = withParser(
  Schema.Literal('P1_FIXTURE_SIMULATION', 'P2_HOSTED_CI', 'P3_LOCAL_DEV_MACHINE')
);

export const TrackingFamilyDashboardRollupSeveritySchema = withParser(
  Schema.Literal('calm', 'attention', 'manual-required')
);

export const TrackingFamilyDashboardRollupRowSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    rowId: TrackingFamilyDashboardRollupRowIdSchema,
    rollupKind: TrackingFamilyDashboardRollupKindSchema,
    rollupState: TrackingFamilyDashboardRollupStateSchema,
    requiredProofTier: TrackingFamilyDashboardRollupProofTierSchema,
    currentProofTier: TrackingFamilyDashboardRollupProofTierSchema,
    generatedAt: ParentTimestampSchema,
    sourceProofRefs: Schema.Array(TrackingFamilyDashboardRollupProofRefSchema),
    productSurfaceSummaryRefs: Schema.Array(TrackingFamilyDashboardRollupProofRefSchema),
    reportConsumerRefs: Schema.Array(TrackingFamilyDashboardRollupProofRefSchema),
    evidenceReferences: Schema.Array(TrackingEvidenceTraceSchema),
    visibleChildCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
    attentionItemCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
    retainedAuditItemCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
    severity: TrackingFamilyDashboardRollupSeveritySchema,
    reasonCodes: Schema.Array(TrackingPolicyReasonCodeSchema),
    auditRefs: Schema.Array(TrackingPolicyAuditRefSchema),
    dashboardRollupClaimed: Schema.Literal(true),
    portalUiClaimed: Schema.Literal(false),
    childDeviceDeliveryClaimed: Schema.Literal(false),
    providerDeliveryClaimed: Schema.Literal(false),
    notificationReceiptClaimed: Schema.Literal(false),
    physicalDeviceClaimed: Schema.Literal(false),
    authorityClaimed: Schema.Literal(false),
    productClaimReady: Schema.Literal(false),
  })
    .pipe(Schema.filter((row) => row.sourceProofRefs.length > 0 || 'Tracking dashboard rollups need source proof refs'))
    .pipe(
      Schema.filter(
        (row) =>
          row.productSurfaceSummaryRefs.length > 0 || 'Tracking dashboard rollups need product-surface summary refs'
      )
    )
    .pipe(Schema.filter((row) => row.evidenceReferences.length > 0 || 'Tracking dashboard rollups need evidence refs'))
    .pipe(
      Schema.filter(
        (row) =>
          row.rollupKind !== 'child-attention-summary' ||
          row.attentionItemCount > 0 ||
          'Tracking child attention rollups need at least one attention item'
      )
    )
);

export const TrackingFamilyDashboardRollupProofSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    proofMode: Schema.Literal('tracking-family-dashboard-rollup-proof'),
    generatedAt: ParentTimestampSchema,
    rows: Schema.Array(TrackingFamilyDashboardRollupRowSchema),
    productClaims: Schema.Struct({
      productClaimReady: Schema.Literal(false),
      portalUiClaimed: Schema.Literal(false),
      childDeviceDeliveryClaimed: Schema.Literal(false),
      providerDeliveryClaimed: Schema.Literal(false),
      notificationReceiptClaimed: Schema.Literal(false),
      physicalDeviceClaimed: Schema.Literal(false),
      authorityClaimed: Schema.Literal(false),
    }),
  }).pipe(Schema.filter((proof) => proof.rows.length >= 3 || 'Tracking dashboard rollup proof needs all rollup rows'))
);

export type TrackingFamilyDashboardRollupKind = Infer<typeof TrackingFamilyDashboardRollupKindSchema>;
export type TrackingFamilyDashboardRollupRow = Infer<typeof TrackingFamilyDashboardRollupRowSchema>;
export type TrackingFamilyDashboardRollupProof = Infer<typeof TrackingFamilyDashboardRollupProofSchema>;
type TrackingFamilyDashboardRollupEvidence = Infer<typeof TrackingEvidenceTraceSchema>;

export function buildTrackingFamilyDashboardRollupProof(generatedAt: string): TrackingFamilyDashboardRollupProof {
  const timestamp = generatedAt;

  return TrackingFamilyDashboardRollupProofSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    proofMode: 'tracking-family-dashboard-rollup-proof',
    generatedAt: timestamp,
    rows: [
      row({
        rowId: 'tracking-family-dashboard-row-active-summary',
        rollupKind: 'family-active-summary',
        generatedAt: timestamp,
        evidenceReferences: [
          evidence('tracking-family-dashboard-evidence-active-summary', 'query-store-summary', timestamp),
        ],
        visibleChildCount: 2,
        attentionItemCount: 1,
        retainedAuditItemCount: 0,
        severity: 'attention',
        reasonCodes: ['tracking-family-dashboard-active-summary-ready'],
        auditRefs: ['tracking-family-dashboard-audit-active-summary'],
      }),
      row({
        rowId: 'tracking-family-dashboard-row-child-attention',
        rollupKind: 'child-attention-summary',
        generatedAt: timestamp,
        evidenceReferences: [
          evidence('tracking-family-dashboard-evidence-child-attention', 'policy-decision', timestamp),
        ],
        visibleChildCount: 1,
        attentionItemCount: 2,
        retainedAuditItemCount: 0,
        severity: 'attention',
        reasonCodes: ['tracking-family-dashboard-child-attention-ready'],
        auditRefs: ['tracking-family-dashboard-audit-child-attention'],
      }),
      row({
        rowId: 'tracking-family-dashboard-row-retention-audit',
        rollupKind: 'retention-audit-summary',
        generatedAt: timestamp,
        evidenceReferences: [
          evidence('tracking-family-dashboard-evidence-retention-audit', 'query-store-summary', timestamp),
        ],
        visibleChildCount: 0,
        attentionItemCount: 0,
        retainedAuditItemCount: 2,
        severity: 'calm',
        reasonCodes: ['tracking-family-dashboard-retention-audit-ready'],
        auditRefs: ['tracking-family-dashboard-audit-retention-audit'],
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
  readonly rollupKind: TrackingFamilyDashboardRollupKind;
  readonly generatedAt: string;
  readonly evidenceReferences: readonly TrackingFamilyDashboardRollupEvidence[];
  readonly visibleChildCount: number;
  readonly attentionItemCount: number;
  readonly retainedAuditItemCount: number;
  readonly severity: TrackingFamilyDashboardRollupRow['severity'];
  readonly reasonCodes: readonly string[];
  readonly auditRefs: readonly string[];
}): TrackingFamilyDashboardRollupRow {
  return TrackingFamilyDashboardRollupRowSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    rowId: input.rowId,
    rollupKind: input.rollupKind,
    rollupState: 'rollup-ready',
    requiredProofTier: 'P2_HOSTED_CI',
    currentProofTier: 'P2_HOSTED_CI',
    generatedAt: input.generatedAt,
    sourceProofRefs: [
      'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/18-service-read-model-proof.json',
      'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/21-product-surface-summary-proof.json',
      'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/22-report-policy-consumer-proof.json',
    ],
    productSurfaceSummaryRefs: [
      'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/21-product-surface-summary-proof.json',
    ],
    reportConsumerRefs: [
      'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/22-report-policy-consumer-proof.json',
    ],
    evidenceReferences: input.evidenceReferences,
    visibleChildCount: input.visibleChildCount,
    attentionItemCount: input.attentionItemCount,
    retainedAuditItemCount: input.retainedAuditItemCount,
    severity: input.severity,
    reasonCodes: input.reasonCodes,
    auditRefs: input.auditRefs,
    dashboardRollupClaimed: true,
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
  kind: TrackingFamilyDashboardRollupEvidence['kind'],
  observedAt: string
): TrackingFamilyDashboardRollupEvidence {
  return TrackingEvidenceTraceSchema.parse({
    evidenceReferenceId,
    kind,
    observedAt,
  });
}
