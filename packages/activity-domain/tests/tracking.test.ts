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
} from '../src/tracking';

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
