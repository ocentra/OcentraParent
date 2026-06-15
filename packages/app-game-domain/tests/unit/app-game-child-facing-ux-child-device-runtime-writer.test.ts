import { describe, expect, it } from 'vitest';
import {
  AppGameChildDeviceDeliveryReadinessReadModelSchema,
  AppGameChildDeviceDeliveryReadinessStatus,
} from '../../src/app-game-child-facing-ux-child-device-delivery-readiness';
import {
  AppGameChildDeviceRuntimeWriterReadModelSchema,
  AppGameChildDeviceRuntimeWriterState,
  buildAppGameChildDeviceRuntimeWriterReadModel,
  summarizeAppGameChildDeviceRuntimeWriter,
} from '../../src/app-game-child-facing-ux-child-device-runtime-writer';

const Timestamp = '2026-06-08T22:45:00Z';

describe('app/game child-device runtime writer', () => {
  it('maps transport-required readiness rows into writer envelopes without executing delivery', () => {
    const readModel = buildAppGameChildDeviceRuntimeWriterReadModel(
      {
        generatedAt: Timestamp,
        runtimeWriterId: 'app-game-child-device-runtime-writer-proof',
      },
      deliveryReadinessFixture()
    );

    expect(summarizeAppGameChildDeviceRuntimeWriter(readModel)).toEqual({
      writerEnvelopeReadyCount: 2,
      manualRequiredCount: 1,
      unavailableCount: 1,
      rowCount: 4,
    });
    expect(readModel.rows.map((row) => row.writerEnvelopeState)).toEqual([
      AppGameChildDeviceRuntimeWriterState.EnvelopeReady,
      AppGameChildDeviceRuntimeWriterState.EnvelopeReady,
      AppGameChildDeviceRuntimeWriterState.ManualRequired,
      AppGameChildDeviceRuntimeWriterState.Unavailable,
    ]);
    expect(readModel.rows.every((row) => row.runtimeWriterExecuted === false)).toBe(true);
    expect(readModel.rows.every((row) => row.childRuntimeTransportAttached === false)).toBe(true);
    expect(readModel.rows.every((row) => row.childRuntimeReceiptIngested === false)).toBe(true);
  });

  it('keeps manual-required and unavailable readiness rows as blocked writer states', () => {
    const readModel = buildAppGameChildDeviceRuntimeWriterReadModel(
      {
        generatedAt: Timestamp,
        runtimeWriterId: 'app-game-child-device-runtime-writer-proof',
      },
      deliveryReadinessFixture()
    );
    const manual = readModel.rows[2];
    const unavailable = readModel.rows[3];

    expect(manual.writerEnvelopeState).toBe(AppGameChildDeviceRuntimeWriterState.ManualRequired);
    expect(manual.childDeliveryTargetRefs).toEqual(['manual-proof-required', 'child-runtime-transport-not-attached']);
    expect(unavailable.writerEnvelopeState).toBe(AppGameChildDeviceRuntimeWriterState.Unavailable);
    expect(unavailable.childDeliveryTargetRefs).toEqual(['source-unavailable', 'child-runtime-transport-not-attached']);
  });

  it('rejects runtime writer child transport receipt provider platform adapter and raw-source overclaims', () => {
    const readModel = buildAppGameChildDeviceRuntimeWriterReadModel(
      {
        generatedAt: Timestamp,
        runtimeWriterId: 'app-game-child-device-runtime-writer-proof',
      },
      deliveryReadinessFixture()
    );
    const row = readModel.rows[0];

    for (const invalid of [
      { ...readModel, runtimeWriterExecuted: true },
      { ...readModel, childRuntimeTransportAttached: true },
      { ...readModel, adapterDispatchClaimed: true },
      { ...readModel, platformEnforcementClaimed: true },
      { ...readModel, rows: [{ ...row, runtimeWriterExecuted: true }, ...readModel.rows.slice(1)] },
      { ...readModel, rows: [{ ...row, providerDeliveryExecuted: true }, ...readModel.rows.slice(1)] },
      { ...readModel, rows: [{ ...row, platformDeliveryChannelClaimed: true }, ...readModel.rows.slice(1)] },
    ]) {
      expect(AppGameChildDeviceRuntimeWriterReadModelSchema.safeParse(invalid).success).toBe(false);
    }
  });
});

function deliveryReadinessFixture() {
  return AppGameChildDeviceDeliveryReadinessReadModelSchema.parse({
    schemaVersion: 'v0.6',
    readinessId: 'app-game-child-device-delivery-readiness-proof',
    generatedAt: Timestamp,
    family: { familyId: 'family-child-delivery-runtime-writer' },
    sourceProviderStatusHandoffId: 'app-game-child-delivery-readiness-provider-status-handoff',
    rows: [
      readinessRow('limit-reached', AppGameChildDeviceDeliveryReadinessStatus.TransportRequired),
      readinessRow('request-submitted', AppGameChildDeviceDeliveryReadinessStatus.TransportRequired),
      readinessRow('manual-required', AppGameChildDeviceDeliveryReadinessStatus.ManualRequired),
      readinessRow('unavailable', AppGameChildDeviceDeliveryReadinessStatus.Unavailable),
    ],
    transportRequiredCount: 2,
    manualRequiredCount: 1,
    unavailableCount: 1,
    nonClaims: [
      'no-child-runtime-transport',
      'no-child-runtime-receipt-ingestion',
      'no-provider-delivery-execution',
      'no-platform-delivery-channel',
      'no-adapter-dispatch',
      'no-platform-enforcement',
      'no-raw-private-source-rows',
    ],
    childRuntimeTransportClaimed: false,
    childRuntimeReceiptIngested: false,
    providerDeliveryExecuted: false,
    platformDeliveryChannelClaimed: false,
    adapterDispatchClaimed: false,
    platformEnforcementClaimed: false,
    rawPrivateSourceRowsIncluded: false,
  });
}

function readinessRow(suffix: string, deliveryReadinessStatus: AppGameChildDeviceDeliveryReadinessStatus) {
  return {
    deliveryReadinessRowId: `app-game-child-device-delivery-readiness-${suffix}`,
    sourceProviderStatusHandoffRowId: `app-game-child-ux-provider-status-handoff-${suffix}`,
    sourceProviderStatus:
      deliveryReadinessStatus === AppGameChildDeviceDeliveryReadinessStatus.Unavailable
        ? 'unavailable'
        : 'manual-required',
    sourceOutboxRecordRef:
      deliveryReadinessStatus === AppGameChildDeviceDeliveryReadinessStatus.TransportRequired
        ? `app-game-child-ux-local-outbox-${suffix}`
        : null,
    sourceSchedulerEntryRef:
      deliveryReadinessStatus === AppGameChildDeviceDeliveryReadinessStatus.TransportRequired
        ? `app-game-child-ux-local-outbox-scheduler-${suffix}`
        : null,
    deliveryReadinessStatus,
    requiredTransportRefs:
      deliveryReadinessStatus === AppGameChildDeviceDeliveryReadinessStatus.TransportRequired
        ? [
            'child-runtime-transport-contract-ref',
            'child-runtime-receipt-contract-ref',
            'child-device-local-agent-route-ref',
          ]
        : ['manual-proof-required'],
    openGaps: openGapsFor(deliveryReadinessStatus),
    childRuntimeTransportClaimed: false,
    childRuntimeReceiptIngested: false,
    providerDeliveryExecuted: false,
    platformDeliveryChannelClaimed: false,
  };
}

function openGapsFor(status: AppGameChildDeviceDeliveryReadinessStatus) {
  if (status === AppGameChildDeviceDeliveryReadinessStatus.Unavailable) {
    return ['source-unavailable', 'child-runtime-transport-not-attached'];
  }
  if (status === AppGameChildDeviceDeliveryReadinessStatus.ManualRequired) {
    return ['manual-proof-required', 'child-runtime-transport-not-attached'];
  }
  return [
    'child-runtime-transport-not-attached',
    'child-runtime-receipt-not-ingested',
    'provider-delivery-not-executed',
    'platform-delivery-channel-not-proved',
  ];
}
