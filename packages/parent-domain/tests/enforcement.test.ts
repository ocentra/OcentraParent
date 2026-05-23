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
} from '../src/enforcement';
import { ParentContractSchemaVersion, ParentEvidenceReferenceKind, ParentPlatform } from '../src/reference-primitives';

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

function enforcementAction(intent: ReturnType<typeof enforcementIntent>) {
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
