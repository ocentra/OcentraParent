import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentActionReferenceSchema, ParentEvidenceReferenceSchema } from './references';
import { ParentContractSchemaVersionSchema, ParentPolicyVersionSchema } from './reference-primitives';
import {
  LocalAiConfidenceSchema,
  LocalAiDerivedIndexVersionSchema,
  LocalAiTimestampSchema,
} from './local-ai-primitives';
import { LocalAiContextNonNegativeCountSchema, LocalAiContextReasonCodeSchema } from './local-ai-context-primitives';

const LocalAiDerivedKnowledgeText = Schema.String.pipe(Schema.minLength(1));

export const LocalAiDerivedKnowledgeIndexIdSchema = LocalAiDerivedKnowledgeText.pipe(
  Schema.brand('LocalAiDerivedKnowledgeIndexId')
);
export const LocalAiDerivedKnowledgeEntryIdSchema = LocalAiDerivedKnowledgeText.pipe(
  Schema.brand('LocalAiDerivedKnowledgeEntryId')
);

export const LocalAiDerivedKnowledgeIndexKindSchema = withParser(Schema.Literal('memory', 'knowledge-graph', 'hybrid'));

export const LocalAiDerivedKnowledgeIndexStateSchema = withParser(
  Schema.Literal('unavailable', 'building', 'ready', 'degraded', 'stale')
);

export const LocalAiDerivedKnowledgeEntryKindSchema = withParser(
  Schema.Literal('memory-summary', 'memory-pattern', 'graph-entity', 'graph-edge', 'semantic-match')
);

export const LocalAiDerivedKnowledgeEntryStatusSchema = withParser(
  Schema.Literal('candidate', 'usable', 'degraded', 'stale', 'rejected')
);

interface LocalAiDerivedKnowledgeCitationShape {
  readonly sourceEvidenceReferences: readonly unknown[];
  readonly sourcePolicyVersions: readonly unknown[];
  readonly sourceParentActionReferences: readonly unknown[];
}

function hasKnowledgeCitation(citations: LocalAiDerivedKnowledgeCitationShape): boolean {
  return (
    citations.sourceEvidenceReferences.length > 0 ||
    citations.sourcePolicyVersions.length > 0 ||
    citations.sourceParentActionReferences.length > 0
  );
}

export const LocalAiDerivedKnowledgeCitationSetSchema = withParser(
  Schema.Struct({
    sourceEvidenceReferences: Schema.Array(ParentEvidenceReferenceSchema),
    sourcePolicyVersions: Schema.Array(ParentPolicyVersionSchema),
    sourceParentActionReferences: Schema.Array(ParentActionReferenceSchema),
  }).pipe(
    Schema.filter(
      (citations) =>
        hasKnowledgeCitation(citations) ||
        'Expected derived knowledge citations to include stored evidence, policy version, or parent action refs'
    )
  )
);

export const LocalAiDerivedKnowledgeIndexStatusSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    indexId: LocalAiDerivedKnowledgeIndexIdSchema,
    indexKind: LocalAiDerivedKnowledgeIndexKindSchema,
    indexVersion: LocalAiDerivedIndexVersionSchema,
    state: LocalAiDerivedKnowledgeIndexStateSchema,
    generatedAt: Schema.Union(LocalAiTimestampSchema, Schema.Null),
    refreshedAt: Schema.Union(LocalAiTimestampSchema, Schema.Null),
    entryCount: LocalAiContextNonNegativeCountSchema,
    usableEntryCount: LocalAiContextNonNegativeCountSchema,
    sourceEvidenceCitationCount: LocalAiContextNonNegativeCountSchema,
    sourcePolicyVersionCitationCount: LocalAiContextNonNegativeCountSchema,
    sourceParentActionCitationCount: LocalAiContextNonNegativeCountSchema,
    degradedReasons: Schema.Array(LocalAiContextReasonCodeSchema),
  }).pipe(
    Schema.filter(
      (status) =>
        status.usableEntryCount <= status.entryCount ||
        'Expected derived knowledge usable entry count to be less than or equal to entry count'
    )
  )
);

export const LocalAiDerivedKnowledgeEntrySchema = withParser(
  Schema.Struct({
    entryId: LocalAiDerivedKnowledgeEntryIdSchema,
    indexId: LocalAiDerivedKnowledgeIndexIdSchema,
    indexKind: LocalAiDerivedKnowledgeIndexKindSchema,
    entryKind: LocalAiDerivedKnowledgeEntryKindSchema,
    entryStatus: LocalAiDerivedKnowledgeEntryStatusSchema,
    derivedIndexVersion: LocalAiDerivedIndexVersionSchema,
    generatedAt: LocalAiTimestampSchema,
    expiresAt: Schema.Union(LocalAiTimestampSchema, Schema.Null),
    confidence: LocalAiConfidenceSchema,
    citations: LocalAiDerivedKnowledgeCitationSetSchema,
    degradedReasons: Schema.Array(LocalAiContextReasonCodeSchema),
  })
);

export type LocalAiDerivedKnowledgeIndexId = typeof LocalAiDerivedKnowledgeIndexIdSchema.Type;
export type LocalAiDerivedKnowledgeEntryId = typeof LocalAiDerivedKnowledgeEntryIdSchema.Type;
export type LocalAiDerivedKnowledgeIndexKind = Infer<typeof LocalAiDerivedKnowledgeIndexKindSchema>;
export type LocalAiDerivedKnowledgeIndexState = Infer<typeof LocalAiDerivedKnowledgeIndexStateSchema>;
export type LocalAiDerivedKnowledgeEntryKind = Infer<typeof LocalAiDerivedKnowledgeEntryKindSchema>;
export type LocalAiDerivedKnowledgeEntryStatus = Infer<typeof LocalAiDerivedKnowledgeEntryStatusSchema>;
export type LocalAiDerivedKnowledgeCitationSet = Infer<typeof LocalAiDerivedKnowledgeCitationSetSchema>;
export type LocalAiDerivedKnowledgeIndexStatus = Infer<typeof LocalAiDerivedKnowledgeIndexStatusSchema>;
export type LocalAiDerivedKnowledgeEntry = Infer<typeof LocalAiDerivedKnowledgeEntrySchema>;

export const LocalAiDerivedKnowledgeIndexState = {
  Unavailable: LocalAiDerivedKnowledgeIndexStateSchema.parse('unavailable'),
  Building: LocalAiDerivedKnowledgeIndexStateSchema.parse('building'),
  Ready: LocalAiDerivedKnowledgeIndexStateSchema.parse('ready'),
  Degraded: LocalAiDerivedKnowledgeIndexStateSchema.parse('degraded'),
  Stale: LocalAiDerivedKnowledgeIndexStateSchema.parse('stale'),
} as const;

export const LocalAiDerivedKnowledgeEntryStatus = {
  Candidate: LocalAiDerivedKnowledgeEntryStatusSchema.parse('candidate'),
  Usable: LocalAiDerivedKnowledgeEntryStatusSchema.parse('usable'),
  Degraded: LocalAiDerivedKnowledgeEntryStatusSchema.parse('degraded'),
  Stale: LocalAiDerivedKnowledgeEntryStatusSchema.parse('stale'),
  Rejected: LocalAiDerivedKnowledgeEntryStatusSchema.parse('rejected'),
} as const;
