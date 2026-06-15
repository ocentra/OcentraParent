import { ActivityEvidenceKind } from '@ocentra-parent/evidence-domain/kinds';

export const EvidenceRef = {
  evidenceId: 'tracking-journal-row-1',
  kind: ActivityEvidenceKind.JournalEntry,
  digest: 'sha256:tracking-proof',
  uri: null,
} as const;

export const LocationEvidence = {
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

export const DeviceStatusEvidence = {
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

export const CapabilityStatus = {
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

export const RetentionPolicy = {
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

export const CircleRule = {
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

export const GeofenceTransition = {
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

export const ExpectedPlaceSchedule = {
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
  enabled: true,
  auditRefs: ['expected-place-schedule-created'],
} as const;

export const ExpectedPlaceDecision = {
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
  reasonCodes: ['home-window-active'],
  evidence: [EvidenceRef],
} as const;

export const NearbyPlaceEvidence = {
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

export function trackingReadModelSample() {
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
    timeline: [trackingTimelineRowSample()],
  };
}

export function trackingTimelineRowSample() {
  return {
    rowId: 'location-evidence-1',
    kind: 'location',
    observedAt: '2026-06-03T02:00:00.000Z',
    capabilityStatus: 'recent',
    reasonCodes: ['foreground-location-sample'],
    evidence: [EvidenceRef],
  };
}
