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
  returned: 2,
  activeRows: 1,
  tombstoneRows: 1,
  capabilityStatus: 'recent',
  latestEventId: 'tracking-delete-1',
  latestObservedAt: '2026-06-03T07:20:00Z',
  latestTombstoneEventId: 'tracking-delete-1',
  latestTombstoneObservedAt: '2026-06-03T07:20:00Z',
  deletedEvidenceReferenceIds: ['tracking-evidence-1'],
  coverageRows: [
    {
      schemaVersion: ActivityQuerySchemaVersion,
      surface: 'expected-place',
      activeRows: 1,
      tombstoneRows: 0,
      citationCount: 1,
      latestEventId: 'tracking-event-1',
      latestObservedAt: '2026-06-03T07:19:00Z',
      readyForProductClaim: false,
      missingProof: 'platform-replay-proof-required',
    },
    {
      schemaVersion: ActivityQuerySchemaVersion,
      surface: 'retention',
      activeRows: 0,
      tombstoneRows: 1,
      citationCount: 1,
      latestEventId: 'tracking-delete-1',
      latestObservedAt: '2026-06-03T07:20:00Z',
      readyForProductClaim: false,
      missingProof: 'broader-product-ui-proof-required',
    },
  ],
  productClaimState: {
    physicalDeviceClaimed: false,
    providerDeliveryClaimed: false,
    notificationDeliveryClaimed: false,
    childDeviceRuntimeClaimed: false,
    ocentraHostedStorageClaimed: false,
    productCompleteClaimed: false,
  },
  rows: [
    {
      schemaVersion: ActivityQuerySchemaVersion,
      eventId: 'tracking-delete-1',
      observedAt: '2026-06-03T07:20:00Z',
      deviceId: 'child-device-1',
      platform: 'android',
      observer: 'tracking-engine',
      kind: 'activity.tracking.retention.deleted',
      subjectKind: 'retention',
      subjectId: 'tracking-retention-24h-local',
      subjectDisplayName: 'Tracking retention delete',
      capabilityStatus: 'recent',
      queryVisibility: 'tombstone',
      deletedAt: '2026-06-03T07:20:00Z',
      evidenceReferenceIds: ['tracking-evidence-1'],
      deletedEvidenceReferenceIds: ['tracking-evidence-1'],
      evidence: [],
    },
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
      queryVisibility: 'active',
      deletedAt: null,
      evidenceReferenceIds: ['tracking-evidence-1'],
      deletedEvidenceReferenceIds: [],
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
