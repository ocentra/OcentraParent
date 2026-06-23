import { describe, expect, it } from 'vitest';
import {
  AppGameAndroidChildRuntimeLocalReceiptProofSchema,
  createAppGameAndroidChildRuntimeLocalReceiptProof,
  summarizeAppGameAndroidChildRuntimeLocalReceiptProof,
} from '@ocentra-parent/schema-domain/app-game-android-child-runtime-local-receipt-proof';

const Timestamp = '2026-06-08T22:20:00.000Z';

describe('app-game Android child runtime local receipt proof', () => {
  it('records package-local receipt append and readback without service or provider delivery claims', () => {
    const proof = createAppGameAndroidChildRuntimeLocalReceiptProof({
      receiptStoreState: 'internal-receipt-store-available',
      receiptAppendState: 'local-receipt-append-recorded',
      receiptReadbackState: 'local-receipt-readback-observed',
      packageActivityVisible: true,
      uiReceiptAppendStateObserved: true,
      uiReceiptReadbackStateObserved: true,
      checkedAt: Timestamp,
    });

    expect(summarizeAppGameAndroidChildRuntimeLocalReceiptProof(proof)).toEqual({
      receiptStoreState: 'internal-receipt-store-available',
      receiptAppendState: 'local-receipt-append-recorded',
      receiptReadbackState: 'local-receipt-readback-observed',
      localReceiptRecordCount: 1,
      localReceiptAppendExecuted: true,
      localReceiptReadbackObserved: true,
      serviceReceiptIngested: false,
      openGapCount: 6,
    });
    expect(proof.proofRefs).toEqual([
      'android-child-runtime-internal-receipt-store-ref',
      'android-child-runtime-local-receipt-write-ref',
      'android-child-runtime-local-receipt-readback-ref',
      'android-child-runtime-status-ui-ref',
    ]);
    expect(proof.openGaps).toEqual([
      'android-child-runtime-transport-not-executed',
      'android-child-runtime-receipt-not-ingested-by-service',
      'android-provider-delivery-not-executed',
      'android-platform-delivery-channel-not-proved',
      'android-adapter-dispatch-not-proved',
      'android-platform-enforcement-not-proved',
    ]);
    expect(proof.localReceiptAppendExecuted).toBe(true);
    expect(proof.localReceiptReadbackObserved).toBe(true);
    expect(proof.runtimeTransportExecuted).toBe(false);
    expect(proof.serviceReceiptIngested).toBe(false);
    expect(proof.providerDeliveryExecuted).toBe(false);
  });

  it('rejects missing local receipt proof and transport or enforcement overclaims', () => {
    const proof = createAppGameAndroidChildRuntimeLocalReceiptProof({
      receiptStoreState: 'internal-receipt-store-available',
      receiptAppendState: 'local-receipt-append-recorded',
      receiptReadbackState: 'local-receipt-readback-observed',
      packageActivityVisible: true,
      uiReceiptAppendStateObserved: true,
      uiReceiptReadbackStateObserved: true,
      checkedAt: Timestamp,
    });

    for (const invalid of [
      { ...proof, receiptStoreState: 'internal-receipt-store-unavailable' },
      { ...proof, receiptAppendState: 'local-receipt-append-unavailable' },
      { ...proof, receiptReadbackState: 'local-receipt-readback-unavailable' },
      { ...proof, localReceiptRecordCount: 0 },
      { ...proof, packageActivityVisible: false },
      { ...proof, uiReceiptAppendStateObserved: false },
      { ...proof, uiReceiptReadbackStateObserved: false },
      { ...proof, localReceiptAppendExecuted: false },
      { ...proof, localReceiptReadbackObserved: false },
      { ...proof, runtimeTransportExecuted: true },
      { ...proof, serviceReceiptIngested: true },
      { ...proof, providerDeliveryExecuted: true },
      { ...proof, platformDeliveryChannelClaimed: true },
      { ...proof, adapterDispatchClaimed: true },
      { ...proof, platformEnforcementClaimed: true },
      { ...proof, rawPrivateSourceRowsIncluded: true },
    ]) {
      expect(AppGameAndroidChildRuntimeLocalReceiptProofSchema.safeParse(invalid).success).toBe(false);
    }
  });
});
