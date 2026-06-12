import { describe, expect, it } from 'vitest';
import {
  AppGameAndroidAccessibilityEnabledSampleProofSchema,
  createAppGameAndroidAccessibilityEnabledSampleProof,
  summarizeAppGameAndroidAccessibilityEnabledSampleProof,
} from '../../src/app-game-android-accessibility-enabled-sample-proof';

const expectRejectedAccessibilityEnabledSample = (candidate: unknown): void => {
  expect(AppGameAndroidAccessibilityEnabledSampleProofSchema.safeParse(candidate).success).toBe(false);
};

describe('app-game Android Accessibility enabled sample proof', () => {
  it('accepts settings-enabled service and count-only event sample evidence', () => {
    const proof = createAppGameAndroidAccessibilityEnabledSampleProof({
      eventSampleCount: 3,
      checkedAt: '2026-06-08T21:50:00.000Z',
    });
    const summary = summarizeAppGameAndroidAccessibilityEnabledSampleProof(proof);

    expect(summary.runtimeState).toBe('accessibility-runtime-bound');
    expect(summary.eventSampleState).toBe('accessibility-event-sample-observed');
    expect(summary.eventSampleCount).toBe(3);
    expect(proof.serviceEnabledBySettings).toBe(true);
    expect(proof.rawAccessibilityEventRowsStored).toBe(false);
    expect(proof.rawOverlayContentStored).toBe(false);
    expect(proof.overlayRuntimeClaimed).toBe(false);
    expect(proof.adapterDispatchClaimed).toBe(false);
    expect(proof.platformEnforcementClaimed).toBe(false);
    expect(proof.childDeviceDeliveryClaimed).toBe(false);
  });

  it('keeps overlay, authority, delivery, and enforcement gaps explicit', () => {
    const proof = createAppGameAndroidAccessibilityEnabledSampleProof({
      eventSampleCount: 1,
      checkedAt: '2026-06-08T21:50:00.000Z',
    });

    expect(proof.openGaps).toEqual(
      expect.arrayContaining([
        'android-accessibility-overlay-runtime-not-proved',
        'android-device-owner-authority-not-proved',
        'android-play-policy-not-proved',
        'android-child-device-delivery-not-proved',
        'android-platform-enforcement-not-proved',
      ])
    );
  });

  it('rejects empty event samples and any raw, overlay, dispatch, delivery, or enforcement claims', () => {
    const proof = createAppGameAndroidAccessibilityEnabledSampleProof({
      eventSampleCount: 1,
      checkedAt: '2026-06-08T21:50:00.000Z',
    });

    expectRejectedAccessibilityEnabledSample({ ...proof, eventSampleCount: 0 });
    expectRejectedAccessibilityEnabledSample({ ...proof, rawAccessibilityEventRowsStored: true });
    expectRejectedAccessibilityEnabledSample({ ...proof, rawOverlayContentStored: true });
    expectRejectedAccessibilityEnabledSample({ ...proof, overlayRuntimeClaimed: true });
    expectRejectedAccessibilityEnabledSample({ ...proof, adapterDispatchClaimed: true });
    expectRejectedAccessibilityEnabledSample({ ...proof, platformEnforcementClaimed: true });
    expectRejectedAccessibilityEnabledSample({ ...proof, childDeviceDeliveryClaimed: true });
  });
});
