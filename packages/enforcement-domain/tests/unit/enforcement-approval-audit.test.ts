import { describe, expect, it } from 'vitest';
import {
  EnforcementActionSchema,
  EnforcementAdapterKind,
  EnforcementAdapterResultCode,
  EnforcementAuditEventKind,
  EnforcementAuditEventSchema,
  EnforcementCapabilityState,
  EnforcementCapabilityStatusSchema,
  EnforcementIntentSource,
  EnforcementIntentSchema,
  EnforcementMode,
  EnforcementResultSchema,
  EnforcementResultStatus,
  EnforcementRollbackState,
} from '../../src/enforcement';
import {
  ParentActorRole,
  ParentContractSchemaVersion,
  ParentEvidenceReferenceKind,
  ParentPlatform,
} from '../../src/reference-primitives';

const observedAt = '2026-05-24T22:55:00.000Z';
const expiresAt = '2026-05-24T23:25:00.000Z';
const parentActor = {
  actorId: 'parent-approval-actor-1',
  role: ParentActorRole.Parent,
};
const parentApproval = {
  actionReferenceId: 'parent-approval-action-1',
  actor: parentActor,
  policyVersion: 'policy-version-approval-1',
  createdAt: observedAt,
};
const evidenceReference = {
  evidenceReferenceId: 'evidence-parent-approval-1',
  kind: ParentEvidenceReferenceKind.PolicyDecision,
  observedAt,
};
const device = {
  deviceId: 'child-device-approval-1',
  childProfileId: 'child-profile-approval-1',
  label: 'Child approval test device',
  platform: ParentPlatform.Windows,
};
const target = {
  targetId: 'target-parent-approval-process-1',
  targetType: 'process',
  targetValue: 'parent-approved-process',
};

describe('parent enforcement approval audit contracts', () => {
  it('carries parent approval and override references through intent, action, and audit data', () => {
    const intent = approvalIntent();
    const action = approvalAction(intent);
    const result = approvalResult(action);
    const audit = approvalAudit(action, result);

    expect(intent.actor).toEqual(parentActor);
    expect(intent.parentApproval).toEqual(parentApproval);
    expect(action.parentApproval).toEqual(intent.parentApproval);
    expect(audit.actor).toEqual(parentActor);
    expect(audit.parentOverride).toEqual(parentApproval);
    expect(audit.action.parentApproval).toEqual(parentApproval);
    expect(audit.result.status).toBe(EnforcementResultStatus.ActuallyEnforced);
    expect(audit.parentOverride?.actionReferenceId).toBe(parentApproval.actionReferenceId);
  });

  it('rejects malformed parent override reference data', () => {
    const action = approvalAction(approvalIntent());
    const result = approvalResult(action);
    const malformedAudit = {
      ...approvalAudit(action, result),
      parentOverride: {
        ...parentApproval,
        actor: {
          ...parentActor,
          role: 'owner',
        },
      },
    };

    expect(() => EnforcementAuditEventSchema.parse(malformedAudit)).toThrow();
  });
});

function capabilityStatus() {
  return EnforcementCapabilityStatusSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    platform: ParentPlatform.Windows,
    adapterKind: EnforcementAdapterKind.ProcessControl,
    capabilityState: EnforcementCapabilityState.Supported,
    permissionState: 'not-required',
    dependencyState: 'installed',
    supportedActions: [EnforcementMode.TerminateProcess],
    degradedReason: null,
    lastCheckedAt: observedAt,
  });
}

function approvalIntent() {
  return EnforcementIntentSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    intentId: 'intent-parent-approval-1',
    source: EnforcementIntentSource.ParentPortal,
    actor: parentActor,
    device,
    policyDecisionId: 'decision-parent-approval-1',
    target,
    requestedAction: 'block',
    evidenceReferences: [evidenceReference],
    parentApproval,
    idempotencyKey: 'decision-parent-approval-1:target-parent-approval-process-1',
  });
}

function approvalAction(intent: ReturnType<typeof approvalIntent>) {
  const capability = capabilityStatus();

  return EnforcementActionSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    actionId: 'action-parent-approval-1',
    intentId: intent.intentId,
    policyDecisionId: intent.policyDecisionId,
    policyAction: intent.requestedAction,
    adapterKind: EnforcementAdapterKind.ProcessControl,
    platform: ParentPlatform.Windows,
    target,
    mode: EnforcementMode.TerminateProcess,
    capability,
    reasonCodes: ['parent-approved-block'],
    evidenceReferences: [evidenceReference],
    localAiResultId: null,
    parentApproval: intent.parentApproval,
    dryRun: false,
    requestedAt: observedAt,
    expiresAt,
    rollbackToken: 'rollback-parent-approval-1',
  });
}

function approvalResult(action: ReturnType<typeof approvalAction>) {
  return EnforcementResultSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    resultId: 'result-parent-approval-1',
    actionId: action.actionId,
    status: EnforcementResultStatus.ActuallyEnforced,
    adapterResultCode: EnforcementAdapterResultCode.ProcessTerminated,
    startedAt: observedAt,
    completedAt: observedAt,
    rollbackToken: action.rollbackToken,
    rollbackState: EnforcementRollbackState.Available,
    unavailableReason: null,
    unavailableStatus: null,
    failedReason: null,
    nextCheckAt: null,
    capability: action.capability,
  });
}

function approvalAudit(action: ReturnType<typeof approvalAction>, result: ReturnType<typeof approvalResult>) {
  return EnforcementAuditEventSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    auditEventId: 'audit-parent-approval-1',
    auditEventKind: EnforcementAuditEventKind.Succeeded,
    action,
    result,
    capability: result.capability,
    unavailableStatus: result.unavailableStatus,
    policyVersion: parentApproval.policyVersion,
    evidenceReferences: [evidenceReference],
    actor: parentActor,
    parentOverride: action.parentApproval,
    journalSequence: 'journal-parent-approval-1',
    observedAt,
  });
}
