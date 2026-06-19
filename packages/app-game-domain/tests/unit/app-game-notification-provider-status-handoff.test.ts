import { describe, expect, it } from 'vitest';
import {
  AppGameNotificationProviderPreflightReadModelSchema,
  AppGameNotificationProviderPreflightStatus,
} from '../../src/app-game-notification-provider-preflight';
import {
  AppGameNotificationProviderStatusHandoffReadModelSchema,
  AppGameNotificationProviderStatusHandoffRowSchema,
  buildAppGameNotificationProviderStatusHandoffReadModel,
} from '../../src/app-game-notification-provider-status-handoff';
import { ParentContractSchemaVersion } from '@ocentra-parent/schema-domain/family-reference-primitives';

const Timestamp = '2026-06-05T06:44:00Z';
const HandoffOptions = {
  generatedAt: Timestamp,
  handoffId: 'app-game-notification-provider-status-handoff-proof',
  sourceContractRefs: [
    'app-game-notification-provider-preflight',
    'v0-8-notification-provider-status-boundary',
    'notifications-expectation-provider-boundary',
  ],
} as const;

describe('app/game notification provider status handoff', () => {
  it('maps provider-preflight rows into manual-required and unavailable provider status boundary rows', () => {
    expectProviderStatusRows(buildProviderStatusHandoffReadModel());
  });

  it('preserves preflight refs while keeping delivery receipt and sensitive payload claims false', () => {
    expectPreflightRefsAndNonClaims(buildProviderStatusHandoffReadModel());
  });

  it('rejects provider delivery overclaims and mismatched unavailable status rows', () => {
    const readModel = buildProviderStatusHandoffReadModel();
    const unavailableRow = readModel.rows[2];

    expect(
      AppGameNotificationProviderStatusHandoffReadModelSchema.safeParse({
        ...readModel,
        providerDeliveryRuntimeClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameNotificationProviderStatusHandoffRowSchema.safeParse({
        ...unavailableRow,
        providerStatusBoundaryEntry: {
          ...unavailableRow.providerStatusBoundaryEntry,
          providerStatus: 'manual-required',
          statusProofState: 'manual-action-required',
          quietHoursReadiness: 'manual-required',
          escalationReadiness: 'manual-required',
        },
      }).success
    ).toBe(false);
  });
});

function buildProviderStatusHandoffReadModel() {
  return buildAppGameNotificationProviderStatusHandoffReadModel(HandoffOptions, sourcePreflightReadModel());
}

function expectProviderStatusRows(readModel: ReturnType<typeof buildProviderStatusHandoffReadModel>): void {
  expect(readModel.providerStatusManualRequiredCount).toBe(2);
  expect(readModel.providerStatusUnavailableCount).toBe(1);
  expect(readModel.rows.map((row) => row.providerStatusBoundaryEntry.providerStatus)).toEqual([
    'manual-required',
    'manual-required',
    'unavailable',
  ]);
  expect(readModel.rows.map((row) => row.providerStatusBoundaryEntry.statusProofState)).toEqual([
    'manual-action-required',
    'manual-action-required',
    'provider-unavailable-contract',
  ]);
  expect(readModel.providerStatusBoundaryCoverageRefs).toEqual([
    'notification-provider-queued-contract',
    'notification-provider-delivered-receipt-required',
    'notification-provider-failed-contract',
    'notification-provider-unavailable-contract',
    'notification-provider-manual-required-contract',
  ]);
}

function expectPreflightRefsAndNonClaims(readModel: ReturnType<typeof buildProviderStatusHandoffReadModel>): void {
  const providerSetupRow = readModel.rows[0];
  const unavailableRow = readModel.rows[2];

  expect(providerSetupRow.sourceSchedulerEntryRef).toBe('scheduler-entry-app-game-time-limit');
  expect(providerSetupRow.sourceOutboxRecordRef).toBe('outbox-record-app-game-time-limit');
  expect(providerSetupRow.sourceProviderChannelRef).toBe('in-app');
  expect(providerSetupRow.providerStatusBoundaryEntry.manualProofRequirements).toEqual([
    'provider-adapter-required-scheduler-entry-app-game-time-limit',
    'provider-credentials-required-scheduler-entry-app-game-time-limit',
    'provider-smoke-proof-required-scheduler-entry-app-game-time-limit',
  ]);
  expect(unavailableRow.providerStatusBoundaryEntry.readinessRefs).toEqual([
    'app-game-notification-provider-readiness-unavailable',
  ]);
  expect(readModel.providerDeliveryRuntimeClaimed).toBe(false);
  expect(readModel.providerReceiptIngestionClaimed).toBe(false);
  expect(readModel.providerCredentialsClaimed).toBe(false);
  expect(readModel.adapterDispatchClaimed).toBe(false);
  expect(readModel.rows.every((row) => row.providerStatusBoundaryEntry.providerReceiptRefs.length === 0)).toBe(true);
  expect(readModel.rows.every((row) => row.providerStatusBoundaryEntry.sensitiveProviderPayloadClaimed === false)).toBe(
    true
  );
}

function sourcePreflightReadModel() {
  return AppGameNotificationProviderPreflightReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    providerPreflightId: 'app-game-notification-provider-preflight-for-status-handoff',
    generatedAt: Timestamp,
    family: { familyId: 'family-app-game-provider-status-handoff' },
    sourceSchedulerBridgeId: 'scheduler-bridge-app-game-provider-status-handoff',
    sourceContractRefs: [
      'app-game-notification-scheduler-bridge',
      'notification-local-outbox-scheduler-proof',
      'notification-provider-adapter-boundary-required',
    ],
    rows: [providerAdapterRequiredRow(), manualRequiredRow(), unavailableRow()],
    providerAdapterRequiredCount: 1,
    manualRequiredCount: 1,
    unavailableCount: 1,
    preflightNonClaims: [
      'no-provider-delivery-execution',
      'no-provider-receipt-ingestion',
      'no-provider-credentials',
      'no-cloud-routing',
      'no-parent-notification-ui',
      'no-child-delivery',
      'no-retry-worker-runtime',
      'no-quiet-hours-timer-runtime',
      'no-production-durable-outbox-storage',
      'no-adapter-dispatch',
    ],
    providerDeliveryRuntimeClaimed: false,
    providerReceiptIngestionClaimed: false,
    providerCredentialsClaimed: false,
    cloudRoutingClaimed: false,
    parentNotificationUiClaimed: false,
    childDeliveryClaimed: false,
    retryExecutionRuntimeClaimed: false,
    quietHoursTimerRuntimeClaimed: false,
    productionDurableOutboxStorageClaimed: false,
    adapterDispatchClaimed: false,
  });
}

function providerAdapterRequiredRow() {
  return {
    preflightRowId: 'provider-preflight-app-game-time-limit',
    sourceSchedulerBridgeRecordId: 'scheduler-bridge-row-app-game-time-limit',
    status: AppGameNotificationProviderPreflightStatus.ProviderAdapterRequired,
    sourceSchedulerEntryRef: 'scheduler-entry-app-game-time-limit',
    sourceOutboxRecordRef: 'outbox-record-app-game-time-limit',
    schedulerDecisionRef: 'scheduler-decision-app-game-time-limit',
    providerChannelRef: 'in-app',
    reasonCodeRef: 'policy-violation',
    adapterRequirementRefs: [
      'provider-adapter-required-scheduler-entry-app-game-time-limit',
      'provider-credentials-required-scheduler-entry-app-game-time-limit',
      'provider-smoke-proof-required-scheduler-entry-app-game-time-limit',
    ],
    manualProofRequirements: [
      'provider-adapter-required-scheduler-entry-app-game-time-limit',
      'provider-credentials-required-scheduler-entry-app-game-time-limit',
      'provider-smoke-proof-required-scheduler-entry-app-game-time-limit',
    ],
  };
}

function manualRequiredRow() {
  return {
    preflightRowId: 'provider-preflight-app-game-manual-required',
    sourceSchedulerBridgeRecordId: 'scheduler-bridge-row-app-game-manual-required',
    status: AppGameNotificationProviderPreflightStatus.ManualRequired,
    sourceSchedulerEntryRef: null,
    sourceOutboxRecordRef: null,
    schedulerDecisionRef: null,
    providerChannelRef: null,
    reasonCodeRef: null,
    adapterRequirementRefs: ['provider preference setup before app game notification can be scheduled'],
    manualProofRequirements: ['provider preference setup before app game notification can be scheduled'],
  };
}

function unavailableRow() {
  return {
    preflightRowId: 'provider-preflight-app-game-unavailable',
    sourceSchedulerBridgeRecordId: 'scheduler-bridge-row-app-game-unavailable',
    status: AppGameNotificationProviderPreflightStatus.Unavailable,
    sourceSchedulerEntryRef: null,
    sourceOutboxRecordRef: null,
    schedulerDecisionRef: null,
    providerChannelRef: null,
    reasonCodeRef: null,
    adapterRequirementRefs: ['local evidence and policy readiness before unavailable notification can be scheduled'],
    manualProofRequirements: ['local evidence and policy readiness before unavailable notification can be scheduled'],
  };
}
