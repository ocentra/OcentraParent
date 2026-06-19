import { describe, expect, it } from 'vitest';
import {
  buildSocialAlertReportProviderPreflightReadModel,
  SocialAlertReportProviderPreflightStatus,
} from '../../src/social-alert-report-provider-preflight-proof';
import { buildSocialAlertReportProviderStatusHandoffReadModel } from '../../src/social-alert-report-provider-status-handoff-proof';
import {
  buildSocialAlertReportProviderReceiptBoundaryReadModel,
  SocialAlertReportProviderReceiptBoundaryReadModelSchema,
  SocialAlertReportProviderReceiptBoundaryRowSchema,
} from '../../src/social-alert-report-provider-receipt-boundary-proof';
import {
  SocialAlertReportAdapterDispatchState,
  SocialAlertReportDeliveryClaimState,
  SocialAlertReportIntentKind,
  SocialAlertReportIntentStatus,
  SocialAlertReportParentCopyToken,
  SocialAlertReportPayloadField,
  SocialAlertReportReasonCode,
} from '../../src/social-alert-report-intent';
import {
  ParentContractSchemaVersion,
  ParentEvidenceReferenceKind,
  ParentPlatform,
} from '@ocentra-parent/schema-domain/family-reference-primitives';

const Timestamp = '2026-06-08T04:13:00Z';
const MinimalPayloadFields = Object.values(SocialAlertReportPayloadField);

const BaseIntent = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  alertReportIntentId: 'social-provider-receipt-high-risk',
  intentKind: SocialAlertReportIntentKind.HighRiskSignal,
  intentStatus: SocialAlertReportIntentStatus.LocalOutboxEligible,
  priority: 'urgent',
  severity: 'critical',
  device: {
    deviceId: 'device-social-provider-receipt',
    childProfileId: 'child-social-provider-receipt',
    label: 'Study Phone',
    platform: ParentPlatform.Android,
  },
  notificationReasonCode: SocialAlertReportReasonCode.HighRiskSignal,
  providerChannelPreference: 'in-app',
  parentTitleToken: SocialAlertReportParentCopyToken.HighRiskTitle,
  parentBodyToken: SocialAlertReportParentCopyToken.HighRiskBody,
  parentActionToken: SocialAlertReportParentCopyToken.OpenParentReviewAction,
  dashboardPanelRefs: ['panel-feed-video-gates'],
  explanationSnapshotRef: 'social-explanation-snapshot-provider-receipt',
  explanationEventRefs: ['social-explanation-event-provider-receipt'],
  evidenceReferences: [
    {
      evidenceReferenceId: 'evidence-social-provider-receipt',
      kind: ParentEvidenceReferenceKind.PolicyDecision,
      observedAt: Timestamp,
    },
  ],
  policyRefs: ['policy-ref-social-provider-receipt'],
  auditRefs: ['audit-ref-social-provider-receipt'],
  parentReportRef: null,
  parentActionRef: null,
  localOutboxRecordRef: 'local-outbox-social-provider-receipt',
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

describe('social alert/report provider receipt boundary proof', () => {
  it('projects provider-status handoff rows into receipt boundary states without delivery claims', () => {
    const readModel = buildReceiptBoundaryReadModel();

    expect(readModel.providerDispatchRequiredCount).toBe(1);
    expect(readModel.manualReceiptRequiredCount).toBe(1);
    expect(readModel.providerUnavailableCount).toBe(1);
    expect(readModel.rows.map((row) => row.receiptBoundaryState)).toEqual([
      'provider-dispatch-required',
      'manual-receipt-required',
      'provider-unavailable',
    ]);
    expect(readModel.rows.map((row) => row.providerReceiptRefs)).toEqual([[], [], []]);
    expect(readModel.providerDeliveryRuntimeClaimed).toBe(false);
    expect(readModel.providerReceiptIngestionRuntimeClaimed).toBe(false);
    expect(readModel.finalPolicyExecutionClaimed).toBe(false);
    expect(readModel.enforcementClaimed).toBe(false);
  });

  it('preserves source refs from provider-status handoff rows', () => {
    const readModel = buildReceiptBoundaryReadModel();
    const providerDispatchRequired = readModel.rows[0];
    const manualReceiptRequired = readModel.rows[1];
    const providerUnavailable = readModel.rows[2];

    expect(providerDispatchRequired.sourcePreflightRowRef).toBe(
      'social-provider-preflight-social-provider-receipt-high-risk'
    );
    expect(providerDispatchRequired.sourceLocalOutboxRecordRef).toBe('local-outbox-social-provider-receipt');
    expect(providerDispatchRequired.receiptProofRequirements).toEqual([
      'social-provider-dispatch-runtime-required-social-provider-receipt-high-risk',
      'social-provider-receipt-ingestion-contract-required-social-provider-receipt-high-risk',
    ]);
    expect(manualReceiptRequired.sourceProviderStatus).toBe('manual-required');
    expect(manualReceiptRequired.receiptProofRequirements).toEqual([
      'social-provider-receipt-manual-provider-setup-social-provider-receipt-manual-required',
    ]);
    expect(providerUnavailable.sourceProviderStatus).toBe('unavailable');
    expect(providerUnavailable.receiptProofRequirements).toEqual([
      'social-provider-receipt-provider-unavailable-social-provider-receipt-unavailable',
    ]);
  });

  it('rejects receipt, delivery, and status/source overclaims', () => {
    const readModel = buildReceiptBoundaryReadModel();
    const providerDispatchRequired = readModel.rows[0];
    const unavailable = readModel.rows[2];

    expect(
      SocialAlertReportProviderReceiptBoundaryReadModelSchema.safeParse({
        ...readModel,
        providerDeliveryRuntimeClaimed: true,
      }).success
    ).toBe(false);
    expect(
      SocialAlertReportProviderReceiptBoundaryRowSchema.safeParse({
        ...providerDispatchRequired,
        providerReceiptRefs: ['provider-receipt-observed'],
      }).success
    ).toBe(false);
    expect(
      SocialAlertReportProviderReceiptBoundaryRowSchema.safeParse({
        ...unavailable,
        sourceProviderStatus: 'manual-required',
      }).success
    ).toBe(false);
  });
});

function buildReceiptBoundaryReadModel() {
  const preflight = buildSocialAlertReportProviderPreflightReadModel(
    {
      generatedAt: Timestamp,
      providerPreflightId: 'social-alert-report-provider-preflight-for-receipt-boundary',
      sourceContractRefs: ['social-alert-report-intent', 'social-alert-report-provider-status-handoff-proof'],
    },
    [BaseIntent, manualRequiredIntent(), unavailableIntent()]
  );
  const statusHandoff = buildSocialAlertReportProviderStatusHandoffReadModel(
    {
      generatedAt: Timestamp,
      handoffId: 'social-alert-report-provider-status-handoff-for-receipt-boundary',
      sourceContractRefs: [
        'social-alert-report-provider-preflight-proof',
        'v0-8-notification-provider-status-boundary',
        'notifications-expectation-provider-boundary',
      ],
    },
    preflight
  );

  expect(statusHandoff.rows.map((row) => row.sourcePreflightStatus)).toEqual([
    SocialAlertReportProviderPreflightStatus.ProviderAdapterRequired,
    SocialAlertReportProviderPreflightStatus.ManualRequired,
    SocialAlertReportProviderPreflightStatus.Unavailable,
  ]);

  return buildSocialAlertReportProviderReceiptBoundaryReadModel(
    {
      generatedAt: Timestamp,
      receiptBoundaryId: 'social-alert-report-provider-receipt-boundary-proof',
      sourceContractRefs: [
        'social-alert-report-provider-status-handoff-proof',
        'v0-8-notification-provider-status-boundary',
        'notifications-expectation-provider-boundary',
      ],
    },
    statusHandoff
  );
}

function manualRequiredIntent() {
  return {
    ...BaseIntent,
    alertReportIntentId: 'social-provider-receipt-manual-required',
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
    manualProofRequirements: ['manual-proof-social-provider-receipt-required'],
  } as const;
}

function unavailableIntent() {
  return {
    ...manualRequiredIntent(),
    alertReportIntentId: 'social-provider-receipt-unavailable',
    intentKind: SocialAlertReportIntentKind.CapabilityUnavailable,
    intentStatus: SocialAlertReportIntentStatus.Unavailable,
    notificationReasonCode: SocialAlertReportReasonCode.CapabilityUnavailable,
    parentTitleToken: SocialAlertReportParentCopyToken.UnavailableTitle,
    parentBodyToken: SocialAlertReportParentCopyToken.UnavailableBody,
    manualProofRequirements: ['manual-proof-social-provider-receipt-unavailable'],
  } as const;
}
