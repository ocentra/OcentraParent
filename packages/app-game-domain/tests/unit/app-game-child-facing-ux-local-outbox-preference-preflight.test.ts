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
import { buildAppGameChildUxHandoffReadModel } from '../../src/app-game-child-facing-ux-handoff';
import { buildAppGameChildUxLocalHandoffArtifactReadModel } from '../../src/app-game-child-facing-ux-local-handoff';
import { buildAppGameChildUxLocalOutboxBridgeReadModel } from '../../src/app-game-child-facing-ux-local-outbox-bridge';
import {
  buildAppGameChildUxLocalOutboxSchedulerBridgeReadModel,
  type AppGameChildUxLocalOutboxSchedulerBridgeReadModel,
} from '../../src/app-game-child-facing-ux-local-outbox-scheduler-bridge';
import {
  AppGameChildUxLocalOutboxPreferencePreflightReadModelSchema,
  AppGameChildUxLocalOutboxPreferencePreflightStatus,
  buildAppGameChildUxLocalOutboxPreferencePreflightReadModel,
} from '../../src/app-game-child-facing-ux-local-outbox-preference-preflight';
import {
  ParentActorRole,
  ParentContractSchemaVersion,
  ParentEvidenceReferenceKind,
  ParentPlatform,
} from '@ocentra-parent/family-domain/reference-primitives';

const Timestamp = '2026-06-07T21:54:00Z';

const Device = {
  deviceId: 'device-child-ux-preference-preflight',
  childProfileId: 'child-profile-child-ux-preference-preflight',
  label: 'Study PC',
  platform: ParentPlatform.Windows,
} as const;

const EvidenceReference = {
  evidenceReferenceId: 'evidence-child-ux-preference-preflight',
  kind: ParentEvidenceReferenceKind.PolicyDecision,
  observedAt: Timestamp,
} as const;

const ParentAction = {
  actionReferenceId: 'parent-action-child-ux-preference-preflight',
  actor: {
    actorId: 'parent-child-ux-preference-preflight',
    role: ParentActorRole.Parent,
  },
  policyVersion: 'policy-child-ux-preference-preflight-v1',
  createdAt: Timestamp,
} as const;

const ApprovalRequestRef = {
  actionReferenceId: 'approval-request-child-ux-preference-preflight',
  actor: {
    actorId: 'child-device-local-agent',
    role: ParentActorRole.System,
  },
  policyVersion: 'policy-child-ux-preference-preflight-v1',
  createdAt: Timestamp,
} as const;

const BaseChildCard = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  childUxStateId: 'child-ux-limit-reached-preference-preflight',
  device: Device,
  target: {
    targetKind: AppGameChildUxTargetKind.NativeGame,
    targetRef: 'target-native-game-child-ux-preference-preflight',
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
  childReasonReferences: ['child-reason-limit-reached-preference-preflight'],
  childStatusReferences: ['child-status-limit-reached-preference-preflight'],
  approvalRequestRef: ApprovalRequestRef,
  privateDiagnosticReferences: [],
  adapterActionRef: null,
} as const;

const HandoffOptions = {
  generatedAt: Timestamp,
  handoffId: 'app-game-child-ux-preference-preflight-handoff',
  localHandoffRootRef: 'child-device-local-ux-preference-preflight-handoff-root',
} as const;

const ArtifactOptions = {
  generatedAt: Timestamp,
  localArtifactRootRef: 'child-device-local-ux-preference-preflight-artifact-root',
  localArtifactFileRef: 'child-device-local-ux-preference-preflight-artifact-jsonl',
} as const;

const BridgeOptions = {
  family: { familyId: 'family-child-ux-preference-preflight' },
  parentAction: ParentAction,
  generatedAt: Timestamp,
  bridgeId: 'app-game-child-ux-preference-preflight-source-bridge-proof',
  outboxRootRef: 'parent-owned-child-ux-preference-preflight-root',
  outboxFileRef: 'parent-owned-child-ux-preference-preflight-jsonl-ref',
  localDataPathRef: 'parent-owned-child-ux-preference-preflight-data-path-ref',
} as const;

const SchedulerOptions = {
  generatedAt: Timestamp,
  schedulerBridgeId: 'app-game-child-ux-preference-preflight-scheduler-bridge-proof',
  schedulerArtifactRootRef: 'parent-owned-child-ux-preference-preflight-scheduler-root-ref',
  schedulerArtifactRef: 'parent-owned-child-ux-preference-preflight-scheduler-jsonl-ref',
  schedulerNowAt: Timestamp,
} as const;

const PreferencePreflightOptions = {
  generatedAt: Timestamp,
  preferencePreflightId: 'app-game-child-ux-local-outbox-preference-preflight-proof',
  sourceContractRefs: [
    'app-game-child-ux-local-outbox-scheduler-bridge',
    'notification-local-outbox-scheduler-proof',
    'notification-parent-preference-quiet-hours-required',
  ],
} as const;

describe('app/game child UX local outbox preference preflight', () => {
  it('marks scheduled child UX local outbox rows as parent preference required without UI mutation', () => {
    const readModel = buildAppGameChildUxLocalOutboxPreferencePreflightReadModel(
      PreferencePreflightOptions,
      schedulerReadModel([BaseChildCard, nativeAppSubmittedCard()])
    );

    expect(readModel.parentPreferenceRequiredCount).toBe(2);
    expect(readModel.manualRequiredCount).toBe(0);
    expect(readModel.unavailableCount).toBe(0);
    expect(readModel.rows.map((row) => row.status)).toEqual([
      AppGameChildUxLocalOutboxPreferencePreflightStatus.ParentPreferenceRequired,
      AppGameChildUxLocalOutboxPreferencePreflightStatus.ParentPreferenceRequired,
    ]);
    expect(readModel.rows.map((row) => row.parentPreferenceState)).toEqual([
      'manual-setup-required',
      'manual-setup-required',
    ]);
    expect(readModel.rows.map((row) => row.quietHoursDecision)).toEqual(['manual-required', 'manual-required']);
    expect(readModel.rows.every((row) => row.parentPreferenceRequirementRefs.length === 2)).toBe(true);
    expect(readModel.rows.every((row) => row.quietHoursRequirementRefs.length === 1)).toBe(true);
  });

  it('keeps manual and unavailable child UX source rows blocked before preference setup', () => {
    const readModel = buildAppGameChildUxLocalOutboxPreferencePreflightReadModel(
      PreferencePreflightOptions,
      schedulerReadModel([manualCard(), unavailableCard()])
    );

    expect(readModel.rows.map((row) => row.status)).toEqual([
      AppGameChildUxLocalOutboxPreferencePreflightStatus.ManualRequired,
      AppGameChildUxLocalOutboxPreferencePreflightStatus.Unavailable,
    ]);
    expect(readModel.rows.map((row) => row.sourceSchedulerEntryRef)).toEqual([null, null]);
    expect(readModel.rows.map((row) => row.parentPreferenceState)).toEqual([null, null]);
    expect(readModel.rows.map((row) => row.quietHoursDecision)).toEqual([null, null]);
    expect(readModel.rows.map((row) => row.manualProofRequirements.length)).toEqual([1, 1]);
  });

  it('rejects parent UI quiet-hours provider child adapter platform and source-row overclaims', () => {
    const readModel = buildAppGameChildUxLocalOutboxPreferencePreflightReadModel(
      PreferencePreflightOptions,
      schedulerReadModel([BaseChildCard])
    );

    expect(readModel.parentPreferenceUiClaimed).toBe(false);
    expect(readModel.quietHoursTimerRuntimeClaimed).toBe(false);
    expect(readModel.childDeliveryRuntimeClaimed).toBe(false);
    expect(readModel.adapterDispatchClaimed).toBe(false);
    expect(
      AppGameChildUxLocalOutboxPreferencePreflightReadModelSchema.safeParse({
        ...readModel,
        parentPreferenceUiClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameChildUxLocalOutboxPreferencePreflightReadModelSchema.safeParse({
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
    childUxStateId: 'child-ux-native-app-request-submitted-preference-preflight',
    target: {
      targetKind: AppGameChildUxTargetKind.NativeApp,
      targetRef: 'target-native-app-child-ux-preference-preflight',
      childSafeDisplayLabelToken: AppGameChildUxCopyToken.RequestSubmittedTitle,
    },
    surfaceState: AppGameChildUxSurfaceState.RequestSubmitted,
    claimState: AppGameChildUxClaimState.RequestSubmitted,
    titleToken: AppGameChildUxCopyToken.RequestSubmittedTitle,
    bodyToken: AppGameChildUxCopyToken.RequestSubmittedBody,
    primaryAction: AppGameChildUxPrimaryAction.Dismiss,
    primaryActionToken: AppGameChildUxCopyToken.DismissAction,
    childReasonReferences: ['child-reason-request-submitted-preference-preflight'],
    childStatusReferences: ['child-status-request-submitted-preference-preflight'],
    approvalRequestRef: null,
  } as const;
}

function manualCard() {
  return {
    ...BaseChildCard,
    childUxStateId: 'child-ux-manual-required-preference-preflight',
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
    childUxStateId: 'child-ux-unavailable-preference-preflight',
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
