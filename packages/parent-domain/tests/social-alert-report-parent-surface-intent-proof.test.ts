import { describe, expect, it } from 'vitest';
import {
  buildSocialAlertReportParentSurfaceIntentReadModel,
  SocialAlertReportParentSurfaceIntentReadModelSchema,
  SocialAlertReportParentSurfaceIntentRowSchema,
} from '../src/social-alert-report-parent-surface-intent-proof';
import { SocialAlertReportProviderPreflightStatus } from '../src/social-alert-report-provider-preflight-proof';
import { SocialAlertReportProviderStatusHandoffReadModelSchema } from '../src/social-alert-report-provider-status-handoff-proof';
import { ParentContractSchemaVersion } from '../src/reference-primitives';

const Timestamp = '2026-06-07T07:24:00Z';
const SurfaceOptions = {
  generatedAt: Timestamp,
  intentId: 'social-alert-report-parent-surface-intent-proof',
  sourceContractRefs: [
    'social-alert-report-provider-status-handoff-proof',
    'social-alert-report-local-outbox-bridge-proof',
    'notifications-expectation-parent-surface-boundary',
  ],
} as const;

describe('social alert/report parent surface intent proof', () => {
  it('maps provider status rows into manual and unavailable parent surface rows', () => {
    const readModel = buildParentSurfaceIntentReadModel();

    expect(readModel.manualActionRequiredCount).toBe(2);
    expect(readModel.unavailableVisibleCount).toBe(1);
    expect(readModel.historyVisibleCount).toBe(3);
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
  });

  it('preserves drill-in refs and keeps delivery UI claims false', () => {
    const readModel = buildParentSurfaceIntentReadModel();
    const firstRow = readModel.rows[0];

    expect(firstRow.sourceLocalOutboxRecordRef).toBe('local-outbox-social-parent-surface-high-risk');
    expect(firstRow.sourceProviderChannelRef).toBe('social-provider-channel-in-app');
    expect(firstRow.drillInRefs).toEqual([
      'social-provider-status-ref-high-risk',
      'provider-adapter-required-high-risk',
      'provider-credentials-required-high-risk',
    ]);
    expect(firstRow.auditRefs).toEqual(['audit-social-parent-surface-high-risk']);
    expect(readModel.parentNotificationUiRendered).toBe(false);
    expect(readModel.providerDeliveryRuntimeClaimed).toBe(false);
    expect(readModel.providerReceiptIngestionClaimed).toBe(false);
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
        finalPolicyExecutionClaimed: true,
      }).success
    ).toBe(false);
    expect(
      SocialAlertReportParentSurfaceIntentRowSchema.safeParse({
        ...unavailableRow,
        providerDeliveryClaimed: true,
      }).success
    ).toBe(false);
  });
});

function buildParentSurfaceIntentReadModel() {
  return buildSocialAlertReportParentSurfaceIntentReadModel(SurfaceOptions, providerStatusReadModel());
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
