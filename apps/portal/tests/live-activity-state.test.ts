import { describe, expect, it } from 'vitest';
import { AgentEvent } from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import { AgentProtocolDefaults } from '@ocentra-parent/schema-domain/agent-protocol-defaults';
import {
  AgentBrowserRuntimeCapabilityStatus,
  AgentBrowserRuntimeEventType,
  AgentBrowserRuntimePhase,
  AgentBrowserRuntimeQueryVisibility,
} from '@ocentra-parent/schema-domain/agent-browser-runtime-events';
import { AgentNetworkRuntimeEventType } from '@ocentra-parent/schema-domain/network-runtime-events';
import { resolveLiveActivityState } from '../src/live-activity-state';
import {
  FlowObserved,
  activityReportEvent,
  appGameAdapterDispatchExecutedEvent,
  browserEvidenceEvent,
  browserInventoryEvent,
  browserRuntimeEventChainStreamEvent,
  browserRuntimeStreamEntry,
  eventWithPayload,
  ingestStatusEvent,
  recentSummaryEvent,
} from './live-activity-state-test-support';
import { emptyBrowserEvidenceEvent, unavailableRecentSummaryEvent } from './live-activity-state-event-fixtures';

type ResolvedLiveActivityState = ReturnType<typeof resolveLiveActivityState>;
type BrowserEvidenceReadModel = NonNullable<ResolvedLiveActivityState['browserEvidenceReadModel']>;
type BrowserEvidenceRow = BrowserEvidenceReadModel['rows'][number];
type BrowserRuntimeEventChainStream = NonNullable<ResolvedLiveActivityState['browserRuntimeEventChainStream']>;

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

describe('portal live app-game adapter dispatch state', () => {
  it('keeps the latest scoped app-game adapter dispatch executed result parent-visible', () => {
    const state = resolveLiveActivityState([
      appGameAdapterDispatchExecutedEvent('evt-app-game-dispatch-executed-earlier', 'earlier-execute-command'),
      appGameAdapterDispatchExecutedEvent('evt-app-game-dispatch-executed-latest', 'latest-execute-command'),
    ]);

    expectLatestAppGameAdapterDispatchExecutedResult(state);
  });
});

describe('portal live activity network service state', portalLiveActivityNetworkServiceTests);

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
  if (latestRow === undefined) {
    throw new Error('Expected a browser evidence row to be present.');
  }
  expect(latestRow.url).toBe('https://example.test/learn');
  expect(latestRow.activeState).toBe('unknown');
  expect(latestRow.activeProofSource).toBe('target-list-only');
}

function expectLatestAppGameAdapterDispatchExecutedResult(state: ResolvedLiveActivityState) {
  expect(state.appGameAdapterDispatchExecutedEvent?.eventId).toBe('evt-app-game-dispatch-executed-latest');
  expect(state.appGameAdapterDispatchExecutedResult?.ok).toBe(true);
  if (state.appGameAdapterDispatchExecutedResult?.ok !== true) {
    return;
  }
  expect(state.appGameAdapterDispatchExecutedResult.value.commandId).toBe('latest-execute-command');
  expect(state.appGameAdapterDispatchExecutedResult.value.executionStatus).toBe('actually-enforced');
  expect(state.appGameAdapterDispatchExecutedResult.value.adapterDispatchExecutedClaimed).toBe(true);
  expect(state.appGameAdapterDispatchExecutedResult.value.platformEnforcementClaimed).toBe(false);
  expect(state.appGameAdapterDispatchExecutedResult.value.childDeviceDeliveryClaimed).toBe(false);
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

function portalLiveActivityNetworkServiceTests(): void {
  it('resolves network service events through typed parsers', () => {
    const liveActivity = resolveLiveActivityState(networkServiceEvents());

    expectNetworkRuntimeEventChain(liveActivity);
    expectInvalidNetworkStatusParsers(liveActivity);
  });
}

function networkServiceEvents() {
  return [
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
    eventWithPayload(AgentEvent.NetworkAndroidVpnServiceGateStatusReported, {
      [AgentProtocolDefaults.Field.NetworkAndroidVpnServiceGateStatus]: '{',
    }),
    eventWithPayload(AgentEvent.NetworkAppleNetworkExtensionGateStatusReported, {
      [AgentProtocolDefaults.Field.NetworkAppleNetworkExtensionGateStatus]: '{',
    }),
  ];
}

function expectNetworkRuntimeEventChain(liveActivity: ResolvedLiveActivityState): void {
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
}

function expectInvalidNetworkStatusParsers(liveActivity: ResolvedLiveActivityState): void {
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
  expect(liveActivity.networkAndroidVpnServiceGateStatusEvent?.event).toBe(
    'agent.network.android-vpn-service-gate.status.reported'
  );
  expect(liveActivity.networkAndroidVpnServiceGateStatusResult).toEqual({
    ok: false,
    reason: 'invalid-android-vpn-service-gate-status-json',
  });
  expect(liveActivity.networkAppleNetworkExtensionGateStatusEvent?.event).toBe(
    'agent.network.apple-network-extension-gate.status.reported'
  );
  expect(liveActivity.networkAppleNetworkExtensionGateStatusResult).toEqual({
    ok: false,
    reason: 'invalid-apple-network-extension-gate-status-json',
  });
}

function browserRuntimeStreamOrThrow(state: ResolvedLiveActivityState): BrowserRuntimeEventChainStream {
  const stream = state.browserRuntimeEventChainStream;
  if (stream === null) {
    throw new Error('Expected browser runtime event-chain stream');
  }
  return stream;
}
