import { describe, expect, it } from 'vitest';
import {
  AppGameNotificationPreferencePreflightReadModelSchema,
  AppGameNotificationPreferencePreflightStatus,
} from '../src/app-game-notification-preference-preflight';
import {
  AppGameNotificationPreferenceStatusHandoffReadModelSchema,
  AppGameNotificationPreferenceStatusHandoffRowSchema,
  buildAppGameNotificationPreferenceStatusHandoffReadModel,
} from '../src/app-game-notification-preference-status-handoff';
import { ParentContractSchemaVersion } from '../src/reference-primitives';

const Timestamp = '2026-06-05T08:39:00Z';
const HandoffOptions = {
  generatedAt: Timestamp,
  handoffId: 'app-game-notification-preference-status-handoff-proof',
  sourceContractRefs: [
    'app-game-notification-preference-preflight',
    'v3-notification-rule-provider-retry-contract',
    'notifications-expectation-preference-boundary',
  ],
} as const;

describe('app/game notification preference status handoff', () => {
  it('maps preference-preflight rows into V3 manual-required and unavailable status rows', () => {
    expectPreferenceStatusRows(buildPreferenceStatusHandoffReadModel());
  });

  it('preserves preflight refs while keeping preference UI delivery and receipt claims false', () => {
    expectPreflightRefsAndNonClaims(buildPreferenceStatusHandoffReadModel());
  });

  it('rejects parent preference UI overclaims and mismatched unavailable status rows', () => {
    const readModel = buildPreferenceStatusHandoffReadModel();
    const unavailableRow = readModel.rows[2];

    expect(
      AppGameNotificationPreferenceStatusHandoffReadModelSchema.safeParse({
        ...readModel,
        parentPreferenceUiClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameNotificationPreferenceStatusHandoffRowSchema.safeParse({
        ...unavailableRow,
        notificationPreferenceStatusEntry: {
          ...unavailableRow.notificationPreferenceStatusEntry,
          deliveryAttemptState: 'eligible',
          deliveryResultState: 'manual-required',
          retryPolicyState: 'manual-review',
          quietHoursDecision: 'manual-required',
          escalationDecision: 'manual-review',
          parentPreferenceState: 'manual-setup-required',
        },
      }).success
    ).toBe(false);
  });
});

function buildPreferenceStatusHandoffReadModel() {
  return buildAppGameNotificationPreferenceStatusHandoffReadModel(HandoffOptions, sourcePreflightReadModel());
}

function expectPreferenceStatusRows(readModel: ReturnType<typeof buildPreferenceStatusHandoffReadModel>): void {
  expect(readModel.parentPreferenceManualSetupRequiredCount).toBe(2);
  expect(readModel.quietHoursManualRequiredCount).toBe(2);
  expect(readModel.preferenceStatusUnavailableCount).toBe(1);
  expect(readModel.rows.map((row) => row.notificationPreferenceStatusEntry.deliveryResultState)).toEqual([
    'manual-required',
    'manual-required',
    'not-sent',
  ]);
  expect(readModel.rows.map((row) => row.notificationPreferenceStatusEntry.parentPreferenceState)).toEqual([
    'manual-setup-required',
    'manual-setup-required',
    'channel-disabled',
  ]);
  expect(readModel.rows.map((row) => row.notificationPreferenceStatusEntry.reasonCode)).toEqual([
    'policy-violation',
    'parent-request',
    'provider-failure',
  ]);
}

function expectPreflightRefsAndNonClaims(readModel: ReturnType<typeof buildPreferenceStatusHandoffReadModel>): void {
  const scheduledPreferenceRow = readModel.rows[0];
  const manualRow = readModel.rows[1];

  expect(scheduledPreferenceRow.sourceSchedulerEntryRef).toBe('scheduler-entry-app-game-time-limit');
  expect(scheduledPreferenceRow.sourceOutboxRecordRef).toBe('outbox-record-app-game-time-limit');
  expect(scheduledPreferenceRow.sourceProviderChannelRef).toBe('in-app');
  expect(scheduledPreferenceRow.notificationPreferenceStatusEntry.parentPreferenceRef).toBe(
    'parent-preference-required-in-app-scheduler-entry-app-game-time-limit'
  );
  expect(scheduledPreferenceRow.notificationPreferenceStatusEntry.quietHoursPolicyRef).toBe(
    'quiet-hours-policy-required-scheduler-entry-app-game-time-limit'
  );
  expect(manualRow.notificationPreferenceStatusEntry.evidenceRefs).toEqual([
    'provider preference setup before app game notification can be scheduled',
  ]);
  expect(readModel.parentPreferenceUiClaimed).toBe(false);
  expect(readModel.parentFrequencyControlUiClaimed).toBe(false);
  expect(readModel.parentNotificationUiClaimed).toBe(false);
  expect(readModel.providerDeliveryRuntimeClaimed).toBe(false);
  expect(readModel.rows.every((row) => row.notificationPreferenceStatusEntry.providerReceiptRefs.length === 0)).toBe(
    true
  );
  expect(
    readModel.rows.every((row) => row.notificationPreferenceStatusEntry.providerAdapterImplemented === false)
  ).toBe(true);
}

function sourcePreflightReadModel() {
  return AppGameNotificationPreferencePreflightReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    preferencePreflightId: 'app-game-notification-preference-preflight-for-status-handoff',
    generatedAt: Timestamp,
    family: { familyId: 'family-app-game-preference-status-handoff' },
    sourceSchedulerBridgeId: 'scheduler-bridge-app-game-preference-status-handoff',
    sourceContractRefs: [
      'app-game-notification-scheduler-bridge',
      'notification-parent-preference-boundary',
      'notification-quiet-hours-policy-boundary',
    ],
    rows: [parentPreferenceRequiredRow(), manualRequiredRow(), unavailableRow()],
    parentPreferenceRequiredCount: 1,
    manualRequiredCount: 1,
    unavailableCount: 1,
    preflightNonClaims: [
      'no-parent-preference-ui',
      'no-parent-frequency-control-ui',
      'no-quiet-hours-timer-runtime',
      'no-provider-delivery-execution',
      'no-provider-receipt-ingestion',
      'no-provider-credentials',
      'no-cloud-routing',
      'no-child-delivery',
      'no-retry-worker-runtime',
      'no-production-durable-outbox-storage',
      'no-adapter-dispatch',
    ],
    parentPreferenceUiClaimed: false,
    parentFrequencyControlUiClaimed: false,
    quietHoursTimerRuntimeClaimed: false,
    providerDeliveryRuntimeClaimed: false,
    providerReceiptIngestionClaimed: false,
    providerCredentialsClaimed: false,
    cloudRoutingClaimed: false,
    childDeliveryClaimed: false,
    retryExecutionRuntimeClaimed: false,
    productionDurableOutboxStorageClaimed: false,
    adapterDispatchClaimed: false,
  });
}

function parentPreferenceRequiredRow() {
  return {
    preferenceRowId: 'preference-preflight-app-game-time-limit',
    sourceSchedulerBridgeRecordId: 'scheduler-bridge-row-app-game-time-limit',
    status: AppGameNotificationPreferencePreflightStatus.ParentPreferenceRequired,
    sourceSchedulerEntryRef: 'scheduler-entry-app-game-time-limit',
    sourceOutboxRecordRef: 'outbox-record-app-game-time-limit',
    providerChannelRef: 'in-app',
    reasonCodeRef: 'policy-violation',
    parentPreferenceState: 'manual-setup-required',
    quietHoursDecision: 'manual-required',
    parentPreferenceRequirementRefs: [
      'parent-preference-required-in-app-scheduler-entry-app-game-time-limit',
      'notification-frequency-control-required-scheduler-entry-app-game-time-limit',
    ],
    quietHoursRequirementRefs: ['quiet-hours-policy-required-scheduler-entry-app-game-time-limit'],
    manualProofRequirements: [
      'parent-preference-required-in-app-scheduler-entry-app-game-time-limit',
      'notification-frequency-control-required-scheduler-entry-app-game-time-limit',
      'quiet-hours-policy-required-scheduler-entry-app-game-time-limit',
    ],
  };
}

function manualRequiredRow() {
  return {
    preferenceRowId: 'preference-preflight-app-game-manual-required',
    sourceSchedulerBridgeRecordId: 'scheduler-bridge-row-app-game-manual-required',
    status: AppGameNotificationPreferencePreflightStatus.ManualRequired,
    sourceSchedulerEntryRef: null,
    sourceOutboxRecordRef: null,
    providerChannelRef: null,
    reasonCodeRef: null,
    parentPreferenceState: null,
    quietHoursDecision: null,
    parentPreferenceRequirementRefs: ['provider preference setup before app game notification can be scheduled'],
    quietHoursRequirementRefs: ['provider preference setup before app game notification can be scheduled'],
    manualProofRequirements: ['provider preference setup before app game notification can be scheduled'],
  };
}

function unavailableRow() {
  return {
    preferenceRowId: 'preference-preflight-app-game-unavailable',
    sourceSchedulerBridgeRecordId: 'scheduler-bridge-row-app-game-unavailable',
    status: AppGameNotificationPreferencePreflightStatus.Unavailable,
    sourceSchedulerEntryRef: null,
    sourceOutboxRecordRef: null,
    providerChannelRef: null,
    reasonCodeRef: null,
    parentPreferenceState: null,
    quietHoursDecision: null,
    parentPreferenceRequirementRefs: [
      'local evidence and policy readiness before unavailable notification can be scheduled',
    ],
    quietHoursRequirementRefs: ['local evidence and policy readiness before unavailable notification can be scheduled'],
    manualProofRequirements: ['local evidence and policy readiness before unavailable notification can be scheduled'],
  };
}
