import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  LocalAiActivityMemoryGraphReadResultSchema,
  type LocalAiActivityMemoryGraphReadResult,
} from './local-ai-activity-memory-graph';
import { PolicyActionSchema, PolicyDecisionIdSchema } from './policy';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from './reference-primitives';
import { ParentActionReferenceSchema, ParentEvidenceReferenceSchema } from './references';

const NonEmptyMemoryProofText = Schema.String.pipe(Schema.minLength(1));

export const ScreenAiMemoryGraphSourceProofIdSchema = NonEmptyMemoryProofText.pipe(
  Schema.brand('ScreenAiMemoryGraphSourceProofId')
);
export const ScreenAiMemoryGraphSourceArtifactRefSchema = NonEmptyMemoryProofText.pipe(
  Schema.brand('ScreenAiMemoryGraphSourceArtifactRef')
);
export const ScreenAiMemoryGraphSourceAssertionLabelSchema = NonEmptyMemoryProofText.pipe(
  Schema.brand('ScreenAiMemoryGraphSourceAssertionLabel')
);

export const ScreenAiMemoryGraphSourceCustodySchema = withParser(
  Schema.Struct({
    sourceImageDeletionState: Schema.Literal('deleted'),
    rawImageRetained: Schema.Literal(false),
    custodyState: Schema.Literal('child-device-journal'),
  })
);

export const ScreenAiMemoryGraphSourceAssertionSchema = withParser(
  Schema.Struct({
    sourceUsedRealServiceOcrPolicyArtifact: Schema.Boolean,
    graphReadUsedRealMemoryReader: Schema.Boolean,
    graphEdgesCiteSelectedEvidence: Schema.Boolean,
    graphEdgesCiteSelectedPolicy: Schema.Boolean,
    graphEdgesCiteSelectedAction: Schema.Boolean,
    rawImageNotRetained: Schema.Boolean,
    deletedImageCustodyPreserved: Schema.Boolean,
    remoteAiNotIntroduced: Schema.Boolean,
  }).pipe(
    Schema.filter(
      (assertions) =>
        Object.values(assertions).every((assertion) => assertion === true) ||
        'Expected screen AI memory graph proof assertions to stay true'
    )
  )
);

export const ScreenAiMemoryGraphSourceProofSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    proofId: ScreenAiMemoryGraphSourceProofIdSchema,
    generatedAt: ParentTimestampSchema,
    sourceProofArtifact: ScreenAiMemoryGraphSourceArtifactRefSchema,
    sourcePolicyReadModelArtifact: ScreenAiMemoryGraphSourceArtifactRefSchema,
    sourcePolicyDecisionId: PolicyDecisionIdSchema,
    sourcePolicyAction: PolicyActionSchema,
    sourceEvidenceReferences: Schema.Array(ParentEvidenceReferenceSchema),
    sourceParentActionReferences: Schema.Array(ParentActionReferenceSchema),
    sourceCustody: ScreenAiMemoryGraphSourceCustodySchema,
    memoryGraphRead: LocalAiActivityMemoryGraphReadResultSchema,
    assertionLabels: Schema.Array(ScreenAiMemoryGraphSourceAssertionLabelSchema),
    assertions: ScreenAiMemoryGraphSourceAssertionSchema,
  }).pipe(
    Schema.filter(
      (proof) =>
        screenAiMemoryGraphSourceProofIsReady(proof) ||
        'Expected screen AI memory graph proof to cite selected source evidence, policy, action, and deleted custody'
    )
  )
);

export type ScreenAiMemoryGraphSourceProof = Infer<typeof ScreenAiMemoryGraphSourceProofSchema>;

export function screenAiMemoryGraphSourceProofIsReady(proof: {
  readonly sourceEvidenceReferences: readonly { readonly evidenceReferenceId: unknown }[];
  readonly sourceParentActionReferences: readonly { readonly actionReferenceId: unknown }[];
  readonly sourceCustody: { readonly sourceImageDeletionState: unknown; readonly rawImageRetained: unknown };
  readonly memoryGraphRead: LocalAiActivityMemoryGraphReadResult;
}): boolean {
  return (
    proof.sourceEvidenceReferences.length > 0 &&
    proof.sourceParentActionReferences.length > 0 &&
    proof.sourceCustody.sourceImageDeletionState === 'deleted' &&
    proof.sourceCustody.rawImageRetained === false &&
    proof.memoryGraphRead.nodes.length > 0 &&
    proof.memoryGraphRead.edges.length > 0 &&
    memoryEdgesCiteSelectedSources(proof)
  );
}

export function summarizeScreenAiMemoryGraphSourceProof(proof: ScreenAiMemoryGraphSourceProof) {
  return {
    nodeCount: proof.memoryGraphRead.returnedNodeCount,
    edgeCount: proof.memoryGraphRead.returnedEdgeCount,
    sourceEvidenceReferenceCount: proof.sourceEvidenceReferences.length,
    sourceParentActionReferenceCount: proof.sourceParentActionReferences.length,
    rawImageRetained: proof.sourceCustody.rawImageRetained,
    imageDeletionState: proof.sourceCustody.sourceImageDeletionState,
    assertionCount: proof.assertionLabels.length,
  };
}

function memoryEdgesCiteSelectedSources(proof: {
  readonly sourceEvidenceReferences: readonly { readonly evidenceReferenceId: unknown }[];
  readonly sourceParentActionReferences: readonly { readonly actionReferenceId: unknown }[];
  readonly memoryGraphRead: LocalAiActivityMemoryGraphReadResult;
}): boolean {
  const evidenceIds = new Set(proof.sourceEvidenceReferences.map((reference) => reference.evidenceReferenceId));
  const actionIds = new Set(proof.sourceParentActionReferences.map((reference) => reference.actionReferenceId));
  return proof.memoryGraphRead.edges.every(
    (edge) =>
      edge.trace.sourceEvidenceReferences.length > 0 &&
      edge.trace.sourceEvidenceReferences.every((reference) => evidenceIds.has(reference.evidenceReferenceId)) &&
      edge.trace.sourcePolicyVersion !== null &&
      edge.trace.sourceParentActionReferences.length > 0 &&
      edge.trace.sourceParentActionReferences.every((reference) => actionIds.has(reference.actionReferenceId))
  );
}
