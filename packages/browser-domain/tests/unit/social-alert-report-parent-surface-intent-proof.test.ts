import { expect, it } from 'vitest';
import {
  buildSocialAlertReportParentSurfaceIntentReadModel,
  SocialAlertReportParentSurfaceIntentReadModelSchema,
  SocialAlertReportParentSurfaceIntentRowSchema,
} from '../../src/social-alert-report-parent-surface-intent-proof';
import { SocialAlertReportPreferenceStatusHandoffReadModelSchema } from '../../src/social-alert-report-preference-status-handoff';
import { SocialAlertReportPreferencePreflightStatus } from '../../src/social-alert-report-preference-preflight';
import { SocialAlertReportProviderPreflightStatus } from '../../src/social-alert-report-provider-preflight-proof';
import { SocialAlertReportProviderStatusHandoffReadModelSchema } from '../../src/social-alert-report-provider-status-handoff-proof';
import { ParentContractSchemaVersion } from '@ocentra-parent/family-domain/reference-primitives';

const Timestamp = '2026-06-07T07:24:00Z';
const SurfaceOptions = {
  generatedAt: Timestamp,
  intentId: 'social-alert-report-parent-surface-intent-proof',
  sourceContractRefs: [
    'social-alert-report-provider-status-handoff-proof',
    'social-alert-report-preference-status-handoff-proof',
    'social-alert-report-local-outbox-bridge-proof',
    'notifications-expectation-parent-surface-boundary',
  ],
} as const;
const PreferenceStatusEntryState = {
  [SocialAlertReportPreferencePreflightStatus.ParentPreferenceRequired]: {
    sourceRefEnabled: true,
    reasonCode: 'policy-violation',
    parentPreferenceState: 'manual-setup-required',
    quietHoursDecision: 'manual-required',
    deliveryAttemptState: 'eligible',
    deliveryResultState: 'manual-required',
    retryPolicyState: 'manual-review',
    escalationDecision: 'manual-review',
  },
  [SocialAlertReportPreferencePreflightStatus.ManualRequired]: {
    sourceRefEnabled: true,
    reasonCode: 'parent-request',
    parentPreferenceState: 'manual-setup-required',
    quietHoursDecision: 'manual-required',
    deliveryAttemptState: 'eligible',
    deliveryResultState: 'manual-required',
    retryPolicyState: 'manual-review',
    escalationDecision: 'manual-review',
  },
  [SocialAlertReportPreferencePreflightStatus.Unavailable]: {
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

it('maps provider and preference status rows into manual and unavailable parent surface rows', () => {
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
  expect(readModel.rows.map((row) => row.historyVisibility)).toEqual([
    'manual-review-only',
    'manual-review-only',
    'unavailable-row-visible',
  ]);
  expect(readModel.rows.map((row) => row.preferenceVisibility)).toEqual([
    'preference-setup-required',
    'preference-setup-required',
    'preference-disabled-visible',
  ]);
});

it('preserves drill-in preference refs and quiet-hours state', () => {
  const readModel = buildParentSurfaceIntentReadModel();
  const firstRow = readModel.rows[0];

  expect(firstRow.sourceLocalOutboxRecordRef).toBe('local-outbox-social-parent-surface-high-risk');
  expect(firstRow.sourceProviderChannelRef).toBe('social-provider-channel-in-app');
  expect(firstRow.sourceSchedulerEntryRef).toBe('scheduler-entry-social-parent-surface-high-risk');
  expect(firstRow.deliveryResultState).toBe('manual-required');
  expect(firstRow.parentPreferenceState).toBe('manual-setup-required');
  expect(firstRow.quietHoursDecision).toBe('manual-required');
  expect(firstRow.drillInRefs).toEqual([
    'social-provider-status-ref-high-risk',
    'social-preference-status-result-high-risk',
  ]);
  expect(firstRow.auditRefs).toEqual([
    'audit-social-parent-surface-high-risk',
    'social-preference-status-audit-high-risk',
  ]);
});

it('keeps UI delivery and runtime claims false', () => {
  const readModel = buildParentSurfaceIntentReadModel();

  expect(readModel.parentNotificationUiRendered).toBe(false);
  expect(readModel.parentNotificationPreferenceUiRendered).toBe(false);
  expect(readModel.parentFrequencyControlUiRendered).toBe(false);
  expect(readModel.parentNotificationHistoryUiRendered).toBe(false);
  expect(readModel.providerDeliveryRuntimeClaimed).toBe(false);
  expect(readModel.providerReceiptIngestionClaimed).toBe(false);
  expect(readModel.adapterDispatchClaimed).toBe(false);
  expect(readModel.reportDeliveryExecutionClaimed).toBe(false);
  expect(readModel.finalPolicyExecutionClaimed).toBe(false);
  expect(readModel.enforcementClaimed).toBe(false);
  expect(readModel.rows.every((row) => row.sensitiveDetailIncluded === false)).toBe(true);
});

it('rejects UI delivery and policy overclaims', () => {
  const readModel = buildParentSurfaceIntentReadModel();
  const unavailableRow = readModel.rows[2];

  expect(
    SocialAlertReportParentSurfaceIntentReadModelSchema.safeParse({
      ...readModel,
      parentNotificationUiRendered: true,
    }).success
  ).toBe(false);
  expect(
    SocialAlertReportParentSurfaceIntentReadModelSchema.safeParse({
      ...readModel,
      parentNotificationPreferenceUiRendered: true,
    }).success
  ).toBe(false);
  expect(
    SocialAlertReportParentSurfaceIntentReadModelSchema.safeParse({
      ...readModel,
      finalPolicyExecutionClaimed: true,
    }).success
  ).toBe(false);
  expect(
    SocialAlertReportParentSurfaceIntentRowSchema.safeParse({
      ...unavailableRow,
      providerDeliveryClaimed: true,
    }).success
  ).toBe(false);
  expect(() =>
    buildSocialAlertReportParentSurfaceIntentReadModel(
      SurfaceOptions,
      providerStatusReadModel(),
      twoRowPreferenceStatusReadModel()
    )
  ).toThrow('Expected social alert/report parent-surface inputs to have matching row counts');
});

function buildParentSurfaceIntentReadModel() {
  return buildSocialAlertReportParentSurfaceIntentReadModel(
    SurfaceOptions,
    providerStatusReadModel(),
    preferenceStatusReadModel()
  );
}

function providerStatusReadModel() {
  return SocialAlertReportProviderStatusHandoffReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    handoffId: 'social-provider-status-handoff-parent-surface',
    generatedAt: Timestamp,
    sourceProviderPreflightId: 'social-provider-preflight-parent-surface',
    sourceContractRefs: ['social-alert-report-provider-preflight-proof'],
    providerStatusBoundaryReadModelRef: 'v0-8-notification-provider-status-boundary',
    providerStatusBoundaryCoverageRefs: [
      'notification-provider-queued-contract',
      'notification-provider-delivered-receipt-required',
      'notification-provider-failed-contract',
      'notification-provider-unavailable-contract',
      'notification-provider-manual-required-contract',
    ],
    rows: [
      providerStatusRow('high-risk', SocialAlertReportProviderPreflightStatus.ProviderAdapterRequired),
      providerStatusRow('manual-required', SocialAlertReportProviderPreflightStatus.ManualRequired),
      providerStatusRow('unavailable', SocialAlertReportProviderPreflightStatus.Unavailable),
    ],
    providerStatusManualRequiredCount: 2,
    providerStatusUnavailableCount: 1,
    handoffNonClaims: [
      'no-provider-delivery-execution',
      'no-provider-receipt-ingestion',
      'no-provider-credentials',
      'no-cloud-routing',
      'no-parent-notification-ui-delivery',
      'no-report-delivery-execution',
      'no-final-policy-execution',
      'no-connector-native-runtime',
      'no-enforcement',
    ],
    providerDeliveryRuntimeClaimed: false,
    providerReceiptIngestionClaimed: false,
    providerCredentialsClaimed: false,
    cloudRoutingClaimed: false,
    parentNotificationUiDeliveryClaimed: false,
    reportDeliveryExecutionClaimed: false,
    finalPolicyExecutionClaimed: false,
    connectorNativeRuntimeClaimed: false,
    enforcementClaimed: false,
  });
}

function providerStatusRow(label: string, status: SocialAlertReportProviderPreflightStatus) {
  const unavailable = status === SocialAlertReportProviderPreflightStatus.Unavailable;

  return {
    handoffRowId: `social-provider-status-handoff-${label}`,
    sourcePreflightRowId: `social-provider-preflight-${label}`,
    sourceIntentRef: `social-alert-report-intent-${label}`,
    sourcePreflightStatus: status,
    sourceLocalOutboxRecordRef: unavailable ? null : `local-outbox-social-parent-surface-${label}`,
    sourceProviderChannelRef: unavailable ? null : 'social-provider-channel-in-app',
    providerStatusBoundaryEntry: {
      schemaVersion: ParentContractSchemaVersion.V0_6,
      statusEntryId: `social-provider-status-${label}`,
      providerStatus: unavailable ? 'unavailable' : 'manual-required',
      statusProofState: unavailable ? 'provider-unavailable-contract' : 'manual-action-required',
      quietHoursReadiness: unavailable ? 'unavailable' : 'manual-required',
      escalationReadiness: unavailable ? 'unavailable' : 'manual-required',
      deliveryClaimState: unavailable ? 'not-implemented' : 'not-observed',
      notificationIntentRef: `social-provider-status-intent-${label}`,
      notificationStatusRef: `social-provider-status-ref-${label}`,
      providerAttemptRef: `social-provider-attempt-${label}`,
      auditRefs: [`audit-social-parent-surface-${label}`],
      preferenceRefs: [`social-provider-preference-${label}`],
      readinessRefs: unavailable
        ? ['social-provider-readiness-unavailable']
        : [`provider-adapter-required-${label}`, `provider-credentials-required-${label}`],
      providerReceiptRefs: [],
      manualProofRequirements: [`manual-proof-social-parent-surface-${label}`],
      minimalPayloadBoundary: 'Provider status remains a manual or unavailable setup row without delivery.',
      providerDeliveryImplemented: false,
      providerDeliveryObserved: false,
      deliveredNotificationClaimed: false,
      sensitiveProviderPayloadClaimed: false,
      providerStoresChildEvidenceClaimed: false,
      lastCheckedAt: Timestamp,
    },
    manualProofRequirements: [`manual-proof-social-parent-surface-${label}`],
  };
}

function preferenceStatusReadModel() {
  return SocialAlertReportPreferenceStatusHandoffReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    handoffId: 'social-preference-status-handoff-parent-surface',
    generatedAt: Timestamp,
    family: { familyId: 'family-social-parent-surface' },
    sourcePreferencePreflightId: 'social-preference-preflight-parent-surface',
    sourceContractRefs: ['social-alert-report-preference-preflight'],
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
      preferenceStatusRow('high-risk', SocialAlertReportPreferencePreflightStatus.ParentPreferenceRequired),
      preferenceStatusRow('manual-required', SocialAlertReportPreferencePreflightStatus.ManualRequired),
      preferenceStatusRow('unavailable', SocialAlertReportPreferencePreflightStatus.Unavailable),
    ],
    parentPreferenceManualSetupRequiredCount: 2,
    quietHoursManualRequiredCount: 2,
    preferenceStatusUnavailableCount: 1,
    handoffNonClaims: [
      'no-parent-notification-preference-ui',
      'no-parent-notification-history-ui',
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
      'no-report-delivery-execution',
      'no-final-policy-execution',
      'no-enforcement',
    ],
    parentNotificationPreferenceUiClaimed: false,
    parentNotificationHistoryUiClaimed: false,
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
    reportDeliveryExecutionClaimed: false,
    finalPolicyExecutionClaimed: false,
    enforcementClaimed: false,
  });
}

function twoRowPreferenceStatusReadModel() {
  const readModel = preferenceStatusReadModel();
  return SocialAlertReportPreferenceStatusHandoffReadModelSchema.parse({
    ...readModel,
    rows: readModel.rows.slice(0, 2),
    preferenceStatusUnavailableCount: 0,
  });
}

function preferenceStatusRow(label: string, status: SocialAlertReportPreferencePreflightStatus) {
  const state = PreferenceStatusEntryState[status];
  const manualRef = `manual-proof-preference-${label}`;

  return {
    handoffRowId: `preference-status-handoff-${label}`,
    sourcePreferenceRowId: `preference-preflight-${label}`,
    sourcePreferenceStatus: status,
    sourceSchedulerEntryRef: sourceRefOrNull(state.sourceRefEnabled, `scheduler-entry-social-parent-surface-${label}`),
    sourceOutboxRecordRef: sourceRefOrNull(state.sourceRefEnabled, `local-outbox-social-parent-surface-${label}`),
    sourceProviderChannelRef: sourceRefOrNull(state.sourceRefEnabled, 'social-provider-channel-in-app'),
    sourceReasonCodeRef: sourceRefOrNull(state.sourceRefEnabled, state.reasonCode),
    sourceSchedulerDecisionRef: sourceRefOrNull(
      state.sourceRefEnabled,
      `scheduler-decision-social-parent-surface-${label}`
    ),
    sourceParentPreferenceState: sourceRefOrNull(state.sourceRefEnabled, 'manual-setup-required'),
    sourceQuietHoursDecision: sourceRefOrNull(state.sourceRefEnabled, 'manual-required'),
    sourceParentPreferenceRequirementRefs: [manualRef],
    sourceQuietHoursRequirementRefs: [manualRef],
    notificationPreferenceStatusEntry: {
      schemaVersion: ParentContractSchemaVersion.V0_6,
      contractEntryId: `social-preference-status-${label}`,
      reasonCode: state.reasonCode,
      providerChannel: 'in-app',
      deliveryAttemptState: state.deliveryAttemptState,
      deliveryResultState: state.deliveryResultState,
      retryPolicyState: state.retryPolicyState,
      quietHoursDecision: state.quietHoursDecision,
      escalationDecision: state.escalationDecision,
      parentPreferenceState: state.parentPreferenceState,
      notificationRuleRef: `social-preference-status-rule-${label}`,
      notificationIntentRef: `social-preference-status-intent-${label}`,
      deliveryAttemptRef: `social-preference-status-attempt-${label}`,
      deliveryResultRef: `social-preference-status-result-${label}`,
      retryPolicyRef: `social-preference-status-retry-${label}`,
      quietHoursPolicyRef: `social-preference-status-quiet-hours-${label}`,
      escalationPolicyRef: `social-preference-status-escalation-${label}`,
      parentPreferenceRef: `social-preference-status-parent-preference-${label}`,
      auditRefs: [`social-preference-status-audit-${label}`],
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
