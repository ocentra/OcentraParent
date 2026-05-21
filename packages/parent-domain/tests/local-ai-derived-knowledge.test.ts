import { describe, expect, it } from 'vitest';
import {
  LocalAiDerivedKnowledgeEntrySchema,
  LocalAiDerivedKnowledgeIndexStatusSchema,
} from '../src/local-ai-derived-knowledge';
import { selectUsableDerivedKnowledgeEntries } from '../src/local-ai-derived-knowledge-selection';

const observedAt = '2026-05-21T20:30:00.000Z';
const asOf = '2026-05-21T20:35:00.000Z';
const futureExpiry = '2026-05-21T20:45:00.000Z';
const staleExpiry = '2026-05-21T20:34:00.000Z';
const sourceEvidence = { evidenceReferenceId: 'stored-evidence-1', kind: 'journal-event', observedAt };
const unselectedEvidence = { evidenceReferenceId: 'stored-evidence-unselected', kind: 'journal-event', observedAt };
const selectedParentAction = {
  actionReferenceId: 'parent-action-1',
  actor: { actorId: 'parent-1', role: 'parent' },
  policyVersion: 'policy-v1',
  createdAt: observedAt,
};
const unselectedParentAction = {
  actionReferenceId: 'parent-action-unselected',
  actor: { actorId: 'parent-1', role: 'parent' },
  policyVersion: 'policy-v1',
  createdAt: observedAt,
};

const baseEntry = {
  entryId: 'derived-entry-usable',
  indexId: 'derived-index-1',
  indexKind: 'hybrid',
  entryKind: 'semantic-match',
  entryStatus: 'usable',
  derivedIndexVersion: 'derived-index-v1',
  generatedAt: observedAt,
  expiresAt: futureExpiry,
  confidence: 0.82,
  citations: {
    sourceEvidenceReferences: [sourceEvidence],
    sourcePolicyVersions: ['policy-v1'],
    sourceParentActionReferences: [selectedParentAction],
  },
  degradedReasons: [],
};

function selectEntryIds(entries: readonly unknown[]): readonly string[] {
  return selectUsableDerivedKnowledgeEntries({
    entries,
    selectedEvidenceReferences: [sourceEvidence],
    selectedPolicyVersions: ['policy-v1'],
    selectedParentActionReferences: [selectedParentAction],
    asOf,
  }).map((entry) => entry.entryId);
}

function assertRejectsEntryWithoutCitations(): void {
  const parsed = LocalAiDerivedKnowledgeEntrySchema.safeParse({
    ...baseEntry,
    citations: {
      sourceEvidenceReferences: [],
      sourcePolicyVersions: [],
      sourceParentActionReferences: [],
    },
  });

  expect(parsed.success).toBe(false);
}

function assertRejectsImpossibleStatusCounts(): void {
  const parsed = LocalAiDerivedKnowledgeIndexStatusSchema.safeParse({
    schemaVersion: 'v0.6',
    indexId: 'derived-index-1',
    indexKind: 'hybrid',
    indexVersion: 'derived-index-v1',
    state: 'ready',
    generatedAt: observedAt,
    refreshedAt: observedAt,
    entryCount: 1,
    usableEntryCount: 2,
    sourceEvidenceCitationCount: 1,
    sourcePolicyVersionCitationCount: 1,
    sourceParentActionCitationCount: 1,
    degradedReasons: [],
  });

  expect(parsed.success).toBe(false);
}

function assertExcludesStaleEntries(): void {
  const selectedEntryIds = selectEntryIds([
    baseEntry,
    { ...baseEntry, entryId: 'derived-entry-stale-expiry', expiresAt: staleExpiry },
    { ...baseEntry, entryId: 'derived-entry-stale-status', entryStatus: 'stale' },
  ]);

  expect(selectedEntryIds).toEqual(['derived-entry-usable']);
}

function assertExcludesUnselectedEvidenceCitations(): void {
  const selectedEntryIds = selectEntryIds([
    baseEntry,
    {
      ...baseEntry,
      entryId: 'derived-entry-unselected-evidence',
      citations: {
        ...baseEntry.citations,
        sourceEvidenceReferences: [unselectedEvidence],
      },
    },
  ]);

  expect(selectedEntryIds).toEqual(['derived-entry-usable']);
}

function assertExcludesUnselectedPolicyAndParentActionCitations(): void {
  const selectedEntryIds = selectEntryIds([
    baseEntry,
    {
      ...baseEntry,
      entryId: 'derived-entry-unselected-policy',
      citations: {
        ...baseEntry.citations,
        sourcePolicyVersions: ['policy-v2'],
      },
    },
    {
      ...baseEntry,
      entryId: 'derived-entry-unselected-action',
      citations: {
        ...baseEntry.citations,
        sourceParentActionReferences: [unselectedParentAction],
      },
    },
  ]);

  expect(selectedEntryIds).toEqual(['derived-entry-usable']);
}

describe('local AI derived knowledge index contracts', () => {
  it('rejects derived knowledge entries without evidence, policy, or parent-action citations', () => {
    assertRejectsEntryWithoutCitations();
  });

  it('rejects index status counts where usable entries exceed total entries', () => {
    assertRejectsImpossibleStatusCounts();
  });

  it('excludes stale derived knowledge entries from usable context', () => {
    assertExcludesStaleEntries();
  });

  it('excludes derived knowledge entries whose stored evidence citation was not selected', () => {
    assertExcludesUnselectedEvidenceCitations();
  });

  it('excludes derived knowledge entries whose policy or parent-action citation was not selected', () => {
    assertExcludesUnselectedPolicyAndParentActionCitations();
  });
});
