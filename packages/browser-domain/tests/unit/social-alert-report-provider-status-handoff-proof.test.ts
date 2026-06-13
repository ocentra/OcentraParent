import { describe, expect, it } from 'vitest';
import { buildSocialAlertReportProviderPreflightReadModel } from '../../src/social-alert-report-provider-preflight-proof';
import {
  buildSocialAlertReportProviderStatusHandoffReadModel,
  SocialAlertReportProviderStatusHandoffReadModelSchema,
  SocialAlertReportProviderStatusHandoffRowSchema,
} from '../../src/social-alert-report-provider-status-handoff-proof';
import {
  SocialAlertReportAdapterDispatchState,
  SocialAlertReportDeliveryClaimState,
  SocialAlertReportIntentKind,
  SocialAlertReportIntentStatus,
  SocialAlertReportParentCopyToken,
  SocialAlertReportPayloadField,
  SocialAlertReportReasonCode,
} from '../../src/social-alert-report-intent';
import { ParentContractSchemaVersion, ParentEvidenceReferenceKind, ParentPlatform } from '@ocentra-parent/family-domain/reference-primitives';

const Timestamp = '2026-06-07T05:46:00Z';
const MinimalPayloadFields = Object.values(SocialAlertReportPayloadField);

const BaseIntent = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  alertReportIntentId: 'social-provider-status-high-risk',
  intentKind: SocialAlertReportIntentKind.HighRiskSignal,
  intentStatus: SocialAlertReportIntentStatus.LocalOutboxEligible,
  priority: 'urgent',
  severity: 'critical',
  device: {
    deviceId: 'device-social-provider-status',
    childProfileId: 'child-social-provider-status',
    label: 'Study Phone',
    platform: ParentPlatform.Android,
  },
  notificationReasonCode: SocialAlertReportReasonCode.HighRiskSignal,
  providerChannelPreference: 'in-app',
  parentTitleToken: SocialAlertReportParentCopyToken.HighRiskTitle,
  parentBodyToken: SocialAlertReportParentCopyToken.HighRiskBody,
  parentActionToken: SocialAlertReportParentCopyToken.OpenParentReviewAction,
  dashboardPanelRefs: ['panel-feed-video-gates'],
  explanationSnapshotRef: 'social-explanation-snapshot-provider-status',
  explanationEventRefs: ['social-explanation-event-provider-status'],
  evidenceReferences: [
    {
      evidenceReferenceId: 'evidence-social-provider-status',
      kind: ParentEvidenceReferenceKind.PolicyDecision,
      observedAt: Timestamp,
    },
  ],
  policyRefs: ['policy-ref-social-provider-status'],
  auditRefs: ['audit-ref-social-provider-status'],
  parentReportRef: null,
  parentActionRef: null,
  localOutboxRecordRef: 'local-outbox-social-provider-status',
  providerAttemptRefs: [],
  providerReceiptRefs: [],
  manualProofRequirements: [],
  minimalPayloadFields: MinimalPayloadFields,
  deliveryClaimState: SocialAlertReportDeliveryClaimState.LocalOutboxOnly,
  rawAccountDataIncluded: false,
  rawVideoContentIncluded: false,
  rawMessageContentIncluded: false,
  screenshotIncluded: false,
  providerDeliveryAttempted: false,
  providerDeliveryObserved: false,
  providerReceiptIngested: false,
  cloudRoutingClaimed: false,
  parentNotificationUiClaimed: false,
  reportDeliveryClaimed: false,
  finalPolicyDecisionClaimed: false,
  enforcementClaimed: false,
  adapterDispatchState: SocialAlertReportAdapterDispatchState.NotDispatched,
  adapterActionClaimed: false,
  createdAt: Timestamp,
} as const;

describe('social alert/report provider status handoff proof', () => {
  it('maps provider preflight rows into manual-required and unavailable provider status boundary rows', () => {
    const readModel = buildProviderStatusHandoffReadModel();

    expect(readModel.providerStatusManualRequiredCount).toBe(2);
    expect(readModel.providerStatusUnavailableCount).toBe(1);
    expect(readModel.rows.map((row) => row.providerStatusBoundaryEntry.providerStatus)).toEqual([
      'manual-required',
      'manual-required',
      'unavailable',
    ]);
    expect(readModel.providerStatusBoundaryCoverageRefs).toEqual([
      'notification-provider-queued-contract',
      'notification-provider-delivered-receipt-required',
      'notification-provider-failed-contract',
      'notification-provider-unavailable-contract',
      'notification-provider-manual-required-contract',
    ]);
  });

  it('preserves social preflight refs while keeping delivery, receipt, and sensitive payload claims false', () => {
    const readModel = buildProviderStatusHandoffReadModel();
    const adapterRequiredRow = readModel.rows[0];
    const unavailableRow = readModel.rows[2];

    expect(adapterRequiredRow.sourceLocalOutboxRecordRef).toBe('local-outbox-social-provider-status');
    expect(adapterRequiredRow.sourceProviderChannelRef).toBe('social-provider-channel-in-app');
    expect(adapterRequiredRow.providerStatusBoundaryEntry.readinessRefs).toEqual([
      'provider-adapter-required-social-provider-status-high-risk',
      'provider-credentials-required-social-provider-status-high-risk',
      'provider-smoke-proof-required-social-provider-status-high-risk',
    ]);
    expect(unavailableRow.providerStatusBoundaryEntry.readinessRefs).toEqual(['social-provider-readiness-unavailable']);
    expect(readModel.providerDeliveryRuntimeClaimed).toBe(false);
    expect(readModel.providerReceiptIngestionClaimed).toBe(false);
    expect(readModel.parentNotificationUiDeliveryClaimed).toBe(false);
    expect(readModel.reportDeliveryExecutionClaimed).toBe(false);
    expect(readModel.finalPolicyExecutionClaimed).toBe(false);
    expect(readModel.enforcementClaimed).toBe(false);
    expect(readModel.rows.every((row) => row.providerStatusBoundaryEntry.providerReceiptRefs.length === 0)).toBe(true);
    expect(
      readModel.rows.every((row) => row.providerStatusBoundaryEntry.sensitiveProviderPayloadClaimed === false)
    ).toBe(true);
  });

  it('rejects provider delivery overclaims and mismatched unavailable status rows', () => {
    const readModel = buildProviderStatusHandoffReadModel();
    const unavailableRow = readModel.rows[2];

    expect(
      SocialAlertReportProviderStatusHandoffReadModelSchema.safeParse({
        ...readModel,
        providerDeliveryRuntimeClaimed: true,
      }).success
    ).toBe(false);
    expect(
      SocialAlertReportProviderStatusHandoffReadModelSchema.safeParse({
        ...readModel,
        providerStatusBoundaryCoverageRefs: [],
      }).success
    ).toBe(false);
    expect(
      SocialAlertReportProviderStatusHandoffRowSchema.safeParse({
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
  return buildSocialAlertReportProviderStatusHandoffReadModel(
    {
      generatedAt: Timestamp,
      handoffId: 'social-alert-report-provider-status-handoff-proof',
      sourceContractRefs: [
        'social-alert-report-provider-preflight-proof',
        'v0-8-notification-provider-status-boundary',
        'notifications-expectation-provider-boundary',
      ],
    },
    buildSocialAlertReportProviderPreflightReadModel(
      {
        generatedAt: Timestamp,
        providerPreflightId: 'social-alert-report-provider-preflight-for-status-handoff',
        sourceContractRefs: ['social-alert-report-intent', 'social-alert-report-intent-ui-proof'],
      },
      [BaseIntent, manualRequiredIntent(), unavailableIntent()]
    )
  );
}

function manualRequiredIntent() {
  return {
    ...BaseIntent,
    alertReportIntentId: 'social-provider-status-manual-required',
    intentKind: SocialAlertReportIntentKind.ManualRequired,
    intentStatus: SocialAlertReportIntentStatus.ManualRequired,
    priority: 'attention',
    severity: 'warning',
    notificationReasonCode: SocialAlertReportReasonCode.ManualRequired,
    parentTitleToken: SocialAlertReportParentCopyToken.ManualRequiredTitle,
    parentBodyToken: SocialAlertReportParentCopyToken.ManualRequiredBody,
    parentActionToken: SocialAlertReportParentCopyToken.ReviewManuallyAction,
    localOutboxRecordRef: null,
    deliveryClaimState: SocialAlertReportDeliveryClaimState.ManualRequired,
    manualProofRequirements: ['manual-proof-social-provider-status-required'],
  } as const;
}

function unavailableIntent() {
  return {
    ...manualRequiredIntent(),
    alertReportIntentId: 'social-provider-status-unavailable',
    intentKind: SocialAlertReportIntentKind.CapabilityUnavailable,
    intentStatus: SocialAlertReportIntentStatus.Unavailable,
    notificationReasonCode: SocialAlertReportReasonCode.CapabilityUnavailable,
    parentTitleToken: SocialAlertReportParentCopyToken.UnavailableTitle,
    parentBodyToken: SocialAlertReportParentCopyToken.UnavailableBody,
    manualProofRequirements: ['manual-proof-social-provider-status-unavailable'],
  } as const;
}
