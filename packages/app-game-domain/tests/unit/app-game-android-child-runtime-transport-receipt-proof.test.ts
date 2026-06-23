import { describe, expect, it } from 'vitest';
import {
  AppGameAndroidChildRuntimeTransportReceiptProofSchema,
  createAppGameAndroidChildRuntimeTransportReceiptProof,
  summarizeAppGameAndroidChildRuntimeTransportReceiptProof,
} from '@ocentra-parent/schema-domain/app-game-android-child-runtime-transport-receipt-proof';

const Timestamp = '2026-06-08T21:45:00.000Z';

describe('app-game Android child runtime transport receipt proof', () => {
  it('records parent-safe Android child runtime transport and receipt readiness without execution claims', () => {
    const proof = createAppGameAndroidChildRuntimeTransportReceiptProof({
      transportChannelState: 'activity-visible-transport-channel',
      receiptStoreState: 'internal-receipt-store-available',
      receiptAckState: 'receipt-ack-waiting-for-runtime',
      packageActivityVisible: true,
      uiTransportStateObserved: true,
      uiReceiptStateObserved: true,
      checkedAt: Timestamp,
    });

    expect(summarizeAppGameAndroidChildRuntimeTransportReceiptProof(proof)).toEqual({
      transportChannelState: 'activity-visible-transport-channel',
      receiptStoreState: 'internal-receipt-store-available',
      receiptAckState: 'receipt-ack-waiting-for-runtime',
      packageActivityVisible: true,
      uiTransportStateObserved: true,
      uiReceiptStateObserved: true,
      openGapCount: 6,
    });
    expect(proof.proofRefs).toEqual([
      'android-child-runtime-activity-transport-ref',
      'android-child-runtime-internal-receipt-store-ref',
      'android-child-runtime-status-ui-ref',
    ]);
    expect(proof.openGaps).toEqual([
      'android-child-runtime-transport-not-executed',
      'android-child-runtime-receipt-not-ingested',
      'android-provider-delivery-not-executed',
      'android-platform-delivery-channel-not-proved',
      'android-adapter-dispatch-not-proved',
      'android-platform-enforcement-not-proved',
    ]);
    expect(proof.runtimeTransportExecuted).toBe(false);
    expect(proof.runtimeReceiptIngested).toBe(false);
    expect(proof.rawPrivateSourceRowsIncluded).toBe(false);
  });

  it('rejects missing activity UI receipt-store evidence and execution overclaims', () => {
    const proof = createAppGameAndroidChildRuntimeTransportReceiptProof({
      transportChannelState: 'activity-visible-transport-channel',
      receiptStoreState: 'internal-receipt-store-available',
      receiptAckState: 'receipt-ack-waiting-for-runtime',
      packageActivityVisible: true,
      uiTransportStateObserved: true,
      uiReceiptStateObserved: true,
      checkedAt: Timestamp,
    });

    for (const invalid of [
      { ...proof, packageActivityVisible: false },
      { ...proof, uiTransportStateObserved: false },
      { ...proof, uiReceiptStateObserved: false },
      { ...proof, transportChannelState: 'activity-unavailable-transport-channel' },
      { ...proof, receiptStoreState: 'internal-receipt-store-unavailable' },
      { ...proof, runtimeTransportExecuted: true },
      { ...proof, runtimeReceiptIngested: true },
      { ...proof, providerDeliveryExecuted: true },
      { ...proof, platformDeliveryChannelClaimed: true },
      { ...proof, adapterDispatchClaimed: true },
      { ...proof, platformEnforcementClaimed: true },
      { ...proof, rawPrivateSourceRowsIncluded: true },
    ]) {
      expect(AppGameAndroidChildRuntimeTransportReceiptProofSchema.safeParse(invalid).success).toBe(false);
    }
  });
});
