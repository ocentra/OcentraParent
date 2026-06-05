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
import { buildAppGameNotificationLocalOutboxBridgeReadModel } from '../src/app-game-notification-local-outbox-bridge';
import { buildAppGameNotificationSchedulerBridgeReadModel } from '../src/app-game-notification-scheduler-bridge';
import {
  AppGameNotificationProviderPreflightReadModelSchema,
  AppGameNotificationProviderPreflightStatus,
  buildAppGameNotificationProviderPreflightReadModel,
} from '../src/app-game-notification-provider-preflight';
import {
  ParentActorRole,
  ParentContractSchemaVersion,
  ParentEvidenceReferenceKind,
  ParentPlatform,
} from '../src/reference-primitives';

const Timestamp = '2026-06-05T02:43:00Z';
const ProviderPreflightOptions = {
  generatedAt: Timestamp,
  providerPreflightId: 'app-game-notification-provider-preflight-proof',
  sourceContractRefs: [
    'app-game-notification-scheduler-bridge',
    'notification-local-outbox-scheduler-proof',
    'notification-provider-adapter-boundary-required',
  ],
} as const;

describe('app/game notification provider preflight', () => {
  it('marks scheduled app/game rows as provider adapter required without sending', () => {
    const readModel = buildAppGameNotificationProviderPreflightReadModel(
      ProviderPreflightOptions,
      schedulerReadModel()
    );

    expect(readModel.providerAdapterRequiredCount).toBe(2);
    expect(readModel.manualRequiredCount).toBe(1);
    expect(readModel.unavailableCount).toBe(1);
    expect(readModel.rows.map((row) => row.status)).toEqual([
      AppGameNotificationProviderPreflightStatus.ProviderAdapterRequired,
      AppGameNotificationProviderPreflightStatus.ProviderAdapterRequired,
      AppGameNotificationProviderPreflightStatus.ManualRequired,
      AppGameNotificationProviderPreflightStatus.Unavailable,
    ]);
    expect(readModel.rows.slice(0, 2).map((row) => row.providerChannelRef)).toEqual(['in-app', 'email']);
    expect(readModel.rows.slice(0, 2).every((row) => row.adapterRequirementRefs.length === 3)).toBe(true);
  });

  it('keeps manual and unavailable source rows blocked before provider preflight', () => {
    const readModel = buildAppGameNotificationProviderPreflightReadModel(
      ProviderPreflightOptions,
      schedulerReadModel()
    );
    const blockedRows = readModel.rows.slice(2);

    expect(blockedRows.map((row) => row.sourceSchedulerEntryRef)).toEqual([null, null]);
    expect(blockedRows.map((row) => row.providerChannelRef)).toEqual([null, null]);
    expect(blockedRows.map((row) => row.manualProofRequirements)).toEqual([
      ['provider preference setup before app game notification can be scheduled'],
      ['local evidence and policy readiness before unavailable notification can be scheduled'],
    ]);
  });

  it('rejects provider runtime and credential overclaims', () => {
    const readModel = buildAppGameNotificationProviderPreflightReadModel(
      ProviderPreflightOptions,
      schedulerReadModel()
    );

    expect(readModel.providerDeliveryRuntimeClaimed).toBe(false);
    expect(readModel.providerCredentialsClaimed).toBe(false);
    expect(readModel.adapterDispatchClaimed).toBe(false);
    expect(
      AppGameNotificationProviderPreflightReadModelSchema.safeParse({
        ...readModel,
        providerCredentialsClaimed: true,
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
  family: { familyId: 'family-app-game-provider-preflight' },
  parentAction: {
    actionReferenceId: 'parent-action-app-game-provider-preflight',
    actor: { actorId: 'parent-app-game-provider-preflight', role: ParentActorRole.Parent },
    policyVersion: 'policy-app-game-provider-preflight-v1',
    createdAt: Timestamp,
  },
  generatedAt: Timestamp,
  bridgeId: 'app-game-notification-local-outbox-bridge-for-provider-preflight',
  outboxRootRef: 'parent-owned-app-game-local-outbox-root-for-provider-preflight',
  outboxFileRef: 'parent-owned-app-game-local-outbox-jsonl-for-provider-preflight',
  localDataPathRef: 'parent-owned-app-game-local-outbox-data-path-for-provider-preflight',
} as const;
const SchedulerOptions = {
  generatedAt: Timestamp,
  schedulerBridgeId: 'app-game-notification-scheduler-bridge-for-provider-preflight',
  schedulerArtifactRootRef: 'parent-owned-app-game-notification-scheduler-root-for-provider-preflight',
  schedulerArtifactRef: 'parent-owned-app-game-notification-scheduler-jsonl-for-provider-preflight',
  schedulerNowAt: Timestamp,
} as const;
const BaseIntent = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  notificationIntentId: 'notification-intent-time-limit-provider-preflight',
  intentKind: AppGameNotificationIntentKind.TimeLimitReached,
  intentStatus: AppGameNotificationIntentStatus.LocalOutboxEligible,
  priority: AppGameNotificationPriority.Urgent,
  device: {
    deviceId: 'device-app-game-provider-preflight',
    childProfileId: 'child-app-game-provider-preflight',
    label: 'Study PC',
    platform: ParentPlatform.Windows,
  },
  targetKind: AppGameChildUxTargetKind.NativeGame,
  targetRef: 'target-native-game-provider-preflight',
  notificationReasonCode: AppGameNotificationReasonCode.TimeLimit,
  providerChannelPreference: 'in-app',
  parentTitleToken: AppGameNotificationParentCopyToken.TimeLimitTitle,
  parentBodyToken: AppGameNotificationParentCopyToken.TimeLimitBody,
  parentActionToken: AppGameNotificationParentCopyToken.OpenParentReviewAction,
  childTitleToken: AppGameChildUxCopyToken.LimitReachedTitle,
  childBodyToken: AppGameChildUxCopyToken.LimitReachedBody,
  notificationRuleRef: 'notification-rule-app-game-time-limit-provider-preflight',
  notificationStatusRef: 'notification-status-app-game-time-limit-provider-preflight',
  policyRefs: ['policy-ref-app-game-time-limit-provider-preflight'],
  auditRefs: ['audit-ref-app-game-time-limit-provider-preflight'],
  evidenceReferences: [
    {
      evidenceReferenceId: 'evidence-ref-app-game-time-limit-provider-preflight',
      kind: ParentEvidenceReferenceKind.PolicyDecision,
      observedAt: Timestamp,
    },
  ],
  childReasonReferences: [],
  childStatusReferences: ['child-status-app-game-time-limit-provider-preflight'],
  approvalActionRef: null,
  timeBudgetDecisionRef: 'time-budget-decision-app-game-provider-preflight',
  unknownCandidateRef: null,
  localOutboxRecordRef: 'local-outbox-record-app-game-time-limit-provider-preflight',
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
    notificationIntentId: 'notification-intent-suspicious-unknown-provider-preflight',
    intentKind: AppGameNotificationIntentKind.SuspiciousUnknown,
    priority: AppGameNotificationPriority.Attention,
    targetKind: AppGameChildUxTargetKind.UnknownApp,
    targetRef: 'target-unknown-app-provider-preflight',
    notificationReasonCode: AppGameNotificationReasonCode.SuspiciousUnknown,
    providerChannelPreference: 'email',
    parentTitleToken: AppGameNotificationParentCopyToken.SuspiciousUnknownTitle,
    parentBodyToken: AppGameNotificationParentCopyToken.SuspiciousUnknownBody,
    childTitleToken: AppGameChildUxCopyToken.NewAppTitle,
    childBodyToken: AppGameChildUxCopyToken.NewAppBody,
    localOutboxRecordRef: 'local-outbox-record-app-game-suspicious-unknown-provider-preflight',
    timeBudgetDecisionRef: null,
    unknownCandidateRef: 'unknown-app-candidate-provider-preflight',
  } as const;
}

function manualRequiredIntent() {
  return {
    ...BaseIntent,
    notificationIntentId: 'notification-intent-manual-required-provider-preflight',
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
    notificationIntentId: 'notification-intent-unavailable-provider-preflight',
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
