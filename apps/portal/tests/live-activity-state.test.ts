import { describe, expect, it } from 'vitest';
import { ActivitySurfaceSchemaVersion } from '@ocentra-parent/activity-domain/activity-surface';
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
  type AgentEventName,
} from '@ocentra-parent/agent-protocol-domain/contracts';
import {
  AgentNetworkRuntimeEventSchemaVersion,
  AgentNetworkRuntimeEventType,
} from '@ocentra-parent/agent-protocol-domain/network-runtime-events';
import { resolveLiveActivityState } from '../src/live-activity-state';

type ResolvedLiveActivityState = ReturnType<typeof resolveLiveActivityState>;
type BrowserEvidenceReadModel = NonNullable<ResolvedLiveActivityState['browserEvidenceReadModel']>;
type BrowserEvidenceRow = BrowserEvidenceReadModel['rows'][number];
type BrowserRuntimeEventChainStream = NonNullable<ResolvedLiveActivityState['browserRuntimeEventChainStream']>;
type BrowserRuntimeStreamEntry = ReturnType<typeof browserRuntimeStreamEntry>;
type BrowserRuntimeEventChainStreamEventInput = {
  readonly entries?: readonly BrowserRuntimeStreamEntry[];
  readonly streamedEvents?: number;
  readonly actionIntentCandidates?: number;
  readonly actionIntentHandoffCandidates?: number;
  readonly actionIntentHandoffOutboxRefs?: readonly string[];
  readonly actionIntentHandoffRefs?: readonly string[];
  readonly actionIntentChildAcceptedRows?: number;
  readonly actionIntentChildCommandRefs?: readonly string[];
  readonly actionIntentChildAcceptedEventRefs?: readonly string[];
  readonly actionIntentParentReadModelRefs?: readonly string[];
  readonly actionIntentDispatchAttempts?: number;
  readonly socialProviderReceiptBoundaryRows?: number;
  readonly socialProviderDispatchRequiredRows?: number;
  readonly socialProviderManualReceiptRequiredRows?: number;
  readonly socialProviderAttemptRefs?: readonly string[];
  readonly socialProviderReceiptProofRefs?: readonly string[];
  readonly socialProviderDurableRows?: number;
  readonly socialProviderDurableResultRefs?: readonly string[];
  readonly socialProviderDurableStoreRefs?: readonly string[];
  readonly socialProviderReadModelRefs?: readonly string[];
  readonly socialProviderSupportStatusRefs?: readonly string[];
};

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

describe('portal live activity state', () => {
  it('parses real service ingest and recent-summary payload fields', () => {
    const state = resolveLiveActivityState([browserEvidenceEvent(), recentSummaryEvent(), ingestStatusEvent()]);

    expectIngestStatus(state.ingestStatus);
    expectRecentSummary(state.recentSummary);
    expectBrowserEvidenceReadModel(state.browserEvidenceReadModel);
  });

  it('keeps unavailable activity-store responses visible without inventing rows', () => {
    const state = resolveLiveActivityState([unavailableRecentSummaryEvent()]);

    expect(state.ingestStatus).toBeNull();
    expect(state.recentSummary).toBeNull();
    expect(state.recentSummaryEvent?.severity).toBe('error');
    expect(state.recentSummaryEvent?.payload['reason']).toBe('Activity store is unavailable.');
  });

  it('keeps empty browser evidence summaries visible without inventing a URL', () => {
    const state = resolveLiveActivityState([emptyBrowserEvidenceEvent()]);

    expect(state.browserEvidenceReadModel?.returned).toBe(0);
    expect(state.browserEvidenceReadModel?.rows.length).toBe(0);
    expect(state.browserEvidenceReadModel?.capabilityStatus).toBeNull();
  });

  it('parses browser inventory read-model payload fields without exact URL overclaim', () => {
    const state = resolveLiveActivityState([browserInventoryEvent()]);
    const latestRow = state.browserInventoryReadModel?.rows.at(0);

    expect(state.browserInventoryReadModel?.returned).toBe(1);
    expect(state.browserInventoryReadModel?.latestObservedAt).toBe('2026-05-21T01:00:00Z');
    expect(latestRow?.browserFamily).toBe('edge');
    expect(latestRow?.productName).toBe('Microsoft Edge');
    expect(latestRow?.scannedAt).toBe('2026-05-21T01:00:00Z');
    expect(latestRow?.exactUrlCapability).toBe('managed-target-list-only');
    expect(latestRow?.activeTabCapability).toBe('target-list-only');
    expect(latestRow?.unmanagedFallbackCapability).toBe('report-only');
    expect(latestRow?.publisherSignatureRef).toBeNull();
    expect(latestRow?.fileHashRef).toBeNull();
  });

  it('uses the latest matching events for portal live activity state', () => {
    const state = resolveLiveActivityState([
      browserEvidenceEvent('evt-browser-earlier', 'https://earlier.example/learn', '2026-05-21T01:00:01Z'),
      activityReportEvent({
        eventId: 'evt-report-earlier',
        event: 'agent.activity.report.generated',
        reportId: 'activity-report-earlier',
        sentAt: '2026-05-21T01:00:01Z',
      }),
      browserEvidenceEvent('evt-browser-latest', 'https://latest.example/learn', '2026-05-21T01:00:02Z'),
      activityReportEvent({
        eventId: 'evt-report-latest',
        event: 'agent.activity.report.saved',
        reportId: 'activity-report-latest',
        sentAt: '2026-05-21T01:00:02Z',
      }),
    ]);

    expect(state.browserEvidenceEvent?.eventId).toBe('evt-browser-latest');
    expect(state.browserEvidenceReadModel?.rows.at(0)?.url).toBe('https://latest.example/learn');
    expect(state.activityReportEvent?.eventId).toBe('evt-report-latest');
    expect(state.activityReport?.ok ? state.activityReport.value.reportId : null).toBe('activity-report-latest');
  });
});

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

  it('rejects browser runtime child status refs when accepted row counts drift', () => {
    const state = resolveLiveActivityState([
      browserRuntimeEventChainStreamEvent({
        actionIntentChildAcceptedRows: 1,
        actionIntentChildCommandRefs: ['browser-child-command-ref-test'],
        actionIntentChildAcceptedEventRefs: [],
        actionIntentParentReadModelRefs: ['browser-parent-read-model-ref-test'],
      }),
    ]);

    expect(state.browserRuntimeEventChainStream).toBeNull();
  });
});

describe('portal browser runtime social provider receipt state', () => {
  it('projects social provider receipt stream and ingestion readiness status from parsed state', () => {
    const state = resolveLiveActivityState([
      browserRuntimeEventChainStreamEvent({
        socialProviderReceiptBoundaryRows: 1,
        socialProviderDispatchRequiredRows: 1,
        socialProviderAttemptRefs: ['browser-social-provider-attempt-ref-test'],
        socialProviderReceiptProofRefs: ['browser-social-provider-receipt-proof-ref-test'],
        socialProviderDurableRows: 1,
        socialProviderDurableResultRefs: ['browser-social-provider-durable-result-ref-test'],
        socialProviderDurableStoreRefs: ['browser-social-provider-durable-store-ref-test'],
        socialProviderReadModelRefs: ['browser-social-provider-read-model-ref-test'],
        socialProviderSupportStatusRefs: ['browser-social-provider-support-status-ref-test'],
      }),
    ]);

    expect(state.browserRuntimeEventChainStream?.socialProviderReceiptBoundaryRows).toBe(1);
    expect(state.browserSocialProviderReceiptStreamStatusIntent?.summary).toBe('1 receipt boundary rows');
    expect(state.browserSocialProviderReceiptStreamStatusIntent?.productClaim).toContain(
      'enforcement remain unclaimed'
    );
    expect(state.browserSocialProviderReceiptIngestionReadinessStatusIntent?.summary).toBe('1 readiness rows');
    expect(state.browserSocialProviderReceiptIngestionReadinessStatusIntent?.productClaim).toContain(
      'receipt ingestion runtime'
    );
    expect(
      state.browserSocialProviderReceiptIngestionReadinessStatusIntent?.details.some(
        (detail) => detail.value === 'ingestion-contract-required'
      )
    ).toBe(true);
  });

  it('rejects dishonest social provider receipt stream rows before projecting portal status', () => {
    const state = resolveLiveActivityState([
      browserRuntimeEventChainStreamEvent({
        socialProviderReceiptBoundaryRows: 1,
        socialProviderManualReceiptRequiredRows: 1,
        socialProviderDurableRows: 1,
      }),
    ]);

    expect(state.browserRuntimeEventChainStream).toBeNull();
    expect(state.browserSocialProviderReceiptStreamStatusIntent).toBeNull();
    expect(state.browserSocialProviderReceiptIngestionReadinessStatusIntent).toBeNull();
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
        actionIntentChildAcceptedRows: 0,
        actionIntentChildCommandRefs: [],
        actionIntentChildAcceptedEventRefs: [],
        actionIntentParentReadModelRefs: [],
      }),
    ]);

    expect(state.browserRuntimeEventChainStream?.actionIntentHandoffCandidates).toBe(1);
    expect(state.browserRuntimeEventChainStream?.actionIntentHandoffOutboxRefs).toEqual([
      'browser-action-intent-outbox-ref-test',
    ]);
    expect(state.browserRuntimeEventChainStream?.actionIntentHandoffRefs).toEqual([
      'browser-action-intent-handoff-ref-test',
    ]);
    expect(state.browserRuntimeEventChainStream?.actionIntentChildAcceptedRows).toBe(0);
    expect(state.browserRuntimeEventChainStream?.actionIntentChildCommandRefs).toEqual([]);
    expect(state.browserRuntimeEventChainStream?.actionIntentChildAcceptedEventRefs).toEqual([]);
    expect(state.browserRuntimeEventChainStream?.actionIntentParentReadModelRefs).toEqual([]);
    expect(state.browserRuntimeEventChainStream?.actionIntentDispatchAttempts).toBe(0);
    expect(state.browserRuntimeEventChainStream?.actionIntentChildInterventionExecutions).toBe(0);
    expect(state.browserRuntimeEventChainStream?.actionIntentEnforcementExecutions).toBe(0);
  });
});

function expectIngestStatus(ingestStatus: ResolvedLiveActivityState['ingestStatus']) {
  expect(ingestStatus).not.toBeNull();
  if (ingestStatus === null) {
    return;
  }
  expect(ingestStatus.databaseReady).toBe(true);
  expect(ingestStatus.eventsStored).toBe(1);
}

function expectRecentSummary(recentSummary: ResolvedLiveActivityState['recentSummary']) {
  expect(recentSummary).not.toBeNull();
  if (recentSummary === null) {
    return;
  }
  expect(recentSummary.returned).toBe(1);
  expect(recentSummary.mostRecentSubjectName).toBe('notepad.exe');
}

function expectBrowserEvidenceReadModel(readModel: ResolvedLiveActivityState['browserEvidenceReadModel']) {
  expect(readModel).not.toBeNull();
  if (readModel === null) {
    return;
  }
  expect(readModel.returned).toBe(1);
  expect(readModel.capabilityStatus).toBe('tab-list-only');
  expectBrowserEvidenceRow(readModel.rows[0]);
}

function expectBrowserEvidenceRow(latestRow: BrowserEvidenceRow | undefined) {
  expect(latestRow).toBeDefined();
  if (latestRow === undefined) {
    return;
  }
  expect(latestRow.url).toBe('https://example.test/learn');
  expect(latestRow.activeState).toBe('unknown');
  expect(latestRow.activeProofSource).toBe('target-list-only');
}

function expectBrowserRuntimeEventEnvelope(state: ResolvedLiveActivityState) {
  expect(state.browserRuntimeEventChainStreamEvent?.event).toBe('agent.browser.runtime.event-chain.stream.reported');
}

function expectBrowserRuntimeStreamCounts(state: ResolvedLiveActivityState) {
  const stream = browserRuntimeStreamOrThrow(state);

  expect(stream.observedRows).toBe(1);
  expect(stream.streamedEvents).toBe(4);
  expect(stream.manualRequiredRows).toBe(1);
  expect(stream.interventionCommandEvents).toBe(0);
  expect(stream.readModelProjectionEvents).toBe(1);
  expect(stream.actionIntentCandidates).toBe(0);
  expect(stream.actionIntentHandoffCandidates).toBe(0);
  expect(stream.actionIntentHandoffOutboxRefs).toEqual([]);
  expect(stream.actionIntentHandoffRefs).toEqual([]);
  expect(stream.actionIntentChildAcceptedRows).toBe(0);
  expect(stream.actionIntentChildCommandRefs).toEqual([]);
  expect(stream.actionIntentChildAcceptedEventRefs).toEqual([]);
  expect(stream.actionIntentParentReadModelRefs).toEqual([]);
  expect(stream.actionIntentDispatchAttempts).toBe(0);
  expect(stream.actionIntentAdapterExecutions).toBe(0);
  expect(stream.actionIntentChildInterventionExecutions).toBe(0);
  expect(stream.actionIntentEnforcementExecutions).toBe(0);
}

function expectBrowserRuntimeStreamEntries(state: ResolvedLiveActivityState) {
  const stream = browserRuntimeStreamOrThrow(state);
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

function browserRuntimeStreamOrThrow(state: ResolvedLiveActivityState): BrowserRuntimeEventChainStream {
  const stream = state.browserRuntimeEventChainStream;
  if (stream === null) {
    throw new Error('Expected browser runtime event-chain stream');
  }
  return stream;
}

function recentSummaryEvent() {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: 1,
    eventId: 'evt-recent',
    correlationId: 'cmd-recent',
    sentAt: '2026-05-20T18:45:01Z',
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    event: 'agent.activity.recent.summary.reported',
    severity: 'info',
    payload: {
      limit: 25,
      returned: 1,
      firstObservedAt: '2026-05-20T18:44:59Z',
      lastObservedAt: '2026-05-20T18:44:59Z',
      lastEventId: 'activity-event-1',
      mostRecentKind: 'activity.process.observed',
      mostRecentObserver: 'windows-process',
      mostRecentSubjectKind: 'process',
      mostRecentSubjectId: 'process-1',
      mostRecentSubjectName: 'notepad.exe',
    },
    snapshot: null,
  });
}

function ingestStatusEvent() {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: 1,
    eventId: 'evt-ingest',
    correlationId: 'cmd-ingest',
    sentAt: '2026-05-20T18:45:00Z',
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    event: 'agent.activity.ingest.status.reported',
    severity: 'info',
    payload: {
      databaseReady: true,
      eventsIngested: 0,
      eventsStored: 1,
      duplicateEvents: 0,
      lastEventId: 'activity-event-1',
    },
    snapshot: null,
  });
}

function browserEvidenceEvent(
  eventId = 'evt-browser',
  url = 'https://example.test/learn',
  sentAt = '2026-05-21T01:00:01Z'
) {
  const origin = new URL(url).origin;
  const domain = new URL(url).hostname;

  return AgentEventEnvelopeSchema.parse({
    schemaVersion: 1,
    eventId,
    correlationId: 'cmd-browser',
    sentAt,
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    event: 'agent.browser.evidence.recent.reported',
    severity: 'info',
    payload: {
      generatedAt: '2026-05-21T01:00:01Z',
      limit: 10,
      returned: 1,
      latestEventId: 'activity-browser-url-observed-1',
      latestObservedAt: '2026-05-21T01:00:00Z',
      browserEvidenceId: 'browser-evidence-1',
      sourceId: 'managed-chromium-devtools',
      adapterId: 'managed-chromium-devtools-adapter',
      managedBrowserSessionId: 'managed-browser-session-1',
      browserFamily: 'edge',
      browserChannel: 'stable',
      profileId: 'managed-browser-profile-dev',
      processId: 4242,
      windowId: null,
      tabId: null,
      targetId: 'target-1',
      activeState: 'unknown',
      activeProofSource: 'target-list-only',
      url,
      origin,
      domain,
      title: 'Example learning page',
      freshUntil: '2026-05-21T01:00:30Z',
      staleAt: '2026-05-21T01:00:30Z',
      capabilityStatus: 'tab-list-only',
      custodyLabel: 'child-device-local',
      queryVisibility: 'live-local',
    },
    snapshot: null,
  });
}

function activityReportEvent(input: {
  readonly eventId: unknown;
  readonly event: AgentEventName;
  readonly reportId: unknown;
  readonly sentAt?: string;
}) {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: 1,
    eventId: input.eventId,
    correlationId: 'cmd-report',
    sentAt: input.sentAt ?? '2026-05-21T01:00:01Z',
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    event: input.event,
    severity: 'info',
    payload: {
      [AgentProtocolDefaults.Field.ActivitySurfaceState]: 'ready',
      [AgentProtocolDefaults.Field.ActivityReportDocument]: JSON.stringify({
        schemaVersion: ActivitySurfaceSchemaVersion,
        reportId: input.reportId,
        frequency: 'daily',
        scope: {
          scopeKind: 'device',
          familyId: null,
          deviceId: 'local-dev-agent',
        },
        requestedAt: '2026-05-21T01:00:00Z',
        rangeStart: '2026-05-21T00:00:00Z',
        rangeEnd: '2026-05-21T01:00:00Z',
        generatedAt: '2026-05-21T01:00:01Z',
        savedMetadata: null,
        sourceStates: [
          {
            deviceId: 'local-dev-agent',
            reachabilityState: 'reachable',
            state: 'ready',
            reason: null,
            lastUpdatedAt: '2026-05-21T01:00:00Z',
          },
        ],
        sections: [
          {
            sectionKind: 'summary',
            title: 'Summary',
            state: 'ready',
            summary: 'Activity data is available from the local query store.',
            itemCount: 1,
            evidence: [],
          },
        ],
      }),
    },
    snapshot: null,
  });
}

function browserInventoryEvent() {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: 1,
    eventId: 'evt-browser-inventory',
    correlationId: 'cmd-browser-inventory',
    sentAt: '2026-05-21T01:00:01Z',
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    event: 'agent.browser.inventory.read-model.reported',
    severity: 'info',
    payload: {
      generatedAt: '2026-05-21T01:00:01Z',
      limit: 20,
      returned: 1,
      latestObservedAt: '2026-05-21T01:00:00Z',
      capabilityStatus: 'tab-list-only',
      custodyLabel: 'child-device-local',
      queryVisibility: 'live-local',
      browserInventoryRowId: 'browser-inventory-row-1',
      browserFamily: 'edge',
      browserChannel: 'stable',
      productName: 'Microsoft Edge',
      browserVersion: '124.0.0.0',
      profileId: 'managed-browser-profile-dev',
      processId: 4242,
      executablePathRef: 'managed-edge-path-ref',
      installState: 'installed',
      runningState: 'running-managed',
      managementTier: 'managed',
      supportTier: 'managed-target-list',
      exactUrlCapability: 'managed-target-list-only',
      activeTabCapability: 'target-list-only',
      managedProfileState: 'ready',
      unmanagedFallbackCapability: 'report-only',
      reason: 'managed-target-list-only',
      publisherSignatureRef: null,
      fileHashRef: null,
    },
    snapshot: null,
  });
}

function browserRuntimeEventChainStreamEvent(input: BrowserRuntimeEventChainStreamEventInput = {}) {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: 1,
    eventId: 'evt-browser-runtime-stream',
    correlationId: 'cmd-browser-runtime-stream',
    sentAt: '2026-05-21T01:00:02Z',
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    event: 'agent.browser.runtime.event-chain.stream.reported',
    severity: 'info',
    payload: browserRuntimeEventChainStreamPayload(input),
    snapshot: null,
  });
}

function browserRuntimeEventChainStreamPayload(input: BrowserRuntimeEventChainStreamEventInput) {
  const entries = input.entries ?? defaultBrowserRuntimeStreamEntries();
  return {
    ...browserRuntimeCounterPayload(input, entries),
    ...browserRuntimeActionIntentPayload(input),
    ...browserRuntimeSocialProviderReceiptPayload(input),
    [AgentProtocolDefaults.Field.BrowserRuntimeEventChainStream]: JSON.stringify(entries),
  };
}

function defaultBrowserRuntimeStreamEntries(): readonly BrowserRuntimeStreamEntry[] {
  return [
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
}

function browserRuntimeCounterPayload(
  input: BrowserRuntimeEventChainStreamEventInput,
  entries: readonly BrowserRuntimeStreamEntry[]
) {
  return {
    [AgentProtocolDefaults.Field.BrowserRuntimeObservedRows]: 1,
    [AgentProtocolDefaults.Field.BrowserRuntimeStreamedEvents]: input.streamedEvents ?? entries.length,
    [AgentProtocolDefaults.Field.BrowserRuntimeFailedRows]: 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeExactUrlRows]: 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeManualRequiredRows]: 1,
    [AgentProtocolDefaults.Field.BrowserRuntimeInterventionCommandEvents]: 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeReadModelProjectionEvents]: 1,
  };
}

function browserRuntimeActionIntentPayload(input: BrowserRuntimeEventChainStreamEventInput) {
  return {
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentCandidates]: input.actionIntentCandidates ?? 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentHandoffCandidates]: input.actionIntentHandoffCandidates ?? 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentHandoffOutboxRefs]: JSON.stringify(
      input.actionIntentHandoffOutboxRefs ?? []
    ),
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentHandoffRefs]: JSON.stringify(
      input.actionIntentHandoffRefs ?? []
    ),
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentChildAcceptedRows]: input.actionIntentChildAcceptedRows ?? 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentChildCommandRefs]: JSON.stringify(
      input.actionIntentChildCommandRefs ?? []
    ),
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentChildAcceptedEventRefs]: JSON.stringify(
      input.actionIntentChildAcceptedEventRefs ?? []
    ),
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentParentReadModelRefs]: JSON.stringify(
      input.actionIntentParentReadModelRefs ?? []
    ),
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentDispatchAttempts]: input.actionIntentDispatchAttempts ?? 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentAdapterExecutions]: 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentChildInterventionExecutions]: 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeActionIntentEnforcementExecutions]: 0,
  };
}

function browserRuntimeSocialProviderReceiptPayload(input: BrowserRuntimeEventChainStreamEventInput) {
  return {
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderReceiptBoundaryRows]:
      input.socialProviderReceiptBoundaryRows ?? 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderDispatchRequiredRows]:
      input.socialProviderDispatchRequiredRows ?? 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderManualReceiptRequiredRows]:
      input.socialProviderManualReceiptRequiredRows ?? 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderAttemptRefs]: JSON.stringify(
      input.socialProviderAttemptRefs ?? []
    ),
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderReceiptProofRefs]: JSON.stringify(
      input.socialProviderReceiptProofRefs ?? []
    ),
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderDurableRows]: input.socialProviderDurableRows ?? 0,
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderDurableResultRefs]: JSON.stringify(
      input.socialProviderDurableResultRefs ?? []
    ),
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderDurableStoreRefs]: JSON.stringify(
      input.socialProviderDurableStoreRefs ?? []
    ),
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderReadModelRefs]: JSON.stringify(
      input.socialProviderReadModelRefs ?? []
    ),
    [AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderSupportStatusRefs]: JSON.stringify(
      input.socialProviderSupportStatusRefs ?? []
    ),
  };
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
) {
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

function emptyBrowserEvidenceEvent() {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: 1,
    eventId: 'evt-browser',
    correlationId: 'cmd-browser',
    sentAt: '2026-05-21T01:00:01Z',
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    event: 'agent.browser.evidence.recent.reported',
    severity: 'info',
    payload: {
      generatedAt: '2026-05-21T01:00:01Z',
      limit: 10,
      returned: 0,
      latestEventId: null,
      latestObservedAt: null,
      browserEvidenceId: null,
      sourceId: null,
      adapterId: null,
      managedBrowserSessionId: null,
      browserFamily: null,
      activeState: null,
      activeProofSource: null,
      url: null,
      origin: null,
      domain: null,
      title: null,
      capabilityStatus: null,
      custodyLabel: null,
      queryVisibility: null,
    },
    snapshot: null,
  });
}

function unavailableRecentSummaryEvent() {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: 1,
    eventId: 'evt-recent',
    correlationId: 'cmd-recent',
    sentAt: '2026-05-20T18:45:01Z',
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    event: 'agent.activity.recent.summary.reported',
    severity: 'error',
    payload: {
      reason: 'Activity store is unavailable.',
    },
    snapshot: null,
  });
}
