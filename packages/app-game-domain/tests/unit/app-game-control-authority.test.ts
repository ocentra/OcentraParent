import { describe, expect, it } from 'vitest';
import {
  AppGameControlActionResultSchema,
  AppGameControlApprovalAuthoritySchema,
  AppGameControlApprovalDecisionSchema,
  AppGameControlApprovalRequestSchema,
} from '../../src/app-game-control-authority';
import {
  EnforcementAdapterKind,
  EnforcementAdapterResultCode,
  EnforcementCapabilityState,
  EnforcementMode,
  EnforcementResultStatus,
  EnforcementRollbackState,
} from '@ocentra-parent/enforcement-domain/enforcement';
import { PolicyAction } from '@ocentra-parent/policy-domain/policy';
import { ParentContractSchemaVersion, ParentEvidenceReferenceKind, ParentPlatform } from '@ocentra-parent/schema-domain/family-reference-primitives';

const Timestamp = '2026-06-02T23:35:00Z';
const PolicyVersion = 'policy-version-1';

const ParentActor = {
  actorId: 'parent-actor-1',
  role: 'parent',
} as const;

const ChildDevice = {
  deviceId: 'device-windows-1',
  childProfileId: 'child-1',
  label: 'Study PC',
  platform: ParentPlatform.Windows,
} as const;

const EvidenceReference = {
  evidenceReferenceId: 'evidence-app-game-session-1',
  kind: ParentEvidenceReferenceKind.ActivityEvent,
  observedAt: Timestamp,
} as const;

const ParentAction = {
  actionReferenceId: 'parent-action-1',
  actor: ParentActor,
  policyVersion: PolicyVersion,
  createdAt: Timestamp,
} as const;

const AppTarget = {
  targetId: 'target-app-1',
  targetType: 'app',
  targetValue: 'process:eldenring.exe',
} as const;

const SupportedCapability = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  platform: ParentPlatform.Windows,
  adapterKind: EnforcementAdapterKind.ProcessControl,
  capabilityState: EnforcementCapabilityState.Supported,
  permissionState: 'allowed',
  dependencyState: 'installed',
  supportedActions: [EnforcementMode.TerminateProcess, EnforcementMode.TimeLimit],
  degradedReason: null,
  lastCheckedAt: Timestamp,
} as const;

const ManualRequiredCapability = {
  ...SupportedCapability,
  capabilityState: EnforcementCapabilityState.ManualRequired,
  degradedReason: 'manual-required',
} as const;

const DryRunCapability = {
  ...SupportedCapability,
  capabilityState: EnforcementCapabilityState.DryRun,
} as const;

const ApprovedAppRequest = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  requestId: 'approval-request-1',
  policyKind: 'app-control',
  device: ChildDevice,
  target: AppTarget,
  requestedAction: PolicyAction.Block,
  requestedMode: EnforcementMode.TerminateProcess,
  requestedSettingRefs: [
    {
      settingId: 'app.enforcement.allowedActions',
      writesTo: '/appPolicy/enforcement/allowedActions',
    },
  ],
  evidenceReferences: [EvidenceReference],
  expiresAt: Timestamp,
  unansweredFallback: 'deny',
} as const;

const ApprovedDecision = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  decisionId: 'approval-decision-1',
  requestId: 'approval-request-1',
  policyKind: 'app-control',
  decisionState: 'approved',
  parentAction: ParentAction,
  reasonCodes: ['parent-approved'],
  policyVersion: PolicyVersion,
  evidenceReferences: [EvidenceReference],
  decidedAt: Timestamp,
} as const;

const EnforcedResult = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  resultId: 'enforcement-result-1',
  actionId: 'enforcement-action-1',
  status: EnforcementResultStatus.ActuallyEnforced,
  adapterResultCode: EnforcementAdapterResultCode.ProcessTerminated,
  startedAt: Timestamp,
  completedAt: Timestamp,
  rollbackToken: null,
  rollbackState: EnforcementRollbackState.NotRequired,
  unavailableReason: null,
  unavailableStatus: null,
  failedReason: null,
  nextCheckAt: null,
  capability: SupportedCapability,
} as const;

const WouldEnforceResult = {
  ...EnforcedResult,
  resultId: 'enforcement-result-dry-run-1',
  status: EnforcementResultStatus.WouldEnforce,
  adapterResultCode: EnforcementAdapterResultCode.DryRunNoAction,
  rollbackState: EnforcementRollbackState.NotRequired,
  capability: DryRunCapability,
} as const;

const SuccessfulActionResult = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  resultId: 'app-game-action-result-1',
  request: ApprovedAppRequest,
  decision: ApprovedDecision,
  approvalState: 'approved',
  capabilityState: EnforcementCapabilityState.Supported,
  capability: SupportedCapability,
  evidenceProofKind: 'app-identity-proof',
  resultStatus: 'enforced',
  enforcementResult: EnforcedResult,
  recordedAt: Timestamp,
} as const;

const ActiveAuthority = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  authorityId: 'authority-1',
  actor: ParentActor,
  device: ChildDevice,
  policyVersion: PolicyVersion,
  authorityState: 'active',
  allowedPolicyKinds: ['app-control', 'game-control'],
  canApprove: true,
  canDeny: true,
  canExtend: true,
  canOverride: false,
  canObserveOnly: true,
  checkedAt: Timestamp,
} as const;

const assertAcceptsValidAuthorityAndActionResult = () => {
  expect(AppGameControlApprovalAuthoritySchema.safeParse(ActiveAuthority).success).toBe(true);
  expect(AppGameControlApprovalRequestSchema.safeParse(ApprovedAppRequest).success).toBe(true);
  expect(AppGameControlApprovalDecisionSchema.safeParse(ApprovedDecision).success).toBe(true);
  expect(AppGameControlActionResultSchema.safeParse(SuccessfulActionResult).success).toBe(true);
};

const assertRejectsObserverAuthorityGrants = () => {
  expect(
    AppGameControlApprovalAuthoritySchema.safeParse({
      ...ActiveAuthority,
      authorityState: 'observe-only',
      canApprove: true,
    }).success
  ).toBe(false);
};

const assertRejectsMissingEvidenceAndCrossKindSettings = () => {
  expect(
    AppGameControlApprovalRequestSchema.safeParse({
      ...ApprovedAppRequest,
      evidenceReferences: [],
    }).success
  ).toBe(false);
  expect(
    AppGameControlApprovalRequestSchema.safeParse({
      ...ApprovedAppRequest,
      policyKind: 'game-control',
      requestedSettingRefs: [{ settingId: 'game.rules.allowedActions', writesTo: '/appPolicy/rules/allowedActions' }],
    }).success
  ).toBe(false);
};

const assertRejectsStaleDecisionPolicyVersion = () => {
  expect(
    AppGameControlApprovalDecisionSchema.safeParse({
      ...ApprovedDecision,
      policyVersion: 'policy-version-2',
    }).success
  ).toBe(false);
};

const assertRejectsInvalidActionResults = () => {
  expect(
    AppGameControlActionResultSchema.safeParse({
      ...SuccessfulActionResult,
      capabilityState: EnforcementCapabilityState.ManualRequired,
      capability: ManualRequiredCapability,
      resultStatus: 'enforced',
      enforcementResult: null,
    }).success
  ).toBe(false);
  expect(
    AppGameControlActionResultSchema.safeParse({
      ...SuccessfulActionResult,
      request: {
        ...ApprovedAppRequest,
        policyKind: 'game-control',
        requestedSettingRefs: [
          { settingId: 'game.rules.allowedActions', writesTo: '/gamePolicy/rules/allowedActions' },
        ],
      },
      decision: { ...ApprovedDecision, policyKind: 'game-control' },
      evidenceProofKind: 'launcher-only',
      resultStatus: 'dispatch-ready',
      enforcementResult: null,
    }).success
  ).toBe(false);
  expect(
    AppGameControlActionResultSchema.safeParse({
      ...SuccessfulActionResult,
      evidenceProofKind: 'unknown-app',
      resultStatus: 'dispatch-ready',
      enforcementResult: null,
    }).success
  ).toBe(false);
  expect(
    AppGameControlActionResultSchema.safeParse({
      ...SuccessfulActionResult,
      decision: { ...ApprovedDecision, decisionState: 'denied', parentAction: null },
      approvalState: 'denied',
      resultStatus: 'enforced',
    }).success
  ).toBe(false);
};

const assertKeepsDryRunAndManualRequiredOutOfAdapterExecution = () => {
  expect(
    AppGameControlActionResultSchema.safeParse({
      ...SuccessfulActionResult,
      resultId: 'app-game-action-result-dry-run-1',
      capabilityState: EnforcementCapabilityState.DryRun,
      capability: DryRunCapability,
      resultStatus: 'would-enforce',
      enforcementResult: WouldEnforceResult,
    }).success
  ).toBe(true);
  expect(
    AppGameControlActionResultSchema.safeParse({
      ...SuccessfulActionResult,
      resultId: 'app-game-action-result-dry-run-claims-enforced',
      capabilityState: EnforcementCapabilityState.DryRun,
      capability: DryRunCapability,
      resultStatus: 'enforced',
      enforcementResult: { ...WouldEnforceResult, status: EnforcementResultStatus.ActuallyEnforced },
    }).success
  ).toBe(false);
  expect(
    AppGameControlActionResultSchema.safeParse({
      ...SuccessfulActionResult,
      resultId: 'app-game-action-result-manual-required-adapter',
      capabilityState: EnforcementCapabilityState.ManualRequired,
      capability: ManualRequiredCapability,
      resultStatus: 'manual-required',
      enforcementResult: EnforcedResult,
    }).success
  ).toBe(false);
};

describe('app/game control authority contracts', () => {
  it('accepts active parent authority and action result', assertAcceptsValidAuthorityAndActionResult);
  it('rejects observe-only authority with approval grants', assertRejectsObserverAuthorityGrants);
  it('rejects missing evidence and cross-kind settings', assertRejectsMissingEvidenceAndCrossKindSettings);
  it('rejects decisions whose parent action policy version is stale', assertRejectsStaleDecisionPolicyVersion);
  it('rejects invalid action-result claims', assertRejectsInvalidActionResults);
  it(
    'keeps dry-run and manual-required action results out of adapter execution',
    assertKeepsDryRunAndManualRequiredOutOfAdapterExecution
  );
});
