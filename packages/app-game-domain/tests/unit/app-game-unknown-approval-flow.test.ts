import { describe, expect, it } from 'vitest';
import {
  AppGameControlActionResultSchema,
  AppGameControlApprovalDecisionSchema,
  AppGameControlApprovalRequestSchema,
} from '../../src/app-game-control-authority';
import { EnforcementAdapterKind, EnforcementCapabilityState, EnforcementMode } from '@ocentra-parent/enforcement-domain/enforcement';
import { PolicyAction } from '@ocentra-parent/policy-domain/policy';
import { ParentContractSchemaVersion, ParentEvidenceReferenceKind, ParentPlatform } from '@ocentra-parent/schema-domain/family-reference-primitives';

const Timestamp = '2026-06-03T07:40:00Z';
const PolicyVersion = 'policy-version-unknown-approval-1';

const ParentActor = {
  actorId: 'parent-unknown-approval-actor',
  role: 'parent',
} as const;

const Device = {
  deviceId: 'device-unknown-approval-windows',
  childProfileId: 'child-unknown-approval',
  label: 'Study PC',
  platform: ParentPlatform.Windows,
} as const;

const EvidenceReference = {
  evidenceReferenceId: 'evidence-unknown-process-ref',
  kind: ParentEvidenceReferenceKind.ActivityEvent,
  observedAt: Timestamp,
} as const;

const ParentAction = {
  actionReferenceId: 'parent-action-unknown-approval',
  actor: ParentActor,
  policyVersion: PolicyVersion,
  createdAt: Timestamp,
} as const;

const AppTarget = {
  targetId: 'target-unknown-app',
  targetType: 'app',
  targetValue: 'process:unknown-portable.exe',
} as const;

const NewAppCandidate = {
  candidateId: 'candidate-new-inventory-app',
  candidateKind: 'new-inventory-app',
  candidateSource: 'inventory',
  detectedAt: Timestamp,
  evidenceReferences: [EvidenceReference],
} as const;

const UnknownGameCandidate = {
  candidateId: 'candidate-unknown-game-like',
  candidateKind: 'unknown-game-like-executable',
  candidateSource: 'runtime',
  detectedAt: Timestamp,
  evidenceReferences: [EvidenceReference],
} as const;

const SupportedCapability = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  platform: ParentPlatform.Windows,
  adapterKind: EnforcementAdapterKind.ProcessControl,
  capabilityState: EnforcementCapabilityState.Supported,
  permissionState: 'allowed',
  dependencyState: 'installed',
  supportedActions: [EnforcementMode.TerminateProcess],
  degradedReason: null,
  lastCheckedAt: Timestamp,
} as const;

const ManualRequiredCapability = {
  ...SupportedCapability,
  capabilityState: EnforcementCapabilityState.ManualRequired,
  degradedReason: 'manual-required',
} as const;

const NewAppRequest = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  requestId: 'approval-request-new-app',
  policyKind: 'app-control',
  device: Device,
  target: AppTarget,
  requestedAction: PolicyAction.AskParent,
  requestedMode: null,
  requestedSettingRefs: [{ settingId: 'app.unknown.behavior', writesTo: '/appPolicy/unknown/behavior' }],
  evidenceReferences: [EvidenceReference],
  candidate: NewAppCandidate,
  childReasonState: 'not-requested',
  childReasonReferences: [],
  childStatusReferences: ['child-status-new-app-review-ref'],
  expiresAt: Timestamp,
  unansweredFallback: 'expire',
} as const;

describe('app/game unknown approval flow contracts', () => {
  acceptsNewInventoryAppApprovalRequest();
  keepsAllowOnceApprovalsExpiring();
  rejectsWeakFallbackAndMissingChildRefs();
  keepsUnsupportedBlocksManualRequired();
  rejectsMalformedResponseScopes();
});

function acceptsNewInventoryAppApprovalRequest() {
  it('accepts a new inventory app approval request with evidence and child status refs', () => {
    const parsed = AppGameControlApprovalRequestSchema.parse(NewAppRequest);

    expect(parsed.candidate?.candidateKind).toBe('new-inventory-app');
    expect(parsed.childStatusReferences).toEqual(['child-status-new-app-review-ref']);
    expect(parsed.evidenceReferences).toEqual([EvidenceReference]);
    expect(parsed.unansweredFallback).toBe('expire');
  });
}

function keepsAllowOnceApprovalsExpiring() {
  it('keeps allow-once approvals expiring and persistent approvals audit-backed', () => {
    const allowOnce = AppGameControlApprovalDecisionSchema.parse({
      ...approvalDecisionBase('approval-decision-allow-once'),
      responseScope: 'allow-once',
      decisionExpiresAt: Timestamp,
    });
    const persistent = AppGameControlApprovalDecisionSchema.parse({
      ...approvalDecisionBase('approval-decision-persistent'),
      responseScope: 'allow-this-app-game',
      auditReferences: ['audit-app-approval-replayed-ref'],
      persistenceState: 'replayed',
    });

    expect(allowOnce.responseScope).toBe('allow-once');
    expect(allowOnce.decisionExpiresAt).toBe(Timestamp);
    expect(persistent.auditReferences).toEqual(['audit-app-approval-replayed-ref']);
    expect(persistent.persistenceState).toBe('replayed');
  });
}

function rejectsWeakFallbackAndMissingChildRefs() {
  it('rejects unsafe weak game candidate fallback and missing child reason refs', () => {
    const unsafeGameFallback = AppGameControlApprovalRequestSchema.safeParse({
      ...NewAppRequest,
      policyKind: 'game-control',
      target: { ...AppTarget, targetType: 'process' },
      requestedSettingRefs: [{ settingId: 'game.unknown.behavior', writesTo: '/gamePolicy/unknown/behavior' }],
      candidate: UnknownGameCandidate,
      unansweredFallback: 'deny',
    });
    const missingChildReasonRef = AppGameControlApprovalRequestSchema.safeParse({
      ...NewAppRequest,
      childReasonState: 'reason-ref-backed',
      childReasonReferences: [],
    });

    expect(unsafeGameFallback.success).toBe(false);
    expect(missingChildReasonRef.success).toBe(false);
  });
}

function keepsUnsupportedBlocksManualRequired() {
  it('keeps unsupported parent block outcomes manual-required instead of dispatching', () => {
    const manualRequiredResult = AppGameControlActionResultSchema.parse({
      schemaVersion: ParentContractSchemaVersion.V0_6,
      resultId: 'action-result-manual-required-block',
      request: {
        ...NewAppRequest,
        requestedAction: PolicyAction.Block,
        requestedMode: EnforcementMode.TerminateProcess,
      },
      decision: {
        ...approvalDecisionBase('approval-decision-block-if-supported'),
        responseScope: 'block-if-supported',
      },
      approvalState: 'approved',
      capabilityState: EnforcementCapabilityState.ManualRequired,
      capability: ManualRequiredCapability,
      evidenceProofKind: 'unknown-app',
      resultStatus: 'manual-required',
      enforcementResult: null,
      recordedAt: Timestamp,
    });
    const unsafeDispatch = AppGameControlActionResultSchema.safeParse({
      ...manualRequiredResult,
      capabilityState: EnforcementCapabilityState.Supported,
      capability: SupportedCapability,
      evidenceProofKind: 'unknown-app',
      resultStatus: 'dispatch-ready',
    });

    expect(manualRequiredResult.resultStatus).toBe('manual-required');
    expect(manualRequiredResult.enforcementResult).toBeNull();
    expect(unsafeDispatch.success).toBe(false);
  });
}

function rejectsMalformedResponseScopes() {
  it('rejects malformed response scope and replay state claims', () => {
    expect(
      AppGameControlApprovalDecisionSchema.safeParse({
        ...approvalDecisionBase('approval-decision-missing-expiry'),
        responseScope: 'allow-once',
        decisionExpiresAt: null,
      }).success
    ).toBe(false);
    expect(
      AppGameControlApprovalDecisionSchema.safeParse({
        ...approvalDecisionBase('approval-decision-replay-no-audit'),
        responseScope: 'allow-this-app-game',
        persistenceState: 'replayable',
        auditReferences: [],
      }).success
    ).toBe(false);
    expect(
      AppGameControlApprovalDecisionSchema.safeParse({
        ...approvalDecisionBase('approval-decision-storage-unavailable-with-audit'),
        responseScope: 'report-only',
        decisionState: 'manual-required',
        parentAction: null,
        persistenceState: 'storage-unavailable',
        auditReferences: ['audit-ref-that-cannot-exist'],
      }).success
    ).toBe(false);
  });
}

function approvalDecisionBase(decisionId: string) {
  return {
    schemaVersion: ParentContractSchemaVersion.V0_6,
    decisionId,
    requestId: NewAppRequest.requestId,
    policyKind: 'app-control',
    decisionState: 'approved',
    parentAction: ParentAction,
    reasonCodes: ['parent-approved-unknown-app'],
    policyVersion: PolicyVersion,
    evidenceReferences: [EvidenceReference],
    decidedAt: Timestamp,
  } as const;
}
