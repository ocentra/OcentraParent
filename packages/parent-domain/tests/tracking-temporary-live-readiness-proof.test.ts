import { describe, expect, it } from 'vitest';
import {
  TrackingTemporaryLiveReadinessReadModelSchema,
  TrackingTemporaryLiveReadinessRowSchema,
  buildTrackingTemporaryLiveReadinessReadModel,
} from '../src/tracking-temporary-live-readiness-proof';
import { TrackingPolicySchemaVersion } from '../src/tracking-location-policy';

describe('tracking temporary live readiness proof', () => {
  it('derives ready, active, expired, revoked, unavailable, and duration manual states', () => {
    const readModel = buildTrackingTemporaryLiveReadinessReadModel(proofMetadata(), sourceReadModelFixture());

    expect(readModel.rows).toHaveLength(6);
    expect(readModel.readyToStartCount).toBe(1);
    expect(readModel.activeCount).toBe(1);
    expect(readModel.expiredAutoStopCount).toBe(1);
    expect(readModel.manualRequiredCount).toBe(6);
    expect(readModel.revokedOrDeniedCount).toBe(1);
    expect(readModel.productClaimReady).toBe(false);

    expect(row(readModel, 'tracking-live-ready')?.readinessState).toBe('ready-to-start');
    expect(row(readModel, 'tracking-live-active')?.readinessState).toBe('active-time-boxed');
    expect(row(readModel, 'tracking-live-expired')?.readinessState).toBe('expired-auto-stop-required');
    expect(row(readModel, 'tracking-live-revoked')?.readinessState).toBe('revoked-by-parent');
    expect(row(readModel, 'tracking-live-unavailable')?.readinessState).toBe('platform-unavailable-manual-required');
    expect(row(readModel, 'tracking-live-too-long')?.readinessState).toBe('duration-policy-manual-required');
  });

  it('preserves cadence, duration, expiry, audit, and retention proof fields', () => {
    const readModel = buildTrackingTemporaryLiveReadinessReadModel(proofMetadata(), sourceReadModelFixture());
    const active = row(readModel, 'tracking-live-active');
    const expired = row(readModel, 'tracking-live-expired');

    expect(active?.durationSeconds).toBe(900);
    expect(active?.maximumDurationSeconds).toBe(3600);
    expect(active?.cadenceSeconds).toBe(60);
    expect(active?.autoStopReason).toBe('expires-at');
    expect(active?.retentionAction).toBe('retain-until-expiry');
    expect(active?.auditRefs).toEqual(['tracking-live-active-audit']);
    expect(expired?.autoStopReason).toBe('expired');
    expect(expired?.retentionAction).toBe('delete-after-retention-window');
  });

  it('keeps platform, battery, runtime, UI, provider, sync, and physical-device claims false', () => {
    const readModel = buildTrackingTemporaryLiveReadinessReadModel(proofMetadata(), sourceReadModelFixture());
    const active = row(readModel, 'tracking-live-active');

    expect(active?.platformRuntimeProofState).toBe('real-device-required');
    expect(active?.batteryStatusProofState).toBe('manual-required');
    expect(active?.manualProofRequirements).toContain('live-location-runtime-proof-required');
    expect(active?.manualProofRequirements).toContain('battery-status-runtime-proof-required');
    expect(readModel.liveLocationRuntimeClaimed).toBe(false);
    expect(readModel.physicalDeviceProofClaimed).toBe(false);
    expect(readModel.backgroundLocationClaimed).toBe(false);
    expect(readModel.batteryRuntimeClaimed).toBe(false);
    expect(readModel.childDisclosureUiClaimed).toBe(false);
    expect(readModel.parentLiveUiClaimed).toBe(false);
    expect(readModel.remoteSyncClaimed).toBe(false);
    expect(readModel.providerDeliveryClaimed).toBe(false);
  });

  it('rejects active rows without disclosure and rejects runtime overclaims', () => {
    const readModel = buildTrackingTemporaryLiveReadinessReadModel(proofMetadata(), sourceReadModelFixture());
    const active = row(readModel, 'tracking-live-active');
    const unsafeActive = TrackingTemporaryLiveReadinessRowSchema.safeParse({
      ...active,
      childDisclosureRequired: false,
    });
    const unsafeReadModel = TrackingTemporaryLiveReadinessReadModelSchema.safeParse({
      ...readModel,
      physicalDeviceProofClaimed: true,
    });

    expect(unsafeActive.success).toBe(false);
    expect(unsafeReadModel.success).toBe(false);
  });
});

function row(readModel: ReturnType<typeof buildTrackingTemporaryLiveReadinessReadModel>, grantId: string) {
  return readModel.rows.find((entry) => entry.grantId === grantId);
}

function proofMetadata() {
  return {
    generatedAt: '2026-06-05T14:10:00.000Z',
    proofId: 'tracking-temporary-live-readiness-proof',
    sourceTrackingReadModelRef: 'tracking-location-policy-temporary-live-proof',
    sourceContractRefs: ['tracking-location-policy', 'location-geofence-device-status', 'temporary-live-tracking-mode'],
  };
}

function sourceReadModelFixture() {
  return {
    schemaVersion: TrackingPolicySchemaVersion,
    generatedAt: '2026-06-05T14:10:00.000Z',
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
      grant('tracking-live-ready', 'requested', '2026-06-05T14:35:00.000Z', 1200, true, true),
      grant('tracking-live-active', 'active', '2026-06-05T14:20:00.000Z', 900, true, true),
      grant('tracking-live-expired', 'active', '2026-06-05T14:05:00.000Z', 300, true, true),
      grant('tracking-live-revoked', 'revoked', '2026-06-05T14:25:00.000Z', 1200, true, true),
      grant('tracking-live-unavailable', 'unavailable', '2026-06-05T14:25:00.000Z', 1200, true, true),
      grant('tracking-live-too-long', 'requested', '2026-06-05T16:10:00.000Z', 7200, true, true),
    ],
    missingDeviceCases: [],
    platformProofRoutes: [
      {
        schemaVersion: TrackingPolicySchemaVersion,
        platform: 'android',
        foregroundLocation: 'real-device-required',
        backgroundLocation: 'background-permission-required',
        geofence: 'real-device-required',
        deviceStatus: 'manual-required',
        proofArtifactRefs: [],
        manualRequiredReason: 'android-physical-device-required',
      },
    ],
  };
}

function grant(
  grantId: string,
  state: string,
  expiresAt: string,
  durationSeconds: number,
  parentApproved: boolean,
  childDisclosureRequired: boolean
) {
  return {
    schemaVersion: TrackingPolicySchemaVersion,
    grantId,
    state,
    requestedAt: '2026-06-05T14:00:00.000Z',
    expiresAt,
    durationSeconds,
    parentApproved,
    childDisclosureRequired,
    auditRefs: [`${grantId}-audit`],
  } as const;
}
