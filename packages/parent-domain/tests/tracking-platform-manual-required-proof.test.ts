import { describe, expect, it } from 'vitest';
import {
  buildTrackingPlatformManualRequiredProof,
  summarizeTrackingPlatformManualRequiredProof,
  TrackingPlatformManualRequiredProofSchema,
} from '../src/tracking-platform-manual-required-proof';

describe('tracking platform manual-required proof', () => {
  it('keeps Android and iOS privileged tracking capabilities manual-required before device proof', () => {
    const proof = buildTrackingPlatformManualRequiredProof();
    const summary = summarizeTrackingPlatformManualRequiredProof(proof);

    expect(summary.rowCount).toBe(8);
    expect(summary.androidRows).toBe(4);
    expect(summary.iosRows).toBe(4);
    expect(summary.manualRequiredCount).toBe(4);
    expect(summary.authorityRequiredCount).toBe(2);
    expect(summary.productClaimReadyCount).toBe(0);
    expect(summary.nonClaims.noPhysicalDeviceClaim).toBe(true);
    expect(summary.nonClaims.noAuthorityEnrolledClaim).toBe(true);
  });

  it('preserves parent-visible and child-safe unsupported platform copy for every row', () => {
    const proof = buildTrackingPlatformManualRequiredProof();

    for (const row of proof.rows) {
      expect(row.proofArtifactRefs.length).toBeGreaterThan(0);
      expect(row.parentVisibleStatus.length).toBeGreaterThan(10);
      expect(row.childSafeStatus.length).toBeGreaterThan(10);
      expect(row.productClaimReady).toBe(false);
    }

    expect(proof.rows.map((row) => row.manualRequiredReason)).toEqual([
      'android-foreground-location-real-device-required',
      'android-background-location-real-device-required',
      'android-geofence-real-device-required',
      'android-device-status-emulator-scaffold-only',
      'ios-core-location-device-required',
      'ios-background-region-entitlement-required',
      'ios-region-monitoring-entitlement-required',
      'ios-simulator-package-mechanics-only',
    ]);
  });

  it('rejects product-ready or contract-proved states without matching proof shape', () => {
    const proof = buildTrackingPlatformManualRequiredProof();
    const [firstRow] = proof.rows;
    if (!firstRow) {
      throw new Error('fixture must include rows');
    }

    expect(() =>
      TrackingPlatformManualRequiredProofSchema.parse({
        ...proof,
        rows: [
          {
            ...firstRow,
            routeState: 'contract-proved',
            claimState: 'manual-required',
          },
        ],
      })
    ).toThrow(/contract-proved route states/iu);

    expect(() =>
      TrackingPlatformManualRequiredProofSchema.parse({
        ...proof,
        rows: [
          {
            ...firstRow,
            claimState: 'proved',
            routeState: 'manual-required',
          },
        ],
      })
    ).toThrow(/proved rows must use the contract-proved route state/iu);
  });
});
