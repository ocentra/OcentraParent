import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it } from 'vitest';
import { AgentEvent, AgentEventEnvelopeSchema } from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import { AgentProtocolDefaults } from '@ocentra-parent/schema-domain/agent-protocol-defaults';
import { parseActivityMemoryGraphReadModel } from '@ocentra-parent/portal-domain/contracts';
import { ParentRoute } from '../../generated/parent-ui-bridge';
import { AiRuntimeRoutePanel, shouldRenderAiRuntimeRoute } from '../../src/AiRuntimeRoutePanel';
import { EMPTY_ROUTE_LIVE_ACTIVITY_STATE } from '../../src/route-live-activity-state';

describe('AI runtime route panel', () => {
  it('renders only on the AI runtime route', () => {
    expect(shouldRenderAiRuntimeRoute(ParentRoute.AiRuntime)).toBe(true);
    expect(shouldRenderAiRuntimeRoute(ParentRoute.Memory)).toBe(false);
    expect(shouldRenderAiRuntimeRoute(ParentRoute.Overview)).toBe(false);
  });

  it('renders local AI runtime, household AI job, cited memory, and remote assistant boundary details', () => {
    const markup = renderToStaticMarkup(
      createElement(AiRuntimeRoutePanel, {
        actions: aiRuntimeActions(),
        commandEnabled: true,
        liveActivity: routeLiveActivity(),
      })
    );

    expect(markup).toContain('AI jobs and runtime activity');
    expect(markup).toContain('Refresh local AI');
    expect(markup).toContain('screen-local-vlm-v1');
    expect(markup).toContain('claimed');
    expect(markup).toContain('evidence-screen-summary-1');
    expect(markup).toContain('parent-authorized-report-bundle');
    expect(markup).toContain('worker-only-child-agent-authority');
    expect(markup).toContain('source-cited-memory-graph-read-model-only');
    expect(markup).toContain('remote-assistant-report-only-local-policy-authority');
  });
});

function routeLiveActivity() {
  const activityMemoryGraphReadModel = parseActivityMemoryGraphReadModel(memoryGraphEvent().payload);
  if (activityMemoryGraphReadModel === null) {
    throw new Error('Expected activity memory graph read model fixture to parse.');
  }

  return {
    ...EMPTY_ROUTE_LIVE_ACTIVITY_STATE,
    localAiRuntimeStatusEvent: localAiRuntimeStatusEvent(),
    lanAiJobEvent: lanAiJobEvent(),
    activityMemoryGraphReadModel,
    parentAssistantBoundaryEvent: parentAssistantBoundaryEvent(),
  };
}

function aiRuntimeActions() {
  return {
    reconnect: () => undefined,
    selectCommandResult: () => undefined,
    sendCommand: async () => null,
    refreshRouteSnapshot: async () => null,
  };
}

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

function parentAssistantBoundaryEvent() {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: 1,
    eventId: 'evt-parent-assistant-boundary',
    correlationId: 'cmd-parent-assistant-provider',
    sentAt: '2026-06-07T19:17:00Z',
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    event: AgentEvent.ParentAssistantAnswerReported,
    severity: 'info',
    payload: {
      [AgentProtocolDefaults.Field.ParentAssistantRequestId]: 'remote-assistant-request-1',
      [AgentProtocolDefaults.Field.ParentAssistantApiProviderBoundary]: 'parent-authorized-report-bundle',
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
