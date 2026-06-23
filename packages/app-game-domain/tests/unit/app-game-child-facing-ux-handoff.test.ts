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
import {
  AppGameChildUxHandoffReadModelSchema,
  AppGameChildUxHandoffStatus,
  buildAppGameChildUxHandoffReadModel,
} from '@ocentra-parent/schema-domain/app-game-child-facing-ux-handoff';
import {
  ParentActorRole,
  ParentContractSchemaVersion,
  ParentEvidenceReferenceKind,
  ParentPlatform,
} from '@ocentra-parent/schema-domain/family-reference-primitives';

const Timestamp = '2026-06-07T19:58:00Z';

const Device = {
  deviceId: 'device-child-ux-handoff',
  childProfileId: 'child-profile-child-ux-handoff',
  label: 'Study PC',
  platform: ParentPlatform.Windows,
} as const;

const EvidenceReference = {
  evidenceReferenceId: 'evidence-child-ux-handoff',
  kind: ParentEvidenceReferenceKind.PolicyDecision,
  observedAt: Timestamp,
} as const;

const ApprovalRequestRef = {
  actionReferenceId: 'approval-request-child-ux-handoff',
  actor: {
    actorId: 'child-device-local-agent',
    role: ParentActorRole.System,
  },
  policyVersion: 'policy-child-ux-handoff-v1',
  createdAt: Timestamp,
} as const;

const BaseChildCard = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  childUxStateId: 'child-ux-limit-reached-handoff',
  device: Device,
  target: {
    targetKind: AppGameChildUxTargetKind.NativeGame,
    targetRef: 'target-native-game-child-ux-handoff',
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
  childReasonReferences: ['parent-approved'],
  childStatusReferences: ['child-status-limit-reached'],
  approvalRequestRef: ApprovalRequestRef,
  privateDiagnosticReferences: [],
  adapterActionRef: null,
} as const;

const Options = {
  generatedAt: Timestamp,
  handoffId: 'app-game-child-ux-handoff-readiness',
  localHandoffRootRef: 'child-device-local-ux-handoff-root',
} as const;

describe('app/game child-facing UX handoff readiness', () => {
  it('marks child UX cards with reason and status refs ready for local handoff without delivery claims', () => {
    const readModel = buildAppGameChildUxHandoffReadModel(Options, [AppGameChildUxCardSchema.parse(BaseChildCard)]);

    expect(readModel.readyCount).toBe(1);
    expect(readModel.blockedMissingRefsCount).toBe(0);
    expect(readModel.rows[0]?.status).toBe(AppGameChildUxHandoffStatus.Ready);
    expect(readModel.rows[0]?.handoffReferenceId).toBe('app-game-child-ux-handoff-child-ux-limit-reached-handoff');
    expect(readModel.childDeliveryRuntimeClaimed).toBe(false);
    expect(readModel.notificationDeliveryClaimed).toBe(false);
    expect(readModel.adapterDispatchClaimed).toBe(false);
  });

  it('blocks child UX handoff readiness until child reason and status refs are both present', () => {
    const blockedCard = AppGameChildUxCardSchema.parse({
      ...BaseChildCard,
      childUxStateId: 'child-ux-warning-missing-reason-handoff',
      surfaceState: AppGameChildUxSurfaceState.FamilyRuleWarning,
      claimState: AppGameChildUxClaimState.WarningOnly,
      titleToken: AppGameChildUxCopyToken.FamilyRuleTitle,
      bodyToken: AppGameChildUxCopyToken.FamilyRuleBody,
      primaryAction: AppGameChildUxPrimaryAction.Dismiss,
      primaryActionToken: AppGameChildUxCopyToken.DismissAction,
      childReasonReferences: [],
      approvalRequestRef: null,
    });
    const readModel = buildAppGameChildUxHandoffReadModel(Options, [blockedCard]);

    expect(readModel.readyCount).toBe(0);
    expect(readModel.blockedMissingRefsCount).toBe(1);
    expect(readModel.rows[0]?.status).toBe(AppGameChildUxHandoffStatus.BlockedMissingRefs);
    expect(readModel.rows[0]?.blockedReasonRefs).toEqual(['child-reason-and-status-refs-required']);
  });

  it('rejects child delivery and adapter overclaims at the handoff boundary', () => {
    const readModel = buildAppGameChildUxHandoffReadModel(Options, [AppGameChildUxCardSchema.parse(BaseChildCard)]);

    expect(
      AppGameChildUxHandoffReadModelSchema.safeParse({
        ...readModel,
        childDeliveryRuntimeClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameChildUxHandoffReadModelSchema.safeParse({
        ...readModel,
        adapterDispatchClaimed: true,
      }).success
    ).toBe(false);
  });
});
