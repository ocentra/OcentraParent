import { describe, expect, it } from 'vitest';
import {
  screenAiMemoryGraphSourceProofIsReady,
  ScreenAiMemoryGraphSourceProofSchema,
  summarizeScreenAiMemoryGraphSourceProof,
} from '../src/screen-ai-memory-graph-source-proof';

const generatedAt = '2026-06-05T15:59:33.027Z';
const evidenceReferences = [
  { evidenceReferenceId: 'screen-activity-row', kind: 'activity-event', observedAt: generatedAt },
  { evidenceReferenceId: 'screen-summary-ref', kind: 'query-store-summary', observedAt: generatedAt },
];
const parentActionReference = {
  actionReferenceId: 'screen-policy-dry-run',
  actor: { actorId: 'system-screen-ai-policy', role: 'system' },
  policyVersion: 'screen-service-winrt-ocr-school-rule',
  createdAt: generatedAt,
};
const trace = {
  entryStatus: 'usable',
  sourceEvidenceReferences: evidenceReferences,
  sourcePolicyVersion: 'screen-service-winrt-ocr-school-rule',
  sourceParentActionReferences: [parentActionReference],
  generatedAt,
  expiresAt: null,
  confidence: 0.91,
  derivedIndexVersion: 'screen-ai-memory-graph-v1',
  degradedReasons: [],
};
const query = {
  queryId: 'screen-ai-memory-graph-source-query',
  queryKind: 'explain-evidence',
  childProfile: { childProfileId: 'screen-ai-memory-child', displayName: 'Sam' },
  device: {
    deviceId: 'local-dev-agent',
    childProfileId: 'screen-ai-memory-child',
    label: 'Sam Windows PC',
    platform: 'windows',
  },
  timeRange: { observedFrom: generatedAt, observedUntil: generatedAt },
  asOf: generatedAt,
  limit: 10,
};
const memoryGraphRead = {
  query,
  readAt: generatedAt,
  nodes: [
    {
      graphId: 'screen-ai-memory-graph',
      nodeId: 'node-child',
      nodeKind: 'child-profile',
      label: 'Sam',
      childProfile: query.childProfile,
      device: null,
      trace,
    },
    {
      graphId: 'screen-ai-memory-graph',
      nodeId: 'node-activity',
      nodeKind: 'activity-session',
      label: 'Windows WinRT OCR school activity',
      childProfile: query.childProfile,
      device: query.device,
      trace,
    },
  ],
  edges: [
    {
      graphId: 'screen-ai-memory-graph',
      edgeId: 'edge-child-activity',
      edgeKind: 'performed-by-child',
      fromNodeId: 'node-child',
      toNodeId: 'node-activity',
      observedFrom: generatedAt,
      observedUntil: generatedAt,
      durationMs: 0,
      trace,
    },
  ],
  returnedNodeCount: 2,
  returnedEdgeCount: 1,
  omittedEdgeCount: 0,
  degradedReasons: [],
};

describe('screen AI memory graph source proof', () => {
  it('accepts a source-cited memory graph proof over deleted screen OCR policy evidence', () => {
    const proof = ScreenAiMemoryGraphSourceProofSchema.parse(validProof());

    expect(screenAiMemoryGraphSourceProofIsReady(proof)).toBe(true);
    expect(summarizeScreenAiMemoryGraphSourceProof(proof)).toEqual({
      nodeCount: 2,
      edgeCount: 1,
      sourceEvidenceReferenceCount: 2,
      sourceParentActionReferenceCount: 1,
      rawImageRetained: false,
      imageDeletionState: 'deleted',
      assertionCount: 8,
    });
  });

  it('rejects graph proofs that retain raw screen images', () => {
    expect(() =>
      ScreenAiMemoryGraphSourceProofSchema.parse({
        ...validProof(),
        sourceCustody: {
          sourceImageDeletionState: 'deleted',
          rawImageRetained: true,
          custodyState: 'child-device-journal',
        },
      })
    ).toThrow(/false/u);
  });

  it('rejects graph proofs whose returned edge is not cited by selected evidence', () => {
    expect(() =>
      ScreenAiMemoryGraphSourceProofSchema.parse({
        ...validProof(),
        memoryGraphRead: {
          ...memoryGraphRead,
          edges: [
            {
              ...memoryGraphRead.edges[0],
              trace: {
                ...trace,
                sourceEvidenceReferences: [
                  { evidenceReferenceId: 'unselected-evidence', kind: 'activity-event', observedAt: generatedAt },
                ],
              },
            },
          ],
        },
      })
    ).toThrow(/selected source evidence/u);
  });
});

function validProof() {
  return {
    schemaVersion: 'v0.6',
    proofId: 'screen-ai-memory-graph-source-proof',
    generatedAt,
    sourceProofArtifact: 'output/screen-ai-pipeline-proof/service-winrt-ocr-policy/proof-summary.json',
    sourcePolicyReadModelArtifact:
      'output/screen-ai-pipeline-proof/service-winrt-ocr-policy/activity-screen-policy-read-model.json',
    sourcePolicyDecisionId: 'screen-policy-dry-run',
    sourcePolicyAction: 'allow',
    sourceEvidenceReferences: evidenceReferences,
    sourceParentActionReferences: [parentActionReference],
    sourceCustody: {
      sourceImageDeletionState: 'deleted',
      rawImageRetained: false,
      custodyState: 'child-device-journal',
    },
    memoryGraphRead,
    assertionLabels: [
      'sourceUsedRealServiceOcrPolicyArtifact',
      'graphReadUsedRealMemoryReader',
      'graphEdgesCiteSelectedEvidence',
      'graphEdgesCiteSelectedPolicy',
      'graphEdgesCiteSelectedAction',
      'rawImageNotRetained',
      'deletedImageCustodyPreserved',
      'remoteAiNotIntroduced',
    ],
    assertions: {
      sourceUsedRealServiceOcrPolicyArtifact: true,
      graphReadUsedRealMemoryReader: true,
      graphEdgesCiteSelectedEvidence: true,
      graphEdgesCiteSelectedPolicy: true,
      graphEdgesCiteSelectedAction: true,
      rawImageNotRetained: true,
      deletedImageCustodyPreserved: true,
      remoteAiNotIntroduced: true,
    },
  };
}
