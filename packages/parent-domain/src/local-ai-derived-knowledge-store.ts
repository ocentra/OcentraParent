import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentActionReferenceSchema, ParentEvidenceReferenceSchema } from './references';
import { ParentPolicyVersionSchema } from './reference-primitives';
import { LocalAiContextNonNegativeCountSchema, LocalAiContextReasonCodeSchema } from './local-ai-context-primitives';
import { LocalAiTimestampSchema } from './local-ai-primitives';
import {
  type LocalAiDerivedKnowledgeEntry,
  LocalAiDerivedKnowledgeEntrySchema,
  type LocalAiDerivedKnowledgeIndexState,
  LocalAiDerivedKnowledgeIndexStatusSchema,
} from './local-ai-derived-knowledge';
import { selectUsableDerivedKnowledgeEntries } from './local-ai-derived-knowledge-selection';

interface LocalAiDerivedKnowledgeStoreSnapshotShape {
  readonly status: typeof LocalAiDerivedKnowledgeIndexStatusSchema.Type;
  readonly entries: readonly LocalAiDerivedKnowledgeEntry[];
}

function citationCountFor(
  entries: readonly LocalAiDerivedKnowledgeEntry[],
  citationKey: keyof LocalAiDerivedKnowledgeEntry['citations']
): number {
  return entries.reduce((count, entry) => count + entry.citations[citationKey].length, 0);
}

function usableStatusCount(entries: readonly LocalAiDerivedKnowledgeEntry[]): number {
  return entries.filter((entry) => entry.entryStatus === 'usable').length;
}

function entriesMatchStatusIdentity(snapshot: LocalAiDerivedKnowledgeStoreSnapshotShape): boolean {
  return snapshot.entries.every(
    (entry) =>
      entry.indexId === snapshot.status.indexId &&
      entry.indexKind === snapshot.status.indexKind &&
      entry.derivedIndexVersion === snapshot.status.indexVersion
  );
}

function statusCountsMatchEntries(snapshot: LocalAiDerivedKnowledgeStoreSnapshotShape): boolean {
  return (
    snapshot.status.entryCount === snapshot.entries.length &&
    snapshot.status.usableEntryCount === usableStatusCount(snapshot.entries) &&
    snapshot.status.sourceEvidenceCitationCount === citationCountFor(snapshot.entries, 'sourceEvidenceReferences') &&
    snapshot.status.sourcePolicyVersionCitationCount === citationCountFor(snapshot.entries, 'sourcePolicyVersions') &&
    snapshot.status.sourceParentActionCitationCount ===
      citationCountFor(snapshot.entries, 'sourceParentActionReferences')
  );
}

function snapshotMatchesStatus(snapshot: LocalAiDerivedKnowledgeStoreSnapshotShape): boolean {
  return entriesMatchStatusIdentity(snapshot) && statusCountsMatchEntries(snapshot);
}

export const LocalAiDerivedKnowledgeStoreSnapshotSchema = withParser(
  Schema.Struct({
    status: LocalAiDerivedKnowledgeIndexStatusSchema,
    entries: Schema.Array(LocalAiDerivedKnowledgeEntrySchema),
  }).pipe(
    Schema.filter(
      (snapshot) =>
        snapshotMatchesStatus(snapshot) ||
        'Expected derived knowledge store snapshot rows to match index identity and status counts'
    )
  )
);

export const LocalAiDerivedKnowledgeStoreReadInputSchema = withParser(
  Schema.Struct({
    snapshot: LocalAiDerivedKnowledgeStoreSnapshotSchema,
    selectedEvidenceReferences: Schema.Array(ParentEvidenceReferenceSchema),
    selectedPolicyVersions: Schema.Array(ParentPolicyVersionSchema),
    selectedParentActionReferences: Schema.Array(ParentActionReferenceSchema),
    asOf: LocalAiTimestampSchema,
  })
);

export const LocalAiDerivedKnowledgeStoreReadResultSchema = withParser(
  Schema.Struct({
    status: LocalAiDerivedKnowledgeIndexStatusSchema,
    readAt: LocalAiTimestampSchema,
    usableEntries: Schema.Array(LocalAiDerivedKnowledgeEntrySchema),
    returnedEntryCount: LocalAiContextNonNegativeCountSchema,
    excludedEntryCount: LocalAiContextNonNegativeCountSchema,
    degradedReasons: Schema.Array(LocalAiContextReasonCodeSchema),
  }).pipe(
    Schema.filter(
      (result) =>
        result.returnedEntryCount === result.usableEntries.length ||
        'Expected derived knowledge read result count to match returned entries'
    )
  )
);

export type LocalAiDerivedKnowledgeStoreSnapshot = Infer<typeof LocalAiDerivedKnowledgeStoreSnapshotSchema>;
export type LocalAiDerivedKnowledgeStoreReadInput = Infer<typeof LocalAiDerivedKnowledgeStoreReadInputSchema>;
export type LocalAiDerivedKnowledgeStoreReadResult = Infer<typeof LocalAiDerivedKnowledgeStoreReadResultSchema>;

function isReadableIndexState(state: LocalAiDerivedKnowledgeIndexState): boolean {
  return state === 'ready' || state === 'degraded';
}

function readDegradedReasons(state: LocalAiDerivedKnowledgeIndexState): readonly string[] {
  if (state === 'stale') {
    return ['stale-evidence'];
  }
  if (state === 'unavailable') {
    return ['missing-evidence'];
  }
  return [];
}

function selectedUsableEntries(input: LocalAiDerivedKnowledgeStoreReadInput): LocalAiDerivedKnowledgeEntry[] {
  if (!isReadableIndexState(input.snapshot.status.state)) {
    return [];
  }
  return selectUsableDerivedKnowledgeEntries({
    entries: input.snapshot.entries,
    selectedEvidenceReferences: input.selectedEvidenceReferences,
    selectedPolicyVersions: input.selectedPolicyVersions,
    selectedParentActionReferences: input.selectedParentActionReferences,
    asOf: input.asOf,
  });
}

export function parseLocalAiDerivedKnowledgeStoreSnapshot(input: unknown): LocalAiDerivedKnowledgeStoreSnapshot {
  return LocalAiDerivedKnowledgeStoreSnapshotSchema.parse(input);
}

export function readLocalAiDerivedKnowledgeStore(input: unknown): LocalAiDerivedKnowledgeStoreReadResult {
  const parsed = LocalAiDerivedKnowledgeStoreReadInputSchema.parse(input);
  const usableEntries = selectedUsableEntries(parsed);
  return LocalAiDerivedKnowledgeStoreReadResultSchema.parse({
    status: parsed.snapshot.status,
    readAt: parsed.asOf,
    usableEntries,
    returnedEntryCount: usableEntries.length,
    excludedEntryCount: parsed.snapshot.entries.length - usableEntries.length,
    degradedReasons: [...parsed.snapshot.status.degradedReasons, ...readDegradedReasons(parsed.snapshot.status.state)],
  });
}
