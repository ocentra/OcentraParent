import { describe, expect, it } from 'vitest';
import {
  SocialAlertReportAdapterDispatchState,
  SocialAlertReportDeliveryClaimState,
  SocialAlertReportIntentKind,
  SocialAlertReportIntentSchema,
  SocialAlertReportIntentStatus,
  SocialAlertReportParentCopyToken,
  SocialAlertReportPayloadField,
  SocialAlertReportReasonCode,
} from '../../src/social-alert-report-intent';
import {
  ParentActorRole,
  ParentContractSchemaVersion,
  ParentEvidenceReferenceKind,
  ParentPlatform,
} from '@ocentra-parent/schema-domain/family-reference-primitives';

const Timestamp = '2026-06-06T07:52:00Z';
const PolicyVersion = 'policy-social-alert-report-v1';

const ChildDevice = {
  deviceId: 'device-social-alert-report',
  childProfileId: 'child-social-alert-report',
  label: 'Study Phone',
  platform: ParentPlatform.Android,
} as const;

const EvidenceReference = {
  evidenceReferenceId: 'evidence-social-route-gate',
  kind: ParentEvidenceReferenceKind.PolicyDecision,
  observedAt: Timestamp,
} as const;

const ParentActionRef = {
  actionReferenceId: 'parent-action-social-review',
  actor: {
    actorId: 'parent-local-account',
    role: ParentActorRole.Parent,
  },
  policyVersion: PolicyVersion,
  createdAt: Timestamp,
} as const;

const MinimalPayloadFields = [
  SocialAlertReportPayloadField.AlertId,
  SocialAlertReportPayloadField.FamilyDeviceScope,
  SocialAlertReportPayloadField.Severity,
  SocialAlertReportPayloadField.ReasonCode,
  SocialAlertReportPayloadField.EvidenceRef,
  SocialAlertReportPayloadField.PolicyRef,
  SocialAlertReportPayloadField.ExplanationRef,
  SocialAlertReportPayloadField.ParentActionLinkRef,
] as const;

const BaseIntent = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  alertReportIntentId: 'social-alert-report-high-risk',
  intentKind: SocialAlertReportIntentKind.HighRiskSignal,
  intentStatus: SocialAlertReportIntentStatus.LocalOutboxEligible,
  priority: 'urgent',
  severity: 'critical',
  device: ChildDevice,
  notificationReasonCode: SocialAlertReportReasonCode.HighRiskSignal,
  providerChannelPreference: 'in-app',
  parentTitleToken: SocialAlertReportParentCopyToken.HighRiskTitle,
  parentBodyToken: SocialAlertReportParentCopyToken.HighRiskBody,
  parentActionToken: SocialAlertReportParentCopyToken.OpenParentReviewAction,
  dashboardPanelRefs: ['panel-feed-video-gates'],
  explanationSnapshotRef: 'social-explanation-snapshot-alert-report',
  explanationEventRefs: ['social-explanation-event-feed-video-gate'],
  evidenceReferences: [EvidenceReference],
  policyRefs: ['policy-ref-social-high-risk'],
  auditRefs: ['audit-ref-social-alert-report'],
  parentReportRef: null,
  parentActionRef: null,
  localOutboxRecordRef: 'local-outbox-social-alert-report',
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

describe('social alert/report intent contracts', () => {
  acceptsLocalOutboxAlertIntent();
  acceptsApprovalAndWeeklyReportIntents();
  rejectsUnsafeRuntimeAndRawContentClaims();
  rejectsMismatchedRefsAndCopy();
  keepsManualAndUnavailableStatesHonest();
});

function acceptsLocalOutboxAlertIntent() {
  it('accepts a high-risk social alert intent with local-outbox-only delivery claims', () => {
    const parsed = SocialAlertReportIntentSchema.parse(BaseIntent);

    expect(parsed.intentKind).toBe(SocialAlertReportIntentKind.HighRiskSignal);
    expect(parsed.deliveryClaimState).toBe(SocialAlertReportDeliveryClaimState.LocalOutboxOnly);
    expect(parsed.providerDeliveryAttempted).toBe(false);
    expect(parsed.adapterDispatchState).toBe(SocialAlertReportAdapterDispatchState.NotDispatched);
    expect(parsed.minimalPayloadFields).toEqual(MinimalPayloadFields);
  });
}

function acceptsApprovalAndWeeklyReportIntents() {
  it('accepts account approval and weekly summary intents when action/report refs are present', () => {
    const approval = SocialAlertReportIntentSchema.parse({
      ...BaseIntent,
      alertReportIntentId: 'social-alert-report-account-approval',
      intentKind: SocialAlertReportIntentKind.AccountApprovalNeeded,
      priority: 'attention',
      severity: 'warning',
      notificationReasonCode: SocialAlertReportReasonCode.AccountApproval,
      parentTitleToken: SocialAlertReportParentCopyToken.ApprovalTitle,
      parentBodyToken: SocialAlertReportParentCopyToken.ApprovalBody,
      parentActionRef: ParentActionRef,
      dashboardPanelRefs: ['panel-account-approval-queue'],
      explanationEventRefs: ['social-explanation-event-account-approval'],
    });
    const weeklySummary = SocialAlertReportIntentSchema.parse({
      ...BaseIntent,
      alertReportIntentId: 'social-alert-report-weekly-summary',
      intentKind: SocialAlertReportIntentKind.WeeklySummary,
      intentStatus: SocialAlertReportIntentStatus.IntentOnly,
      priority: 'info',
      severity: 'info',
      notificationReasonCode: SocialAlertReportReasonCode.WeeklySummary,
      parentTitleToken: SocialAlertReportParentCopyToken.WeeklySummaryTitle,
      parentBodyToken: SocialAlertReportParentCopyToken.WeeklySummaryBody,
      parentReportRef: 'parent-report-social-weekly-summary',
      localOutboxRecordRef: null,
      deliveryClaimState: SocialAlertReportDeliveryClaimState.NotClaimed,
      dashboardPanelRefs: ['panel-decision-memory'],
      explanationEventRefs: ['social-explanation-event-decision-memory'],
    });

    expect(approval.parentActionRef).toEqual(ParentActionRef);
    expect(weeklySummary.parentReportRef).toBe('parent-report-social-weekly-summary');
  });
}

function rejectsUnsafeRuntimeAndRawContentClaims() {
  it('rejects raw social details provider delivery report delivery and enforcement claims', () => {
    for (const invalidIntent of [
      { ...BaseIntent, alertReportIntentId: 'social-alert-report-raw-account', rawAccountDataIncluded: true },
      { ...BaseIntent, alertReportIntentId: 'social-alert-report-raw-video', rawVideoContentIncluded: true },
      { ...BaseIntent, alertReportIntentId: 'social-alert-report-raw-message', rawMessageContentIncluded: true },
      { ...BaseIntent, alertReportIntentId: 'social-alert-report-screenshot', screenshotIncluded: true },
      { ...BaseIntent, alertReportIntentId: 'social-alert-report-provider-delivery', providerDeliveryAttempted: true },
      { ...BaseIntent, alertReportIntentId: 'social-alert-report-provider-receipt', providerReceiptRefs: ['receipt'] },
      { ...BaseIntent, alertReportIntentId: 'social-alert-report-delivered', reportDeliveryClaimed: true },
      { ...BaseIntent, alertReportIntentId: 'social-alert-report-final-policy', finalPolicyDecisionClaimed: true },
      { ...BaseIntent, alertReportIntentId: 'social-alert-report-enforcement', enforcementClaimed: true },
    ]) {
      expect(SocialAlertReportIntentSchema.safeParse(invalidIntent).success).toBe(false);
    }
  });
}

function rejectsMismatchedRefsAndCopy() {
  it('rejects mismatched reason/copy and missing action report or evidence refs', () => {
    for (const invalidIntent of [
      {
        ...BaseIntent,
        alertReportIntentId: 'social-alert-report-wrong-reason',
        notificationReasonCode: SocialAlertReportReasonCode.WeeklySummary,
      },
      {
        ...BaseIntent,
        alertReportIntentId: 'social-alert-report-wrong-copy',
        parentTitleToken: SocialAlertReportParentCopyToken.WeeklySummaryTitle,
      },
      {
        ...BaseIntent,
        alertReportIntentId: 'social-alert-report-missing-evidence',
        evidenceReferences: [],
      },
      {
        ...BaseIntent,
        alertReportIntentId: 'social-alert-report-approval-missing-action',
        intentKind: SocialAlertReportIntentKind.AccountApprovalNeeded,
        notificationReasonCode: SocialAlertReportReasonCode.AccountApproval,
        parentTitleToken: SocialAlertReportParentCopyToken.ApprovalTitle,
        parentBodyToken: SocialAlertReportParentCopyToken.ApprovalBody,
      },
      {
        ...BaseIntent,
        alertReportIntentId: 'social-alert-report-weekly-missing-report',
        intentKind: SocialAlertReportIntentKind.WeeklySummary,
        notificationReasonCode: SocialAlertReportReasonCode.WeeklySummary,
        parentTitleToken: SocialAlertReportParentCopyToken.WeeklySummaryTitle,
        parentBodyToken: SocialAlertReportParentCopyToken.WeeklySummaryBody,
      },
    ]) {
      expect(SocialAlertReportIntentSchema.safeParse(invalidIntent).success).toBe(false);
    }
  });
}

function keepsManualAndUnavailableStatesHonest() {
  it('keeps manual-required and unavailable intents out of queued provider claims', () => {
    const manualRequired = SocialAlertReportIntentSchema.parse({
      ...BaseIntent,
      alertReportIntentId: 'social-alert-report-manual-required',
      intentKind: SocialAlertReportIntentKind.ManualRequired,
      intentStatus: SocialAlertReportIntentStatus.ManualRequired,
      priority: 'attention',
      severity: 'warning',
      notificationReasonCode: SocialAlertReportReasonCode.ManualRequired,
      parentTitleToken: SocialAlertReportParentCopyToken.ManualRequiredTitle,
      parentBodyToken: SocialAlertReportParentCopyToken.ManualRequiredBody,
      parentActionToken: SocialAlertReportParentCopyToken.ReviewManuallyAction,
      localOutboxRecordRef: null,
      manualProofRequirements: ['social-provider-notification-capability-proof-required'],
      deliveryClaimState: SocialAlertReportDeliveryClaimState.ManualRequired,
    });
    const falseLocalOutboxClaim = SocialAlertReportIntentSchema.safeParse({
      ...manualRequired,
      localOutboxRecordRef: 'local-outbox-false-claim',
    });

    expect(manualRequired.deliveryClaimState).toBe(SocialAlertReportDeliveryClaimState.ManualRequired);
    expect(falseLocalOutboxClaim.success).toBe(false);
  });
}
