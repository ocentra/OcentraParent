import { describe, expect, it } from 'vitest';
import {
  EnforcementActionSchema,
  EnforcementAdapterKind,
  EnforcementAdapterResultCode,
  EnforcementAuditEventKind,
  EnforcementAuditEventSchema,
  EnforcementCapabilityState,
  type EnforcementCapabilityStatus,
  EnforcementCapabilityStatusSchema,
  EnforcementMode,
  EnforcementResultSchema,
  EnforcementResultStatus,
  EnforcementRollbackState,
  EnforcementTimerEventKind,
  EnforcementTimerEventSchema,
  EnforcementUnavailableReason,
  type EnforcementUnavailableStatus,
  EnforcementUnavailableStatusSchema,
} from '@ocentra-parent/schema-domain/enforcement';
import {
  ParentContractSchemaVersion,
  ParentEvidenceReferenceKind,
  ParentPlatform,
} from '@ocentra-parent/schema-domain/family-reference-primitives';

const observedAt = '2026-05-24T18:50:00.000Z';
const expiresAt = '2026-05-24T19:05:00.000Z';
const evidenceReference = {
  evidenceReferenceId: 'evidence-enforcement-permission-dependency-1',
  kind: ParentEvidenceReferenceKind.ActivityEvent,
  observedAt,
};
const unavailableCases = [
  {
    label: 'missing permission',
    permissionState: 'missing-permission',
    dependencyState: 'installed',
    unavailableReason: EnforcementUnavailableReason.MissingPermission,
  },
  {
    label: 'missing dependency',
    permissionState: 'allowed',
    dependencyState: 'missing',
    unavailableReason: EnforcementUnavailableReason.MissingDependency,
  },
] as const;

type EnforcementUnavailableReasonValue =
  (typeof EnforcementUnavailableReason)[keyof typeof EnforcementUnavailableReason];

describe('parent enforcement permission and dependency unavailable contracts', () => {
  for (const unavailableCase of unavailableCases) {
    it(`carries ${unavailableCase.label} as unavailable recovery data`, () => {
      const capability = EnforcementCapabilityStatusSchema.parse(capabilityInput(unavailableCase));
      const action = enforcementAction(capability);
      const unavailableStatus = EnforcementUnavailableStatusSchema.parse({
        schemaVersion: ParentContractSchemaVersion.V0_6,
        capability,
        unavailableReason: unavailableCase.unavailableReason,
        retryable: false,
        checkedAt: observedAt,
      });
      const result = enforcementResult(action, capability, unavailableStatus);
      const audit = enforcementAudit(action, result);
      const timer = enforcementTimer(action, unavailableCase.unavailableReason);

      expect(result.status).toBe(EnforcementResultStatus.Unavailable);
      expect(result.adapterResultCode).toBe(EnforcementAdapterResultCode.AdapterUnavailable);
      expect(result.unavailableReason).toBe(unavailableCase.unavailableReason);
      expect(result.unavailableStatus?.retryable).toBe(false);
      expect(audit.auditEventKind).toBe(EnforcementAuditEventKind.Unavailable);
      expect(audit.unavailableStatus?.unavailableReason).toBe(unavailableCase.unavailableReason);
      expect(timer.timerEventKind).toBe(EnforcementTimerEventKind.Unavailable);
      expect(timer.unavailableReason).toBe(unavailableCase.unavailableReason);
    });
  }
});

function capabilityInput(unavailableCase: (typeof unavailableCases)[number]) {
  return {
    schemaVersion: ParentContractSchemaVersion.V0_6,
    platform: ParentPlatform.Windows,
    adapterKind: EnforcementAdapterKind.ProcessControl,
    capabilityState: EnforcementCapabilityState.Unavailable,
    permissionState: unavailableCase.permissionState,
    dependencyState: unavailableCase.dependencyState,
    supportedActions: [],
    degradedReason: unavailableCase.unavailableReason,
    lastCheckedAt: observedAt,
  };
}

function enforcementAction(capability: EnforcementCapabilityStatus) {
  return EnforcementActionSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    actionId: 'action-permission-dependency-1',
    intentId: 'intent-permission-dependency-1',
    policyDecisionId: 'decision-permission-dependency-1',
    policyAction: 'block',
    adapterKind: EnforcementAdapterKind.ProcessControl,
    platform: ParentPlatform.Windows,
    target: {
      targetId: 'target-process-permission-dependency-1',
      targetType: 'process',
      targetValue: 'owned-child-process',
    },
    mode: EnforcementMode.TerminateProcess,
    capability,
    reasonCodes: ['policy-blocked-process'],
    evidenceReferences: [evidenceReference],
    localAiResultId: null,
    parentApproval: null,
    dryRun: false,
    requestedAt: observedAt,
    expiresAt,
    rollbackToken: 'rollback-permission-dependency-1',
  });
}

function enforcementResult(
  action: ReturnType<typeof enforcementAction>,
  capability: EnforcementCapabilityStatus,
  unavailableStatus: EnforcementUnavailableStatus
) {
  return EnforcementResultSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    resultId: 'result-permission-dependency-1',
    actionId: action.actionId,
    status: EnforcementResultStatus.Unavailable,
    adapterResultCode: EnforcementAdapterResultCode.AdapterUnavailable,
    startedAt: observedAt,
    completedAt: observedAt,
    rollbackToken: action.rollbackToken,
    rollbackState: EnforcementRollbackState.Unavailable,
    unavailableReason: unavailableStatus.unavailableReason,
    unavailableStatus,
    failedReason: null,
    nextCheckAt: action.expiresAt,
    capability,
  });
}

function enforcementAudit(action: ReturnType<typeof enforcementAction>, result: ReturnType<typeof enforcementResult>) {
  return EnforcementAuditEventSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    auditEventId: 'audit-permission-dependency-1',
    auditEventKind: EnforcementAuditEventKind.Unavailable,
    action,
    result,
    capability: result.capability,
    unavailableStatus: result.unavailableStatus,
    policyVersion: 'policy-version-permission-dependency-1',
    evidenceReferences: [evidenceReference],
    actor: null,
    parentOverride: null,
    journalSequence: 'journal-sequence-permission-dependency-1',
    observedAt,
  });
}

function enforcementTimer(action: ReturnType<typeof enforcementAction>, reason: EnforcementUnavailableReasonValue) {
  return EnforcementTimerEventSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    timerEventId: 'timer-permission-dependency-1',
    timerEventKind: EnforcementTimerEventKind.Unavailable,
    actionId: action.actionId,
    policyDecisionId: action.policyDecisionId,
    evidenceReferences: [evidenceReference],
    scheduledAt: observedAt,
    effectiveAt: null,
    rollbackToken: action.rollbackToken,
    recoveredAfterRestart: false,
    unavailableReason: reason,
  });
}
