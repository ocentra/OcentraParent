import { describe, expect, it } from 'vitest';
import {
  AgentEvent,
  AgentEventEnvelopeSchema,
  AgentProtocolDefaults,
} from '@ocentra-parent/agent-protocol-domain/contracts';
import { createLocalAiRuntimePanelIntent, parseActivityMemoryGraphReadModel, PortalDetails } from '../src/contracts';

describe('local AI runtime panel intent', () => {
  it('renders runtime and household job rows from real agent event envelopes', () => {
    const intent = createLocalAiRuntimePanelIntent(localAiRuntimeStatusEvent(), lanAiJobEvent());

    expect(intent.title).toBe('AI jobs and runtime activity');
    expect(intent.summaryDetails).toContainEqual({
      label: PortalDetails.Status,
      value: 'reported',
    });
    expect(intent.summaryDetails).toContainEqual({
      label: PortalDetails.ProductClaim,
      value: 'no-model-quality-or-enforcement-claim',
    });
    expect(intent.cards.map((card) => card.title)).toEqual(['Local AI runtime status', 'Household AI job activity']);
    expect(intent.cards[0]?.details).toContainEqual({
      label: PortalDetails.Model,
      value: 'screen-local-vlm-v1',
    });
    expect(intent.cards[1]?.details).toContainEqual({
      label: PortalDetails.Status,
      value: 'claimed',
    });
  });

  it('renders source-cited memory and graph evidence rows from the service read model', () => {
    const graph = parseActivityMemoryGraphReadModel(memoryGraphEvent().payload);
    const intent = createLocalAiRuntimePanelIntent(localAiRuntimeStatusEvent(), lanAiJobEvent(), graph);

    expect(intent.cards.map((card) => card.title)).toEqual([
      'Local AI runtime status',
      'Household AI job activity',
      'Cited memory and graph evidence',
    ]);
    expect(intent.cards[2]?.details).toContainEqual({
      label: PortalDetails.GraphEdges,
      value: '1',
    });
    expect(intent.cards[2]?.details).toContainEqual({
      label: PortalDetails.EvidenceReferences,
      value: 'evidence-screen-summary-1',
    });
    expect(intent.cards[2]?.details).toContainEqual({
      label: PortalDetails.ProductClaim,
      value: 'source-cited-memory-graph-read-model-only',
    });
  });

  it('keeps missing runtime/job events visible as no-data rather than success', () => {
    const intent = createLocalAiRuntimePanelIntent(null, null);

    expect(intent.cards).toEqual([]);
    expect(intent.emptyMessage).toBe('No local AI runtime or job event has been reported yet.');
    expect(intent.summaryDetails).toContainEqual({
      label: PortalDetails.Status,
      value: 'not-reported',
    });
  });
});

function localAiRuntimeStatusEvent() {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: 1,
    eventId: 'evt-local-ai-runtime',
    correlationId: 'cmd-local-ai-runtime',
    sentAt: '2026-06-07T19:15:00Z',
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    event: AgentEvent.LocalAiRuntimeStatusReported,
    severity: 'info',
    payload: {
      [AgentProtocolDefaults.Field.LocalAiRuntimeReferenceId]: 'runtime-child-device-1',
      [AgentProtocolDefaults.Field.LocalAiProviderId]: 'local-provider-1',
      [AgentProtocolDefaults.Field.LocalAiModelId]: 'screen-local-vlm-v1',
      [AgentProtocolDefaults.Field.LoadState]: 'loaded',
      [AgentProtocolDefaults.Field.LocalAiCapabilityFlags]: 'ocr,vision',
      [AgentProtocolDefaults.Field.LocalAiResourceClass]: 'gpu',
      [AgentProtocolDefaults.Field.LocalAiDegradedState]: 'ready',
      [AgentProtocolDefaults.Field.LocalAiPrivacyMode]: 'local-only',
      [AgentProtocolDefaults.Field.LocalAiExecutionState]: 'ready',
    },
    snapshot: null,
  });
}

function lanAiJobEvent() {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: 1,
    eventId: 'evt-lan-ai-job',
    correlationId: 'cmd-lan-ai-job',
    sentAt: '2026-06-07T19:15:02Z',
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    event: AgentEvent.LanAiJobReported,
    severity: 'info',
    payload: {
      [AgentProtocolDefaults.Field.LanAiJobId]: 'lan-ai-job-1',
      [AgentProtocolDefaults.Field.LanAiJobStatus]: 'claimed',
      [AgentProtocolDefaults.Field.LanAiJobState]: 'worker-running',
      [AgentProtocolDefaults.Field.LocalAiProviderId]: 'household-desktop-provider',
      [AgentProtocolDefaults.Field.LanAiProviderCustodyLabel]: 'local-lan-redacted',
      [AgentProtocolDefaults.Field.LocalAiExecutionState]: 'running',
    },
    snapshot: null,
  });
}

function memoryGraphEvent() {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: 1,
    eventId: 'evt-memory-graph',
    correlationId: 'cmd-memory-graph',
    sentAt: '2026-06-07T19:16:00Z',
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
      [AgentProtocolDefaults.Field.ActivityDigest]: JSON.stringify(memoryGraphDigest()),
    },
    snapshot: null,
  });
}

function memoryGraphDigest() {
  return {
    schemaVersion: 1,
    generatedAt: '2026-06-07T19:16:00Z',
    custody: 'child-device-query-store',
    capabilityStatus: 'ready',
    query: {
      queryId: 'query-memory-graph-1',
      queryKind: 'explain-evidence',
      childProfile: {
        childProfileId: 'child-profile-1',
        displayName: 'Child',
      },
      device: {
        deviceId: 'child-device-1',
        childProfileId: 'child-profile-1',
        label: 'Child desktop',
        platform: 'windows',
      },
      timeRange: {
        observedFrom: '2026-06-07T19:00:00Z',
        observedUntil: '2026-06-07T19:16:00Z',
      },
      asOf: '2026-06-07T19:16:00Z',
      limit: 10,
    },
    readAt: '2026-06-07T19:16:01Z',
    nodes: [memoryGraphNode('node-device', 'device', 'Child desktop')],
    edges: [memoryGraphEdge()],
    returnedNodeCount: 1,
    returnedEdgeCount: 1,
    omittedEdgeCount: 0,
    degradedReasons: [],
  };
}

function memoryGraphNode(nodeId: string, nodeKind: string, label: string) {
  return {
    graphId: 'activity-memory-v1',
    nodeId,
    nodeKind,
    label,
    childProfile: null,
    device: null,
    trace: memoryGraphTrace(),
  };
}

function memoryGraphEdge() {
  return {
    graphId: 'activity-memory-v1',
    edgeId: 'edge-screen-summary-1',
    edgeKind: 'derived-from-evidence',
    fromNodeId: 'node-device',
    toNodeId: 'node-device',
    observedFrom: '2026-06-07T19:00:00Z',
    observedUntil: '2026-06-07T19:16:00Z',
    durationMs: 960000,
    trace: memoryGraphTrace(),
  };
}

function memoryGraphTrace() {
  return {
    entryStatus: 'usable',
    sourceEvidenceReferences: [
      {
        evidenceReferenceId: 'evidence-screen-summary-1',
        kind: 'screen-summary',
        observedAt: '2026-06-07T19:00:00Z',
      },
    ],
    sourcePolicyVersion: null,
    sourceParentActionReferences: [],
    generatedAt: '2026-06-07T19:16:00Z',
    expiresAt: null,
    confidence: 0.91,
    derivedIndexVersion: 'activity-memory-v1',
    degradedReasons: [],
  };
}
