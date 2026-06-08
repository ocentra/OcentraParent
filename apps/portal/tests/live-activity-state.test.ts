import { describe, expect, it } from 'vitest';
import {
  AgentBrowserRuntimeCapabilityStatus,
  AgentBrowserRuntimeCustodyLabel,
  AgentBrowserRuntimeEventType,
  AgentBrowserRuntimePhase,
  AgentBrowserRuntimeQueryVisibility,
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

  it('rejects browser runtime event-chain streams that claim action execution', () => {
    const state = resolveLiveActivityState([
      browserRuntimeEventChainStreamEvent({
        actionIntentDispatchAttempts: 1,
      }),
    ]);

    expect(state.browserRuntimeEventChainStream).toBeNull();
  });
});

describe('portal browser runtime action-intent handoff state', () => {
  it('keeps prepared browser action-intent handoff refs visible without execution claims', () => {
    const state = resolveLiveActivityState([
      browserRuntimeEventChainStreamEvent({
        entries: [
          browserRuntimeStreamEntry(
            AgentBrowserRuntimeEventType.PolicyDecisionCompleted,
            'cmd-browser-runtime-stream-browser.policy-decision.completed',
            {
              capabilityStatus: AgentBrowserRuntimeCapabilityStatus.TabListOnly,
              queryVisibility: AgentBrowserRuntimeQueryVisibility.LiveLocal,
              degradedReason: null,
              exactUrlClaimed: true,
              policyPreviewId: 'browser-policy-preview-test',
              assistantActionIntentId: 'browser-action-intent-test',
              dryRun: true,
            }
          ),
        ],
        streamedEvents: 1,
        actionIntentCandidates: 1,
        actionIntentHandoffCandidates: 1,
        actionIntentHandoffOutboxRefs: ['browser-action-intent-outbox-ref-test'],
        actionIntentHandoffRefs: ['browser-action-intent-handoff-ref-test'],
      }),
    ]);

    expect(state.browserRuntimeEventChainStream?.actionIntentHandoffCandidates).toBe(1);
    expect(state.browserRuntimeEventChainStream?.actionIntentHandoffOutboxRefs).toEqual([
      'browser-action-intent-outbox-ref-test',
    ]);
    expect(state.browserRuntimeEventChainStream?.actionIntentHandoffRefs).toEqual([
      'browser-action-intent-handoff-ref-test',
    ]);
    expect(state.browserRuntimeEventChainStream?.actionIntentDispatchAttempts).toBe(0);
    expect(state.browserRuntimeEventChainStream?.actionIntentChildInterventionExecutions).toBe(0);
    expect(state.browserRuntimeEventChainStream?.actionIntentEnforcementExecutions).toBe(0);
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
  expect(stream?.actionIntentCandidates).toBe(0);
  expect(stream?.actionIntentHandoffCandidates).toBe(0);
  expect(stream?.actionIntentHandoffOutboxRefs).toEqual([]);
  expect(stream?.actionIntentHandoffRefs).toEqual([]);
  expect(stream?.actionIntentDispatchAttempts).toBe(0);
  expect(stream?.actionIntentAdapterExecutions).toBe(0);
  expect(stream?.actionIntentChildInterventionExecutions).toBe(0);
  expect(stream?.actionIntentEnforcementExecutions).toBe(0);
}

function expectBrowserRuntimeStreamEntries(state: ResolvedLiveActivityState): void {
  const stream = state.browserRuntimeEventChainStream;

  if (stream === null) {
    throw new Error('Expected browser runtime event-chain stream');
  }
  const firstPayload = stream.entries[0]?.payload;
  if (firstPayload === undefined) {
    throw new Error('Expected first browser runtime event-chain payload');
  }

  expect(stream.entries.map((entry) => entry.eventType)).toEqual([
    AgentBrowserRuntimeEventType.EvidenceObserved,
    AgentBrowserRuntimeEventType.EvidenceJournaled,
    AgentBrowserRuntimeEventType.AuditEntryCommitted,
    AgentBrowserRuntimeEventType.ReadModelProjected,
  ]);
  expect(firstPayload.exactUrlClaimed).toBe(false);
  expect(firstPayload.capabilityStatus).toBe(AgentBrowserRuntimeCapabilityStatus.BridgeMissing);
  expect(firstPayload.queryVisibility).toBe(AgentBrowserRuntimeQueryVisibility.Unavailable);
  expect(firstPayload.degradedReason).toBe('browser-bridge-no-page-targets');
  expect(firstPayload.interventionCommandAllowed).toBe(false);
  expect(firstPayload.phase).toBe(AgentBrowserRuntimePhase.EvidenceObserved);
}

function browserRuntimeEventChainStreamEvent(
  input: {
    readonly entries?: readonly ReturnType<typeof browserRuntimeStreamEntry>[];
    readonly streamedEvents?: number;
    readonly actionIntentCandidates?: number;
    readonly actionIntentHandoffCandidates?: number;
    readonly actionIntentHandoffOutboxRefs?: readonly string[];
    readonly actionIntentHandoffRefs?: readonly string[];
    readonly actionIntentDispatchAttempts?: number;
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
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentCandidates]: input.actionIntentCandidates ?? 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentHandoffCandidates]:
      input.actionIntentHandoffCandidates ?? 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentHandoffOutboxRefs]: JSON.stringify(
      input.actionIntentHandoffOutboxRefs ?? []
    ),
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentHandoffRefs]: JSON.stringify(
      input.actionIntentHandoffRefs ?? []
    ),
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentDispatchAttempts]: input.actionIntentDispatchAttempts ?? 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentAdapterExecutions]: 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentChildInterventionExecutions]: 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentEnforcementExecutions]: 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeEventChainStream]: JSON.stringify(entries),
  });
}

function browserRuntimeStreamEntry(
  eventType: AgentBrowserRuntimeEventType,
  eventRef: string,
  payloadOverrides: Partial<{
    readonly phase: AgentBrowserRuntimePhase;
    readonly capabilityStatus: AgentBrowserRuntimeCapabilityStatus;
    readonly custodyLabel: AgentBrowserRuntimeCustodyLabel;
    readonly queryVisibility: AgentBrowserRuntimeQueryVisibility;
    readonly degradedReason: string | null;
    readonly exactUrlClaimed: boolean;
    readonly aiAuthority: boolean;
    readonly policyPreviewId: string | null;
    readonly assistantActionIntentId: string | null;
    readonly dryRun: boolean;
  }> = {}
): Record<string, unknown> {
  return {
    [AgentProtocolDefaults.Field.EventType]: eventType,
    [AgentProtocolDefaults.Field.EventRef]: eventRef,
    [AgentProtocolDefaults.Field.Payload]: {
      phase: payloadOverrides.phase ?? browserRuntimePhaseForEventType(eventType),
      sourceRef: 'browser-runtime-source-ref',
      evidenceRef: 'browser-runtime-evidence-ref',
      capabilityStatus: payloadOverrides.capabilityStatus ?? AgentBrowserRuntimeCapabilityStatus.BridgeMissing,
      custodyLabel: payloadOverrides.custodyLabel ?? AgentBrowserRuntimeCustodyLabel.ChildDeviceLocal,
      queryVisibility: payloadOverrides.queryVisibility ?? AgentBrowserRuntimeQueryVisibility.Unavailable,
      degradedReason: payloadOverrides.degradedReason ?? 'browser-bridge-no-page-targets',
      journalRef: 'browser-runtime-journal-ref',
      aiRequestRef: null,
      aiAnalysisRef: null,
      policyEvaluationRef: null,
      policyDecisionRef: null,
      policyPreviewId: payloadOverrides.policyPreviewId ?? null,
      assistantActionIntentId: payloadOverrides.assistantActionIntentId ?? null,
      interventionCommandRef: null,
      interventionResultRef: null,
      auditEntryRef: 'browser-runtime-audit-ref',
      readModelRef: 'browser-runtime-read-model-ref',
      previousPhaseRef: 'browser-runtime-previous-phase-ref',
      exactUrlClaimed: payloadOverrides.exactUrlClaimed ?? false,
      aiAuthority: payloadOverrides.aiAuthority ?? false,
      policyAuthority: true,
      dryRun: payloadOverrides.dryRun ?? false,
      adapterDispatchClaimed: false,
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
