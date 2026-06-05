import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  LocalAiConfidenceSchema,
  LocalAiModelIdSchema,
  LocalAiPromptVersionSchema,
  LocalAiRuntimeReferenceIdSchema,
} from './local-ai-primitives';
import { PolicyActionSchema, PolicyDecisionHandoffStateSchema, PolicyDecisionIdSchema } from './policy';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from './reference-primitives';
import {
  ScreenAiCategorySchema,
  ScreenAiImageDigestSchema,
  ScreenAiJournalEntryIdSchema,
  ScreenAiJournalNonNegativeIntegerSchema,
  ScreenAiJournalReadModelClaimBoundarySchema,
  ScreenAiJournalReadModelRowIdSchema,
  ScreenAiJournalReadModelSnapshotIdSchema,
  ScreenAiJournalReadModelSourceRowBaseSchema,
  ScreenAiJournalTextSchema,
  ScreenAiLocalAiResultIdSchema,
  ScreenAiQueueJobIdSchema,
  ScreenAiReadModelEvidenceRefsSchema,
  ScreenAiReadModelPolicyReasonCodesSchema,
  ScreenAiReadModelPolicyRuleRefsSchema,
  ScreenAiSourceAnalysisRowIdSchema,
  ScreenAiSqliteRowIdSchema,
  type ScreenAiJournalReadModelSourceRowCandidate,
} from './screen-ai-journal-read-model-proof-values';

const ScreenAiJournalReadModelSourceRowSchema = withParser(
  ScreenAiJournalReadModelSourceRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        screenAiJournalReadModelSourceRowIsReady(row) ||
        'Expected deleted local screen AI source row with policy/read-model refs'
    )
  )
);

const ScreenAiJournalReadModelRowBaseSchema = Schema.Struct({
  rowId: ScreenAiJournalReadModelRowIdSchema,
  analysisRowRef: ScreenAiSourceAnalysisRowIdSchema,
  queueJobRef: ScreenAiQueueJobIdSchema,
  localAiResultRef: ScreenAiLocalAiResultIdSchema,
  journalEntryRef: ScreenAiJournalEntryIdSchema,
  sqliteRowRef: ScreenAiSqliteRowIdSchema,
  journalState: Schema.Literal('journaled'),
  sqliteProjectionState: Schema.Literal('read-model-present'),
  modelRuntimeRef: LocalAiRuntimeReferenceIdSchema,
  modelId: LocalAiModelIdSchema,
  promptOrTemplateVersion: LocalAiPromptVersionSchema,
  primaryCategory: ScreenAiCategorySchema,
  confidence: LocalAiConfidenceSchema,
  imageDigest: ScreenAiImageDigestSchema,
  imageDeletionState: Schema.Literal('deleted'),
  rawImageRetained: Schema.Literal(false),
  custodyState: Schema.Literal('child-device-journal'),
  evidenceReferenceIds: ScreenAiReadModelEvidenceRefsSchema,
  policyDecisionRef: PolicyDecisionIdSchema,
  policyAction: PolicyActionSchema,
  policyReasonCodes: ScreenAiReadModelPolicyReasonCodesSchema,
  policyDryRun: Schema.Literal(true),
  enforcementHandoffState: PolicyDecisionHandoffStateSchema,
  parentRuleRefs: ScreenAiReadModelPolicyRuleRefsSchema,
  claimBoundaries: ScreenAiJournalReadModelClaimBoundarySchema,
});

type ScreenAiJournalReadModelRowCandidate = Infer<typeof ScreenAiJournalReadModelRowBaseSchema>;

export const ScreenAiJournalReadModelRowSchema = withParser(
  ScreenAiJournalReadModelRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        screenAiJournalReadModelRowIsHonest(row) ||
        'Expected screen AI journal/read-model row to preserve refs, deletion, dry-run policy, and non-claims'
    )
  )
);

const ScreenAiJournalReadModelSnapshotBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  snapshotId: ScreenAiJournalReadModelSnapshotIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceProof: ScreenAiJournalTextSchema,
  sourceRows: Schema.Array(ScreenAiJournalReadModelSourceRowSchema).pipe(
    Schema.filter((rows) => rows.length > 0 || 'Expected source screen AI journal rows')
  ),
  rows: Schema.Array(ScreenAiJournalReadModelRowSchema).pipe(
    Schema.filter((rows) => rows.length > 0 || 'Expected screen AI journal read-model rows')
  ),
  summary: Schema.Struct({
    rowCount: ScreenAiJournalNonNegativeIntegerSchema,
    journaledRowCount: ScreenAiJournalNonNegativeIntegerSchema,
    sqliteProjectedRowCount: ScreenAiJournalNonNegativeIntegerSchema,
    deletedImageRowCount: ScreenAiJournalNonNegativeIntegerSchema,
    dryRunPolicyRowCount: ScreenAiJournalNonNegativeIntegerSchema,
    rawImageRetained: Schema.Literal(false),
    remoteAiUsed: Schema.Literal(false),
    apiAiUsed: Schema.Literal(false),
    enforcementClaimed: Schema.Literal(false),
  }),
  claimBoundaries: ScreenAiJournalReadModelClaimBoundarySchema,
});

type ScreenAiJournalReadModelSnapshotCandidate = Infer<typeof ScreenAiJournalReadModelSnapshotBaseSchema>;

export const ScreenAiJournalReadModelSnapshotSchema = withParser(
  ScreenAiJournalReadModelSnapshotBaseSchema.pipe(
    Schema.filter(
      (snapshot) =>
        screenAiJournalReadModelSnapshotIsHonest(snapshot) ||
        'Expected screen AI journal/read-model snapshot counts and non-claims to match rows'
    )
  )
);

export const ScreenAiJournalReadModelInputSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    snapshotId: ScreenAiJournalReadModelSnapshotIdSchema,
    generatedAt: ParentTimestampSchema,
    sourceProof: ScreenAiJournalTextSchema,
    sourceRows: Schema.Array(ScreenAiJournalReadModelSourceRowSchema).pipe(
      Schema.filter((rows) => rows.length > 0 || 'Expected source screen AI rows')
    ),
    claimBoundaries: ScreenAiJournalReadModelClaimBoundarySchema,
  })
);

export type ScreenAiJournalReadModelRow = Infer<typeof ScreenAiJournalReadModelRowSchema>;
export type ScreenAiJournalReadModelSnapshot = Infer<typeof ScreenAiJournalReadModelSnapshotSchema>;
export type ScreenAiJournalReadModelInput = Infer<typeof ScreenAiJournalReadModelInputSchema>;

export function buildScreenAiJournalReadModelSnapshot(input: unknown): ScreenAiJournalReadModelSnapshot {
  const parsed = ScreenAiJournalReadModelInputSchema.parse(input);
  const rows = parsed.sourceRows.map((row) => ScreenAiJournalReadModelRowSchema.parse(readModelRowFromSource(row)));
  return ScreenAiJournalReadModelSnapshotSchema.parse({
    schemaVersion: parsed.schemaVersion,
    snapshotId: parsed.snapshotId,
    generatedAt: parsed.generatedAt,
    sourceProof: parsed.sourceProof,
    sourceRows: parsed.sourceRows,
    rows,
    summary: readModelSummary(rows),
    claimBoundaries: parsed.claimBoundaries,
  });
}

function readModelRowFromSource(row: Infer<typeof ScreenAiJournalReadModelSourceRowSchema>) {
  return {
    rowId: `${row.analysisRowId}-screen-ai-journal-read-model`,
    analysisRowRef: row.analysisRowId,
    queueJobRef: row.queueJobId,
    localAiResultRef: row.localAiResultId,
    journalEntryRef: `${row.analysisRowId}-journal`,
    sqliteRowRef: `${row.readModelRowId}-sqlite-read-model`,
    journalState: 'journaled',
    sqliteProjectionState: 'read-model-present',
    modelRuntimeRef: row.modelRuntimeRef,
    modelId: row.modelId,
    promptOrTemplateVersion: row.promptOrTemplateVersion,
    primaryCategory: row.primaryCategory,
    confidence: row.confidence,
    imageDigest: row.imageDigest,
    imageDeletionState: row.imageDeletionState,
    rawImageRetained: row.rawImageRetained,
    custodyState: row.custodyState,
    evidenceReferenceIds: row.evidenceReferenceIds,
    policyDecisionRef: row.policyDecisionRef,
    policyAction: row.policyAction,
    policyReasonCodes: row.policyReasonCodes,
    policyDryRun: row.policyDryRun,
    enforcementHandoffState: row.enforcementHandoffState,
    parentRuleRefs: row.parentRuleRefs,
    claimBoundaries: noClaims(),
  };
}

function readModelSummary(rows: ReadonlyArray<ScreenAiJournalReadModelRowCandidate>) {
  return {
    rowCount: rows.length,
    journaledRowCount: rows.filter((row) => row.journalState === 'journaled').length,
    sqliteProjectedRowCount: rows.filter((row) => row.sqliteProjectionState === 'read-model-present').length,
    deletedImageRowCount: rows.filter((row) => row.imageDeletionState === 'deleted' && !row.rawImageRetained).length,
    dryRunPolicyRowCount: rows.filter((row) => row.policyDryRun).length,
    rawImageRetained: false,
    remoteAiUsed: false,
    apiAiUsed: false,
    enforcementClaimed: false,
  };
}

function screenAiJournalReadModelSourceRowIsReady(row: ScreenAiJournalReadModelSourceRowCandidate): boolean {
  return (
    row.policyDryRun &&
    row.enforcementHandoffState !== 'handed-off' &&
    row.evidenceReferenceIds.length > 0 &&
    row.parentRuleRefs.length > 0 &&
    row.readModelRowId === row.analysisRowId
  );
}

function screenAiJournalReadModelRowIsHonest(row: ScreenAiJournalReadModelRowCandidate): boolean {
  return (
    row.journalEntryRef.length > 0 &&
    row.sqliteRowRef.length > 0 &&
    row.imageDeletionState === 'deleted' &&
    !row.rawImageRetained &&
    row.policyDryRun &&
    row.enforcementHandoffState !== 'handed-off' &&
    Object.values(row.claimBoundaries).every((claim) => claim === false)
  );
}

function screenAiJournalReadModelSnapshotIsHonest(snapshot: ScreenAiJournalReadModelSnapshotCandidate): boolean {
  return (
    snapshot.summary.rowCount === snapshot.rows.length &&
    snapshot.summary.journaledRowCount === snapshot.rows.length &&
    snapshot.summary.sqliteProjectedRowCount === snapshot.rows.length &&
    snapshot.summary.deletedImageRowCount === snapshot.rows.length &&
    snapshot.summary.dryRunPolicyRowCount === snapshot.rows.length &&
    Object.values(snapshot.claimBoundaries).every((claim) => claim === false)
  );
}

function noClaims() {
  return {
    rawImageRetained: false,
    remoteAiUsed: false,
    apiAiUsed: false,
    policyAuthorityClaimed: false,
    portalRuntimeClaimed: false,
    enforcementClaimed: false,
    runtimeSqliteWriterClaimed: false,
  };
}
