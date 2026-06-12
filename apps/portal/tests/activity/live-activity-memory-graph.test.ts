import { describe, expect, it } from 'vitest';
import { AgentEvent, AgentEventEnvelopeSchema } from '@ocentra-parent/agent-protocol-domain/contracts';
import { resolveLiveActivityState } from '../../src/live-activity-state';

describe('portal live activity memory graph state', () => {
  it('parses service memory graph digest only when edges cite evidence', () => {
    const state = resolveLiveActivityState([memoryGraphEvent(memoryGraphDigest())]);

    expect(state.activityMemoryGraphReadModel?.returnedEdgeCount).toBe(1);
    expect(state.activityMemoryGraphReadModel?.edges[0]?.edgeKind).toBe('visited');
    expect(state.activityMemoryGraphReadModel?.edges[0]?.trace.sourceEvidenceReferences[0]?.evidenceReferenceId).toBe(
      'activity-browser-url-observed-1'
    );
  });

  it('rejects uncited memory graph digests instead of rendering ungrounded memory', () => {
    const digest = memoryGraphDigest();
    const edge = digest.edges[0]!;
    const uncitedDigest = {
      ...digest,
      edges: [
        {
          ...edge,
          trace: {
            ...edge.trace,
            sourceEvidenceReferences: [],
            sourcePolicyVersion: null,
            sourceParentActionReferences: [],
          },
        },
      ],
    };
    const state = resolveLiveActivityState([memoryGraphEvent(uncitedDigest)]);

    expect(state.activityMemoryGraphReadModel).toBeNull();
  });
});

function memoryGraphEvent(digest: unknown) {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: 1,
    eventId: 'evt-memory-graph',
    correlationId: 'cmd-memory-graph',
    sentAt: '2026-05-21T02:00:01Z',
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    event: AgentEvent.ActivityMemoryGraphReported,
    severity: 'info',
    payload: {
      generatedAt: '2026-05-21T02:00:01Z',
      custody: 'child-device-activity-store',
      limit: 10,
      returned: 1,
      capabilityStatus: 'ready',
      activityDigest: JSON.stringify(digest),
    },
    snapshot: null,
  });
}

function memoryGraphDigest() {
  return {
    schemaVersion: 1,
    generatedAt: '2026-05-21T02:00:01Z',
    custody: 'child-device-activity-store',
    capabilityStatus: 'ready',
    query: memoryGraphQuery(),
    readAt: '2026-05-21T02:00:01Z',
    nodes: memoryGraphNodes(),
    edges: memoryGraphEdges(),
    returnedNodeCount: 2,
    returnedEdgeCount: 1,
    omittedEdgeCount: 0,
    degradedReasons: [],
  };
}

function memoryGraphQuery() {
  return {
    queryId: 'query-1',
    queryKind: 'activity-by-time-range',
    childProfile: null,
    device: deviceReference(),
    timeRange: {
      observedFrom: '2026-05-21T02:00:00Z',
      observedUntil: '2026-05-21T02:00:01Z',
    },
    asOf: '2026-05-21T02:00:01Z',
    limit: 10,
  };
}

function memoryGraphNodes() {
  return [
    memoryGraphNode('local-dev-agent', 'device', 'local-dev-agent'),
    memoryGraphNode('url-example', 'browser-url', 'https://example.test/learn'),
  ];
}

function memoryGraphNode(nodeId: string, nodeKind: string, label: string) {
  return {
    graphId: 'activity-memory-v1',
    nodeId,
    nodeKind,
    label,
    childProfile: null,
    device: deviceReference(),
    trace: citedTrace(),
  };
}

function memoryGraphEdges() {
  return [
    {
      graphId: 'activity-memory-v1',
      edgeId: 'activity-browser-url-observed-1',
      edgeKind: 'visited',
      fromNodeId: 'local-dev-agent',
      toNodeId: 'url-example',
      observedFrom: '2026-05-21T02:00:00Z',
      observedUntil: null,
      durationMs: null,
      trace: citedTrace(),
    },
  ];
}

function deviceReference() {
  return {
    deviceId: 'local-dev-agent',
    childProfileId: null,
    label: 'local-dev-agent',
    platform: 'windows',
  };
}

function citedTrace() {
  return {
    entryStatus: 'usable',
    sourceEvidenceReferences: [
      {
        evidenceReferenceId: 'activity-browser-url-observed-1',
        kind: 'activity-event',
        observedAt: '2026-05-21T02:00:00Z',
      },
    ],
    sourcePolicyVersion: null,
    sourceParentActionReferences: [],
    generatedAt: '2026-05-21T02:00:01Z',
    expiresAt: null,
    confidence: 1,
    derivedIndexVersion: 'activity-memory-v1',
    degradedReasons: [],
  };
}
