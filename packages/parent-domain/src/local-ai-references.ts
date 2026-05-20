import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentEvidenceReferenceSchema } from './references';
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
    generatedAt: LocalAiTimestampSchema,
    confidence: LocalAiConfidenceSchema,
    derivedIndexVersion: LocalAiDerivedIndexVersionSchema,
  })
);

export const LocalAiGraphReferenceSchema = withParser(
  Schema.Struct({
    graphReferenceId: LocalAiGraphReferenceIdSchema,
    kind: LocalAiGraphReferenceKindSchema,
    sourceEvidenceReferences: Schema.Array(ParentEvidenceReferenceSchema),
    sourcePolicyVersion: Schema.Union(ParentPolicyVersionSchema, Schema.Null),
    generatedAt: LocalAiTimestampSchema,
    confidence: LocalAiConfidenceSchema,
    derivedIndexVersion: LocalAiDerivedIndexVersionSchema,
  })
);

export type LocalAiObservationReference = Infer<typeof LocalAiObservationReferenceSchema>;
export type LocalAiMemoryReference = Infer<typeof LocalAiMemoryReferenceSchema>;
export type LocalAiGraphReference = Infer<typeof LocalAiGraphReferenceSchema>;
