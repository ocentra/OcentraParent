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
} from '../../src/enforcement';
import { ParentContractSchemaVersion, ParentEvidenceReferenceKind, ParentPlatform } from '@ocentra-parent/family-domain/reference-primitives';

const observedAt = '2026-05-24T17:55:00.000Z';
const evidenceReference = {
  evidenceReferenceId: 'evidence-enforcement-unavailable-1',
  kind: ParentEvidenceReferenceKind.ActivityEvent,
  observedAt,
};

describe('parent enforcement unavailable adapter contracts', () => {
  it('requires degraded reason and carries unavailable audit plus timer data', () => {
    expect(() => EnforcementCapabilityStatusSchema.parse(degradedCapability(null))).toThrow();

    const capability = EnforcementCapabilityStatusSchema.parse(
      degradedCapability(EnforcementUnavailableReason.AdapterUnavailable)
    );
    const action = enforcementAction(capability);
    const unavailableStatus = EnforcementUnavailableStatusSchema.parse({
      schemaVersion: ParentContractSchemaVersion.V0_6,
      capability,
      unavailableReason: EnforcementUnavailableReason.AdapterUnavailable,
      retryable: true,
      checkedAt: observedAt,
    });
    const result = enforcementResult(action, capability, unavailableStatus);
    const audit = enforcementAudit(action, result);
    const timer = enforcementTimer(action);

    expect(result.capability.capabilityState).toBe(EnforcementCapabilityState.Degraded);
    expect(result.unavailableStatus?.retryable).toBe(true);
    expect(audit.auditEventKind).toBe(EnforcementAuditEventKind.Unavailable);
    expect(audit.unavailableStatus?.unavailableReason).toBe(EnforcementUnavailableReason.AdapterUnavailable);
    expect(timer.timerEventKind).toBe(EnforcementTimerEventKind.Unavailable);
    expect(timer.unavailableReason).toBe(EnforcementUnavailableReason.AdapterUnavailable);
  });

  it('parses manual-required adapter state without pretending enforcement was applied', () => {
    const capability = EnforcementCapabilityStatusSchema.parse({
      schemaVersion: ParentContractSchemaVersion.V0_6,
      platform: ParentPlatform.Windows,
      adapterKind: EnforcementAdapterKind.NetworkControl,
      capabilityState: EnforcementCapabilityState.ManualRequired,
      permissionState: 'unknown',
      dependencyState: 'unknown',
      supportedActions: [EnforcementMode.TemporaryBlock],
      degradedReason: EnforcementUnavailableReason.ManualRequired,
      lastCheckedAt: observedAt,
    });
    const action = enforcementAction(capability, EnforcementAdapterKind.NetworkControl, EnforcementMode.TemporaryBlock);
    const unavailableStatus = EnforcementUnavailableStatusSchema.parse({
      schemaVersion: ParentContractSchemaVersion.V0_6,
      capability,
      unavailableReason: EnforcementUnavailableReason.ManualRequired,
      retryable: false,
      checkedAt: observedAt,
    });
    const result = enforcementResult(action, capability, unavailableStatus);

    expect(result.status).toBe(EnforcementResultStatus.Unavailable);
    expect(result.capability.capabilityState).toBe(EnforcementCapabilityState.ManualRequired);
    expect(result.unavailableReason).toBe(EnforcementUnavailableReason.ManualRequired);
    expect(result.unavailableStatus?.unavailableReason).toBe(EnforcementUnavailableReason.ManualRequired);
    expect(result.unavailableStatus?.retryable).toBe(false);
  });
});

function degradedCapability(degradedReason: unknown) {
  return {
    schemaVersion: ParentContractSchemaVersion.V0_6,
    platform: ParentPlatform.Windows,
    adapterKind: EnforcementAdapterKind.ProcessControl,
    capabilityState: EnforcementCapabilityState.Degraded,
    permissionState: 'allowed',
    dependencyState: 'installed',
    supportedActions: [EnforcementMode.TerminateProcess],
    degradedReason,
    lastCheckedAt: observedAt,
  };
}

function enforcementAction(
  capability: EnforcementCapabilityStatus,
  adapterKind = EnforcementAdapterKind.ProcessControl,
  mode = EnforcementMode.TerminateProcess
) {
  return EnforcementActionSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    actionId: 'action-unavailable-1',
    intentId: 'intent-unavailable-1',
    policyDecisionId: 'decision-unavailable-1',
    policyAction: 'block',
    adapterKind,
    platform: ParentPlatform.Windows,
    target: {
      targetId: 'target-process-unavailable-1',
      targetType: 'process',
      targetValue: 'owned-child-process',
    },
    mode,
    capability,
    reasonCodes: ['policy-blocked-process'],
    evidenceReferences: [evidenceReference],
    localAiResultId: null,
    parentApproval: null,
    dryRun: false,
    requestedAt: observedAt,
    expiresAt: '2026-05-24T18:10:00.000Z',
    rollbackToken: 'rollback-unavailable-1',
  });
}

function enforcementResult(
  action: ReturnType<typeof enforcementAction>,
  capability: EnforcementCapabilityStatus,
  unavailableStatus: EnforcementUnavailableStatus
) {
  return EnforcementResultSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    resultId: 'result-unavailable-1',
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
    auditEventId: 'audit-unavailable-1',
    auditEventKind: EnforcementAuditEventKind.Unavailable,
    action,
    result,
    capability: result.capability,
    unavailableStatus: result.unavailableStatus,
    policyVersion: 'policy-version-unavailable-1',
    evidenceReferences: [evidenceReference],
    actor: null,
    parentOverride: null,
    journalSequence: 'journal-sequence-unavailable-1',
    observedAt,
  });
}

function enforcementTimer(action: ReturnType<typeof enforcementAction>) {
  return EnforcementTimerEventSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    timerEventId: 'timer-unavailable-1',
    timerEventKind: EnforcementTimerEventKind.Unavailable,
    actionId: action.actionId,
    policyDecisionId: action.policyDecisionId,
    evidenceReferences: [evidenceReference],
    scheduledAt: observedAt,
    effectiveAt: null,
    rollbackToken: action.rollbackToken,
    recoveredAfterRestart: false,
    unavailableReason: EnforcementUnavailableReason.AdapterUnavailable,
  });
}
