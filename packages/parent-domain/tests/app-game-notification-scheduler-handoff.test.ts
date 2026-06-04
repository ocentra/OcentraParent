import { describe, expect, it } from 'vitest';
import { AppGameChildUxCopyToken, AppGameChildUxTargetKind } from '../src/app-game-child-facing-ux-rules';
import {
  AppGameNotificationAdapterDispatchState,
  AppGameNotificationDeliveryClaimState,
  AppGameNotificationIntentKind,
  AppGameNotificationIntentStatus,
  AppGameNotificationParentCopyToken,
  AppGameNotificationPayloadField,
  AppGameNotificationPriority,
  AppGameNotificationReasonCode,
} from '../src/app-game-notification-intent';
import { buildAppGameNotificationLocalOutboxBridgeProof } from '../src/app-game-notification-local-outbox-bridge';
import {
  AppGameNotificationSchedulerHandoffProofSchema,
  buildAppGameNotificationSchedulerHandoffProof,
  dueAppGameNotificationSchedulerHandoffRecords,
  summarizeAppGameNotificationSchedulerHandoffChannels,
  summarizeAppGameNotificationSchedulerHandoffStates,
} from '../src/app-game-notification-scheduler-handoff';
import {
  ParentActorRole,
  ParentContractSchemaVersion,
  ParentEvidenceReferenceKind,
  ParentPlatform,
} from '../src/reference-primitives';

const Timestamp = '2026-06-04T19:14:00Z';
const SchedulerNowAt = '2026-06-04T19:15:00Z';
const PolicyVersion = 'policy-app-game-notification-scheduler-v1';

const Family = {
  familyId: 'family-app-game-notification-scheduler',
} as const;

const ChildDevice = {
  deviceId: 'device-app-game-notification-scheduler',
  childProfileId: 'child-app-game-notification-scheduler',
  label: 'Study PC',
  platform: ParentPlatform.Windows,
} as const;

const ParentAction = {
  actionReferenceId: 'parent-action-app-game-notification-scheduler',
  actor: {
    actorId: 'parent-app-game-notification-scheduler',
    role: ParentActorRole.Parent,
  },
  policyVersion: PolicyVersion,
  createdAt: Timestamp,
} as const;

const EvidenceReference = {
  evidenceReferenceId: 'evidence-app-game-notification-scheduler-session',
  kind: ParentEvidenceReferenceKind.PolicyDecision,
  observedAt: Timestamp,
} as const;

const ApprovalActionRef = {
  actionReferenceId: 'approval-action-app-game-notification-scheduler',
  actor: {
    actorId: 'child-local-agent',
    role: ParentActorRole.System,
  },
  policyVersion: PolicyVersion,
  createdAt: Timestamp,
} as const;

const MinimalPayloadFields = [
  AppGameNotificationPayloadField.AlertId,
  AppGameNotificationPayloadField.FamilyDeviceScope,
  AppGameNotificationPayloadField.Severity,
  AppGameNotificationPayloadField.ReasonCode,
  AppGameNotificationPayloadField.EvidenceRef,
  AppGameNotificationPayloadField.PolicyRef,
  AppGameNotificationPayloadField.ParentActionLinkRef,
] as const;

const TimeLimitIntent = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  notificationIntentId: 'notification-intent-scheduler-time-limit',
  intentKind: AppGameNotificationIntentKind.TimeLimitReached,
  intentStatus: AppGameNotificationIntentStatus.LocalOutboxEligible,
  priority: AppGameNotificationPriority.Urgent,
  device: ChildDevice,
  targetKind: AppGameChildUxTargetKind.NativeGame,
  targetRef: 'target-native-game-claim',
  notificationReasonCode: AppGameNotificationReasonCode.TimeLimit,
  providerChannelPreference: 'push',
  parentTitleToken: AppGameNotificationParentCopyToken.TimeLimitTitle,
  parentBodyToken: AppGameNotificationParentCopyToken.TimeLimitBody,
  parentActionToken: AppGameNotificationParentCopyToken.OpenParentReviewAction,
  childTitleToken: AppGameChildUxCopyToken.LimitReachedTitle,
  childBodyToken: AppGameChildUxCopyToken.LimitReachedBody,
  notificationRuleRef: 'notification-rule-app-game-time-limit',
  notificationStatusRef: 'notification-status-app-game-time-limit',
  policyRefs: ['policy-ref-game-limit'],
  auditRefs: ['audit-ref-game-limit-notification'],
  evidenceReferences: [EvidenceReference],
  childReasonReferences: [],
  childStatusReferences: ['child-status-time-limit-reached'],
  approvalActionRef: null,
  timeBudgetDecisionRef: 'time-budget-decision-game-limit',
  unknownCandidateRef: null,
  localOutboxRecordRef: 'local-outbox-record-scheduler-game-limit',
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
  adapterDispatchState: AppGameNotificationAdapterDispatchState.NotDispatched,
  adapterActionClaimed: false,
  createdAt: Timestamp,
} as const;

describe('app/game notification scheduler handoff', () => {
  schedulesBridgeRecordsAsDueLocalRows();
  keepsBlockedIntentsOutOfSchedulerRows();
  rejectsSchedulerHandoffOverclaimsAndBrokenMappings();
});

function schedulesBridgeRecordsAsDueLocalRows(): void {
  it('turns app/game local outbox bridge rows into due-local scheduler rows', () => {
    const proof = buildAppGameNotificationSchedulerHandoffProof(buildHandoffInput());

    expect(proof.records).toHaveLength(3);
    expect(proof.records.map((record) => record.schedulerState)).toEqual(['due-local', 'due-local', 'due-local']);
    expect(proof.records.map((record) => record.nextAttemptAt)).toEqual([
      SchedulerNowAt,
      SchedulerNowAt,
      SchedulerNowAt,
    ]);
    expect(proof.records.map((record) => record.sourceEntryId)).toEqual([
      'local-outbox-record-scheduler-game-limit',
      'local-outbox-record-scheduler-approval-request',
      'local-outbox-record-scheduler-suspicious-unknown',
    ]);
    expect(proof.scheduledIntentRefs.map((link) => link.schedulerEntryRef)).toEqual([
      'app-game-scheduler-due-local-outbox-record-scheduler-game-limit',
      'app-game-scheduler-due-local-outbox-record-scheduler-approval-request',
      'app-game-scheduler-due-local-outbox-record-scheduler-suspicious-unknown',
    ]);
    expect(dueAppGameNotificationSchedulerHandoffRecords(proof)).toHaveLength(3);
    expect(summarizeAppGameNotificationSchedulerHandoffStates(proof)).toEqual({
      'due-local': 3,
      'held-quiet-hours': 0,
      'retry-window-scheduled': 0,
      'dead-letter-review': 0,
      'receipt-required': 0,
      'manual-required': 0,
    });
    expect(summarizeAppGameNotificationSchedulerHandoffChannels(proof)).toEqual({
      push: 1,
      email: 1,
      sms: 0,
      whatsapp: 0,
      'in-app': 1,
    });
    expect(proof.records.every((record) => record.parentOwnedArtifactWritten)).toBe(true);
    expect(proof.records.every((record) => record.providerDeliveryAttempted === false)).toBe(true);
    expect(proof.retryExecutionRuntimeClaimed).toBe(false);
    expect(proof.durableServicePersistenceClaimed).toBe(false);
    expect(proof.childDeviceDeliveryClaimed).toBe(false);
    expect(proof.broadAppBlockingClaimed).toBe(false);
  });
}

function keepsBlockedIntentsOutOfSchedulerRows(): void {
  it('preserves manual-required and unavailable bridge blocks without scheduling them', () => {
    const proof = buildAppGameNotificationSchedulerHandoffProof(buildHandoffInput());

    expect(proof.blockedIntentRefs).toEqual([
      {
        notificationIntentRef: 'notification-intent-scheduler-manual-required',
        blockReason: 'manual-required-no-local-outbox',
        manualProofRequirements: ['parent manual review required'],
      },
      {
        notificationIntentRef: 'notification-intent-scheduler-unavailable',
        blockReason: 'capability-unavailable-no-local-outbox',
        manualProofRequirements: ['provider or capability availability proof required'],
      },
    ]);
    expect(
      proof.scheduledIntentRefs.some(
        (link) => link.notificationIntentRef === 'notification-intent-scheduler-manual-required'
      )
    ).toBe(false);
  });
}

function rejectsSchedulerHandoffOverclaimsAndBrokenMappings(): void {
  it('rejects provider claims missing rows and incoherent scheduler links', () => {
    const proof = buildAppGameNotificationSchedulerHandoffProof(buildHandoffInput());
    const brokenBridgeProof = {
      ...buildAppGameNotificationLocalOutboxBridgeProof(buildBridgeInput()),
      records: proof.sourceLocalOutboxRecords.slice(1),
    };

    expect(
      AppGameNotificationSchedulerHandoffProofSchema.safeParse({
        ...proof,
        providerDeliveryRuntimeClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameNotificationSchedulerHandoffProofSchema.safeParse({
        ...proof,
        records: proof.records.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppGameNotificationSchedulerHandoffProofSchema.safeParse({
        ...proof,
        nonClaims: proof.nonClaims.filter((claim) => claim !== 'no-provider-credentials'),
      }).success
    ).toBe(false);
    expect(
      AppGameNotificationSchedulerHandoffProofSchema.safeParse({
        ...proof,
        scheduledIntentRefs: [
          { ...proof.scheduledIntentRefs[0], schedulerEntryRef: 'wrong-scheduler-entry-ref' },
          ...proof.scheduledIntentRefs.slice(1),
        ],
      }).success
    ).toBe(false);
    expect(() =>
      buildAppGameNotificationSchedulerHandoffProof({
        ...buildHandoffInput(),
        bridgeProof: brokenBridgeProof,
      })
    ).toThrow();
  });
}

function buildHandoffInput() {
  return {
    generatedAt: Timestamp,
    schedulerNowAt: SchedulerNowAt,
    schedulerArtifactRootRef: 'parent-owned-app-game-notification-scheduler-root',
    bridgeProof: buildAppGameNotificationLocalOutboxBridgeProof(buildBridgeInput()),
  } as const;
}

function buildBridgeInput(intents = validIntents()) {
  return {
    generatedAt: Timestamp,
    family: Family,
    parentAction: ParentAction,
    sourceIntentReadModelRef: 'app-game-notification-intent-contract-proof',
    localOutboxReadModelRef: 'notification-local-outbox-adapter-proof',
    outboxRootRef: 'parent-owned-local-app-game-notification-outbox-root',
    outboxFileRef: 'parent-owned-app-game-notification-outbox-jsonl-ref',
    localDataPathRef: 'parent-owned-app-game-notification-local-data-path-ref',
    intents,
  } as const;
}

function validIntents() {
  return [TimeLimitIntent, approvalIntent(), suspiciousUnknownIntent(), manualIntent(), unavailableIntent()] as const;
}

function approvalIntent() {
  return {
    ...TimeLimitIntent,
    notificationIntentId: 'notification-intent-scheduler-approval-request',
    intentKind: AppGameNotificationIntentKind.ApprovalRequested,
    priority: AppGameNotificationPriority.Attention,
    targetKind: AppGameChildUxTargetKind.UnknownApp,
    targetRef: 'target-unknown-app',
    notificationReasonCode: AppGameNotificationReasonCode.ApprovalRequest,
    providerChannelPreference: 'in-app',
    parentTitleToken: AppGameNotificationParentCopyToken.ApprovalTitle,
    parentBodyToken: AppGameNotificationParentCopyToken.ApprovalBody,
    childTitleToken: AppGameChildUxCopyToken.NewAppTitle,
    childBodyToken: AppGameChildUxCopyToken.NewAppBody,
    childReasonReferences: ['child-reason-new-app-request'],
    childStatusReferences: ['child-status-new-app-request'],
    approvalActionRef: ApprovalActionRef,
    timeBudgetDecisionRef: null,
    unknownCandidateRef: 'unknown-app-candidate-request',
    localOutboxRecordRef: 'local-outbox-record-scheduler-approval-request',
  } as const;
}

function suspiciousUnknownIntent() {
  return {
    ...approvalIntent(),
    notificationIntentId: 'notification-intent-scheduler-suspicious-unknown',
    intentKind: AppGameNotificationIntentKind.SuspiciousUnknown,
    notificationReasonCode: AppGameNotificationReasonCode.SuspiciousUnknown,
    providerChannelPreference: 'email',
    parentTitleToken: AppGameNotificationParentCopyToken.SuspiciousUnknownTitle,
    parentBodyToken: AppGameNotificationParentCopyToken.SuspiciousUnknownBody,
    approvalActionRef: null,
    localOutboxRecordRef: 'local-outbox-record-scheduler-suspicious-unknown',
  } as const;
}

function manualIntent() {
  return {
    ...TimeLimitIntent,
    notificationIntentId: 'notification-intent-scheduler-manual-required',
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
    manualProofRequirements: ['parent manual review required'],
    deliveryClaimState: AppGameNotificationDeliveryClaimState.ManualRequired,
  } as const;
}

function unavailableIntent() {
  return {
    ...manualIntent(),
    notificationIntentId: 'notification-intent-scheduler-unavailable',
    intentKind: AppGameNotificationIntentKind.CapabilityUnavailable,
    intentStatus: AppGameNotificationIntentStatus.Unavailable,
    notificationReasonCode: AppGameNotificationReasonCode.CapabilityUnavailable,
    parentTitleToken: AppGameNotificationParentCopyToken.UnavailableTitle,
    parentBodyToken: AppGameNotificationParentCopyToken.UnavailableBody,
    childTitleToken: AppGameChildUxCopyToken.UnavailableTitle,
    childBodyToken: AppGameChildUxCopyToken.UnavailableBody,
    manualProofRequirements: ['provider or capability availability proof required'],
  } as const;
}
