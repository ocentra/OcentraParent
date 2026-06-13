import { describe, expect, it } from 'vitest';
import { AppGameChildUxCopyToken, AppGameChildUxTargetKind } from '../../src/app-game-child-facing-ux-rules';
import {
  AppGameNotificationDeliveryClaimState,
  AppGameNotificationIntentKind,
  AppGameNotificationIntentStatus,
  AppGameNotificationParentCopyToken,
  AppGameNotificationPayloadField,
  AppGameNotificationPriority,
  AppGameNotificationReasonCode,
} from '../../src/app-game-notification-intent';
import {
  AppGameNotificationLocalOutboxBridgeReadModelSchema,
  AppGameNotificationLocalOutboxBridgeStatus,
  buildAppGameNotificationLocalOutboxBridgeReadModel,
  parseAppGameNotificationLocalOutboxJsonl,
  serializeAppGameNotificationLocalOutboxJsonl,
} from '../../src/app-game-notification-local-outbox-bridge';
import {
  NotificationLocalOutboxRecordSchema,
  type NotificationLocalOutboxRecord,
} from '@ocentra-parent/notification-domain/notification-local-outbox-adapter-proof';
import {
  ParentActorRole,
  ParentContractSchemaVersion,
  ParentEvidenceReferenceKind,
  ParentPlatform,
} from '@ocentra-parent/family-domain/reference-primitives';

const Timestamp = '2026-06-05T00:24:00Z';
const PolicyVersion = 'policy-app-game-notification-outbox-v1';
const MinimalPayloadFields = [
  AppGameNotificationPayloadField.AlertId,
  AppGameNotificationPayloadField.FamilyDeviceScope,
  AppGameNotificationPayloadField.Severity,
  AppGameNotificationPayloadField.ReasonCode,
  AppGameNotificationPayloadField.EvidenceRef,
  AppGameNotificationPayloadField.PolicyRef,
  AppGameNotificationPayloadField.ParentActionLinkRef,
] as const;

const BridgeOptions = {
  family: { familyId: 'family-app-game-outbox-bridge' },
  parentAction: {
    actionReferenceId: 'parent-action-app-game-outbox-bridge',
    actor: { actorId: 'parent-app-game-outbox-bridge', role: ParentActorRole.Parent },
    policyVersion: PolicyVersion,
    createdAt: Timestamp,
  },
  generatedAt: Timestamp,
  bridgeId: 'app-game-notification-local-outbox-bridge-proof',
  outboxRootRef: 'parent-owned-app-game-local-outbox-root',
  outboxFileRef: 'parent-owned-app-game-local-outbox-jsonl-ref',
  localDataPathRef: 'parent-owned-app-game-local-outbox-data-path-ref',
} as const;

const BaseIntent = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  notificationIntentId: 'notification-intent-time-limit-outbox',
  intentKind: AppGameNotificationIntentKind.TimeLimitReached,
  intentStatus: AppGameNotificationIntentStatus.LocalOutboxEligible,
  priority: AppGameNotificationPriority.Urgent,
  device: {
    deviceId: 'device-app-game-outbox-bridge',
    childProfileId: 'child-app-game-outbox-bridge',
    label: 'Study PC',
    platform: ParentPlatform.Windows,
  },
  targetKind: AppGameChildUxTargetKind.NativeGame,
  targetRef: 'target-native-game-outbox-bridge',
  notificationReasonCode: AppGameNotificationReasonCode.TimeLimit,
  providerChannelPreference: 'in-app',
  parentTitleToken: AppGameNotificationParentCopyToken.TimeLimitTitle,
  parentBodyToken: AppGameNotificationParentCopyToken.TimeLimitBody,
  parentActionToken: AppGameNotificationParentCopyToken.OpenParentReviewAction,
  childTitleToken: AppGameChildUxCopyToken.LimitReachedTitle,
  childBodyToken: AppGameChildUxCopyToken.LimitReachedBody,
  notificationRuleRef: 'notification-rule-app-game-time-limit-outbox',
  notificationStatusRef: 'notification-status-app-game-time-limit-outbox',
  policyRefs: ['policy-ref-app-game-time-limit-outbox'],
  auditRefs: ['audit-ref-app-game-time-limit-outbox'],
  evidenceReferences: [
    {
      evidenceReferenceId: 'evidence-ref-app-game-time-limit-outbox',
      kind: ParentEvidenceReferenceKind.PolicyDecision,
      observedAt: Timestamp,
    },
  ],
  childReasonReferences: [],
  childStatusReferences: ['child-status-app-game-time-limit-outbox'],
  approvalActionRef: null,
  timeBudgetDecisionRef: 'time-budget-decision-app-game-outbox',
  unknownCandidateRef: null,
  localOutboxRecordRef: 'local-outbox-record-app-game-time-limit',
  providerAttemptRefs: [],
  providerReceiptRefs: [],
  manualProofRequirements: [],
  minimalPayloadFields: MinimalPayloadFields,
  deliveryClaimState: AppGameNotificationDeliveryClaimState.LocalOutboxOnly,
  rawChildEvidenceIncluded: false,
  rawUrlOrTitleIncluded: false,
  rawMessageTextIncluded: false,
  screenshotOrReportIncluded: false,
  providerDeliveryAttempted: false,
  providerDeliveryObserved: false,
  providerReceiptIngested: false,
  cloudRoutingClaimed: false,
  parentNotificationUiClaimed: false,
  adapterDispatchState: 'not-dispatched',
  adapterActionClaimed: false,
  createdAt: Timestamp,
} as const;

describe('app/game notification local outbox bridge', () => {
  it('writes and rereads local-outbox records only for eligible app/game notification intents', () => {
    const readModel = buildAppGameNotificationLocalOutboxBridgeReadModel(BridgeOptions, [
      BaseIntent,
      suspiciousUnknownIntent(),
      manualRequiredIntent(),
      unavailableIntent(),
    ]);
    const jsonl = serializeAppGameNotificationLocalOutboxJsonl(readModel);
    const records = parseAppGameNotificationLocalOutboxJsonl(jsonl);

    expect(readModel.linkedRecordCount).toBe(2);
    expect(readModel.manualRequiredCount).toBe(1);
    expect(readModel.unavailableCount).toBe(1);
    expect(records.map((record) => record.entryId)).toEqual([
      'local-outbox-record-app-game-time-limit',
      'local-outbox-record-app-game-suspicious-unknown',
    ]);
    expect(records.map((record) => record.envelope.reasonCode)).toEqual(['policy-violation', 'suspicious-unknown']);
    expect(records.map((record) => record.providerDeliveryAttempted)).toEqual([false, false]);
    expect(records.map((record) => record.envelope.rawChildEvidenceIncluded)).toEqual([false, false]);
  });

  it('keeps manual-required and unavailable app/game notification intents out of queued JSONL records', () => {
    const readModel = buildAppGameNotificationLocalOutboxBridgeReadModel(BridgeOptions, [
      manualRequiredIntent(),
      unavailableIntent(),
    ]);
    const statuses = readModel.rows.map((row) => row.status);
    const blockedRefs = readModel.rows.map((row) => row.blockedReasonRefs);

    expect(statuses).toEqual([
      AppGameNotificationLocalOutboxBridgeStatus.ManualRequired,
      AppGameNotificationLocalOutboxBridgeStatus.Unavailable,
    ]);
    expect(blockedRefs).toEqual([
      ['provider preference setup before app game notification can be queued'],
      ['local evidence and policy readiness before unavailable notification can be queued'],
    ]);
    expect(serializeAppGameNotificationLocalOutboxJsonl(readModel)).toBe('\n');
  });

  it('rejects provider delivery overclaims and unsafe JSONL records at the bridge boundary', () => {
    const readModel = buildAppGameNotificationLocalOutboxBridgeReadModel(BridgeOptions, [BaseIntent]);
    const record = readModel.rows[0]?.outboxRecord as NotificationLocalOutboxRecord;

    expect(
      AppGameNotificationLocalOutboxBridgeReadModelSchema.safeParse({
        ...readModel,
        providerDeliveryRuntimeClaimed: true,
      }).success
    ).toBe(false);
    expect(NotificationLocalOutboxRecordSchema.safeParse({ ...record, providerDeliveryObserved: true }).success).toBe(
      false
    );
    expect(() =>
      parseAppGameNotificationLocalOutboxJsonl(`${JSON.stringify({ ...record, providerDeliveryAttempted: true })}\n`)
    ).toThrow();
  });
});

function suspiciousUnknownIntent() {
  return {
    ...BaseIntent,
    notificationIntentId: 'notification-intent-suspicious-unknown-outbox',
    intentKind: AppGameNotificationIntentKind.SuspiciousUnknown,
    priority: AppGameNotificationPriority.Attention,
    targetKind: AppGameChildUxTargetKind.UnknownApp,
    targetRef: 'target-unknown-app-outbox-bridge',
    notificationReasonCode: AppGameNotificationReasonCode.SuspiciousUnknown,
    providerChannelPreference: 'email',
    parentTitleToken: AppGameNotificationParentCopyToken.SuspiciousUnknownTitle,
    parentBodyToken: AppGameNotificationParentCopyToken.SuspiciousUnknownBody,
    childTitleToken: AppGameChildUxCopyToken.NewAppTitle,
    childBodyToken: AppGameChildUxCopyToken.NewAppBody,
    notificationRuleRef: 'notification-rule-app-game-suspicious-unknown-outbox',
    notificationStatusRef: 'notification-status-app-game-suspicious-unknown-outbox',
    policyRefs: ['policy-ref-app-game-suspicious-unknown-outbox'],
    auditRefs: ['audit-ref-app-game-suspicious-unknown-outbox'],
    childStatusReferences: ['child-status-app-game-suspicious-unknown-outbox'],
    timeBudgetDecisionRef: null,
    unknownCandidateRef: 'unknown-app-candidate-outbox-bridge',
    localOutboxRecordRef: 'local-outbox-record-app-game-suspicious-unknown',
  } as const;
}

function manualRequiredIntent() {
  return {
    ...BaseIntent,
    notificationIntentId: 'notification-intent-manual-required-outbox',
    intentKind: AppGameNotificationIntentKind.ManualRequired,
    intentStatus: AppGameNotificationIntentStatus.ManualRequired,
    priority: AppGameNotificationPriority.Attention,
    notificationReasonCode: AppGameNotificationReasonCode.ManualReviewRequired,
    parentTitleToken: AppGameNotificationParentCopyToken.ManualRequiredTitle,
    parentBodyToken: AppGameNotificationParentCopyToken.ManualRequiredBody,
    parentActionToken: AppGameNotificationParentCopyToken.ReviewManuallyAction,
    childTitleToken: AppGameChildUxCopyToken.ManualRequiredTitle,
    childBodyToken: AppGameChildUxCopyToken.ManualRequiredBody,
    timeBudgetDecisionRef: null,
    localOutboxRecordRef: null,
    manualProofRequirements: ['provider preference setup before app game notification can be queued'],
    deliveryClaimState: AppGameNotificationDeliveryClaimState.ManualRequired,
  } as const;
}

function unavailableIntent() {
  return {
    ...BaseIntent,
    notificationIntentId: 'notification-intent-unavailable-outbox',
    intentKind: AppGameNotificationIntentKind.CapabilityUnavailable,
    intentStatus: AppGameNotificationIntentStatus.Unavailable,
    priority: AppGameNotificationPriority.Info,
    notificationReasonCode: AppGameNotificationReasonCode.CapabilityUnavailable,
    parentTitleToken: AppGameNotificationParentCopyToken.UnavailableTitle,
    parentBodyToken: AppGameNotificationParentCopyToken.UnavailableBody,
    parentActionToken: AppGameNotificationParentCopyToken.ReviewManuallyAction,
    childTitleToken: AppGameChildUxCopyToken.UnavailableTitle,
    childBodyToken: AppGameChildUxCopyToken.UnavailableBody,
    timeBudgetDecisionRef: null,
    localOutboxRecordRef: null,
    manualProofRequirements: ['local evidence and policy readiness before unavailable notification can be queued'],
    deliveryClaimState: AppGameNotificationDeliveryClaimState.ManualRequired,
  } as const;
}
