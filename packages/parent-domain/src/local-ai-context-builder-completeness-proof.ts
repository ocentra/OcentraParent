import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  LocalAiEvidenceContextBuildResultSchema,
  LocalAiStoredEvidenceContextBuildInputSchema,
  type LocalAiEvidenceContextBuildResult,
} from './local-ai-context';
import { buildLocalAiEvidenceContext } from './local-ai-context-builder';
import { ParentContractSchemaVersionSchema } from './reference-primitives';

const LocalAiContextBuilderCompletenessCountSchema = Schema.Number.pipe(Schema.nonNegative(), Schema.int());

export const LocalAiContextBuilderCompletenessClaimBoundariesSchema = withParser(
  Schema.Struct({
    modelExecutionClaimed: Schema.Literal(false),
    modelQualityClaimed: Schema.Literal(false),
    policyAuthorityClaimed: Schema.Literal(false),
    enforcementClaimed: Schema.Literal(false),
    portalUiClaimed: Schema.Literal(false),
    remoteApiAiUsed: Schema.Literal(false),
    rawPromptRetained: Schema.Literal(false),
    rawEvidenceRetained: Schema.Literal(false),
  })
);

export const LocalAiContextBuilderCompletenessProofInputSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    readyInput: LocalAiStoredEvidenceContextBuildInputSchema,
    partialInput: LocalAiStoredEvidenceContextBuildInputSchema,
    forbiddenCustodyInput: LocalAiStoredEvidenceContextBuildInputSchema,
    unallowedCustodyInput: LocalAiStoredEvidenceContextBuildInputSchema,
    unavailableRuntimeInput: LocalAiStoredEvidenceContextBuildInputSchema,
    claimBoundaries: LocalAiContextBuilderCompletenessClaimBoundariesSchema,
  })
);

export const LocalAiContextBuilderCompletenessSummarySchema = withParser(
  Schema.Struct({
    readyEvidenceReferenceCount: LocalAiContextBuilderCompletenessCountSchema,
    readyRuntimeReferenceCount: LocalAiContextBuilderCompletenessCountSchema,
    readyParentRuleReferenceCount: LocalAiContextBuilderCompletenessCountSchema,
    readyMemoryReferenceCount: LocalAiContextBuilderCompletenessCountSchema,
    readyGraphReferenceCount: LocalAiContextBuilderCompletenessCountSchema,
    partialMissingEvidenceKindCount: LocalAiContextBuilderCompletenessCountSchema,
    rejectedForbiddenCustodyCount: LocalAiContextBuilderCompletenessCountSchema,
    rejectedUnallowedCustodyCount: LocalAiContextBuilderCompletenessCountSchema,
    unavailableRuntimeDegradedCount: LocalAiContextBuilderCompletenessCountSchema,
  })
);

const LocalAiContextBuilderCompletenessProofBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  proofKind: Schema.Literal('local-ai-context-builder-completeness-proof'),
  readyResult: LocalAiEvidenceContextBuildResultSchema,
  partialResult: LocalAiEvidenceContextBuildResultSchema,
  forbiddenCustodyResult: LocalAiEvidenceContextBuildResultSchema,
  unallowedCustodyResult: LocalAiEvidenceContextBuildResultSchema,
  unavailableRuntimeResult: LocalAiEvidenceContextBuildResultSchema,
  summary: LocalAiContextBuilderCompletenessSummarySchema,
  claimBoundaries: LocalAiContextBuilderCompletenessClaimBoundariesSchema,
});

type LocalAiContextBuilderCompletenessProofCandidate = Infer<typeof LocalAiContextBuilderCompletenessProofBaseSchema>;

export const LocalAiContextBuilderCompletenessProofSchema = withParser(
  LocalAiContextBuilderCompletenessProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        localAiContextBuilderCompletenessProofIsReady(proof) ||
        'Expected local AI context builder completeness proof to cover ready, partial, rejected, and degraded local-only states'
    )
  )
);

export type LocalAiContextBuilderCompletenessClaimBoundaries = Infer<
  typeof LocalAiContextBuilderCompletenessClaimBoundariesSchema
>;
export type LocalAiContextBuilderCompletenessProofInput = Infer<
  typeof LocalAiContextBuilderCompletenessProofInputSchema
>;
export type LocalAiContextBuilderCompletenessProof = Infer<typeof LocalAiContextBuilderCompletenessProofSchema>;

function localAiContextBuilderCompletenessProofIsReady(
  proof: LocalAiContextBuilderCompletenessProofCandidate
): boolean {
  return [
    readyContextIsComplete(proof),
    partialContextIsTyped(proof),
    rejectedCustodyStatesAreTyped(proof),
    unavailableRuntimeStateIsTyped(proof),
  ].every(Boolean);
}

function readyContextIsComplete(proof: LocalAiContextBuilderCompletenessProofCandidate): boolean {
  const readyContext = proof.readyResult.context;
  if (proof.readyResult.state !== 'ready' || readyContext === null) {
    return false;
  }
  return [
    proof.summary.readyEvidenceReferenceCount === readyContext.evidenceReferences.length,
    proof.summary.readyRuntimeReferenceCount === readyContext.localModelRuntimeRefs.length,
    proof.summary.readyParentRuleReferenceCount === readyContext.parentRuleReferences.length,
    proof.summary.readyMemoryReferenceCount === readyContext.memoryReferences.length,
    proof.summary.readyGraphReferenceCount === readyContext.graphReferences.length,
  ].every(Boolean);
}

function partialContextIsTyped(proof: LocalAiContextBuilderCompletenessProofCandidate): boolean {
  return proof.partialResult.state === 'partial' && proof.partialResult.missingEvidenceKinds.length > 0;
}

function rejectedCustodyStatesAreTyped(proof: LocalAiContextBuilderCompletenessProofCandidate): boolean {
  return (
    proof.forbiddenCustodyResult.state === 'rejected' &&
    proof.forbiddenCustodyResult.degradedSourceRefs.length > 0 &&
    proof.unallowedCustodyResult.state === 'rejected' &&
    proof.unallowedCustodyResult.rejectedFields.length > 0
  );
}

function unavailableRuntimeStateIsTyped(proof: LocalAiContextBuilderCompletenessProofCandidate): boolean {
  return (
    proof.unavailableRuntimeResult.state === 'partial' &&
    proof.unavailableRuntimeResult.context?.degradedReasons.includes('model-unavailable') === true
  );
}

function count<T>(values: readonly T[] | undefined): number {
  return values?.length ?? 0;
}

function unavailableRuntimeDegradedCount(result: LocalAiEvidenceContextBuildResult): number {
  return count(result.context?.degradedReasons.filter((reason) => reason === 'model-unavailable'));
}

function summaryFor(results: {
  readyResult: LocalAiEvidenceContextBuildResult;
  partialResult: LocalAiEvidenceContextBuildResult;
  forbiddenCustodyResult: LocalAiEvidenceContextBuildResult;
  unallowedCustodyResult: LocalAiEvidenceContextBuildResult;
  unavailableRuntimeResult: LocalAiEvidenceContextBuildResult;
}): Infer<typeof LocalAiContextBuilderCompletenessSummarySchema> {
  const readyContext = results.readyResult.context;
  return {
    readyEvidenceReferenceCount: count(readyContext?.evidenceReferences),
    readyRuntimeReferenceCount: count(readyContext?.localModelRuntimeRefs),
    readyParentRuleReferenceCount: count(readyContext?.parentRuleReferences),
    readyMemoryReferenceCount: count(readyContext?.memoryReferences),
    readyGraphReferenceCount: count(readyContext?.graphReferences),
    partialMissingEvidenceKindCount: results.partialResult.missingEvidenceKinds.length,
    rejectedForbiddenCustodyCount: results.forbiddenCustodyResult.degradedSourceRefs.length,
    rejectedUnallowedCustodyCount: results.unallowedCustodyResult.rejectedFields.length,
    unavailableRuntimeDegradedCount: unavailableRuntimeDegradedCount(results.unavailableRuntimeResult),
  };
}

export function buildLocalAiContextBuilderCompletenessProof(input: unknown): LocalAiContextBuilderCompletenessProof {
  const parsed = LocalAiContextBuilderCompletenessProofInputSchema.parse(input);
  const results = {
    readyResult: buildLocalAiEvidenceContext(parsed.readyInput),
    partialResult: buildLocalAiEvidenceContext(parsed.partialInput),
    forbiddenCustodyResult: buildLocalAiEvidenceContext(parsed.forbiddenCustodyInput),
    unallowedCustodyResult: buildLocalAiEvidenceContext(parsed.unallowedCustodyInput),
    unavailableRuntimeResult: buildLocalAiEvidenceContext(parsed.unavailableRuntimeInput),
  };
  return LocalAiContextBuilderCompletenessProofSchema.parse({
    schemaVersion: parsed.schemaVersion,
    proofKind: 'local-ai-context-builder-completeness-proof',
    ...results,
    summary: summaryFor(results),
    claimBoundaries: parsed.claimBoundaries,
  });
}
