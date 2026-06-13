import { describe, expect, it } from 'vitest';
import {
  CapabilityStatus,
  CircleRule,
  DeviceStatusEvidence,
  ExpectedPlaceDecision,
  ExpectedPlaceSchedule,
  GeofenceTransition,
  LocationEvidence,
  NearbyPlaceEvidence,
  RetentionPolicy,
  trackingReadModelSample,
} from './tracking-fixtures';
import {
  TrackingCapabilityStatusSchema,
  TrackingCapabilityStatusMatrixSchema,
  TrackingDeviceStatusEvidenceSchema,
  TrackingExpectedPlaceDecisionSchema,
  TrackingExpectedPlaceScheduleSchema,
  TrackingGeofenceRuleSchema,
  TrackingGeofenceTransitionSchema,
  TrackingLocationEvidenceSchema,
  TrackingNearbyPlaceEvidenceSchema,
  TrackingParentDefinedPlaceSchema,
  TrackingReadModelSchema,
  TrackingRetentionPolicySchema,
  applyTrackingRetentionDelete,
  applyTrackingRetentionExport,
  evaluateTrackingExpectedPlaceDecision,
  evaluateTrackingGeofenceTransition,
} from '../../src/tracking';

describe('tracking evidence contracts', () => {
  it('parses location, device status, capability, and retention evidence', () => {
    const location = TrackingLocationEvidenceSchema.parse(LocationEvidence);
    const status = TrackingDeviceStatusEvidenceSchema.parse(DeviceStatusEvidence);
    const capability = TrackingCapabilityStatusMatrixSchema.parse(CapabilityStatus);
    const retention = TrackingRetentionPolicySchema.parse(RetentionPolicy);

    expect(location.accuracyMeters).toBe(22);
    expect(status.battery.percent).toBe(64);
    expect(capability.manualActionRequired).toBe(true);
    expect(retention.remoteSyncDefault).toBe('disabled');
  });

  it('parses aggregate tracking read-model evidence', () => {
    const readModel = TrackingReadModelSchema.parse(trackingReadModelSample());

    expect(readModel.timeline[0]?.kind).toBe('location');
    expect(readModel.locationRows[0]?.custodyLabel).toBe('child-device-local');
  });

  it('parses foreground, background, and degraded tracking capability states', () => {
    expect(TrackingCapabilityStatusSchema.parse('foreground-only')).toBe('foreground-only');
    expect(TrackingCapabilityStatusSchema.parse('background-ready')).toBe('background-ready');
    expect(TrackingCapabilityStatusSchema.parse('approximate-only')).toBe('approximate-only');
    expect(TrackingCapabilityStatusSchema.parse('permission-required')).toBe('permission-required');
    expect(TrackingCapabilityStatusSchema.parse('background-permission-required')).toBe(
      'background-permission-required'
    );
    expect(TrackingCapabilityStatusSchema.parse('platform-unsupported')).toBe('platform-unsupported');
  });
});

describe('tracking evidence guards', () => {
  it('rejects IP or LAN hint evidence when it carries precise coordinates', () => {
    const result = TrackingLocationEvidenceSchema.safeParse({
      ...LocationEvidence,
      sourceKind: 'desktop-presence-hint',
      coordinate: {
        latitude: 43.6532,
        longitude: -79.3832,
      },
      accuracyMeters: 22,
      hint: {
        quality: 'ip-coarse-hint',
        coarseRadiusMeters: 5000,
        label: 'Toronto area',
      },
      reasonCodes: ['ip-is-coarse-only'],
    });

    expect(result.success).toBe(false);
  });

  it('rejects hint-only source kinds even when they claim GPS-quality coordinates', () => {
    const result = TrackingLocationEvidenceSchema.safeParse({
      ...LocationEvidence,
      sourceKind: 'desktop-presence-hint',
      coordinate: {
        latitude: 43.6532,
        longitude: -79.3832,
      },
      accuracyMeters: 22,
      hint: {
        quality: 'gps',
        coarseRadiusMeters: null,
        label: null,
      },
      reasonCodes: ['desktop-presence-is-hint-only'],
    });

    expect(result.success).toBe(false);
  });
});

describe('tracking shape guards', () => {
  it('rejects invalid accuracy and geofence shapes', () => {
    const badAccuracy = TrackingLocationEvidenceSchema.safeParse({
      ...LocationEvidence,
      accuracyMeters: -1,
    });
    const badCircle = TrackingGeofenceRuleSchema.safeParse({
      ...CircleRule,
      shape: {
        ...CircleRule.shape,
        center: null,
      },
    });
    const badPolygon = TrackingGeofenceRuleSchema.safeParse({
      ...CircleRule,
      shape: {
        kind: 'polygon',
        center: null,
        radiusMeters: null,
        polygon: [
          {
            latitude: 43.6532,
            longitude: -79.3832,
          },
          {
            latitude: 43.654,
            longitude: -79.384,
          },
        ],
      },
    });

    expect(badAccuracy.success).toBe(false);
    expect(badCircle.success).toBe(false);
    expect(badPolygon.success).toBe(false);
  });
});

describe('tracking geofence status guards', () => {
  it('rejects non-ambiguous geofence transitions from degraded location capability', () => {
    const staleEnter = TrackingGeofenceTransitionSchema.safeParse({
      ...GeofenceTransition,
      transitionId: 'offline-last-known-enter-transition',
      transition: 'enter',
      capabilityStatus: 'offline-last-known-only',
      reasonCodes: ['offline-last-known-only'],
    });
    const unavailableDwell = TrackingGeofenceTransitionSchema.safeParse({
      ...GeofenceTransition,
      transitionId: 'unavailable-dwell-transition',
      transition: 'dwell',
      capabilityStatus: 'unavailable',
      reasonCodes: ['location-unavailable'],
    });

    expect(staleEnter.success).toBe(false);
    expect(unavailableDwell.success).toBe(false);
  });
});

describe('tracking rule and nearby place contracts', () => {
  it('parses geofence, expected-place, nearby-place, and parent-defined place evidence', () => {
    const rule = TrackingGeofenceRuleSchema.parse(CircleRule);
    const transition = TrackingGeofenceTransitionSchema.parse(GeofenceTransition);
    const schedule = TrackingExpectedPlaceScheduleSchema.parse(ExpectedPlaceSchedule);
    const decision = TrackingExpectedPlaceDecisionSchema.parse(ExpectedPlaceDecision);
    const nearby = TrackingNearbyPlaceEvidenceSchema.parse(NearbyPlaceEvidence);
    const place = TrackingParentDefinedPlaceSchema.parse({
      schemaVersion: 1,
      placeId: 'home',
      label: 'Home',
      placeKind: 'home',
      shape: CircleRule.shape,
      createdAt: '2026-06-03T01:00:00.000Z',
      updatedAt: '2026-06-03T02:00:00.000Z',
      auditRefs: ['parent-defined-home-created'],
    });

    expect(rule.placeKind).toBe('home');
    expect(transition.transition).toBe('enter');
    expect(schedule.windows[0]?.timezone).toBe('America/Toronto');
    expect(decision.outcome).toBe('where-expected');
    expect(nearby.ambiguityState).toBe('clear');
    expect(place.placeId).toBe('home');
  });
});

describe('tracking first-target runtime helpers', () => {
  it('evaluates geofence and expected-place decisions from parsed evidence', () => {
    const transition = evaluateTrackingGeofenceTransition({
      transitionId: 'runtime-home-enter-transition',
      observedAt: '2026-06-03T02:01:00.000Z',
      rule: TrackingGeofenceRuleSchema.parse(CircleRule),
      location: TrackingLocationEvidenceSchema.parse(LocationEvidence),
      wasInside: false,
    });
    const decision = evaluateTrackingExpectedPlaceDecision({
      decisionId: 'runtime-expected-place-decision',
      observedAt: '2026-06-03T02:01:00.000Z',
      schedule: TrackingExpectedPlaceScheduleSchema.parse(ExpectedPlaceSchedule),
      location: TrackingLocationEvidenceSchema.parse(LocationEvidence),
      transition,
    });

    expect(transition.transition).toBe('enter');
    expect(transition.locationEvidenceId).toBe(LocationEvidence.evidenceId);
    expect(decision.outcome).toBe('where-expected');
    expect(decision.evidence.length).toBeGreaterThan(0);
  });

  it('keeps low-accuracy geofence samples ambiguous instead of alert-ready', () => {
    const transition = evaluateTrackingGeofenceTransition({
      transitionId: 'runtime-low-accuracy-transition',
      observedAt: '2026-06-03T02:01:00.000Z',
      rule: TrackingGeofenceRuleSchema.parse(CircleRule),
      location: TrackingLocationEvidenceSchema.parse({
        ...LocationEvidence,
        evidenceId: 'low-accuracy-location',
        accuracyMeters: 250,
        reasonCodes: ['low-accuracy-sample'],
      }),
      wasInside: false,
    });

    expect(transition.transition).toBe('ambiguous');
    expect(transition.reasonCodes).toContain('location-accuracy-below-rule-threshold');
  });

  it('keeps stale and permission-denied geofence samples ambiguous instead of alert-ready', () => {
    const transition = evaluateTrackingGeofenceTransition({
      transitionId: 'runtime-permission-denied-transition',
      observedAt: '2026-06-03T02:01:00.000Z',
      rule: TrackingGeofenceRuleSchema.parse(CircleRule),
      location: TrackingLocationEvidenceSchema.parse({
        ...LocationEvidence,
        evidenceId: 'permission-denied-location',
        capabilityStatus: 'permission-denied',
        permissionState: 'denied',
        reasonCodes: ['location-permission-denied'],
      }),
      wasInside: false,
    });

    expect(transition.transition).toBe('ambiguous');
    expect(transition.reasonCodes).toContain('fresh-location-required');
  });
});

describe('tracking retention runtime helpers', () => {
  it('deletes retained tracking rows from the read model and marks stale history', () => {
    const proof = applyTrackingRetentionDelete({
      readModel: TrackingReadModelSchema.parse(trackingReadModelSample()),
      generatedAt: '2026-06-03T03:00:00.000Z',
      deletedEvidenceIds: [LocationEvidence.evidenceId],
    });

    expect(proof.beforeLocationRows).toBe(1);
    expect(proof.afterLocationRows).toBe(0);
    expect(proof.readModel.locationRows).toHaveLength(0);
    expect(proof.readModel.geofenceTransitions).toHaveLength(0);
    expect(proof.readModel.timeline).toHaveLength(0);
    expect(proof.readModel.capabilityStatus).toBe('stale');
  });

  it('exports a parent-owned local snapshot without enabling remote sync', () => {
    const proof = applyTrackingRetentionExport({
      readModel: TrackingReadModelSchema.parse(trackingReadModelSample()),
      generatedAt: '2026-06-03T03:05:00.000Z',
      policy: TrackingRetentionPolicySchema.parse({
        ...RetentionPolicy,
        policyId: 'tracking-retention-parent-export',
        mode: 'export-only',
        custodyLabel: 'parent-owned-export',
        auditRefs: ['tracking-retention-parent-export'],
      }),
    });

    expect(proof.exportAllowed).toBe(true);
    expect(proof.sourceLocationRows).toBe(1);
    expect(proof.exportedLocationRows).toBe(1);
    expect(proof.custodyLabel).toBe('parent-owned-export');
    expect(proof.retentionMode).toBe('export-only');
    expect(proof.remoteSyncDefault).toBe('disabled');
    expect(proof.readModel.locationRows[0]?.custodyLabel).toBe('parent-owned-export');
    expect(proof.readModel.deviceStatusRows[0]?.retentionMode).toBe('export-only');
  });
});
