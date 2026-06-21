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
} from '@ocentra-parent/schema-domain/app-game-child-facing-ux-rules';
import { buildAppGameChildUxHandoffReadModel } from '../../src/app-game-child-facing-ux-handoff';
import { buildAppGameChildUxLocalHandoffArtifactReadModel } from '../../src/app-game-child-facing-ux-local-handoff';
import { buildAppGameChildUxLocalOutboxProviderPreflightReadModel } from '../../src/app-game-child-facing-ux-local-outbox-provider-preflight';
import { buildAppGameChildUxLocalOutboxProviderStatusHandoffReadModel } from '../../src/app-game-child-facing-ux-local-outbox-provider-status-handoff';
import { buildAppGameChildUxLocalOutboxPreferencePreflightReadModel } from '../../src/app-game-child-facing-ux-local-outbox-preference-preflight';
import { buildAppGameChildUxLocalOutboxPreferenceStatusHandoffReadModel } from '../../src/app-game-child-facing-ux-local-outbox-preference-status-handoff';
import {
  buildAppGameChildUxLocalOutboxParentSurfaceIntentReadModel,
} from '../../src/app-game-child-facing-ux-local-outbox-parent-surface-intent';
import { buildAppGameChildUxLocalOutboxBridgeReadModel } from '@ocentra-parent/schema-domain/app-game-child-facing-ux-local-outbox-bridge';
import {
  buildAppGameChildUxLocalOutboxSchedulerBridgeReadModel,
  type AppGameChildUxLocalOutboxSchedulerBridgeReadModel,
} from '@ocentra-parent/schema-domain/app-game-child-facing-ux-local-outbox-scheduler-bridge';
import {
  AppGameChildUxLocalOutboxParentSurfaceIntentReadModelSchema,
  AppGameChildUxLocalOutboxParentSurfaceIntentRowSchema,
} from '@ocentra-parent/schema-domain/app-game-child-facing-ux-local-outbox-parent-surface-intent';
import {
  ParentActorRole,
  ParentContractSchemaVersion,
  ParentEvidenceReferenceKind,
  ParentPlatform,
} from '@ocentra-parent/schema-domain/family-reference-primitives';

const Timestamp = '2026-06-07T22:14:00Z';

const Device = {
  deviceId: 'device-child-ux-parent-surface',
  childProfileId: 'child-profile-child-ux-parent-surface',
  label: 'Study PC',
  platform: ParentPlatform.Windows,
} as const;

const EvidenceReference = {
  evidenceReferenceId: 'evidence-child-ux-parent-surface',
  kind: ParentEvidenceReferenceKind.PolicyDecision,
  observedAt: Timestamp,
} as const;

const ParentAction = {
  actionReferenceId: 'parent-action-child-ux-parent-surface',
  actor: {
    actorId: 'parent-child-ux-parent-surface',
    role: ParentActorRole.Parent,
  },
  policyVersion: 'policy-child-ux-parent-surface-v1',
  createdAt: Timestamp,
} as const;

const ApprovalRequestRef = {
  actionReferenceId: 'approval-request-child-ux-parent-surface',
  actor: {
    actorId: 'child-device-local-agent',
    role: ParentActorRole.System,
  },
  policyVersion: 'policy-child-ux-parent-surface-v1',
  createdAt: Timestamp,
} as const;

const BaseChildCard = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  childUxStateId: 'child-ux-limit-reached-parent-surface',
  device: Device,
  target: {
    targetKind: AppGameChildUxTargetKind.NativeGame,
    targetRef: 'target-native-game-child-ux-parent-surface',
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
  childReasonReferences: ['child-reason-limit-reached-parent-surface'],
  childStatusReferences: ['child-status-limit-reached-parent-surface'],
  approvalRequestRef: ApprovalRequestRef,
  privateDiagnosticReferences: [],
  adapterActionRef: null,
} as const;

const HandoffOptions = {
  generatedAt: Timestamp,
  handoffId: 'app-game-child-ux-parent-surface-handoff',
  localHandoffRootRef: 'child-device-local-ux-parent-surface-handoff-root',
} as const;

const ArtifactOptions = {
  generatedAt: Timestamp,
  localArtifactRootRef: 'child-device-local-ux-parent-surface-artifact-root',
  localArtifactFileRef: 'child-device-local-ux-parent-surface-artifact-jsonl',
} as const;

const BridgeOptions = {
  family: { familyId: 'family-child-ux-parent-surface' },
  parentAction: ParentAction,
  generatedAt: Timestamp,
  bridgeId: 'app-game-child-ux-parent-surface-source-bridge-proof',
  outboxRootRef: 'parent-owned-child-ux-parent-surface-root',
  outboxFileRef: 'parent-owned-child-ux-parent-surface-jsonl-ref',
  localDataPathRef: 'parent-owned-child-ux-parent-surface-data-path-ref',
} as const;

const SchedulerOptions = {
  generatedAt: Timestamp,
  schedulerBridgeId: 'app-game-child-ux-parent-surface-scheduler-bridge-proof',
  schedulerArtifactRootRef: 'parent-owned-child-ux-parent-surface-scheduler-root-ref',
  schedulerArtifactRef: 'parent-owned-child-ux-parent-surface-scheduler-jsonl-ref',
  schedulerNowAt: Timestamp,
} as const;

const ProviderPreflightOptions = {
  generatedAt: Timestamp,
  providerPreflightId: 'app-game-child-ux-parent-surface-provider-preflight-proof',
  sourceContractRefs: ['app-game-child-ux-local-outbox-provider-preflight'],
} as const;

const ProviderStatusOptions = {
  generatedAt: Timestamp,
  handoffId: 'app-game-child-ux-parent-surface-provider-status-proof',
  sourceContractRefs: ['app-game-child-ux-local-outbox-provider-status-handoff'],
} as const;

const PreferencePreflightOptions = {
  generatedAt: Timestamp,
  preferencePreflightId: 'app-game-child-ux-parent-surface-preference-preflight-proof',
  sourceContractRefs: ['app-game-child-ux-local-outbox-preference-preflight'],
} as const;

const PreferenceStatusOptions = {
  generatedAt: Timestamp,
  handoffId: 'app-game-child-ux-parent-surface-preference-status-proof',
  sourceContractRefs: ['app-game-child-ux-local-outbox-preference-status-handoff'],
} as const;

const ParentSurfaceOptions = {
  generatedAt: Timestamp,
  intentId: 'app-game-child-ux-local-outbox-parent-surface-intent-proof',
  sourceContractRefs: [
    'app-game-child-ux-local-outbox-provider-status-handoff',
    'app-game-child-ux-local-outbox-preference-status-handoff',
  ],
} as const;

describe('app/game child UX local outbox parent surface intent', () => {
  it('combines child UX provider and preference status rows into redacted parent surface rows', () => {
    const readModel = buildParentSurfaceIntentReadModel([BaseChildCard, nativeAppSubmittedCard()]);

    expect(readModel.manualActionRequiredCount).toBe(2);
    expect(readModel.unavailableVisibleCount).toBe(0);
    expect(readModel.historyVisibleCount).toBe(2);
    expect(readModel.preferenceSetupRequiredCount).toBe(2);
    expect(readModel.rows.map((row) => row.parentSurfaceStatus)).toEqual([
      'manual-action-required',
      'manual-action-required',
    ]);
    expect(readModel.rows.map((row) => row.preferenceVisibility)).toEqual([
      'preference-setup-required',
      'preference-setup-required',
    ]);
  });

  it('preserves drill-in refs and keeps UI delivery adapter and platform claims false', () => {
    const readModel = buildParentSurfaceIntentReadModel([BaseChildCard]);
    const firstRow = readModel.rows[0];

    expect(firstRow.sourceSchedulerEntryRef).not.toBeNull();
    expect(firstRow.sourceOutboxRecordRef).not.toBeNull();
    expect(firstRow.drillInRefs).toHaveLength(2);
    expect(firstRow.auditRefs).toHaveLength(2);
    expect(firstRow.manualProofRequirements.length).toBeGreaterThanOrEqual(6);
    expect(readModel.parentNotificationUiRendered).toBe(false);
    expect(readModel.parentPreferenceMutationClaimed).toBe(false);
    expect(readModel.providerDeliveryRuntimeClaimed).toBe(false);
    expect(readModel.childDeliveryClaimed).toBe(false);
    expect(readModel.adapterDispatchClaimed).toBe(false);
    expect(readModel.platformEnforcementClaimed).toBe(false);
  });

  it('rejects parent surface overclaims and mismatched upstream row counts', () => {
    const readModel = buildParentSurfaceIntentReadModel([BaseChildCard, unavailableCard()]);
    const unavailableRow = readModel.rows[1];

    expect(readModel.unavailableVisibleCount).toBe(1);
    expect(unavailableRow.preferenceVisibility).toBe('preference-disabled-visible');
    expect(
      AppGameChildUxLocalOutboxParentSurfaceIntentReadModelSchema.safeParse({
        ...readModel,
        parentNotificationUiRendered: true,
      }).success
    ).toBe(false);
    expect(
      AppGameChildUxLocalOutboxParentSurfaceIntentRowSchema.safeParse({
        ...unavailableRow,
        adapterDispatchClaimed: true,
      }).success
    ).toBe(false);
    expect(() =>
      buildAppGameChildUxLocalOutboxParentSurfaceIntentReadModel(
        ParentSurfaceOptions,
        providerStatusReadModel([BaseChildCard]),
        preferenceStatusReadModel([BaseChildCard, nativeAppSubmittedCard()])
      )
    ).toThrow('Expected child UX parent-surface inputs to have matching row counts');
  });
});

function buildParentSurfaceIntentReadModel(cards: ReadonlyArray<unknown>) {
  return buildAppGameChildUxLocalOutboxParentSurfaceIntentReadModel(
    ParentSurfaceOptions,
    providerStatusReadModel(cards),
    preferenceStatusReadModel(cards)
  );
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
    childUxStateId: 'child-ux-native-app-request-submitted-parent-surface',
    target: {
      targetKind: AppGameChildUxTargetKind.NativeApp,
      targetRef: 'target-native-app-child-ux-parent-surface',
      childSafeDisplayLabelToken: AppGameChildUxCopyToken.RequestSubmittedTitle,
    },
    surfaceState: AppGameChildUxSurfaceState.RequestSubmitted,
    claimState: AppGameChildUxClaimState.RequestSubmitted,
    titleToken: AppGameChildUxCopyToken.RequestSubmittedTitle,
    bodyToken: AppGameChildUxCopyToken.RequestSubmittedBody,
    primaryAction: AppGameChildUxPrimaryAction.Dismiss,
    primaryActionToken: AppGameChildUxCopyToken.DismissAction,
    childReasonReferences: ['child-reason-request-submitted-parent-surface'],
    childStatusReferences: ['child-status-request-submitted-parent-surface'],
    approvalRequestRef: null,
  } as const;
}

function unavailableCard() {
  return {
    ...BaseChildCard,
    childUxStateId: 'child-ux-unavailable-parent-surface',
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
