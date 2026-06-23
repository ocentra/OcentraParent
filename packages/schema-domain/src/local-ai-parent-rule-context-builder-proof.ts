import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from './effect';
import {
  LocalAiStoredEvidenceContextBuildInputSchema,
  type LocalAiEvidenceContextBuildResult,
  type LocalAiStoredEvidenceContextBuildInput,
} from './ai-context';
import {
  LocalAiContextBuildStateSchema,
  LocalAiContextReasonCodeSchema,
  LocalAiContextNonNegativeCountSchema,
  LocalAiEvidenceContextIdSchema,
  LocalAiEvidenceContextRefIdSchema,
  LocalAiEvidenceContextSummarySchema,
  LocalAiParentRuleContextRefIdSchema,
  type LocalAiContextReasonCode,
} from './ai-context-primitives';
import { buildLocalAiEvidenceContext } from './local-ai-context-builder';
import { LocalAiEvaluationRequestIdSchema, LocalAiTimestampSchema } from './ai-primitives';
import { PolicyRuleIdSchema } from './policy-contracts';
import { ChildProfileReferenceSchema, ParentDeviceReferenceSchema } from './family-references';

const LocalAiParentRuleContextBuilderProofRowBaseSchema = Schema.Struct({
  state: LocalAiContextBuildStateSchema,
  contextId: Schema.Union(LocalAiEvidenceContextIdSchema, Schema.Null),
  requestId: LocalAiEvaluationRequestIdSchema,
  builtAt: LocalAiTimestampSchema,
  childProfile: ChildProfileReferenceSchema,
  device: ParentDeviceReferenceSchema,
  selectedParentRuleContextRefs: Schema.Array(LocalAiParentRuleContextRefIdSchema),
  selectedParentRuleRefs: Schema.Array(PolicyRuleIdSchema),
  selectedTargetEvidenceRefs: Schema.Array(LocalAiEvidenceContextRefIdSchema),
  selectedEvidenceRefs: Schema.Array(LocalAiEvidenceContextRefIdSchema),
  ungroundedParentRuleReferenceCount: LocalAiContextNonNegativeCountSchema,
  degradedReasons: Schema.Array(LocalAiContextReasonCodeSchema),
  custodyBoundarySummary: LocalAiEvidenceContextSummarySchema,
  validationGateSummary: LocalAiEvidenceContextSummarySchema,
  rawEvidenceRetained: Schema.Literal(false),
  remoteAiUsed: Schema.Literal(false),
  modelExecutionClaimed: Schema.Literal(false),
  modelQualityClaimed: Schema.Literal(false),
  policyAuthorityClaimed: Schema.Literal(false),
  enforcementClaimed: Schema.Literal(false),
  portalUiClaimed: Schema.Literal(false),
});

type LocalAiParentRuleContextBuilderProofRowCandidate = Infer<typeof LocalAiParentRuleContextBuilderProofRowBaseSchema>;

export const LocalAiParentRuleContextBuilderProofRowSchema = withParser(
  LocalAiParentRuleContextBuilderProofRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        localAiParentRuleContextBuilderProofRowIsConsistent(row) ||
        'Expected parent-rule context proof row to preserve selected rule/evidence refs without runtime or enforcement claims'
    )
  )
);

const LocalAiParentRuleContextBuilderProofBaseSchema = Schema.Struct({
  proofId: brandedNonEmptyStringSchema('LocalAiParentRuleContextBuilderProofId'),
  generatedAt: LocalAiTimestampSchema,
  readyRow: LocalAiParentRuleContextBuilderProofRowSchema,
  ungroundedRow: LocalAiParentRuleContextBuilderProofRowSchema,
  validationSummary: Schema.Struct({
    readySelectedParentRuleCount: LocalAiContextNonNegativeCountSchema,
    readySelectedEvidenceCount: LocalAiContextNonNegativeCountSchema,
    ungroundedRejectedParentRuleCount: LocalAiContextNonNegativeCountSchema,
    remoteAiRows: LocalAiContextNonNegativeCountSchema,
    rawEvidenceRetainedRows: LocalAiContextNonNegativeCountSchema,
    policyAuthorityRows: LocalAiContextNonNegativeCountSchema,
    enforcementRows: LocalAiContextNonNegativeCountSchema,
  }),
});

type LocalAiParentRuleContextBuilderProofCandidate = Infer<typeof LocalAiParentRuleContextBuilderProofBaseSchema>;

export const LocalAiParentRuleContextBuilderProofSchema = withParser(
  LocalAiParentRuleContextBuilderProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        localAiParentRuleContextBuilderProofIsReady(proof) ||
        'Expected local AI parent-rule context proof to select grounded rule refs and reject ungrounded refs'
    )
  )
);

export type LocalAiParentRuleContextBuilderProofRow = Infer<typeof LocalAiParentRuleContextBuilderProofRowSchema>;
export type LocalAiParentRuleContextBuilderProof = Infer<typeof LocalAiParentRuleContextBuilderProofSchema>;

export function buildLocalAiParentRuleContextBuilderProofRow(input: unknown): LocalAiParentRuleContextBuilderProofRow {
  const parsed = LocalAiStoredEvidenceContextBuildInputSchema.parse(input);
  const result = buildLocalAiEvidenceContext(parsed);

  return LocalAiParentRuleContextBuilderProofRowSchema.parse({
    state: result.state,
    contextId: result.context?.contextId ?? null,
    requestId: parsed.request.requestId,
    builtAt: parsed.request.requestedAt,
    childProfile: parsed.request.childProfile,
    device: parsed.request.device,
    selectedParentRuleContextRefs: selectedParentRuleContextRefs(result),
    selectedParentRuleRefs: result.context?.parentRuleReferences ?? [],
    selectedTargetEvidenceRefs: selectedTargetEvidenceRefs(result),
    selectedEvidenceRefs: selectedEvidenceRefs(result),
    ungroundedParentRuleReferenceCount: ungroundedParentRuleReferenceCount(parsed, result),
    degradedReasons: degradedReasonsFor(result),
    custodyBoundarySummary: result.custodyBoundarySummary,
    validationGateSummary: result.validationGateSummary,
    rawEvidenceRetained: false,
    remoteAiUsed: false,
    modelExecutionClaimed: false,
    modelQualityClaimed: false,
    policyAuthorityClaimed: false,
    enforcementClaimed: false,
    portalUiClaimed: false,
  });
}

function selectedParentRuleContextRefs(
  result: LocalAiEvidenceContextBuildResult
): LocalAiParentRuleContextBuilderProofRow['selectedParentRuleContextRefs'] {
  return result.context?.parentRuleContextReferences.map((reference) => reference.parentRuleRefId) ?? [];
}

function selectedTargetEvidenceRefs(
  result: LocalAiEvidenceContextBuildResult
): LocalAiParentRuleContextBuilderProofRow['selectedTargetEvidenceRefs'] {
  return result.context?.parentRuleContextReferences.flatMap((reference) => reference.targetEvidenceRefs) ?? [];
}

function selectedEvidenceRefs(
  result: LocalAiEvidenceContextBuildResult
): LocalAiParentRuleContextBuilderProofRow['selectedEvidenceRefs'] {
  return result.context?.evidenceReferences.map((reference) => reference.evidenceRefId) ?? [];
}

function ungroundedParentRuleReferenceCount(
  input: LocalAiStoredEvidenceContextBuildInput,
  result: LocalAiEvidenceContextBuildResult
): number {
  return (
    result.context?.validationSummary.ungroundedParentRuleReferenceCount ??
    input.request.parentRuleContextReferences.length
  );
}

function degradedReasonsFor(result: LocalAiEvidenceContextBuildResult): LocalAiContextReasonCode[] {
  return result.context === null
    ? result.missingEvidenceKinds.map(() => 'missing-evidence' as const)
    : [...result.context.degradedReasons];
}

export function buildLocalAiParentRuleContextBuilderProof(
  readyInput: unknown,
  ungroundedInput: unknown,
  generatedAt: string
): LocalAiParentRuleContextBuilderProof {
  const readyRow = buildLocalAiParentRuleContextBuilderProofRow(readyInput);
  const ungroundedRow = buildLocalAiParentRuleContextBuilderProofRow(ungroundedInput);

  return LocalAiParentRuleContextBuilderProofSchema.parse({
    proofId: 'local-ai-parent-rule-context-builder-proof',
    generatedAt,
    readyRow,
    ungroundedRow,
    validationSummary: {
      readySelectedParentRuleCount: readyRow.selectedParentRuleRefs.length,
      readySelectedEvidenceCount: readyRow.selectedEvidenceRefs.length,
      ungroundedRejectedParentRuleCount: ungroundedRow.ungroundedParentRuleReferenceCount,
      remoteAiRows: [readyRow, ungroundedRow].filter((row) => row.remoteAiUsed).length,
      rawEvidenceRetainedRows: [readyRow, ungroundedRow].filter((row) => row.rawEvidenceRetained).length,
      policyAuthorityRows: [readyRow, ungroundedRow].filter((row) => row.policyAuthorityClaimed).length,
      enforcementRows: [readyRow, ungroundedRow].filter((row) => row.enforcementClaimed).length,
    },
  });
}

function localAiParentRuleContextBuilderProofRowIsConsistent(
  row: LocalAiParentRuleContextBuilderProofRowCandidate
): boolean {
  if (row.state === 'ready') {
    return (
      row.selectedParentRuleRefs.length > 0 &&
      row.selectedParentRuleContextRefs.length === row.selectedParentRuleRefs.length &&
      row.selectedTargetEvidenceRefs.every((referenceId) => row.selectedEvidenceRefs.includes(referenceId))
    );
  }
  return row.ungroundedParentRuleReferenceCount > 0 || row.selectedParentRuleRefs.length > 0;
}

function localAiParentRuleContextBuilderProofIsReady(proof: LocalAiParentRuleContextBuilderProofCandidate): boolean {
  return (
    proof.readyRow.state === 'ready' &&
    proof.validationSummary.readySelectedParentRuleCount > 0 &&
    proof.validationSummary.readySelectedEvidenceCount > 0 &&
    proof.validationSummary.ungroundedRejectedParentRuleCount > 0 &&
    proof.ungroundedRow.degradedReasons.includes('parent-rule-missing') &&
    proof.validationSummary.remoteAiRows === 0 &&
    proof.validationSummary.rawEvidenceRetainedRows === 0 &&
    proof.validationSummary.policyAuthorityRows === 0 &&
    proof.validationSummary.enforcementRows === 0
  );
}
