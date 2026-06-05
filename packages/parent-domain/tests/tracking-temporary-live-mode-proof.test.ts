import { describe, expect, it } from 'vitest';
import {
  TrackingTemporaryLiveModeReadModelSchema,
  TrackingTemporaryLiveModeRowSchema,
  buildTrackingTemporaryLiveModeReadModel,
} from '../src/tracking-temporary-live-mode-proof';
import { TrackingPolicySchemaVersion } from '../src/tracking-location-policy';

describe('tracking temporary live mode proof', () => {
  it('derives authorized, degraded, expired, retention, and manual-required temporary live rows', () => {
    const readModel = buildProofReadModel();

    expect(readModel.rows).toHaveLength(6);
    expect(readModel.activeAuthorizedCount).toBe(1);
    expect(readModel.degradedCount).toBe(2);
    expect(readModel.autoStoppedCount).toBe(1);
    expect(readModel.retentionDeleteReadyCount).toBe(1);
    expect(readModel.manualRequiredCount).toBe(1);
    expect(readModel.productClaimReady).toBe(false);
    expect(readModel.liveLocationRuntimeClaimed).toBe(false);
    expect(readModel.currentLocationRuntimeClaimed).toBe(false);
    expect(readModel.backgroundLocationRuntimeClaimed).toBe(false);
    expect(readModel.providerDeliveryClaimed).toBe(false);
    expect(readModel.remoteRelayRuntimeClaimed).toBe(false);
    expect(readModel.physicalDeviceProofClaimed).toBe(false);

    expect(rowState(readModel, 'tracking-live-active')).toBe('active-authorized');
    expect(rowState(readModel, 'tracking-live-battery')).toBe('battery-degraded');
    expect(rowState(readModel, 'tracking-live-permission')).toBe('permission-degraded');
    expect(rowState(readModel, 'tracking-live-expired')).toBe('expired-auto-stopped');
    expect(rowState(readModel, 'tracking-live-retention')).toBe('retention-delete-ready');
    expect(rowState(readModel, 'tracking-live-manual-required')).toBe('manual-required');
  });

  it('keeps cadence, audit, degraded, and retention proof refs attached to every row', () => {
    const readModel = buildProofReadModel();
    const battery = row(readModel, 'tracking-live-battery');
    const permission = row(readModel, 'tracking-live-permission');
    const retention = row(readModel, 'tracking-live-retention');

    for (const proofRow of readModel.rows) {
      expect(proofRow.durationSeconds).toBeLessThanOrEqual(proofRow.maxDurationSeconds);
      expect(proofRow.locationEvidenceRefs.length).toBeGreaterThan(0);
      expect(proofRow.policyDecisionRefs.length).toBeGreaterThan(0);
      expect(proofRow.auditRefs.length).toBeGreaterThan(0);
    }

    expect(battery.batteryState).toBe('battery-throttled');
    expect(battery.manualProofRequirements).toContain('battery-throttled-cadence-reduction-proof');
    expect(permission.permissionState).toBe('background-permission-required');
    expect(permission.manualProofRequirements).toContain('background-location-permission-proof-required');
    expect(retention.retentionRefs).toEqual(['temporary-live-retention-delete-proof']);
    expect(retention.autoStopReason).toBe('duration-expired-retention-delete-ready');
  });

  it('rejects unsafe active rows and read-model overclaims', () => {
    const readModel = buildProofReadModel();
    const active = row(readModel, 'tracking-live-active');

    expect(
      TrackingTemporaryLiveModeRowSchema.safeParse({
        ...active,
        parentApproved: false,
      }).success
    ).toBe(false);
    expect(
      TrackingTemporaryLiveModeRowSchema.safeParse({
        ...active,
        durationSeconds: active.maxDurationSeconds + 1,
      }).success
    ).toBe(false);
    expect(
      TrackingTemporaryLiveModeReadModelSchema.safeParse({
        ...readModel,
        liveLocationRuntimeClaimed: true,
      }).success
    ).toBe(false);
    expect(
      TrackingTemporaryLiveModeReadModelSchema.safeParse({
        ...readModel,
        productClaimReady: true,
      }).success
    ).toBe(false);
  });
});

function buildProofReadModel() {
  return buildTrackingTemporaryLiveModeReadModel(
    {
      generatedAt: '2026-06-05T16:40:00.000Z',
      proofId: 'tracking-temporary-live-mode-proof',
      sourceContractRefs: [
        'tracking-location-policy',
        'location-geofence-device-status',
        'temporary-live-tracking-mode-workpack',
      ],
      contexts: [
        context('tracking-live-active', 'interval', 60, 900, 'foreground-only', 'normal', null, []),
        context(
          'tracking-live-battery',
          'interval',
          300,
          900,
          'foreground-only',
          'battery-throttled',
          null,
          ['battery-throttled-cadence-reduction-proof']
        ),
        context(
          'tracking-live-permission',
          'on-change',
          0,
          900,
          'background-permission-required',
          'normal',
          null,
          ['background-location-permission-proof-required']
        ),
        context('tracking-live-expired', 'interval', 60, 900, 'foreground-only', 'normal', 'duration-expired', []),
        context(
          'tracking-live-retention',
          'interval',
          60,
          900,
          'foreground-only',
          'normal',
          'duration-expired-retention-delete-ready',
          ['temporary-live-retention-delete-proof']
        ),
        context(
          'tracking-live-manual-required',
          'high-accuracy-burst',
          15,
          900,
          'permission-required',
          'low-power-mode',
          null,
          ['physical-device-live-session-proof-required']
        ),
      ],
    },
    {
      schemaVersion: TrackingPolicySchemaVersion,
      generatedAt: '2026-06-05T16:40:00.000Z',
      rules: [],
      decisions: [],
      acknowledgements: [],
      checkInRequests: [],
      checkInResponses: [],
      aiRoutes: [],
      aiResults: [],
      alerts: [],
      escalations: [],
      temporaryLiveGrants: [
        grant('tracking-live-active', 'active', true, true),
        grant('tracking-live-battery', 'active', true, true),
        grant('tracking-live-permission', 'active', true, true),
        grant('tracking-live-expired', 'expired', true, true),
        grant('tracking-live-retention', 'expired', true, true),
        grant('tracking-live-manual-required', 'unavailable', false, false),
      ],
      missingDeviceCases: [],
      platformProofRoutes: [],
    }
  );
}

function grant(grantId: string, state: string, parentApproved: boolean, childDisclosureRequired: boolean) {
  return {
    schemaVersion: TrackingPolicySchemaVersion,
    grantId,
    state,
    requestedAt: '2026-06-05T16:30:00.000Z',
    expiresAt: '2026-06-05T16:45:00.000Z',
    durationSeconds: 900,
    parentApproved,
    childDisclosureRequired,
    auditRefs: [`temporary-live-audit-${grantId}`],
  } as const;
}

function context(
  grantId: string,
  requestedCadence: string,
  requestedCadenceSeconds: number,
  maxDurationSeconds: number,
  permissionState: string,
  batteryState: string,
  autoStopReason: string | null,
  extraProofRefs: readonly string[]
) {
  return {
    grantId,
    requestedCadence,
    requestedCadenceSeconds,
    maxDurationSeconds,
    permissionState,
    batteryState,
    deliveryPath: 'local-lan',
    locationEvidenceRefs: [`temporary-live-location-evidence-${grantId}`],
    policyDecisionRefs: [`temporary-live-policy-decision-${grantId}`],
    retentionRefs: grantId === 'tracking-live-retention' ? ['temporary-live-retention-delete-proof'] : [],
    manualProofRequirements: [
      `temporary-live-runtime-proof-required-${grantId}`,
      `temporary-live-physical-device-proof-required-${grantId}`,
      ...extraProofRefs,
    ],
    autoStopReason,
  } as const;
}

function row(readModel: ReturnType<typeof buildProofReadModel>, grantId: string) {
  const proofRow = readModel.rows.find((candidate) => candidate.grantId === grantId);
  if (proofRow === undefined) {
    throw new Error(`Missing row ${grantId}`);
  }
  return proofRow;
}

function rowState(readModel: ReturnType<typeof buildProofReadModel>, grantId: string) {
  return row(readModel, grantId).sessionState;
}
