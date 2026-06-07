import { expect, it } from 'vitest';
import { ParentContractSchemaVersion } from '../src/reference-primitives';
import {
  SocialAlertReportPreferencePreflightReadModelSchema,
  SocialAlertReportPreferencePreflightStatus,
} from '../src/social-alert-report-preference-preflight';
import {
  SocialAlertReportPreferenceStatusHandoffReadModelSchema,
  SocialAlertReportPreferenceStatusHandoffRowSchema,
  buildSocialAlertReportPreferenceStatusHandoffReadModel,
} from '../src/social-alert-report-preference-status-handoff';

const Timestamp = '2026-06-07T08:48:00Z';
const HandoffOptions = {
  generatedAt: Timestamp,
  handoffId: 'social-alert-report-preference-status-handoff-proof',
  sourceContractRefs: [
    'social-alert-report-preference-preflight',
    'v3-notification-rule-provider-retry-contract',
    'notifications-expectation-preference-boundary',
  ],
} as const;

it('maps social preference-preflight rows into V3 manual-required and unavailable status rows', () => {
  const readModel = buildPreferenceStatusHandoffReadModel();

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
});

it('preserves preflight refs while keeping social notification delivery claims false', () => {
  const readModel = buildPreferenceStatusHandoffReadModel();
  const scheduledPreferenceRow = readModel.rows[0];
  const manualRow = readModel.rows[1];

  expect(scheduledPreferenceRow.sourceSchedulerEntryRef).toBe('scheduler-entry-social-high-risk');
  expect(scheduledPreferenceRow.sourceOutboxRecordRef).toBe('outbox-record-social-high-risk');
  expect(scheduledPreferenceRow.sourceSchedulerDecisionRef).toBe('scheduler-decision-social-high-risk');
  expect(scheduledPreferenceRow.notificationPreferenceStatusEntry.parentPreferenceRef).toBe(
    'social-parent-notification-preference-required-scheduler-entry-social-high-risk'
  );
  expect(scheduledPreferenceRow.notificationPreferenceStatusEntry.quietHoursPolicyRef).toBe(
    'social-quiet-hours-policy-required-scheduler-entry-social-high-risk'
  );
  expect(manualRow.notificationPreferenceStatusEntry.evidenceRefs).toEqual([
    'provider preference setup before social alert/report can be queued',
  ]);
  expect(readModel.parentNotificationPreferenceUiClaimed).toBe(false);
  expect(readModel.parentNotificationHistoryUiClaimed).toBe(false);
  expect(readModel.parentFrequencyControlUiClaimed).toBe(false);
  expect(readModel.providerDeliveryRuntimeClaimed).toBe(false);
  expect(readModel.finalPolicyExecutionClaimed).toBe(false);
  expect(readModel.enforcementClaimed).toBe(false);
  expect(readModel.rows.every((row) => row.notificationPreferenceStatusEntry.providerReceiptRefs.length === 0)).toBe(
    true
  );
});

it('rejects preference UI and provider delivery overclaims', () => {
  const readModel = buildPreferenceStatusHandoffReadModel();

  expect(
    SocialAlertReportPreferenceStatusHandoffReadModelSchema.safeParse({
      ...readModel,
      parentNotificationPreferenceUiClaimed: true,
    }).success
  ).toBe(false);
  expect(
    SocialAlertReportPreferenceStatusHandoffReadModelSchema.safeParse({
      ...readModel,
      providerDeliveryRuntimeClaimed: true,
    }).success
  ).toBe(false);
});

it('rejects mismatched unavailable status rows', () => {
  const unavailableRow = buildPreferenceStatusHandoffReadModel().rows[2];

  expect(
    SocialAlertReportPreferenceStatusHandoffRowSchema.safeParse({
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

function buildPreferenceStatusHandoffReadModel() {
  return buildSocialAlertReportPreferenceStatusHandoffReadModel(HandoffOptions, sourcePreflightReadModel());
}

function sourcePreflightReadModel() {
  return SocialAlertReportPreferencePreflightReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    preferencePreflightId: 'social-alert-report-preference-preflight-for-status-handoff',
    generatedAt: Timestamp,
    family: { familyId: 'family-social-preference-status-handoff' },
    sourceSchedulerBridgeId: 'scheduler-bridge-social-preference-status-handoff',
    sourceContractRefs: [
      'social-alert-report-scheduler-bridge',
      'notification-parent-preference-boundary',
      'notification-quiet-hours-policy-boundary',
    ],
    rows: [parentPreferenceRequiredRow(), manualRequiredRow(), unavailableRow()],
    parentPreferenceRequiredCount: 1,
    manualRequiredCount: 1,
    unavailableCount: 1,
    preflightNonClaims: [
      'no-parent-notification-preference-ui',
      'no-parent-notification-history-ui',
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
      'no-report-delivery-execution',
      'no-final-policy-execution',
      'no-enforcement',
    ],
    parentNotificationPreferenceUiClaimed: false,
    parentNotificationHistoryUiClaimed: false,
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
    reportDeliveryExecutionClaimed: false,
    finalPolicyExecutionClaimed: false,
    enforcementClaimed: false,
  });
}

function parentPreferenceRequiredRow() {
  return {
    preferenceRowId: 'preference-preflight-social-high-risk',
    sourceSchedulerBridgeRecordId: 'scheduler-bridge-row-social-high-risk',
    status: SocialAlertReportPreferencePreflightStatus.ParentPreferenceRequired,
    sourceSchedulerEntryRef: 'scheduler-entry-social-high-risk',
    sourceOutboxRecordRef: 'outbox-record-social-high-risk',
    providerChannelRef: 'in-app',
    reasonCodeRef: 'policy-violation',
    schedulerDecisionRef: 'scheduler-decision-social-high-risk',
    parentPreferenceState: 'manual-setup-required',
    quietHoursDecision: 'manual-required',
    parentPreferenceRequirementRefs: [
      'social-parent-notification-preference-required-scheduler-entry-social-high-risk',
      'social-notification-frequency-control-required-scheduler-entry-social-high-risk',
    ],
    quietHoursRequirementRefs: ['social-quiet-hours-policy-required-scheduler-entry-social-high-risk'],
    manualProofRequirements: [
      'social-parent-notification-preference-required-scheduler-entry-social-high-risk',
      'social-notification-frequency-control-required-scheduler-entry-social-high-risk',
      'social-quiet-hours-policy-required-scheduler-entry-social-high-risk',
    ],
  };
}

function manualRequiredRow() {
  return {
    preferenceRowId: 'preference-preflight-social-manual-required',
    sourceSchedulerBridgeRecordId: 'scheduler-bridge-row-social-manual-required',
    status: SocialAlertReportPreferencePreflightStatus.ManualRequired,
    sourceSchedulerEntryRef: null,
    sourceOutboxRecordRef: null,
    providerChannelRef: null,
    reasonCodeRef: null,
    schedulerDecisionRef: null,
    parentPreferenceState: null,
    quietHoursDecision: null,
    parentPreferenceRequirementRefs: ['provider preference setup before social alert/report can be queued'],
    quietHoursRequirementRefs: ['provider preference setup before social alert/report can be queued'],
    manualProofRequirements: ['provider preference setup before social alert/report can be queued'],
  };
}

function unavailableRow() {
  return {
    preferenceRowId: 'preference-preflight-social-unavailable',
    sourceSchedulerBridgeRecordId: 'scheduler-bridge-row-social-unavailable',
    status: SocialAlertReportPreferencePreflightStatus.Unavailable,
    sourceSchedulerEntryRef: null,
    sourceOutboxRecordRef: null,
    providerChannelRef: null,
    reasonCodeRef: null,
    schedulerDecisionRef: null,
    parentPreferenceState: null,
    quietHoursDecision: null,
    parentPreferenceRequirementRefs: [
      'local evidence and policy readiness before unavailable social alert/report can be queued',
    ],
    quietHoursRequirementRefs: [
      'local evidence and policy readiness before unavailable social alert/report can be queued',
    ],
    manualProofRequirements: [
      'local evidence and policy readiness before unavailable social alert/report can be queued',
    ],
  };
}
