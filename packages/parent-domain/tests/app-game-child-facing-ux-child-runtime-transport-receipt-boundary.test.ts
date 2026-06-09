import { describe, expect, it } from 'vitest';
import {
  AppGameChildDeviceRuntimeWriterReadModelSchema,
  AppGameChildDeviceRuntimeWriterState,
} from '../src/app-game-child-facing-ux-child-device-runtime-writer';
import {
  AppGameChildRuntimeTransportReceiptBoundaryReadModelSchema,
  AppGameChildRuntimeTransportReceiptBoundaryState,
  buildAppGameChildRuntimeTransportReceiptBoundaryReadModel,
  summarizeAppGameChildRuntimeTransportReceiptBoundary,
} from '../src/app-game-child-facing-ux-child-runtime-transport-receipt-boundary';

const Timestamp = '2026-06-08T23:05:00Z';

const Options = {
  generatedAt: Timestamp,
  boundaryId: 'app-game-child-runtime-transport-receipt-boundary-proof',
  receiptContractRefs: [
    'child-runtime-delivery-receipt-contract-ref',
    'child-runtime-delivery-receipt-storage-ref',
  ],
} as const;

describe('app/game child runtime transport receipt boundary', () => {
  it('maps writer-envelope-ready rows into transport-required receipt boundary rows', () => {
    const readModel = buildAppGameChildRuntimeTransportReceiptBoundaryReadModel(Options, runtimeWriterFixture());

    expect(summarizeAppGameChildRuntimeTransportReceiptBoundary(readModel)).toEqual({
      transportRequiredCount: 2,
      manualRequiredCount: 1,
      unavailableCount: 1,
      rowCount: 4,
    });
    expect(readModel.rows.map((row) => row.boundaryState)).toEqual([
      AppGameChildRuntimeTransportReceiptBoundaryState.TransportRequired,
      AppGameChildRuntimeTransportReceiptBoundaryState.TransportRequired,
      AppGameChildRuntimeTransportReceiptBoundaryState.ManualRequired,
      AppGameChildRuntimeTransportReceiptBoundaryState.Unavailable,
    ]);
    expect(readModel.rows.slice(0, 2).every((row) => row.requiredReceiptRefs.length === 2)).toBe(true);
    expect(readModel.rows.every((row) => row.openGaps.includes('child-runtime-transport-not-executed'))).toBe(true);
  });

  it('keeps manual-required and unavailable writer rows blocked before transport execution', () => {
    const readModel = buildAppGameChildRuntimeTransportReceiptBoundaryReadModel(Options, runtimeWriterFixture());
    const manual = readModel.rows[2];
    const unavailable = readModel.rows[3];

    expect(manual.boundaryState).toBe(AppGameChildRuntimeTransportReceiptBoundaryState.ManualRequired);
    expect(manual.requiredTransportRefs).toEqual([
      'manual-proof-required',
      'child-runtime-transport-not-executed',
    ]);
    expect(unavailable.boundaryState).toBe(AppGameChildRuntimeTransportReceiptBoundaryState.Unavailable);
    expect(unavailable.requiredReceiptRefs).toEqual([
      'source-unavailable',
      'child-runtime-transport-not-executed',
    ]);
  });

  it('rejects runtime transport receipt provider platform adapter enforcement and raw-source overclaims', () => {
    const readModel = buildAppGameChildRuntimeTransportReceiptBoundaryReadModel(Options, runtimeWriterFixture());
    const row = readModel.rows[0];

    for (const invalid of [
      { ...readModel, runtimeTransportExecuted: true },
      { ...readModel, runtimeReceiptIngested: true },
      { ...readModel, providerDeliveryExecuted: true },
      { ...readModel, adapterDispatchClaimed: true },
      { ...readModel, platformEnforcementClaimed: true },
      { ...readModel, rows: [{ ...row, runtimeTransportExecuted: true }, ...readModel.rows.slice(1)] },
      { ...readModel, rows: [{ ...row, runtimeReceiptIngested: true }, ...readModel.rows.slice(1)] },
      { ...readModel, rows: [{ ...row, platformDeliveryChannelClaimed: true }, ...readModel.rows.slice(1)] },
    ]) {
      expect(AppGameChildRuntimeTransportReceiptBoundaryReadModelSchema.safeParse(invalid).success).toBe(false);
    }
  });
});

function runtimeWriterFixture() {
  return AppGameChildDeviceRuntimeWriterReadModelSchema.parse({
    schemaVersion: 'v0.6',
    runtimeWriterId: 'app-game-child-device-runtime-writer-proof',
    generatedAt: Timestamp,
    family: { familyId: 'family-child-runtime-transport-receipt' },
    sourceDeliveryReadinessId: 'app-game-child-device-delivery-readiness-proof',
    rows: [
      writerRow('limit-reached', AppGameChildDeviceRuntimeWriterState.EnvelopeReady),
      writerRow('request-submitted', AppGameChildDeviceRuntimeWriterState.EnvelopeReady),
      writerRow('manual-required', AppGameChildDeviceRuntimeWriterState.ManualRequired),
      writerRow('unavailable', AppGameChildDeviceRuntimeWriterState.Unavailable),
    ],
    writerEnvelopeReadyCount: 2,
    manualRequiredCount: 1,
    unavailableCount: 1,
    nonClaims: [
      'no-runtime-writer-execution',
      'no-child-runtime-transport',
      'no-child-runtime-receipt-ingestion',
      'no-provider-delivery-execution',
      'no-platform-delivery-channel',
      'no-adapter-dispatch',
      'no-platform-enforcement',
      'no-raw-private-source-rows',
    ],
    runtimeWriterExecuted: false,
    childRuntimeTransportAttached: false,
    childRuntimeReceiptIngested: false,
    providerDeliveryExecuted: false,
    platformDeliveryChannelClaimed: false,
    adapterDispatchClaimed: false,
    platformEnforcementClaimed: false,
    rawPrivateSourceRowsIncluded: false,
  });
}

function writerRow(suffix: string, writerEnvelopeState: AppGameChildDeviceRuntimeWriterState) {
  return {
    runtimeWriterRowId: `app-game-child-device-runtime-writer-${suffix}`,
    sourceDeliveryReadinessRowId: `app-game-child-device-delivery-readiness-${suffix}`,
    sourceDeliveryReadinessStatus:
      writerEnvelopeState === AppGameChildDeviceRuntimeWriterState.EnvelopeReady
        ? 'child-transport-required'
        : writerEnvelopeState,
    writerEnvelopeState,
    childDeliveryTargetRefs:
      writerEnvelopeState === AppGameChildDeviceRuntimeWriterState.EnvelopeReady
        ? [
            'child-runtime-transport-contract-ref',
            'child-runtime-receipt-contract-ref',
            'child-device-local-agent-route-ref',
          ]
        : [blockedTargetRefFor(writerEnvelopeState)],
    runtimeWriterAuditRefs: [`app-game-child-device-runtime-writer-audit-${suffix}`],
    runtimeWriterExecuted: false,
    childRuntimeTransportAttached: false,
    childRuntimeReceiptIngested: false,
    providerDeliveryExecuted: false,
    platformDeliveryChannelClaimed: false,
  };
}

function blockedTargetRefFor(writerEnvelopeState: AppGameChildDeviceRuntimeWriterState): string {
  return writerEnvelopeState === AppGameChildDeviceRuntimeWriterState.Unavailable
    ? 'source-unavailable'
    : 'manual-proof-required';
}
