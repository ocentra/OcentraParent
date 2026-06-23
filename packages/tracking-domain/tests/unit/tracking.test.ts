import { describe, expect, it } from 'vitest';
import {
  TrackingCapabilityStatusMatrixSchema,
  TrackingDeviceStatusEvidenceSchema,
  TrackingLocationEvidenceSchema,
  TrackingRetentionPolicySchema,
} from '@ocentra-parent/schema-domain/tracking-evidence';
import { TrackingCapabilityStatusSchema } from '@ocentra-parent/schema-domain/tracking-primitives';
import {
  TrackingExpectedPlaceDecisionSchema,
  TrackingExpectedPlaceScheduleSchema,
  TrackingGeofenceRuleSchema,
  TrackingGeofenceTransitionSchema,
  TrackingNearbyPlaceEvidenceSchema,
  TrackingParentDefinedPlaceSchema,
} from '@ocentra-parent/schema-domain/tracking-geofence';
import { TrackingReadModelSchema } from '@ocentra-parent/schema-domain/tracking-read-model';
import { applyTrackingRetentionDelete, applyTrackingRetentionExport } from '../../src/tracking-retention-runtime';
import { evaluateTrackingExpectedPlaceDecision, evaluateTrackingGeofenceTransition } from '../../src/tracking-runtime';

const EvidenceRef = {
  evidenceId: 'tracking-journal-row-1',
  kind: 'journal-entry',
  digest: 'sha256:tracking-proof',
  uri: null,
} as const;

const LocationEvidence = {
  schemaVersion: 1,
  evidenceId: 'location-evidence-1',
  observedAt: '2026-06-03T02:00:00.000Z',
  freshUntil: '2026-06-03T02:05:00.000Z',
  staleAt: '2026-06-03T02:15:00.000Z',
  sourceId: 'android-child-agent',
  adapterId: 'android-fused-location-adapter',
  deviceId: 'child-device-1',
  sourceKind: 'android-fused-location',
  capabilityStatus: 'recent',
  permissionState: 'granted-foreground',
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
  confidence: 0.92,
  custodyLabel: 'child-device-local',
  retentionMode: '24h',
  reasonCodes: ['foreground-location-sample'],
  evidence: [EvidenceRef],
} as const;

const DeviceStatusEvidence = {
  schemaVersion: 1,
  evidenceId: 'device-status-1',
  observedAt: '2026-06-03T02:00:00.000Z',
  freshUntil: '2026-06-03T02:05:00.000Z',
  staleAt: '2026-06-03T02:15:00.000Z',
  sourceId: 'android-child-agent',
  adapterId: 'android-device-status-adapter',
  deviceId: 'child-device-1',
  sourceKind: 'android-device-status',
  capabilityStatus: 'recent',
  lastLocationEvidenceId: 'location-evidence-1',
  heartbeatStatus: 'healthy',
  battery: {
    percent: 64,
    chargingState: 'discharging',
    lowPowerMode: 'disabled',
  },
  connectivityStatus: 'online',
  pendingUploadCount: 0,
  custodyLabel: 'child-device-local',
  retentionMode: '24h',
  degradedReasons: [],
  evidence: [EvidenceRef],
} as const;

const CapabilityStatus = {
  schemaVersion: 1,
  checkedAt: '2026-06-03T02:00:00.000Z',
  platform: 'android',
  foregroundLocation: 'manual-required',
  backgroundLocation: 'manual-required',
  geofenceTransitions: 'manual-required',
  deviceStatus: 'manual-required',
  permissionState: 'manual-required',
  manualActionRequired: true,
  sourceId: 'tracking-plan-proof',
  adapterId: 'tracking-contract-proof',
  reasonCodes: ['real-device-permission-proof-required'],
  auditRefs: ['output/tracking-plan-proof/manual-platform-proof'],
} as const;

const RetentionPolicy = {
  schemaVersion: 1,
  policyId: 'tracking-retention-24h-local',
  mode: '24h',
  custodyLabel: 'child-device-local',
  customRetentionHours: null,
  deleteOnResolution: false,
  exportAllowed: true,
  remoteSyncDefault: 'disabled',
  auditRefs: ['tracking-retention-local-first'],
} as const;

const CircleRule = {
  schemaVersion: 1,
  ruleId: 'home-arrival-rule',
  geofenceId: 'home-circle',
  placeId: 'home',
  label: 'Home',
  placeKind: 'home',
  shape: {
    kind: 'circle',
    center: {
      latitude: 43.6532,
      longitude: -79.3832,
    },
    radiusMeters: 150,
    polygon: [],
  },
  minAccuracyMeters: 100,
  enterGraceSeconds: 120,
  exitGraceSeconds: 120,
  dwellSeconds: 300,
  scheduleId: 'school-night-schedule',
  enabled: true,
  retentionMode: '24h',
  auditRefs: ['home-rule-created'],
} as const;

const GeofenceTransition = {
  schemaVersion: 1,
  transitionId: 'home-enter-transition',
  observedAt: '2026-06-03T02:01:00.000Z',
  ruleId: 'home-arrival-rule',
  geofenceId: 'home-circle',
  locationEvidenceId: 'location-evidence-1',
  transition: 'enter',
  capabilityStatus: 'recent',
  distanceMeters: 42,
  reasonCodes: ['inside-circle-with-accuracy'],
  evidence: [EvidenceRef],
} as const;

const ExpectedPlaceSchedule = {
  schemaVersion: 1,
  scheduleId: 'school-night-schedule',
  ruleId: 'home-arrival-rule',
  placeId: 'home',
  label: 'Home on school nights',
  windows: [
    {
      startsAt: '2026-06-03T01:00:00.000Z',
      endsAt: '2026-06-03T11:00:00.000Z',
      timezone: 'America/Toronto',
    },
  ],
  distanceToleranceMeters: 150,
  lateGraceSeconds: 600,
  earlyExitGraceSeconds: 600,
  activeException: null,
  enabled: true,
  auditRefs: ['expected-place-schedule-created'],
} as const;

const ExpectedPlaceDecision = {
  schemaVersion: 1,
  decisionId: 'expected-place-decision-1',
  observedAt: '2026-06-03T02:01:00.000Z',
  scheduleId: 'school-night-schedule',
  ruleId: 'home-arrival-rule',
  locationEvidenceId: 'location-evidence-1',
  outcome: 'where-expected',
  distanceToleranceMeters: 150,
  lateGraceSeconds: 600,
  earlyExitGraceSeconds: 600,
  exceptionState: null,
  exceptionAuditRef: null,
  reasonCodes: ['home-window-active'],
  evidence: [EvidenceRef],
} as const;

const NearbyPlaceEvidence = {
  schemaVersion: 1,
  evidenceId: 'nearby-place-1',
  observedAt: '2026-06-03T02:01:00.000Z',
  locationEvidenceId: 'location-evidence-1',
  providerKind: 'parent-defined',
  providerRef: 'parent-place-db',
  queryRadiusMeters: 250,
  distanceMeters: 42,
  category: 'home',
  confidence: 0.91,
  ambiguityState: 'clear',
  reasonCodes: ['parent-defined-place-match'],
  evidence: [EvidenceRef],
} as const;

function trackingReadModelSample() {
  return {
    schemaVersion: 1,
    generatedAt: '2026-06-03T02:02:00.000Z',
    custodyLabel: 'child-device-local',
    capabilityStatus: 'recent',
    limit: 25,
    returned: 5,
    locationRows: [LocationEvidence],
    deviceStatusRows: [DeviceStatusEvidence],
    capabilityRows: [CapabilityStatus],
    geofenceTransitions: [GeofenceTransition],
    expectedPlaceDecisions: [ExpectedPlaceDecision],
    nearbyPlaceRows: [NearbyPlaceEvidence],
    retentionPolicies: [RetentionPolicy],
    timeline: [
      {
        rowId: 'location-evidence-1',
        kind: 'location',
        observedAt: '2026-06-03T02:00:00.000Z',
        capabilityStatus: 'recent',
        reasonCodes: ['foreground-location-sample'],
        evidence: [EvidenceRef],
      },
    ],
  };
}

const DistinctExpectedPlaceRuntimeCases = [
  {
    observedAt: '2026-06-03T12:05:00.000Z',
    schedule: {
      ...ExpectedPlaceSchedule,
      scheduleId: 'school-weekday-schedule',
      ruleId: 'school-arrival-rule',
      placeId: 'school-campus',
      label: 'School weekday arrival',
      windows: [
        {
          startsAt: '2026-06-03T11:45:00.000Z',
          endsAt: '2026-06-03T19:00:00.000Z',
          timezone: 'America/Toronto',
        },
      ],
      distanceToleranceMeters: 75,
      lateGraceSeconds: 900,
      earlyExitGraceSeconds: 60,
    },
    transition: {
      ...GeofenceTransition,
      transitionId: 'school-enter-transition',
      observedAt: '2026-06-03T12:05:00.000Z',
      ruleId: 'school-arrival-rule',
      transition: 'enter' as const,
      distanceMeters: 18,
      reasonCodes: ['school-arrival-sample'],
    },
    expectedOutcome: 'where-expected' as const,
    expectedReasonCode: 'inside-expected-place-window',
  },
  {
    observedAt: '2026-06-03T21:20:00.000Z',
    schedule: {
      ...ExpectedPlaceSchedule,
      scheduleId: 'after-school-activity-schedule',
      ruleId: 'soccer-practice-rule',
      placeId: 'soccer-practice',
      label: 'After-school soccer practice',
      windows: [
        {
          startsAt: '2026-06-03T20:00:00.000Z',
          endsAt: '2026-06-03T22:00:00.000Z',
          timezone: 'America/Toronto',
        },
      ],
      distanceToleranceMeters: 120,
      lateGraceSeconds: 300,
      earlyExitGraceSeconds: 180,
    },
    transition: {
      ...GeofenceTransition,
      transitionId: 'activity-exit-transition',
      observedAt: '2026-06-03T21:20:00.000Z',
      ruleId: 'soccer-practice-rule',
      transition: 'exit' as const,
      distanceMeters: 140,
      reasonCodes: ['activity-exit-sample'],
    },
    expectedOutcome: 'left-expected-place' as const,
    expectedReasonCode: 'exited-expected-place-window',
  },
  {
    observedAt: '2026-06-03T14:36:00.000Z',
    schedule: {
      ...ExpectedPlaceSchedule,
      scheduleId: 'calendar-appointment-schedule',
      ruleId: 'calendar-appointment-rule',
      placeId: 'calendar-appointment',
      label: 'Calendar-backed appointment',
      windows: [
        {
          startsAt: '2026-06-03T14:00:00.000Z',
          endsAt: '2026-06-03T15:00:00.000Z',
          timezone: 'America/Toronto',
        },
      ],
      distanceToleranceMeters: 30,
      lateGraceSeconds: 300,
      earlyExitGraceSeconds: 0,
    },
    transition: {
      ...GeofenceTransition,
      transitionId: 'calendar-missed-arrival-transition',
      observedAt: '2026-06-03T14:36:00.000Z',
      ruleId: 'calendar-appointment-rule',
      transition: 'missed-arrival' as const,
      distanceMeters: null,
      reasonCodes: ['calendar-missed-arrival-sample'],
    },
    expectedOutcome: 'late-arrival' as const,
    expectedReasonCode: 'missed-expected-place-arrival',
  },
] as const;

function assertDistinctExpectedPlaceRuntimeCases() {
  for (const testCase of DistinctExpectedPlaceRuntimeCases) {
    const decision = evaluateTrackingExpectedPlaceDecision({
      decisionId: `runtime-${testCase.schedule.scheduleId}-decision`,
      observedAt: testCase.observedAt,
      schedule: TrackingExpectedPlaceScheduleSchema.parse(testCase.schedule),
      location: TrackingLocationEvidenceSchema.parse(LocationEvidence),
      transition: TrackingGeofenceTransitionSchema.parse(testCase.transition),
    });

    expect(decision.scheduleId).toBe(testCase.schedule.scheduleId);
    expect(decision.ruleId).toBe(testCase.schedule.ruleId);
    expect(decision.outcome).toBe(testCase.expectedOutcome);
    expect(decision.reasonCodes).toEqual([testCase.expectedReasonCode]);
    expect(decision.distanceToleranceMeters).toBe(testCase.schedule.distanceToleranceMeters);
    expect(decision.lateGraceSeconds).toBe(testCase.schedule.lateGraceSeconds);
    expect(decision.earlyExitGraceSeconds).toBe(testCase.schedule.earlyExitGraceSeconds);
  }
}

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
    expect(schedule.ruleId).toBe('home-arrival-rule');
    expect(schedule.distanceToleranceMeters).toBe(150);
    expect(schedule.activeException).toBeNull();
    expect(decision.outcome).toBe('where-expected');
    expect(decision.ruleId).toBe('home-arrival-rule');
    expect(decision.distanceToleranceMeters).toBe(150);
    expect(decision.lateGraceSeconds).toBe(600);
    expect(decision.earlyExitGraceSeconds).toBe(600);
    expect(decision.exceptionState).toBeNull();
    expect(decision.exceptionAuditRef).toBeNull();
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
    expect(decision.ruleId).toBe('home-arrival-rule');
    expect(decision.distanceToleranceMeters).toBe(150);
    expect(decision.lateGraceSeconds).toBe(600);
    expect(decision.earlyExitGraceSeconds).toBe(600);
    expect(decision.evidence.length).toBeGreaterThan(0);
  });

  it('evaluates distinct school, activity, and calendar-backed expected-place rules', () => {
    assertDistinctExpectedPlaceRuntimeCases();
  });
});

describe('tracking first-target runtime helpers ambiguity handling', () => {
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

  it('keeps low-accuracy expected-place decisions ambiguous instead of late or exit accusations', () => {
    const location = TrackingLocationEvidenceSchema.parse({
      ...LocationEvidence,
      evidenceId: 'low-accuracy-expected-place-location',
      accuracyMeters: 250,
      reasonCodes: ['low-accuracy-expected-place-sample'],
    });
    const transition = evaluateTrackingGeofenceTransition({
      transitionId: 'runtime-low-accuracy-expected-place-transition',
      observedAt: '2026-06-03T02:01:00.000Z',
      rule: TrackingGeofenceRuleSchema.parse(CircleRule),
      location,
      wasInside: false,
    });
    const decision = evaluateTrackingExpectedPlaceDecision({
      decisionId: 'runtime-low-accuracy-expected-place-decision',
      observedAt: '2026-06-03T02:01:00.000Z',
      schedule: TrackingExpectedPlaceScheduleSchema.parse(ExpectedPlaceSchedule),
      location,
      transition,
    });

    expect(transition.transition).toBe('ambiguous');
    expect(decision.outcome).toBe('unknown');
    expect(decision.reasonCodes).toEqual(['expected-place-ambiguous']);
    expect(decision.ruleId).toBe('home-arrival-rule');
    expect(decision.distanceToleranceMeters).toBe(150);
  });
});

describe('tracking first-target runtime helpers boundary handling', () => {
  it('treats the geofence boundary as the expected-place tolerance equivalent', () => {
    const exactBoundaryLocation = TrackingLocationEvidenceSchema.parse({
      ...LocationEvidence,
      evidenceId: 'boundary-inside-location',
      reasonCodes: ['boundary-inside-sample'],
    });
    const outsideBoundaryLocation = TrackingLocationEvidenceSchema.parse({
      ...LocationEvidence,
      evidenceId: 'boundary-outside-location',
      coordinate: {
        latitude: 43.6533,
        longitude: -79.3832,
      },
      reasonCodes: ['boundary-outside-sample'],
    });
    const boundaryRule = TrackingGeofenceRuleSchema.parse({
      ...CircleRule,
      ruleId: 'boundary-tolerance-rule',
      geofenceId: 'boundary-tolerance-circle',
      scheduleId: 'boundary-tolerance-schedule',
      label: 'Boundary tolerance rule',
      shape: {
        ...CircleRule.shape,
        radiusMeters: 0,
      },
    });
    const boundarySchedule = TrackingExpectedPlaceScheduleSchema.parse({
      ...ExpectedPlaceSchedule,
      scheduleId: 'boundary-tolerance-schedule',
      ruleId: 'boundary-tolerance-rule',
      distanceToleranceMeters: 0,
    });

    const insideTransition = evaluateTrackingGeofenceTransition({
      transitionId: 'boundary-inside-transition',
      observedAt: '2026-06-03T02:01:00.000Z',
      rule: boundaryRule,
      location: exactBoundaryLocation,
      wasInside: false,
    });
    const outsideTransition = evaluateTrackingGeofenceTransition({
      transitionId: 'boundary-outside-transition',
      observedAt: '2026-06-03T02:01:00.000Z',
      rule: boundaryRule,
      location: outsideBoundaryLocation,
      wasInside: false,
    });
    const insideDecision = evaluateTrackingExpectedPlaceDecision({
      decisionId: 'boundary-inside-decision',
      observedAt: '2026-06-03T02:01:00.000Z',
      schedule: boundarySchedule,
      location: exactBoundaryLocation,
      transition: insideTransition,
    });
    const outsideDecision = evaluateTrackingExpectedPlaceDecision({
      decisionId: 'boundary-outside-decision',
      observedAt: '2026-06-03T02:01:00.000Z',
      schedule: boundarySchedule,
      location: outsideBoundaryLocation,
      transition: outsideTransition,
    });

    expect(insideTransition.transition).toBe('enter');
    expect(insideTransition.reasonCodes).toEqual(['inside-geofence-with-accuracy']);
    expect(insideDecision.outcome).toBe('where-expected');
    expect(insideDecision.distanceToleranceMeters).toBe(0);
    expect(insideDecision.reasonCodes).toEqual(['inside-expected-place-window']);

    expect(outsideTransition.transition).toBe('ambiguous');
    expect(outsideTransition.reasonCodes).toEqual(['outside-geofence-with-accuracy']);
    expect(outsideDecision.outcome).toBe('unknown');
    expect(outsideDecision.distanceToleranceMeters).toBe(0);
    expect(outsideDecision.reasonCodes).toEqual(['expected-place-ambiguous']);
  });
});

describe('tracking first-target runtime helpers freshness handling', () => {
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

describe('tracking first-target runtime helpers grace periods', () => {
  it('maps missed-arrival transitions outside late grace to late-arrival', () => {
    const decision = evaluateTrackingExpectedPlaceDecision({
      decisionId: 'runtime-late-arrival-decision',
      observedAt: '2026-06-03T01:11:00.000Z',
      schedule: TrackingExpectedPlaceScheduleSchema.parse(ExpectedPlaceSchedule),
      location: TrackingLocationEvidenceSchema.parse(LocationEvidence),
      transition: TrackingGeofenceTransitionSchema.parse({
        ...GeofenceTransition,
        transitionId: 'runtime-missed-arrival-transition',
        observedAt: '2026-06-03T01:11:00.000Z',
        transition: 'missed-arrival',
        distanceMeters: null,
        reasonCodes: ['missed-arrival-sample'],
      }),
    });

    expect(decision.outcome).toBe('late-arrival');
    expect(decision.reasonCodes).toEqual(['missed-expected-place-arrival']);
  });

  it('suppresses missed-arrival decisions while late grace is active', () => {
    const decision = evaluateTrackingExpectedPlaceDecision({
      decisionId: 'runtime-late-grace-decision',
      observedAt: '2026-06-03T01:05:00.000Z',
      schedule: TrackingExpectedPlaceScheduleSchema.parse(ExpectedPlaceSchedule),
      location: TrackingLocationEvidenceSchema.parse(LocationEvidence),
      transition: TrackingGeofenceTransitionSchema.parse({
        ...GeofenceTransition,
        transitionId: 'runtime-late-grace-transition',
        observedAt: '2026-06-03T01:05:00.000Z',
        transition: 'missed-arrival',
        distanceMeters: null,
        reasonCodes: ['late-grace-sample'],
      }),
    });

    expect(decision.outcome).toBe('unknown');
    expect(decision.reasonCodes).toEqual(['expected-place-late-grace-active']);
  });

  it('suppresses exit decisions while early-exit grace is active', () => {
    const decision = evaluateTrackingExpectedPlaceDecision({
      decisionId: 'runtime-early-exit-grace-decision',
      observedAt: '2026-06-03T10:55:00.000Z',
      schedule: TrackingExpectedPlaceScheduleSchema.parse(ExpectedPlaceSchedule),
      location: TrackingLocationEvidenceSchema.parse(LocationEvidence),
      transition: TrackingGeofenceTransitionSchema.parse({
        ...GeofenceTransition,
        transitionId: 'runtime-early-exit-grace-transition',
        observedAt: '2026-06-03T10:55:00.000Z',
        transition: 'exit',
        distanceMeters: 220,
        reasonCodes: ['early-exit-sample'],
      }),
    });

    expect(decision.outcome).toBe('unknown');
    expect(decision.reasonCodes).toEqual(['expected-place-early-exit-grace-active']);
  });
});

describe('tracking first-target runtime helpers exception handling', () => {
  it('suppresses expected-place actions while a schedule exception is active', () => {
    const exceptionStates = [
      {
        state: 'holiday-mode' as const,
        auditRef: 'expected-place-holiday-exception',
        transition: 'dwell' as const,
        reasonCode: 'expected-place-holiday-exception-active',
      },
      {
        state: 'trip-exception' as const,
        auditRef: 'expected-place-trip-exception',
        transition: 'missed-arrival' as const,
        reasonCode: 'expected-place-trip-exception-active',
      },
    ];

    for (const exception of exceptionStates) {
      const decision = evaluateTrackingExpectedPlaceDecision({
        decisionId: `runtime-${exception.state}-decision`,
        observedAt: '2026-06-03T02:01:00.000Z',
        schedule: TrackingExpectedPlaceScheduleSchema.parse({
          ...ExpectedPlaceSchedule,
          activeException: {
            state: exception.state,
            auditRef: exception.auditRef,
          },
        }),
        location: TrackingLocationEvidenceSchema.parse(LocationEvidence),
        transition: TrackingGeofenceTransitionSchema.parse({
          ...GeofenceTransition,
          transitionId: `runtime-${exception.state}-transition`,
          transition: exception.transition,
          reasonCodes: [`runtime-${exception.state}`],
        }),
      });

      expect(decision.outcome).toBe('unknown');
      expect(decision.exceptionState).toBe(exception.state);
      expect(decision.exceptionAuditRef).toBe(exception.auditRef);
      expect(decision.reasonCodes).toEqual([exception.reasonCode]);
    }
  });
});

describe('tracking first-target runtime helpers degraded capability handling', () => {
  it('marks the Rust degraded expected-place capability states as manual-required', () => {
    const manualReviewStatuses = [
      'stale',
      'last-known',
      'offline-last-known-only',
      'permission-required',
      'background-permission-required',
      'approximate-only',
      'manual-required',
      'unavailable',
      'adapter-error',
      'disabled-by-parent',
    ] as const;

    for (const capabilityStatus of manualReviewStatuses) {
      const decision = evaluateTrackingExpectedPlaceDecision({
        decisionId: `runtime-manual-required-${capabilityStatus}-decision`,
        observedAt: '2026-06-03T02:01:00.000Z',
        schedule: TrackingExpectedPlaceScheduleSchema.parse(ExpectedPlaceSchedule),
        location: TrackingLocationEvidenceSchema.parse({
          ...LocationEvidence,
          evidenceId: `runtime-manual-required-${capabilityStatus}-location`,
          capabilityStatus,
          reasonCodes: [`runtime-manual-required-${capabilityStatus}`],
        }),
        transition: TrackingGeofenceTransitionSchema.parse({
          ...GeofenceTransition,
          transitionId: `runtime-manual-required-${capabilityStatus}-transition`,
          transition: 'dwell',
          reasonCodes: [`runtime-manual-required-${capabilityStatus}`],
        }),
      });

      expect(decision.outcome).toBe('manual-required');
      expect(decision.reasonCodes).toEqual(['fresh-location-required']);
    }
  });
});

describe('tracking first-target runtime helpers time windows', () => {
  it('keeps DST-spanning expected-place windows active when the UTC observation falls inside the encoded window', () => {
    const decision = evaluateTrackingExpectedPlaceDecision({
      decisionId: 'runtime-dst-window-decision',
      observedAt: '2026-03-08T07:30:00.000Z',
      schedule: TrackingExpectedPlaceScheduleSchema.parse({
        ...ExpectedPlaceSchedule,
        windows: [
          {
            startsAt: '2026-03-08T06:30:00.000Z',
            endsAt: '2026-03-08T08:30:00.000Z',
            timezone: 'America/Toronto',
          },
        ],
      }),
      location: TrackingLocationEvidenceSchema.parse(LocationEvidence),
      transition: TrackingGeofenceTransitionSchema.parse({
        ...GeofenceTransition,
        observedAt: '2026-03-08T07:30:00.000Z',
        transitionId: 'runtime-dst-window-transition',
        transition: 'dwell',
      }),
    });

    expect(decision.outcome).toBe('where-expected');
    expect(decision.ruleId).toBe('home-arrival-rule');
    expect(decision.reasonCodes).toEqual(['inside-expected-place-window']);
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
