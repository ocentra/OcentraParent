import { describe, expect, it } from 'vitest';
import {
  AppGameAndroidChildRuntimeLocalNotificationRequestQueueProofSchema,
  createAppGameAndroidChildRuntimeLocalNotificationRequestQueueProof,
  summarizeAppGameAndroidChildRuntimeLocalNotificationRequestQueueProof,
} from '../../src/app-game-android-child-runtime-local-notification-request-queue-proof';

const expectRejectedNotificationRequestQueue = (candidate: unknown): void => {
  expect(AppGameAndroidChildRuntimeLocalNotificationRequestQueueProofSchema.safeParse(candidate).success).toBe(false);
};

describe('app-game Android child runtime local notification request queue proof', () => {
  it('accepts package-local request queue, readback, and drain evidence', () => {
    const proof = createAppGameAndroidChildRuntimeLocalNotificationRequestQueueProof({
      checkedAt: '2026-06-08T22:45:00.000Z',
    });
    const summary = summarizeAppGameAndroidChildRuntimeLocalNotificationRequestQueueProof(proof);

    expect(summary.notificationRequestQueueState).toBe('local-notification-request-queue-recorded');
    expect(summary.notificationRequestReadbackState).toBe('local-notification-request-readback-observed');
    expect(summary.notificationRequestDrainState).toBe('local-notification-request-drain-recorded');
    expect(proof.requestQueueReadbackObserved).toBe(true);
    expect(proof.requestDrainReadbackObserved).toBe(true);
  });

  it('keeps service ingestion, approval, provider, dispatch, and enforcement gaps explicit', () => {
    const proof = createAppGameAndroidChildRuntimeLocalNotificationRequestQueueProof({
      checkedAt: '2026-06-08T22:45:00.000Z',
    });

    expect(proof.openGaps).toEqual(
      expect.arrayContaining([
        'service-request-ingestion-not-proved',
        'parent-approval-round-trip-not-proved',
        'provider-delivery-not-proved',
        'platform-delivery-outside-package-not-proved',
        'adapter-dispatch-not-proved',
        'platform-enforcement-not-proved',
      ])
    );
  });

  it('rejects missing readback and service, approval, provider, dispatch, enforcement, or raw-row claims', () => {
    const proof = createAppGameAndroidChildRuntimeLocalNotificationRequestQueueProof({
      checkedAt: '2026-06-08T22:45:00.000Z',
    });

    expectRejectedNotificationRequestQueue({ ...proof, requestQueueReadbackObserved: false });
    expectRejectedNotificationRequestQueue({ ...proof, requestDrainReadbackObserved: false });
    expectRejectedNotificationRequestQueue({ ...proof, serviceRequestIngestionClaimed: true });
    expectRejectedNotificationRequestQueue({ ...proof, parentApprovalRoundTripClaimed: true });
    expectRejectedNotificationRequestQueue({ ...proof, providerDeliveryClaimed: true });
    expectRejectedNotificationRequestQueue({ ...proof, adapterDispatchClaimed: true });
    expectRejectedNotificationRequestQueue({ ...proof, platformEnforcementClaimed: true });
    expectRejectedNotificationRequestQueue({ ...proof, rawPrivateSourceRowsStored: true });
  });
});
