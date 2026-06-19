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
import { buildAppGameChildUxLocalOutboxSchedulerBridgeReadModel } from '../../src/app-game-child-facing-ux-local-outbox-scheduler-bridge';
import { buildAppGameChildUxLocalOutboxProviderPreflightReadModel } from '../../src/app-game-child-facing-ux-local-outbox-provider-preflight';
import {
  AppGameChildUxLocalOutboxProviderStatusHandoffReadModelSchema,
  AppGameChildUxLocalOutboxProviderStatusHandoffRowSchema,
  buildAppGameChildUxLocalOutboxProviderStatusHandoffReadModel,
} from '../../src/app-game-child-facing-ux-local-outbox-provider-status-handoff';
import {
  ParentActorRole,
  ParentContractSchemaVersion,
  ParentEvidenceReferenceKind,
  ParentPlatform,
} from '@ocentra-parent/schema-domain/family-reference-primitives';

const Timestamp = '2026-06-07T21:45:00Z';

const Device = {
  deviceId: 'device-child-ux-provider-status',
  childProfileId: 'child-profile-child-ux-provider-status',
  label: 'Study PC',
  platform: ParentPlatform.Windows,
} as const;

const EvidenceReference = {
  evidenceReferenceId: 'evidence-child-ux-provider-status',
  kind: ParentEvidenceReferenceKind.PolicyDecision,
  observedAt: Timestamp,
} as const;

const ParentAction = {
  actionReferenceId: 'parent-action-child-ux-provider-status',
  actor: {
    actorId: 'parent-child-ux-provider-status',
    role: ParentActorRole.Parent,
  },
  policyVersion: 'policy-child-ux-provider-status-v1',
  createdAt: Timestamp,
} as const;

const ApprovalRequestRef = {
  actionReferenceId: 'approval-request-child-ux-provider-status',
  actor: {
    actorId: 'child-device-local-agent',
    role: ParentActorRole.System,
  },
  policyVersion: 'policy-child-ux-provider-status-v1',
  createdAt: Timestamp,
} as const;

const BaseChildCard = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  childUxStateId: 'child-ux-limit-reached-provider-status',
  device: Device,
  target: {
    targetKind: AppGameChildUxTargetKind.NativeGame,
    targetRef: 'target-native-game-child-ux-provider-status',
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
  childReasonReferences: ['child-reason-limit-reached-provider-status'],
  childStatusReferences: ['child-status-limit-reached-provider-status'],
  approvalRequestRef: ApprovalRequestRef,
  privateDiagnosticReferences: [],
  adapterActionRef: null,
} as const;

const HandoffOptions = {
  generatedAt: Timestamp,
  handoffId: 'app-game-child-ux-provider-status-handoff',
  localHandoffRootRef: 'child-device-local-ux-provider-status-handoff-root',
} as const;

const ArtifactOptions = {
  generatedAt: Timestamp,
  localArtifactRootRef: 'child-device-local-ux-provider-status-artifact-root',
  localArtifactFileRef: 'child-device-local-ux-provider-status-artifact-jsonl',
} as const;

const BridgeOptions = {
  family: { familyId: 'family-child-ux-provider-status' },
  parentAction: ParentAction,
  generatedAt: Timestamp,
  bridgeId: 'app-game-child-ux-provider-status-source-bridge-proof',
  outboxRootRef: 'parent-owned-child-ux-provider-status-root',
  outboxFileRef: 'parent-owned-child-ux-provider-status-jsonl-ref',
  localDataPathRef: 'parent-owned-child-ux-provider-status-data-path-ref',
} as const;

const SchedulerOptions = {
  generatedAt: Timestamp,
  schedulerBridgeId: 'app-game-child-ux-provider-status-scheduler-bridge-proof',
  schedulerArtifactRootRef: 'parent-owned-child-ux-provider-status-scheduler-root-ref',
  schedulerArtifactRef: 'parent-owned-child-ux-provider-status-scheduler-jsonl-ref',
  schedulerNowAt: Timestamp,
} as const;

const ProviderPreflightOptions = {
  generatedAt: Timestamp,
  providerPreflightId: 'app-game-child-ux-provider-status-preflight-proof',
  sourceContractRefs: [
    'app-game-child-ux-local-outbox-scheduler-bridge',
    'notification-local-outbox-scheduler-proof',
    'notification-provider-adapter-boundary-required',
  ],
} as const;

const ProviderStatusOptions = {
  generatedAt: Timestamp,
  handoffId: 'app-game-child-ux-local-outbox-provider-status-handoff-proof',
  sourceContractRefs: [
    'app-game-child-ux-local-outbox-provider-preflight',
    'v0-8-notification-provider-status-boundary',
    'notifications-expectation-provider-boundary',
  ],
} as const;

describe('app/game child UX local outbox provider status handoff', () => {
  it('maps provider-preflight rows into manual-required and unavailable provider status boundary rows', () => {
    const readModel = buildProviderStatusHandoffReadModel([BaseChildCard, nativeAppSubmittedCard(), unavailableCard()]);

    expect(readModel.providerStatusManualRequiredCount).toBe(2);
    expect(readModel.providerStatusUnavailableCount).toBe(1);
    expect(readModel.rows.map((row) => row.providerStatusBoundaryEntry.providerStatus)).toEqual([
      'manual-required',
      'manual-required',
      'unavailable',
    ]);
    expect(readModel.rows.map((row) => row.providerStatusBoundaryEntry.statusProofState)).toEqual([
      'manual-action-required',
      'manual-action-required',
      'provider-unavailable-contract',
    ]);
    expect(readModel.providerStatusBoundaryCoverageRefs).toEqual([
      'notification-provider-queued-contract',
      'notification-provider-delivered-receipt-required',
      'notification-provider-failed-contract',
      'notification-provider-unavailable-contract',
      'notification-provider-manual-required-contract',
    ]);
  });

  it('preserves preflight refs while keeping delivery receipt and sensitive payload claims false', () => {
    const readModel = buildProviderStatusHandoffReadModel([BaseChildCard]);
    const providerSetupRow = readModel.rows[0];

    expect(providerSetupRow.sourceSchedulerEntryRef).toContain('app-game-child-ux-local-outbox-scheduler-');
    expect(providerSetupRow.sourceOutboxRecordRef).toContain('app-game-child-ux-local-outbox-');
    expect(providerSetupRow.sourceProviderChannelRef).toBe('in-app');
    expect(providerSetupRow.providerStatusBoundaryEntry.manualProofRequirements.length).toBe(3);
    expect(readModel.providerDeliveryRuntimeClaimed).toBe(false);
    expect(readModel.providerReceiptIngestionClaimed).toBe(false);
    expect(readModel.providerCredentialsClaimed).toBe(false);
    expect(readModel.adapterDispatchClaimed).toBe(false);
    expect(readModel.platformEnforcementClaimed).toBe(false);
    expect(readModel.rows.every((row) => row.providerStatusBoundaryEntry.providerReceiptRefs.length === 0)).toBe(true);
    expect(
      readModel.rows.every((row) => row.providerStatusBoundaryEntry.sensitiveProviderPayloadClaimed === false)
    ).toBe(true);
  });

  it('rejects provider delivery overclaims and mismatched unavailable status rows', () => {
    const readModel = buildProviderStatusHandoffReadModel([unavailableCard()]);
    const unavailableRow = readModel.rows[0];

    expect(
      AppGameChildUxLocalOutboxProviderStatusHandoffReadModelSchema.safeParse({
        ...readModel,
        providerDeliveryRuntimeClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameChildUxLocalOutboxProviderStatusHandoffRowSchema.safeParse({
        ...unavailableRow,
        providerStatusBoundaryEntry: {
          ...unavailableRow.providerStatusBoundaryEntry,
          providerStatus: 'manual-required',
          statusProofState: 'manual-action-required',
          quietHoursReadiness: 'manual-required',
          escalationReadiness: 'manual-required',
        },
      }).success
    ).toBe(false);
  });
});

function buildProviderStatusHandoffReadModel(cards: ReadonlyArray<unknown>) {
  const handoff = buildAppGameChildUxHandoffReadModel(
    HandoffOptions,
    cards.map((card) => AppGameChildUxCardSchema.parse(card))
  );
  const artifacts = buildAppGameChildUxLocalHandoffArtifactReadModel(ArtifactOptions, handoff);
  const outbox = buildAppGameChildUxLocalOutboxBridgeReadModel(BridgeOptions, artifacts);
  const scheduler = buildAppGameChildUxLocalOutboxSchedulerBridgeReadModel(SchedulerOptions, outbox);
  const preflight = buildAppGameChildUxLocalOutboxProviderPreflightReadModel(ProviderPreflightOptions, scheduler);
  return buildAppGameChildUxLocalOutboxProviderStatusHandoffReadModel(ProviderStatusOptions, preflight);
}

function nativeAppSubmittedCard() {
  return {
    ...BaseChildCard,
    childUxStateId: 'child-ux-native-app-request-submitted-provider-status',
    target: {
      targetKind: AppGameChildUxTargetKind.NativeApp,
      targetRef: 'target-native-app-child-ux-provider-status',
      childSafeDisplayLabelToken: AppGameChildUxCopyToken.RequestSubmittedTitle,
    },
    surfaceState: AppGameChildUxSurfaceState.RequestSubmitted,
    claimState: AppGameChildUxClaimState.RequestSubmitted,
    titleToken: AppGameChildUxCopyToken.RequestSubmittedTitle,
    bodyToken: AppGameChildUxCopyToken.RequestSubmittedBody,
    primaryAction: AppGameChildUxPrimaryAction.Dismiss,
    primaryActionToken: AppGameChildUxCopyToken.DismissAction,
    childReasonReferences: ['child-reason-request-submitted-provider-status'],
    childStatusReferences: ['child-status-request-submitted-provider-status'],
    approvalRequestRef: null,
  } as const;
}

function unavailableCard() {
  return {
    ...BaseChildCard,
    childUxStateId: 'child-ux-unavailable-provider-status',
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
