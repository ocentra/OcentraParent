import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  LocalAiActivityMemoryGraphReadInputSchema,
  LocalAiActivityMemoryGraphReadResultSchema,
  type LocalAiActivityMemoryGraphReadInput,
  type LocalAiActivityMemoryGraphReadResult,
} from './local-ai-activity-memory-graph';
import { readLocalAiActivityMemoryGraph } from './local-ai-activity-memory-graph-read';
import { LocalAiGraphReferenceSchema } from './local-ai-references';
import { ParentContractSchemaVersionSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';

export const LocalAiGraphReferenceContractClaimBoundariesSchema = withParser(
  Schema.Struct({
    remoteAiUsed: Schema.Literal(false),
    apiAiUsed: Schema.Literal(false),
    modelQualityClaimed: Schema.Literal(false),
    policyAuthorityClaimed: Schema.Literal(false),
    enforcementClaimed: Schema.Literal(false),
    uiClaimed: Schema.Literal(false),
    rawEvidenceRetained: Schema.Literal(false),
    uncitedGraphAllowed: Schema.Literal(false),
  })
);

export const LocalAiGraphReferenceContractProofInputSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    graphReadInput: LocalAiActivityMemoryGraphReadInputSchema,
    graphReferences: Schema.Array(LocalAiGraphReferenceSchema),
    claimBoundaries: LocalAiGraphReferenceContractClaimBoundariesSchema,
  })
);

export const LocalAiGraphReferenceContractSummarySchema = withParser(
  Schema.Struct({
    inputGraphReferenceCount: Schema.Number,
    selectedGraphReferenceCount: Schema.Number,
    returnedNodeCount: Schema.Number,
    returnedEdgeCount: Schema.Number,
    omittedEdgeCount: Schema.Number,
    selectedEvidenceReferenceCount: Schema.Number,
  })
);

type LocalAiGraphReferenceContractCandidate = {
  graphReadResult: LocalAiActivityMemoryGraphReadResult;
  selectedGraphReferences: ReadonlyArray<Infer<typeof LocalAiGraphReferenceSchema>>;
  summary: Infer<typeof LocalAiGraphReferenceContractSummarySchema>;
};

export const LocalAiGraphReferenceContractProofSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    proofKind: Schema.Literal('local-ai-graph-reference-contract-proof'),
    graphReadResult: LocalAiActivityMemoryGraphReadResultSchema,
    selectedGraphReferences: Schema.Array(LocalAiGraphReferenceSchema),
    summary: LocalAiGraphReferenceContractSummarySchema,
    claimBoundaries: LocalAiGraphReferenceContractClaimBoundariesSchema,
  }).pipe(
    Schema.filter(
      (proof) =>
        localAiGraphReferenceContractProofIsReady(proof) ||
        'Expected local AI graph proof to return grounded minimal graph edges without overclaiming authority'
    )
  )
);

export type LocalAiGraphReferenceContractClaimBoundaries = Infer<
  typeof LocalAiGraphReferenceContractClaimBoundariesSchema
>;
export type LocalAiGraphReferenceContractProofInput = Infer<typeof LocalAiGraphReferenceContractProofInputSchema>;
export type LocalAiGraphReferenceContractProof = Infer<typeof LocalAiGraphReferenceContractProofSchema>;

function selectedEvidenceIds(input: LocalAiActivityMemoryGraphReadInput): Set<string> {
  return new Set(input.selectedEvidenceReferences.map((reference) => reference.evidenceReferenceId));
}

function graphReferencesSelected(
  input: LocalAiActivityMemoryGraphReadInput,
  graphReferences: readonly Infer<typeof LocalAiGraphReferenceSchema>[]
): Infer<typeof LocalAiGraphReferenceSchema>[] {
  const evidenceIds = selectedEvidenceIds(input);
  return graphReferences.filter((reference) =>
    reference.sourceEvidenceReferences.every(
      (source: Infer<typeof LocalAiGraphReferenceSchema>['sourceEvidenceReferences'][number]) =>
        evidenceIds.has(source.evidenceReferenceId)
    )
  );
}

function localAiGraphReferenceContractProofIsReady(candidate: LocalAiGraphReferenceContractCandidate): boolean {
  return (
    candidate.graphReadResult.returnedEdgeCount > 0 &&
    candidate.graphReadResult.returnedNodeCount >= 2 &&
    candidate.selectedGraphReferences.length > 0 &&
    candidate.summary.selectedGraphReferenceCount === candidate.selectedGraphReferences.length &&
    candidate.summary.returnedEdgeCount === candidate.graphReadResult.returnedEdgeCount &&
    candidate.summary.returnedNodeCount === candidate.graphReadResult.returnedNodeCount
  );
}

export function buildLocalAiGraphReferenceContractProof(input: unknown): LocalAiGraphReferenceContractProof {
  const parsed = LocalAiGraphReferenceContractProofInputSchema.parse(input);
  const graphReadResult = readLocalAiActivityMemoryGraph(parsed.graphReadInput);
  const selectedGraphReferences = graphReferencesSelected(parsed.graphReadInput, parsed.graphReferences);
  return LocalAiGraphReferenceContractProofSchema.parse({
    schemaVersion: parsed.schemaVersion,
    proofKind: 'local-ai-graph-reference-contract-proof',
    graphReadResult,
    selectedGraphReferences,
    summary: {
      inputGraphReferenceCount: parsed.graphReferences.length,
      selectedGraphReferenceCount: selectedGraphReferences.length,
      returnedNodeCount: graphReadResult.returnedNodeCount,
      returnedEdgeCount: graphReadResult.returnedEdgeCount,
      omittedEdgeCount: graphReadResult.omittedEdgeCount,
      selectedEvidenceReferenceCount: parsed.graphReadInput.selectedEvidenceReferences.length,
    },
    claimBoundaries: parsed.claimBoundaries,
  });
}
