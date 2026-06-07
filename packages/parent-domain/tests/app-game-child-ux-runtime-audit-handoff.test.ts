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
  AppGameChildUxRuntimeAuditHandoffSchema,
  buildAppGameChildUxRuntimeAuditHandoff,
} from '../src/app-game-child-ux-runtime-audit-handoff';
import { AppGameChildUxRuntimeAuditHandoffState } from '../src/app-game-child-ux-runtime-audit-handoff-rules';
import { ParentContractSchemaVersion, ParentEvidenceReferenceKind, ParentPlatform } from '../src/reference-primitives';

const Timestamp = '2026-06-07T05:20:00Z';

const Device = {
  deviceId: 'device-child-runtime-audit-windows',
  childProfileId: 'child-runtime-audit-profile',
  label: 'Study PC',
  platform: ParentPlatform.Windows,
} as const;

const EvidenceReference = {
  evidenceReferenceId: 'evidence-child-runtime-audit-policy',
  kind: ParentEvidenceReferenceKind.PolicyDecision,
  observedAt: Timestamp,
} as const;

const ApprovalRequestRef = {
  actionReferenceId: 'approval-request-child-runtime-audit',
  actor: {
    actorId: 'child-device-local-agent',
    role: 'system',
  },
  policyVersion: 'policy-child-runtime-audit-v1',
  createdAt: Timestamp,
} as const;

const HandoffOptions = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  childUxRuntimeAuditHandoffId: 'app-game-child-ux-runtime-audit-handoff-proof',
  generatedAt: Timestamp,
  sourceContractRefs: [
    'app-game-child-facing-ux',
    'docs/expectations/app-game-evidence.md',
    'docs/expectations/enforcement.md',
  ],
  runtimeAuditProofRefs: ['future-app-game-child-ux-runtime-audit-persistence-proof'],
} as const;

const ReadyNativeAppCard = AppGameChildUxCardSchema.parse({
  schemaVersion: ParentContractSchemaVersion.V0_6,
  childUxStateId: 'child-ux-runtime-audit-ready-app',
  device: Device,
  target: {
    targetKind: AppGameChildUxTargetKind.NativeApp,
    targetRef: 'target-native-app-study-game',
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
  childReasonReferences: ['child-reason-runtime-audit-limit-reached'],
  childStatusReferences: ['child-status-runtime-audit-limit-reached'],
  approvalRequestRef: ApprovalRequestRef,
  privateDiagnosticReferences: [],
  adapterActionRef: null,
});

const MissingReasonNativeGameCard = AppGameChildUxCardSchema.parse({
  ...ReadyNativeAppCard,
  childUxStateId: 'child-ux-runtime-audit-missing-reason-game',
  target: {
    targetKind: AppGameChildUxTargetKind.NativeGame,
    targetRef: 'target-native-game-study',
    childSafeDisplayLabelToken: AppGameChildUxCopyToken.FamilyRuleTitle,
  },
  surfaceState: AppGameChildUxSurfaceState.FamilyRuleWarning,
  claimState: AppGameChildUxClaimState.WarningOnly,
  titleToken: AppGameChildUxCopyToken.FamilyRuleTitle,
  bodyToken: AppGameChildUxCopyToken.FamilyRuleBody,
  primaryAction: AppGameChildUxPrimaryAction.Dismiss,
  primaryActionToken: AppGameChildUxCopyToken.DismissAction,
  childReasonReferences: [],
  approvalRequestRef: null,
});

const MissingStatusUnknownAppCard = AppGameChildUxCardSchema.parse({
  ...ReadyNativeAppCard,
  childUxStateId: 'child-ux-runtime-audit-missing-status-unknown-app',
  target: {
    targetKind: AppGameChildUxTargetKind.UnknownApp,
    targetRef: 'target-unknown-app-review',
    childSafeDisplayLabelToken: AppGameChildUxCopyToken.FamilyRuleTitle,
  },
  surfaceState: AppGameChildUxSurfaceState.FamilyRuleWarning,
  claimState: AppGameChildUxClaimState.WarningOnly,
  titleToken: AppGameChildUxCopyToken.FamilyRuleTitle,
  bodyToken: AppGameChildUxCopyToken.FamilyRuleBody,
  primaryAction: AppGameChildUxPrimaryAction.Dismiss,
  primaryActionToken: AppGameChildUxCopyToken.DismissAction,
  childReasonReferences: ['child-reason-runtime-audit-new-app'],
  childStatusReferences: [],
  approvalRequestRef: null,
});

const ManualRequiredLauncherCard = AppGameChildUxCardSchema.parse({
  ...ReadyNativeAppCard,
  childUxStateId: 'child-ux-runtime-audit-manual-launcher',
  target: {
    targetKind: AppGameChildUxTargetKind.LauncherGameCandidate,
    targetRef: 'target-launcher-game-candidate',
    childSafeDisplayLabelToken: AppGameChildUxCopyToken.ManualRequiredTitle,
  },
  surfaceState: AppGameChildUxSurfaceState.ManualRequired,
  capabilityState: AppGameChildUxCapabilityState.ManualRequired,
  claimState: AppGameChildUxClaimState.ManualRequired,
  explanationSource: AppGameChildUxExplanationSource.Capability,
  titleToken: AppGameChildUxCopyToken.ManualRequiredTitle,
  bodyToken: AppGameChildUxCopyToken.ManualRequiredBody,
  primaryAction: AppGameChildUxPrimaryAction.TryLater,
  primaryActionToken: AppGameChildUxCopyToken.TryLaterAction,
  approvalRequestRef: null,
});

describe('app/game child UX runtime audit handoff', () => {
  it('projects child UX cards into runtime audit readiness rows', () => {
    const handoff = buildHandoff();

    expect(handoff.nativeAppRowCount).toBe(2);
    expect(handoff.nativeGameRowCount).toBe(2);
    expect(handoff.runtimeAuditReadyCount).toBe(1);
    expect(handoff.blockedMissingChildReasonCount).toBe(1);
    expect(handoff.blockedMissingChildStatusCount).toBe(1);
    expect(handoff.manualRequiredNoAdapterCount).toBe(1);
    expect(handoff.rows.map((row) => row.runtimeAuditHandoffState)).toEqual([
      AppGameChildUxRuntimeAuditHandoffState.RuntimeAuditReady,
      AppGameChildUxRuntimeAuditHandoffState.BlockedMissingChildReason,
      AppGameChildUxRuntimeAuditHandoffState.BlockedMissingChildStatus,
      AppGameChildUxRuntimeAuditHandoffState.ManualRequiredNoAdapter,
    ]);
  });

  it('requires ready rows to preserve child reason and status refs before future audit proof', () => {
    const handoff = buildHandoff();

    expect(handoff.rows[0]?.inheritedChildReasonReferences).toEqual(['child-reason-runtime-audit-limit-reached']);
    expect(handoff.rows[0]?.inheritedChildStatusReferences).toEqual(['child-status-runtime-audit-limit-reached']);
    expect(handoff.rows[0]?.requiredRuntimeAuditProofRefs).toEqual([
      'future-app-game-child-ux-runtime-audit-persistence-proof',
    ]);
    expect(handoff.rows[1]?.requiredRuntimeAuditProofRefs).toEqual([]);
    expect(handoff.rows[2]?.requiredRuntimeAuditProofRefs).toEqual([]);
    expect(handoff.rows[3]?.requiredRuntimeAuditProofRefs).toEqual([]);
  });

  it('rejects runtime delivery, adapter, platform, private diagnostic, and count overclaims', () => {
    const handoff = buildHandoff();

    expect(AppGameChildUxRuntimeAuditHandoffSchema.safeParse({ ...handoff, childRuntimeDelivered: true }).success).toBe(
      false
    );
    expect(AppGameChildUxRuntimeAuditHandoffSchema.safeParse({ ...handoff, runtimeAuditPersisted: true }).success).toBe(
      false
    );
    expect(
      AppGameChildUxRuntimeAuditHandoffSchema.safeParse({ ...handoff, adapterDispatchClaimed: true }).success
    ).toBe(false);
    expect(
      AppGameChildUxRuntimeAuditHandoffSchema.safeParse({ ...handoff, platformEnforcementClaimed: true }).success
    ).toBe(false);
    expect(
      AppGameChildUxRuntimeAuditHandoffSchema.safeParse({ ...handoff, privateDiagnosticsExposed: true }).success
    ).toBe(false);
    expect(AppGameChildUxRuntimeAuditHandoffSchema.safeParse({ ...handoff, runtimeAuditReadyCount: 0 }).success).toBe(
      false
    );
  });
});

function buildHandoff() {
  return buildAppGameChildUxRuntimeAuditHandoff(HandoffOptions, [
    ReadyNativeAppCard,
    MissingReasonNativeGameCard,
    MissingStatusUnknownAppCard,
    ManualRequiredLauncherCard,
  ]);
}
