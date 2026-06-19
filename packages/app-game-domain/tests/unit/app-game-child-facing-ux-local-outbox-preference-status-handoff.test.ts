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
import { buildAppGameChildUxLocalOutboxPreferencePreflightReadModel } from '../../src/app-game-child-facing-ux-local-outbox-preference-preflight';
import {
  AppGameChildUxLocalOutboxPreferenceStatusHandoffReadModelSchema,
  AppGameChildUxLocalOutboxPreferenceStatusHandoffRowSchema,
  buildAppGameChildUxLocalOutboxPreferenceStatusHandoffReadModel,
} from '../../src/app-game-child-facing-ux-local-outbox-preference-status-handoff';
import {
  ParentActorRole,
  ParentContractSchemaVersion,
  ParentEvidenceReferenceKind,
  ParentPlatform,
} from '@ocentra-parent/schema-domain/family-reference-primitives';

const Timestamp = '2026-06-07T22:04:00Z';

const Device = {
  deviceId: 'device-child-ux-preference-status',
  childProfileId: 'child-profile-child-ux-preference-status',
  label: 'Study PC',
  platform: ParentPlatform.Windows,
} as const;

const EvidenceReference = {
  evidenceReferenceId: 'evidence-child-ux-preference-status',
  kind: ParentEvidenceReferenceKind.PolicyDecision,
  observedAt: Timestamp,
} as const;

const ParentAction = {
  actionReferenceId: 'parent-action-child-ux-preference-status',
  actor: {
    actorId: 'parent-child-ux-preference-status',
    role: ParentActorRole.Parent,
  },
  policyVersion: 'policy-child-ux-preference-status-v1',
  createdAt: Timestamp,
} as const;

const ApprovalRequestRef = {
  actionReferenceId: 'approval-request-child-ux-preference-status',
  actor: {
    actorId: 'child-device-local-agent',
    role: ParentActorRole.System,
  },
  policyVersion: 'policy-child-ux-preference-status-v1',
  createdAt: Timestamp,
} as const;

const BaseChildCard = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  childUxStateId: 'child-ux-limit-reached-preference-status',
  device: Device,
  target: {
    targetKind: AppGameChildUxTargetKind.NativeGame,
    targetRef: 'target-native-game-child-ux-preference-status',
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
  childReasonReferences: ['child-reason-limit-reached-preference-status'],
  childStatusReferences: ['child-status-limit-reached-preference-status'],
  approvalRequestRef: ApprovalRequestRef,
  privateDiagnosticReferences: [],
  adapterActionRef: null,
} as const;

const HandoffOptions = {
  generatedAt: Timestamp,
  handoffId: 'app-game-child-ux-preference-status-handoff',
  localHandoffRootRef: 'child-device-local-ux-preference-status-handoff-root',
} as const;

const ArtifactOptions = {
  generatedAt: Timestamp,
  localArtifactRootRef: 'child-device-local-ux-preference-status-artifact-root',
  localArtifactFileRef: 'child-device-local-ux-preference-status-artifact-jsonl',
} as const;

const BridgeOptions = {
  family: { familyId: 'family-child-ux-preference-status' },
  parentAction: ParentAction,
  generatedAt: Timestamp,
  bridgeId: 'app-game-child-ux-preference-status-source-bridge-proof',
  outboxRootRef: 'parent-owned-child-ux-preference-status-root',
  outboxFileRef: 'parent-owned-child-ux-preference-status-jsonl-ref',
  localDataPathRef: 'parent-owned-child-ux-preference-status-data-path-ref',
} as const;

const SchedulerOptions = {
  generatedAt: Timestamp,
  schedulerBridgeId: 'app-game-child-ux-preference-status-scheduler-bridge-proof',
  schedulerArtifactRootRef: 'parent-owned-child-ux-preference-status-scheduler-root-ref',
  schedulerArtifactRef: 'parent-owned-child-ux-preference-status-scheduler-jsonl-ref',
  schedulerNowAt: Timestamp,
} as const;

const PreferencePreflightOptions = {
  generatedAt: Timestamp,
  preferencePreflightId: 'app-game-child-ux-local-outbox-preference-status-preflight-proof',
  sourceContractRefs: ['app-game-child-ux-local-outbox-preference-preflight'],
} as const;

const PreferenceStatusOptions = {
  generatedAt: Timestamp,
  handoffId: 'app-game-child-ux-local-outbox-preference-status-proof',
  sourceContractRefs: [
    'app-game-child-ux-local-outbox-preference-preflight',
    'v3-notification-rule-provider-retry-contract',
  ],
} as const;

describe('app/game child UX local outbox preference status handoff', () => {
  it('maps scheduled child UX preference preflight rows into V3 manual setup status entries', () => {
    const readModel = buildAppGameChildUxLocalOutboxPreferenceStatusHandoffReadModel(
      PreferenceStatusOptions,
      preferencePreflightReadModel([BaseChildCard, nativeAppSubmittedCard()])
    );

    expect(readModel.parentPreferenceManualSetupRequiredCount).toBe(2);
    expect(readModel.quietHoursManualRequiredCount).toBe(2);
    expect(readModel.preferenceStatusUnavailableCount).toBe(0);
    expect(readModel.rows.map((row) => row.notificationPreferenceStatusEntry.parentPreferenceState)).toEqual([
      'manual-setup-required',
      'manual-setup-required',
    ]);
    expect(readModel.rows.map((row) => row.notificationPreferenceStatusEntry.quietHoursDecision)).toEqual([
      'manual-required',
      'manual-required',
    ]);
    expect(readModel.rows.every((row) => row.notificationPreferenceStatusEntry.deliveryAttemptExecuted === false)).toBe(
      true
    );
  });

  it('keeps manual and unavailable child UX source rows blocked as manual or disabled status entries', () => {
    const readModel = buildAppGameChildUxLocalOutboxPreferenceStatusHandoffReadModel(
      PreferenceStatusOptions,
      preferencePreflightReadModel([manualCard(), unavailableCard()])
    );

    expect(readModel.rows.map((row) => row.notificationPreferenceStatusEntry.parentPreferenceState)).toEqual([
      'manual-setup-required',
      'channel-disabled',
    ]);
    expect(readModel.rows.map((row) => row.notificationPreferenceStatusEntry.deliveryResultState)).toEqual([
      'manual-required',
      'not-sent',
    ]);
    expect(readModel.rows.map((row) => row.notificationPreferenceStatusEntry.providerReceiptRefs)).toEqual([[], []]);
  });

  it('rejects UI mutation delivery receipt adapter platform and source-row overclaims', () => {
    const readModel = buildAppGameChildUxLocalOutboxPreferenceStatusHandoffReadModel(
      PreferenceStatusOptions,
      preferencePreflightReadModel([BaseChildCard])
    );
    const row = readModel.rows[0];

    expect(readModel.parentPreferenceMutationClaimed).toBe(false);
    expect(readModel.parentPreferenceUiClaimed).toBe(false);
    expect(readModel.providerDeliveryRuntimeClaimed).toBe(false);
    expect(readModel.childDeliveryClaimed).toBe(false);
    expect(
      AppGameChildUxLocalOutboxPreferenceStatusHandoffReadModelSchema.safeParse({
        ...readModel,
        parentPreferenceMutationClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameChildUxLocalOutboxPreferenceStatusHandoffReadModelSchema.safeParse({
        ...readModel,
        rawPrivateSourceRowsIncluded: true,
      }).success
    ).toBe(false);
    expect(
      AppGameChildUxLocalOutboxPreferenceStatusHandoffRowSchema.safeParse({
        ...row,
        notificationPreferenceStatusEntry: {
          ...row.notificationPreferenceStatusEntry,
          deliveryAttemptExecuted: true,
        },
      }).success
    ).toBe(false);
  });
});

function preferencePreflightReadModel(cards: ReadonlyArray<unknown>) {
  return buildAppGameChildUxLocalOutboxPreferencePreflightReadModel(
    PreferencePreflightOptions,
    schedulerReadModel(cards)
  );
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
    childUxStateId: 'child-ux-native-app-request-submitted-preference-status',
    target: {
      targetKind: AppGameChildUxTargetKind.NativeApp,
      targetRef: 'target-native-app-child-ux-preference-status',
      childSafeDisplayLabelToken: AppGameChildUxCopyToken.RequestSubmittedTitle,
    },
    surfaceState: AppGameChildUxSurfaceState.RequestSubmitted,
    claimState: AppGameChildUxClaimState.RequestSubmitted,
    titleToken: AppGameChildUxCopyToken.RequestSubmittedTitle,
    bodyToken: AppGameChildUxCopyToken.RequestSubmittedBody,
    primaryAction: AppGameChildUxPrimaryAction.Dismiss,
    primaryActionToken: AppGameChildUxCopyToken.DismissAction,
    childReasonReferences: ['child-reason-request-submitted-preference-status'],
    childStatusReferences: ['child-status-request-submitted-preference-status'],
    approvalRequestRef: null,
  } as const;
}

function manualCard() {
  return {
    ...BaseChildCard,
    childUxStateId: 'child-ux-manual-required-preference-status',
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
    childUxStateId: 'child-ux-unavailable-preference-status',
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
