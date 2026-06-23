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
import {
  AppGameChildDeviceDeliveryReadinessReadModelSchema,
  AppGameChildDeviceDeliveryReadinessStatus,
  buildAppGameChildDeviceDeliveryReadinessReadModel,
  summarizeAppGameChildDeviceDeliveryReadiness,
} from '@ocentra-parent/schema-domain/app-game-child-facing-ux-child-device-delivery-readiness';
import { buildAppGameChildUxLocalOutboxBridgeReadModel } from '@ocentra-parent/schema-domain/app-game-child-facing-ux-local-outbox-bridge';
import { buildAppGameChildUxLocalOutboxSchedulerBridgeReadModel } from '@ocentra-parent/schema-domain/app-game-child-facing-ux-local-outbox-scheduler-bridge';
import {
  ParentActorRole,
  ParentContractSchemaVersion,
  ParentEvidenceReferenceKind,
  ParentPlatform,
} from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  buildAppGameChildUxLocalOutboxProviderPreflightReadModel,
  buildAppGameChildUxLocalOutboxProviderStatusHandoffReadModel,
} from './app-game-child-facing-ux-local-outbox-centralized-schema-fixtures';

const Timestamp = '2026-06-08T22:15:00Z';

const Device = {
  deviceId: 'device-child-delivery-readiness',
  childProfileId: 'child-profile-child-delivery-readiness',
  label: 'Study PC',
  platform: ParentPlatform.Windows,
} as const;

const EvidenceReference = {
  evidenceReferenceId: 'evidence-child-delivery-readiness',
  kind: ParentEvidenceReferenceKind.PolicyDecision,
  observedAt: Timestamp,
} as const;

const ParentAction = {
  actionReferenceId: 'parent-action-child-delivery-readiness',
  actor: {
    actorId: 'parent-child-delivery-readiness',
    role: ParentActorRole.Parent,
  },
  policyVersion: 'policy-child-delivery-readiness-v1',
  createdAt: Timestamp,
} as const;

const ApprovalRequestRef = {
  actionReferenceId: 'approval-request-child-delivery-readiness',
  actor: {
    actorId: 'child-device-local-agent',
    role: ParentActorRole.System,
  },
  policyVersion: 'policy-child-delivery-readiness-v1',
  createdAt: Timestamp,
} as const;

const BaseChildCard = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  childUxStateId: 'child-ux-limit-reached-child-delivery-readiness',
  device: Device,
  target: {
    targetKind: AppGameChildUxTargetKind.NativeGame,
    targetRef: 'target-native-game-child-delivery-readiness',
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
  childReasonReferences: ['child-reason-limit-reached-child-delivery-readiness'],
  childStatusReferences: ['child-status-limit-reached-child-delivery-readiness'],
  approvalRequestRef: ApprovalRequestRef,
  privateDiagnosticReferences: [],
  adapterActionRef: null,
} as const;

const HandoffOptions = {
  generatedAt: Timestamp,
  handoffId: 'app-game-child-delivery-readiness-handoff',
  localHandoffRootRef: 'child-device-delivery-readiness-handoff-root',
} as const;

const ArtifactOptions = {
  generatedAt: Timestamp,
  localArtifactRootRef: 'child-device-delivery-readiness-artifact-root',
  localArtifactFileRef: 'child-device-delivery-readiness-artifact-jsonl',
} as const;

const BridgeOptions = {
  family: { familyId: 'family-child-delivery-readiness' },
  parentAction: ParentAction,
  generatedAt: Timestamp,
  bridgeId: 'app-game-child-delivery-readiness-source-bridge-proof',
  outboxRootRef: 'parent-owned-child-delivery-readiness-root',
  outboxFileRef: 'parent-owned-child-delivery-readiness-jsonl-ref',
  localDataPathRef: 'parent-owned-child-delivery-readiness-data-path-ref',
} as const;

const SchedulerOptions = {
  generatedAt: Timestamp,
  schedulerBridgeId: 'app-game-child-delivery-readiness-scheduler-bridge-proof',
  schedulerArtifactRootRef: 'parent-owned-child-delivery-readiness-scheduler-root-ref',
  schedulerArtifactRef: 'parent-owned-child-delivery-readiness-scheduler-jsonl-ref',
  schedulerNowAt: Timestamp,
} as const;

const ProviderPreflightOptions = {
  generatedAt: Timestamp,
  providerPreflightId: 'app-game-child-delivery-readiness-provider-preflight-proof',
  sourceContractRefs: [
    'app-game-child-ux-local-outbox-scheduler-bridge',
    'notification-local-outbox-scheduler-proof',
    'notification-provider-adapter-boundary-required',
  ],
} as const;

const ProviderStatusOptions = {
  generatedAt: Timestamp,
  handoffId: 'app-game-child-delivery-readiness-provider-status-handoff',
  sourceContractRefs: [
    'app-game-child-ux-local-outbox-provider-preflight',
    'v0-8-notification-provider-status-boundary',
  ],
} as const;

const DeliveryOptions = {
  generatedAt: Timestamp,
  readinessId: 'app-game-child-device-delivery-readiness-proof',
  requiredTransportRefs: [
    'child-runtime-transport-contract-ref',
    'child-runtime-receipt-contract-ref',
    'child-device-local-agent-route-ref',
  ],
} as const;

describe('app/game child-device delivery readiness', () => {
  it('maps scheduled child UX provider-status rows into child transport-required readiness rows', () => {
    const readModel = buildAppGameChildDeviceDeliveryReadinessReadModel(
      DeliveryOptions,
      providerStatusReadModel([BaseChildCard, nativeAppSubmittedCard()])
    );
    const summary = summarizeAppGameChildDeviceDeliveryReadiness(readModel);

    expect(summary.transportRequiredCount).toBe(2);
    expect(summary.manualRequiredCount).toBe(0);
    expect(summary.unavailableCount).toBe(0);
    expect(readModel.rows.map((row) => row.deliveryReadinessStatus)).toEqual([
      AppGameChildDeviceDeliveryReadinessStatus.TransportRequired,
      AppGameChildDeviceDeliveryReadinessStatus.TransportRequired,
    ]);
    expect(readModel.rows.every((row) => row.sourceOutboxRecordRef !== null)).toBe(true);
    expect(readModel.rows.every((row) => row.sourceSchedulerEntryRef !== null)).toBe(true);
    expect(readModel.rows.every((row) => row.requiredTransportRefs.length === 3)).toBe(true);
    expect(readModel.rows.every((row) => row.openGaps.includes('child-runtime-transport-not-attached'))).toBe(true);
  });

  it('keeps manual-required and unavailable source rows out of transport-required readiness', () => {
    const readModel = buildAppGameChildDeviceDeliveryReadinessReadModel(
      DeliveryOptions,
      providerStatusReadModel([manualCard(), unavailableCard()])
    );

    expect(readModel.rows.map((row) => row.deliveryReadinessStatus)).toEqual([
      AppGameChildDeviceDeliveryReadinessStatus.ManualRequired,
      AppGameChildDeviceDeliveryReadinessStatus.Unavailable,
    ]);
    expect(readModel.manualRequiredCount).toBe(1);
    expect(readModel.unavailableCount).toBe(1);
    expect(readModel.transportRequiredCount).toBe(0);
  });

  it('rejects child runtime provider platform adapter and raw-source overclaims', () => {
    const readModel = buildAppGameChildDeviceDeliveryReadinessReadModel(
      DeliveryOptions,
      providerStatusReadModel([BaseChildCard])
    );
    const row = readModel.rows[0];

    expect(
      AppGameChildDeviceDeliveryReadinessReadModelSchema.safeParse({
        ...readModel,
        childRuntimeTransportClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameChildDeviceDeliveryReadinessReadModelSchema.safeParse({
        ...readModel,
        platformEnforcementClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameChildDeviceDeliveryReadinessReadModelSchema.safeParse({
        ...readModel,
        rows: [{ ...row, providerDeliveryExecuted: true }],
      }).success
    ).toBe(false);
  });
});

function providerStatusReadModel(cards: ReadonlyArray<unknown>) {
  const handoff = buildAppGameChildUxHandoffReadModel(
    HandoffOptions,
    cards.map((card) => AppGameChildUxCardSchema.parse(card))
  );
  const artifacts = buildAppGameChildUxLocalHandoffArtifactReadModel(ArtifactOptions, handoff);
  const outbox = buildAppGameChildUxLocalOutboxBridgeReadModel(BridgeOptions, artifacts);
  const scheduler = buildAppGameChildUxLocalOutboxSchedulerBridgeReadModel(SchedulerOptions, outbox);
  const providerPreflight = buildAppGameChildUxLocalOutboxProviderPreflightReadModel(
    ProviderPreflightOptions,
    scheduler
  );
  return buildAppGameChildUxLocalOutboxProviderStatusHandoffReadModel(ProviderStatusOptions, providerPreflight);
}

function nativeAppSubmittedCard() {
  return {
    ...BaseChildCard,
    childUxStateId: 'child-ux-native-app-request-submitted-child-delivery-readiness',
    target: {
      targetKind: AppGameChildUxTargetKind.NativeApp,
      targetRef: 'target-native-app-child-delivery-readiness',
      childSafeDisplayLabelToken: AppGameChildUxCopyToken.RequestSubmittedTitle,
    },
    surfaceState: AppGameChildUxSurfaceState.RequestSubmitted,
    claimState: AppGameChildUxClaimState.RequestSubmitted,
    titleToken: AppGameChildUxCopyToken.RequestSubmittedTitle,
    bodyToken: AppGameChildUxCopyToken.RequestSubmittedBody,
    primaryAction: AppGameChildUxPrimaryAction.Dismiss,
    primaryActionToken: AppGameChildUxCopyToken.DismissAction,
    childReasonReferences: ['child-reason-request-submitted-child-delivery-readiness'],
    childStatusReferences: ['child-status-request-submitted-child-delivery-readiness'],
    approvalRequestRef: null,
  } as const;
}

function manualCard() {
  return {
    ...BaseChildCard,
    childUxStateId: 'child-ux-manual-required-child-delivery-readiness',
    surfaceState: AppGameChildUxSurfaceState.ManualRequired,
    capabilityState: AppGameChildUxCapabilityState.ManualRequired,
    claimState: AppGameChildUxClaimState.ManualRequired,
    titleToken: AppGameChildUxCopyToken.ManualRequiredTitle,
    bodyToken: AppGameChildUxCopyToken.ManualRequiredBody,
    primaryAction: AppGameChildUxPrimaryAction.TryLater,
    primaryActionToken: AppGameChildUxCopyToken.TryLaterAction,
    adapterActionRef: null,
    approvalRequestRef: null,
  } as const;
}

function unavailableCard() {
  return {
    ...manualCard(),
    childUxStateId: 'child-ux-unavailable-child-delivery-readiness',
    surfaceState: AppGameChildUxSurfaceState.Unavailable,
    capabilityState: AppGameChildUxCapabilityState.Unavailable,
    claimState: AppGameChildUxClaimState.Unavailable,
    titleToken: AppGameChildUxCopyToken.UnavailableTitle,
    bodyToken: AppGameChildUxCopyToken.UnavailableBody,
  } as const;
}
