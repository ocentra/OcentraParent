import { describe, expect, it } from 'vitest';
import { createAppGameChildRuntimeTransportReceiptPanelIntent } from '../src/app-game-child-runtime-transport-receipt-panel';

const ReadModel = {
  schemaVersion: 1,
  readModelId: 'app-game-child-runtime-transport-receipt',
  generatedAt: '2026-06-08T20:45:00.000Z',
  sourceReadModelIds: ['app-game-child-device-runtime-writer'],
  custodyLabel: 'app-game-child-runtime-transport-receipt',
  capabilityStatus: 'child-runtime-transport-required',
  returned: 3,
  transportRequiredCount: 2,
  manualRequiredCount: 1,
  unavailableCount: 0,
  runtimeTransportExecuted: false,
  runtimeReceiptIngested: false,
  providerDeliveryExecuted: false,
  platformDeliveryChannelClaimed: false,
  adapterDispatchClaimed: false,
  platformEnforcementClaimed: false,
  rawPrivateSourceRowsIncluded: false,
  rows: [
    childRuntimeTransportReceiptRow(
      'app-game-child-runtime-transport-receipt-warning',
      'child-runtime-transport-required',
      ['child-runtime-warning-transport-ref'],
      ['child-runtime-warning-receipt-ref']
    ),
    childRuntimeTransportReceiptRow(
      'app-game-child-runtime-transport-receipt-request',
      'child-runtime-transport-required',
      ['child-runtime-request-transport-ref'],
      ['child-runtime-request-receipt-ref']
    ),
    childRuntimeTransportReceiptRow(
      'app-game-child-runtime-transport-receipt-apple',
      'manual-required',
      ['apple-child-runtime-ci-transport-ref'],
      ['apple-child-runtime-ci-receipt-ref']
    ),
  ],
} as const;

describe('app-game child runtime transport receipt panel intent', () => {
  it('renders child runtime transport readiness without delivery or receipt claims', () => {
    const intent = createAppGameChildRuntimeTransportReceiptPanelIntent(ReadModel);

    expect(intent.title).toBe('App/game child runtime transport receipts');
    expect(intent.loadState).toBe('Review');
    expect(intent.productClaim).toBe(
      'Child runtime transport receipt rows are parent-visible readiness only. Runtime transport execution, receipt ingestion, provider delivery, platform channel delivery, adapter dispatch, platform enforcement, and raw private rows remain unclaimed.'
    );
    expect(intent.summaryDetails).toEqual(
      expect.arrayContaining([
        expect.objectContaining({ label: 'Transport rows', value: '3' }),
        expect.objectContaining({ label: 'Transport-required rows', value: '2' }),
        expect.objectContaining({ label: 'Manual-required rows', value: '1' }),
        expect.objectContaining({ label: 'Runtime transport', value: 'Not claimed' }),
        expect.objectContaining({ label: 'Runtime receipt', value: 'Not claimed' }),
        expect.objectContaining({ label: 'Provider', value: 'Not claimed' }),
        expect.objectContaining({ label: 'Platform delivery', value: 'Not claimed' }),
        expect.objectContaining({ label: 'Raw private rows', value: 'Not claimed' }),
      ])
    );
    expect(intent.rows.map((row) => row.title)).toEqual([
      'app-game-child-runtime-transport-receipt-warning',
      'app-game-child-runtime-transport-receipt-request',
      'app-game-child-runtime-transport-receipt-apple',
    ]);
    expect(intent.rows[0]?.details).toEqual(
      expect.arrayContaining([
        expect.objectContaining({ label: 'Status', value: 'child-runtime-transport-required' }),
        expect.objectContaining({ label: 'Product meanings', value: 'native-app | native-game' }),
        expect.objectContaining({ label: 'Required transport refs', value: 'child-runtime-warning-transport-ref' }),
        expect.objectContaining({ label: 'Required receipt refs', value: 'child-runtime-warning-receipt-ref' }),
        expect.objectContaining({ label: 'Runtime transport', value: 'Not claimed' }),
        expect.objectContaining({ label: 'Runtime receipt', value: 'Not claimed' }),
      ])
    );
  });

  it('renders missing read models as unavailable without rows', () => {
    const intent = createAppGameChildRuntimeTransportReceiptPanelIntent(null);

    expect(intent.loadState).toBe('Unavailable');
    expect(intent.rows).toHaveLength(0);
    expect(intent.summaryDetails).toEqual(
      expect.arrayContaining([expect.objectContaining({ label: 'Status', value: 'Unavailable' })])
    );
  });
});

function childRuntimeTransportReceiptRow(
  rowId: string,
  boundaryState: string,
  requiredTransportRefs: readonly string[],
  requiredReceiptRefs: readonly string[]
) {
  return {
    schemaVersion: 1,
    rowId,
    sourceRuntimeWriterRowId: `${rowId}-source-writer`,
    boundaryState,
    productMeanings: ['native-app', 'native-game'],
    requiredTransportRefs,
    requiredReceiptRefs,
    openGaps: [
      'child-runtime-transport-not-executed',
      'child-runtime-receipt-not-ingested',
      'provider-delivery-not-executed',
    ],
    runtimeTransportExecuted: false,
    runtimeReceiptIngested: false,
    providerDeliveryExecuted: false,
    platformDeliveryChannelClaimed: false,
  };
}
