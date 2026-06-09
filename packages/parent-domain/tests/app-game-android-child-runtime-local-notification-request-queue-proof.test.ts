import { describe, expect, it } from 'vitest';
import {
  AppGameAndroidChildRuntimeLocalNotificationRequestQueueProofSchema,
  createAppGameAndroidChildRuntimeLocalNotificationRequestQueueProof,
  summarizeAppGameAndroidChildRuntimeLocalNotificationRequestQueueProof,
} from '../src/app-game-android-child-runtime-local-notification-request-queue-proof';

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

    expect(AppGameAndroidChildRuntimeLocalNotificationRequestQueueProofSchema.safeParse({
      ...proof,
      requestQueueReadbackObserved: false,
    }).success).toBe(false);
    expect(AppGameAndroidChildRuntimeLocalNotificationRequestQueueProofSchema.safeParse({
      ...proof,
      requestDrainReadbackObserved: false,
    }).success).toBe(false);
    expect(AppGameAndroidChildRuntimeLocalNotificationRequestQueueProofSchema.safeParse({
      ...proof,
      serviceRequestIngestionClaimed: true,
    }).success).toBe(false);
    expect(AppGameAndroidChildRuntimeLocalNotificationRequestQueueProofSchema.safeParse({
      ...proof,
      parentApprovalRoundTripClaimed: true,
    }).success).toBe(false);
    expect(AppGameAndroidChildRuntimeLocalNotificationRequestQueueProofSchema.safeParse({
      ...proof,
      providerDeliveryClaimed: true,
    }).success).toBe(false);
    expect(AppGameAndroidChildRuntimeLocalNotificationRequestQueueProofSchema.safeParse({
      ...proof,
      adapterDispatchClaimed: true,
    }).success).toBe(false);
    expect(AppGameAndroidChildRuntimeLocalNotificationRequestQueueProofSchema.safeParse({
      ...proof,
      platformEnforcementClaimed: true,
    }).success).toBe(false);
    expect(AppGameAndroidChildRuntimeLocalNotificationRequestQueueProofSchema.safeParse({
      ...proof,
      rawPrivateSourceRowsStored: true,
    }).success).toBe(false);
  });
});
