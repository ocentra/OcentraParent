import { describe, expect, it } from 'vitest';
import {
  AgentEvent,
  AgentEventEnvelopeSchema,
  AgentProtocolDefaults,
} from '@ocentra-parent/agent-protocol-domain/contracts';
import { createLocalAiRuntimePanelIntent, parseActivityMemoryGraphReadModel, PortalDetails } from '../src/contracts';

describe('local AI runtime panel intent', () => {
  it('renders runtime and household job rows from real agent event envelopes', rendersRuntimeAndHouseholdRows);
  it('renders source-cited memory and graph evidence rows from the service read model', rendersMemoryGraphRows);
  it(
    'renders the parent-authorized remote assistant boundary without policy authority claims',
    rendersRemoteAssistantBoundary
  );
  it('keeps missing runtime/job events visible as no-data rather than success', rendersMissingEventState);
});

function rendersRuntimeAndHouseholdRows() {
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
  expect(intent.cards[1]?.details).toContainEqual({
    label: PortalDetails.ProviderSource,
    value: 'trusted-household-desktop',
  });
  expect(intent.cards[1]?.details).toContainEqual({
    label: PortalDetails.Capability,
    value: 'screen-hard-visual-analysis',
  });
  expect(intent.cards[1]?.details).toContainEqual({
    label: PortalDetails.PolicyReadiness,
    value: 'authorized-result',
  });
  expect(intent.cards[1]?.details).toContainEqual({
    label: PortalDetails.DecisionSource,
    value: 'child-agent-local-policy-authority',
  });
  expect(intent.cards[1]?.details).toContainEqual({
    label: PortalDetails.ProductClaim,
    value: 'worker-only-child-agent-authority',
  });
}

function rendersMemoryGraphRows() {
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
}

function rendersRemoteAssistantBoundary() {
  const graph = parseActivityMemoryGraphReadModel(memoryGraphEvent().payload);
  const intent = createLocalAiRuntimePanelIntent(
    localAiRuntimeStatusEvent(),
    lanAiJobEvent(),
    graph,
    parentAssistantBoundaryEvent()
  );

  expect(intent.cards.map((card) => card.title)).toEqual([
    'Local AI runtime status',
    'Household AI job activity',
    'Cited memory and graph evidence',
    'Remote assistant boundary',
  ]);
  expect(intent.cards[3]?.details).toContainEqual({
    label: PortalDetails.AdapterBoundary,
    value: 'parent-authorized-report-bundle',
  });
  expect(intent.cards[3]?.details).toContainEqual({
    label: PortalDetails.PolicyReadiness,
    value: 'parent-authorized',
  });
  expect(intent.cards[3]?.details).toContainEqual({
    label: PortalDetails.Custody,
    value: 'parent-owned-local-storage',
  });
  expect(intent.cards[3]?.details).toContainEqual({
    label: PortalDetails.DeletedEvidence,
    value: 'raw-model-output-not-retained',
  });
  expect(intent.cards[3]?.details).toContainEqual({
    label: PortalDetails.ProductClaim,
    value: 'remote-assistant-report-only-local-policy-authority',
  });
}

function rendersMissingEventState() {
  const intent = createLocalAiRuntimePanelIntent(null, null);

  expect(intent.cards).toEqual([]);
  expect(intent.emptyMessage).toBe('No local AI runtime or job event has been reported yet.');
  expect(intent.summaryDetails).toContainEqual({
    label: PortalDetails.Status,
    value: 'not-reported',
  });
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
      [AgentProtocolDefaults.Field.LocalAiProviderSource]: 'trusted-household-desktop',
      [AgentProtocolDefaults.Field.LocalAiCapabilityFlags]: 'screen-hard-visual-analysis',
      [AgentProtocolDefaults.Field.LocalAiResourceClass]: 'gpu',
      [AgentProtocolDefaults.Field.LocalAiAdapterReadinessState]: 'ready',
      [AgentProtocolDefaults.Field.LocalAiPrivacyMode]: 'local-lan-redacted',
      [AgentProtocolDefaults.Field.LanAiProviderCustodyLabel]: 'local-lan-redacted',
      [AgentProtocolDefaults.Field.LanAiProviderRoutingState]: 'authorized-result',
      [AgentProtocolDefaults.Field.ClaimBoundary]: 'claim-lease-child-owned-job',
      [AgentProtocolDefaults.Field.LanControllerLeaseId]: 'lease-screen-ai-1',
      [AgentProtocolDefaults.Field.LanControllerLeaseIssuedAt]: '2026-06-07T19:15:01Z',
      [AgentProtocolDefaults.Field.LanControllerLeaseExpiresAt]: '2026-06-07T19:20:01Z',
      [AgentProtocolDefaults.Field.LanParentAuthority]: 'child-agent-local-policy-authority',
      [AgentProtocolDefaults.Field.LocalAiExecutionState]: 'running',
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
      [AgentProtocolDefaults.Field.ParentAssistantAnswerState]: 'ready-answer',
      [AgentProtocolDefaults.Field.ParentAssistantProviderRoute]: 'remote-api-report-only',
      [AgentProtocolDefaults.Field.ParentAssistantApiProviderBoundary]: 'parent-authorized-report-bundle',
      [AgentProtocolDefaults.Field.ParentAssistantApiAuthorizationState]: 'parent-authorized',
      [AgentProtocolDefaults.Field.ParentAssistantApiCustodyLabel]: 'parent-owned-local-storage',
      [AgentProtocolDefaults.Field.ParentAssistantApiDeletionState]: 'raw-model-output-not-retained',
      [AgentProtocolDefaults.Field.ParentAssistantApiRetentionState]: 'report-summary-only',
      [AgentProtocolDefaults.Field.ParentAssistantEvidenceSummary]: 'evidence-screen-summary-1',
      [AgentProtocolDefaults.Field.ParentAssistantCitationCount]: '1',
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
