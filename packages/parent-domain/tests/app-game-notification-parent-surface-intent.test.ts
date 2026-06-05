import { describe, expect, it } from 'vitest';
import { AppGameNotificationPreferencePreflightStatus } from '../src/app-game-notification-preference-preflight';
import {
  AppGameNotificationParentSurfaceIntentReadModelSchema,
  AppGameNotificationParentSurfaceIntentRowSchema,
  buildAppGameNotificationParentSurfaceIntentReadModel,
} from '../src/app-game-notification-parent-surface-intent';
import { AppGameNotificationPreferenceStatusHandoffReadModelSchema } from '../src/app-game-notification-preference-status-handoff';
import { AppGameNotificationProviderPreflightStatus } from '../src/app-game-notification-provider-preflight';
import { AppGameNotificationProviderStatusHandoffReadModelSchema } from '../src/app-game-notification-provider-status-handoff';
import { ParentContractSchemaVersion } from '../src/reference-primitives';

const Timestamp = '2026-06-05T09:12:00Z';
const SurfaceOptions = {
  generatedAt: Timestamp,
  intentId: 'app-game-notification-parent-surface-intent-proof',
  sourceContractRefs: [
    'app-game-notification-provider-status-handoff',
    'app-game-notification-preference-status-handoff',
    'notifications-expectation-parent-surface-boundary',
  ],
} as const;
const PreferenceStatusEntryState = {
  [AppGameNotificationPreferencePreflightStatus.ParentPreferenceRequired]: {
    sourceRefEnabled: true,
    reasonCode: 'policy-violation',
    parentPreferenceState: 'manual-setup-required',
    quietHoursDecision: 'manual-required',
    deliveryAttemptState: 'eligible',
    deliveryResultState: 'manual-required',
    retryPolicyState: 'manual-review',
    escalationDecision: 'manual-review',
  },
  [AppGameNotificationPreferencePreflightStatus.ManualRequired]: {
    sourceRefEnabled: true,
    reasonCode: 'policy-violation',
    parentPreferenceState: 'manual-setup-required',
    quietHoursDecision: 'manual-required',
    deliveryAttemptState: 'eligible',
    deliveryResultState: 'manual-required',
    retryPolicyState: 'manual-review',
    escalationDecision: 'manual-review',
  },
  [AppGameNotificationPreferencePreflightStatus.Unavailable]: {
    sourceRefEnabled: false,
    reasonCode: 'provider-failure',
    parentPreferenceState: 'channel-disabled',
    quietHoursDecision: 'allow',
    deliveryAttemptState: 'provider-disabled',
    deliveryResultState: 'not-sent',
    retryPolicyState: 'provider-disabled',
    escalationDecision: 'none',
  },
} as const;

describe('app/game notification parent surface intent', () => {
  it('combines provider and preference status rows into redacted parent surface rows', () => {
    const readModel = buildParentSurfaceIntentReadModel();

    expect(readModel.manualActionRequiredCount).toBe(2);
    expect(readModel.unavailableVisibleCount).toBe(1);
    expect(readModel.historyVisibleCount).toBe(3);
    expect(readModel.preferenceSetupRequiredCount).toBe(2);
    expect(readModel.rows.map((row) => row.parentSurfaceStatus)).toEqual([
      'manual-action-required',
      'manual-action-required',
      'unavailable-visible',
    ]);
    expect(readModel.rows.map((row) => row.preferenceVisibility)).toEqual([
      'preference-setup-required',
      'preference-setup-required',
      'preference-disabled-visible',
    ]);
  });

  it('preserves drill-in refs and keeps UI delivery runtime claims false', () => {
    const readModel = buildParentSurfaceIntentReadModel();
    const firstRow = readModel.rows[0];

    expect(firstRow.sourceSchedulerEntryRef).toBe('scheduler-entry-app-game-time-limit');
    expect(firstRow.sourceOutboxRecordRef).toBe('outbox-record-app-game-time-limit');
    expect(firstRow.drillInRefs).toEqual([
      'app-game-provider-status-ref-time-limit',
      'app-game-preference-status-result-time-limit',
    ]);
    expect(firstRow.auditRefs).toEqual([
      'app-game-provider-status-audit-time-limit',
      'app-game-preference-status-audit-time-limit',
    ]);
    expect(readModel.parentNotificationUiRendered).toBe(false);
    expect(readModel.parentPreferenceUiRendered).toBe(false);
    expect(readModel.providerDeliveryRuntimeClaimed).toBe(false);
    expect(readModel.providerReceiptIngestionClaimed).toBe(false);
    expect(readModel.adapterDispatchClaimed).toBe(false);
    expect(readModel.rows.every((row) => row.sensitiveDetailIncluded === false)).toBe(true);
  });

  it('rejects UI overclaims and mismatched upstream row counts', () => {
    const readModel = buildParentSurfaceIntentReadModel();
    const unavailableRow = readModel.rows[2];

    expect(
      AppGameNotificationParentSurfaceIntentReadModelSchema.safeParse({
        ...readModel,
        parentNotificationUiRendered: true,
      }).success
    ).toBe(false);
    expect(
      AppGameNotificationParentSurfaceIntentRowSchema.safeParse({
        ...unavailableRow,
        providerDeliveryClaimed: true,
      }).success
    ).toBe(false);
    expect(() =>
      buildAppGameNotificationParentSurfaceIntentReadModel(
        SurfaceOptions,
        providerStatusReadModel(),
        twoRowPreferenceStatusReadModel()
      )
    ).toThrow('Expected app/game notification parent-surface inputs to have matching row counts');
  });
});

function buildParentSurfaceIntentReadModel() {
  return buildAppGameNotificationParentSurfaceIntentReadModel(
    SurfaceOptions,
    providerStatusReadModel(),
    preferenceStatusReadModel()
  );
}

function providerStatusReadModel() {
  return AppGameNotificationProviderStatusHandoffReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    handoffId: 'app-game-provider-status-handoff-parent-surface',
    generatedAt: Timestamp,
    family: { familyId: 'family-app-game-parent-surface' },
    sourceProviderPreflightId: 'app-game-provider-preflight-parent-surface',
    sourceContractRefs: ['app-game-notification-provider-preflight'],
    providerStatusBoundaryReadModelRef: 'v0-8-notification-provider-status-boundary',
    providerStatusBoundaryCoverageRefs: [
      'notification-provider-queued-contract',
      'notification-provider-delivered-receipt-required',
      'notification-provider-failed-contract',
      'notification-provider-unavailable-contract',
      'notification-provider-manual-required-contract',
    ],
    rows: [
      providerStatusRow('time-limit', AppGameNotificationProviderPreflightStatus.ProviderAdapterRequired),
      providerStatusRow('manual-required', AppGameNotificationProviderPreflightStatus.ManualRequired),
      providerStatusRow('unavailable', AppGameNotificationProviderPreflightStatus.Unavailable),
    ],
    providerStatusManualRequiredCount: 2,
    providerStatusUnavailableCount: 1,
    handoffNonClaims: [
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

function providerStatusRow(label: string, status: AppGameNotificationProviderPreflightStatus) {
  const unavailable = status === AppGameNotificationProviderPreflightStatus.Unavailable;
  const manualRef = `manual-proof-provider-${label}`;

  return {
    handoffRowId: `provider-status-handoff-${label}`,
    sourcePreflightRowId: `provider-preflight-${label}`,
    sourcePreflightStatus: status,
    sourceSchedulerEntryRef: unavailable ? null : `scheduler-entry-app-game-${label}`,
    sourceOutboxRecordRef: unavailable ? null : `outbox-record-app-game-${label}`,
    sourceProviderChannelRef: unavailable ? null : 'in-app',
    providerStatusBoundaryEntry: {
      schemaVersion: ParentContractSchemaVersion.V0_6,
      statusEntryId: `app-game-provider-status-${label}`,
      providerStatus: unavailable ? 'unavailable' : 'manual-required',
      statusProofState: unavailable ? 'provider-unavailable-contract' : 'manual-action-required',
      quietHoursReadiness: unavailable ? 'unavailable' : 'manual-required',
      escalationReadiness: unavailable ? 'unavailable' : 'manual-required',
      deliveryClaimState: unavailable ? 'not-implemented' : 'not-observed',
      notificationIntentRef: `app-game-provider-status-intent-${label}`,
      notificationStatusRef: `app-game-provider-status-ref-${label}`,
      providerAttemptRef: `app-game-provider-status-attempt-${label}`,
      auditRefs: [`app-game-provider-status-audit-${label}`],
      preferenceRefs: [`app-game-provider-status-preference-${label}`],
      readinessRefs: [`app-game-provider-status-readiness-${label}`],
      providerReceiptRefs: [],
      manualProofRequirements: [manualRef],
      minimalPayloadBoundary: 'Provider status remains a manual or unavailable setup row without delivery.',
      providerDeliveryImplemented: false,
      providerDeliveryObserved: false,
      deliveredNotificationClaimed: false,
      sensitiveProviderPayloadClaimed: false,
      providerStoresChildEvidenceClaimed: false,
      lastCheckedAt: Timestamp,
    },
    manualProofRequirements: [manualRef],
  };
}

function preferenceStatusReadModel() {
  return AppGameNotificationPreferenceStatusHandoffReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    handoffId: 'app-game-preference-status-handoff-parent-surface',
    generatedAt: Timestamp,
    family: { familyId: 'family-app-game-parent-surface' },
    sourcePreferencePreflightId: 'app-game-preference-preflight-parent-surface',
    sourceContractRefs: ['app-game-notification-preference-preflight'],
    notificationRuleProviderRetryReadModelRef: 'v3-notification-rule-provider-retry-contract',
    notificationRuleProviderRetryCoverageRefs: [
      'notification-rule-provider-retry-policy-violation',
      'notification-rule-provider-retry-parent-request',
      'notification-rule-provider-retry-suspicious-unknown',
      'notification-rule-provider-retry-device-offline',
      'notification-rule-provider-retry-sync-failure',
      'notification-rule-provider-retry-provider-failure',
    ],
    rows: [
      preferenceStatusRow('time-limit', AppGameNotificationPreferencePreflightStatus.ParentPreferenceRequired),
      preferenceStatusRow('manual-required', AppGameNotificationPreferencePreflightStatus.ManualRequired),
      preferenceStatusRow('unavailable', AppGameNotificationPreferencePreflightStatus.Unavailable),
    ],
    parentPreferenceManualSetupRequiredCount: 2,
    quietHoursManualRequiredCount: 2,
    preferenceStatusUnavailableCount: 1,
    handoffNonClaims: [
      'no-parent-preference-ui',
      'no-parent-frequency-control-ui',
      'no-parent-notification-ui',
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
    parentNotificationUiClaimed: false,
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

function twoRowPreferenceStatusReadModel() {
  const readModel = preferenceStatusReadModel();
  return AppGameNotificationPreferenceStatusHandoffReadModelSchema.parse({
    ...readModel,
    rows: readModel.rows.slice(0, 2),
    preferenceStatusUnavailableCount: 0,
  });
}

function preferenceStatusRow(label: string, status: AppGameNotificationPreferencePreflightStatus) {
  const state = PreferenceStatusEntryState[status];
  const manualRef = `manual-proof-preference-${label}`;

  return {
    handoffRowId: `preference-status-handoff-${label}`,
    sourcePreferenceRowId: `preference-preflight-${label}`,
    sourcePreferenceStatus: status,
    sourceSchedulerEntryRef: sourceRefOrNull(state.sourceRefEnabled, `scheduler-entry-app-game-${label}`),
    sourceOutboxRecordRef: sourceRefOrNull(state.sourceRefEnabled, `outbox-record-app-game-${label}`),
    sourceProviderChannelRef: sourceRefOrNull(state.sourceRefEnabled, 'in-app'),
    sourceReasonCodeRef: sourceRefOrNull(state.sourceRefEnabled, state.reasonCode),
    sourceParentPreferenceState: sourceRefOrNull(state.sourceRefEnabled, 'manual-setup-required'),
    sourceQuietHoursDecision: sourceRefOrNull(state.sourceRefEnabled, 'manual-required'),
    sourceParentPreferenceRequirementRefs: [manualRef],
    sourceQuietHoursRequirementRefs: [manualRef],
    notificationPreferenceStatusEntry: {
      schemaVersion: ParentContractSchemaVersion.V0_6,
      contractEntryId: `app-game-preference-status-${label}`,
      reasonCode: state.reasonCode,
      providerChannel: 'in-app',
      deliveryAttemptState: state.deliveryAttemptState,
      deliveryResultState: state.deliveryResultState,
      retryPolicyState: state.retryPolicyState,
      quietHoursDecision: state.quietHoursDecision,
      escalationDecision: state.escalationDecision,
      parentPreferenceState: state.parentPreferenceState,
      notificationRuleRef: `app-game-preference-status-rule-${label}`,
      notificationIntentRef: `app-game-preference-status-intent-${label}`,
      deliveryAttemptRef: `app-game-preference-status-attempt-${label}`,
      deliveryResultRef: `app-game-preference-status-result-${label}`,
      retryPolicyRef: `app-game-preference-status-retry-${label}`,
      quietHoursPolicyRef: `app-game-preference-status-quiet-hours-${label}`,
      escalationPolicyRef: `app-game-preference-status-escalation-${label}`,
      parentPreferenceRef: `app-game-preference-status-parent-preference-${label}`,
      auditRefs: [`app-game-preference-status-audit-${label}`],
      evidenceRefs: [manualRef],
      providerReceiptRefs: [],
      manualProofRequirements: [manualRef],
      minimalProviderPayloadBoundary: 'Preference status remains setup-only without provider delivery.',
      providerAdapterImplemented: false,
      deliveryAttemptExecuted: false,
      providerReceiptObserved: false,
      rawEvidenceInProviderPayload: false,
      providerStoresChildEvidenceClaimed: false,
      lastCheckedAt: Timestamp,
    },
    manualProofRequirements: [manualRef],
  };
}

function sourceRefOrNull(enabled: boolean, ref: string): string | null {
  return enabled ? ref : null;
}
