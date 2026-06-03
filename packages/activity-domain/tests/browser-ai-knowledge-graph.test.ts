import { describe, expect, it } from 'vitest';
import {
  BrowserAiKnowledgeGraphReferenceBundleSchema,
  BrowserAiKnowledgeGraphSchemaVersion,
} from '../src/browser-ai-knowledge-graph-schemas';

describe('browser AI knowledge graph reference contract', () => {
  it('accepts fresh evidence-backed graph refs as candidate support', acceptsFreshGraphCandidateSupport);
  it('rejects platform label nodes as policy authority', rejectsPlatformLabelAuthority);
  it('rejects stale graph refs that still drive policy input', rejectsStalePolicyInput);
  it('rejects graph edges that point outside the bundle', rejectsUnknownEdgeNodeRef);
  it('accepts stale manual-required graph refs when they cannot drive policy', acceptsManualRequiredGraphRefs);
  it('rejects raw content or direct authority claims', rejectsRawContentAuthority);
  it('rejects duplicate graph node refs', rejectsDuplicateNodeRefs);
});

function acceptsFreshGraphCandidateSupport() {
  const parsed = BrowserAiKnowledgeGraphReferenceBundleSchema.safeParse(freshReferenceBundle());

  expect(parsed.success).toBe(true);
  if (parsed.success) {
    expect(parsed.data.canDrivePolicyInput).toBe(true);
    expect(parsed.data.uses).toContain('policy-candidate-support');
    expect(parsed.data.nodes.map((node) => node.nodeKind)).toEqual([
      'platform-video',
      'platform-channel',
      'content-category',
      'risk-signal',
    ]);
  }
}

function rejectsPlatformLabelAuthority() {
  const parsed = BrowserAiKnowledgeGraphReferenceBundleSchema.safeParse({
    ...freshReferenceBundle(),
    nodes: [
      {
        ...freshReferenceBundle().nodes[0],
        sourceKind: 'platform-label',
        platformLabelUsedAsAuthority: true,
      },
      ...freshReferenceBundle().nodes.slice(1),
    ],
  });

  expect(parsed.success).toBe(false);
}

function rejectsStalePolicyInput() {
  const staleNodeBundle = {
    ...freshReferenceBundle(),
    nodes: [{ ...freshReferenceBundle().nodes[0], stale: true }, ...freshReferenceBundle().nodes.slice(1)],
  };

  expect(BrowserAiKnowledgeGraphReferenceBundleSchema.safeParse(staleNodeBundle).success).toBe(false);
}

function rejectsUnknownEdgeNodeRef() {
  const parsed = BrowserAiKnowledgeGraphReferenceBundleSchema.safeParse({
    ...freshReferenceBundle(),
    edges: [
      {
        ...freshReferenceBundle().edges[0],
        toNodeRef: 'kg-node-missing-risk',
      },
    ],
  });

  expect(parsed.success).toBe(false);
}

function acceptsManualRequiredGraphRefs() {
  const parsed = BrowserAiKnowledgeGraphReferenceBundleSchema.safeParse({
    ...freshReferenceBundle(),
    policyVersionRef: null,
    uses: ['ai-input-context', 'parent-explanation'],
    nodes: freshReferenceBundle().nodes.map((node) => ({
      ...node,
      stale: true,
      canDrivePolicyInput: false,
    })),
    edges: freshReferenceBundle().edges.map((edge) => ({
      ...edge,
      stale: true,
      canDrivePolicyInput: false,
    })),
    canDrivePolicyInput: false,
  });

  expect(parsed.success).toBe(true);
  if (parsed.success) {
    expect(parsed.data.policyVersionRef).toBeNull();
    expect(parsed.data.canDrivePolicyInput).toBe(false);
  }
}

function rejectsRawContentAuthority() {
  const parsed = BrowserAiKnowledgeGraphReferenceBundleSchema.safeParse({
    ...freshReferenceBundle(),
    rawContentStored: true,
    directPolicyAuthorityClaimed: true,
    directEnforcementClaimed: true,
  });

  expect(parsed.success).toBe(false);
}

function rejectsDuplicateNodeRefs() {
  const firstNode = freshReferenceBundle().nodes[0];
  const parsed = BrowserAiKnowledgeGraphReferenceBundleSchema.safeParse({
    ...freshReferenceBundle(),
    nodes: [
      firstNode,
      { ...freshReferenceBundle().nodes[1], nodeRef: firstNode.nodeRef },
      ...freshReferenceBundle().nodes.slice(2),
    ],
  });

  expect(parsed.success).toBe(false);
}

function freshReferenceBundle() {
  return {
    schemaVersion: BrowserAiKnowledgeGraphSchemaVersion,
    graphId: 'browser-ai-knowledge-graph-household',
    graphRef: 'kg-ref-youtube-math-video',
    snapshotId: 'browser-ai-knowledge-graph-snapshot-1',
    graphVersionRef: 'browser-ai-knowledge-graph-v1',
    capturedAt: '2026-06-03T03:58:00.000Z',
    policyVersionRef: 'browser-policy-version-family-2026-06',
    uses: ['ai-input-context', 'policy-candidate-support', 'parent-explanation'],
    nodes: [
      graphNode('kg-node-video', 'platform-video', 'browser-evidence'),
      graphNode('kg-node-channel', 'platform-channel', 'metadata-evidence'),
      graphNode('kg-node-education', 'content-category', 'ai-analysis'),
      graphNode('kg-node-privacy-risk', 'risk-signal', 'memory-cache'),
    ],
    edges: [
      graphEdge('kg-edge-channel', 'belongs-to-channel', 'kg-node-video', 'kg-node-channel'),
      graphEdge('kg-edge-category', 'has-category-signal', 'kg-node-video', 'kg-node-education'),
      graphEdge('kg-edge-risk', 'has-risk-signal', 'kg-node-video', 'kg-node-privacy-risk'),
    ],
    retentionBounded: true,
    rawContentStored: false,
    canDrivePolicyInput: true,
    directPolicyAuthorityClaimed: false,
    directEnforcementClaimed: false,
  };
}

function graphNode(nodeRef: unknown, nodeKind: unknown, sourceKind: unknown) {
  return {
    graphRef: 'kg-ref-youtube-math-video',
    nodeRef,
    nodeKind,
    sourceKind,
    sourceEvidenceIds: ['browser-evidence-youtube-math-video'],
    parentRuleRefs: ['parent-rule-homework-help'],
    confidence: 'medium',
    uncertaintyReasons: [],
    stale: false,
    parentApproved: sourceKind !== 'external-taxonomy',
    canDrivePolicyInput: true,
    rawContentStored: false,
    platformLabelUsedAsAuthority: false,
    directPolicyAuthorityClaimed: false,
    directEnforcementClaimed: false,
  };
}

function graphEdge(edgeRef: unknown, edgeKind: unknown, fromNodeRef: unknown, toNodeRef: unknown) {
  return {
    edgeRef,
    edgeKind,
    fromNodeRef,
    toNodeRef,
    sourceEvidenceIds: ['browser-evidence-youtube-math-video'],
    confidence: 'medium',
    uncertaintyReasons: [],
    stale: false,
    canDrivePolicyInput: true,
    directPolicyAuthorityClaimed: false,
    directEnforcementClaimed: false,
  };
}
