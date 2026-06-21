import { describe, expect, it } from 'vitest';
import { buildLocalAiGraphReferenceContractProof } from '@ocentra-parent/schema-domain/local-ai-graph-reference-contract-proof';

const observedAt = '2026-06-06T06:40:00.000Z';
const asOf = '2026-06-06T06:45:00.000Z';
const childProfile = { childProfileId: 'child-local-ai-graph', displayName: 'Sam' };
const device = {
  deviceId: 'device-local-ai-graph',
  childProfileId: 'child-local-ai-graph',
  label: 'Sam Windows PC',
  platform: 'windows',
};
const sourceEvidence = {
  evidenceReferenceId: 'local-ai-graph-source-evidence',
  kind: 'journal-event',
  observedAt,
};
const otherEvidence = {
  evidenceReferenceId: 'local-ai-graph-other-evidence',
  kind: 'journal-event',
  observedAt,
};
const parentAction = {
  actionReferenceId: 'local-ai-graph-parent-action',
  actor: { actorId: 'parent-local-ai-graph', role: 'parent' },
  policyVersion: 'local-ai-graph-policy-v1',
  createdAt: '2026-06-06T06:35:00.000Z',
};
const trace = {
  entryStatus: 'usable',
  sourceEvidenceReferences: [sourceEvidence],
  sourcePolicyVersion: 'local-ai-graph-policy-v1',
  sourceParentActionReferences: [parentAction],
  generatedAt: '2026-06-06T06:41:00.000Z',
  expiresAt: '2026-06-06T07:00:00.000Z',
  confidence: 0.84,
  derivedIndexVersion: 'local-ai-graph-index-v1',
  degradedReasons: [],
};
const query = {
  queryId: 'local-ai-graph-query',
  queryKind: 'activity-by-time-range',
  childProfile,
  device,
  timeRange: { observedFrom: '2026-06-06T06:30:00.000Z', observedUntil: asOf },
  asOf,
  limit: 2,
};
const childNode = {
  graphId: 'local-ai-graph',
  nodeId: 'local-ai-graph-child',
  nodeKind: 'child-profile',
  label: 'Sam',
  childProfile,
  device: null,
  trace,
};
const domainNode = {
  graphId: 'local-ai-graph',
  nodeId: 'local-ai-graph-domain',
  nodeKind: 'domain',
  label: 'example.test',
  childProfile: null,
  device,
  trace,
};
const visitedEdge = {
  graphId: 'local-ai-graph',
  edgeId: 'local-ai-graph-edge-visited',
  edgeKind: 'visited',
  fromNodeId: 'local-ai-graph-child',
  toNodeId: 'local-ai-graph-domain',
  observedFrom: observedAt,
  observedUntil: '2026-06-06T06:44:00.000Z',
  durationMs: 240000,
  trace,
};
const claimBoundaries = {
  remoteAiUsed: false,
  apiAiUsed: false,
  modelQualityClaimed: false,
  policyAuthorityClaimed: false,
  enforcementClaimed: false,
  uiClaimed: false,
  rawEvidenceRetained: false,
  uncitedGraphAllowed: false,
};

function proofInput(graphSource = sourceEvidence, edge = visitedEdge): unknown {
  return {
    schemaVersion: 'v0.6',
    graphReadInput: {
      query,
      nodes: [childNode, domainNode],
      edges: [edge],
      selectedEvidenceReferences: [sourceEvidence],
      selectedPolicyVersions: ['local-ai-graph-policy-v1'],
      selectedParentActionReferences: [parentAction],
    },
    graphReferences: [
      {
        graphReferenceId: 'local-ai-graph-reference',
        kind: 'graph-edge',
        sourceEvidenceReferences: [graphSource],
        sourcePolicyVersion: 'local-ai-graph-policy-v1',
        generatedAt: observedAt,
        confidence: 0.81,
        derivedIndexVersion: 'local-ai-graph-index-v1',
      },
    ],
    claimBoundaries,
  };
}

describe('local AI graph reference contract proof', () => {
  it('returns minimal graph edges only when graph refs cite selected evidence', () => {
    const proof = buildLocalAiGraphReferenceContractProof(proofInput());

    expect(proof.graphReadResult.edges.map((edge) => edge.edgeId)).toEqual(['local-ai-graph-edge-visited']);
    expect(proof.graphReadResult.nodes.map((node) => node.nodeId).sort()).toEqual([
      'local-ai-graph-child',
      'local-ai-graph-domain',
    ]);
    expect(proof.selectedGraphReferences.map((reference) => reference.graphReferenceId)).toEqual([
      'local-ai-graph-reference',
    ]);
    expect(proof.summary).toEqual({
      inputGraphReferenceCount: 1,
      selectedGraphReferenceCount: 1,
      returnedNodeCount: 2,
      returnedEdgeCount: 1,
      omittedEdgeCount: 0,
      selectedEvidenceReferenceCount: 1,
    });
  });

  it('rejects graph references without selected source evidence', () => {
    expect(() => buildLocalAiGraphReferenceContractProof(proofInput(otherEvidence))).toThrow(
      'Expected local AI graph proof to return grounded minimal graph edges without overclaiming authority'
    );
  });

  it('rejects edges that point to the same graph node', () => {
    expect(() =>
      buildLocalAiGraphReferenceContractProof(
        proofInput(sourceEvidence, { ...visitedEdge, toNodeId: 'local-ai-graph-child' })
      )
    ).toThrow();
  });

  it('rejects UI, model-quality, policy-authority, enforcement, remote, or retention overclaims', () => {
    expect(() =>
      buildLocalAiGraphReferenceContractProof({
        ...proofInput(),
        claimBoundaries: { ...claimBoundaries, uiClaimed: true },
      })
    ).toThrow();
  });
});
