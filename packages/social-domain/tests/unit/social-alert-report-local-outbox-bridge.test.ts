import { describe, expect, it } from 'vitest';
import {
  NotificationLocalOutboxRecordSchema,
  type NotificationLocalOutboxRecord,
} from '../../src/notification-local-outbox-adapter-proof';
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
  parseSocialAlertReportLocalOutboxJsonl,
  serializeSocialAlertReportLocalOutboxJsonl,
  SocialAlertReportLocalOutboxBridgeReadModelSchema,
  SocialAlertReportLocalOutboxBridgeStatus,
} from '../../src/social-alert-report-local-outbox-bridge';
import {
  ParentActorRole,
  ParentContractSchemaVersion,
  ParentEvidenceReferenceKind,
  ParentPlatform,
} from '@ocentra-parent/family-domain/reference-primitives';

const Timestamp = '2026-06-07T07:04:00Z';
const PolicyVersion = 'policy-social-alert-report-outbox-v1';
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
  family: { familyId: 'family-social-alert-report-outbox-bridge' },
  parentAction: {
    actionReferenceId: 'parent-action-social-alert-report-outbox-bridge',
    actor: { actorId: 'parent-social-alert-report-outbox-bridge', role: ParentActorRole.Parent },
    policyVersion: PolicyVersion,
    createdAt: Timestamp,
  },
  generatedAt: Timestamp,
  bridgeId: 'social-alert-report-local-outbox-bridge-proof',
  outboxRootRef: 'parent-owned-social-alert-report-local-outbox-root',
  outboxFileRef: 'parent-owned-social-alert-report-local-outbox-jsonl-ref',
  localDataPathRef: 'parent-owned-social-alert-report-local-outbox-data-path-ref',
} as const;

const BaseIntent = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  alertReportIntentId: 'social-alert-report-high-risk-outbox',
  intentKind: SocialAlertReportIntentKind.HighRiskSignal,
  intentStatus: SocialAlertReportIntentStatus.LocalOutboxEligible,
  priority: 'urgent',
  severity: 'critical',
  device: {
    deviceId: 'device-social-alert-report-outbox-bridge',
    childProfileId: 'child-social-alert-report-outbox-bridge',
    label: 'Study Phone',
    platform: ParentPlatform.Android,
  },
  notificationReasonCode: SocialAlertReportReasonCode.HighRiskSignal,
  providerChannelPreference: 'in-app',
  parentTitleToken: SocialAlertReportParentCopyToken.HighRiskTitle,
  parentBodyToken: SocialAlertReportParentCopyToken.HighRiskBody,
  parentActionToken: SocialAlertReportParentCopyToken.OpenParentReviewAction,
  dashboardPanelRefs: ['panel-feed-video-gates'],
  explanationSnapshotRef: 'social-explanation-snapshot-outbox-bridge',
  explanationEventRefs: ['social-explanation-event-outbox-bridge'],
  evidenceReferences: [
    {
      evidenceReferenceId: 'evidence-social-alert-report-outbox-bridge',
      kind: ParentEvidenceReferenceKind.PolicyDecision,
      observedAt: Timestamp,
    },
  ],
  policyRefs: ['policy-ref-social-alert-report-outbox-bridge'],
  auditRefs: ['audit-ref-social-alert-report-outbox-bridge'],
  parentReportRef: null,
  parentActionRef: null,
  localOutboxRecordRef: 'local-outbox-record-social-alert-report-high-risk',
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

describe('social alert/report local outbox bridge', () => {
  it('writes and rereads local-outbox records only for eligible social alert/report intents', () => {
    const readModel = buildSocialAlertReportLocalOutboxBridgeReadModel(BridgeOptions, [
      BaseIntent,
      accountApprovalIntent(),
      manualRequiredIntent(),
      unavailableIntent(),
    ]);
    const jsonl = serializeSocialAlertReportLocalOutboxJsonl(readModel);
    const records = parseSocialAlertReportLocalOutboxJsonl(jsonl);

    expect(readModel.linkedRecordCount).toBe(2);
    expect(readModel.manualRequiredCount).toBe(1);
    expect(readModel.unavailableCount).toBe(1);
    expect(records.map((record) => record.entryId)).toEqual([
      'local-outbox-record-social-alert-report-high-risk',
      'local-outbox-record-social-alert-report-account-approval',
    ]);
    expect(records.map((record) => record.envelope.reasonCode)).toEqual(['policy-violation', 'parent-request']);
    expect(records.map((record) => record.providerDeliveryAttempted)).toEqual([false, false]);
    expect(records.map((record) => record.envelope.rawMessageTextIncluded)).toEqual([false, false]);
  });

  it('keeps manual-required and unavailable social alert/report intents out of queued JSONL records', () => {
    const readModel = buildSocialAlertReportLocalOutboxBridgeReadModel(BridgeOptions, [
      manualRequiredIntent(),
      unavailableIntent(),
    ]);
    const statuses = readModel.rows.map((row) => row.status);
    const blockedRefs = readModel.rows.map((row) => row.blockedReasonRefs);

    expect(statuses).toEqual([
      SocialAlertReportLocalOutboxBridgeStatus.ManualRequired,
      SocialAlertReportLocalOutboxBridgeStatus.Unavailable,
    ]);
    expect(blockedRefs).toEqual([
      ['provider preference setup before social alert/report can be queued'],
      ['local evidence and policy readiness before unavailable social alert/report can be queued'],
    ]);
    expect(serializeSocialAlertReportLocalOutboxJsonl(readModel)).toBe('\n');
  });

  it('rejects provider delivery overclaims and unsafe JSONL records at the bridge boundary', () => {
    const readModel = buildSocialAlertReportLocalOutboxBridgeReadModel(BridgeOptions, [BaseIntent]);
    const record = readModel.rows[0]?.outboxRecord as NotificationLocalOutboxRecord;

    expect(
      SocialAlertReportLocalOutboxBridgeReadModelSchema.safeParse({
        ...readModel,
        providerDeliveryRuntimeClaimed: true,
      }).success
    ).toBe(false);
    expect(NotificationLocalOutboxRecordSchema.safeParse({ ...record, providerDeliveryObserved: true }).success).toBe(
      false
    );
    expect(() =>
      parseSocialAlertReportLocalOutboxJsonl(`${JSON.stringify({ ...record, providerDeliveryAttempted: true })}\n`)
    ).toThrow();
  });
});

function accountApprovalIntent() {
  return {
    ...BaseIntent,
    alertReportIntentId: 'social-alert-report-account-approval-outbox',
    intentKind: SocialAlertReportIntentKind.AccountApprovalNeeded,
    priority: 'attention',
    severity: 'warning',
    notificationReasonCode: SocialAlertReportReasonCode.AccountApproval,
    providerChannelPreference: 'email',
    parentTitleToken: SocialAlertReportParentCopyToken.ApprovalTitle,
    parentBodyToken: SocialAlertReportParentCopyToken.ApprovalBody,
    parentActionRef: BridgeOptions.parentAction,
    localOutboxRecordRef: 'local-outbox-record-social-alert-report-account-approval',
  } as const;
}

function manualRequiredIntent() {
  return {
    ...BaseIntent,
    alertReportIntentId: 'social-alert-report-manual-required-outbox',
    intentKind: SocialAlertReportIntentKind.ManualRequired,
    intentStatus: SocialAlertReportIntentStatus.ManualRequired,
    priority: 'attention',
    severity: 'warning',
    notificationReasonCode: SocialAlertReportReasonCode.ManualRequired,
    providerChannelPreference: 'in-app',
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
    alertReportIntentId: 'social-alert-report-unavailable-outbox',
    intentKind: SocialAlertReportIntentKind.CapabilityUnavailable,
    intentStatus: SocialAlertReportIntentStatus.Unavailable,
    priority: 'info',
    severity: 'info',
    notificationReasonCode: SocialAlertReportReasonCode.CapabilityUnavailable,
    providerChannelPreference: 'in-app',
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
