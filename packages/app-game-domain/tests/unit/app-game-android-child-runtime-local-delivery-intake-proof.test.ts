import { describe, expect, it } from 'vitest';
import {
  AppGameAndroidChildRuntimeLocalDeliveryIntakeProofSchema,
  createAppGameAndroidChildRuntimeLocalDeliveryIntakeProof,
  summarizeAppGameAndroidChildRuntimeLocalDeliveryIntakeProof,
} from '@ocentra-parent/schema-domain/app-game-android-child-runtime-local-delivery-intake-proof';

const Timestamp = '2026-06-08T23:30:00.000Z';

describe('app-game Android child runtime local delivery intake proof', () => {
  it('records package-local delivery intake with receipt channel evidence and no external delivery claims', () => {
    const proof = createAppGameAndroidChildRuntimeLocalDeliveryIntakeProof({
      deliveryIntakeState: 'package-local-delivery-intake-recorded',
      deliveryReadbackState: 'package-local-delivery-readback-observed',
      receiptChannelState: 'package-local-receipt-channel-recorded',
      receiptAppendState: 'local-receipt-append-recorded',
      receiptLocalAckState: 'local-receipt-ack-recorded',
      packageLocalDeliveryReceiverDeclared: true,
      packageLocalDeliveryTriggeredByActivity: true,
      checkedAt: Timestamp,
    });

    expect(summarizeAppGameAndroidChildRuntimeLocalDeliveryIntakeProof(proof)).toEqual({
      deliveryIntakeState: 'package-local-delivery-intake-recorded',
      deliveryReadbackState: 'package-local-delivery-readback-observed',
      packageLocalDeliveryReceiverDeclared: true,
      packageLocalDeliveryTriggeredByActivity: true,
      packageLocalDeliveryExecuted: true,
      serviceDeliveryIngested: false,
      providerDeliveryExecuted: false,
      openGapCount: 7,
    });
    expect(proof.proofRefs).toEqual([
      'android-child-runtime-package-local-delivery-intake-ref',
      'android-child-runtime-package-local-delivery-readback-ref',
      'android-child-runtime-package-local-receipt-channel-ref',
      'android-child-runtime-local-receipt-write-ref',
      'android-child-runtime-local-receipt-ack-write-ref',
      'android-child-runtime-package-local-delivery-receiver-ref',
      'android-child-runtime-package-local-delivery-activity-trigger-ref',
    ]);
    expect(proof.serviceDeliveryIngested).toBe(false);
    expect(proof.providerDeliveryExecuted).toBe(false);
    expect(proof.platformDeliveryChannelClaimedOutsidePackage).toBe(false);
  });

  it('rejects missing delivery evidence and service provider or enforcement overclaims', () => {
    const proof = createAppGameAndroidChildRuntimeLocalDeliveryIntakeProof({
      deliveryIntakeState: 'package-local-delivery-intake-recorded',
      deliveryReadbackState: 'package-local-delivery-readback-observed',
      receiptChannelState: 'package-local-receipt-channel-recorded',
      receiptAppendState: 'local-receipt-append-recorded',
      receiptLocalAckState: 'local-receipt-ack-recorded',
      packageLocalDeliveryReceiverDeclared: true,
      packageLocalDeliveryTriggeredByActivity: true,
      checkedAt: Timestamp,
    });

    for (const invalid of [
      { ...proof, deliveryIntakeState: 'package-local-delivery-intake-unavailable' },
      { ...proof, deliveryReadbackState: 'package-local-delivery-readback-unavailable' },
      { ...proof, receiptChannelState: 'package-local-receipt-channel-unavailable' },
      { ...proof, receiptAppendState: 'local-receipt-append-unavailable' },
      { ...proof, receiptLocalAckState: 'local-receipt-ack-unavailable' },
      { ...proof, packageLocalDeliveryReceiverDeclared: false },
      { ...proof, packageLocalDeliveryTriggeredByActivity: false },
      { ...proof, packageLocalDeliveryRecordCount: 0 },
      { ...proof, packageLocalChannelRecordCount: 0 },
      { ...proof, localReceiptRecordCount: 0 },
      { ...proof, localReceiptAckRecordCount: 0 },
      { ...proof, packageLocalDeliveryExecuted: false },
      { ...proof, serviceDeliveryIngested: true },
      { ...proof, serviceReceiptIngested: true },
      { ...proof, providerDeliveryExecuted: true },
      { ...proof, platformDeliveryChannelClaimedOutsidePackage: true },
      { ...proof, adapterDispatchClaimed: true },
      { ...proof, platformEnforcementClaimed: true },
      { ...proof, rawPrivateSourceRowsIncluded: true },
    ]) {
      expect(AppGameAndroidChildRuntimeLocalDeliveryIntakeProofSchema.safeParse(invalid).success).toBe(false);
    }
  });
});
