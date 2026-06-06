import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from './reference-primitives';
import { TrackingEvidenceTraceSchema } from './tracking-location-policy';
import {
  TrackingPolicyAuditRefSchema,
  TrackingPolicyReasonCodeSchema,
  TrackingPolicySchemaVersion,
} from './tracking-location-policy-primitives';

const TrackingPortalDisplayBoundaryTextSchema = Schema.String.pipe(Schema.minLength(1));

export const TrackingPortalDisplayBoundaryRowIdSchema = TrackingPortalDisplayBoundaryTextSchema.pipe(
  Schema.brand('TrackingPortalDisplayBoundaryRowId')
);

export const TrackingPortalDisplayBoundaryProofRefSchema = TrackingPortalDisplayBoundaryTextSchema.pipe(
  Schema.brand('TrackingPortalDisplayBoundaryProofRef')
);

export const TrackingPortalDisplayBoundaryKindSchema = withParser(
  Schema.Literal(
    'service-read-model-display',
    'retention-settings-display',
    'family-dashboard-rollup-display',
    'unsupported-platform-manual-display'
  )
);

export const TrackingPortalDisplayBoundaryStateSchema = withParser(
  Schema.Literal('display-ready', 'authoring-ready', 'manual-required')
);

export const TrackingPortalDisplayBoundaryProofTierSchema = withParser(
  Schema.Literal('P1_FIXTURE_SIMULATION', 'P2_HOSTED_CI', 'P3_LOCAL_DEV_MACHINE')
);

export const TrackingPortalDisplayBoundaryRowSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    rowId: TrackingPortalDisplayBoundaryRowIdSchema,
    boundaryKind: TrackingPortalDisplayBoundaryKindSchema,
    boundaryState: TrackingPortalDisplayBoundaryStateSchema,
    requiredProofTier: TrackingPortalDisplayBoundaryProofTierSchema,
    currentProofTier: TrackingPortalDisplayBoundaryProofTierSchema,
    generatedAt: ParentTimestampSchema,
    sourceProofRefs: Schema.Array(TrackingPortalDisplayBoundaryProofRefSchema),
    hostedProofRefs: Schema.Array(TrackingPortalDisplayBoundaryProofRefSchema),
    evidenceReferences: Schema.Array(TrackingEvidenceTraceSchema),
    visibleStatusCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
    editableSettingCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
    reasonCodes: Schema.Array(TrackingPolicyReasonCodeSchema),
    auditRefs: Schema.Array(TrackingPolicyAuditRefSchema),
    portalDisplayClaimed: Schema.Literal(true),
    portalAuthoringClaimed: Schema.Boolean,
    portalEvaluatorClaimed: Schema.Literal(false),
    policyEvaluationClaimed: Schema.Literal(false),
    serviceMutationClaimed: Schema.Literal(false),
    platformWriterExecutionClaimed: Schema.Literal(false),
    childRuntimeExecutionClaimed: Schema.Literal(false),
    providerDeliveryClaimed: Schema.Literal(false),
    notificationReceiptClaimed: Schema.Literal(false),
    physicalDeviceClaimed: Schema.Literal(false),
    authorityClaimed: Schema.Literal(false),
    productClaimReady: Schema.Literal(false),
  })
    .pipe(Schema.filter((row) => row.sourceProofRefs.length > 0 || 'Tracking portal display rows need proof refs'))
    .pipe(
      Schema.filter((row) => row.hostedProofRefs.length > 0 || 'Tracking portal display rows need hosted proof refs')
    )
    .pipe(
      Schema.filter((row) => row.evidenceReferences.length > 0 || 'Tracking portal display rows need evidence refs')
    )
    .pipe(
      Schema.filter(
        (row) =>
          row.boundaryKind !== 'retention-settings-display' ||
          row.editableSettingCount > 0 ||
          'Tracking retention settings display rows need visible editable settings'
      )
    )
);

export const TrackingPortalDisplayBoundaryProofSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    proofMode: Schema.Literal('tracking-portal-display-boundary-proof'),
    generatedAt: ParentTimestampSchema,
    rows: Schema.Array(TrackingPortalDisplayBoundaryRowSchema),
    productClaims: Schema.Struct({
      productClaimReady: Schema.Literal(false),
      portalEvaluatorClaimed: Schema.Literal(false),
      policyEvaluationClaimed: Schema.Literal(false),
      serviceMutationClaimed: Schema.Literal(false),
      platformWriterExecutionClaimed: Schema.Literal(false),
      childRuntimeExecutionClaimed: Schema.Literal(false),
      providerDeliveryClaimed: Schema.Literal(false),
      notificationReceiptClaimed: Schema.Literal(false),
      physicalDeviceClaimed: Schema.Literal(false),
      authorityClaimed: Schema.Literal(false),
    }),
  }).pipe(Schema.filter((proof) => proof.rows.length >= 4 || 'Tracking portal display proof needs all boundary rows'))
);

export type TrackingPortalDisplayBoundaryKind = Infer<typeof TrackingPortalDisplayBoundaryKindSchema>;
export type TrackingPortalDisplayBoundaryRow = Infer<typeof TrackingPortalDisplayBoundaryRowSchema>;
export type TrackingPortalDisplayBoundaryProof = Infer<typeof TrackingPortalDisplayBoundaryProofSchema>;
type TrackingPortalDisplayBoundaryEvidence = Infer<typeof TrackingEvidenceTraceSchema>;

export function buildTrackingPortalDisplayBoundaryProof(generatedAt: string): TrackingPortalDisplayBoundaryProof {
  return TrackingPortalDisplayBoundaryProofSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    proofMode: 'tracking-portal-display-boundary-proof',
    generatedAt,
    rows: [
      row({
        rowId: 'tracking-portal-display-boundary-service-read-model',
        boundaryKind: 'service-read-model-display',
        boundaryState: 'display-ready',
        generatedAt,
        evidenceReferenceId: 'tracking-portal-display-evidence-service-read-model',
        evidenceKind: 'query-store-summary',
        visibleStatusCount: 4,
        editableSettingCount: 0,
        reasonCodes: ['tracking-portal-service-read-model-display-ready'],
        auditRefs: ['tracking-portal-display-audit-service-read-model'],
      }),
      row({
        rowId: 'tracking-portal-display-boundary-retention-settings',
        boundaryKind: 'retention-settings-display',
        boundaryState: 'authoring-ready',
        generatedAt,
        evidenceReferenceId: 'tracking-portal-display-evidence-retention-settings',
        evidenceKind: 'policy-decision',
        visibleStatusCount: 5,
        editableSettingCount: 5,
        reasonCodes: ['tracking-portal-retention-settings-authoring-ready'],
        auditRefs: ['tracking-portal-display-audit-retention-settings'],
        portalAuthoringClaimed: true,
      }),
      row({
        rowId: 'tracking-portal-display-boundary-family-dashboard',
        boundaryKind: 'family-dashboard-rollup-display',
        boundaryState: 'display-ready',
        generatedAt,
        evidenceReferenceId: 'tracking-portal-display-evidence-family-dashboard',
        evidenceKind: 'query-store-summary',
        visibleStatusCount: 3,
        editableSettingCount: 0,
        reasonCodes: ['tracking-portal-family-dashboard-display-ready'],
        auditRefs: ['tracking-portal-display-audit-family-dashboard'],
      }),
      row({
        rowId: 'tracking-portal-display-boundary-unsupported-platform',
        boundaryKind: 'unsupported-platform-manual-display',
        boundaryState: 'manual-required',
        generatedAt,
        evidenceReferenceId: 'tracking-portal-display-evidence-unsupported-platform',
        evidenceKind: 'query-store-summary',
        visibleStatusCount: 4,
        editableSettingCount: 0,
        reasonCodes: ['tracking-portal-unsupported-platform-manual-display-ready'],
        auditRefs: ['tracking-portal-display-audit-unsupported-platform'],
      }),
    ],
    productClaims: {
      productClaimReady: false,
      portalEvaluatorClaimed: false,
      policyEvaluationClaimed: false,
      serviceMutationClaimed: false,
      platformWriterExecutionClaimed: false,
      childRuntimeExecutionClaimed: false,
      providerDeliveryClaimed: false,
      notificationReceiptClaimed: false,
      physicalDeviceClaimed: false,
      authorityClaimed: false,
    },
  });
}

function row(input: {
  readonly rowId: string;
  readonly boundaryKind: TrackingPortalDisplayBoundaryKind;
  readonly boundaryState: TrackingPortalDisplayBoundaryRow['boundaryState'];
  readonly generatedAt: string;
  readonly evidenceReferenceId: string;
  readonly evidenceKind: TrackingPortalDisplayBoundaryEvidence['kind'];
  readonly visibleStatusCount: number;
  readonly editableSettingCount: number;
  readonly reasonCodes: readonly string[];
  readonly auditRefs: readonly string[];
  readonly portalAuthoringClaimed?: boolean;
}): TrackingPortalDisplayBoundaryRow {
  return TrackingPortalDisplayBoundaryRowSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    rowId: input.rowId,
    boundaryKind: input.boundaryKind,
    boundaryState: input.boundaryState,
    requiredProofTier: 'P2_HOSTED_CI',
    currentProofTier: 'P2_HOSTED_CI',
    generatedAt: input.generatedAt,
    sourceProofRefs: [
      'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/18-service-read-model-proof.json',
      'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/20-service-data-ui-proof.json',
      'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/23-family-dashboard-rollup-proof.json',
      'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/24-retention-settings-read-model-proof.json',
    ],
    hostedProofRefs: [
      'output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/18-service-data-ui-proof.json',
      'output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/19-child-runtime-ui-proof.json',
      'output/tracking-plan-proof/31-platform-extension-checklists-and-proof-routing/19-unsupported-manual-hosted-ui-proof.json',
      'test-results/tracking-plan-hosted-ui-proof/accessibility-summary.json',
    ],
    evidenceReferences: [
      TrackingEvidenceTraceSchema.parse({
        evidenceReferenceId: input.evidenceReferenceId,
        kind: input.evidenceKind,
        observedAt: input.generatedAt,
      }),
    ],
    visibleStatusCount: input.visibleStatusCount,
    editableSettingCount: input.editableSettingCount,
    reasonCodes: input.reasonCodes,
    auditRefs: input.auditRefs,
    portalDisplayClaimed: true,
    portalAuthoringClaimed: input.portalAuthoringClaimed ?? false,
    portalEvaluatorClaimed: false,
    policyEvaluationClaimed: false,
    serviceMutationClaimed: false,
    platformWriterExecutionClaimed: false,
    childRuntimeExecutionClaimed: false,
    providerDeliveryClaimed: false,
    notificationReceiptClaimed: false,
    physicalDeviceClaimed: false,
    authorityClaimed: false,
    productClaimReady: false,
  });
}
