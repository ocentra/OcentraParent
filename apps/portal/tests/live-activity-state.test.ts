import { describe, expect, it } from 'vitest';
import { AgentEventEnvelopeSchema } from '@ocentra-parent/agent-protocol-domain/contracts';
import { resolveLiveActivityState } from '../src/live-activity-state';

describe('portal live activity state', () => {
  it('parses real service ingest and recent-summary payload fields', () => {
    const state = resolveLiveActivityState([recentSummaryEvent(), ingestStatusEvent()]);

    expect(state.ingestStatus?.databaseReady).toBe(true);
    expect(state.ingestStatus?.eventsStored).toBe(1);
    expect(state.recentSummary?.returned).toBe(1);
    expect(state.recentSummary?.mostRecentSubjectName).toBe('notepad.exe');
  });

  it('keeps unavailable activity-store responses visible without inventing rows', () => {
    const state = resolveLiveActivityState([unavailableRecentSummaryEvent()]);

    expect(state.ingestStatus).toBeNull();
    expect(state.recentSummary).toBeNull();
    expect(state.recentSummaryEvent?.severity).toBe('error');
    expect(state.recentSummaryEvent?.payload['reason']).toBe('Activity store is unavailable.');
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
