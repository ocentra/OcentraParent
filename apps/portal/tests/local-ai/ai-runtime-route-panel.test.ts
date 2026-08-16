import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it } from 'vitest';
import { parseActivityMemoryGraphReadModel } from '@ocentra-parent/portal-domain/contracts';
import {
  ParentActivityMemoryGraphEdgeKind,
  ParentActivityMemoryGraphEntryStatus,
  ParentActivityMemoryGraphNodeKind,
  type ParentActivityMemoryGraphReadModelSnapshot,
  ParentAgentEvent,
  type ParentAgentEventName,
  ParentAgentProtocolField,
  type ParentAgentProtocolPayload,
  ParentRoute,
  type ParentRouteEventSnapshot,
} from '../../generated/parent-ui-bridge';
import { AiRuntimeRoutePanel, shouldRenderAiRuntimeRoute } from '../../src/AiRuntimeRoutePanel';
import { EMPTY_ROUTE_LIVE_ACTIVITY_STATE } from '../../src/route-live-activity-state';

type AiRuntimeRouteEventFixture = ParentRouteEventSnapshot & {
  readonly correlationId: string;
  readonly event: ParentAgentEventName;
  readonly eventId: string;
  readonly payload: ParentAgentProtocolPayload;
  readonly sentAt: string;
  readonly severity: string;
  readonly sourcePeerId: string;
  readonly sourceRole: NonNullable<ParentRouteEventSnapshot['sourceRole']>;
  readonly snapshot: null;
  readonly targetPeerId: string;
  readonly targetRole: NonNullable<ParentRouteEventSnapshot['targetRole']>;
};

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
  return routeEvent({
    eventId: 'evt-local-ai-runtime',
    correlationId: 'cmd-local-ai-runtime',
    sentAt: '2026-06-07T19:15:00Z',
    event: ParentAgentEvent.LocalAiRuntimeStatusReported,
    payload: {
      [ParentAgentProtocolField.LocalAiRuntimeReferenceId]: 'runtime-child-device-1',
      [ParentAgentProtocolField.LocalAiProviderId]: 'local-provider-1',
      [ParentAgentProtocolField.LocalAiModelId]: 'screen-local-vlm-v1',
      [ParentAgentProtocolField.LoadState]: 'loaded',
    },
  });
}

function lanAiJobEvent() {
  return routeEvent({
    eventId: 'evt-lan-ai-job',
    correlationId: 'cmd-lan-ai-job',
    sentAt: '2026-06-07T19:15:02Z',
    event: ParentAgentEvent.LanAiJobReported,
    payload: {
      [ParentAgentProtocolField.LanAiJobId]: 'lan-ai-job-1',
      [ParentAgentProtocolField.LanAiJobStatus]: 'claimed',
      [ParentAgentProtocolField.LanAiJobState]: 'worker-running',
    },
  });
}

function parentAssistantBoundaryEvent() {
  return routeEvent({
    eventId: 'evt-parent-assistant-boundary',
    correlationId: 'cmd-parent-assistant-provider',
    sentAt: '2026-06-07T19:17:00Z',
    event: ParentAgentEvent.ParentAssistantAnswerReported,
    payload: {
      [ParentAgentProtocolField.ParentAssistantRequestId]: 'remote-assistant-request-1',
      [ParentAgentProtocolField.ParentAssistantApiProviderBoundary]: 'parent-authorized-report-bundle',
    },
  });
}

function memoryGraphEvent() {
  return routeEvent({
    eventId: 'evt-memory-graph',
    correlationId: 'cmd-memory-graph',
    sentAt: '2026-06-07T19:16:00Z',
    event: ParentAgentEvent.ActivityMemoryGraphReported,
    payload: {
      [ParentAgentProtocolField.ActivityDigest]: JSON.stringify(memoryGraphDigest()),
    },
  });
}

function routeEvent({
  correlationId,
  event,
  eventId,
  payload,
  sentAt,
}: {
  readonly correlationId: string;
  readonly event: ParentAgentEventName;
  readonly eventId: string;
  readonly payload: ParentAgentProtocolPayload;
  readonly sentAt: string;
}): AiRuntimeRouteEventFixture {
  return {
    correlationId,
    event,
    eventId,
    payload,
    sentAt,
    severity: 'info',
    sourcePeerId: 'local-dev-agent',
    sourceRole: 'agent-service',
    snapshot: null,
    targetPeerId: 'portal-dev',
    targetRole: 'portal',
  };
}

function memoryGraphDigest(): ParentActivityMemoryGraphReadModelSnapshot {
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
    nodeKind: ParentActivityMemoryGraphNodeKind.Device,
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
    edgeKind: ParentActivityMemoryGraphEdgeKind.DerivedFromEvidence,
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
    entryStatus: ParentActivityMemoryGraphEntryStatus.Usable,
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
