import { describe, expect, it } from 'vitest';
import { DeviceStatusEvidence, EvidenceRef, LocationEvidence } from './tracking-fixtures';
import {
  TrackingDeviceStatusEvidenceSchema,
  TrackingTemporaryLiveAuthorizationSchema,
  evaluateTrackingTemporaryLiveRuntime,
} from '../src/tracking';

describe('tracking temporary live runtime proof', () => {
  it('keeps an authorized bounded session active and schedules the next sample', assertActiveSession);
  it('expires the session without presenting stale location as live', assertExpiredSession);
  it('records parent stop as a disabled-by-parent audited state', assertParentStoppedSession);
  it('degrades live tracking when device status is battery-throttled', assertBatteryThrottledSession);
  it('rejects unbounded temporary live authorization windows', assertUnboundedAuthorizationRejected);
});

function assertActiveSession() {
  const runtime = evaluateTrackingTemporaryLiveRuntime({
    authorization: temporaryLiveAuthorization(),
    evaluatedAt: '2026-06-03T02:05:00.000Z',
    locationCapabilityStatus: 'live',
    permissionState: 'granted-foreground',
    deviceStatus: TrackingDeviceStatusEvidenceSchema.parse(DeviceStatusEvidence),
    locationEvidenceId: LocationEvidence.evidenceId,
  });

  expect(runtime.state).toBe('active');
  expect(runtime.capabilityStatus).toBe('live');
  expect(runtime.stopReason).toBe('none');
  expect(runtime.nextSampleDueAt).toBe('2026-06-03T02:06:00.000Z');
  expect(runtime.reasonCodes).toEqual(['temporary-live-active-parent-authorized']);
  expect(runtime.parentAuthorizationRef).toBe('parent-approved-live-session');
  expect(runtime.productClaimReady).toBe(false);
}

function assertExpiredSession() {
  const runtime = evaluateTrackingTemporaryLiveRuntime({
    authorization: temporaryLiveAuthorization(),
    evaluatedAt: '2026-06-03T03:00:00.000Z',
    locationCapabilityStatus: 'recent',
    permissionState: 'granted-foreground',
    deviceStatus: TrackingDeviceStatusEvidenceSchema.parse(DeviceStatusEvidence),
    locationEvidenceId: LocationEvidence.evidenceId,
  });

  expect(runtime.state).toBe('expired');
  expect(runtime.capabilityStatus).toBe('stale');
  expect(runtime.stopReason).toBe('duration-expired');
  expect(runtime.nextSampleDueAt).toBeNull();
  expect(runtime.reasonCodes).toEqual(['temporary-live-duration-expired']);
}

function assertParentStoppedSession() {
  const runtime = evaluateTrackingTemporaryLiveRuntime({
    authorization: temporaryLiveAuthorization(),
    evaluatedAt: '2026-06-03T02:10:00.000Z',
    locationCapabilityStatus: 'live',
    permissionState: 'granted-foreground',
    deviceStatus: TrackingDeviceStatusEvidenceSchema.parse(DeviceStatusEvidence),
    locationEvidenceId: LocationEvidence.evidenceId,
    parentStopRequestedAt: '2026-06-03T02:09:00.000Z',
  });

  expect(runtime.state).toBe('stopped');
  expect(runtime.capabilityStatus).toBe('disabled-by-parent');
  expect(runtime.stopReason).toBe('parent-stopped');
  expect(runtime.parentStopRequestedAt).toBe('2026-06-03T02:09:00.000Z');
  expect(runtime.reasonCodes).toEqual(['temporary-live-parent-stopped']);
}

function assertBatteryThrottledSession() {
  const runtime = evaluateTrackingTemporaryLiveRuntime({
    authorization: temporaryLiveAuthorization(),
    evaluatedAt: '2026-06-03T02:06:00.000Z',
    locationCapabilityStatus: 'live',
    permissionState: 'granted-foreground',
    deviceStatus: TrackingDeviceStatusEvidenceSchema.parse(lowPowerDeviceStatus()),
    locationEvidenceId: LocationEvidence.evidenceId,
  });

  expect(runtime.state).toBe('degraded');
  expect(runtime.capabilityStatus).toBe('battery-throttled');
  expect(runtime.stopReason).toBe('battery-throttled');
  expect(runtime.nextSampleDueAt).toBeNull();
  expect(runtime.reasonCodes).toEqual(['temporary-live-battery-throttled']);
}

function assertUnboundedAuthorizationRejected() {
  const parsed = TrackingTemporaryLiveAuthorizationSchema.safeParse({
    ...temporaryLiveAuthorization(),
    durationMinutes: 90,
    maxDurationMinutes: 60,
    expiresAt: '2026-06-03T03:30:00.000Z',
  });

  expect(parsed.success).toBe(false);
}

function temporaryLiveAuthorization() {
  return TrackingTemporaryLiveAuthorizationSchema.parse({
    schemaVersion: 1,
    sessionId: 'temporary-live-session-1',
    childDeviceId: 'child-device-1',
    requestedAt: '2026-06-03T02:00:00.000Z',
    startsAt: '2026-06-03T02:00:00.000Z',
    expiresAt: '2026-06-03T03:00:00.000Z',
    parentAuthorizationRef: 'parent-approved-live-session',
    durationMinutes: 60,
    maxDurationMinutes: 60,
    cadenceSeconds: 60,
    retentionMode: 'delete-on-resolution',
    disclosureRequired: true,
    remoteSyncDefault: 'disabled',
    auditRefs: ['temporary-live-parent-approved'],
    evidence: [EvidenceRef],
  });
}

function lowPowerDeviceStatus() {
  return {
    ...DeviceStatusEvidence,
    evidenceId: 'device-status-low-power',
    capabilityStatus: 'battery-throttled',
    battery: {
      ...DeviceStatusEvidence.battery,
      lowPowerMode: 'enabled',
    },
    degradedReasons: ['android-low-power-mode'],
  };
}
