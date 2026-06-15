import { describe, expect, it } from 'vitest';
import {
  AppGameAndroidChildRuntimeLocalNotificationActionProofSchema,
  createAppGameAndroidChildRuntimeLocalNotificationActionProof,
  summarizeAppGameAndroidChildRuntimeLocalNotificationActionProof,
} from '../../src/app-game-android-child-runtime-local-notification-action-proof';

const expectRejectedNotificationAction = (candidate: unknown): void => {
  expect(AppGameAndroidChildRuntimeLocalNotificationActionProofSchema.safeParse(candidate).success).toBe(false);
};

describe('app-game Android child runtime local notification action proof', () => {
  it('accepts package-local notification action and request marker readback evidence', () => {
    const proof = createAppGameAndroidChildRuntimeLocalNotificationActionProof({
      notificationSeenInSystemUi: true,
      checkedAt: '2026-06-08T22:30:00.000Z',
    });
    const summary = summarizeAppGameAndroidChildRuntimeLocalNotificationActionProof(proof);

    expect(summary.notificationChannelState).toBe('local-notification-channel-declared');
    expect(summary.notificationPostState).toBe('local-notification-post-recorded');
    expect(summary.notificationRequestActionState).toBe('local-notification-request-action-recorded');
    expect(summary.notificationRequestActionMarkerState).toBe('local-notification-request-action-marker-recorded');
    expect(summary.notificationSeenInSystemUi).toBe(true);
    expect(proof.requestActionReadbackObserved).toBe(true);
  });

  it('keeps service ingestion, approval, provider, dispatch, and enforcement gaps explicit', () => {
    const proof = createAppGameAndroidChildRuntimeLocalNotificationActionProof({
      notificationSeenInSystemUi: false,
      checkedAt: '2026-06-08T22:30:00.000Z',
    });

    expect(proof.openGaps).toEqual(
      expect.arrayContaining([
        'provider-delivery-not-proved',
        'platform-delivery-outside-package-not-proved',
        'service-request-ingestion-not-proved',
        'parent-approval-round-trip-not-proved',
        'adapter-dispatch-not-proved',
        'platform-enforcement-not-proved',
      ])
    );
  });

  it('rejects missing action readback and delivery, approval, dispatch, enforcement, or raw-row claims', () => {
    const proof = createAppGameAndroidChildRuntimeLocalNotificationActionProof({
      notificationSeenInSystemUi: true,
      checkedAt: '2026-06-08T22:30:00.000Z',
    });

    expectRejectedNotificationAction({ ...proof, requestActionReadbackObserved: false });
    expectRejectedNotificationAction({ ...proof, serviceRequestIngestionClaimed: true });
    expectRejectedNotificationAction({ ...proof, parentApprovalRoundTripClaimed: true });
    expectRejectedNotificationAction({ ...proof, providerDeliveryClaimed: true });
    expectRejectedNotificationAction({ ...proof, platformDeliveryOutsidePackageClaimed: true });
    expectRejectedNotificationAction({ ...proof, adapterDispatchClaimed: true });
    expectRejectedNotificationAction({ ...proof, platformEnforcementClaimed: true });
    expectRejectedNotificationAction({ ...proof, rawPrivateSourceRowsStored: true });
  });
});
