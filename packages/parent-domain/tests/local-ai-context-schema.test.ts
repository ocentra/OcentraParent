import { describe, expect, it } from 'vitest';
import { LocalAiEvidenceContextSourceRefSchema } from '../src/local-ai-context';
import { LocalAiGraphReferenceSchema, LocalAiMemoryReferenceSchema } from '../src/local-ai-references';

const childProfile = { childProfileId: 'child-1', displayName: 'Sam' };
const device = { deviceId: 'device-1', childProfileId: 'child-1', label: 'Sam Windows PC', platform: 'windows' };
const observedAt = '2026-05-21T09:10:00.000Z';
const sourceEvidence = {
  evidenceReferenceId: 'journal-event-1',
  kind: 'journal-event',
  observedAt,
};

const contextSourceRef = {
  evidenceRefId: 'schema-ref-1',
  evidence: {
    evidenceReferenceId: 'stored-schema-ref-1',
    kind: 'query-store-summary',
    observedAt,
  },
  evidenceKind: 'network-flow',
  sourceSchemaVersion: 'v0.6',
  observedAt,
  ingestedAt: '2026-05-21T09:10:01.000Z',
  freshUntil: '2026-05-21T09:15:00.000Z',
  sourceId: 'source-schema-ref-1',
  adapterId: 'adapter-schema-ref-1',
  device,
  childProfile,
  custody: 'child-device-query-store',
  retentionState: 'local',
  confidence: 0.85,
  confidenceKind: 'classifier',
  capabilityStatus: 'available',
  degradedReasons: [],
  unknownReasons: [],
  sourceEvidenceReferences: [sourceEvidence],
};

describe('local AI evidence context source schema', () => {
  it('rejects missing source evidence and invalid confidence', () => {
    const missingSourceEvidence = LocalAiEvidenceContextSourceRefSchema.safeParse({
      ...contextSourceRef,
      sourceEvidenceReferences: [],
    });
    const invalidConfidence = LocalAiEvidenceContextSourceRefSchema.safeParse({
      ...contextSourceRef,
      confidence: 1.01,
    });

    expect(missingSourceEvidence.success).toBe(false);
    expect(invalidConfidence.success).toBe(false);
    if (!invalidConfidence.success) {
      expect([...new Set(invalidConfidence.error.issues.map((issue) => issue.path.join('.')))]).toEqual(['confidence']);
    }
  });

  it('rejects memory and graph references without source evidence', () => {
    const missingMemorySource = LocalAiMemoryReferenceSchema.safeParse({
      memoryReferenceId: 'memory-schema-1',
      kind: 'evidence-memory',
      sourceEvidenceReferences: [],
      sourcePolicyVersion: null,
      generatedAt: observedAt,
      confidence: 0.8,
      derivedIndexVersion: 'memory-index-v1',
    });
    const missingGraphSource = LocalAiGraphReferenceSchema.safeParse({
      graphReferenceId: 'graph-schema-1',
      kind: 'graph-edge',
      sourceEvidenceReferences: [],
      sourcePolicyVersion: null,
      generatedAt: observedAt,
      confidence: 0.8,
      derivedIndexVersion: 'graph-index-v1',
    });

    expect(missingMemorySource.success).toBe(false);
    expect(missingGraphSource.success).toBe(false);
  });
});
