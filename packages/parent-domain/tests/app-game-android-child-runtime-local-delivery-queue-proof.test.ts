import { describe, expect, it } from 'vitest';
import {
  AppGameAndroidChildRuntimeLocalDeliveryQueueProofSchema,
  createAppGameAndroidChildRuntimeLocalDeliveryQueueProof,
  summarizeAppGameAndroidChildRuntimeLocalDeliveryQueueProof,
} from '../src/app-game-android-child-runtime-local-delivery-queue-proof';

const Timestamp = '2026-06-08T23:40:00.000Z';

describe('app-game Android child runtime local delivery queue proof', () => {
  it('records package-local delivery queue and drain markers without service or provider claims', () => {
    const proof = createAppGameAndroidChildRuntimeLocalDeliveryQueueProof({
      deliveryIntakeState: 'package-local-delivery-intake-recorded',
      deliveryReadbackState: 'package-local-delivery-readback-observed',
      deliveryQueueState: 'package-local-delivery-queue-recorded',
      deliveryDrainState: 'package-local-delivery-drain-recorded',
      receiptChannelState: 'package-local-receipt-channel-recorded',
      receiptAppendState: 'local-receipt-append-recorded',
      receiptLocalAckState: 'local-receipt-ack-recorded',
      checkedAt: Timestamp,
    });

    expect(summarizeAppGameAndroidChildRuntimeLocalDeliveryQueueProof(proof)).toEqual({
      deliveryQueueState: 'package-local-delivery-queue-recorded',
      deliveryDrainState: 'package-local-delivery-drain-recorded',
      packageLocalDeliveryQueued: true,
      packageLocalDeliveryDrained: true,
      serviceDeliveryIngested: false,
      providerDeliveryExecuted: false,
      openGapCount: 7,
    });
    expect(proof.proofRefs).toEqual([
      'android-child-runtime-package-local-delivery-intake-ref',
      'android-child-runtime-package-local-delivery-readback-ref',
      'android-child-runtime-package-local-delivery-queue-ref',
      'android-child-runtime-package-local-delivery-drain-ref',
      'android-child-runtime-package-local-receipt-channel-ref',
      'android-child-runtime-local-receipt-write-ref',
      'android-child-runtime-local-receipt-ack-write-ref',
    ]);
  });

  it('rejects missing queue/drain custody and delivery or enforcement overclaims', () => {
    const proof = createAppGameAndroidChildRuntimeLocalDeliveryQueueProof({
      deliveryIntakeState: 'package-local-delivery-intake-recorded',
      deliveryReadbackState: 'package-local-delivery-readback-observed',
      deliveryQueueState: 'package-local-delivery-queue-recorded',
      deliveryDrainState: 'package-local-delivery-drain-recorded',
      receiptChannelState: 'package-local-receipt-channel-recorded',
      receiptAppendState: 'local-receipt-append-recorded',
      receiptLocalAckState: 'local-receipt-ack-recorded',
      checkedAt: Timestamp,
    });

    for (const invalid of [
      { ...proof, deliveryQueueState: 'package-local-delivery-queue-unavailable' },
      { ...proof, deliveryDrainState: 'package-local-delivery-drain-unavailable' },
      { ...proof, packageLocalDeliveryQueueRecordCount: 0 },
      { ...proof, packageLocalDeliveryDrainRecordCount: 0 },
      { ...proof, packageLocalDeliveryQueued: false },
      { ...proof, packageLocalDeliveryDrained: false },
      { ...proof, serviceDeliveryIngested: true },
      { ...proof, serviceReceiptIngested: true },
      { ...proof, providerDeliveryExecuted: true },
      { ...proof, platformDeliveryChannelClaimedOutsidePackage: true },
      { ...proof, adapterDispatchClaimed: true },
      { ...proof, platformEnforcementClaimed: true },
      { ...proof, rawPrivateSourceRowsIncluded: true },
    ]) {
      expect(AppGameAndroidChildRuntimeLocalDeliveryQueueProofSchema.safeParse(invalid).success).toBe(false);
    }
  });
});
