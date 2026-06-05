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
import {
  AppGameNotificationPayloadPreflightReadModelSchema,
  AppGameNotificationPayloadPreflightStatus,
  buildAppGameNotificationPayloadPreflightReadModel,
} from '../src/app-game-notification-payload-preflight';
import {
  buildAppGameNotificationSchedulerBridgeReadModel,
  type AppGameNotificationSchedulerBridgeReadModel,
} from '../src/app-game-notification-scheduler-bridge';
import {
  ParentActorRole,
  ParentContractSchemaVersion,
  ParentEvidenceReferenceKind,
  ParentPlatform,
} from '../src/reference-primitives';

const Timestamp = '2026-06-05T03:18:00Z';
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
  family: { familyId: 'family-app-game-payload-preflight' },
  parentAction: {
    actionReferenceId: 'parent-action-app-game-payload-preflight',
    actor: { actorId: 'parent-app-game-payload-preflight', role: ParentActorRole.Parent },
    policyVersion: 'policy-app-game-notification-payload-v1',
    createdAt: Timestamp,
  },
  generatedAt: Timestamp,
  bridgeId: 'app-game-notification-local-outbox-bridge-for-payload-proof',
  outboxRootRef: 'parent-owned-app-game-local-outbox-root-for-payload',
  outboxFileRef: 'parent-owned-app-game-local-outbox-jsonl-for-payload',
  localDataPathRef: 'parent-owned-app-game-local-outbox-data-path-for-payload',
} as const;
const SchedulerOptions = {
  generatedAt: Timestamp,
  schedulerBridgeId: 'app-game-notification-scheduler-bridge-for-payload-proof',
  schedulerArtifactRootRef: 'parent-owned-app-game-notification-scheduler-root-for-payload',
  schedulerArtifactRef: 'parent-owned-app-game-notification-scheduler-jsonl-for-payload',
  schedulerNowAt: Timestamp,
} as const;
const PayloadOptions = {
  generatedAt: Timestamp,
  payloadPreflightId: 'app-game-notification-payload-preflight-proof',
} as const;
const BaseIntent = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  notificationIntentId: 'notification-intent-time-limit-payload',
  intentKind: AppGameNotificationIntentKind.TimeLimitReached,
  intentStatus: AppGameNotificationIntentStatus.LocalOutboxEligible,
  priority: AppGameNotificationPriority.Urgent,
  device: {
    deviceId: 'device-app-game-payload-preflight',
    childProfileId: 'child-app-game-payload-preflight',
    label: 'Study PC',
    platform: ParentPlatform.Windows,
  },
  targetKind: AppGameChildUxTargetKind.NativeGame,
  targetRef: 'target-native-game-payload-preflight',
  notificationReasonCode: AppGameNotificationReasonCode.TimeLimit,
  providerChannelPreference: 'in-app',
  parentTitleToken: AppGameNotificationParentCopyToken.TimeLimitTitle,
  parentBodyToken: AppGameNotificationParentCopyToken.TimeLimitBody,
  parentActionToken: AppGameNotificationParentCopyToken.OpenParentReviewAction,
  childTitleToken: AppGameChildUxCopyToken.LimitReachedTitle,
  childBodyToken: AppGameChildUxCopyToken.LimitReachedBody,
  notificationRuleRef: 'notification-rule-app-game-time-limit-payload',
  notificationStatusRef: 'notification-status-app-game-time-limit-payload',
  policyRefs: ['policy-ref-app-game-time-limit-payload'],
  auditRefs: ['audit-ref-app-game-time-limit-payload'],
  evidenceReferences: [
    {
      evidenceReferenceId: 'evidence-ref-app-game-time-limit-payload',
      kind: ParentEvidenceReferenceKind.PolicyDecision,
      observedAt: Timestamp,
    },
  ],
  childReasonReferences: [],
  childStatusReferences: ['child-status-app-game-time-limit-payload'],
  approvalActionRef: null,
  timeBudgetDecisionRef: 'time-budget-decision-app-game-payload',
  unknownCandidateRef: null,
  localOutboxRecordRef: 'local-outbox-record-app-game-time-limit-payload',
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

describe('app/game notification payload preflight', () => {
  it('requires minimal provider payload fields and sensitive-detail exclusions for scheduled rows', () => {
    const readModel = buildAppGameNotificationPayloadPreflightReadModel(PayloadOptions, schedulerReadModel());
    const scheduledRows = readModel.rows.filter(
      (row) => row.status === AppGameNotificationPayloadPreflightStatus.MinimalPayloadRequired
    );

    expect(readModel.minimalPayloadRequiredCount).toBe(2);
    expect(readModel.manualRequiredCount).toBe(1);
    expect(readModel.unavailableCount).toBe(1);
    expect(scheduledRows.map((row) => row.providerChannel)).toEqual(['in-app', 'email']);
    expect(scheduledRows.map((row) => row.minimalPayloadFields)).toEqual([
      [...MinimalPayloadFields],
      [...MinimalPayloadFields],
    ]);
    expect(scheduledRows.map((row) => row.sensitiveDetailExclusionRefs.length)).toEqual([5, 5]);
    expect(scheduledRows.map((row) => row.providerTemplateRequirementRefs.length)).toEqual([1, 1]);
  });

  it('keeps manual-required and unavailable source rows blocked without payload refs', () => {
    const readModel = buildAppGameNotificationPayloadPreflightReadModel(PayloadOptions, schedulerReadModel());
    const blockedRows = readModel.rows.filter(
      (row) => row.status !== AppGameNotificationPayloadPreflightStatus.MinimalPayloadRequired
    );

    expect(blockedRows.map((row) => row.status)).toEqual([
      AppGameNotificationPayloadPreflightStatus.ManualRequired,
      AppGameNotificationPayloadPreflightStatus.Unavailable,
    ]);
    expect(blockedRows.map((row) => row.sourceSchedulerEntryRef)).toEqual([null, null]);
    expect(blockedRows.map((row) => row.providerChannel)).toEqual([null, null]);
    expect(blockedRows.map((row) => row.payloadProofRequirements)).toEqual([
      ['provider preference setup before app game notification payload can be claimed'],
      ['local evidence and policy readiness before unavailable notification payload can be claimed'],
    ]);
  });

  it('rejects provider payload runtime and sensitive detail overclaims', () => {
    const readModel = buildAppGameNotificationPayloadPreflightReadModel(PayloadOptions, schedulerReadModel());

    expect(
      AppGameNotificationPayloadPreflightReadModelSchema.safeParse({
        ...readModel,
        providerPayloadTemplateRuntimeClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameNotificationPayloadPreflightReadModelSchema.safeParse({
        ...readModel,
        rawUrlOrTitleIncluded: true,
      }).success
    ).toBe(false);
    expect(
      AppGameNotificationPayloadPreflightReadModelSchema.safeParse({
        ...readModel,
        rows: [{ ...readModel.rows[0], sensitiveDetailExclusionRefs: [] }, ...readModel.rows.slice(1)],
      }).success
    ).toBe(false);
  });
});

function schedulerReadModel(): AppGameNotificationSchedulerBridgeReadModel {
  const sourceReadModel = buildAppGameNotificationLocalOutboxBridgeReadModel(BridgeOptions, [
    BaseIntent,
    suspiciousUnknownIntent(),
    manualRequiredIntent(),
    unavailableIntent(),
  ]);
  return buildAppGameNotificationSchedulerBridgeReadModel(SchedulerOptions, sourceReadModel);
}

function suspiciousUnknownIntent() {
  return {
    ...BaseIntent,
    notificationIntentId: 'notification-intent-suspicious-unknown-payload',
    intentKind: AppGameNotificationIntentKind.SuspiciousUnknown,
    priority: AppGameNotificationPriority.Attention,
    targetKind: AppGameChildUxTargetKind.UnknownApp,
    targetRef: 'target-unknown-app-payload-preflight',
    notificationReasonCode: AppGameNotificationReasonCode.SuspiciousUnknown,
    providerChannelPreference: 'email',
    parentTitleToken: AppGameNotificationParentCopyToken.SuspiciousUnknownTitle,
    parentBodyToken: AppGameNotificationParentCopyToken.SuspiciousUnknownBody,
    childTitleToken: AppGameChildUxCopyToken.NewAppTitle,
    childBodyToken: AppGameChildUxCopyToken.NewAppBody,
    localOutboxRecordRef: 'local-outbox-record-app-game-suspicious-unknown-payload',
    timeBudgetDecisionRef: null,
    unknownCandidateRef: 'unknown-app-candidate-payload-preflight',
  } as const;
}

function manualRequiredIntent() {
  return {
    ...BaseIntent,
    notificationIntentId: 'notification-intent-manual-required-payload',
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
    manualProofRequirements: ['provider preference setup before app game notification payload can be claimed'],
    deliveryClaimState: AppGameNotificationDeliveryClaimState.ManualRequired,
  } as const;
}

function unavailableIntent() {
  return {
    ...BaseIntent,
    notificationIntentId: 'notification-intent-unavailable-payload',
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
    manualProofRequirements: [
      'local evidence and policy readiness before unavailable notification payload can be claimed',
    ],
    deliveryClaimState: AppGameNotificationDeliveryClaimState.ManualRequired,
  } as const;
}
