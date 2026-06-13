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
  EnforcementTimerEventKind,
  EnforcementTimerEventSchema,
  EnforcementUnavailableReason,
  EnforcementUnavailableStatusSchema,
} from '../../src/enforcement';
import { ParentContractSchemaVersion, ParentEvidenceReferenceKind, ParentPlatform } from '@ocentra-parent/family-domain/reference-primitives';

const observedAt = '2026-05-23T14:45:00.000Z';
const evidenceReference = {
  evidenceReferenceId: 'evidence-enforcement-1',
  kind: ParentEvidenceReferenceKind.ActivityEvent,
  observedAt,
};
const device = {
  deviceId: 'child-device-1',
  childProfileId: 'child-1',
  label: 'Child Windows PC',
  platform: ParentPlatform.Windows,
};
const target = {
  targetId: 'target-process-1',
  targetType: 'process',
  targetValue: 'owned-child-process',
};

describe('parent enforcement contracts', () => {
  it('parses the intent, action, result, audit, capability, and timer spine', () => {
    const capability = capabilityStatus();
    const intent = enforcementIntent();
    const action = enforcementAction(intent);
    const result = enforcementResult(action, capability);
    const audit = enforcementAudit(action, result);
    const timer = enforcementTimer(action);

    expect(audit.result.status).toBe('actually-enforced');
    expect(audit.result.rollbackState).toBe('available');
    expect(audit.capability.capabilityState).toBe('supported');
    expect(audit.unavailableStatus).toBeNull();
    expect(timer).toEqual({
      schemaVersion: 'v0.6',
      timerEventId: 'timer-1',
      timerEventKind: 'restart-recovered',
      actionId: 'action-1',
      policyDecisionId: 'decision-1',
      evidenceReferences: [evidenceReference],
      scheduledAt: observedAt,
      effectiveAt: '2026-05-23T15:00:00.000Z',
      rollbackToken: 'rollback-1',
      recoveredAfterRestart: true,
      unavailableReason: null,
    });
  });

  it('rejects raw, unsupported enforcement result statuses', () => {
    expect(() => EnforcementResultSchema.parse(unsupportedStatusPayload())).toThrow();
  });

  it('rejects unavailable results without typed unavailable status', () => {
    expect(() => EnforcementResultSchema.parse(unavailableResultWithoutStatus())).toThrow();
  });

  it('parses unavailable result status with typed capability detail', () => {
    const capability = unavailableCapabilityStatus();
    const intent = enforcementIntent();
    const action = enforcementAction(intent, capability);
    const unavailableStatus = enforcementUnavailableStatus(capability);
    const result = EnforcementResultSchema.parse({
      schemaVersion: ParentContractSchemaVersion.V0_6,
      resultId: 'result-2',
      actionId: action.actionId,
      status: EnforcementResultStatus.Unavailable,
      adapterResultCode: EnforcementAdapterResultCode.UnsupportedPlatform,
      startedAt: observedAt,
      completedAt: observedAt,
      rollbackToken: null,
      rollbackState: EnforcementRollbackState.Unavailable,
      unavailableReason: EnforcementUnavailableReason.UnsupportedPlatform,
      unavailableStatus,
      failedReason: null,
      nextCheckAt: null,
      capability,
    });

    expect(result.unavailableStatus).toEqual(unavailableStatus);
    expect(result.unavailableStatus?.retryable).toBe(false);
    expect(result.unavailableStatus?.capability.supportedActions).toEqual([]);
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
    supportedActions: [EnforcementMode.TerminateProcess, EnforcementMode.TemporaryBlock],
    degradedReason: null,
    lastCheckedAt: observedAt,
  });
}

function unavailableCapabilityStatus() {
  return EnforcementCapabilityStatusSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    platform: ParentPlatform.Linux,
    adapterKind: EnforcementAdapterKind.ProcessControl,
    capabilityState: EnforcementCapabilityState.Unavailable,
    permissionState: 'not-required',
    dependencyState: 'not-required',
    supportedActions: [],
    degradedReason: EnforcementUnavailableReason.UnsupportedPlatform,
    lastCheckedAt: observedAt,
  });
}

function enforcementUnavailableStatus(capability: ReturnType<typeof unavailableCapabilityStatus>) {
  return EnforcementUnavailableStatusSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    capability,
    unavailableReason: EnforcementUnavailableReason.UnsupportedPlatform,
    retryable: false,
    checkedAt: observedAt,
  });
}

function unavailableResultWithoutStatus() {
  const capability = unavailableCapabilityStatus();
  return {
    schemaVersion: ParentContractSchemaVersion.V0_6,
    resultId: 'result-2',
    actionId: 'action-1',
    status: EnforcementResultStatus.Unavailable,
    adapterResultCode: EnforcementAdapterResultCode.UnsupportedPlatform,
    startedAt: observedAt,
    completedAt: observedAt,
    rollbackToken: null,
    rollbackState: EnforcementRollbackState.Unavailable,
    unavailableReason: EnforcementUnavailableReason.UnsupportedPlatform,
    unavailableStatus: null,
    failedReason: null,
    nextCheckAt: null,
    capability,
  };
}

function enforcementIntent() {
  return EnforcementIntentSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    intentId: 'intent-1',
    source: EnforcementIntentSource.LocalPolicyEvaluator,
    actor: null,
    device,
    policyDecisionId: 'decision-1',
    target,
    requestedAction: 'block',
    evidenceReferences: [evidenceReference],
    parentApproval: null,
    idempotencyKey: 'decision-1:target-process-1',
  });
}

function enforcementAction(intent: ReturnType<typeof enforcementIntent>, capability = capabilityStatus()) {
  return EnforcementActionSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    actionId: 'action-1',
    intentId: intent.intentId,
    policyDecisionId: intent.policyDecisionId,
    policyAction: intent.requestedAction,
    adapterKind: EnforcementAdapterKind.ProcessControl,
    platform: ParentPlatform.Windows,
    target,
    mode: EnforcementMode.TerminateProcess,
    capability,
    reasonCodes: ['policy-blocked-process'],
    evidenceReferences: [evidenceReference],
    localAiResultId: null,
    parentApproval: null,
    dryRun: false,
    requestedAt: observedAt,
    expiresAt: '2026-05-23T15:00:00.000Z',
    rollbackToken: 'rollback-1',
  });
}

function enforcementResult(
  action: ReturnType<typeof enforcementAction>,
  capability: ReturnType<typeof capabilityStatus>
) {
  return EnforcementResultSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    resultId: 'result-1',
    actionId: action.actionId,
    status: EnforcementResultStatus.ActuallyEnforced,
    adapterResultCode: EnforcementAdapterResultCode.ProcessTerminated,
    startedAt: observedAt,
    completedAt: '2026-05-23T14:45:01.000Z',
    rollbackToken: action.rollbackToken,
    rollbackState: EnforcementRollbackState.Available,
    unavailableReason: null,
    unavailableStatus: null,
    failedReason: null,
    nextCheckAt: null,
    capability,
  });
}

function enforcementAudit(action: ReturnType<typeof enforcementAction>, result: ReturnType<typeof enforcementResult>) {
  return EnforcementAuditEventSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    auditEventId: 'audit-1',
    auditEventKind: EnforcementAuditEventKind.Succeeded,
    action,
    result,
    capability: result.capability,
    unavailableStatus: result.unavailableStatus,
    policyVersion: 'policy-version-1',
    evidenceReferences: [evidenceReference],
    actor: null,
    parentOverride: null,
    journalSequence: 'journal-sequence-1',
    observedAt: '2026-05-23T14:45:02.000Z',
  });
}

function enforcementTimer(action: ReturnType<typeof enforcementAction>) {
  return EnforcementTimerEventSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    timerEventId: 'timer-1',
    timerEventKind: EnforcementTimerEventKind.RestartRecovered,
    actionId: action.actionId,
    policyDecisionId: action.policyDecisionId,
    evidenceReferences: [evidenceReference],
    scheduledAt: observedAt,
    effectiveAt: '2026-05-23T15:00:00.000Z',
    rollbackToken: action.rollbackToken,
    recoveredAfterRestart: true,
    unavailableReason: null,
  });
}

function unsupportedStatusPayload() {
  return {
    schemaVersion: ParentContractSchemaVersion.V0_6,
    resultId: 'result-1',
    actionId: 'action-1',
    status: 'blocked-by-label',
    adapterResultCode: EnforcementAdapterResultCode.NoOp,
    startedAt: observedAt,
    completedAt: null,
    rollbackToken: null,
    rollbackState: EnforcementRollbackState.NotRequired,
    unavailableReason: null,
    unavailableStatus: null,
    failedReason: null,
    nextCheckAt: null,
    capability: {
      schemaVersion: ParentContractSchemaVersion.V0_6,
      platform: ParentPlatform.Windows,
      adapterKind: EnforcementAdapterKind.ProcessControl,
      capabilityState: EnforcementCapabilityState.ObserveOnly,
      permissionState: 'not-required',
      dependencyState: 'installed',
      supportedActions: [EnforcementMode.ObserveOnly],
      degradedReason: null,
      lastCheckedAt: observedAt,
    },
  };
}
