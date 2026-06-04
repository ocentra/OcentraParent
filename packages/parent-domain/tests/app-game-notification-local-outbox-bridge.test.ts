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
import {
  AppGameNotificationLocalOutboxBridgeProofSchema,
  buildAppGameNotificationLocalOutboxBridgeProof,
  summarizeAppGameNotificationLocalOutboxBridgeChannels,
  summarizeAppGameNotificationLocalOutboxBridgeReasons,
} from '../src/app-game-notification-local-outbox-bridge';
import {
  ParentActorRole,
  ParentContractSchemaVersion,
  ParentEvidenceReferenceKind,
  ParentPlatform,
} from '../src/reference-primitives';

const Timestamp = '2026-06-04T19:04:00Z';
const PolicyVersion = 'policy-app-game-notification-outbox-v1';

const Family = {
  familyId: 'family-app-game-notification-outbox',
} as const;

const ChildDevice = {
  deviceId: 'device-app-game-notification-outbox',
  childProfileId: 'child-app-game-notification-outbox',
  label: 'Study PC',
  platform: ParentPlatform.Windows,
} as const;

const ParentAction = {
  actionReferenceId: 'parent-action-app-game-notification-outbox',
  actor: {
    actorId: 'parent-app-game-notification-outbox',
    role: ParentActorRole.Parent,
  },
  policyVersion: PolicyVersion,
  createdAt: Timestamp,
} as const;

const EvidenceReference = {
  evidenceReferenceId: 'evidence-app-game-notification-outbox-session',
  kind: ParentEvidenceReferenceKind.PolicyDecision,
  observedAt: Timestamp,
} as const;

const ApprovalActionRef = {
  actionReferenceId: 'approval-action-app-game-notification-outbox',
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
  notificationIntentId: 'notification-intent-outbox-time-limit',
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
  localOutboxRecordRef: 'local-outbox-record-game-limit',
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

describe('app/game notification local outbox bridge', () => {
  bridgesEligibleIntentsToLocalOutboxRecords();
  blocksManualAndUnavailableIntents();
  rejectsUnsafeIntentAndBridgeOverclaims();
});

function bridgesEligibleIntentsToLocalOutboxRecords(): void {
  it('turns local-outbox-eligible app/game notification intents into minimal queued local records', () => {
    const proof = buildAppGameNotificationLocalOutboxBridgeProof(buildBridgeInput());

    expect(proof.records).toHaveLength(3);
    expect(proof.records.map((record) => record.state)).toEqual(['queued-local', 'queued-local', 'queued-local']);
    expect(proof.bridgedIntentRefs.map((link) => link.outboxEntryRef)).toEqual([
      'local-outbox-record-game-limit',
      'local-outbox-record-approval-request',
      'local-outbox-record-suspicious-unknown',
    ]);
    expect(summarizeAppGameNotificationLocalOutboxBridgeReasons(proof)).toEqual({
      'policy-violation': 1,
      'parent-request': 1,
      'suspicious-unknown': 1,
      'device-offline': 0,
      'sync-failure': 0,
      'provider-failure': 0,
    });
    expect(summarizeAppGameNotificationLocalOutboxBridgeChannels(proof)).toEqual({
      push: 1,
      email: 1,
      sms: 0,
      whatsapp: 0,
      'in-app': 1,
    });
    expect(proof.records.every((record) => record.envelope.sensitiveDetailMinimized)).toBe(true);
    expect(proof.records.every((record) => record.providerDeliveryAttempted === false)).toBe(true);
    expect(proof.adapterDispatchClaimed).toBe(false);
  });
}

function blocksManualAndUnavailableIntents(): void {
  it('keeps manual-required and unavailable notification intents out of local outbox records', () => {
    const proof = buildAppGameNotificationLocalOutboxBridgeProof(buildBridgeInput());

    expect(proof.blockedIntentRefs).toEqual([
      {
        notificationIntentRef: 'notification-intent-manual-required',
        intentKind: AppGameNotificationIntentKind.ManualRequired,
        intentStatus: AppGameNotificationIntentStatus.ManualRequired,
        deliveryClaimState: AppGameNotificationDeliveryClaimState.ManualRequired,
        blockReason: 'manual-required-no-local-outbox',
        manualProofRequirements: ['parent manual review required'],
      },
      {
        notificationIntentRef: 'notification-intent-unavailable',
        intentKind: AppGameNotificationIntentKind.CapabilityUnavailable,
        intentStatus: AppGameNotificationIntentStatus.Unavailable,
        deliveryClaimState: AppGameNotificationDeliveryClaimState.ManualRequired,
        blockReason: 'capability-unavailable-no-local-outbox',
        manualProofRequirements: ['provider or capability availability proof required'],
      },
    ]);
    expect(proof.records.some((record) => record.entryId === 'notification-intent-manual-required')).toBe(false);
  });
}

function rejectsUnsafeIntentAndBridgeOverclaims(): void {
  it('rejects provider claims false local-outbox claims and incoherent bridge records', () => {
    expect(() =>
      buildAppGameNotificationLocalOutboxBridgeProof(
        buildBridgeInput([{ ...TimeLimitIntent, providerDeliveryAttempted: true }])
      )
    ).toThrow();
    expect(() =>
      buildAppGameNotificationLocalOutboxBridgeProof(
        buildBridgeInput([{ ...manualIntent(), localOutboxRecordRef: 'false-local-outbox-record' }])
      )
    ).toThrow();

    const proof = buildAppGameNotificationLocalOutboxBridgeProof(buildBridgeInput());
    expect(
      AppGameNotificationLocalOutboxBridgeProofSchema.safeParse({ ...proof, adapterDispatchClaimed: true }).success
    ).toBe(false);
    expect(
      AppGameNotificationLocalOutboxBridgeProofSchema.safeParse({
        ...proof,
        records: proof.records.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppGameNotificationLocalOutboxBridgeProofSchema.safeParse({
        ...proof,
        nonClaims: proof.nonClaims.filter((claim) => claim !== 'no-provider-delivery'),
      }).success
    ).toBe(false);
  });
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
    notificationIntentId: 'notification-intent-approval-request',
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
    localOutboxRecordRef: 'local-outbox-record-approval-request',
  } as const;
}

function suspiciousUnknownIntent() {
  return {
    ...approvalIntent(),
    notificationIntentId: 'notification-intent-suspicious-unknown',
    intentKind: AppGameNotificationIntentKind.SuspiciousUnknown,
    notificationReasonCode: AppGameNotificationReasonCode.SuspiciousUnknown,
    providerChannelPreference: 'email',
    parentTitleToken: AppGameNotificationParentCopyToken.SuspiciousUnknownTitle,
    parentBodyToken: AppGameNotificationParentCopyToken.SuspiciousUnknownBody,
    approvalActionRef: null,
    localOutboxRecordRef: 'local-outbox-record-suspicious-unknown',
  } as const;
}

function manualIntent() {
  return {
    ...TimeLimitIntent,
    notificationIntentId: 'notification-intent-manual-required',
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
    notificationIntentId: 'notification-intent-unavailable',
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
