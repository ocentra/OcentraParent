import { describe, expect, it } from 'vitest';
import { AppGameChildUxCardSchema } from '@ocentra-parent/schema-domain/app-game-child-facing-ux';
import {
  AppGameChildUxCapabilityState,
  AppGameChildUxClaimState,
  AppGameChildUxCopyToken,
  AppGameChildUxExplanationSource,
  AppGameChildUxPrimaryAction,
  AppGameChildUxSurfaceState,
  AppGameChildUxTargetKind,
} from '@ocentra-parent/schema-domain/app-game-child-facing-ux-rules';
import { buildAppGameChildUxHandoffReadModel } from '@ocentra-parent/schema-domain/app-game-child-facing-ux-handoff';
import { buildAppGameChildUxLocalHandoffArtifactReadModel } from '@ocentra-parent/schema-domain/app-game-child-facing-ux-local-handoff';
import { buildAppGameChildUxLocalOutboxBridgeReadModel } from '@ocentra-parent/schema-domain/app-game-child-facing-ux-local-outbox-bridge';
import {
  buildAppGameChildUxLocalOutboxSchedulerBridgeReadModel,
  type AppGameChildUxLocalOutboxSchedulerBridgeReadModel,
} from '@ocentra-parent/schema-domain/app-game-child-facing-ux-local-outbox-scheduler-bridge';
import {
  AppGameChildUxParentPreferenceSetupDraftReadModelSchema,
  AppGameChildUxParentPreferenceSetupDraftRowSchema,
  AppGameChildUxParentPreferenceSetupDraftStatus,
} from '@ocentra-parent/schema-domain/app-game-child-facing-ux-parent-preference-setup-draft';
import {
  ParentActorRole,
  ParentContractSchemaVersion,
  ParentEvidenceReferenceKind,
  ParentPlatform,
} from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  buildAppGameChildUxLocalOutboxParentSurfaceIntentReadModel,
  buildAppGameChildUxLocalOutboxPreferencePreflightReadModel,
  buildAppGameChildUxLocalOutboxPreferenceStatusHandoffReadModel,
  buildAppGameChildUxLocalOutboxProviderPreflightReadModel,
  buildAppGameChildUxLocalOutboxProviderStatusHandoffReadModel,
  buildAppGameChildUxParentPreferenceSetupDraftReadModel,
} from './app-game-child-facing-ux-local-outbox-centralized-schema-fixtures';

const Timestamp = '2026-06-07T23:17:00Z';

const Device = {
  deviceId: 'device-child-ux-preference-setup-draft',
  childProfileId: 'child-profile-child-ux-preference-setup-draft',
  label: 'Study PC',
  platform: ParentPlatform.Windows,
} as const;

const EvidenceReference = {
  evidenceReferenceId: 'evidence-child-ux-preference-setup-draft',
  kind: ParentEvidenceReferenceKind.PolicyDecision,
  observedAt: Timestamp,
} as const;

const ParentAction = {
  actionReferenceId: 'parent-action-child-ux-preference-setup-draft',
  actor: {
    actorId: 'parent-child-ux-preference-setup-draft',
    role: ParentActorRole.Parent,
  },
  policyVersion: 'policy-child-ux-preference-setup-draft-v1',
  createdAt: Timestamp,
} as const;

const ApprovalRequestRef = {
  actionReferenceId: 'approval-request-child-ux-preference-setup-draft',
  actor: {
    actorId: 'child-device-local-agent',
    role: ParentActorRole.System,
  },
  policyVersion: 'policy-child-ux-preference-setup-draft-v1',
  createdAt: Timestamp,
} as const;

const BaseChildCard = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  childUxStateId: 'child-ux-limit-reached-preference-setup-draft',
  device: Device,
  target: {
    targetKind: AppGameChildUxTargetKind.NativeGame,
    targetRef: 'target-native-game-child-ux-preference-setup-draft',
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
  childReasonReferences: ['child-reason-limit-reached-preference-setup-draft'],
  childStatusReferences: ['child-status-limit-reached-preference-setup-draft'],
  approvalRequestRef: ApprovalRequestRef,
  privateDiagnosticReferences: [],
  adapterActionRef: null,
} as const;

const HandoffOptions = {
  generatedAt: Timestamp,
  handoffId: 'app-game-child-ux-preference-setup-draft-handoff',
  localHandoffRootRef: 'child-device-local-ux-preference-setup-draft-root',
} as const;

const ArtifactOptions = {
  generatedAt: Timestamp,
  localArtifactRootRef: 'child-device-local-ux-preference-setup-draft-artifact-root',
  localArtifactFileRef: 'child-device-local-ux-preference-setup-draft-artifact-jsonl',
} as const;

const BridgeOptions = {
  family: { familyId: 'family-child-ux-preference-setup-draft' },
  parentAction: ParentAction,
  generatedAt: Timestamp,
  bridgeId: 'app-game-child-ux-preference-setup-draft-source-bridge-proof',
  outboxRootRef: 'parent-owned-child-ux-preference-setup-draft-root',
  outboxFileRef: 'parent-owned-child-ux-preference-setup-draft-jsonl-ref',
  localDataPathRef: 'parent-owned-child-ux-preference-setup-draft-data-path-ref',
} as const;

const SchedulerOptions = {
  generatedAt: Timestamp,
  schedulerBridgeId: 'app-game-child-ux-preference-setup-draft-scheduler-bridge-proof',
  schedulerArtifactRootRef: 'parent-owned-child-ux-preference-setup-draft-scheduler-root-ref',
  schedulerArtifactRef: 'parent-owned-child-ux-preference-setup-draft-scheduler-jsonl-ref',
  schedulerNowAt: Timestamp,
} as const;

const ProviderPreflightOptions = {
  generatedAt: Timestamp,
  providerPreflightId: 'app-game-child-ux-preference-setup-draft-provider-preflight-proof',
  sourceContractRefs: ['app-game-child-ux-local-outbox-provider-preflight'],
} as const;

const ProviderStatusOptions = {
  generatedAt: Timestamp,
  handoffId: 'app-game-child-ux-preference-setup-draft-provider-status-proof',
  sourceContractRefs: ['app-game-child-ux-local-outbox-provider-status-handoff'],
} as const;

const PreferencePreflightOptions = {
  generatedAt: Timestamp,
  preferencePreflightId: 'app-game-child-ux-preference-setup-draft-preference-preflight-proof',
  sourceContractRefs: ['app-game-child-ux-local-outbox-preference-preflight'],
} as const;

const PreferenceStatusOptions = {
  generatedAt: Timestamp,
  handoffId: 'app-game-child-ux-preference-setup-draft-preference-status-proof',
  sourceContractRefs: ['app-game-child-ux-local-outbox-preference-status-handoff'],
} as const;

const ParentSurfaceOptions = {
  generatedAt: Timestamp,
  intentId: 'app-game-child-ux-local-outbox-parent-surface-preference-setup-draft-proof',
  sourceContractRefs: [
    'app-game-child-ux-local-outbox-provider-status-handoff',
    'app-game-child-ux-local-outbox-preference-status-handoff',
  ],
} as const;

const DraftOptions = {
  generatedAt: Timestamp,
  draftId: 'app-game-child-ux-parent-preference-setup-draft-proof',
  sourceContractRefs: ['app-game-child-ux-local-outbox-parent-surface-intent'],
} as const;

describe('app/game child UX parent preference setup draft', () => {
  it('derives parent-safe preference setup draft rows from child UX parent-surface intent rows', () => {
    const readModel = buildPreferenceSetupDraftReadModel([BaseChildCard, nativeAppSubmittedCard()]);

    expect(readModel.draftReadyCount).toBe(2);
    expect(readModel.unavailableVisibleCount).toBe(0);
    expect(readModel.rows.map((row) => row.draftStatus)).toEqual([
      AppGameChildUxParentPreferenceSetupDraftStatus.DraftReady,
      AppGameChildUxParentPreferenceSetupDraftStatus.DraftReady,
    ]);
    expect(readModel.rows.every((row) => row.preferenceRequirementRefs.length > 0)).toBe(true);
    expect(readModel.rows.every((row) => row.quietHoursRequirementRefs.length > 0)).toBe(true);
    expect(readModel.rows.every((row) => row.parentSafeDrillInRefs.length === 2)).toBe(true);
  });

  it('keeps unavailable rows visible without claiming parent preference UI or mutation', () => {
    const readModel = buildPreferenceSetupDraftReadModel([unavailableCard()]);
    const firstRow = readModel.rows[0];

    expect(readModel.draftReadyCount).toBe(0);
    expect(readModel.unavailableVisibleCount).toBe(1);
    expect(firstRow.draftStatus).toBe(AppGameChildUxParentPreferenceSetupDraftStatus.UnavailableVisible);
    expect(firstRow.preferenceRequirementRefs).toHaveLength(0);
    expect(firstRow.quietHoursRequirementRefs).toHaveLength(0);
    expect(readModel.parentPreferenceUiRendered).toBe(false);
    expect(readModel.parentFrequencyControlUiRendered).toBe(false);
    expect(readModel.parentPreferenceMutationClaimed).toBe(false);
    expect(readModel.notificationRuleMutationClaimed).toBe(false);
    expect(readModel.providerDeliveryRuntimeClaimed).toBe(false);
    expect(readModel.childDeliveryClaimed).toBe(false);
    expect(readModel.adapterDispatchClaimed).toBe(false);
    expect(readModel.platformEnforcementClaimed).toBe(false);
  });

  it('rejects parent preference setup overclaims and incomplete draft-ready rows', () => {
    const readModel = buildPreferenceSetupDraftReadModel([BaseChildCard]);
    const firstRow = readModel.rows[0];

    expect(
      AppGameChildUxParentPreferenceSetupDraftReadModelSchema.safeParse({
        ...readModel,
        parentPreferenceMutationClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameChildUxParentPreferenceSetupDraftRowSchema.safeParse({
        ...firstRow,
        notificationRuleMutationClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameChildUxParentPreferenceSetupDraftRowSchema.safeParse({
        ...firstRow,
        preferenceRequirementRefs: [],
      }).success
    ).toBe(false);
  });
});

function buildPreferenceSetupDraftReadModel(cards: ReadonlyArray<unknown>) {
  const parentSurface = buildAppGameChildUxLocalOutboxParentSurfaceIntentReadModel(
    ParentSurfaceOptions,
    providerStatusReadModel(cards),
    preferenceStatusReadModel(cards)
  );
  return buildAppGameChildUxParentPreferenceSetupDraftReadModel(DraftOptions, parentSurface);
}

function providerStatusReadModel(cards: ReadonlyArray<unknown>) {
  const scheduler = schedulerReadModel(cards);
  const preflight = buildAppGameChildUxLocalOutboxProviderPreflightReadModel(ProviderPreflightOptions, scheduler);
  return buildAppGameChildUxLocalOutboxProviderStatusHandoffReadModel(ProviderStatusOptions, preflight);
}

function preferenceStatusReadModel(cards: ReadonlyArray<unknown>) {
  const scheduler = schedulerReadModel(cards);
  const preflight = buildAppGameChildUxLocalOutboxPreferencePreflightReadModel(PreferencePreflightOptions, scheduler);
  return buildAppGameChildUxLocalOutboxPreferenceStatusHandoffReadModel(PreferenceStatusOptions, preflight);
}

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
    childUxStateId: 'child-ux-native-app-request-submitted-preference-setup-draft',
    target: {
      targetKind: AppGameChildUxTargetKind.NativeApp,
      targetRef: 'target-native-app-child-ux-preference-setup-draft',
      childSafeDisplayLabelToken: AppGameChildUxCopyToken.RequestSubmittedTitle,
    },
    surfaceState: AppGameChildUxSurfaceState.RequestSubmitted,
    claimState: AppGameChildUxClaimState.RequestSubmitted,
    titleToken: AppGameChildUxCopyToken.RequestSubmittedTitle,
    bodyToken: AppGameChildUxCopyToken.RequestSubmittedBody,
    primaryAction: AppGameChildUxPrimaryAction.Dismiss,
    primaryActionToken: AppGameChildUxCopyToken.DismissAction,
    childReasonReferences: ['child-reason-request-submitted-preference-setup-draft'],
    childStatusReferences: ['child-status-request-submitted-preference-setup-draft'],
    approvalRequestRef: null,
  } as const;
}

function unavailableCard() {
  return {
    ...BaseChildCard,
    childUxStateId: 'child-ux-unavailable-preference-setup-draft',
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
