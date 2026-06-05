import { describe, expect, it } from 'vitest';
import {
  TrackingPlatformManualStateProofRowSchema,
  summarizeTrackingPlatformManualStateProof,
  trackingPlatformManualStateProofRows,
} from '../src/tracking-platform-manual-state-proof';

describe('tracking platform manual state proof', () => {
  it('parses every platform manual state row through the schema', () => {
    const rows = trackingPlatformManualStateProofRows();

    expect(rows).toHaveLength(12);
    expect(rows.every((row) => TrackingPlatformManualStateProofRowSchema.parse(row).productClaimReady === false)).toBe(
      true
    );
  });

  it('renders unsupported or unproved platform states as parent-visible non-claims', () => {
    const rows = trackingPlatformManualStateProofRows();

    expect(rows.find((row) => row.platform === 'web' && row.capability === 'child-runtime-delivery')).toMatchObject({
      proofState: 'unavailable',
      displayState: 'show-unavailable',
      productClaimReady: false,
    });
    expect(rows.find((row) => row.platform === 'android' && row.capability === 'background-location')).toMatchObject({
      proofState: 'manual-required',
      displayState: 'show-manual-required',
      productClaimReady: false,
    });
    expect(rows.find((row) => row.platform === 'windows' && row.capability === 'foreground-location')).toMatchObject({
      proofState: 'not-claimed',
      displayState: 'show-not-claimed',
      productClaimReady: false,
    });
  });

  it('summarizes proof without upgrading scaffold or manual states into capability claims', () => {
    const summary = summarizeTrackingPlatformManualStateProof();

    expect(summary.productClaimReady).toBe(false);
    expect(summary.fakeCapabilityRows).toEqual([]);
    expect(summary.manualRequiredCount).toBe(6);
    expect(summary.unavailableCount).toBe(2);
    expect(summary.notClaimedCount).toBe(3);
    expect(summary.scaffoldObservedCount).toBe(1);
  });
});
