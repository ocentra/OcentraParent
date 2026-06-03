import { describe, expect, it } from 'vitest';
import { ActivityQuerySchemaVersion } from '@ocentra-parent/activity-domain/query';
import { AgentProtocolSchemaVersion } from '../src/primitives';
import { AgentEvent, AgentProtocolDefaults, type AgentEventEnvelope } from '../src/contracts';
import { parseAgentActivityTrackingReadModelEvent } from '../src/tracking-read-model';

const Source = {
  peerId: 'agent-service',
  role: 'agent-service',
} as const;

const Target = {
  peerId: 'portal-dev',
  role: 'portal',
} as const;

const TrackingReadModel = {
  schemaVersion: ActivityQuerySchemaVersion,
  generatedAt: '2026-06-03T07:20:00Z',
  custodyLabel: 'child-device-query-store',
  limit: 20,
  returned: 1,
  capabilityStatus: 'recent',
  latestEventId: 'tracking-event-1',
  latestObservedAt: '2026-06-03T07:19:00Z',
  evidenceReferenceIds: ['tracking-evidence-1', 'retention-tombstone-evidence-1'],
  retentionTombstoneCount: 1,
  retentionTombstoneEvidenceReferenceIds: ['retention-tombstone-evidence-1'],
  rows: [
    {
      schemaVersion: ActivityQuerySchemaVersion,
      eventId: 'tracking-event-1',
      observedAt: '2026-06-03T07:19:00Z',
      deviceId: 'child-device-1',
      platform: 'android',
      observer: 'tracking-engine',
      kind: 'tracking.expected-place.evaluated',
      subjectKind: 'tracking-rule',
      subjectId: 'expected-place-school',
      subjectDisplayName: 'School',
      capabilityStatus: 'recent',
      evidenceReferenceIds: ['tracking-evidence-1'],
      evidence: [],
    },
  ],
} as const;

describe('agent activity tracking read-model parser', () => {
  it('parses the service-backed tracking read-model event payload', () => {
    const parsed = parseAgentActivityTrackingReadModelEvent(trackingEvent(JSON.stringify(TrackingReadModel)));

    expect(parsed).toEqual({
      ok: true,
      value: TrackingReadModel,
    });
  });

  it('rejects wrong events and invalid payloads without inventing rows', () => {
    expect(
      parseAgentActivityTrackingReadModelEvent({
        ...trackingEvent(JSON.stringify(TrackingReadModel)),
        event: AgentEvent.HealthReported,
      })
    ).toEqual({
      ok: false,
      reason: 'wrong-event',
    });
    expect(parseAgentActivityTrackingReadModelEvent(trackingEvent('{'))).toEqual({
      ok: false,
      reason: 'invalid-json',
    });
    expect(
      parseAgentActivityTrackingReadModelEvent(trackingEvent(JSON.stringify({ ...TrackingReadModel, rows: [{}] })))
    ).toEqual({
      ok: false,
      reason: 'invalid-payload',
    });
  });
});

function trackingEvent(serializedReadModel: string): AgentEventEnvelope {
  return {
    schemaVersion: AgentProtocolSchemaVersion,
    eventId: 'tracking-read-model-event',
    correlationId: 'tracking-read-model-command',
    sentAt: '2026-06-03T07:20:01Z',
    source: Source,
    target: Target,
    event: AgentEvent.ActivityTrackingReadModelReported,
    severity: 'info',
    payload: {
      [AgentProtocolDefaults.Field.ActivityTrackingReadModel]: serializedReadModel,
    },
    snapshot: null,
  };
}
