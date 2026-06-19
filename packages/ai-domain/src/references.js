import { Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentEvidenceReferenceSchema } from '@ocentra-parent/family-domain/references';
import { ParentPolicyVersionSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import { LocalAiConfidenceSchema, LocalAiContextKindSchema, LocalAiDerivedIndexVersionSchema, LocalAiGraphReferenceIdSchema, LocalAiGraphReferenceKindSchema, LocalAiMemoryReferenceIdSchema, LocalAiMemoryReferenceKindSchema, LocalAiTimestampSchema, } from './primitives';
export const LocalAiObservationReferenceSchema = withParser(Schema.Struct({
    contextKind: LocalAiContextKindSchema,
    evidence: ParentEvidenceReferenceSchema,
}));
export const LocalAiMemoryReferenceSchema = withParser(Schema.Struct({
    memoryReferenceId: LocalAiMemoryReferenceIdSchema,
    kind: LocalAiMemoryReferenceKindSchema,
    sourceEvidenceReferences: Schema.Array(ParentEvidenceReferenceSchema),
    sourcePolicyVersion: Schema.Union(ParentPolicyVersionSchema, Schema.Null),
    generatedAt: LocalAiTimestampSchema,
    confidence: LocalAiConfidenceSchema,
    derivedIndexVersion: LocalAiDerivedIndexVersionSchema,
}).pipe(Schema.filter((reference) => reference.sourceEvidenceReferences.length > 0 || 'Expected local AI memory to cite stored evidence')));
export const LocalAiGraphReferenceSchema = withParser(Schema.Struct({
    graphReferenceId: LocalAiGraphReferenceIdSchema,
    kind: LocalAiGraphReferenceKindSchema,
    sourceEvidenceReferences: Schema.Array(ParentEvidenceReferenceSchema),
    sourcePolicyVersion: Schema.Union(ParentPolicyVersionSchema, Schema.Null),
    generatedAt: LocalAiTimestampSchema,
    confidence: LocalAiConfidenceSchema,
    derivedIndexVersion: LocalAiDerivedIndexVersionSchema,
}).pipe(Schema.filter((reference) => reference.sourceEvidenceReferences.length > 0 || 'Expected local AI graph to cite stored evidence')));
//# sourceMappingURL=references.js.map
