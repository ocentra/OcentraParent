import { describe, expect, it } from 'vitest';
import { AppGameChildUxCopyToken, AppGameChildUxTargetKind } from '../src/app-game-child-facing-ux-rules';
import {
  AppGameNotificationDeliveryClaimState,
  AppGameNotificationIntentKind,
  AppGameNotificationIntentStatus,
  AppGameNotificationParentCopyToken,
  AppGameNotificationPayloadField,
  AppGameNotificationPriority,
  AppGameNotificationReasonCode,
} from '../src/app-game-notification-intent';
import {
  buildAppGameNotificationLocalOutboxBridgeReadModel,
  type AppGameNotificationLocalOutboxBridgeReadModel,
} from '../src/app-game-notification-local-outbox-bridge';
import {
  AppGameNotificationSchedulerBridgeReadModelSchema,
  AppGameNotificationSchedulerBridgeStatus,
  buildAppGameNotificationSchedulerBridgeReadModel,
  parseAppGameNotificationSchedulerJsonl,
  serializeAppGameNotificationSchedulerJsonl,
} from '../src/app-game-notification-scheduler-bridge';
import { NotificationLocalOutboxSchedulerRecordSchema } from '../src/notification-local-outbox-scheduler-proof';
import {
  ParentActorRole,
  ParentContractSchemaVersion,
  ParentEvidenceReferenceKind,
  ParentPlatform,
} from '../src/reference-primitives';

const Timestamp = '2026-06-05T01:02:00Z';
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
  family: { familyId: 'family-app-game-scheduler-bridge' },
  parentAction: {
    actionReferenceId: 'parent-action-app-game-scheduler-bridge',
    actor: { actorId: 'parent-app-game-scheduler-bridge', role: ParentActorRole.Parent },
    policyVersion: 'policy-app-game-notification-scheduler-v1',
    createdAt: Timestamp,
  },
  generatedAt: Timestamp,
  bridgeId: 'app-game-notification-local-outbox-bridge-for-scheduler-proof',
  outboxRootRef: 'parent-owned-app-game-local-outbox-root-for-scheduler',
  outboxFileRef: 'parent-owned-app-game-local-outbox-jsonl-for-scheduler',
  localDataPathRef: 'parent-owned-app-game-local-outbox-data-path-for-scheduler',
} as const;
const SchedulerOptions = {
  generatedAt: Timestamp,
  schedulerBridgeId: 'app-game-notification-scheduler-bridge-proof',
  schedulerArtifactRootRef: 'parent-owned-app-game-notification-scheduler-root',
  schedulerArtifactRef: 'parent-owned-app-game-notification-scheduler-jsonl-ref',
  schedulerNowAt: Timestamp,
} as const;
const BaseIntent = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  notificationIntentId: 'notification-intent-time-limit-scheduler',
  intentKind: AppGameNotificationIntentKind.TimeLimitReached,
  intentStatus: AppGameNotificationIntentStatus.LocalOutboxEligible,
  priority: AppGameNotificationPriority.Urgent,
  device: {
    deviceId: 'device-app-game-scheduler-bridge',
    childProfileId: 'child-app-game-scheduler-bridge',
    label: 'Study PC',
    platform: ParentPlatform.Windows,
  },
  targetKind: AppGameChildUxTargetKind.NativeGame,
  targetRef: 'target-native-game-scheduler-bridge',
  notificationReasonCode: AppGameNotificationReasonCode.TimeLimit,
  providerChannelPreference: 'in-app',
  parentTitleToken: AppGameNotificationParentCopyToken.TimeLimitTitle,
  parentBodyToken: AppGameNotificationParentCopyToken.TimeLimitBody,
  parentActionToken: AppGameNotificationParentCopyToken.OpenParentReviewAction,
  childTitleToken: AppGameChildUxCopyToken.LimitReachedTitle,
  childBodyToken: AppGameChildUxCopyToken.LimitReachedBody,
  notificationRuleRef: 'notification-rule-app-game-time-limit-scheduler',
  notificationStatusRef: 'notification-status-app-game-time-limit-scheduler',
  policyRefs: ['policy-ref-app-game-time-limit-scheduler'],
  auditRefs: ['audit-ref-app-game-time-limit-scheduler'],
  evidenceReferences: [
    {
      evidenceReferenceId: 'evidence-ref-app-game-time-limit-scheduler',
      kind: ParentEvidenceReferenceKind.PolicyDecision,
      observedAt: Timestamp,
    },
  ],
  childReasonReferences: [],
  childStatusReferences: ['child-status-app-game-time-limit-scheduler'],
  approvalActionRef: null,
  timeBudgetDecisionRef: 'time-budget-decision-app-game-scheduler',
  unknownCandidateRef: null,
  localOutboxRecordRef: 'local-outbox-record-app-game-time-limit-scheduler',
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

describe('app/game notification scheduler bridge', () => {
  it('schedules only linked app/game local outbox records through existing scheduler records', () => {
    const readModel = buildAppGameNotificationSchedulerBridgeReadModel(SchedulerOptions, sourceReadModel());
    const records = parseAppGameNotificationSchedulerJsonl(serializeAppGameNotificationSchedulerJsonl(readModel));

    expect(readModel.scheduledRecordCount).toBe(2);
    expect(readModel.unscheduledManualRequiredCount).toBe(1);
    expect(readModel.unscheduledUnavailableCount).toBe(1);
    expect(records.map((record) => record.sourceEntryId)).toEqual([
      'local-outbox-record-app-game-time-limit-scheduler',
      'local-outbox-record-app-game-suspicious-unknown-scheduler',
    ]);
    expect(records.map((record) => record.schedulerState)).toEqual(['due-local', 'due-local']);
    expect(records.map((record) => record.nextAttemptAt)).toEqual([Timestamp, Timestamp]);
    expect(records.map((record) => record.providerDeliveryAttempted)).toEqual([false, false]);
  });

  it('keeps manual-required and unavailable app/game rows out of scheduler JSONL', () => {
    const readModel = buildAppGameNotificationSchedulerBridgeReadModel(SchedulerOptions, sourceReadModel());
    const unscheduledRows = readModel.rows.filter(
      (row) => row.status !== AppGameNotificationSchedulerBridgeStatus.ScheduledLocal
    );

    expect(unscheduledRows.map((row) => row.status)).toEqual([
      AppGameNotificationSchedulerBridgeStatus.ManualRequired,
      AppGameNotificationSchedulerBridgeStatus.Unavailable,
    ]);
    expect(unscheduledRows.map((row) => row.schedulerRecord)).toEqual([null, null]);
    expect(unscheduledRows.map((row) => row.blockedReasonRefs)).toEqual([
      ['provider preference setup before app game notification can be scheduled'],
      ['local evidence and policy readiness before unavailable notification can be scheduled'],
    ]);
  });

  it('rejects scheduler runtime and provider overclaims at the app/game bridge boundary', () => {
    const readModel = buildAppGameNotificationSchedulerBridgeReadModel(SchedulerOptions, sourceReadModel());
    const record = readModel.rows[0]?.schedulerRecord;
    if (record === null || record === undefined) {
      throw new Error('expected scheduled record');
    }

    expect(
      AppGameNotificationSchedulerBridgeReadModelSchema.safeParse({
        ...readModel,
        retryExecutionRuntimeClaimed: true,
      }).success
    ).toBe(false);
    expect(
      NotificationLocalOutboxSchedulerRecordSchema.safeParse({ ...record, providerDeliveryObserved: true }).success
    ).toBe(false);
    expect(() =>
      parseAppGameNotificationSchedulerJsonl(`${JSON.stringify({ ...record, rawUrlOrTitleIncluded: true })}\n`)
    ).toThrow();
  });
});

function sourceReadModel(): AppGameNotificationLocalOutboxBridgeReadModel {
  return buildAppGameNotificationLocalOutboxBridgeReadModel(BridgeOptions, [
    BaseIntent,
    suspiciousUnknownIntent(),
    manualRequiredIntent(),
    unavailableIntent(),
  ]);
}

function suspiciousUnknownIntent() {
  return {
    ...BaseIntent,
    notificationIntentId: 'notification-intent-suspicious-unknown-scheduler',
    intentKind: AppGameNotificationIntentKind.SuspiciousUnknown,
    priority: AppGameNotificationPriority.Attention,
    targetKind: AppGameChildUxTargetKind.UnknownApp,
    targetRef: 'target-unknown-app-scheduler-bridge',
    notificationReasonCode: AppGameNotificationReasonCode.SuspiciousUnknown,
    providerChannelPreference: 'email',
    parentTitleToken: AppGameNotificationParentCopyToken.SuspiciousUnknownTitle,
    parentBodyToken: AppGameNotificationParentCopyToken.SuspiciousUnknownBody,
    childTitleToken: AppGameChildUxCopyToken.NewAppTitle,
    childBodyToken: AppGameChildUxCopyToken.NewAppBody,
    localOutboxRecordRef: 'local-outbox-record-app-game-suspicious-unknown-scheduler',
    timeBudgetDecisionRef: null,
    unknownCandidateRef: 'unknown-app-candidate-scheduler-bridge',
  } as const;
}

function manualRequiredIntent() {
  return {
    ...BaseIntent,
    notificationIntentId: 'notification-intent-manual-required-scheduler',
    intentKind: AppGameNotificationIntentKind.ManualRequired,
    intentStatus: AppGameNotificationIntentStatus.ManualRequired,
    priority: AppGameNotificationPriority.Attention,
    notificationReasonCode: AppGameNotificationReasonCode.ManualReviewRequired,
    parentTitleToken: AppGameNotificationParentCopyToken.ManualRequiredTitle,
    parentBodyToken: AppGameNotificationParentCopyToken.ManualRequiredBody,
    parentActionToken: AppGameNotificationParentCopyToken.ReviewManuallyAction,
    childTitleToken: AppGameChildUxCopyToken.ManualRequiredTitle,
    childBodyToken: AppGameChildUxCopyToken.ManualRequiredBody,
    localOutboxRecordRef: null,
    timeBudgetDecisionRef: null,
    manualProofRequirements: ['provider preference setup before app game notification can be scheduled'],
    deliveryClaimState: AppGameNotificationDeliveryClaimState.ManualRequired,
  } as const;
}

function unavailableIntent() {
  return {
    ...BaseIntent,
    notificationIntentId: 'notification-intent-unavailable-scheduler',
    intentKind: AppGameNotificationIntentKind.CapabilityUnavailable,
    intentStatus: AppGameNotificationIntentStatus.Unavailable,
    priority: AppGameNotificationPriority.Info,
    notificationReasonCode: AppGameNotificationReasonCode.CapabilityUnavailable,
    parentTitleToken: AppGameNotificationParentCopyToken.UnavailableTitle,
    parentBodyToken: AppGameNotificationParentCopyToken.UnavailableBody,
    parentActionToken: AppGameNotificationParentCopyToken.ReviewManuallyAction,
    childTitleToken: AppGameChildUxCopyToken.UnavailableTitle,
    childBodyToken: AppGameChildUxCopyToken.UnavailableBody,
    localOutboxRecordRef: null,
    timeBudgetDecisionRef: null,
    manualProofRequirements: ['local evidence and policy readiness before unavailable notification can be scheduled'],
    deliveryClaimState: AppGameNotificationDeliveryClaimState.ManualRequired,
  } as const;
}
