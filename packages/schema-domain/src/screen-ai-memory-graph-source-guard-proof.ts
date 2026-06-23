import { type Infer, Schema, withParser } from './effect';
import {
  LocalAiEvidenceContextBuildResultSchema,
  LocalAiStoredEvidenceContextBuildInputSchema,
  type LocalAiEvidenceContext,
  type LocalAiEvidenceContextBuildResult,
  type LocalAiStoredEvidenceContextBuildInput,
} from './ai-context';
import { buildLocalAiEvidenceContext } from './local-ai-context-builder';
import { ParentContractSchemaVersionSchema } from './family-reference-primitives';

export const ScreenAiMemoryGraphSourceGuardClaimBoundariesSchema = withParser(
  Schema.Struct({
    remoteAiUsed: Schema.Literal(false),
    apiAiUsed: Schema.Literal(false),
    policyAuthorityClaimed: Schema.Literal(false),
    enforcementClaimed: Schema.Literal(false),
    uncitedMemoryAllowed: Schema.Literal(false),
    uncitedGraphAllowed: Schema.Literal(false),
    rawEvidenceEmbedded: Schema.Literal(false),
  })
);

export const ScreenAiMemoryGraphSourceGuardInputSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    contextInput: LocalAiStoredEvidenceContextBuildInputSchema,
    claimBoundaries: ScreenAiMemoryGraphSourceGuardClaimBoundariesSchema,
  })
);

const ScreenAiMemoryGraphSourceGuardSummarySchema = withParser(
  Schema.Struct({
    evidenceReferenceCount: Schema.Number,
    sourceEvidenceReferenceCount: Schema.Number,
    memoryReferenceCount: Schema.Number,
    graphReferenceCount: Schema.Number,
    rejectedUncitedMemoryReferenceCount: Schema.Number,
    rejectedUncitedGraphReferenceCount: Schema.Number,
  })
);

type ScreenAiMemoryGraphSourceGuardProofCandidate = {
  contextResult: LocalAiEvidenceContextBuildResult;
  sourceGuardSummary: Infer<typeof ScreenAiMemoryGraphSourceGuardSummarySchema>;
};

export const ScreenAiMemoryGraphSourceGuardProofSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    proofKind: Schema.Literal('screen-ai-memory-graph-source-guard-proof'),
    contextResult: LocalAiEvidenceContextBuildResultSchema,
    sourceGuardSummary: ScreenAiMemoryGraphSourceGuardSummarySchema,
    claimBoundaries: ScreenAiMemoryGraphSourceGuardClaimBoundariesSchema,
  }).pipe(
    Schema.filter(
      (proof) =>
        screenAiMemoryGraphSourceGuardProofIsReady(proof) ||
        'Expected screen AI memory/graph proof to cite stored screen evidence and reject uncited derived context'
    )
  )
);

export type ScreenAiMemoryGraphSourceGuardClaimBoundaries = Infer<
  typeof ScreenAiMemoryGraphSourceGuardClaimBoundariesSchema
>;
export type ScreenAiMemoryGraphSourceGuardInput = Infer<typeof ScreenAiMemoryGraphSourceGuardInputSchema>;
export type ScreenAiMemoryGraphSourceGuardProof = Infer<typeof ScreenAiMemoryGraphSourceGuardProofSchema>;

function sourceEvidenceIds(context: LocalAiEvidenceContext): Set<string> {
  const evidenceIds = new Set<string>();
  for (const reference of context.evidenceReferences) {
    evidenceIds.add(reference.evidenceRefId);
    evidenceIds.add(reference.evidence.evidenceReferenceId);
    for (const sourceReference of reference.sourceEvidenceReferences) {
      evidenceIds.add(sourceReference.evidenceReferenceId);
    }
  }
  return evidenceIds;
}

function referenceSourcesAreGrounded(
  sourceReferences: readonly { evidenceReferenceId: string }[],
  evidenceIds: ReadonlySet<string>
): boolean {
  return sourceReferences.every((reference) => evidenceIds.has(reference.evidenceReferenceId));
}

function contextMemoryGraphSourcesAreGrounded(context: LocalAiEvidenceContext): boolean {
  const evidenceIds = sourceEvidenceIds(context);
  return (
    context.memoryReferences.every((reference) =>
      referenceSourcesAreGrounded(reference.sourceEvidenceReferences, evidenceIds)
    ) &&
    context.graphReferences.every((reference) =>
      referenceSourcesAreGrounded(reference.sourceEvidenceReferences, evidenceIds)
    )
  );
}

function countRejectedMemoryReferences(
  input: LocalAiStoredEvidenceContextBuildInput,
  context: LocalAiEvidenceContext
): number {
  return input.memoryReferences.length - context.memoryReferences.length;
}

function countRejectedGraphReferences(
  input: LocalAiStoredEvidenceContextBuildInput,
  context: LocalAiEvidenceContext
): number {
  return input.graphReferences.length - context.graphReferences.length;
}

function screenAiMemoryGraphSourceGuardProofIsReady(proof: ScreenAiMemoryGraphSourceGuardProofCandidate): boolean {
  const context = proof.contextResult.context;
  return (
    proof.contextResult.state === 'ready' &&
    context !== null &&
    context.screenSummaryRefs.length > 0 &&
    context.memoryReferences.length > 0 &&
    context.graphReferences.length > 0 &&
    proof.sourceGuardSummary.memoryReferenceCount === context.memoryReferences.length &&
    proof.sourceGuardSummary.graphReferenceCount === context.graphReferences.length &&
    proof.sourceGuardSummary.rejectedUncitedMemoryReferenceCount === 0 &&
    proof.sourceGuardSummary.rejectedUncitedGraphReferenceCount === 0 &&
    !context.degradedReasons.includes('memory-ungrounded') &&
    !context.degradedReasons.includes('graph-ungrounded') &&
    contextMemoryGraphSourcesAreGrounded(context)
  );
}

export function buildScreenAiMemoryGraphSourceGuardProof(input: unknown): ScreenAiMemoryGraphSourceGuardProof {
  const parsed = ScreenAiMemoryGraphSourceGuardInputSchema.parse(input);
  const contextResult = LocalAiEvidenceContextBuildResultSchema.parse(buildLocalAiEvidenceContext(parsed.contextInput));
  const context = contextResult.context;
  const sourceGuardSummary = {
    evidenceReferenceCount: context?.validationSummary.evidenceReferenceCount ?? 0,
    sourceEvidenceReferenceCount: context?.validationSummary.sourceEvidenceReferenceCount ?? 0,
    memoryReferenceCount: context?.validationSummary.memoryReferenceCount ?? 0,
    graphReferenceCount: context?.validationSummary.graphReferenceCount ?? 0,
    rejectedUncitedMemoryReferenceCount:
      context === null
        ? parsed.contextInput.memoryReferences.length
        : countRejectedMemoryReferences(parsed.contextInput, context),
    rejectedUncitedGraphReferenceCount:
      context === null
        ? parsed.contextInput.graphReferences.length
        : countRejectedGraphReferences(parsed.contextInput, context),
  };
  return ScreenAiMemoryGraphSourceGuardProofSchema.parse({
    schemaVersion: parsed.schemaVersion,
    proofKind: 'screen-ai-memory-graph-source-guard-proof',
    contextResult,
    sourceGuardSummary,
    claimBoundaries: parsed.claimBoundaries,
  });
}
