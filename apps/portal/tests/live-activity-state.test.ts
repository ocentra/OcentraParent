import { describe, expect, it } from 'vitest';
import {
  AgentBrowserRuntimeEventType,
  AgentBrowserRuntimePhase,
  AgentEvent,
  AgentEventEnvelopeSchema,
  AgentProtocolDefaults,
  type AgentEventEnvelope,
} from '@ocentra-parent/agent-protocol-domain/contracts';
import {
  AgentNetworkRuntimeEventSchemaVersion,
  AgentNetworkRuntimeEventType,
} from '@ocentra-parent/agent-protocol-domain/network-runtime-events';
import { resolveLiveActivityState } from '../src/live-activity-state';

type ResolvedLiveActivityState = ReturnType<typeof resolveLiveActivityState>;

const NoClaimBoundary = {
  exactUrlAvailable: false,
  decryptedHttpsPayloadAvailable: false,
  messageContentAvailable: false,
  searchQueryAvailable: false,
  adapterActionExecuted: false,
} as const;

const FlowObserved = {
  schemaVersion: AgentNetworkRuntimeEventSchemaVersion,
  flowEventRef: 'event.network.flow.observed.1',
  observedAt: '2026-06-08T22:45:00Z',
  deviceRef: 'device.child.windows-1',
  flowEvidenceRef: 'evidence.network.flow.1',
  custody: 'child-device-query-store',
  evidenceGrade: 'A',
  claimBoundary: NoClaimBoundary,
} as const;

describe('portal live activity network service state', () => {
  it('resolves network service events through typed parsers', () => {
    const liveActivity = resolveLiveActivityState([
      eventWithPayload(AgentEvent.NetworkRuntimeEventChainStreamReported, {
        [AgentProtocolDefaults.Field.NetworkRuntimeStreamedEvents]: 1,
        [AgentProtocolDefaults.Field.NetworkRuntimeEventChainStream]: JSON.stringify([
          {
            eventType: AgentNetworkRuntimeEventType.NetworkFlowObserved,
            payload: FlowObserved,
          },
        ]),
      }),
      eventWithPayload(AgentEvent.NetworkRemoteDeliveryStatusReported, {
        [AgentProtocolDefaults.Field.NetworkRemoteDeliveryStatus]: '{',
      }),
      eventWithPayload(AgentEvent.NetworkLiveCaptureStatusReported, {}),
      eventWithPayload(AgentEvent.NetworkLinuxNftablesLabStatusReported, {
        [AgentProtocolDefaults.Field.NetworkLinuxNftablesLabStatus]: '{',
      }),
      eventWithPayload(AgentEvent.NetworkWindowsFirewallLabStatusReported, {
        [AgentProtocolDefaults.Field.NetworkWindowsFirewallLabStatus]: '{',
      }),
      eventWithPayload(AgentEvent.NetworkWindowsWfpGateStatusReported, {
        [AgentProtocolDefaults.Field.NetworkWindowsWfpGateStatus]: '{',
      }),
    ]);

    expect(liveActivity.networkRuntimeEventChainEvent?.event).toBe('agent.network.runtime.event-chain.stream.reported');
    expect(liveActivity.networkRuntimeEventChainStream).toEqual({
      streamedEventCount: 1,
      invalidEventCount: 0,
      events: [
        {
          ok: true,
          eventType: 'network.flow.observed',
          value: FlowObserved,
        },
      ],
    });
    expect(liveActivity.networkRemoteDeliveryStatusEvent?.event).toBe('agent.network.remote-delivery.status.reported');
    expect(liveActivity.networkRemoteDeliveryStatusResult).toEqual({
      ok: false,
      reason: 'invalid-remote-delivery-status-json',
    });
    expect(liveActivity.networkLiveCaptureStatusEvent?.event).toBe('agent.network.live-capture.status.reported');
    expect(liveActivity.networkLiveCaptureStatusResult).toEqual({
      ok: false,
      reason: 'missing-live-capture-status',
    });
    expect(liveActivity.networkLinuxNftablesLabStatusEvent?.event).toBe(
      'agent.network.linux-nftables-lab.status.reported'
    );
    expect(liveActivity.networkLinuxNftablesLabStatusResult).toEqual({
      ok: false,
      reason: 'invalid-linux-nftables-lab-status-json',
    });
    expect(liveActivity.networkWindowsFirewallLabStatusEvent?.event).toBe(
      'agent.network.windows-firewall-lab.status.reported'
    );
    expect(liveActivity.networkWindowsFirewallLabStatusResult).toEqual({
      ok: false,
      reason: 'invalid-windows-firewall-lab-status-json',
    });
    expect(liveActivity.networkWindowsWfpGateStatusEvent?.event).toBe('agent.network.windows-wfp-gate.status.reported');
    expect(liveActivity.networkWindowsWfpGateStatusResult).toEqual({
      ok: false,
      reason: 'invalid-windows-wfp-gate-status-json',
    });
  });
});

describe('portal browser runtime event-chain state', () => {
  it('parses browser runtime event-chain stream payload without action overclaim', () => {
    const state = resolveLiveActivityState([browserRuntimeEventChainStreamEvent()]);

    expectBrowserRuntimeEventEnvelope(state);
    expectBrowserRuntimeStreamCounts(state);
    expectBrowserRuntimeStreamEntries(state);
  });

  it('rejects browser runtime event-chain stream entries when event type and phase drift', () => {
    const state = resolveLiveActivityState([
      browserRuntimeEventChainStreamEvent({
        entries: [
          browserRuntimeStreamEntry(
            AgentBrowserRuntimeEventType.EvidenceObserved,
            'cmd-browser-runtime-stream-browser.evidence.observed',
            {
              phase: AgentBrowserRuntimePhase.ReadModelProjected,
            }
          ),
        ],
        streamedEvents: 1,
      }),
    ]);

    expect(state.browserRuntimeEventChainStreamEvent?.event).toBe('agent.browser.runtime.event-chain.stream.reported');
    expect(state.browserRuntimeEventChainStream).toBeNull();
  });

  it('rejects browser runtime event-chain streams that claim AI authority in the portal', () => {
    const state = resolveLiveActivityState([
      browserRuntimeEventChainStreamEvent({
        entries: [
          browserRuntimeStreamEntry(
            AgentBrowserRuntimeEventType.AiAnalysisCompleted,
            'cmd-browser-runtime-stream-browser.ai-analysis.completed',
            {
              aiAuthority: true,
            }
          ),
        ],
        streamedEvents: 1,
      }),
    ]);

    expect(state.browserRuntimeEventChainStream).toBeNull();
  });

  it('rejects browser runtime event-chain streams when the count fields drift from entries', () => {
    const state = resolveLiveActivityState([browserRuntimeEventChainStreamEvent({ streamedEvents: 5 })]);

    expect(state.browserRuntimeEventChainStream).toBeNull();
  });
});

function expectBrowserRuntimeEventEnvelope(state: ResolvedLiveActivityState): void {
  expect(state.browserRuntimeEventChainStreamEvent?.event).toBe('agent.browser.runtime.event-chain.stream.reported');
}

function expectBrowserRuntimeStreamCounts(state: ResolvedLiveActivityState): void {
  const stream = state.browserRuntimeEventChainStream;

  expect(stream?.observedRows).toBe(1);
  expect(stream?.streamedEvents).toBe(4);
  expect(stream?.manualRequiredRows).toBe(1);
  expect(stream?.interventionCommandEvents).toBe(0);
  expect(stream?.readModelProjectionEvents).toBe(1);
}

function expectBrowserRuntimeStreamEntries(state: ResolvedLiveActivityState): void {
  const stream = state.browserRuntimeEventChainStream;

  expect(stream?.entries.map((entry) => entry.eventType)).toEqual([
    AgentBrowserRuntimeEventType.EvidenceObserved,
    AgentBrowserRuntimeEventType.EvidenceJournaled,
    AgentBrowserRuntimeEventType.AuditEntryCommitted,
    AgentBrowserRuntimeEventType.ReadModelProjected,
  ]);
  expect(stream?.entries.at(0)?.payload.exactUrlClaimed).toBe(false);
  expect(stream?.entries.at(0)?.payload.interventionCommandAllowed).toBe(false);
  expect(stream?.entries.at(0)?.payload.phase).toBe(AgentBrowserRuntimePhase.EvidenceObserved);
}

function browserRuntimeEventChainStreamEvent(
  input: {
    readonly entries?: readonly ReturnType<typeof browserRuntimeStreamEntry>[];
    readonly streamedEvents?: number;
  } = {}
): AgentEventEnvelope {
  const entries = input.entries ?? [
    browserRuntimeStreamEntry(
      AgentBrowserRuntimeEventType.EvidenceObserved,
      'cmd-browser-runtime-stream-browser.evidence.observed'
    ),
    browserRuntimeStreamEntry(
      AgentBrowserRuntimeEventType.EvidenceJournaled,
      'cmd-browser-runtime-stream-browser.evidence.journaled'
    ),
    browserRuntimeStreamEntry(
      AgentBrowserRuntimeEventType.AuditEntryCommitted,
      'cmd-browser-runtime-stream-browser.audit-entry.committed'
    ),
    browserRuntimeStreamEntry(
      AgentBrowserRuntimeEventType.ReadModelProjected,
      'cmd-browser-runtime-stream-browser.read-model.projected'
    ),
  ];

  return eventWithPayload(AgentEvent.BrowserRuntimeEventChainStreamReported, {
    [AgentProtocolDefaults.Field.BrowserRuntimeObservedRows]: 1,
    [AgentProtocolDefaults.Field.BrowserRuntimeStreamedEvents]: input.streamedEvents ?? entries.length,
    [AgentProtocolDefaults.Field.BrowserRuntimeFailedRows]: 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeExactUrlRows]: 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeManualRequiredRows]: 1,
    [AgentProtocolDefaults.Field.BrowserRuntimeInterventionCommandEvents]: 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeReadModelProjectionEvents]: 1,
    [AgentProtocolDefaults.Field.BrowserRuntimeEventChainStream]: JSON.stringify(entries),
  });
}

function browserRuntimeStreamEntry(
  eventType: AgentBrowserRuntimeEventType,
  eventRef: string,
  payloadOverrides: Partial<{
    readonly phase: AgentBrowserRuntimePhase;
    readonly aiAuthority: boolean;
  }> = {}
): Record<string, unknown> {
  return {
    [AgentProtocolDefaults.Field.EventType]: eventType,
    [AgentProtocolDefaults.Field.EventRef]: eventRef,
    [AgentProtocolDefaults.Field.Payload]: {
      phase: payloadOverrides.phase ?? browserRuntimePhaseForEventType(eventType),
      sourceRef: 'browser-runtime-source-ref',
      evidenceRef: 'browser-runtime-evidence-ref',
      journalRef: 'browser-runtime-journal-ref',
      aiRequestRef: null,
      aiAnalysisRef: null,
      policyEvaluationRef: null,
      policyDecisionRef: null,
      interventionCommandRef: null,
      interventionResultRef: null,
      auditEntryRef: 'browser-runtime-audit-ref',
      readModelRef: 'browser-runtime-read-model-ref',
      previousPhaseRef: 'browser-runtime-previous-phase-ref',
      exactUrlClaimed: false,
      aiAuthority: payloadOverrides.aiAuthority ?? false,
      policyAuthority: true,
      interventionCommandAllowed: false,
      observedAt: '2026-05-21T01:00:00Z',
    },
  };
}

function browserRuntimePhaseForEventType(eventType: AgentBrowserRuntimeEventType): AgentBrowserRuntimePhase {
  switch (eventType) {
    case AgentBrowserRuntimeEventType.EvidenceObserved:
      return AgentBrowserRuntimePhase.EvidenceObserved;
    case AgentBrowserRuntimeEventType.EvidenceJournaled:
      return AgentBrowserRuntimePhase.EvidenceJournaled;
    case AgentBrowserRuntimeEventType.AiAnalysisRequested:
      return AgentBrowserRuntimePhase.AiAnalysisRequested;
    case AgentBrowserRuntimeEventType.AiAnalysisCompleted:
      return AgentBrowserRuntimePhase.AiAnalysisCompleted;
    case AgentBrowserRuntimeEventType.PolicyEvaluationRequested:
      return AgentBrowserRuntimePhase.PolicyEvaluationRequested;
    case AgentBrowserRuntimeEventType.PolicyDecisionCompleted:
      return AgentBrowserRuntimePhase.PolicyDecisionCompleted;
    case AgentBrowserRuntimeEventType.InterventionCommandIssued:
      return AgentBrowserRuntimePhase.InterventionCommandIssued;
    case AgentBrowserRuntimeEventType.InterventionResultObserved:
      return AgentBrowserRuntimePhase.InterventionResultObserved;
    case AgentBrowserRuntimeEventType.AuditEntryCommitted:
      return AgentBrowserRuntimePhase.AuditEntryCommitted;
    case AgentBrowserRuntimeEventType.ReadModelProjected:
      return AgentBrowserRuntimePhase.ReadModelProjected;
  }
}

function eventWithPayload(
  event: AgentEventEnvelope['event'],
  payload: AgentEventEnvelope['payload']
): AgentEventEnvelope {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: AgentProtocolDefaults.SchemaVersion,
    eventId: 'portal-live-activity-event',
    correlationId: 'portal-live-activity-correlation',
    sentAt: '2026-06-08T22:45:00Z',
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: AgentProtocolDefaults.Peer.PortalDev,
    event,
    severity: 'info',
    payload,
    snapshot: null,
  });
}
