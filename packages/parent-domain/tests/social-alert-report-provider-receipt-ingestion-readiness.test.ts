import { describe, expect, it } from 'vitest';
import {
  buildSocialAlertReportProviderPreflightReadModel,
  SocialAlertReportProviderPreflightStatus,
} from '../src/social-alert-report-provider-preflight-proof';
import { buildSocialAlertReportProviderStatusHandoffReadModel } from '../src/social-alert-report-provider-status-handoff-proof';
import { buildSocialAlertReportProviderReceiptBoundaryReadModel } from '../src/social-alert-report-provider-receipt-boundary-proof';
import {
  buildSocialAlertReportProviderReceiptIngestionReadinessReadModel,
  SocialAlertReportProviderReceiptIngestionReadinessReadModelSchema,
  SocialAlertReportProviderReceiptIngestionReadinessRowSchema,
} from '../src/social-alert-report-provider-receipt-ingestion-readiness';
import {
  SocialAlertReportAdapterDispatchState,
  SocialAlertReportDeliveryClaimState,
  SocialAlertReportIntentKind,
  SocialAlertReportIntentStatus,
  SocialAlertReportParentCopyToken,
  SocialAlertReportPayloadField,
  SocialAlertReportReasonCode,
} from '../src/social-alert-report-intent';
import { ParentContractSchemaVersion, ParentEvidenceReferenceKind, ParentPlatform } from '../src/reference-primitives';

const Timestamp = '2026-06-08T05:55:00Z';
const MinimalPayloadFields = Object.values(SocialAlertReportPayloadField);

const BaseIntent = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  alertReportIntentId: 'social-provider-ingestion-high-risk',
  intentKind: SocialAlertReportIntentKind.HighRiskSignal,
  intentStatus: SocialAlertReportIntentStatus.LocalOutboxEligible,
  priority: 'urgent',
  severity: 'critical',
  device: {
    deviceId: 'device-social-provider-ingestion',
    childProfileId: 'child-social-provider-ingestion',
    label: 'Study Phone',
    platform: ParentPlatform.Android,
  },
  notificationReasonCode: SocialAlertReportReasonCode.HighRiskSignal,
  providerChannelPreference: 'in-app',
  parentTitleToken: SocialAlertReportParentCopyToken.HighRiskTitle,
  parentBodyToken: SocialAlertReportParentCopyToken.HighRiskBody,
  parentActionToken: SocialAlertReportParentCopyToken.OpenParentReviewAction,
  dashboardPanelRefs: ['panel-feed-video-gates'],
  explanationSnapshotRef: 'social-explanation-snapshot-provider-ingestion',
  explanationEventRefs: ['social-explanation-event-provider-ingestion'],
  evidenceReferences: [
    {
      evidenceReferenceId: 'evidence-social-provider-ingestion',
      kind: ParentEvidenceReferenceKind.PolicyDecision,
      observedAt: Timestamp,
    },
  ],
  policyRefs: ['policy-ref-social-provider-ingestion'],
  auditRefs: ['audit-ref-social-provider-ingestion'],
  parentReportRef: null,
  parentActionRef: null,
  localOutboxRecordRef: 'local-outbox-social-provider-ingestion',
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

describe('social alert/report provider receipt ingestion readiness proof', () => {
  it('projects receipt boundary rows into ingestion readiness without webhook or receipt claims', () => {
    const readModel = buildReceiptIngestionReadinessReadModel();

    expect(readModel.ingestionContractRequiredCount).toBe(1);
    expect(readModel.manualReceiptRequiredCount).toBe(1);
    expect(readModel.providerUnavailableCount).toBe(1);
    expect(readModel.providerReceiptObservedCount).toBe(0);
    expect(readModel.rows.map((row) => row.ingestionReadinessState)).toEqual([
      'ingestion-contract-required',
      'manual-receipt-required',
      'provider-unavailable',
    ]);
    expect(readModel.providerReceiptIngestionRuntimeClaimed).toBe(false);
    expect(readModel.providerWebhookRuntimeClaimed).toBe(false);
    expect(readModel.providerCredentialsClaimed).toBe(false);
    expect(readModel.enforcementClaimed).toBe(false);
  });

  it('requires webhook, credential, and durable store proof before ingestion can be observed', () => {
    const readModel = buildReceiptIngestionReadinessReadModel();
    const ingestionRequired = readModel.rows[0];

    expect(ingestionRequired.sourceReceiptBoundaryState).toBe('provider-dispatch-required');
    expect(ingestionRequired.webhookEndpointRef).toBeNull();
    expect(ingestionRequired.providerCredentialRef).toBeNull();
    expect(ingestionRequired.durableReceiptResultRef).toBeNull();
    expect(ingestionRequired.providerReceiptObservedRefs).toEqual([]);
    expect(ingestionRequired.ingestionProofRequirements).toEqual([
      'social-provider-receipt-webhook-contract-required-social-provider-ingestion-high-risk',
      'social-provider-receipt-credential-proof-required-social-provider-ingestion-high-risk',
      'social-provider-receipt-durable-store-required-social-provider-ingestion-high-risk',
    ]);
  });

  it('rejects forged provider receipt ingestion, webhook, and source-state claims', () => {
    const readModel = buildReceiptIngestionReadinessReadModel();
    const ingestionRequired = readModel.rows[0];
    const unavailable = readModel.rows[2];

    expect(
      SocialAlertReportProviderReceiptIngestionReadinessReadModelSchema.safeParse({
        ...readModel,
        providerReceiptObservedCount: 1,
      }).success
    ).toBe(false);
    expect(
      SocialAlertReportProviderReceiptIngestionReadinessRowSchema.safeParse({
        ...ingestionRequired,
        webhookEndpointRef: 'provider-webhook-endpoint-observed',
      }).success
    ).toBe(false);
    expect(
      SocialAlertReportProviderReceiptIngestionReadinessRowSchema.safeParse({
        ...ingestionRequired,
        providerReceiptObservedRefs: ['provider-receipt-observed'],
      }).success
    ).toBe(false);
    expect(
      SocialAlertReportProviderReceiptIngestionReadinessRowSchema.safeParse({
        ...unavailable,
        sourceReceiptBoundaryState: 'provider-dispatch-required',
      }).success
    ).toBe(false);
  });
});

function buildReceiptIngestionReadinessReadModel() {
  const preflight = buildSocialAlertReportProviderPreflightReadModel(
    {
      generatedAt: Timestamp,
      providerPreflightId: 'social-alert-report-provider-preflight-for-receipt-ingestion',
      sourceContractRefs: ['social-alert-report-intent', 'social-alert-report-provider-status-handoff-proof'],
    },
    [BaseIntent, manualRequiredIntent(), unavailableIntent()]
  );
  const statusHandoff = buildSocialAlertReportProviderStatusHandoffReadModel(
    {
      generatedAt: Timestamp,
      handoffId: 'social-alert-report-provider-status-handoff-for-receipt-ingestion',
      sourceContractRefs: [
        'social-alert-report-provider-preflight-proof',
        'v0-8-notification-provider-status-boundary',
        'notifications-expectation-provider-boundary',
      ],
    },
    preflight
  );
  const receiptBoundary = buildSocialAlertReportProviderReceiptBoundaryReadModel(
    {
      generatedAt: Timestamp,
      receiptBoundaryId: 'social-alert-report-provider-receipt-boundary-for-ingestion',
      sourceContractRefs: [
        'social-alert-report-provider-status-handoff-proof',
        'v0-8-notification-provider-status-boundary',
        'notifications-expectation-provider-boundary',
      ],
    },
    statusHandoff
  );

  expect(statusHandoff.rows.map((row) => row.sourcePreflightStatus)).toEqual([
    SocialAlertReportProviderPreflightStatus.ProviderAdapterRequired,
    SocialAlertReportProviderPreflightStatus.ManualRequired,
    SocialAlertReportProviderPreflightStatus.Unavailable,
  ]);

  return buildSocialAlertReportProviderReceiptIngestionReadinessReadModel(
    {
      generatedAt: Timestamp,
      readinessId: 'social-alert-report-provider-receipt-ingestion-readiness-proof',
      sourceContractRefs: [
        'social-alert-report-provider-receipt-boundary-proof',
        'provider-receipt-webhook-contract',
        'provider-receipt-durable-store-contract',
      ],
    },
    receiptBoundary
  );
}

function manualRequiredIntent() {
  return {
    ...BaseIntent,
    alertReportIntentId: 'social-provider-ingestion-manual-required',
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
    manualProofRequirements: ['manual-proof-social-provider-ingestion-required'],
  } as const;
}

function unavailableIntent() {
  return {
    ...manualRequiredIntent(),
    alertReportIntentId: 'social-provider-ingestion-unavailable',
    intentKind: SocialAlertReportIntentKind.CapabilityUnavailable,
    intentStatus: SocialAlertReportIntentStatus.Unavailable,
    notificationReasonCode: SocialAlertReportReasonCode.CapabilityUnavailable,
    parentTitleToken: SocialAlertReportParentCopyToken.UnavailableTitle,
    parentBodyToken: SocialAlertReportParentCopyToken.UnavailableBody,
    manualProofRequirements: ['manual-proof-social-provider-ingestion-unavailable'],
  } as const;
}
