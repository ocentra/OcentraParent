import { describe, expect, it } from 'vitest';
import { ActivityQuerySchemaVersion } from '@ocentra-parent/activity-domain/query';
import {
  trackingStatusLiveSummary,
  trackingStatusServiceDataCoverage,
  type TrackingStatusLiveProjectionInput,
} from '../../src/tracking-status-panel';

describe('tracking status panel', () => {
  it('prefers active summary metadata and active device/kind/capability counts', () => {
    const input = trackingInput(trackingReadModel());

    expect(trackingStatusLiveSummary(input)).toMatchObject({
      lastObserved: '2026-06-03T07:24:00Z',
      eventId: 'tracking-event-1',
      evidenceReferences: 'tracking-evidence-1',
    });
    expect(trackingStatusServiceDataCoverage(input)).toMatchObject({
      rowVisibility: '1 | 1',
      lastObserved: '2026-06-03T07:26:00Z',
      eventId: 'tracking-retention-delete-1',
      deviceCounts: 'child-device-1 (1)',
      capability: 'recent (1)',
      activityKinds: 'tracking.expected-place.evaluated (1)',
      evidenceReferences: 'tracking-evidence-1',
      deletedEvidence: 'location-evidence-1',
    });
  });

  it('falls back to legacy top-level fields when additive active summaries are absent', () => {
    const input = trackingInput(legacyTrackingReadModel());

    expect(trackingStatusLiveSummary(input)).toMatchObject({
      lastObserved: '2026-06-03T07:24:00Z',
      eventId: 'tracking-event-1',
      evidenceReferences: 'tracking-evidence-1',
    });
    expect(trackingStatusServiceDataCoverage(input)).toMatchObject({
      deviceCounts: 'child-device-1',
      capability: 'recent',
      activityKinds: 'tracking.expected-place.evaluated',
    });
  });
});

function trackingInput(readModel: ReturnType<typeof trackingReadModel> | ReturnType<typeof legacyTrackingReadModel>) {
  return {
    activityTrackingReadModelEvent: {
      severity: 'info',
    },
    activityTrackingReadModel: {
      ok: true,
      value: readModel,
    },
  } as unknown as TrackingStatusLiveProjectionInput;
}

function trackingReadModel() {
  return {
    schemaVersion: ActivityQuerySchemaVersion,
    generatedAt: '2026-06-03T07:25:00Z',
    custodyLabel: 'child-device-query-store',
    limit: 20,
    returned: 2,
    activeRows: 1,
    tombstoneRows: 1,
    capabilityStatus: 'recent',
    latestEventId: 'tracking-retention-delete-1',
    latestObservedAt: '2026-06-03T07:26:00Z',
    latestActiveEventId: 'tracking-event-1',
    latestActiveObservedAt: '2026-06-03T07:24:00Z',
    latestTombstoneEventId: 'tracking-retention-delete-1',
    latestTombstoneObservedAt: '2026-06-03T07:26:00Z',
    activeKindCounts: [{ value: 'tracking.expected-place.evaluated', count: 1 }],
    activeDeviceCounts: [{ value: 'child-device-1', count: 1 }],
    activeCapabilityStatusCounts: [{ value: 'recent', count: 1 }],
    deletedEvidenceReferenceIds: ['location-evidence-1'],
    rows: [
      {
        schemaVersion: ActivityQuerySchemaVersion,
        eventId: 'tracking-event-1',
        observedAt: '2026-06-03T07:24:00Z',
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
      {
        schemaVersion: ActivityQuerySchemaVersion,
        eventId: 'tracking-retention-delete-1',
        observedAt: '2026-06-03T07:26:00Z',
        deviceId: 'child-device-1',
        platform: 'android',
        observer: 'tracking-retention',
        kind: 'activity.tracking.retention.deleted',
        subjectKind: 'location-evidence',
        subjectId: 'location-evidence-1',
        subjectDisplayName: null,
        capabilityStatus: 'recent',
        queryVisibility: 'tombstone',
        deletedAt: '2026-06-03T07:26:00Z',
        evidenceReferenceIds: ['location-evidence-1'],
        deletedEvidenceReferenceIds: ['location-evidence-1'],
        evidence: [],
      },
    ],
  } as const;
}

function legacyTrackingReadModel() {
  const {
    latestActiveEventId: _latestActiveEventId,
    latestActiveObservedAt: _latestActiveObservedAt,
    activeKindCounts: _activeKindCounts,
    activeDeviceCounts: _activeDeviceCounts,
    activeCapabilityStatusCounts: _activeCapabilityStatusCounts,
    ...legacy
  } = trackingReadModel();

  return {
    ...legacy,
    latestEventId: 'tracking-event-1',
    latestObservedAt: '2026-06-03T07:24:00Z',
  };
}
