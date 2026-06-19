import { describe, expect, it } from 'vitest';
import { ParentContractSchemaVersion } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  TrackingNotificationPreferencePreflightReadModelSchema,
  TrackingNotificationPreferencePreflightStatus,
} from '../../src/tracking-notification-preference-preflight-proof';
import {
  TrackingNotificationPreferenceStatusHandoffReadModelSchema,
  TrackingNotificationPreferenceStatusHandoffRowSchema,
  buildTrackingNotificationPreferenceStatusHandoffReadModel,
} from '../../src/tracking-notification-preference-status-handoff';

const Timestamp = '2026-06-07T21:20:00.000Z';
const HandoffOptions = {
  generatedAt: Timestamp,
  handoffId: 'tracking-notification-preference-status-handoff-proof',
  sourceContractRefs: [
    'tracking-notification-preference-preflight-proof',
    'v3-notification-rule-provider-retry-contract',
    'notification-parent-preference-boundary',
    'notification-quiet-hours-policy-boundary',
  ],
} as const;

describe('tracking notification preference status handoff', () => {
  it('maps tracking preference preflight rows into V3 notification status rows', () => {
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
  });

  it('preserves tracking source refs while keeping UI delivery and runtime claims false', () => {
    const readModel = buildPreferenceStatusHandoffReadModel();
    const scheduledRow = readModel.rows[0];

    expect(scheduledRow.sourceProviderAttemptRef).toBe('provider-attempt-tracking-home-arrival');
    expect(scheduledRow.notificationPreferenceStatusEntry.parentPreferenceRef).toBe(
      'tracking-parent-notification-preference-required-home-arrival'
    );
    expect(scheduledRow.notificationPreferenceStatusEntry.quietHoursPolicyRef).toBe(
      'tracking-quiet-hours-policy-required-home-arrival'
    );
    expect(readModel.parentNotificationPreferenceUiClaimed).toBe(false);
    expect(readModel.parentNotificationHistoryUiClaimed).toBe(false);
    expect(readModel.parentNotificationUiClaimed).toBe(false);
    expect(readModel.quietHoursTimerRuntimeClaimed).toBe(false);
    expect(readModel.providerDeliveryRuntimeClaimed).toBe(false);
    expect(readModel.providerReceiptIngestionRuntimeClaimed).toBe(false);
    expect(readModel.adapterDispatchClaimed).toBe(false);
    expect(readModel.rows.every((row) => row.notificationPreferenceStatusEntry.providerReceiptRefs.length === 0)).toBe(
      true
    );
  });

  it('rejects notification UI overclaims and mismatched unavailable status rows', () => {
    const readModel = buildPreferenceStatusHandoffReadModel();
    const unavailableRow = readModel.rows[2];

    expect(
      TrackingNotificationPreferenceStatusHandoffReadModelSchema.safeParse({
        ...readModel,
        parentNotificationUiClaimed: true,
      }).success
    ).toBe(false);
    expect(
      TrackingNotificationPreferenceStatusHandoffRowSchema.safeParse({
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
  return buildTrackingNotificationPreferenceStatusHandoffReadModel(HandoffOptions, sourcePreflightReadModel());
}

function sourcePreflightReadModel() {
  return TrackingNotificationPreferencePreflightReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    preferencePreflightId: 'tracking-notification-preference-preflight-for-status-handoff',
    generatedAt: Timestamp,
    family: { familyId: 'family-tracking-preference-status-handoff' },
    sourceProviderNotificationProofId: 'tracking-provider-notification-proof-for-status-handoff',
    sourceContractRefs: [
      'tracking-provider-notification-proof',
      'notification-parent-preference-boundary',
      'notification-quiet-hours-policy-boundary',
    ],
    rows: [parentPreferenceRequiredRow(), sourceManualRequiredRow(), sourceUnavailableRow()],
    parentPreferenceRequiredCount: 1,
    sourceManualRequiredCount: 1,
    sourceUnavailableCount: 1,
    preflightNonClaims: [
      'no-parent-notification-preference-ui',
      'no-parent-notification-history-ui',
      'no-parent-frequency-control-ui',
      'no-quiet-hours-timer-runtime',
      'no-provider-delivery-execution',
      'no-provider-receipt-ingestion-runtime',
      'no-provider-credentials',
      'no-cloud-routing',
      'no-child-device-delivery',
      'no-mobile-physical-device-proof',
      'no-retry-worker-runtime',
      'no-production-durable-outbox-storage',
      'no-adapter-dispatch',
    ],
    parentNotificationPreferenceUiClaimed: false,
    parentNotificationHistoryUiClaimed: false,
    parentFrequencyControlUiClaimed: false,
    quietHoursTimerRuntimeClaimed: false,
    providerDeliveryRuntimeClaimed: false,
    providerReceiptIngestionRuntimeClaimed: false,
    providerCredentialsClaimed: false,
    cloudRoutingClaimed: false,
    childDeviceDeliveryClaimed: false,
    mobilePhysicalDeviceProofClaimed: false,
    retryExecutionRuntimeClaimed: false,
    productionDurableOutboxStorageClaimed: false,
    adapterDispatchClaimed: false,
  });
}

function parentPreferenceRequiredRow() {
  return {
    preferenceRowId: 'tracking-notification-preference-preflight-home-arrival',
    sourceProviderNotificationRowId: 'tracking-provider-notification-home-arrival',
    sourceAlertId: 'home-arrival',
    providerStatusKind: 'provider-adapter-required',
    status: TrackingNotificationPreferencePreflightStatus.ParentPreferenceRequired,
    sourcePolicyDecisionId: 'tracking-decision-home-arrival',
    evidenceRefs: ['tracking-evidence-home-arrival'],
    notificationStatusRefs: ['tracking-notification-intent-home-arrival'],
    reasonCodeRefs: ['parent-request'],
    providerAttemptRef: 'provider-attempt-tracking-home-arrival',
    providerPreferenceRefs: ['tracking-provider-preference-home-arrival'],
    parentPreferenceState: 'manual-setup-required',
    quietHoursDecision: 'manual-required',
    parentPreferenceRequirementRefs: [
      'tracking-parent-notification-preference-required-home-arrival',
      'tracking-notification-frequency-control-required-home-arrival',
    ],
    quietHoursRequirementRefs: ['tracking-quiet-hours-policy-required-home-arrival'],
    manualProofRequirements: [
      'tracking-provider-proof-required-home-arrival',
      'tracking-parent-notification-preference-required-home-arrival',
      'tracking-quiet-hours-policy-required-home-arrival',
    ],
  };
}

function sourceManualRequiredRow() {
  return {
    preferenceRowId: 'tracking-notification-preference-preflight-manual-required',
    sourceProviderNotificationRowId: 'tracking-provider-notification-manual-required',
    sourceAlertId: 'manual-required',
    providerStatusKind: 'manual-required',
    status: TrackingNotificationPreferencePreflightStatus.SourceManualRequired,
    sourcePolicyDecisionId: 'tracking-decision-manual-required',
    evidenceRefs: ['tracking-evidence-manual-required'],
    notificationStatusRefs: ['tracking-notification-intent-manual-required'],
    reasonCodeRefs: ['parent-request'],
    providerAttemptRef: null,
    providerPreferenceRefs: [],
    parentPreferenceState: null,
    quietHoursDecision: null,
    parentPreferenceRequirementRefs: ['tracking-source-manual-required'],
    quietHoursRequirementRefs: ['tracking-source-manual-required'],
    manualProofRequirements: ['tracking-source-manual-required'],
  };
}

function sourceUnavailableRow() {
  return {
    preferenceRowId: 'tracking-notification-preference-preflight-unavailable',
    sourceProviderNotificationRowId: 'tracking-provider-notification-unavailable',
    sourceAlertId: 'unavailable',
    providerStatusKind: 'unavailable',
    status: TrackingNotificationPreferencePreflightStatus.SourceUnavailable,
    sourcePolicyDecisionId: 'tracking-decision-unavailable',
    evidenceRefs: ['tracking-evidence-unavailable'],
    notificationStatusRefs: [],
    reasonCodeRefs: ['provider-failure'],
    providerAttemptRef: null,
    providerPreferenceRefs: [],
    parentPreferenceState: null,
    quietHoursDecision: null,
    parentPreferenceRequirementRefs: ['tracking-source-unavailable'],
    quietHoursRequirementRefs: ['tracking-source-unavailable'],
    manualProofRequirements: ['tracking-source-unavailable'],
  };
}
