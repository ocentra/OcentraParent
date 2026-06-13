import { describe, expect, it } from 'vitest';
import {
  TrackingUnsupportedPlatformManualProofRowSchema,
  buildTrackingUnsupportedPlatformManualProof,
} from '../../src/tracking-unsupported-platform-manual-proof';

describe('tracking unsupported platform manual proof', () => {
  it('builds manual-required and unavailable rows without unsupported capability claims', () => {
    const proof = buildTrackingUnsupportedPlatformManualProof('2026-06-05T19:55:00.000Z');

    expect(proof.rows).toHaveLength(7);
    expect(proof.allRowsAvoidFakeCapability).toBe(true);
    expect(proof.allRowsKeepProductClaimBlocked).toBe(true);
    expect(proof.portalScreenshotClaimed).toBe(false);
    expect(proof.physicalDeviceProofClaimed).toBe(false);
    expect(proof.authorityProofClaimed).toBe(false);
    expect(proof.rows.every((row) => row.fakeCapabilityRendered === false)).toBe(true);
    expect(proof.rows.every((row) => row.productClaimReady === false)).toBe(true);
  });

  it('keeps Android and iOS background/geofence rows manual-required', () => {
    const proof = buildTrackingUnsupportedPlatformManualProof('2026-06-05T19:55:00.000Z');
    const mobileRows = proof.rows.filter(
      (row) =>
        (row.platform === 'android' || row.platform === 'ios') &&
        (row.surface === 'background-location' || row.surface === 'geofence-transition')
    );

    expect(mobileRows).toHaveLength(4);
    expect(mobileRows.every((row) => row.renderedState === 'manual-required')).toBe(true);
    expect(mobileRows.every((row) => row.requiredProofTier === 'P4_PHYSICAL_DEVICE')).toBe(true);
    expect(mobileRows.every((row) => row.physicalDeviceClaimed === false)).toBe(true);
    expect(mobileRows.every((row) => row.backgroundLocationClaimed === false)).toBe(true);
  });

  it('keeps unsupported web child-agent rows unavailable instead of capable', () => {
    const proof = buildTrackingUnsupportedPlatformManualProof('2026-06-05T19:55:00.000Z');
    const webRow = proof.rows.find((row) => row.rowId === 'tracking-web-child-agent-unavailable');

    expect(webRow?.platform).toBe('web');
    expect(webRow?.supportState).toBe('platform-unsupported');
    expect(webRow?.renderedState).toBe('unavailable');
    expect(webRow?.runtimeLocationClaimed).toBe(false);
    expect(webRow?.fakeCapabilityRendered).toBe(false);
  });

  it('rejects rows that render unsupported platforms as manual capability', () => {
    const invalid = TrackingUnsupportedPlatformManualProofRowSchema.safeParse({
      schemaVersion: 'v0.5-tracking',
      rowId: 'tracking-web-invalid',
      platform: 'web',
      surface: 'child-agent-location',
      requiredProofTier: 'P4_PHYSICAL_DEVICE',
      currentProofTier: 'P0_CONTRACT',
      supportState: 'platform-unsupported',
      renderedState: 'manual-required',
      manualProofCommand: 'collect web child-agent proof',
      proofArtifactRefs: ['tracking-web-invalid-ref'],
      reasonCodes: ['tracking-web-invalid-manual'],
      fakeCapabilityRendered: false,
      productClaimReady: false,
      runtimeLocationClaimed: false,
      backgroundLocationClaimed: false,
      geofenceRuntimeClaimed: false,
      physicalDeviceClaimed: false,
      authorityClaimed: false,
    });

    expect(invalid.success).toBe(false);
  });
});
