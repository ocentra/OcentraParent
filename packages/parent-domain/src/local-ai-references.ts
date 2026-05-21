import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentActionReferenceSchema, ParentEvidenceReferenceSchema } from './references';
import { ParentPolicyVersionSchema } from './reference-primitives';
import {
  LocalAiContextKindSchema,
  LocalAiConfidenceSchema,
  LocalAiDerivedIndexVersionSchema,
  LocalAiGraphReferenceIdSchema,
  LocalAiGraphReferenceKindSchema,
  LocalAiMemoryReferenceIdSchema,
  LocalAiMemoryReferenceKindSchema,
  LocalAiTimestampSchema,
} from './local-ai-primitives';

interface LocalAiDerivedSourceCitation {
  readonly sourceEvidenceReferences: readonly unknown[];
  readonly sourcePolicyVersion: unknown | null;
  readonly sourceParentActionReferences: readonly unknown[];
}

function hasDerivedSourceCitation(reference: LocalAiDerivedSourceCitation): boolean {
  return (
    reference.sourceEvidenceReferences.length > 0 ||
    reference.sourcePolicyVersion !== null ||
    reference.sourceParentActionReferences.length > 0
  );
}

export const LocalAiObservationReferenceSchema = withParser(
  Schema.Struct({
    contextKind: LocalAiContextKindSchema,
    evidence: ParentEvidenceReferenceSchema,
  })
);

export const LocalAiMemoryReferenceSchema = withParser(
  Schema.Struct({
    memoryReferenceId: LocalAiMemoryReferenceIdSchema,
    kind: LocalAiMemoryReferenceKindSchema,
    sourceEvidenceReferences: Schema.Array(ParentEvidenceReferenceSchema),
    sourcePolicyVersion: Schema.Union(ParentPolicyVersionSchema, Schema.Null),
    sourceParentActionReferences: Schema.Array(ParentActionReferenceSchema),
    generatedAt: LocalAiTimestampSchema,
    confidence: LocalAiConfidenceSchema,
    derivedIndexVersion: LocalAiDerivedIndexVersionSchema,
  }).pipe(
    Schema.filter(
      (reference) =>
        hasDerivedSourceCitation(reference) ||
        'Expected local AI memory to cite stored evidence, policy version, or parent action'
    )
  )
);

export const LocalAiGraphReferenceSchema = withParser(
  Schema.Struct({
    graphReferenceId: LocalAiGraphReferenceIdSchema,
    kind: LocalAiGraphReferenceKindSchema,
    sourceEvidenceReferences: Schema.Array(ParentEvidenceReferenceSchema),
    sourcePolicyVersion: Schema.Union(ParentPolicyVersionSchema, Schema.Null),
    sourceParentActionReferences: Schema.Array(ParentActionReferenceSchema),
    generatedAt: LocalAiTimestampSchema,
    confidence: LocalAiConfidenceSchema,
    derivedIndexVersion: LocalAiDerivedIndexVersionSchema,
  }).pipe(
    Schema.filter(
      (reference) =>
        hasDerivedSourceCitation(reference) ||
        'Expected local AI graph to cite stored evidence, policy version, or parent action'
    )
  )
);

export type LocalAiObservationReference = Infer<typeof LocalAiObservationReferenceSchema>;
export type LocalAiMemoryReference = Infer<typeof LocalAiMemoryReferenceSchema>;
export type LocalAiGraphReference = Infer<typeof LocalAiGraphReferenceSchema>;
