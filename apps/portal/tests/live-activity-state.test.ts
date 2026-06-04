import { describe, expect, it } from 'vitest';
import { ActivitySurfaceSchemaVersion } from '@ocentra-parent/activity-domain/activity-surface';
import {
  AgentEventEnvelopeSchema,
  AgentProtocolDefaults,
  type AgentEventName,
} from '@ocentra-parent/agent-protocol-domain/contracts';
import { resolveLiveActivityState } from '../src/live-activity-state';

type ResolvedLiveActivityState = ReturnType<typeof resolveLiveActivityState>;
type BrowserEvidenceReadModel = NonNullable<ResolvedLiveActivityState['browserEvidenceReadModel']>;
type BrowserEvidenceRow = BrowserEvidenceReadModel['rows'][number];

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
      browserEvidenceEvent('evt-browser-earlier', 'https://earlier.example/learn'),
      activityReportEvent({
        eventId: 'evt-report-earlier',
        event: 'agent.activity.report.generated',
        reportId: 'activity-report-earlier',
      }),
      browserEvidenceEvent('evt-browser-latest', 'https://latest.example/learn'),
      activityReportEvent({
        eventId: 'evt-report-latest',
        event: 'agent.activity.report.saved',
        reportId: 'activity-report-latest',
      }),
    ]);

    expect(state.browserEvidenceEvent?.eventId).toBe('evt-browser-latest');
    expect(state.browserEvidenceReadModel?.rows.at(0)?.url).toBe('https://latest.example/learn');
    expect(state.activityReportEvent?.eventId).toBe('evt-report-latest');
    expect(state.activityReport?.ok ? state.activityReport.value.reportId : null).toBe('activity-report-latest');
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

function browserEvidenceEvent(eventId = 'evt-browser', url = 'https://example.test/learn') {
  const origin = new URL(url).origin;
  const domain = new URL(url).hostname;

  return AgentEventEnvelopeSchema.parse({
    schemaVersion: 1,
    eventId,
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
}) {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: 1,
    eventId: input.eventId,
    correlationId: 'cmd-report',
    sentAt: '2026-05-21T01:00:01Z',
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
