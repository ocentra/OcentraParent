import { describe, expect, it } from 'vitest';
import {
  AppGameAndroidChildRuntimeLocalNotificationProofSchema,
  createAppGameAndroidChildRuntimeLocalNotificationProof,
  summarizeAppGameAndroidChildRuntimeLocalNotificationProof,
} from '../src/app-game-android-child-runtime-local-notification-proof';

describe('app-game Android child runtime local notification proof', () => {
  it('accepts package-local channel, notification post, and marker readback evidence', () => {
    const proof = createAppGameAndroidChildRuntimeLocalNotificationProof({
      notificationSeenInSystemUi: true,
      checkedAt: '2026-06-08T22:00:00.000Z',
    });
    const summary = summarizeAppGameAndroidChildRuntimeLocalNotificationProof(proof);

    expect(summary.notificationChannelState).toBe('local-notification-channel-declared');
    expect(summary.notificationPostState).toBe('local-notification-post-recorded');
    expect(summary.notificationMarkerState).toBe('local-notification-marker-recorded');
    expect(summary.notificationSeenInSystemUi).toBe(true);
    expect(proof.markerReadbackObserved).toBe(true);
  });

  it('keeps provider, external platform delivery, dispatch, and enforcement gaps explicit', () => {
    const proof = createAppGameAndroidChildRuntimeLocalNotificationProof({
      notificationSeenInSystemUi: false,
      checkedAt: '2026-06-08T22:00:00.000Z',
    });

    expect(proof.openGaps).toEqual(
      expect.arrayContaining([
        'provider-delivery-not-proved',
        'platform-delivery-outside-package-not-proved',
        'adapter-dispatch-not-proved',
        'platform-enforcement-not-proved',
      ])
    );
  });

  it('rejects missing marker readback and delivery, dispatch, enforcement, or raw-row claims', () => {
    const proof = createAppGameAndroidChildRuntimeLocalNotificationProof({
      notificationSeenInSystemUi: true,
      checkedAt: '2026-06-08T22:00:00.000Z',
    });

    expect(
      AppGameAndroidChildRuntimeLocalNotificationProofSchema.safeParse({
        ...proof,
        markerReadbackObserved: false,
      }).success
    ).toBe(false);
    expect(
      AppGameAndroidChildRuntimeLocalNotificationProofSchema.safeParse({
        ...proof,
        providerDeliveryClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameAndroidChildRuntimeLocalNotificationProofSchema.safeParse({
        ...proof,
        platformDeliveryOutsidePackageClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameAndroidChildRuntimeLocalNotificationProofSchema.safeParse({
        ...proof,
        adapterDispatchClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameAndroidChildRuntimeLocalNotificationProofSchema.safeParse({
        ...proof,
        platformEnforcementClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameAndroidChildRuntimeLocalNotificationProofSchema.safeParse({
        ...proof,
        rawPrivateSourceRowsStored: true,
      }).success
    ).toBe(false);
  });
});
