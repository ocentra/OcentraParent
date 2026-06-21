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
import { ParentContractSchemaVersion, ParentEvidenceReferenceKind, ParentPlatform } from '@ocentra-parent/schema-domain/family-reference-primitives';

const Timestamp = '2026-06-03T09:24:00Z';

const Device = {
  deviceId: 'device-child-ux-windows',
  childProfileId: 'child-ux-profile',
  label: 'Study PC',
  platform: ParentPlatform.Windows,
} as const;

const EvidenceReference = {
  evidenceReferenceId: 'evidence-child-ux-session',
  kind: ParentEvidenceReferenceKind.PolicyDecision,
  observedAt: Timestamp,
} as const;

const ApprovalRequestRef = {
  actionReferenceId: 'approval-request-child-ux',
  actor: {
    actorId: 'child-device-local-agent',
    role: 'system',
  },
  policyVersion: 'policy-child-ux-v1',
  createdAt: Timestamp,
} as const;

const BaseChildCard = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  childUxStateId: 'child-ux-family-rule-warning',
  device: Device,
  target: {
    targetKind: AppGameChildUxTargetKind.NativeApp,
    targetRef: 'target-native-app-homework-game',
    childSafeDisplayLabelToken: AppGameChildUxCopyToken.FamilyRuleTitle,
  },
  surfaceState: AppGameChildUxSurfaceState.FamilyRuleWarning,
  capabilityState: AppGameChildUxCapabilityState.Supported,
  claimState: AppGameChildUxClaimState.WarningOnly,
  explanationSource: AppGameChildUxExplanationSource.ParentRule,
  titleToken: AppGameChildUxCopyToken.FamilyRuleTitle,
  bodyToken: AppGameChildUxCopyToken.FamilyRuleBody,
  primaryAction: AppGameChildUxPrimaryAction.Dismiss,
  primaryActionToken: AppGameChildUxCopyToken.DismissAction,
  evidenceReferences: [EvidenceReference],
  childReasonReferences: [],
  childStatusReferences: ['child-status-family-rule-warning'],
  approvalRequestRef: null,
  privateDiagnosticReferences: [],
  adapterActionRef: null,
} as const;

describe('app/game child-facing UX contracts', () => {
  acceptsRespectfulParentRuleWarning();
  requiresAskParentRequestRefs();
  rejectsDiagnosticsAndMismatchedCopy();
  keepsManualRequiredAndUnavailableHonest();
});

function acceptsRespectfulParentRuleWarning() {
  it('accepts respectful parent-rule warning copy without adapter claims', () => {
    const parsed = AppGameChildUxCardSchema.parse(BaseChildCard);

    expect(parsed.explanationSource).toBe(AppGameChildUxExplanationSource.ParentRule);
    expect(parsed.titleToken).toBe(AppGameChildUxCopyToken.FamilyRuleTitle);
    expect(parsed.privateDiagnosticReferences).toEqual([]);
    expect(parsed.adapterActionRef).toBeNull();
  });
}

function requiresAskParentRequestRefs() {
  it('requires ask-parent requests to carry evidence, child reason, and child status refs', () => {
    const askParent = AppGameChildUxCardSchema.parse({
      ...BaseChildCard,
      childUxStateId: 'child-ux-limit-reached-request',
      surfaceState: AppGameChildUxSurfaceState.TimeLimitReached,
      claimState: AppGameChildUxClaimState.LimitReached,
      titleToken: AppGameChildUxCopyToken.LimitReachedTitle,
      bodyToken: AppGameChildUxCopyToken.LimitReachedBody,
      primaryAction: AppGameChildUxPrimaryAction.RequestMoreTime,
      primaryActionToken: AppGameChildUxCopyToken.RequestMoreTimeAction,
      childReasonReferences: ['child-reason-more-time-homework'],
      childStatusReferences: ['child-status-limit-reached'],
      approvalRequestRef: ApprovalRequestRef,
    });
    const missingRefs = AppGameChildUxCardSchema.safeParse({
      ...askParent,
      childReasonReferences: [],
      childStatusReferences: [],
      approvalRequestRef: null,
    });

    expect(askParent.approvalRequestRef).toEqual(ApprovalRequestRef);
    expect(missingRefs.success).toBe(false);
  });
}

function rejectsDiagnosticsAndMismatchedCopy() {
  it('rejects private diagnostics and mismatched child copy tokens', () => {
    const leakedDiagnostics = AppGameChildUxCardSchema.safeParse({
      ...BaseChildCard,
      privateDiagnosticReferences: ['process-path-c-users-child-secret-game-exe'],
    });
    const mismatchedCopy = AppGameChildUxCardSchema.safeParse({
      ...BaseChildCard,
      surfaceState: AppGameChildUxSurfaceState.NewAppApprovalNeeded,
      claimState: AppGameChildUxClaimState.ApprovalNeeded,
    });

    expect(leakedDiagnostics.success).toBe(false);
    expect(mismatchedCopy.success).toBe(false);
  });
}

function keepsManualRequiredAndUnavailableHonest() {
  it('keeps manual-required and unavailable states honest without adapter action refs', () => {
    const manualRequired = AppGameChildUxCardSchema.parse({
      ...BaseChildCard,
      childUxStateId: 'child-ux-manual-required',
      surfaceState: AppGameChildUxSurfaceState.ManualRequired,
      capabilityState: AppGameChildUxCapabilityState.ManualRequired,
      claimState: AppGameChildUxClaimState.ManualRequired,
      explanationSource: AppGameChildUxExplanationSource.Capability,
      titleToken: AppGameChildUxCopyToken.ManualRequiredTitle,
      bodyToken: AppGameChildUxCopyToken.ManualRequiredBody,
      primaryAction: AppGameChildUxPrimaryAction.TryLater,
      primaryActionToken: AppGameChildUxCopyToken.TryLaterAction,
      childStatusReferences: ['child-status-manual-required'],
    });
    const falseAdapterClaim = AppGameChildUxCardSchema.safeParse({
      ...manualRequired,
      adapterActionRef: ApprovalRequestRef,
    });

    expect(manualRequired.adapterActionRef).toBeNull();
    expect(falseAdapterClaim.success).toBe(false);
  });
}
