import { describe, expect, it } from 'vitest';
import {
  LocalAiDerivedKnowledgeStoreSnapshotSchema,
  parseLocalAiDerivedKnowledgeStoreSnapshot,
  readLocalAiDerivedKnowledgeStore,
} from '../src/local-ai-derived-knowledge-store';

const observedAt = '2026-05-21T21:00:00.000Z';
const asOf = '2026-05-21T21:10:00.000Z';
const futureExpiry = '2026-05-21T21:30:00.000Z';
const staleExpiry = '2026-05-21T21:05:00.000Z';
const sourceEvidence = { evidenceReferenceId: 'stored-evidence-1', kind: 'journal-event', observedAt };
const unselectedEvidence = { evidenceReferenceId: 'stored-evidence-unselected', kind: 'journal-event', observedAt };
const selectedParentAction = {
  actionReferenceId: 'parent-action-1',
  actor: { actorId: 'parent-1', role: 'parent' },
  policyVersion: 'policy-v1',
  createdAt: observedAt,
};

const usableEntry = {
  entryId: 'derived-entry-usable',
  indexId: 'derived-index-1',
  indexKind: 'hybrid',
  entryKind: 'semantic-match',
  entryStatus: 'usable',
  derivedIndexVersion: 'derived-index-v1',
  generatedAt: observedAt,
  expiresAt: futureExpiry,
  confidence: 0.88,
  citations: {
    sourceEvidenceReferences: [sourceEvidence],
    sourcePolicyVersions: ['policy-v1'],
    sourceParentActionReferences: [selectedParentAction],
  },
  degradedReasons: [],
};

const staleEntry = {
  ...usableEntry,
  entryId: 'derived-entry-stale',
  expiresAt: staleExpiry,
};

const unselectedCitationEntry = {
  ...usableEntry,
  entryId: 'derived-entry-unselected-citation',
  citations: {
    ...usableEntry.citations,
    sourceEvidenceReferences: [unselectedEvidence],
  },
};

const candidateEntry = {
  ...usableEntry,
  entryId: 'derived-entry-candidate',
  entryStatus: 'candidate',
};

function statusFor(
  entries: readonly (typeof usableEntry)[],
  overrides: Record<string, unknown> = {}
): Record<string, unknown> {
  return {
    schemaVersion: 'v0.6',
    indexId: 'derived-index-1',
    indexKind: 'hybrid',
    indexVersion: 'derived-index-v1',
    state: 'ready',
    generatedAt: observedAt,
    refreshedAt: observedAt,
    entryCount: entries.length,
    usableEntryCount: entries.filter((entry) => entry.entryStatus === 'usable').length,
    sourceEvidenceCitationCount: entries.reduce(
      (count, entry) => count + entry.citations.sourceEvidenceReferences.length,
      0
    ),
    sourcePolicyVersionCitationCount: entries.reduce(
      (count, entry) => count + entry.citations.sourcePolicyVersions.length,
      0
    ),
    sourceParentActionCitationCount: entries.reduce(
      (count, entry) => count + entry.citations.sourceParentActionReferences.length,
      0
    ),
    degradedReasons: [],
    ...overrides,
  };
}

function snapshotFor(entries: readonly (typeof usableEntry)[], statusOverrides: Record<string, unknown> = {}) {
  return {
    status: statusFor(entries, statusOverrides),
    entries,
  };
}

function readEntryIds(snapshot: unknown): readonly string[] {
  return readLocalAiDerivedKnowledgeStore({
    snapshot,
    selectedEvidenceReferences: [sourceEvidence],
    selectedPolicyVersions: ['policy-v1'],
    selectedParentActionReferences: [selectedParentAction],
    asOf,
  }).usableEntries.map((entry) => entry.entryId);
}

describe('local AI derived knowledge store read path', () => {
  it('parses a store snapshot whose status counts match the stored entries and citations', () => {
    const snapshot = parseLocalAiDerivedKnowledgeStoreSnapshot(snapshotFor([usableEntry, candidateEntry]));

    expect(snapshot.status.entryCount).toBe(2);
    expect(snapshot.status.usableEntryCount).toBe(1);
    expect(snapshot.status.sourceEvidenceCitationCount).toBe(2);
  });

  it('rejects snapshots whose status counts do not match stored rows', () => {
    const parsed = LocalAiDerivedKnowledgeStoreSnapshotSchema.safeParse(
      snapshotFor([usableEntry], { sourceEvidenceCitationCount: 2 })
    );

    expect(parsed.success).toBe(false);
  });

  it('rejects snapshots that mix entries from another derived index or version', () => {
    const parsed = LocalAiDerivedKnowledgeStoreSnapshotSchema.safeParse(
      snapshotFor([{ ...usableEntry, indexId: 'derived-index-other' }])
    );

    expect(parsed.success).toBe(false);
  });

  it('reads only citation-selected and fresh usable entries from the store snapshot', () => {
    const selectedEntryIds = readEntryIds(
      snapshotFor([usableEntry, staleEntry, unselectedCitationEntry, candidateEntry])
    );

    expect(selectedEntryIds).toEqual(['derived-entry-usable']);
  });

  it('does not return entries from stale or unavailable index states', () => {
    const staleRead = readLocalAiDerivedKnowledgeStore({
      snapshot: snapshotFor([usableEntry], { state: 'stale', degradedReasons: ['stale-evidence'] }),
      selectedEvidenceReferences: [sourceEvidence],
      selectedPolicyVersions: ['policy-v1'],
      selectedParentActionReferences: [selectedParentAction],
      asOf,
    });

    expect(staleRead.usableEntries).toEqual([]);
    expect(staleRead.degradedReasons).toContain('stale-evidence');
    expect(staleRead.excludedEntryCount).toBe(1);
  });
});
