import { describe, expect, it } from 'vitest';
import {
  AppGameAndroidChildRuntimeLocalReceiptChannelProofSchema,
  createAppGameAndroidChildRuntimeLocalReceiptChannelProof,
  summarizeAppGameAndroidChildRuntimeLocalReceiptChannelProof,
} from '../src/app-game-android-child-runtime-local-receipt-channel-proof';

const Timestamp = '2026-06-08T23:20:00.000Z';

describe('app-game Android child runtime local receipt channel proof', () => {
  it('records an in-package receipt channel without provider or service delivery claims', () => {
    const proof = createAppGameAndroidChildRuntimeLocalReceiptChannelProof({
      receiptChannelState: 'package-local-receipt-channel-recorded',
      receiptAppendState: 'local-receipt-append-recorded',
      receiptLocalAckState: 'local-receipt-ack-recorded',
      packageLocalBroadcastReceiverDeclared: true,
      packageLocalBroadcastTriggeredByActivity: true,
      checkedAt: Timestamp,
    });

    expect(summarizeAppGameAndroidChildRuntimeLocalReceiptChannelProof(proof)).toEqual({
      receiptChannelState: 'package-local-receipt-channel-recorded',
      packageLocalBroadcastReceiverDeclared: true,
      packageLocalBroadcastTriggeredByActivity: true,
      packageLocalChannelExecuted: true,
      serviceReceiptIngested: false,
      providerDeliveryExecuted: false,
      openGapCount: 6,
    });
    expect(proof.proofRefs).toEqual([
      'android-child-runtime-package-local-receipt-channel-ref',
      'android-child-runtime-local-receipt-write-ref',
      'android-child-runtime-local-receipt-ack-write-ref',
      'android-child-runtime-manifest-receiver-ref',
      'android-child-runtime-activity-trigger-ref',
    ]);
    expect(proof.serviceReceiptIngested).toBe(false);
    expect(proof.providerDeliveryExecuted).toBe(false);
    expect(proof.platformDeliveryChannelClaimedOutsidePackage).toBe(false);
  });

  it('rejects missing receiver/trigger evidence and delivery or enforcement overclaims', () => {
    const proof = createAppGameAndroidChildRuntimeLocalReceiptChannelProof({
      receiptChannelState: 'package-local-receipt-channel-recorded',
      receiptAppendState: 'local-receipt-append-recorded',
      receiptLocalAckState: 'local-receipt-ack-recorded',
      packageLocalBroadcastReceiverDeclared: true,
      packageLocalBroadcastTriggeredByActivity: true,
      checkedAt: Timestamp,
    });

    for (const invalid of [
      { ...proof, receiptChannelState: 'package-local-receipt-channel-unavailable' },
      { ...proof, receiptAppendState: 'local-receipt-append-unavailable' },
      { ...proof, receiptLocalAckState: 'local-receipt-ack-unavailable' },
      { ...proof, packageLocalBroadcastReceiverDeclared: false },
      { ...proof, packageLocalBroadcastTriggeredByActivity: false },
      { ...proof, packageLocalChannelRecordCount: 0 },
      { ...proof, localReceiptRecordCount: 0 },
      { ...proof, localReceiptAckRecordCount: 0 },
      { ...proof, packageLocalChannelExecuted: false },
      { ...proof, runtimeTransportExecutedOutsidePackage: true },
      { ...proof, serviceReceiptIngested: true },
      { ...proof, providerDeliveryExecuted: true },
      { ...proof, platformDeliveryChannelClaimedOutsidePackage: true },
      { ...proof, adapterDispatchClaimed: true },
      { ...proof, platformEnforcementClaimed: true },
      { ...proof, rawPrivateSourceRowsIncluded: true },
    ]) {
      expect(AppGameAndroidChildRuntimeLocalReceiptChannelProofSchema.safeParse(invalid).success).toBe(false);
    }
  });
});
