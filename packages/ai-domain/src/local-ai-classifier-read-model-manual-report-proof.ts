import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema,
  NonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import {
  LocalAiDeterministicClassifierResultSchema,
  LocalAiDeterministicClassifierTraceRefSchema,
  type LocalAiDeterministicClassifierResult,
} from './local-ai-deterministic-classifier-proof';
import { LocalAiResultNonClaimSchema, LocalAiResultProofRefSchema } from './local-ai-result-journal-sqlite-proof';
import {
  LocalAiEvaluationRequestIdSchema,
  LocalAiModelIdSchema,
  LocalAiPromptVersionSchema,
  LocalAiProviderIdSchema,
  LocalAiResultIdSchema,
  LocalAiRuntimeReferenceIdSchema,
  LocalAiTimestampSchema,
} from './local-ai-primitives';
import { PolicyActionSchema, PolicyReasonCodeSchema, PolicyRuleIdSchema } from '@ocentra-parent/policy-domain/policy';
import { ParentContractSchemaVersion, ParentContractSchemaVersionSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import { ParentEvidenceReferenceSchema } from '@ocentra-parent/family-domain/references';
const ReportCountSchema = Schema.Number.pipe(Schema.nonNegative(), Schema.int());

export const LocalAiClassifierReportRowIdSchema = brandedNonEmptyStringSchema('LocalAiClassifierReportRowId');
export const LocalAiClassifierReportSnapshotIdSchema = brandedNonEmptyStringSchema('LocalAiClassifierReportSnapshotId');
export const LocalAiClassifierManualReasonSchema = brandedNonEmptyStringSchema('LocalAiClassifierManualReason');

export const LocalAiClassifierReportStateSchema = withParser(Schema.Literal('ready', 'manual-required', 'unavailable'));

const LocalAiClassifierReportRowBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  reportRowId: LocalAiClassifierReportRowIdSchema,
  classifierRunId: NonEmptyStringSchema,
  sourceResultId: LocalAiResultIdSchema,
  requestId: LocalAiEvaluationRequestIdSchema,
  action: PolicyActionSchema,
  classifierState: Schema.Literal('classified', 'low-confidence', 'missing-evidence', 'runtime-unavailable'),
  reportState: LocalAiClassifierReportStateSchema,
  confidence: Schema.Number.pipe(Schema.between(0, 1)),
  reasonCodes: Schema.Array(PolicyReasonCodeSchema),
  manualRequiredReasons: Schema.Array(LocalAiClassifierManualReasonSchema),
  evidenceReferences: Schema.Array(ParentEvidenceReferenceSchema),
  parentRuleReferences: Schema.Array(PolicyRuleIdSchema),
  runtimeReferenceId: LocalAiRuntimeReferenceIdSchema,
  providerId: LocalAiProviderIdSchema,
  modelId: LocalAiModelIdSchema,
  promptVersion: LocalAiPromptVersionSchema,
  classifierTraceRefs: Schema.Array(LocalAiDeterministicClassifierTraceRefSchema),
  sourceProofRefs: Schema.Array(LocalAiResultProofRefSchema),
  reportOnly: Schema.Boolean,
  dryRun: Schema.Boolean,
  modelExecuted: Schema.Boolean,
  rawEvidenceRetained: Schema.Boolean,
  rawModelOutputRetained: Schema.Boolean,
  remoteApiClaimed: Schema.Boolean,
  policyAuthorityClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
  productionModelQualityClaimed: Schema.Boolean,
});

type LocalAiClassifierReportRowCandidate = Infer<typeof LocalAiClassifierReportRowBaseSchema>;

export const LocalAiClassifierReportRowSchema = withParser(
  LocalAiClassifierReportRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        localAiClassifierReportRowIsHonest(row) ||
        'Expected classifier report rows to be report-only dry-run rows without raw retention, model execution, policy authority, or enforcement'
    )
  )
);

const LocalAiClassifierReportSnapshotBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  snapshotId: LocalAiClassifierReportSnapshotIdSchema,
  generatedAt: LocalAiTimestampSchema,
  rows: Schema.Array(LocalAiClassifierReportRowSchema),
  readyRowCount: ReportCountSchema,
  manualRequiredRowCount: ReportCountSchema,
  unavailableRowCount: ReportCountSchema,
  sourceProofRefs: Schema.Array(LocalAiResultProofRefSchema),
  nonClaims: Schema.Array(LocalAiResultNonClaimSchema),
});

type LocalAiClassifierReportSnapshotCandidate = Infer<typeof LocalAiClassifierReportSnapshotBaseSchema>;

export const LocalAiClassifierReportSnapshotSchema = withParser(
  LocalAiClassifierReportSnapshotBaseSchema.pipe(
    Schema.filter(
      (snapshot) =>
        localAiClassifierReportSnapshotIsComplete(snapshot) ||
        'Expected classifier report snapshot counts and source proof refs to match its rows'
    )
  )
);

export type LocalAiClassifierReportRow = Infer<typeof LocalAiClassifierReportRowSchema>;
export type LocalAiClassifierReportSnapshot = Infer<typeof LocalAiClassifierReportSnapshotSchema>;
export type LocalAiClassifierReportState = Infer<typeof LocalAiClassifierReportStateSchema>;

const decodeProofRef = Schema.decodeUnknownSync(LocalAiResultProofRefSchema);
const decodeNonClaim = Schema.decodeUnknownSync(LocalAiResultNonClaimSchema);
const decodeManualReason = Schema.decodeUnknownSync(LocalAiClassifierManualReasonSchema);

export const LocalAiClassifierReadModelManualReportNonClaims = [
  decodeNonClaim('This proof projects deterministic classifier dry-run rows into a parent-facing report snapshot.'),
  decodeNonClaim('Manual-required and unavailable rows stay visible without becoming policy authority.'),
  decodeNonClaim(
    'This proof does not execute a model, prove model quality, render portal UI, dispatch enforcement, or retain raw evidence.'
  ),
] as const;

export function buildLocalAiClassifierReportSnapshot(input: {
  readonly generatedAt: string;
  readonly snapshotId: string;
  readonly sourceProofRefs: readonly string[];
  readonly classifierResults: readonly unknown[];
}): LocalAiClassifierReportSnapshot {
  const sourceProofRefs = input.sourceProofRefs.map((proofRef) => decodeProofRef(proofRef));
  const rows = input.classifierResults.map((result, index) =>
    reportRowFromClassifierResult(LocalAiDeterministicClassifierResultSchema.parse(result), index, sourceProofRefs)
  );

  return LocalAiClassifierReportSnapshotSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    snapshotId: input.snapshotId,
    generatedAt: input.generatedAt,
    rows,
    readyRowCount: rows.filter((row) => row.reportState === 'ready').length,
    manualRequiredRowCount: rows.filter((row) => row.reportState === 'manual-required').length,
    unavailableRowCount: rows.filter((row) => row.reportState === 'unavailable').length,
    sourceProofRefs,
    nonClaims: LocalAiClassifierReadModelManualReportNonClaims,
  });
}

function reportRowFromClassifierResult(
  classifierResult: LocalAiDeterministicClassifierResult,
  index: number,
  sourceProofRefs: readonly ReturnType<typeof decodeProofRef>[]
): LocalAiClassifierReportRow {
  const reportState = reportStateFor(classifierResult);

  return LocalAiClassifierReportRowSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    reportRowId: `local-ai-classifier-report:${index}:${classifierResult.result.resultId}`,
    classifierRunId: classifierResult.classifierRunId,
    sourceResultId: classifierResult.result.resultId,
    requestId: classifierResult.result.requestId,
    action: classifierResult.result.action,
    classifierState: classifierResult.state,
    reportState,
    confidence: classifierResult.result.confidence,
    reasonCodes: classifierResult.result.reasonCodes,
    manualRequiredReasons: manualRequiredReasonsFor(classifierResult, reportState),
    evidenceReferences: classifierResult.result.evidenceReferences,
    parentRuleReferences: classifierResult.result.parentRuleReferences,
    runtimeReferenceId: classifierResult.modelRuntime.runtimeReferenceId,
    providerId: classifierResult.modelRuntime.providerId,
    modelId: classifierResult.modelRuntime.modelId,
    promptVersion: classifierResult.promptVersion,
    classifierTraceRefs: classifierResult.classifierTraceRefs,
    sourceProofRefs,
    reportOnly: true,
    dryRun: classifierResult.dryRun,
    modelExecuted: classifierResult.modelExecuted,
    rawEvidenceRetained: classifierResult.rawEvidenceRetained,
    rawModelOutputRetained: false,
    remoteApiClaimed: classifierResult.remoteApiClaimed,
    policyAuthorityClaimed: classifierResult.policyAuthorityClaimed,
    enforcementClaimed: classifierResult.enforcementClaimed,
    productionModelQualityClaimed: classifierResult.productionModelQualityClaimed,
  });
}

function reportStateFor(classifierResult: LocalAiDeterministicClassifierResult): LocalAiClassifierReportState {
  if (
    classifierResult.state === 'runtime-unavailable' ||
    classifierResult.result.unknownState === 'model-unavailable' ||
    classifierResult.result.degradedState === 'provider-unavailable'
  ) {
    return 'unavailable';
  }
  if (
    classifierResult.state === 'low-confidence' ||
    classifierResult.state === 'missing-evidence' ||
    classifierResult.result.action === 'ask-parent' ||
    classifierResult.result.action === 'unknown' ||
    classifierResult.result.unknownState !== 'none' ||
    classifierResult.result.degradedState !== 'none' ||
    classifierResult.result.confidence < 0.6
  ) {
    return 'manual-required';
  }
  return 'ready';
}

function manualRequiredReasonsFor(
  classifierResult: LocalAiDeterministicClassifierResult,
  reportState: LocalAiClassifierReportState
): ReturnType<typeof decodeManualReason>[] {
  if (reportState === 'ready') {
    return [];
  }

  const reasons = [decodeManualReason(`manual:${classifierResult.state}`)];
  if (classifierResult.result.action === 'ask-parent' || classifierResult.result.action === 'unknown') {
    reasons.push(decodeManualReason(`manual:action:${classifierResult.result.action}`));
  }
  if (classifierResult.result.degradedState !== 'none') {
    reasons.push(decodeManualReason(`manual:degraded:${classifierResult.result.degradedState}`));
  }
  if (classifierResult.result.unknownState !== 'none') {
    reasons.push(decodeManualReason(`manual:unknown:${classifierResult.result.unknownState}`));
  }
  return reasons;
}

function localAiClassifierReportRowIsHonest(row: LocalAiClassifierReportRowCandidate): boolean {
  return (
    row.reportOnly &&
    row.dryRun &&
    classifierReportRefsArePresent(row) &&
    classifierReportStateIsConsistent(row) &&
    classifierReportHasNoOverclaims(row)
  );
}

function classifierReportRefsArePresent(row: LocalAiClassifierReportRowCandidate): boolean {
  return row.sourceProofRefs.length > 0 && row.parentRuleReferences.length > 0 && row.classifierTraceRefs.length > 0;
}

function classifierReportStateIsConsistent(row: LocalAiClassifierReportRowCandidate): boolean {
  if (row.reportState === 'ready') {
    return row.manualRequiredReasons.length === 0 && row.evidenceReferences.length > 0;
  }
  return row.manualRequiredReasons.length > 0;
}

function classifierReportHasNoOverclaims(row: LocalAiClassifierReportRowCandidate): boolean {
  return (
    !row.modelExecuted &&
    !row.rawEvidenceRetained &&
    !row.rawModelOutputRetained &&
    !row.remoteApiClaimed &&
    !row.policyAuthorityClaimed &&
    !row.enforcementClaimed &&
    !row.productionModelQualityClaimed
  );
}

function localAiClassifierReportSnapshotIsComplete(snapshot: LocalAiClassifierReportSnapshotCandidate): boolean {
  return (
    snapshot.sourceProofRefs.length > 0 &&
    snapshot.rows.length > 0 &&
    snapshot.rows.every((row) =>
      snapshot.sourceProofRefs.every((proofRef) => row.sourceProofRefs.includes(proofRef))
    ) &&
    snapshot.readyRowCount === snapshot.rows.filter((row) => row.reportState === 'ready').length &&
    snapshot.manualRequiredRowCount === snapshot.rows.filter((row) => row.reportState === 'manual-required').length &&
    snapshot.unavailableRowCount === snapshot.rows.filter((row) => row.reportState === 'unavailable').length &&
    snapshot.nonClaims.length > 0
  );
}

