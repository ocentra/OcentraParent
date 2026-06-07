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
import { buildAppGameChildUxLocalHandoffArtifactReadModel } from '../src/app-game-child-facing-ux-local-handoff';
import { buildAppGameChildUxLocalOutboxBridgeReadModel } from '../src/app-game-child-facing-ux-local-outbox-bridge';
import {
  buildAppGameChildUxLocalOutboxSchedulerBridgeReadModel,
  type AppGameChildUxLocalOutboxSchedulerBridgeReadModel,
} from '../src/app-game-child-facing-ux-local-outbox-scheduler-bridge';
import {
  AppGameChildUxLocalOutboxProviderPreflightReadModelSchema,
  AppGameChildUxLocalOutboxProviderPreflightStatus,
  buildAppGameChildUxLocalOutboxProviderPreflightReadModel,
} from '../src/app-game-child-facing-ux-local-outbox-provider-preflight';
import {
  ParentActorRole,
  ParentContractSchemaVersion,
  ParentEvidenceReferenceKind,
  ParentPlatform,
} from '../src/reference-primitives';

const Timestamp = '2026-06-07T21:36:00Z';

const Device = {
  deviceId: 'device-child-ux-provider-preflight',
  childProfileId: 'child-profile-child-ux-provider-preflight',
  label: 'Study PC',
  platform: ParentPlatform.Windows,
} as const;

const EvidenceReference = {
  evidenceReferenceId: 'evidence-child-ux-provider-preflight',
  kind: ParentEvidenceReferenceKind.PolicyDecision,
  observedAt: Timestamp,
} as const;

const ParentAction = {
  actionReferenceId: 'parent-action-child-ux-provider-preflight',
  actor: {
    actorId: 'parent-child-ux-provider-preflight',
    role: ParentActorRole.Parent,
  },
  policyVersion: 'policy-child-ux-provider-preflight-v1',
  createdAt: Timestamp,
} as const;

const ApprovalRequestRef = {
  actionReferenceId: 'approval-request-child-ux-provider-preflight',
  actor: {
    actorId: 'child-device-local-agent',
    role: ParentActorRole.System,
  },
  policyVersion: 'policy-child-ux-provider-preflight-v1',
  createdAt: Timestamp,
} as const;

const BaseChildCard = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  childUxStateId: 'child-ux-limit-reached-provider-preflight',
  device: Device,
  target: {
    targetKind: AppGameChildUxTargetKind.NativeGame,
    targetRef: 'target-native-game-child-ux-provider-preflight',
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
  childReasonReferences: ['child-reason-limit-reached-provider-preflight'],
  childStatusReferences: ['child-status-limit-reached-provider-preflight'],
  approvalRequestRef: ApprovalRequestRef,
  privateDiagnosticReferences: [],
  adapterActionRef: null,
} as const;

const HandoffOptions = {
  generatedAt: Timestamp,
  handoffId: 'app-game-child-ux-provider-preflight-handoff',
  localHandoffRootRef: 'child-device-local-ux-provider-preflight-handoff-root',
} as const;

const ArtifactOptions = {
  generatedAt: Timestamp,
  localArtifactRootRef: 'child-device-local-ux-provider-preflight-artifact-root',
  localArtifactFileRef: 'child-device-local-ux-provider-preflight-artifact-jsonl',
} as const;

const BridgeOptions = {
  family: { familyId: 'family-child-ux-provider-preflight' },
  parentAction: ParentAction,
  generatedAt: Timestamp,
  bridgeId: 'app-game-child-ux-provider-preflight-source-bridge-proof',
  outboxRootRef: 'parent-owned-child-ux-provider-preflight-root',
  outboxFileRef: 'parent-owned-child-ux-provider-preflight-jsonl-ref',
  localDataPathRef: 'parent-owned-child-ux-provider-preflight-data-path-ref',
} as const;

const SchedulerOptions = {
  generatedAt: Timestamp,
  schedulerBridgeId: 'app-game-child-ux-provider-preflight-scheduler-bridge-proof',
  schedulerArtifactRootRef: 'parent-owned-child-ux-provider-preflight-scheduler-root-ref',
  schedulerArtifactRef: 'parent-owned-child-ux-provider-preflight-scheduler-jsonl-ref',
  schedulerNowAt: Timestamp,
} as const;

const ProviderPreflightOptions = {
  generatedAt: Timestamp,
  providerPreflightId: 'app-game-child-ux-local-outbox-provider-preflight-proof',
  sourceContractRefs: [
    'app-game-child-ux-local-outbox-scheduler-bridge',
    'notification-local-outbox-scheduler-proof',
    'notification-provider-adapter-boundary-required',
  ],
} as const;

describe('app/game child UX local outbox provider preflight', () => {
  it('marks scheduled child UX local outbox rows as provider adapter required without sending', () => {
    const readModel = buildAppGameChildUxLocalOutboxProviderPreflightReadModel(
      ProviderPreflightOptions,
      schedulerReadModel([BaseChildCard, nativeAppSubmittedCard()])
    );

    expect(readModel.providerAdapterRequiredCount).toBe(2);
    expect(readModel.manualRequiredCount).toBe(0);
    expect(readModel.unavailableCount).toBe(0);
    expect(readModel.rows.map((row) => row.status)).toEqual([
      AppGameChildUxLocalOutboxProviderPreflightStatus.ProviderAdapterRequired,
      AppGameChildUxLocalOutboxProviderPreflightStatus.ProviderAdapterRequired,
    ]);
    expect(readModel.rows.map((row) => row.providerChannelRef)).toEqual(['in-app', 'in-app']);
    expect(readModel.rows.every((row) => row.adapterRequirementRefs.length === 3)).toBe(true);
    expect(readModel.rows.every((row) => row.sourceSchedulerEntryRef !== null)).toBe(true);
  });

  it('keeps manual and unavailable child UX source rows blocked before provider preflight', () => {
    const readModel = buildAppGameChildUxLocalOutboxProviderPreflightReadModel(
      ProviderPreflightOptions,
      schedulerReadModel([manualCard(), unavailableCard()])
    );

    expect(readModel.rows.map((row) => row.status)).toEqual([
      AppGameChildUxLocalOutboxProviderPreflightStatus.ManualRequired,
      AppGameChildUxLocalOutboxProviderPreflightStatus.Unavailable,
    ]);
    expect(readModel.rows.map((row) => row.sourceSchedulerEntryRef)).toEqual([null, null]);
    expect(readModel.rows.map((row) => row.providerChannelRef)).toEqual([null, null]);
    expect(readModel.rows.map((row) => row.manualProofRequirements.length)).toEqual([1, 1]);
  });

  it('rejects provider runtime credential UI adapter platform and source-row overclaims', () => {
    const readModel = buildAppGameChildUxLocalOutboxProviderPreflightReadModel(
      ProviderPreflightOptions,
      schedulerReadModel([BaseChildCard])
    );

    expect(readModel.providerDeliveryRuntimeClaimed).toBe(false);
    expect(readModel.providerCredentialsClaimed).toBe(false);
    expect(readModel.adapterDispatchClaimed).toBe(false);
    expect(readModel.platformEnforcementClaimed).toBe(false);
    expect(
      AppGameChildUxLocalOutboxProviderPreflightReadModelSchema.safeParse({
        ...readModel,
        providerCredentialsClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameChildUxLocalOutboxProviderPreflightReadModelSchema.safeParse({
        ...readModel,
        rawPrivateSourceRowsIncluded: true,
      }).success
    ).toBe(false);
  });
});

function schedulerReadModel(cards: ReadonlyArray<unknown>): AppGameChildUxLocalOutboxSchedulerBridgeReadModel {
  const handoff = buildAppGameChildUxHandoffReadModel(
    HandoffOptions,
    cards.map((card) => AppGameChildUxCardSchema.parse(card))
  );
  const artifacts = buildAppGameChildUxLocalHandoffArtifactReadModel(ArtifactOptions, handoff);
  const outbox = buildAppGameChildUxLocalOutboxBridgeReadModel(BridgeOptions, artifacts);
  return buildAppGameChildUxLocalOutboxSchedulerBridgeReadModel(SchedulerOptions, outbox);
}

function nativeAppSubmittedCard() {
  return {
    ...BaseChildCard,
    childUxStateId: 'child-ux-native-app-request-submitted-provider-preflight',
    target: {
      targetKind: AppGameChildUxTargetKind.NativeApp,
      targetRef: 'target-native-app-child-ux-provider-preflight',
      childSafeDisplayLabelToken: AppGameChildUxCopyToken.RequestSubmittedTitle,
    },
    surfaceState: AppGameChildUxSurfaceState.RequestSubmitted,
    claimState: AppGameChildUxClaimState.RequestSubmitted,
    titleToken: AppGameChildUxCopyToken.RequestSubmittedTitle,
    bodyToken: AppGameChildUxCopyToken.RequestSubmittedBody,
    primaryAction: AppGameChildUxPrimaryAction.Dismiss,
    primaryActionToken: AppGameChildUxCopyToken.DismissAction,
    childReasonReferences: ['child-reason-request-submitted-provider-preflight'],
    childStatusReferences: ['child-status-request-submitted-provider-preflight'],
    approvalRequestRef: null,
  } as const;
}

function manualCard() {
  return {
    ...BaseChildCard,
    childUxStateId: 'child-ux-manual-required-provider-preflight',
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
    childUxStateId: 'child-ux-unavailable-provider-preflight',
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
