import { describe, expect, it } from 'vitest';
import { AppGameChildUxCardSchema } from '../src/app-game-child-facing-ux';
import {
  AppGameChildUxCapabilityState,
  AppGameChildUxClaimState,
  AppGameChildUxCopyToken,
  AppGameChildUxExplanationSource,
  AppGameChildUxPrimaryAction,
  AppGameChildUxSurfaceState,
  AppGameChildUxTargetKind,
} from '../src/app-game-child-facing-ux-rules';
import {
  buildAppGameChildUxHandoffReadModel,
  type AppGameChildUxHandoffReadModel,
} from '../src/app-game-child-facing-ux-handoff';
import {
  buildAppGameChildUxLocalHandoffArtifactReadModel,
  type AppGameChildUxLocalHandoffArtifactReadModel,
} from '../src/app-game-child-facing-ux-local-handoff';
import {
  buildAppGameChildUxLocalOutboxBridgeReadModel,
  type AppGameChildUxLocalOutboxBridgeReadModel,
} from '../src/app-game-child-facing-ux-local-outbox-bridge';
import {
  AppGameChildUxLocalOutboxSchedulerBridgeReadModelSchema,
  AppGameChildUxLocalOutboxSchedulerBridgeStatus,
  buildAppGameChildUxLocalOutboxSchedulerBridgeReadModel,
  parseAppGameChildUxLocalOutboxSchedulerJsonl,
  serializeAppGameChildUxLocalOutboxSchedulerJsonl,
} from '../src/app-game-child-facing-ux-local-outbox-scheduler-bridge';
import { NotificationLocalOutboxSchedulerRecordSchema } from '../src/notification-local-outbox-scheduler-proof';
import {
  ParentActorRole,
  ParentContractSchemaVersion,
  ParentEvidenceReferenceKind,
  ParentPlatform,
} from '../src/reference-primitives';

const Timestamp = '2026-06-07T21:22:00Z';

const Device = {
  deviceId: 'device-child-ux-local-outbox-scheduler',
  childProfileId: 'child-profile-child-ux-local-outbox-scheduler',
  label: 'Study PC',
  platform: ParentPlatform.Windows,
} as const;

const EvidenceReference = {
  evidenceReferenceId: 'evidence-child-ux-local-outbox-scheduler',
  kind: ParentEvidenceReferenceKind.PolicyDecision,
  observedAt: Timestamp,
} as const;

const ParentAction = {
  actionReferenceId: 'parent-action-child-ux-local-outbox-scheduler',
  actor: {
    actorId: 'parent-child-ux-local-outbox-scheduler',
    role: ParentActorRole.Parent,
  },
  policyVersion: 'policy-child-ux-local-outbox-scheduler-v1',
  createdAt: Timestamp,
} as const;

const ApprovalRequestRef = {
  actionReferenceId: 'approval-request-child-ux-local-outbox-scheduler',
  actor: {
    actorId: 'child-device-local-agent',
    role: ParentActorRole.System,
  },
  policyVersion: 'policy-child-ux-local-outbox-scheduler-v1',
  createdAt: Timestamp,
} as const;

const BaseChildCard = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  childUxStateId: 'child-ux-limit-reached-local-outbox-scheduler',
  device: Device,
  target: {
    targetKind: AppGameChildUxTargetKind.NativeGame,
    targetRef: 'target-native-game-child-ux-local-outbox-scheduler',
    childSafeDisplayLabelToken: AppGameChildUxCopyToken.LimitReachedTitle,
  },
  surfaceState: AppGameChildUxSurfaceState.TimeLimitReached,
  capabilityState: AppGameChildUxCapabilityState.Supported,
  claimState: AppGameChildUxClaimState.LimitReached,
  explanationSource: AppGameChildUxExplanationSource.ParentRule,
  titleToken: AppGameChildUxCopyToken.LimitReachedTitle,
  bodyToken: AppGameChildUxCopyToken.LimitReachedBody,
  primaryAction: AppGameChildUxPrimaryAction.RequestMoreTime,
  primaryActionToken: AppGameChildUxCopyToken.RequestMoreTimeAction,
  evidenceReferences: [EvidenceReference],
  childReasonReferences: ['child-reason-limit-reached-local-outbox-scheduler'],
  childStatusReferences: ['child-status-limit-reached-local-outbox-scheduler'],
  approvalRequestRef: ApprovalRequestRef,
  privateDiagnosticReferences: [],
  adapterActionRef: null,
} as const;

const HandoffOptions = {
  generatedAt: Timestamp,
  handoffId: 'app-game-child-ux-local-outbox-scheduler-handoff',
  localHandoffRootRef: 'child-device-local-ux-outbox-scheduler-handoff-root',
} as const;

const ArtifactOptions = {
  generatedAt: Timestamp,
  localArtifactRootRef: 'child-device-local-ux-outbox-scheduler-artifact-root',
  localArtifactFileRef: 'child-device-local-ux-outbox-scheduler-artifact-jsonl',
} as const;

const BridgeOptions = {
  family: { familyId: 'family-child-ux-local-outbox-scheduler' },
  parentAction: ParentAction,
  generatedAt: Timestamp,
  bridgeId: 'app-game-child-ux-local-outbox-scheduler-source-bridge-proof',
  outboxRootRef: 'parent-owned-child-ux-local-outbox-scheduler-root',
  outboxFileRef: 'parent-owned-child-ux-local-outbox-scheduler-jsonl-ref',
  localDataPathRef: 'parent-owned-child-ux-local-outbox-scheduler-data-path-ref',
} as const;

const SchedulerOptions = {
  generatedAt: Timestamp,
  schedulerBridgeId: 'app-game-child-ux-local-outbox-scheduler-bridge-proof',
  schedulerArtifactRootRef: 'parent-owned-child-ux-local-outbox-scheduler-root-ref',
  schedulerArtifactRef: 'parent-owned-child-ux-local-outbox-scheduler-jsonl-ref',
  schedulerNowAt: Timestamp,
} as const;

describe('app/game child UX local outbox scheduler bridge', () => {
  it('schedules deliverable child UX local outbox records through existing scheduler JSONL rows', () => {
    const readModel = buildAppGameChildUxLocalOutboxSchedulerBridgeReadModel(SchedulerOptions, sourceOutboxReadModel());
    const records = parseAppGameChildUxLocalOutboxSchedulerJsonl(
      serializeAppGameChildUxLocalOutboxSchedulerJsonl(readModel)
    );

    expect(readModel.scheduledRecordCount).toBe(2);
    expect(readModel.unscheduledManualRequiredCount).toBe(0);
    expect(readModel.unscheduledUnavailableCount).toBe(0);
    expect(records.map((record) => record.sourceEntryId)).toEqual([
      'app-game-child-ux-local-outbox-app-game-child-ux-local-handoff-child-ux-limit-reached-local-outbox-scheduler',
      'app-game-child-ux-local-outbox-app-game-child-ux-local-handoff-child-ux-native-app-request-submitted-scheduler',
    ]);
    expect(records.map((record) => record.schedulerState)).toEqual(['due-local', 'due-local']);
    expect(records.map((record) => record.nextAttemptAt)).toEqual([Timestamp, Timestamp]);
    expect(records.map((record) => record.providerDeliveryAttempted)).toEqual([false, false]);
    expect(records.map((record) => record.rawChildEvidenceIncluded)).toEqual([false, false]);
  });

  it('keeps manual-required and unavailable child UX local outbox rows unscheduled', () => {
    const readModel = buildAppGameChildUxLocalOutboxSchedulerBridgeReadModel(
      SchedulerOptions,
      blockedSourceOutboxReadModel()
    );

    expect(readModel.rows.map((row) => row.status)).toEqual([
      AppGameChildUxLocalOutboxSchedulerBridgeStatus.ManualRequired,
      AppGameChildUxLocalOutboxSchedulerBridgeStatus.Unavailable,
    ]);
    expect(readModel.unscheduledManualRequiredCount).toBe(1);
    expect(readModel.unscheduledUnavailableCount).toBe(1);
    expect(readModel.rows.map((row) => row.schedulerRecord)).toEqual([null, null]);
    expect(serializeAppGameChildUxLocalOutboxSchedulerJsonl(readModel)).toBe('\n');
  });

  it('rejects scheduler runtime provider UI adapter platform and unsafe scheduler-record overclaims', () => {
    const readModel = buildAppGameChildUxLocalOutboxSchedulerBridgeReadModel(SchedulerOptions, sourceOutboxReadModel());
    const record = readModel.rows[0]?.schedulerRecord;
    if (record === null || record === undefined) {
      throw new Error('expected scheduled child UX local outbox scheduler record');
    }

    expect(
      AppGameChildUxLocalOutboxSchedulerBridgeReadModelSchema.safeParse({
        ...readModel,
        retryExecutionRuntimeClaimed: true,
      }).success
    ).toBe(false);
    expect(
      NotificationLocalOutboxSchedulerRecordSchema.safeParse({
        ...record,
        providerDeliveryObserved: true,
      }).success
    ).toBe(false);
    expect(() =>
      parseAppGameChildUxLocalOutboxSchedulerJsonl(`${JSON.stringify({ ...record, rawUrlOrTitleIncluded: true })}\n`)
    ).toThrow();
  });
});

function sourceOutboxReadModel(): AppGameChildUxLocalOutboxBridgeReadModel {
  return buildAppGameChildUxLocalOutboxBridgeReadModel(BridgeOptions, artifactReadModel());
}

function blockedSourceOutboxReadModel(): AppGameChildUxLocalOutboxBridgeReadModel {
  return buildAppGameChildUxLocalOutboxBridgeReadModel(BridgeOptions, blockedArtifactReadModel());
}

function artifactReadModel(): AppGameChildUxLocalHandoffArtifactReadModel {
  return buildAppGameChildUxLocalHandoffArtifactReadModel(
    ArtifactOptions,
    handoffReadModel([BaseChildCard, nativeAppSubmittedCard()])
  );
}

function blockedArtifactReadModel(): AppGameChildUxLocalHandoffArtifactReadModel {
  return buildAppGameChildUxLocalHandoffArtifactReadModel(
    ArtifactOptions,
    handoffReadModel([manualCard(), unavailableCard()])
  );
}

function handoffReadModel(cards: ReadonlyArray<unknown>): AppGameChildUxHandoffReadModel {
  return buildAppGameChildUxHandoffReadModel(
    HandoffOptions,
    cards.map((card) => AppGameChildUxCardSchema.parse(card))
  );
}

function nativeAppSubmittedCard() {
  return {
    ...BaseChildCard,
    childUxStateId: 'child-ux-native-app-request-submitted-scheduler',
    target: {
      targetKind: AppGameChildUxTargetKind.NativeApp,
      targetRef: 'target-native-app-child-ux-local-outbox-scheduler',
      childSafeDisplayLabelToken: AppGameChildUxCopyToken.RequestSubmittedTitle,
    },
    surfaceState: AppGameChildUxSurfaceState.RequestSubmitted,
    claimState: AppGameChildUxClaimState.RequestSubmitted,
    titleToken: AppGameChildUxCopyToken.RequestSubmittedTitle,
    bodyToken: AppGameChildUxCopyToken.RequestSubmittedBody,
    primaryAction: AppGameChildUxPrimaryAction.Dismiss,
    primaryActionToken: AppGameChildUxCopyToken.DismissAction,
    childReasonReferences: ['child-reason-request-submitted-local-outbox-scheduler'],
    childStatusReferences: ['child-status-request-submitted-local-outbox-scheduler'],
    approvalRequestRef: null,
  } as const;
}

function manualCard() {
  return {
    ...BaseChildCard,
    childUxStateId: 'child-ux-manual-required-local-outbox-scheduler',
    surfaceState: AppGameChildUxSurfaceState.ManualRequired,
    capabilityState: AppGameChildUxCapabilityState.ManualRequired,
    claimState: AppGameChildUxClaimState.ManualRequired,
    titleToken: AppGameChildUxCopyToken.ManualRequiredTitle,
    bodyToken: AppGameChildUxCopyToken.ManualRequiredBody,
    primaryAction: AppGameChildUxPrimaryAction.TryLater,
    primaryActionToken: AppGameChildUxCopyToken.TryLaterAction,
    approvalRequestRef: null,
  } as const;
}

function unavailableCard() {
  return {
    ...BaseChildCard,
    childUxStateId: 'child-ux-unavailable-local-outbox-scheduler',
    surfaceState: AppGameChildUxSurfaceState.Unavailable,
    capabilityState: AppGameChildUxCapabilityState.Unavailable,
    claimState: AppGameChildUxClaimState.Unavailable,
    titleToken: AppGameChildUxCopyToken.UnavailableTitle,
    bodyToken: AppGameChildUxCopyToken.UnavailableBody,
    primaryAction: AppGameChildUxPrimaryAction.TryLater,
    primaryActionToken: AppGameChildUxCopyToken.TryLaterAction,
    approvalRequestRef: null,
  } as const;
}
