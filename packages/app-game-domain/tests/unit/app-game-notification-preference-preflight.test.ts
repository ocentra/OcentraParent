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
import { buildAppGameNotificationLocalOutboxBridgeReadModel } from '../../src/app-game-notification-local-outbox-bridge';
import {
  AppGameNotificationPreferencePreflightReadModelSchema,
  AppGameNotificationPreferencePreflightStatus,
  buildAppGameNotificationPreferencePreflightReadModel,
} from '../../src/app-game-notification-preference-preflight';
import { buildAppGameNotificationSchedulerBridgeReadModel } from '../../src/app-game-notification-scheduler-bridge';
import {
  ParentActorRole,
  ParentContractSchemaVersion,
  ParentEvidenceReferenceKind,
  ParentPlatform,
} from '@ocentra-parent/schema-domain/family-reference-primitives';

const Timestamp = '2026-06-05T03:03:00Z';
const PreferenceOptions = {
  generatedAt: Timestamp,
  preferencePreflightId: 'app-game-notification-preference-preflight-proof',
  sourceContractRefs: [
    'app-game-notification-scheduler-bridge',
    'notification-parent-preference-boundary',
    'notification-quiet-hours-policy-boundary',
  ],
} as const;

describe('app/game notification preference preflight', () => {
  it('marks scheduled app/game rows as parent preference required before delivery', () => {
    const readModel = buildAppGameNotificationPreferencePreflightReadModel(PreferenceOptions, schedulerReadModel());

    expect(readModel.parentPreferenceRequiredCount).toBe(2);
    expect(readModel.manualRequiredCount).toBe(1);
    expect(readModel.unavailableCount).toBe(1);
    expect(readModel.rows.map((row) => row.status)).toEqual([
      AppGameNotificationPreferencePreflightStatus.ParentPreferenceRequired,
      AppGameNotificationPreferencePreflightStatus.ParentPreferenceRequired,
      AppGameNotificationPreferencePreflightStatus.ManualRequired,
      AppGameNotificationPreferencePreflightStatus.Unavailable,
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

  it('keeps manual and unavailable source rows blocked before preference preflight', () => {
    const readModel = buildAppGameNotificationPreferencePreflightReadModel(PreferenceOptions, schedulerReadModel());
    const blockedRows = readModel.rows.slice(2);

    expect(blockedRows.map((row) => row.sourceSchedulerEntryRef)).toEqual([null, null]);
    expect(blockedRows.map((row) => row.parentPreferenceState)).toEqual([null, null]);
    expect(blockedRows.map((row) => row.quietHoursDecision)).toEqual([null, null]);
    expect(blockedRows.map((row) => row.manualProofRequirements)).toEqual([
      ['provider preference setup before app game notification can be scheduled'],
      ['local evidence and policy readiness before unavailable notification can be scheduled'],
    ]);
  });

  it('rejects preference UI and delivery overclaims', () => {
    const readModel = buildAppGameNotificationPreferencePreflightReadModel(PreferenceOptions, schedulerReadModel());

    expect(readModel.parentPreferenceUiClaimed).toBe(false);
    expect(readModel.parentFrequencyControlUiClaimed).toBe(false);
    expect(readModel.providerDeliveryRuntimeClaimed).toBe(false);
    expect(
      AppGameNotificationPreferencePreflightReadModelSchema.safeParse({
        ...readModel,
        parentPreferenceUiClaimed: true,
      }).success
    ).toBe(false);
  });
});

function schedulerReadModel() {
  const bridgeReadModel = buildAppGameNotificationLocalOutboxBridgeReadModel(BridgeOptions, [
    BaseIntent,
    suspiciousUnknownIntent(),
    manualRequiredIntent(),
    unavailableIntent(),
  ]);
  return buildAppGameNotificationSchedulerBridgeReadModel(SchedulerOptions, bridgeReadModel);
}

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
  family: { familyId: 'family-app-game-preference-preflight' },
  parentAction: {
    actionReferenceId: 'parent-action-app-game-preference-preflight',
    actor: { actorId: 'parent-app-game-preference-preflight', role: ParentActorRole.Parent },
    policyVersion: 'policy-app-game-preference-preflight-v1',
    createdAt: Timestamp,
  },
  generatedAt: Timestamp,
  bridgeId: 'app-game-notification-local-outbox-bridge-for-preference-preflight',
  outboxRootRef: 'parent-owned-app-game-local-outbox-root-for-preference-preflight',
  outboxFileRef: 'parent-owned-app-game-local-outbox-jsonl-for-preference-preflight',
  localDataPathRef: 'parent-owned-app-game-local-outbox-data-path-for-preference-preflight',
} as const;
const SchedulerOptions = {
  generatedAt: Timestamp,
  schedulerBridgeId: 'app-game-notification-scheduler-bridge-for-preference-preflight',
  schedulerArtifactRootRef: 'parent-owned-app-game-notification-scheduler-root-for-preference-preflight',
  schedulerArtifactRef: 'parent-owned-app-game-notification-scheduler-jsonl-for-preference-preflight',
  schedulerNowAt: Timestamp,
} as const;
const BaseIntent = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  notificationIntentId: 'notification-intent-time-limit-preference-preflight',
  intentKind: AppGameNotificationIntentKind.TimeLimitReached,
  intentStatus: AppGameNotificationIntentStatus.LocalOutboxEligible,
  priority: AppGameNotificationPriority.Urgent,
  device: {
    deviceId: 'device-app-game-preference-preflight',
    childProfileId: 'child-app-game-preference-preflight',
    label: 'Study PC',
    platform: ParentPlatform.Windows,
  },
  targetKind: AppGameChildUxTargetKind.NativeGame,
  targetRef: 'target-native-game-preference-preflight',
  notificationReasonCode: AppGameNotificationReasonCode.TimeLimit,
  providerChannelPreference: 'in-app',
  parentTitleToken: AppGameNotificationParentCopyToken.TimeLimitTitle,
  parentBodyToken: AppGameNotificationParentCopyToken.TimeLimitBody,
  parentActionToken: AppGameNotificationParentCopyToken.OpenParentReviewAction,
  childTitleToken: AppGameChildUxCopyToken.LimitReachedTitle,
  childBodyToken: AppGameChildUxCopyToken.LimitReachedBody,
  notificationRuleRef: 'notification-rule-app-game-time-limit-preference-preflight',
  notificationStatusRef: 'notification-status-app-game-time-limit-preference-preflight',
  policyRefs: ['policy-ref-app-game-time-limit-preference-preflight'],
  auditRefs: ['audit-ref-app-game-time-limit-preference-preflight'],
  evidenceReferences: [
    {
      evidenceReferenceId: 'evidence-ref-app-game-time-limit-preference-preflight',
      kind: ParentEvidenceReferenceKind.PolicyDecision,
      observedAt: Timestamp,
    },
  ],
  childReasonReferences: [],
  childStatusReferences: ['child-status-app-game-time-limit-preference-preflight'],
  approvalActionRef: null,
  timeBudgetDecisionRef: 'time-budget-decision-app-game-preference-preflight',
  unknownCandidateRef: null,
  localOutboxRecordRef: 'local-outbox-record-app-game-time-limit-preference-preflight',
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

function suspiciousUnknownIntent() {
  return {
    ...BaseIntent,
    notificationIntentId: 'notification-intent-suspicious-unknown-preference-preflight',
    intentKind: AppGameNotificationIntentKind.SuspiciousUnknown,
    priority: AppGameNotificationPriority.Attention,
    targetKind: AppGameChildUxTargetKind.UnknownApp,
    targetRef: 'target-unknown-app-preference-preflight',
    notificationReasonCode: AppGameNotificationReasonCode.SuspiciousUnknown,
    providerChannelPreference: 'email',
    parentTitleToken: AppGameNotificationParentCopyToken.SuspiciousUnknownTitle,
    parentBodyToken: AppGameNotificationParentCopyToken.SuspiciousUnknownBody,
    childTitleToken: AppGameChildUxCopyToken.NewAppTitle,
    childBodyToken: AppGameChildUxCopyToken.NewAppBody,
    localOutboxRecordRef: 'local-outbox-record-app-game-suspicious-unknown-preference-preflight',
    timeBudgetDecisionRef: null,
    unknownCandidateRef: 'unknown-app-candidate-preference-preflight',
  } as const;
}

function manualRequiredIntent() {
  return {
    ...BaseIntent,
    notificationIntentId: 'notification-intent-manual-required-preference-preflight',
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
    notificationIntentId: 'notification-intent-unavailable-preference-preflight',
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
