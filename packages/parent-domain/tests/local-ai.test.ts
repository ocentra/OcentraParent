import { describe, expect, it } from 'vitest';
import { LocalAiEvaluationInputSchema } from '../src/local-ai';
import { LocalAiGraphReferenceSchema, LocalAiMemoryReferenceSchema } from '../src/local-ai-references';

const evidenceReference = {
  evidenceReferenceId: 'evidence-1',
  kind: 'journal-event',
  observedAt: '2026-05-20T20:45:00.000Z',
};

const childProfile = { childProfileId: 'child-1', displayName: 'Sam' };
const device = { deviceId: 'device-1', childProfileId: 'child-1', label: 'Sam Windows PC', platform: 'windows' };
const modelRequest = { providerId: 'local-provider', modelId: 'safety-model', promptVersion: 'prompt-v1' };
const parentActionReference = {
  actionReferenceId: 'parent-action-1',
  actor: { actorId: 'parent-1', role: 'parent' },
  policyVersion: 'policy-v1',
  createdAt: '2026-05-20T20:45:30.000Z',
};
const memoryReference = {
  memoryReferenceId: 'memory-1',
  kind: 'recent-activity',
  sourceEvidenceReferences: [evidenceReference],
  sourcePolicyVersion: 'policy-v1',
  sourceParentActionReferences: [parentActionReference],
  generatedAt: '2026-05-20T20:46:00.000Z',
  confidence: 0.82,
  derivedIndexVersion: 'memory-index-v1',
};
const graphReference = {
  graphReferenceId: 'graph-1',
  kind: 'graph-entity',
  sourceEvidenceReferences: [evidenceReference],
  sourcePolicyVersion: 'policy-v1',
  sourceParentActionReferences: [parentActionReference],
  generatedAt: '2026-05-20T20:46:00.000Z',
  confidence: 0.78,
  derivedIndexVersion: 'graph-index-v1',
};
describe('local AI safety decision contracts', () => {
  it('LocalAiEvaluationInputSchema: parses evidence-backed context, rule, memory, and graph references', () => {
    const parsed = LocalAiEvaluationInputSchema.parse({
      schemaVersion: 'v0.6',
      requestId: 'request-1',
      childProfile,
      device,
      currentObservation: { contextKind: 'domain', evidence: evidenceReference },
      evidenceReferences: [evidenceReference],
      parentRuleReferences: ['rule-1'],
      recentActivityWindow: [evidenceReference],
      memoryReferences: [memoryReference],
      graphReferences: [graphReference],
      modelRequest,
    });

    expect(parsed.currentObservation.contextKind).toBe('domain');
    expect(parsed.memoryReferences[0]?.sourceEvidenceReferences).toEqual([evidenceReference]);
    expect(parsed.memoryReferences[0]?.sourceParentActionReferences).toEqual([parentActionReference]);
    expect(parsed.graphReferences[0]?.sourcePolicyVersion).toBe('policy-v1');
  });

  it('LocalAiMemoryReferenceSchema: rejects confidence below zero and above one', () => {
    for (const confidence of [-0.01, 1.01]) {
      const result = LocalAiMemoryReferenceSchema.safeParse({
        ...memoryReference,
        confidence,
      });

      expect(result.success).toBe(false);
      if (!result.success) {
        expect([...new Set(result.error.issues.map((issue) => issue.path.join('.')))]).toEqual(['confidence']);
      }
    }
  });

  it('LocalAiGraphReferenceSchema: rejects confidence below zero and above one', () => {
    for (const confidence of [-0.01, 1.01]) {
      const result = LocalAiGraphReferenceSchema.safeParse({
        ...graphReference,
        confidence,
      });

      expect(result.success).toBe(false);
      if (!result.success) {
        expect([...new Set(result.error.issues.map((issue) => issue.path.join('.')))]).toEqual(['confidence']);
      }
    }
  });
});
