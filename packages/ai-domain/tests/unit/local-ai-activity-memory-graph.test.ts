import { describe, expect, it } from 'vitest';
import {
  LocalAiActivityMemoryGraphQuerySchema,
  LocalAiActivityMemoryGraphReadResultSchema,
  LocalAiActivityMemoryGraphTraceSchema,
} from '@ocentra-parent/schema-domain/local-ai-activity-memory-graph';
import { readLocalAiActivityMemoryGraph } from '@ocentra-parent/schema-domain/local-ai-activity-memory-graph-read';

const observedAt = '2026-05-20T20:45:00.000Z';
const asOf = '2026-05-20T21:00:00.000Z';
const freshUntil = '2026-05-20T21:15:00.000Z';
const staleAt = '2026-05-20T20:50:00.000Z';
const evidenceReference = {
  evidenceReferenceId: 'evidence-browser-visit-1',
  kind: 'journal-event',
  observedAt,
};
const unselectedEvidenceReference = {
  evidenceReferenceId: 'evidence-unselected-1',
  kind: 'journal-event',
  observedAt,
};
const parentActionReference = {
  actionReferenceId: 'parent-action-1',
  actor: { actorId: 'parent-1', role: 'parent' },
  policyVersion: 'policy-v1',
  createdAt: '2026-05-20T20:40:00.000Z',
};
const childProfile = { childProfileId: 'child-1', displayName: 'Sam' };
const device = { deviceId: 'device-1', childProfileId: 'child-1', label: 'Sam Windows PC', platform: 'windows' };
const trace = {
  entryStatus: 'usable',
  sourceEvidenceReferences: [evidenceReference],
  sourcePolicyVersion: 'policy-v1',
  sourceParentActionReferences: [parentActionReference],
  generatedAt: '2026-05-20T20:46:00.000Z',
  expiresAt: freshUntil,
  confidence: 0.86,
  derivedIndexVersion: 'activity-memory-v1',
  degradedReasons: [],
};
const query = {
  queryId: 'query-visited-urls-1',
  queryKind: 'visited-urls',
  childProfile,
  device,
  timeRange: { observedFrom: '2026-05-20T20:30:00.000Z', observedUntil: asOf },
  asOf,
  limit: 10,
};
const childNode = {
  graphId: 'graph-activity-1',
  nodeId: 'node-child',
  nodeKind: 'child-profile',
  label: 'Sam',
  childProfile,
  device: null,
  trace,
};
const urlNode = {
  graphId: 'graph-activity-1',
  nodeId: 'node-url',
  nodeKind: 'browser-url',
  label: 'https://example.test/game',
  childProfile: null,
  device,
  trace,
};
const gameNode = {
  graphId: 'graph-activity-1',
  nodeId: 'node-game',
  nodeKind: 'game',
  label: 'Example Game',
  childProfile: null,
  device,
  trace,
};
const visitedEdge = {
  graphId: 'graph-activity-1',
  edgeId: 'edge-visited',
  edgeKind: 'visited',
  fromNodeId: 'node-child',
  toNodeId: 'node-url',
  observedFrom: observedAt,
  observedUntil: '2026-05-20T20:55:00.000Z',
  durationMs: 600000,
  trace,
};
const mixedActivityGraphReadInput = {
  query,
  nodes: [childNode, urlNode, gameNode],
  edges: [
    visitedEdge,
    { ...visitedEdge, edgeId: 'edge-played', edgeKind: 'played', toNodeId: 'node-game' },
    { ...visitedEdge, edgeId: 'edge-stale', trace: { ...trace, expiresAt: staleAt } },
    {
      ...visitedEdge,
      edgeId: 'edge-unselected',
      trace: { ...trace, sourceEvidenceReferences: [unselectedEvidenceReference] },
    },
    {
      ...visitedEdge,
      edgeId: 'edge-outside-range',
      observedFrom: '2026-05-20T19:00:00.000Z',
      observedUntil: '2026-05-20T19:05:00.000Z',
    },
    { ...visitedEdge, edgeId: 'edge-dangling', toNodeId: 'node-missing' },
  ],
  selectedEvidenceReferences: [evidenceReference],
  selectedPolicyVersions: ['policy-v1'],
  selectedParentActionReferences: [parentActionReference],
};

describe('local AI activity memory graph contracts', () => {
  it('LocalAiActivityMemoryGraphTraceSchema: rejects uncited derived graph entries', () => {
    const result = LocalAiActivityMemoryGraphTraceSchema.safeParse({
      ...trace,
      sourceEvidenceReferences: [],
      sourcePolicyVersion: null,
      sourceParentActionReferences: [],
    });

    expect(result.success).toBe(false);
    if (!result.success) {
      expect(result.error.issues.map((issue) => issue.message)).toContain(
        'Expected memory graph entry to cite source context'
      );
    }
  });

  it('LocalAiActivityMemoryGraphQuerySchema: rejects inverted time ranges and negative limits', () => {
    const result = LocalAiActivityMemoryGraphQuerySchema.safeParse({
      ...query,
      timeRange: { observedFrom: asOf, observedUntil: observedAt },
      limit: -1,
    });

    expect(result.success).toBe(false);
    if (!result.success) {
      expect([...new Set(result.error.issues.map((issue) => issue.path.join('.')))]).toEqual(['timeRange', 'limit']);
    }
  });

  it('LocalAiActivityMemoryGraphQuerySchema: accepts device-only local queries without child identity', () => {
    const result = LocalAiActivityMemoryGraphQuerySchema.safeParse({
      ...query,
      childProfile: null,
    });

    expect(result.success).toBe(true);
  });

  it('readLocalAiActivityMemoryGraph: returns only fresh visited edges grounded in selected source refs', () => {
    const result = readLocalAiActivityMemoryGraph(mixedActivityGraphReadInput);

    expect(result.edges.map((edge) => edge.edgeId)).toEqual(['edge-visited']);
    expect(result.nodes.map((node) => node.nodeId).sort()).toEqual(['node-child', 'node-url']);
    expect(result.omittedEdgeCount).toBe(5);
    expect(result.degradedReasons).toEqual(['memory-ungrounded']);
  });

  it('LocalAiActivityMemoryGraphReadResultSchema: rejects dangling returned edges', () => {
    const result = LocalAiActivityMemoryGraphReadResultSchema.safeParse({
      query,
      readAt: asOf,
      nodes: [childNode],
      edges: [visitedEdge],
      returnedNodeCount: 1,
      returnedEdgeCount: 1,
      omittedEdgeCount: 0,
      degradedReasons: [],
    });

    expect(result.success).toBe(false);
  });
});
