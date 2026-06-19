import { describe, expect, it } from 'vitest';
import {
  AppGameNotificationAdapterDispatchState,
  AppGameNotificationDeliveryClaimState,
  AppGameNotificationIntentKind,
  AppGameNotificationIntentSchema,
  AppGameNotificationIntentStatus,
  AppGameNotificationParentCopyToken,
  AppGameNotificationPayloadField,
  AppGameNotificationPriority,
  AppGameNotificationReasonCode,
} from '../../src/app-game-notification-intent';
import { AppGameChildUxCopyToken, AppGameChildUxTargetKind } from '../../src/app-game-child-facing-ux-rules';
import {
  ParentActorRole,
  ParentContractSchemaVersion,
  ParentEvidenceReferenceKind,
  ParentPlatform,
} from '@ocentra-parent/schema-domain/family-reference-primitives';

const Timestamp = '2026-06-04T17:54:00Z';
const PolicyVersion = 'policy-app-game-notification-v1';

const ChildDevice = {
  deviceId: 'device-app-game-notification',
  childProfileId: 'child-app-game-notification',
  label: 'Study PC',
  platform: ParentPlatform.Windows,
} as const;

const EvidenceReference = {
  evidenceReferenceId: 'evidence-app-game-notification-session',
  kind: ParentEvidenceReferenceKind.PolicyDecision,
  observedAt: Timestamp,
} as const;

const ApprovalActionRef = {
  actionReferenceId: 'approval-action-app-game-notification',
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

const BaseIntent = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  notificationIntentId: 'notification-intent-time-limit',
  intentKind: AppGameNotificationIntentKind.TimeLimitReached,
  intentStatus: AppGameNotificationIntentStatus.LocalOutboxEligible,
  priority: AppGameNotificationPriority.Urgent,
  device: ChildDevice,
  targetKind: AppGameChildUxTargetKind.NativeGame,
  targetRef: 'target-native-game-claim',
  notificationReasonCode: AppGameNotificationReasonCode.TimeLimit,
  providerChannelPreference: 'in-app',
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

describe('app/game notification intent contracts', () => {
  acceptsTimeLimitNotificationIntent();
  acceptsApprovalAndSuspiciousUnknownIntents();
  rejectsUnsafeRuntimeClaims();
  rejectsMismatchedRefsAndCopy();
  keepsManualAndUnavailableHonest();
});

function acceptsTimeLimitNotificationIntent() {
  it('accepts a time-limit notification intent with local-outbox-only delivery claims', () => {
    const parsed = AppGameNotificationIntentSchema.parse(BaseIntent);

    expect(parsed.intentKind).toBe(AppGameNotificationIntentKind.TimeLimitReached);
    expect(parsed.deliveryClaimState).toBe(AppGameNotificationDeliveryClaimState.LocalOutboxOnly);
    expect(parsed.providerDeliveryAttempted).toBe(false);
    expect(parsed.adapterDispatchState).toBe(AppGameNotificationAdapterDispatchState.NotDispatched);
    expect(parsed.minimalPayloadFields).toEqual(MinimalPayloadFields);
  });
}

function acceptsApprovalAndSuspiciousUnknownIntents() {
  it('accepts approval and suspicious unknown intents only when their audit refs are present', () => {
    const approval = AppGameNotificationIntentSchema.parse({
      ...BaseIntent,
      notificationIntentId: 'notification-intent-approval-request',
      intentKind: AppGameNotificationIntentKind.ApprovalRequested,
      priority: AppGameNotificationPriority.Attention,
      targetKind: AppGameChildUxTargetKind.UnknownApp,
      targetRef: 'target-unknown-app',
      notificationReasonCode: AppGameNotificationReasonCode.ApprovalRequest,
      parentTitleToken: AppGameNotificationParentCopyToken.ApprovalTitle,
      parentBodyToken: AppGameNotificationParentCopyToken.ApprovalBody,
      childTitleToken: AppGameChildUxCopyToken.NewAppTitle,
      childBodyToken: AppGameChildUxCopyToken.NewAppBody,
      childReasonReferences: ['child-reason-new-app-request'],
      childStatusReferences: ['child-status-new-app-request'],
      approvalActionRef: ApprovalActionRef,
      timeBudgetDecisionRef: null,
      unknownCandidateRef: 'unknown-app-candidate-request',
    });
    const suspiciousUnknown = AppGameNotificationIntentSchema.parse({
      ...approval,
      notificationIntentId: 'notification-intent-suspicious-unknown',
      intentKind: AppGameNotificationIntentKind.SuspiciousUnknown,
      notificationReasonCode: AppGameNotificationReasonCode.SuspiciousUnknown,
      parentTitleToken: AppGameNotificationParentCopyToken.SuspiciousUnknownTitle,
      parentBodyToken: AppGameNotificationParentCopyToken.SuspiciousUnknownBody,
      approvalActionRef: null,
    });

    expect(approval.approvalActionRef).toEqual(ApprovalActionRef);
    expect(suspiciousUnknown.unknownCandidateRef).toBe('unknown-app-candidate-request');
  });
}

function rejectsUnsafeRuntimeClaims() {
  it('rejects raw detail leakage provider delivery claims and adapter action claims', () => {
    for (const invalidIntent of [
      { ...BaseIntent, notificationIntentId: 'notification-intent-raw-child-evidence', rawChildEvidenceIncluded: true },
      { ...BaseIntent, notificationIntentId: 'notification-intent-raw-title', rawUrlOrTitleIncluded: true },
      { ...BaseIntent, notificationIntentId: 'notification-intent-provider-delivery', providerDeliveryAttempted: true },
      {
        ...BaseIntent,
        notificationIntentId: 'notification-intent-provider-receipt',
        providerReceiptRefs: ['receipt-ref'],
      },
      { ...BaseIntent, notificationIntentId: 'notification-intent-adapter-action', adapterActionClaimed: true },
    ]) {
      expect(AppGameNotificationIntentSchema.safeParse(invalidIntent).success).toBe(false);
    }
  });
}

function rejectsMismatchedRefsAndCopy() {
  it('rejects mismatched copy tokens reason codes and missing kind-specific refs', () => {
    for (const invalidIntent of [
      {
        ...BaseIntent,
        notificationIntentId: 'notification-intent-wrong-reason',
        notificationReasonCode: AppGameNotificationReasonCode.ApprovalRequest,
      },
      {
        ...BaseIntent,
        notificationIntentId: 'notification-intent-wrong-copy',
        parentTitleToken: AppGameNotificationParentCopyToken.ApprovalTitle,
      },
      {
        ...BaseIntent,
        notificationIntentId: 'notification-intent-missing-budget-ref',
        timeBudgetDecisionRef: null,
      },
      {
        ...BaseIntent,
        notificationIntentId: 'notification-intent-missing-policy-ref',
        policyRefs: [],
      },
    ]) {
      expect(AppGameNotificationIntentSchema.safeParse(invalidIntent).success).toBe(false);
    }
  });
}

function keepsManualAndUnavailableHonest() {
  it('keeps manual-required and unavailable intents out of queued provider claims', () => {
    const manualRequired = AppGameNotificationIntentSchema.parse({
      ...BaseIntent,
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
      manualProofRequirements: ['provider preference setup before delivery can be claimed'],
      deliveryClaimState: AppGameNotificationDeliveryClaimState.ManualRequired,
    });
    const falseLocalOutboxClaim = AppGameNotificationIntentSchema.safeParse({
      ...manualRequired,
      localOutboxRecordRef: 'local-outbox-record-false-claim',
    });

    expect(manualRequired.deliveryClaimState).toBe(AppGameNotificationDeliveryClaimState.ManualRequired);
    expect(falseLocalOutboxClaim.success).toBe(false);
  });
}
