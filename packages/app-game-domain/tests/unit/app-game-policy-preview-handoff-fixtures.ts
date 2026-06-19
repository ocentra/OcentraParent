import {
  AppGamePolicyCompilerAuthorityState,
  AppGamePolicyCompilerEvidenceState,
  AppGamePolicyCompilerOutcomeState,
  AppGamePolicyCompilerProofKind,
  AppGamePolicyCompilerRejectionReason,
  AppGamePolicyCompilerRequestedAction,
  AppGamePolicyTargetKind,
} from '../../src/app-game-policy-target-compiler-rules';
import { PolicyAction, PolicyDecisionHandoffState } from '@ocentra-parent/policy-domain/policy';
import { PolicyCompilerCapabilityState } from '@ocentra-parent/policy-domain/policy-compiler';
import { ParentContractSchemaVersion, ParentEvidenceReferenceKind, ParentPlatform } from '@ocentra-parent/schema-domain/family-reference-primitives';

export const Timestamp = '2026-06-05T14:45:00Z';
export const PolicyVersion = 'app-game-policy-preview-version-1';
export const RuleId = 'policy-rule-app-game-preview-1';
export const LocalUserRef = 'windows-local-user-preview-1';

export const PreviewOptions = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  handoffId: 'app-game-policy-preview-handoff-proof',
  generatedAt: Timestamp,
  sourceContractRefs: [
    'app-game-policy-target-compiler',
    'docs/expectations/policy.md',
    'docs/expectations/enforcement.md',
  ],
} as const;

export const ChildDevice = {
  deviceId: 'device-windows-preview-1',
  childProfileId: 'child-preview-1',
  label: 'Study PC',
  platform: ParentPlatform.Windows,
} as const;

export const EvidenceReference = {
  evidenceReferenceId: 'app-game-policy-preview-evidence-1',
  kind: ParentEvidenceReferenceKind.ActivityEvent,
  observedAt: Timestamp,
} as const;

const CapabilityRef = {
  capabilityRef: 'capability-preview-1',
  capabilityState: PolicyCompilerCapabilityState.Supported,
  evidenceReferences: [EvidenceReference],
} as const;

const AuthorityRef = {
  authorityRef: 'authority-preview-1',
  authorityState: AppGamePolicyCompilerAuthorityState.Proved,
  evidenceReferences: [EvidenceReference],
} as const;

const baseEvidence = (proofKind = AppGamePolicyCompilerProofKind.Identity) =>
  ({
    evidenceReference: EvidenceReference,
    proofKind,
    evidenceState: AppGamePolicyCompilerEvidenceState.Active,
    device: ChildDevice,
    localUserRef: LocalUserRef,
    observedAt: Timestamp,
  }) as const;

const appCompileRequest = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  compileRequestId: 'compile-request-preview-app',
  policyVersion: PolicyVersion,
  ruleId: RuleId,
  device: ChildDevice,
  localUserRef: LocalUserRef,
  target: {
    targetKind: AppGamePolicyTargetKind.SpecificApp,
    targetRef: 'process:study-game-launcher.exe',
  },
  requestedAction: AppGamePolicyCompilerRequestedAction.TimeLimit,
  policyAction: PolicyAction.TimeLimit,
  scheduleRef: null,
  evidence: [baseEvidence()],
  capabilityRefs: [CapabilityRef],
  authorityRefs: [AuthorityRef],
  requestedAt: Timestamp,
} as const;

export const appCompiledDecision = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  compiledDecisionId: 'compiled-decision-preview-app',
  request: appCompileRequest,
  policyTarget: {
    targetId: 'policy-target-preview-app',
    targetType: 'app',
    targetValue: 'process:study-game-launcher.exe',
  },
  policyDecision: {
    schemaVersion: ParentContractSchemaVersion.V0_6,
    decisionId: 'policy-decision-preview-app',
    action: PolicyAction.TimeLimit,
    reasonCodes: ['app-game-preview-rule-match'],
    evidenceReferences: [EvidenceReference],
    ruleIds: [RuleId],
    localAiResultId: null,
    dryRun: true,
    enforcementHandoffState: PolicyDecisionHandoffState.Disabled,
    expiresAt: null,
  },
  outcomeState: AppGamePolicyCompilerOutcomeState.DryRunReady,
  rejectionReason: AppGamePolicyCompilerRejectionReason.None,
  capabilityRefs: ['capability-preview-1'],
  authorityRefs: ['authority-preview-1'],
  auditRefs: ['audit-preview-1'],
  compiledAt: Timestamp,
} as const;

export const gameManualCompiledDecision = {
  ...appCompiledDecision,
  compiledDecisionId: 'compiled-decision-preview-game-manual',
  request: {
    ...appCompileRequest,
    compileRequestId: 'compile-request-preview-game',
    target: {
      targetKind: AppGamePolicyTargetKind.SpecificGame,
      targetRef: 'launcher-game:space-miner',
    },
    requestedAction: AppGamePolicyCompilerRequestedAction.BlockLaunch,
    policyAction: PolicyAction.Block,
    evidence: [baseEvidence()],
    capabilityRefs: [
      {
        ...CapabilityRef,
        capabilityState: PolicyCompilerCapabilityState.ManualRequired,
      },
    ],
    authorityRefs: [
      {
        ...AuthorityRef,
        authorityState: AppGamePolicyCompilerAuthorityState.ManualRequired,
      },
    ],
  },
  policyTarget: {
    targetId: 'policy-target-preview-game',
    targetType: 'app',
    targetValue: 'launcher-game:space-miner',
  },
  policyDecision: {
    ...appCompiledDecision.policyDecision,
    decisionId: 'policy-decision-preview-game',
    action: PolicyAction.Block,
    evidenceReferences: [EvidenceReference],
  },
  outcomeState: AppGamePolicyCompilerOutcomeState.ManualRequired,
  rejectionReason: AppGamePolicyCompilerRejectionReason.BlockLaunchManualRequired,
  capabilityRefs: ['capability-preview-game-manual'],
  authorityRefs: ['authority-preview-game-manual'],
  auditRefs: ['audit-preview-game-manual'],
} as const;
