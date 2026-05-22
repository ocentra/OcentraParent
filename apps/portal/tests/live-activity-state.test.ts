import { describe, expect, it } from 'vitest';
import { AgentEventEnvelopeSchema } from '@ocentra-parent/agent-protocol-domain/contracts';
import { resolveLiveActivityState } from '../src/live-activity-state';

describe('portal live activity state', () => {
  it('parses real service ingest and recent-summary payload fields', () => {
    const state = resolveLiveActivityState([browserEvidenceEvent(), recentSummaryEvent(), ingestStatusEvent()]);

    expect(state.ingestStatus?.databaseReady).toBe(true);
    expect(state.ingestStatus?.eventsStored).toBe(1);
    expect(state.recentSummary?.returned).toBe(1);
    expect(state.recentSummary?.mostRecentSubjectName).toBe('notepad.exe');
    expect(state.browserEvidenceReadModel?.returned).toBe(1);
    expect(state.browserEvidenceReadModel?.rows.at(0)?.url).toBe('https://example.test/learn');
    expect(state.browserEvidenceReadModel?.rows.at(0)?.activeState).toBe('unknown');
    expect(state.browserEvidenceReadModel?.capabilityStatus).toBe('tab-list-only');
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
});

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

function browserEvidenceEvent() {
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
      url: 'https://example.test/learn',
      origin: 'https://example.test',
      domain: 'example.test',
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
