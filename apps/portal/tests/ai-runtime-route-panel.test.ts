import { describe, expect, it } from 'vitest';
import {
  AgentEvent,
  AgentEventEnvelopeSchema,
  AgentProtocolDefaults,
} from '@ocentra-parent/agent-protocol-domain/contracts';
import { PortalRoute } from '@ocentra-parent/portal-domain/contracts';
import { shouldRenderAiRuntimeRoute } from '../src/AiRuntimeRoutePanel';
import { resolveLiveActivityState } from '../src/live-activity-state';

describe('AI runtime route panel', () => {
  it('renders only on the AI runtime route', () => {
    expect(shouldRenderAiRuntimeRoute(PortalRoute.AiRuntime)).toBe(true);
    expect(shouldRenderAiRuntimeRoute(PortalRoute.Memory)).toBe(false);
    expect(shouldRenderAiRuntimeRoute(PortalRoute.Overview)).toBe(false);
  });

  it('selects real local AI runtime and household AI job events from live state', () => {
    const state = resolveLiveActivityState([localAiRuntimeStatusEvent(), lanAiJobEvent(), memoryGraphEvent()]);

    expect(state.localAiRuntimeStatusEvent?.event).toBe(AgentEvent.LocalAiRuntimeStatusReported);
    expect(state.localAiRuntimeStatusEvent?.payload[AgentProtocolDefaults.Field.LocalAiModelId]).toBe(
      'screen-local-vlm-v1'
    );
    expect(state.lanAiJobEvent?.event).toBe(AgentEvent.LanAiJobReported);
    expect(state.lanAiJobEvent?.payload[AgentProtocolDefaults.Field.LanAiJobStatus]).toBe('claimed');
    expect(state.activityMemoryGraphReadModel?.returnedEdgeCount).toBe(1);
    expect(state.activityMemoryGraphReadModel?.edges[0]?.trace.sourceEvidenceReferences[0]?.evidenceReferenceId).toBe(
      'evidence-screen-summary-1'
    );
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
    nodes: [memoryGraphNode()],
    edges: [memoryGraphEdge()],
    returnedNodeCount: 1,
    returnedEdgeCount: 1,
    omittedEdgeCount: 0,
    degradedReasons: [],
  };
}

function memoryGraphNode() {
  return {
    graphId: 'activity-memory-v1',
    nodeId: 'node-device',
    nodeKind: 'device',
    label: 'Child desktop',
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
