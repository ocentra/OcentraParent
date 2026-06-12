import { describe, expect, it } from 'vitest';
import {
  AppGamePolicyCompileRequestSchema,
  AppGamePolicyCompiledDecisionSchema,
} from '../../src/app-game-policy-target-compiler';
import {
  AppGamePolicyCompilerAuthorityState,
  AppGamePolicyCompilerCapabilityState,
  AppGamePolicyCompilerEvidenceState,
  AppGamePolicyCompilerOutcomeState,
  AppGamePolicyCompilerProofKind,
  AppGamePolicyCompilerRejectionReason,
  AppGamePolicyCompilerRequestedAction,
  AppGamePolicyTargetKind,
} from '../../src/app-game-policy-target-compiler-rules';
import { PolicyAction, PolicyDecisionHandoffState } from '@ocentra-parent/policy-domain/policy';
import { ParentContractSchemaVersion, ParentEvidenceReferenceKind, ParentPlatform } from '@ocentra-parent/family-domain/reference-primitives';

const Timestamp = '2026-06-03T08:35:00Z';
const PolicyVersion = 'app-game-policy-version-1';
const RuleId = 'policy-rule-app-game-1';
const LocalUserRef = 'windows-local-user-1';

const ChildDevice = {
  deviceId: 'device-windows-1',
  childProfileId: 'child-1',
  label: 'Study PC',
  platform: ParentPlatform.Windows,
} as const;

const OtherDevice = {
  ...ChildDevice,
  deviceId: 'device-windows-2',
} as const;

const EvidenceReference = {
  evidenceReferenceId: 'app-game-policy-evidence-1',
  kind: ParentEvidenceReferenceKind.ActivityEvent,
  observedAt: Timestamp,
} as const;

const CapabilityRef = {
  capabilityRef: 'capability-ref-1',
  capabilityState: AppGamePolicyCompilerCapabilityState.Supported,
  evidenceReferences: [EvidenceReference],
} as const;

const ManualCapabilityRef = {
  ...CapabilityRef,
  capabilityState: AppGamePolicyCompilerCapabilityState.ManualRequired,
} as const;

const AuthorityRef = {
  authorityRef: 'authority-ref-1',
  authorityState: AppGamePolicyCompilerAuthorityState.Proved,
  evidenceReferences: [EvidenceReference],
} as const;

const ManualAuthorityRef = {
  ...AuthorityRef,
  authorityState: AppGamePolicyCompilerAuthorityState.ManualRequired,
} as const;

const identityEvidence = (overrides: object = {}) =>
  ({
    evidenceReference: EvidenceReference,
    proofKind: AppGamePolicyCompilerProofKind.Identity,
    evidenceState: AppGamePolicyCompilerEvidenceState.Active,
    device: ChildDevice,
    localUserRef: LocalUserRef,
    observedAt: Timestamp,
    ...overrides,
  }) as const;

const baseCompileRequest = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  compileRequestId: 'compile-request-1',
  policyVersion: PolicyVersion,
  ruleId: RuleId,
  device: ChildDevice,
  localUserRef: LocalUserRef,
  target: {
    targetKind: AppGamePolicyTargetKind.SpecificApp,
    targetRef: 'process:game.exe',
  },
  requestedAction: AppGamePolicyCompilerRequestedAction.TimeLimit,
  policyAction: PolicyAction.TimeLimit,
  scheduleRef: null,
  evidence: [identityEvidence()],
  capabilityRefs: [CapabilityRef],
  authorityRefs: [AuthorityRef],
  requestedAt: Timestamp,
} as const;

const baseCompiledDecision = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  compiledDecisionId: 'compiled-decision-1',
  request: baseCompileRequest,
  policyTarget: {
    targetId: 'policy-target-1',
    targetType: 'app',
    targetValue: 'process:game.exe',
  },
  policyDecision: {
    schemaVersion: ParentContractSchemaVersion.V0_6,
    decisionId: 'policy-decision-1',
    action: PolicyAction.TimeLimit,
    reasonCodes: ['app-game-rule-match'],
    evidenceReferences: [EvidenceReference],
    ruleIds: [RuleId],
    localAiResultId: null,
    dryRun: true,
    enforcementHandoffState: PolicyDecisionHandoffState.Disabled,
    expiresAt: null,
  },
  outcomeState: AppGamePolicyCompilerOutcomeState.DryRunReady,
  rejectionReason: AppGamePolicyCompilerRejectionReason.None,
  capabilityRefs: ['capability-ref-1'],
  authorityRefs: ['authority-ref-1'],
  auditRefs: ['audit-ref-1'],
  compiledAt: Timestamp,
} as const;

const assertSpecificTargetRequiresIdentity = () => {
  expect(AppGamePolicyCompileRequestSchema.safeParse(baseCompileRequest).success).toBe(true);
  expect(
    AppGamePolicyCompileRequestSchema.safeParse({
      ...baseCompileRequest,
      evidence: [{ ...identityEvidence(), proofKind: AppGamePolicyCompilerProofKind.Category }],
    }).success
  ).toBe(false);
};

const assertUnknownTargetCompilesOnlyFromUnknownState = () => {
  expect(
    AppGamePolicyCompileRequestSchema.safeParse({
      ...baseCompileRequest,
      target: { targetKind: AppGamePolicyTargetKind.UnknownApp, targetRef: 'unknown:portable-app-1' },
      requestedAction: AppGamePolicyCompilerRequestedAction.AskParent,
      policyAction: PolicyAction.AskParent,
      evidence: [identityEvidence({ proofKind: AppGamePolicyCompilerProofKind.UnknownState })],
    }).success
  ).toBe(true);
  expect(
    AppGamePolicyCompileRequestSchema.safeParse({
      ...baseCompileRequest,
      target: { targetKind: AppGamePolicyTargetKind.UnknownGame, targetRef: 'unknown-game:1' },
      evidence: [identityEvidence()],
    }).success
  ).toBe(false);
};

const assertBlockLaunchWithoutProofIsManualRequired = () => {
  const manualRequest = {
    ...baseCompileRequest,
    requestedAction: AppGamePolicyCompilerRequestedAction.BlockLaunch,
    policyAction: PolicyAction.Block,
    capabilityRefs: [ManualCapabilityRef],
    authorityRefs: [ManualAuthorityRef],
  } as const;

  expect(
    AppGamePolicyCompiledDecisionSchema.safeParse({
      ...baseCompiledDecision,
      request: manualRequest,
      policyDecision: { ...baseCompiledDecision.policyDecision, action: PolicyAction.Block },
      outcomeState: AppGamePolicyCompilerOutcomeState.ManualRequired,
      rejectionReason: AppGamePolicyCompilerRejectionReason.BlockLaunchManualRequired,
    }).success
  ).toBe(true);
  expect(
    AppGamePolicyCompiledDecisionSchema.safeParse({
      ...baseCompiledDecision,
      request: manualRequest,
      policyDecision: { ...baseCompiledDecision.policyDecision, action: PolicyAction.Block },
      outcomeState: AppGamePolicyCompilerOutcomeState.DryRunReady,
      rejectionReason: AppGamePolicyCompilerRejectionReason.None,
    }).success
  ).toBe(false);
};

const assertRejectsWrongDeviceLocalUserAndStaleEvidence = () => {
  expect(
    AppGamePolicyCompileRequestSchema.safeParse({
      ...baseCompileRequest,
      evidence: [identityEvidence({ device: OtherDevice })],
    }).success
  ).toBe(false);
  expect(
    AppGamePolicyCompileRequestSchema.safeParse({
      ...baseCompileRequest,
      evidence: [identityEvidence({ localUserRef: 'windows-local-user-2' })],
    }).success
  ).toBe(false);
  expect(
    AppGamePolicyCompileRequestSchema.safeParse({
      ...baseCompileRequest,
      evidence: [identityEvidence({ evidenceState: AppGamePolicyCompilerEvidenceState.Stale })],
    }).success
  ).toBe(false);
};

const assertPolicyOutputCarriesEvidenceAndCapabilityRefs = () => {
  expect(AppGamePolicyCompiledDecisionSchema.safeParse(baseCompiledDecision).success).toBe(true);
  expect(
    AppGamePolicyCompiledDecisionSchema.safeParse({
      ...baseCompiledDecision,
      policyDecision: { ...baseCompiledDecision.policyDecision, evidenceReferences: [] },
    }).success
  ).toBe(false);
  expect(
    AppGamePolicyCompiledDecisionSchema.safeParse({
      ...baseCompiledDecision,
      capabilityRefs: [],
    }).success
  ).toBe(false);
};

describe('app/game policy target compiler contracts', () => {
  it('requires identity proof for specific targets', () => {
    assertSpecificTargetRequiresIdentity();
  });

  it('compiles unknown targets only from unknown-state evidence', () => {
    assertUnknownTargetCompilesOnlyFromUnknownState();
  });

  it('returns manual-required for unproved block-launch targets', () => {
    assertBlockLaunchWithoutProofIsManualRequired();
  });

  it('rejects wrong device, wrong local user, and stale evidence', () => {
    assertRejectsWrongDeviceLocalUserAndStaleEvidence();
  });

  it('requires compiled output to carry evidence and capability refs', () => {
    assertPolicyOutputCarriesEvidenceAndCapabilityRefs();
  });
});
