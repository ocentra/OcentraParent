import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  LocalAiTextOutputParserProofSchema,
  LocalAiTextOutputParserTraceRefSchema,
  type LocalAiTextOutputParserProof,
} from './local-ai-text-output-parser-proof';
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
import { PolicyActionSchema, PolicyReasonCodeSchema, PolicyRuleIdSchema } from './policy';
import { ParentContractSchemaVersion, ParentContractSchemaVersionSchema } from './reference-primitives';
import { ParentEvidenceReferenceSchema } from './references';

const NonEmptyTextParserReadModelText = Schema.String.pipe(Schema.minLength(1));
const TextParserReadModelCountSchema = Schema.Number.pipe(Schema.nonNegative(), Schema.int());

export const LocalAiTextParserReadModelRowIdSchema = NonEmptyTextParserReadModelText.pipe(
  Schema.brand('LocalAiTextParserReadModelRowId')
);
export const LocalAiTextParserReadModelSnapshotIdSchema = NonEmptyTextParserReadModelText.pipe(
  Schema.brand('LocalAiTextParserReadModelSnapshotId')
);
export const LocalAiTextParserManualReasonSchema = NonEmptyTextParserReadModelText.pipe(
  Schema.brand('LocalAiTextParserManualReason')
);

export const LocalAiTextParserReadModelStateSchema = withParser(Schema.Literal('ready', 'manual-required'));

const LocalAiTextParserReadModelRowBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  readModelRowId: LocalAiTextParserReadModelRowIdSchema,
  parserRunId: NonEmptyTextParserReadModelText,
  adapterRequestId: NonEmptyTextParserReadModelText,
  parserState: Schema.Literal('parsed-local-result', 'rejected-invalid-output', 'manual-required'),
  readModelState: LocalAiTextParserReadModelStateSchema,
  sourceResultId: Schema.Union(LocalAiResultIdSchema, Schema.Null),
  requestId: Schema.Union(LocalAiEvaluationRequestIdSchema, Schema.Null),
  action: Schema.Union(PolicyActionSchema, Schema.Null),
  confidence: Schema.Union(Schema.Number.pipe(Schema.between(0, 1)), Schema.Null),
  reasonCodes: Schema.Array(PolicyReasonCodeSchema),
  manualRequiredReasons: Schema.Array(LocalAiTextParserManualReasonSchema),
  evidenceReferences: Schema.Array(ParentEvidenceReferenceSchema),
  parentRuleReferences: Schema.Array(PolicyRuleIdSchema),
  runtimeReferenceId: Schema.Union(LocalAiRuntimeReferenceIdSchema, Schema.Null),
  providerId: Schema.Union(LocalAiProviderIdSchema, Schema.Null),
  modelId: Schema.Union(LocalAiModelIdSchema, Schema.Null),
  promptVersion: Schema.Union(LocalAiPromptVersionSchema, Schema.Null),
  parserTraceRefs: Schema.Array(LocalAiTextOutputParserTraceRefSchema),
  sourceProofRefs: Schema.Array(LocalAiResultProofRefSchema),
  parserRejectedOutput: Schema.Boolean,
  resultPolicyEligible: Schema.Boolean,
  reportOnly: Schema.Boolean,
  rawModelOutputRetained: Schema.Boolean,
  modelExecuted: Schema.Boolean,
  remoteApiClaimed: Schema.Boolean,
  policyAuthorityClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
  productionModelQualityClaimed: Schema.Boolean,
});

type LocalAiTextParserReadModelRowCandidate = Infer<typeof LocalAiTextParserReadModelRowBaseSchema>;

export const LocalAiTextParserReadModelRowSchema = withParser(
  LocalAiTextParserReadModelRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        localAiTextParserReadModelRowIsHonest(row) ||
        'Expected local text parser read-model rows to keep parser state visible without raw retention or authority claims'
    )
  )
);

const LocalAiTextParserReadModelSnapshotBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  snapshotId: LocalAiTextParserReadModelSnapshotIdSchema,
  generatedAt: LocalAiTimestampSchema,
  rows: Schema.Array(LocalAiTextParserReadModelRowSchema),
  readyRowCount: TextParserReadModelCountSchema,
  manualRequiredRowCount: TextParserReadModelCountSchema,
  rejectedParserRowCount: TextParserReadModelCountSchema,
  sourceProofRefs: Schema.Array(LocalAiResultProofRefSchema),
  nonClaims: Schema.Array(LocalAiResultNonClaimSchema),
});

type LocalAiTextParserReadModelSnapshotCandidate = Infer<typeof LocalAiTextParserReadModelSnapshotBaseSchema>;

export const LocalAiTextParserReadModelSnapshotSchema = withParser(
  LocalAiTextParserReadModelSnapshotBaseSchema.pipe(
    Schema.filter(
      (snapshot) =>
        localAiTextParserReadModelSnapshotIsComplete(snapshot) ||
        'Expected local text parser read-model snapshot counts and source proof refs to match its rows'
    )
  )
);

export type LocalAiTextParserReadModelRow = Infer<typeof LocalAiTextParserReadModelRowSchema>;
export type LocalAiTextParserReadModelSnapshot = Infer<typeof LocalAiTextParserReadModelSnapshotSchema>;
export type LocalAiTextParserReadModelState = Infer<typeof LocalAiTextParserReadModelStateSchema>;

const decodeProofRef = Schema.decodeUnknownSync(LocalAiResultProofRefSchema);
const decodeNonClaim = Schema.decodeUnknownSync(LocalAiResultNonClaimSchema);
const decodeManualReason = Schema.decodeUnknownSync(LocalAiTextParserManualReasonSchema);

export const LocalAiTextParserReadModelNonClaims = [
  decodeNonClaim('This proof projects local text parser outputs into parent-visible read-model rows.'),
  decodeNonClaim('Rejected or manual parser rows stay visible as manual-required and never become policy authority.'),
  decodeNonClaim(
    'This proof does not execute a model, prove model quality, render portal UI, dispatch enforcement, or retain raw model output.'
  ),
] as const;

export function buildLocalAiTextParserReadModelSnapshot(input: {
  readonly generatedAt: string;
  readonly snapshotId: string;
  readonly sourceProofRefs: readonly string[];
  readonly parserProofs: readonly unknown[];
}): LocalAiTextParserReadModelSnapshot {
  const sourceProofRefs = input.sourceProofRefs.map((proofRef) => decodeProofRef(proofRef));
  const rows = input.parserProofs.map((proof, index) =>
    readModelRowFromParserProof(LocalAiTextOutputParserProofSchema.parse(proof), index, sourceProofRefs)
  );

  return LocalAiTextParserReadModelSnapshotSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    snapshotId: input.snapshotId,
    generatedAt: input.generatedAt,
    rows,
    readyRowCount: rows.filter((row) => row.readModelState === 'ready').length,
    manualRequiredRowCount: rows.filter((row) => row.readModelState === 'manual-required').length,
    rejectedParserRowCount: rows.filter((row) => row.parserRejectedOutput).length,
    sourceProofRefs,
    nonClaims: LocalAiTextParserReadModelNonClaims,
  });
}

function readModelRowFromParserProof(
  parserProof: LocalAiTextOutputParserProof,
  index: number,
  sourceProofRefs: readonly ReturnType<typeof decodeProofRef>[]
): LocalAiTextParserReadModelRow {
  const readModelState = readModelStateFor(parserProof);

  return LocalAiTextParserReadModelRowSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    readModelRowId: `local-ai-text-parser-read-model:${index}:${parserProof.parserRunId}`,
    parserRunId: parserProof.parserRunId,
    adapterRequestId: parserProof.adapterRequestId,
    parserState: parserProof.state,
    readModelState,
    ...resultFieldsFor(parserProof),
    manualRequiredReasons: manualRequiredReasonsFor(parserProof, readModelState),
    parserTraceRefs: parserProof.outputTraceRefs,
    sourceProofRefs,
    parserRejectedOutput: parserProof.parserRejectedOutput,
    resultPolicyEligible: parserProof.resultPolicyEligible,
    reportOnly: true,
    ...claimFieldsFor(parserProof),
  });
}

function resultFieldsFor(parserProof: LocalAiTextOutputParserProof) {
  const result = parserProof.result;
  if (result === null) {
    return emptyResultFields();
  }

  return {
    sourceResultId: result.resultId,
    requestId: result.requestId,
    action: result.action,
    confidence: result.confidence,
    reasonCodes: result.reasonCodes,
    evidenceReferences: result.evidenceReferences,
    parentRuleReferences: result.parentRuleReferences,
    runtimeReferenceId: result.modelRuntime.runtimeReferenceId,
    providerId: result.modelRuntime.providerId,
    modelId: result.modelRuntime.modelId,
    promptVersion: result.promptVersion,
  };
}

function emptyResultFields() {
  return {
    sourceResultId: null,
    requestId: null,
    action: null,
    confidence: null,
    reasonCodes: [],
    evidenceReferences: [],
    parentRuleReferences: [],
    runtimeReferenceId: null,
    providerId: null,
    modelId: null,
    promptVersion: null,
  };
}

function claimFieldsFor(parserProof: LocalAiTextOutputParserProof) {
  return {
    rawModelOutputRetained: parserProof.rawModelOutputRetained,
    modelExecuted: parserProof.modelExecuted,
    remoteApiClaimed: parserProof.remoteApiClaimed,
    policyAuthorityClaimed: parserProof.policyAuthorityClaimed,
    enforcementClaimed: parserProof.enforcementClaimed,
    productionModelQualityClaimed: parserProof.productionModelQualityClaimed,
  };
}

function readModelStateFor(parserProof: LocalAiTextOutputParserProof): LocalAiTextParserReadModelState {
  if (
    parserProof.state === 'parsed-local-result' &&
    parserProof.resultPolicyEligible &&
    parserProof.result?.unknownState === 'none' &&
    parserProof.result.degradedState === 'none' &&
    parserProof.result.confidence >= 0.6
  ) {
    return 'ready';
  }

  return 'manual-required';
}

function manualRequiredReasonsFor(
  parserProof: LocalAiTextOutputParserProof,
  readModelState: LocalAiTextParserReadModelState
): ReturnType<typeof decodeManualReason>[] {
  if (readModelState === 'ready') {
    return [];
  }

  const reasons = [decodeManualReason(`manual:parser:${parserProof.state}`)];
  if (parserProof.parserRejectedOutput) {
    reasons.push(decodeManualReason('manual:parser-rejected-output'));
  }
  if (parserProof.result?.unknownState && parserProof.result.unknownState !== 'none') {
    reasons.push(decodeManualReason(`manual:unknown:${parserProof.result.unknownState}`));
  }
  if (parserProof.result?.degradedState && parserProof.result.degradedState !== 'none') {
    reasons.push(decodeManualReason(`manual:degraded:${parserProof.result.degradedState}`));
  }
  return reasons;
}

function localAiTextParserReadModelRowIsHonest(row: LocalAiTextParserReadModelRowCandidate): boolean {
  return (
    row.reportOnly &&
    row.parserTraceRefs.length > 0 &&
    row.sourceProofRefs.length > 0 &&
    localAiTextParserReadModelStateIsConsistent(row) &&
    localAiTextParserReadModelRowHasNoOverclaims(row)
  );
}

function localAiTextParserReadModelStateIsConsistent(row: LocalAiTextParserReadModelRowCandidate): boolean {
  if (row.readModelState === 'ready') {
    return localAiTextParserReadyRowIsConsistent(row);
  }

  return row.manualRequiredReasons.length > 0 && !row.resultPolicyEligible;
}

function localAiTextParserReadyRowIsConsistent(row: LocalAiTextParserReadModelRowCandidate): boolean {
  return (
    row.parserState === 'parsed-local-result' &&
    row.resultPolicyEligible &&
    !row.parserRejectedOutput &&
    row.manualRequiredReasons.length === 0 &&
    localAiTextParserReadyRowHasResultRefs(row)
  );
}

function localAiTextParserReadyRowHasResultRefs(row: LocalAiTextParserReadModelRowCandidate): boolean {
  return (
    row.sourceResultId !== null &&
    row.evidenceReferences.length > 0 &&
    row.parentRuleReferences.length > 0 &&
    row.runtimeReferenceId !== null &&
    row.providerId !== null &&
    row.modelId !== null &&
    row.promptVersion !== null
  );
}

function localAiTextParserReadModelRowHasNoOverclaims(row: LocalAiTextParserReadModelRowCandidate): boolean {
  return (
    !row.rawModelOutputRetained &&
    !row.modelExecuted &&
    !row.remoteApiClaimed &&
    !row.policyAuthorityClaimed &&
    !row.enforcementClaimed &&
    !row.productionModelQualityClaimed
  );
}

function localAiTextParserReadModelSnapshotIsComplete(snapshot: LocalAiTextParserReadModelSnapshotCandidate): boolean {
  return (
    snapshot.sourceProofRefs.length > 0 &&
    snapshot.rows.length > 0 &&
    snapshot.rows.every((row) =>
      snapshot.sourceProofRefs.every((proofRef) => row.sourceProofRefs.includes(proofRef))
    ) &&
    snapshot.readyRowCount === snapshot.rows.filter((row) => row.readModelState === 'ready').length &&
    snapshot.manualRequiredRowCount ===
      snapshot.rows.filter((row) => row.readModelState === 'manual-required').length &&
    snapshot.rejectedParserRowCount === snapshot.rows.filter((row) => row.parserRejectedOutput).length &&
    LocalAiTextParserReadModelNonClaims.every((nonClaim) => snapshot.nonClaims.includes(nonClaim))
  );
}
