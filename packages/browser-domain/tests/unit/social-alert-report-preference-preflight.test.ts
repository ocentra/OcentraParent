import { describe, expect, it } from 'vitest';
import {
  ParentActorRole,
  ParentContractSchemaVersion,
  ParentEvidenceReferenceKind,
  ParentPlatform,
} from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  SocialAlertReportDeliveryClaimState,
  SocialAlertReportIntentKind,
  SocialAlertReportIntentStatus,
  SocialAlertReportParentCopyToken,
  SocialAlertReportPayloadField,
  SocialAlertReportReasonCode,
} from '../../src/social-alert-report-intent';
import {
  buildSocialAlertReportLocalOutboxBridgeReadModel,
  type SocialAlertReportLocalOutboxBridgeReadModel,
} from '../../src/social-alert-report-local-outbox-bridge';
import {
  SocialAlertReportPreferencePreflightReadModelSchema,
  SocialAlertReportPreferencePreflightStatus,
  buildSocialAlertReportPreferencePreflightReadModel,
} from '../../src/social-alert-report-preference-preflight';
import { buildSocialAlertReportSchedulerBridgeReadModel } from '../../src/social-alert-report-scheduler-bridge';

const Timestamp = '2026-06-07T08:27:00Z';
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

const BridgeOptions = {
  family: { familyId: 'family-social-alert-report-preference-preflight' },
  parentAction: {
    actionReferenceId: 'parent-action-social-alert-report-preference-preflight',
    actor: { actorId: 'parent-social-alert-report-preference-preflight', role: ParentActorRole.Parent },
    policyVersion: 'policy-social-alert-report-preference-preflight-v1',
    createdAt: Timestamp,
  },
  generatedAt: Timestamp,
  bridgeId: 'social-alert-report-local-outbox-bridge-for-preference-preflight',
  outboxRootRef: 'parent-owned-social-alert-report-local-outbox-root-for-preference',
  outboxFileRef: 'parent-owned-social-alert-report-local-outbox-jsonl-for-preference',
  localDataPathRef: 'parent-owned-social-alert-report-local-outbox-data-path-for-preference',
} as const;

const SchedulerOptions = {
  generatedAt: Timestamp,
  schedulerBridgeId: 'social-alert-report-scheduler-bridge-for-preference-preflight',
  schedulerArtifactRootRef: 'parent-owned-social-alert-report-scheduler-root-for-preference',
  schedulerArtifactRef: 'parent-owned-social-alert-report-scheduler-jsonl-ref-for-preference',
  schedulerNowAt: Timestamp,
} as const;

const PreferenceOptions = {
  generatedAt: Timestamp,
  preferencePreflightId: 'social-alert-report-preference-preflight-proof',
  sourceContractRefs: [
    'social-alert-report-scheduler-bridge',
    'notification-parent-preference-boundary',
    'notification-quiet-hours-policy-boundary',
  ],
} as const;

const BaseIntent = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  alertReportIntentId: 'social-alert-report-high-risk-preference',
  intentKind: SocialAlertReportIntentKind.HighRiskSignal,
  intentStatus: SocialAlertReportIntentStatus.LocalOutboxEligible,
  priority: 'urgent',
  severity: 'critical',
  device: {
    deviceId: 'device-social-alert-report-preference-preflight',
    childProfileId: 'child-social-alert-report-preference-preflight',
    label: 'Study Phone',
    platform: ParentPlatform.Android,
  },
  notificationReasonCode: SocialAlertReportReasonCode.HighRiskSignal,
  providerChannelPreference: 'in-app',
  parentTitleToken: SocialAlertReportParentCopyToken.HighRiskTitle,
  parentBodyToken: SocialAlertReportParentCopyToken.HighRiskBody,
  parentActionToken: SocialAlertReportParentCopyToken.OpenParentReviewAction,
  dashboardPanelRefs: ['panel-feed-video-gates'],
  explanationSnapshotRef: 'social-explanation-snapshot-preference',
  explanationEventRefs: ['social-explanation-event-preference'],
  evidenceReferences: [
    {
      evidenceReferenceId: 'evidence-social-alert-report-preference',
      kind: ParentEvidenceReferenceKind.PolicyDecision,
      observedAt: Timestamp,
    },
  ],
  policyRefs: ['policy-ref-social-alert-report-preference'],
  auditRefs: ['audit-ref-social-alert-report-preference'],
  parentReportRef: null,
  parentActionRef: BridgeOptions.parentAction,
  localOutboxRecordRef: 'local-outbox-record-social-alert-report-high-risk-preference',
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
  adapterDispatchState: 'not-dispatched',
  adapterActionClaimed: false,
  createdAt: Timestamp,
} as const;

describe('social alert/report preference preflight', () => {
  it('requires parent preference and quiet-hours proof for scheduled social alert/report rows', () => {
    const readModel = buildSocialAlertReportPreferencePreflightReadModel(PreferenceOptions, schedulerReadModel());

    expect(readModel.parentPreferenceRequiredCount).toBe(2);
    expect(readModel.manualRequiredCount).toBe(1);
    expect(readModel.unavailableCount).toBe(1);
    expect(readModel.rows.map((row) => row.status)).toEqual([
      SocialAlertReportPreferencePreflightStatus.ParentPreferenceRequired,
      SocialAlertReportPreferencePreflightStatus.ParentPreferenceRequired,
      SocialAlertReportPreferencePreflightStatus.ManualRequired,
      SocialAlertReportPreferencePreflightStatus.Unavailable,
    ]);
    expect(readModel.rows.slice(0, 2).map((row) => row.providerChannelRef)).toEqual(['in-app', 'email']);
    expect(readModel.rows.slice(0, 2).map((row) => row.parentPreferenceState)).toEqual([
      'manual-setup-required',
      'manual-setup-required',
    ]);
    expect(readModel.rows.slice(0, 2).map((row) => row.quietHoursDecision)).toEqual([
      'manual-required',
      'manual-required',
    ]);
  });

  it('keeps manual-required and unavailable social source rows blocked before preflight', () => {
    const readModel = buildSocialAlertReportPreferencePreflightReadModel(PreferenceOptions, schedulerReadModel());
    const blockedRows = readModel.rows.slice(2);

    expect(blockedRows.map((row) => row.sourceSchedulerEntryRef)).toEqual([null, null]);
    expect(blockedRows.map((row) => row.parentPreferenceState)).toEqual([null, null]);
    expect(blockedRows.map((row) => row.quietHoursDecision)).toEqual([null, null]);
    expect(blockedRows.map((row) => row.manualProofRequirements)).toEqual([
      ['provider preference setup before social alert/report can be queued'],
      ['local evidence and policy readiness before unavailable social alert/report can be queued'],
    ]);
  });

  it('rejects notification preference UI, quiet-hours runtime, and delivery overclaims', () => {
    const readModel = buildSocialAlertReportPreferencePreflightReadModel(PreferenceOptions, schedulerReadModel());

    expect(readModel.parentNotificationPreferenceUiClaimed).toBe(false);
    expect(readModel.quietHoursTimerRuntimeClaimed).toBe(false);
    expect(readModel.providerDeliveryRuntimeClaimed).toBe(false);
    expect(readModel.finalPolicyExecutionClaimed).toBe(false);
    expect(readModel.enforcementClaimed).toBe(false);
    expect(
      SocialAlertReportPreferencePreflightReadModelSchema.safeParse({
        ...readModel,
        parentNotificationPreferenceUiClaimed: true,
      }).success
    ).toBe(false);
    expect(
      SocialAlertReportPreferencePreflightReadModelSchema.safeParse({
        ...readModel,
        quietHoursTimerRuntimeClaimed: true,
      }).success
    ).toBe(false);
    expect(
      SocialAlertReportPreferencePreflightReadModelSchema.safeParse({
        ...readModel,
        providerDeliveryRuntimeClaimed: true,
      }).success
    ).toBe(false);
  });
});

function schedulerReadModel() {
  return buildSocialAlertReportSchedulerBridgeReadModel(SchedulerOptions, sourceReadModel());
}

function sourceReadModel(): SocialAlertReportLocalOutboxBridgeReadModel {
  return buildSocialAlertReportLocalOutboxBridgeReadModel(BridgeOptions, [
    BaseIntent,
    accountApprovalIntent(),
    manualRequiredIntent(),
    unavailableIntent(),
  ]);
}

function accountApprovalIntent() {
  return {
    ...BaseIntent,
    alertReportIntentId: 'social-alert-report-account-approval-preference',
    intentKind: SocialAlertReportIntentKind.AccountApprovalNeeded,
    priority: 'attention',
    severity: 'warning',
    notificationReasonCode: SocialAlertReportReasonCode.AccountApproval,
    providerChannelPreference: 'email',
    parentTitleToken: SocialAlertReportParentCopyToken.ApprovalTitle,
    parentBodyToken: SocialAlertReportParentCopyToken.ApprovalBody,
    localOutboxRecordRef: 'local-outbox-record-social-alert-report-account-approval-preference',
  } as const;
}

function manualRequiredIntent() {
  return {
    ...BaseIntent,
    alertReportIntentId: 'social-alert-report-manual-required-preference',
    intentKind: SocialAlertReportIntentKind.ManualRequired,
    intentStatus: SocialAlertReportIntentStatus.ManualRequired,
    priority: 'attention',
    severity: 'warning',
    notificationReasonCode: SocialAlertReportReasonCode.ManualRequired,
    parentTitleToken: SocialAlertReportParentCopyToken.ManualRequiredTitle,
    parentBodyToken: SocialAlertReportParentCopyToken.ManualRequiredBody,
    parentActionToken: SocialAlertReportParentCopyToken.ReviewManuallyAction,
    localOutboxRecordRef: null,
    manualProofRequirements: ['provider preference setup before social alert/report can be queued'],
    deliveryClaimState: SocialAlertReportDeliveryClaimState.ManualRequired,
  } as const;
}

function unavailableIntent() {
  return {
    ...BaseIntent,
    alertReportIntentId: 'social-alert-report-unavailable-preference',
    intentKind: SocialAlertReportIntentKind.CapabilityUnavailable,
    intentStatus: SocialAlertReportIntentStatus.Unavailable,
    priority: 'info',
    severity: 'info',
    notificationReasonCode: SocialAlertReportReasonCode.CapabilityUnavailable,
    parentTitleToken: SocialAlertReportParentCopyToken.UnavailableTitle,
    parentBodyToken: SocialAlertReportParentCopyToken.UnavailableBody,
    parentActionToken: SocialAlertReportParentCopyToken.ReviewManuallyAction,
    localOutboxRecordRef: null,
    manualProofRequirements: [
      'local evidence and policy readiness before unavailable social alert/report can be queued',
    ],
    deliveryClaimState: SocialAlertReportDeliveryClaimState.ManualRequired,
  } as const;
}
