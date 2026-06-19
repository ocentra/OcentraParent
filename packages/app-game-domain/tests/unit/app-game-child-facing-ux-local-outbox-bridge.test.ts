import { describe, expect, it } from 'vitest';
import { AppGameChildUxCardSchema } from '../../src/app-game-child-facing-ux';
import {
  AppGameChildUxCapabilityState,
  AppGameChildUxClaimState,
  AppGameChildUxCopyToken,
  AppGameChildUxExplanationSource,
  AppGameChildUxPrimaryAction,
  AppGameChildUxSurfaceState,
  AppGameChildUxTargetKind,
} from '../../src/app-game-child-facing-ux-rules';
import {
  buildAppGameChildUxHandoffReadModel,
  type AppGameChildUxHandoffReadModel,
} from '../../src/app-game-child-facing-ux-handoff';
import {
  buildAppGameChildUxLocalHandoffArtifactReadModel,
  type AppGameChildUxLocalHandoffArtifactReadModel,
} from '../../src/app-game-child-facing-ux-local-handoff';
import {
  AppGameChildUxLocalOutboxBridgeReadModelSchema,
  AppGameChildUxLocalOutboxBridgeStatus,
  buildAppGameChildUxLocalOutboxBridgeReadModel,
  parseAppGameChildUxLocalOutboxJsonl,
  serializeAppGameChildUxLocalOutboxJsonl,
} from '../../src/app-game-child-facing-ux-local-outbox-bridge';
import { NotificationLocalOutboxRecordSchema } from '@ocentra-parent/notification-domain/notification-local-outbox-adapter-proof';
import {
  ParentActorRole,
  ParentContractSchemaVersion,
  ParentEvidenceReferenceKind,
  ParentPlatform,
} from '@ocentra-parent/schema-domain/family-reference-primitives';

const Timestamp = '2026-06-07T21:11:00Z';

const Device = {
  deviceId: 'device-child-ux-local-outbox',
  childProfileId: 'child-profile-child-ux-local-outbox',
  label: 'Study PC',
  platform: ParentPlatform.Windows,
} as const;

const EvidenceReference = {
  evidenceReferenceId: 'evidence-child-ux-local-outbox',
  kind: ParentEvidenceReferenceKind.PolicyDecision,
  observedAt: Timestamp,
} as const;

const ParentAction = {
  actionReferenceId: 'parent-action-child-ux-local-outbox',
  actor: {
    actorId: 'parent-child-ux-local-outbox',
    role: ParentActorRole.Parent,
  },
  policyVersion: 'policy-child-ux-local-outbox-v1',
  createdAt: Timestamp,
} as const;

const ApprovalRequestRef = {
  actionReferenceId: 'approval-request-child-ux-local-outbox',
  actor: {
    actorId: 'child-device-local-agent',
    role: ParentActorRole.System,
  },
  policyVersion: 'policy-child-ux-local-outbox-v1',
  createdAt: Timestamp,
} as const;

const BaseChildCard = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  childUxStateId: 'child-ux-limit-reached-local-outbox',
  device: Device,
  target: {
    targetKind: AppGameChildUxTargetKind.NativeGame,
    targetRef: 'target-native-game-child-ux-local-outbox',
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
  childReasonReferences: ['child-reason-limit-reached-local-outbox'],
  childStatusReferences: ['child-status-limit-reached-local-outbox'],
  approvalRequestRef: ApprovalRequestRef,
  privateDiagnosticReferences: [],
  adapterActionRef: null,
} as const;

const HandoffOptions = {
  generatedAt: Timestamp,
  handoffId: 'app-game-child-ux-local-outbox-handoff',
  localHandoffRootRef: 'child-device-local-ux-outbox-handoff-root',
} as const;

const ArtifactOptions = {
  generatedAt: Timestamp,
  localArtifactRootRef: 'child-device-local-ux-outbox-artifact-root',
  localArtifactFileRef: 'child-device-local-ux-outbox-artifact-jsonl',
} as const;

const BridgeOptions = {
  family: { familyId: 'family-child-ux-local-outbox' },
  parentAction: ParentAction,
  generatedAt: Timestamp,
  bridgeId: 'app-game-child-ux-local-outbox-bridge-proof',
  outboxRootRef: 'parent-owned-child-ux-local-outbox-root',
  outboxFileRef: 'parent-owned-child-ux-local-outbox-jsonl-ref',
  localDataPathRef: 'parent-owned-child-ux-local-outbox-data-path-ref',
} as const;

describe('app/game child UX local outbox bridge', () => {
  it('queues local-outbox records for deliverable child UX artifact records', () => {
    const readModel = buildAppGameChildUxLocalOutboxBridgeReadModel(BridgeOptions, artifactReadModel());
    const records = parseAppGameChildUxLocalOutboxJsonl(serializeAppGameChildUxLocalOutboxJsonl(readModel));

    expect(readModel.linkedRecordCount).toBe(2);
    expect(readModel.manualRequiredCount).toBe(0);
    expect(readModel.unavailableCount).toBe(0);
    expect(records.map((record) => record.entryId)).toEqual([
      'app-game-child-ux-local-outbox-app-game-child-ux-local-handoff-child-ux-limit-reached-local-outbox',
      'app-game-child-ux-local-outbox-app-game-child-ux-local-handoff-child-ux-native-app-request-submitted-outbox',
    ]);
    expect(records.map((record) => record.envelope.reasonCode)).toEqual(['policy-violation', 'parent-request']);
    expect(records.map((record) => record.envelope.providerChannel)).toEqual(['in-app', 'in-app']);
    expect(records.map((record) => record.providerDeliveryAttempted)).toEqual([false, false]);
    expect(records.map((record) => record.envelope.rawChildEvidenceIncluded)).toEqual([false, false]);
  });

  it('keeps manual-required and unavailable child UX artifacts out of queued JSONL records', () => {
    const readModel = buildAppGameChildUxLocalOutboxBridgeReadModel(BridgeOptions, blockedArtifactReadModel());

    expect(readModel.rows.map((row) => row.status)).toEqual([
      AppGameChildUxLocalOutboxBridgeStatus.ManualRequired,
      AppGameChildUxLocalOutboxBridgeStatus.Unavailable,
    ]);
    expect(readModel.manualRequiredCount).toBe(1);
    expect(readModel.unavailableCount).toBe(1);
    expect(readModel.rows.map((row) => row.outboxRecord)).toEqual([null, null]);
    expect(serializeAppGameChildUxLocalOutboxJsonl(readModel)).toBe('\n');
  });

  it('rejects delivery runtime provider UI adapter and unsafe queued-record overclaims', () => {
    const readModel = buildAppGameChildUxLocalOutboxBridgeReadModel(BridgeOptions, artifactReadModel());
    const record = readModel.rows[0]?.outboxRecord;

    expect(
      AppGameChildUxLocalOutboxBridgeReadModelSchema.safeParse({
        ...readModel,
        childDeliveryRuntimeClaimed: true,
      }).success
    ).toBe(false);
    expect(
      NotificationLocalOutboxRecordSchema.safeParse({
        ...record,
        providerDeliveryObserved: true,
      }).success
    ).toBe(false);
    expect(() =>
      parseAppGameChildUxLocalOutboxJsonl(`${JSON.stringify({ ...record, parentNotificationUiClaimed: true })}\n`)
    ).toThrow();
  });
});

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
    childUxStateId: 'child-ux-native-app-request-submitted-outbox',
    target: {
      targetKind: AppGameChildUxTargetKind.NativeApp,
      targetRef: 'target-native-app-child-ux-local-outbox',
      childSafeDisplayLabelToken: AppGameChildUxCopyToken.RequestSubmittedTitle,
    },
    surfaceState: AppGameChildUxSurfaceState.RequestSubmitted,
    claimState: AppGameChildUxClaimState.RequestSubmitted,
    titleToken: AppGameChildUxCopyToken.RequestSubmittedTitle,
    bodyToken: AppGameChildUxCopyToken.RequestSubmittedBody,
    primaryAction: AppGameChildUxPrimaryAction.Dismiss,
    primaryActionToken: AppGameChildUxCopyToken.DismissAction,
    childReasonReferences: ['child-reason-request-submitted-local-outbox'],
    childStatusReferences: ['child-status-request-submitted-local-outbox'],
    approvalRequestRef: null,
  } as const;
}

function manualCard() {
  return {
    ...BaseChildCard,
    childUxStateId: 'child-ux-manual-required-local-outbox',
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
    childUxStateId: 'child-ux-unavailable-local-outbox',
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
