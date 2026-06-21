import { type Infer, Schema, withParser } from './effect';
import { PolicyDecisionHandoffStateSchema } from './policy-contracts';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from './family-reference-primitives';
import {
  ScreenSummaryParentExplanationReadModelClaimBoundarySchema,
  ScreenSummaryParentExplanationReadModelDisplayStateSchema,
  ScreenSummaryParentExplanationReadModelIdSchema,
  ScreenSummaryParentExplanationReadModelRowIdSchema,
  ScreenSummaryParentExplanationReadModelSourceRowSchema,
  ScreenSummaryParentReadModelDeletionReasonsSchema,
  ScreenSummaryParentReadModelEvidenceRefsSchema,
  ScreenSummaryParentReadModelExplanationReasonsSchema,
  ScreenSummaryParentReadModelNonNegativeIntegerSchema,
  ScreenSummaryParentReadModelPolicyReasonsSchema,
  ScreenSummaryParentReadModelPolicyRulesSchema,
  ScreenSummaryParentReadModelRuntimeRefsSchema,
  ScreenSummaryParentReadModelScreenRefsSchema,
  ScreenSummaryParentReadModelTextSchema,
  type ScreenSummaryParentExplanationReadModelSourceRow,
} from './local-ai-screen-summary-parent-explanation-read-model-values';

const ScreenSummaryParentExplanationReadModelRowBaseSchema = Schema.Struct({
  rowId: ScreenSummaryParentExplanationReadModelRowIdSchema,
  displayState: ScreenSummaryParentExplanationReadModelDisplayStateSchema,
  sourceOcrResultRef: ScreenSummaryParentReadModelTextSchema,
  sourceQueueJobRef: ScreenSummaryParentReadModelTextSchema,
  primaryCategory: ScreenSummaryParentReadModelTextSchema,
  imageDigest: ScreenSummaryParentReadModelTextSchema,
  screenSummaryRefs: ScreenSummaryParentReadModelScreenRefsSchema,
  auditEvidenceRefs: ScreenSummaryParentReadModelEvidenceRefsSchema,
  policyDecisionRef: ScreenSummaryParentReadModelTextSchema,
  policyAction: ScreenSummaryParentReadModelTextSchema,
  policyReasonCodes: ScreenSummaryParentReadModelPolicyReasonsSchema,
  policyDryRun: Schema.Literal(true),
  enforcementHandoffState: PolicyDecisionHandoffStateSchema,
  parentRuleRefs: ScreenSummaryParentReadModelPolicyRulesSchema,
  localModelRuntimeRefs: ScreenSummaryParentReadModelRuntimeRefsSchema,
  custodyLabels: Schema.Array(ScreenSummaryParentReadModelTextSchema),
  deletionReasons: ScreenSummaryParentReadModelDeletionReasonsSchema,
  explanationReasons: ScreenSummaryParentReadModelExplanationReasonsSchema,
  claimBoundaries: ScreenSummaryParentExplanationReadModelClaimBoundarySchema,
});

type ReadModelRowCandidate = Infer<typeof ScreenSummaryParentExplanationReadModelRowBaseSchema>;

export const ScreenSummaryParentExplanationReadModelRowSchema = withParser(
  ScreenSummaryParentExplanationReadModelRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        screenSummaryParentExplanationReadModelRowIsHonest(row) ||
        'Expected parent read-model row to stay local-only, ref-cited, deleted-image, and non-enforcing'
    )
  )
);

const ScreenSummaryParentExplanationReadModelSnapshotBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  snapshotId: ScreenSummaryParentExplanationReadModelIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceProof: ScreenSummaryParentReadModelTextSchema,
  rows: Schema.Array(ScreenSummaryParentExplanationReadModelRowSchema).pipe(
    Schema.filter((rows) => rows.length > 0 || 'Expected parent read-model rows')
  ),
  summary: Schema.Struct({
    rowCount: ScreenSummaryParentReadModelNonNegativeIntegerSchema,
    readyRowCount: ScreenSummaryParentReadModelNonNegativeIntegerSchema,
    screenSummaryRefCount: ScreenSummaryParentReadModelNonNegativeIntegerSchema,
    localOnly: Schema.Literal(true),
    rawImageShown: Schema.Literal(false),
    rawImageRetained: Schema.Literal(false),
    remoteAiUsed: Schema.Literal(false),
    apiAiUsed: Schema.Literal(false),
    portalRuntimeClaimed: Schema.Literal(false),
    enforcementClaimed: Schema.Literal(false),
  }),
  claimBoundaries: ScreenSummaryParentExplanationReadModelClaimBoundarySchema,
});

type ReadModelSnapshotCandidate = Infer<typeof ScreenSummaryParentExplanationReadModelSnapshotBaseSchema>;

export const ScreenSummaryParentExplanationReadModelSnapshotSchema = withParser(
  ScreenSummaryParentExplanationReadModelSnapshotBaseSchema.pipe(
    Schema.filter(
      (snapshot) =>
        screenSummaryParentExplanationReadModelSnapshotIsHonest(snapshot) ||
        'Expected parent read-model snapshot to preserve refs and non-claims'
    )
  )
);

export const ScreenSummaryParentExplanationReadModelInputSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    snapshotId: ScreenSummaryParentExplanationReadModelIdSchema,
    generatedAt: ParentTimestampSchema,
    sourceProof: ScreenSummaryParentReadModelTextSchema,
    sourceRows: Schema.Array(ScreenSummaryParentExplanationReadModelSourceRowSchema).pipe(
      Schema.filter((rows) => rows.length > 0 || 'Expected source explanation proof rows')
    ),
    claimBoundaries: ScreenSummaryParentExplanationReadModelClaimBoundarySchema,
  })
);

export type ScreenSummaryParentExplanationReadModelRow = Infer<typeof ScreenSummaryParentExplanationReadModelRowSchema>;
export type ScreenSummaryParentExplanationReadModelSnapshot = Infer<
  typeof ScreenSummaryParentExplanationReadModelSnapshotSchema
>;
export type ScreenSummaryParentExplanationReadModelInput = Infer<
  typeof ScreenSummaryParentExplanationReadModelInputSchema
>;

export function buildScreenSummaryParentExplanationReadModelSnapshot(
  input: unknown
): ScreenSummaryParentExplanationReadModelSnapshot {
  const parsed = ScreenSummaryParentExplanationReadModelInputSchema.parse(input);
  const rows = parsed.sourceRows.map((row) =>
    ScreenSummaryParentExplanationReadModelRowSchema.parse(readModelRowFromSource(row))
  );
  return ScreenSummaryParentExplanationReadModelSnapshotSchema.parse({
    schemaVersion: parsed.schemaVersion,
    snapshotId: parsed.snapshotId,
    generatedAt: parsed.generatedAt,
    sourceProof: parsed.sourceProof,
    rows,
    summary: readModelSummary(rows),
    claimBoundaries: parsed.claimBoundaries,
  });
}

function readModelRowFromSource(row: ScreenSummaryParentExplanationReadModelSourceRow) {
  return {
    rowId: `${row.ocrResultId}-parent-read-model-row`,
    displayState: 'ready-for-parent-explanation',
    sourceOcrResultRef: row.ocrResultId,
    sourceQueueJobRef: row.sourceQueueJobId,
    primaryCategory: row.primaryCategory,
    imageDigest: row.imageDigest,
    screenSummaryRefs: row.screenSummaryRefs,
    auditEvidenceRefs: row.auditEvidenceReferences,
    policyDecisionRef: row.policyDecisionRef,
    policyAction: row.policyAction,
    policyReasonCodes: row.policyReasonCodes,
    policyDryRun: row.policyDryRun,
    enforcementHandoffState: row.enforcementHandoffState,
    parentRuleRefs: row.parentRuleRefs,
    localModelRuntimeRefs: row.localModelRuntimeRefs,
    custodyLabels: row.custodyLabels,
    deletionReasons: row.deletionReasons,
    explanationReasons: row.explanationReasons,
    claimBoundaries: {
      rawImageShown: false,
      rawImageRetained: row.claimBoundaries.rawImageRetained,
      remoteAiUsed: row.claimBoundaries.remoteAiUsed,
      apiAiUsed: row.claimBoundaries.apiAiUsed,
      policyAuthorityClaimed: row.claimBoundaries.policyAuthorityClaimed,
      portalRuntimeClaimed: row.claimBoundaries.portalRuntimeClaimed,
      enforcementClaimed: row.claimBoundaries.enforcementClaimed,
    },
  };
}

function readModelSummary(rows: ReadonlyArray<ReadModelRowCandidate>) {
  return {
    rowCount: rows.length,
    readyRowCount: rows.filter((row) => row.displayState === 'ready-for-parent-explanation').length,
    screenSummaryRefCount: rows.reduce((count, row) => count + row.screenSummaryRefs.length, 0),
    localOnly: true,
    rawImageShown: false,
    rawImageRetained: false,
    remoteAiUsed: false,
    apiAiUsed: false,
    portalRuntimeClaimed: false,
    enforcementClaimed: false,
  };
}

function screenSummaryParentExplanationReadModelRowIsHonest(row: ReadModelRowCandidate): boolean {
  return (
    row.displayState === 'ready-for-parent-explanation' &&
    row.policyDryRun &&
    row.enforcementHandoffState !== 'handed-off' &&
    row.custodyLabels.includes('child-device-query-store') &&
    row.deletionReasons.includes('screen-image-deleted') &&
    Object.values(row.claimBoundaries).every((claim) => claim === false)
  );
}

function screenSummaryParentExplanationReadModelSnapshotIsHonest(snapshot: ReadModelSnapshotCandidate): boolean {
  return (
    snapshot.summary.rowCount === snapshot.rows.length &&
    snapshot.summary.readyRowCount === snapshot.rows.length &&
    snapshot.summary.screenSummaryRefCount ===
      snapshot.rows.reduce((count, row) => count + row.screenSummaryRefs.length, 0) &&
    Object.values(snapshot.claimBoundaries).every((claim) => claim === false)
  );
}
