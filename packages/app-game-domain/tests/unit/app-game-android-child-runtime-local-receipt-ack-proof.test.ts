import { describe, expect, it } from 'vitest';
import {
  AppGameAndroidChildRuntimeLocalReceiptAckProofSchema,
  createAppGameAndroidChildRuntimeLocalReceiptAckProof,
  summarizeAppGameAndroidChildRuntimeLocalReceiptAckProof,
} from '../../src/app-game-android-child-runtime-local-receipt-ack-proof';

const Timestamp = '2026-06-08T23:10:00.000Z';

describe('app-game Android child runtime local receipt ack proof', () => {
  it('records package-local receipt and ack write/readback without service ingestion claims', () => {
    const proof = createAppGameAndroidChildRuntimeLocalReceiptAckProof({
      receiptAppendState: 'local-receipt-append-recorded',
      receiptReadbackState: 'local-receipt-readback-observed',
      receiptLocalAckState: 'local-receipt-ack-recorded',
      receiptLocalAckReadbackState: 'local-receipt-ack-readback-observed',
      packageActivityVisible: true,
      uiReceiptAckStateObserved: true,
      uiReceiptAckReadbackStateObserved: true,
      checkedAt: Timestamp,
    });

    expect(summarizeAppGameAndroidChildRuntimeLocalReceiptAckProof(proof)).toEqual({
      receiptAppendState: 'local-receipt-append-recorded',
      receiptReadbackState: 'local-receipt-readback-observed',
      receiptLocalAckState: 'local-receipt-ack-recorded',
      receiptLocalAckReadbackState: 'local-receipt-ack-readback-observed',
      localReceiptRecordCount: 1,
      localReceiptAckRecordCount: 1,
      serviceReceiptIngested: false,
      openGapCount: 6,
    });
    expect(proof.proofRefs).toEqual([
      'android-child-runtime-local-receipt-write-ref',
      'android-child-runtime-local-receipt-readback-ref',
      'android-child-runtime-local-receipt-ack-write-ref',
      'android-child-runtime-local-receipt-ack-readback-ref',
      'android-child-runtime-status-ui-ref',
    ]);
    expect(proof.serviceReceiptIngested).toBe(false);
    expect(proof.providerDeliveryExecuted).toBe(false);
    expect(proof.platformDeliveryChannelClaimed).toBe(false);
  });

  it('rejects missing ack evidence and delivery or enforcement overclaims', () => {
    const proof = createAppGameAndroidChildRuntimeLocalReceiptAckProof({
      receiptAppendState: 'local-receipt-append-recorded',
      receiptReadbackState: 'local-receipt-readback-observed',
      receiptLocalAckState: 'local-receipt-ack-recorded',
      receiptLocalAckReadbackState: 'local-receipt-ack-readback-observed',
      packageActivityVisible: true,
      uiReceiptAckStateObserved: true,
      uiReceiptAckReadbackStateObserved: true,
      checkedAt: Timestamp,
    });

    for (const invalid of [
      { ...proof, receiptAppendState: 'local-receipt-append-unavailable' },
      { ...proof, receiptReadbackState: 'local-receipt-readback-unavailable' },
      { ...proof, receiptLocalAckState: 'local-receipt-ack-unavailable' },
      { ...proof, receiptLocalAckReadbackState: 'local-receipt-ack-readback-unavailable' },
      { ...proof, localReceiptRecordCount: 0 },
      { ...proof, localReceiptAckRecordCount: 0 },
      { ...proof, packageActivityVisible: false },
      { ...proof, uiReceiptAckStateObserved: false },
      { ...proof, uiReceiptAckReadbackStateObserved: false },
      { ...proof, localReceiptAckExecuted: false },
      { ...proof, localReceiptAckReadbackObserved: false },
      { ...proof, runtimeTransportExecuted: true },
      { ...proof, serviceReceiptIngested: true },
      { ...proof, providerDeliveryExecuted: true },
      { ...proof, platformDeliveryChannelClaimed: true },
      { ...proof, adapterDispatchClaimed: true },
      { ...proof, platformEnforcementClaimed: true },
      { ...proof, rawPrivateSourceRowsIncluded: true },
    ]) {
      expect(AppGameAndroidChildRuntimeLocalReceiptAckProofSchema.safeParse(invalid).success).toBe(false);
    }
  });
});
