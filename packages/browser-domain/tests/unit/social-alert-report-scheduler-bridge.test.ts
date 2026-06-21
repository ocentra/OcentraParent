import { describe, expect, it } from 'vitest';
import { NotificationLocalOutboxSchedulerRecordSchema } from '@ocentra-parent/schema-domain/notification-local-outbox';
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
} from '@ocentra-parent/schema-domain/social-alert-report-intent';
import {
  buildSocialAlertReportLocalOutboxBridgeReadModel,
  type SocialAlertReportLocalOutboxBridgeReadModel,
} from '../../src/social-alert-report-local-outbox-bridge';
import {
  SocialAlertReportSchedulerBridgeReadModelSchema,
  SocialAlertReportSchedulerBridgeStatus,
  buildSocialAlertReportSchedulerBridgeReadModel,
  parseSocialAlertReportSchedulerJsonl,
  serializeSocialAlertReportSchedulerJsonl,
} from '../../src/social-alert-report-scheduler-bridge';

const Timestamp = '2026-06-07T07:52:00Z';
const PolicyVersion = 'policy-social-alert-report-scheduler-v1';
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
  family: { familyId: 'family-social-alert-report-scheduler-bridge' },
  parentAction: {
    actionReferenceId: 'parent-action-social-alert-report-scheduler-bridge',
    actor: { actorId: 'parent-social-alert-report-scheduler-bridge', role: ParentActorRole.Parent },
    policyVersion: PolicyVersion,
    createdAt: Timestamp,
  },
  generatedAt: Timestamp,
  bridgeId: 'social-alert-report-local-outbox-bridge-for-scheduler-proof',
  outboxRootRef: 'parent-owned-social-alert-report-local-outbox-root-for-scheduler',
  outboxFileRef: 'parent-owned-social-alert-report-local-outbox-jsonl-for-scheduler',
  localDataPathRef: 'parent-owned-social-alert-report-local-outbox-data-path-for-scheduler',
} as const;

const SchedulerOptions = {
  generatedAt: Timestamp,
  schedulerBridgeId: 'social-alert-report-scheduler-bridge-proof',
  schedulerArtifactRootRef: 'parent-owned-social-alert-report-scheduler-root',
  schedulerArtifactRef: 'parent-owned-social-alert-report-scheduler-jsonl-ref',
  schedulerNowAt: Timestamp,
} as const;

const BaseIntent = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  alertReportIntentId: 'social-alert-report-high-risk-scheduler',
  intentKind: SocialAlertReportIntentKind.HighRiskSignal,
  intentStatus: SocialAlertReportIntentStatus.LocalOutboxEligible,
  priority: 'urgent',
  severity: 'critical',
  device: {
    deviceId: 'device-social-alert-report-scheduler-bridge',
    childProfileId: 'child-social-alert-report-scheduler-bridge',
    label: 'Study Phone',
    platform: ParentPlatform.Android,
  },
  notificationReasonCode: SocialAlertReportReasonCode.HighRiskSignal,
  providerChannelPreference: 'in-app',
  parentTitleToken: SocialAlertReportParentCopyToken.HighRiskTitle,
  parentBodyToken: SocialAlertReportParentCopyToken.HighRiskBody,
  parentActionToken: SocialAlertReportParentCopyToken.OpenParentReviewAction,
  dashboardPanelRefs: ['panel-feed-video-gates'],
  explanationSnapshotRef: 'social-explanation-snapshot-scheduler',
  explanationEventRefs: ['social-explanation-event-scheduler'],
  evidenceReferences: [
    {
      evidenceReferenceId: 'evidence-social-alert-report-scheduler',
      kind: ParentEvidenceReferenceKind.PolicyDecision,
      observedAt: Timestamp,
    },
  ],
  policyRefs: ['policy-ref-social-alert-report-scheduler'],
  auditRefs: ['audit-ref-social-alert-report-scheduler'],
  parentReportRef: null,
  parentActionRef: BridgeOptions.parentAction,
  localOutboxRecordRef: 'local-outbox-record-social-alert-report-high-risk-scheduler',
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

describe('social alert/report scheduler bridge', () => {
  it('schedules only linked social alert/report local outbox records through existing scheduler records', () => {
    const readModel = buildSocialAlertReportSchedulerBridgeReadModel(SchedulerOptions, sourceReadModel());
    const records = parseSocialAlertReportSchedulerJsonl(serializeSocialAlertReportSchedulerJsonl(readModel));

    expect(readModel.scheduledRecordCount).toBe(2);
    expect(readModel.unscheduledManualRequiredCount).toBe(1);
    expect(readModel.unscheduledUnavailableCount).toBe(1);
    expect(records.map((record) => record.sourceEntryId)).toEqual([
      'local-outbox-record-social-alert-report-high-risk-scheduler',
      'local-outbox-record-social-alert-report-account-approval-scheduler',
    ]);
    expect(records.map((record) => record.schedulerState)).toEqual(['due-local', 'due-local']);
    expect(records.map((record) => record.nextAttemptAt)).toEqual([Timestamp, Timestamp]);
    expect(records.map((record) => record.providerDeliveryAttempted)).toEqual([false, false]);
  });

  it('keeps manual-required and unavailable social rows out of scheduler JSONL', () => {
    const readModel = buildSocialAlertReportSchedulerBridgeReadModel(SchedulerOptions, sourceReadModel());
    const unscheduledRows = readModel.rows.filter(
      (row) => row.status !== SocialAlertReportSchedulerBridgeStatus.ScheduledLocal
    );

    expect(unscheduledRows.map((row) => row.status)).toEqual([
      SocialAlertReportSchedulerBridgeStatus.ManualRequired,
      SocialAlertReportSchedulerBridgeStatus.Unavailable,
    ]);
    expect(unscheduledRows.map((row) => row.schedulerRecord)).toEqual([null, null]);
    expect(unscheduledRows.map((row) => row.blockedReasonRefs)).toEqual([
      ['provider preference setup before social alert/report can be queued'],
      ['local evidence and policy readiness before unavailable social alert/report can be queued'],
    ]);
  });

  it('rejects scheduler runtime and provider overclaims at the social bridge boundary', () => {
    const readModel = buildSocialAlertReportSchedulerBridgeReadModel(SchedulerOptions, sourceReadModel());
    const record = readModel.rows[0]?.schedulerRecord;
    if (record === null || record === undefined) {
      throw new Error('expected scheduled record');
    }

    expect(
      SocialAlertReportSchedulerBridgeReadModelSchema.safeParse({
        ...readModel,
        quietHoursTimerRuntimeClaimed: true,
      }).success
    ).toBe(false);
    expect(
      NotificationLocalOutboxSchedulerRecordSchema.safeParse({ ...record, providerDeliveryObserved: true }).success
    ).toBe(false);
    expect(() =>
      parseSocialAlertReportSchedulerJsonl(`${JSON.stringify({ ...record, rawMessageTextIncluded: true })}\n`)
    ).toThrow();
  });
});

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
    alertReportIntentId: 'social-alert-report-account-approval-scheduler',
    intentKind: SocialAlertReportIntentKind.AccountApprovalNeeded,
    priority: 'attention',
    severity: 'warning',
    notificationReasonCode: SocialAlertReportReasonCode.AccountApproval,
    providerChannelPreference: 'email',
    parentTitleToken: SocialAlertReportParentCopyToken.ApprovalTitle,
    parentBodyToken: SocialAlertReportParentCopyToken.ApprovalBody,
    localOutboxRecordRef: 'local-outbox-record-social-alert-report-account-approval-scheduler',
  } as const;
}

function manualRequiredIntent() {
  return {
    ...BaseIntent,
    alertReportIntentId: 'social-alert-report-manual-required-scheduler',
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
    alertReportIntentId: 'social-alert-report-unavailable-scheduler',
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
