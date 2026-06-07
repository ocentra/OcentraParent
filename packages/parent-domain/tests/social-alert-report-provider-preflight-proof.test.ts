import { describe, expect, it } from 'vitest';
import {
  buildSocialAlertReportProviderPreflightReadModel,
  SocialAlertReportProviderPreflightReadModelSchema,
  SocialAlertReportProviderPreflightStatus,
} from '../src/social-alert-report-provider-preflight-proof';
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

const Timestamp = '2026-06-07T05:30:00Z';
const MinimalPayloadFields = Object.values(SocialAlertReportPayloadField);

const BaseIntent = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  alertReportIntentId: 'social-provider-preflight-high-risk',
  intentKind: SocialAlertReportIntentKind.HighRiskSignal,
  intentStatus: SocialAlertReportIntentStatus.LocalOutboxEligible,
  priority: 'urgent',
  severity: 'critical',
  device: {
    deviceId: 'device-social-provider-preflight',
    childProfileId: 'child-social-provider-preflight',
    label: 'Study Phone',
    platform: ParentPlatform.Android,
  },
  notificationReasonCode: SocialAlertReportReasonCode.HighRiskSignal,
  providerChannelPreference: 'in-app',
  parentTitleToken: SocialAlertReportParentCopyToken.HighRiskTitle,
  parentBodyToken: SocialAlertReportParentCopyToken.HighRiskBody,
  parentActionToken: SocialAlertReportParentCopyToken.OpenParentReviewAction,
  dashboardPanelRefs: ['panel-feed-video-gates'],
  explanationSnapshotRef: 'social-explanation-snapshot-provider-preflight',
  explanationEventRefs: ['social-explanation-event-provider-preflight'],
  evidenceReferences: [
    {
      evidenceReferenceId: 'evidence-social-provider-preflight',
      kind: ParentEvidenceReferenceKind.PolicyDecision,
      observedAt: Timestamp,
    },
  ],
  policyRefs: ['policy-ref-social-provider-preflight'],
  auditRefs: ['audit-ref-social-provider-preflight'],
  parentReportRef: null,
  parentActionRef: null,
  localOutboxRecordRef: 'local-outbox-social-provider-preflight',
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

describe('social alert/report provider preflight proof', () => {
  it('turns local-outbox social alert intents into provider-adapter-required rows without delivery claims', () => {
    const readModel = buildSocialAlertReportProviderPreflightReadModel(options(), [BaseIntent]);
    const row = readModel.rows[0];

    expect(readModel.providerAdapterRequiredCount).toBe(1);
    expect(row.status).toBe(SocialAlertReportProviderPreflightStatus.ProviderAdapterRequired);
    expect(row.sourceLocalOutboxRecordRef).toBe('local-outbox-social-provider-preflight');
    expect(row.adapterRequirementRefs).toEqual([
      'provider-adapter-required-social-provider-preflight-high-risk',
      'provider-credentials-required-social-provider-preflight-high-risk',
      'provider-smoke-proof-required-social-provider-preflight-high-risk',
    ]);
    expect(readModel.providerDeliveryRuntimeClaimed).toBe(false);
    expect(readModel.finalPolicyExecutionClaimed).toBe(false);
    expect(readModel.enforcementClaimed).toBe(false);
  });

  it('keeps manual-required and unavailable source intents blocked before provider setup', () => {
    const readModel = buildSocialAlertReportProviderPreflightReadModel(options(), [
      manualRequiredIntent(),
      unavailableIntent(),
    ]);

    expect(readModel.providerAdapterRequiredCount).toBe(0);
    expect(readModel.manualRequiredCount).toBe(1);
    expect(readModel.unavailableCount).toBe(1);
    expect(readModel.rows.map((row) => row.sourceLocalOutboxRecordRef)).toEqual([null, null]);
    expect(readModel.rows.map((row) => row.manualProofRequirements.length)).toEqual([1, 1]);
  });

  it('rejects delivery, receipt, final policy, and enforcement overclaims', () => {
    const readModel = buildSocialAlertReportProviderPreflightReadModel(options(), [BaseIntent]);

    for (const invalidReadModel of [
      { ...readModel, providerDeliveryRuntimeClaimed: true },
      { ...readModel, providerReceiptIngestionClaimed: true },
      { ...readModel, finalPolicyExecutionClaimed: true },
      { ...readModel, enforcementClaimed: true },
      { ...readModel, providerAdapterRequiredCount: 0 },
      { ...readModel, preflightNonClaims: ['no-provider-delivery-execution'] },
    ]) {
      expect(SocialAlertReportProviderPreflightReadModelSchema.safeParse(invalidReadModel).success).toBe(false);
    }
  });

  it('rejects unvalidated source intents before building preflight rows', () => {
    expect(() =>
      buildSocialAlertReportProviderPreflightReadModel(options(), [
        {
          ...BaseIntent,
          providerDeliveryAttempted: true,
        },
      ])
    ).toThrow();
  });
});

function options() {
  return {
    generatedAt: Timestamp,
    providerPreflightId: 'social-alert-report-provider-preflight-proof',
    sourceContractRefs: ['social-alert-report-intent', 'social-alert-report-intent-ui-proof'],
  };
}

function manualRequiredIntent() {
  return {
    ...BaseIntent,
    alertReportIntentId: 'social-provider-preflight-manual-required',
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
    manualProofRequirements: ['manual-proof-social-provider-preflight-required'],
  } as const;
}

function unavailableIntent() {
  return {
    ...manualRequiredIntent(),
    alertReportIntentId: 'social-provider-preflight-unavailable',
    intentKind: SocialAlertReportIntentKind.CapabilityUnavailable,
    intentStatus: SocialAlertReportIntentStatus.Unavailable,
    notificationReasonCode: SocialAlertReportReasonCode.CapabilityUnavailable,
    parentTitleToken: SocialAlertReportParentCopyToken.UnavailableTitle,
    parentBodyToken: SocialAlertReportParentCopyToken.UnavailableBody,
    manualProofRequirements: ['manual-proof-social-provider-capability-unavailable'],
  } as const;
}
