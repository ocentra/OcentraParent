import { describe, expect, it } from 'vitest';
import {
  EnforcementActionSchema,
  EnforcementAdapterKind,
  EnforcementCapabilityState,
  EnforcementCapabilityStatusSchema,
  EnforcementIntentSource,
  EnforcementIntentSchema,
  EnforcementMode,
  EnforcementTimerEventKind,
  EnforcementTimerEventSchema,
  EnforcementUnavailableReason,
} from '../src/enforcement';
import { ParentContractSchemaVersion, ParentEvidenceReferenceKind, ParentPlatform } from '../src/reference-primitives';

const observedAt = '2026-05-23T14:45:00.000Z';
const evidenceReference = {
  evidenceReferenceId: 'evidence-enforcement-1',
  kind: ParentEvidenceReferenceKind.ActivityEvent,
  observedAt,
};
const target = {
  targetId: 'target-process-1',
  targetType: 'process',
  targetValue: 'owned-child-process',
};

describe('parent enforcement timer contracts', () => {
  it('parses every enforcement timer transition state', () => {
    const action = enforcementAction(enforcementIntent());
    const transitions = [
      [EnforcementTimerEventKind.Created, 'created', false, null],
      [EnforcementTimerEventKind.Extended, 'extended', false, null],
      [EnforcementTimerEventKind.Expired, 'expired', false, null],
      [EnforcementTimerEventKind.Cancelled, 'cancelled', false, null],
      [EnforcementTimerEventKind.RestartRecovered, 'restart-recovered', true, null],
      [EnforcementTimerEventKind.RollbackRequested, 'rollback-requested', false, null],
      [EnforcementTimerEventKind.RollbackCompleted, 'rollback-completed', false, null],
      [EnforcementTimerEventKind.RecoveryNeeded, 'recovery-needed', false, EnforcementUnavailableReason.AdapterError],
      [EnforcementTimerEventKind.Unavailable, 'unavailable', false, EnforcementUnavailableReason.AdapterUnavailable],
    ] as const;

    const parsed = transitions.map(
      ([timerEventKind, expectedKind, recoveredAfterRestart, unavailableReason], index) => {
        const timer = EnforcementTimerEventSchema.parse({
          schemaVersion: ParentContractSchemaVersion.V0_6,
          timerEventId: `timer-transition-${index}`,
          timerEventKind,
          actionId: action.actionId,
          policyDecisionId: action.policyDecisionId,
          evidenceReferences: [evidenceReference],
          scheduledAt: observedAt,
          effectiveAt: '2026-05-23T15:00:00.000Z',
          rollbackToken: action.rollbackToken,
          recoveredAfterRestart,
          unavailableReason,
        });

        expect(timer.timerEventKind).toBe(expectedKind);
        expect(timer.recoveredAfterRestart).toBe(recoveredAfterRestart);
        expect(timer.unavailableReason).toBe(unavailableReason);
        return timer.timerEventKind;
      }
    );

    expect(parsed).toEqual(transitions.map(([, expectedKind]) => expectedKind));
    expect(() => EnforcementTimerEventSchema.parse(unavailableTimerWithoutReasonPayload())).toThrow();
    expect(() => EnforcementTimerEventSchema.parse(recoveryNeededTimerWithoutReasonPayload())).toThrow();
    expect(() => EnforcementTimerEventSchema.parse(createdTimerWithUnavailableReasonPayload())).toThrow();
  });
});

function unavailableTimerWithoutReasonPayload() {
  const action = enforcementAction(enforcementIntent());
  return {
    schemaVersion: ParentContractSchemaVersion.V0_6,
    timerEventId: 'timer-unavailable-missing-reason',
    timerEventKind: EnforcementTimerEventKind.Unavailable,
    actionId: action.actionId,
    policyDecisionId: action.policyDecisionId,
    evidenceReferences: [evidenceReference],
    scheduledAt: observedAt,
    effectiveAt: null,
    rollbackToken: action.rollbackToken,
    recoveredAfterRestart: false,
    unavailableReason: null,
  };
}

function recoveryNeededTimerWithoutReasonPayload() {
  const action = enforcementAction(enforcementIntent());
  return {
    schemaVersion: ParentContractSchemaVersion.V0_6,
    timerEventId: 'timer-recovery-needed-missing-reason',
    timerEventKind: EnforcementTimerEventKind.RecoveryNeeded,
    actionId: action.actionId,
    policyDecisionId: action.policyDecisionId,
    evidenceReferences: [evidenceReference],
    scheduledAt: observedAt,
    effectiveAt: null,
    rollbackToken: action.rollbackToken,
    recoveredAfterRestart: false,
    unavailableReason: null,
  };
}

function createdTimerWithUnavailableReasonPayload() {
  const action = enforcementAction(enforcementIntent());
  return {
    schemaVersion: ParentContractSchemaVersion.V0_6,
    timerEventId: 'timer-created-with-unavailable-reason',
    timerEventKind: EnforcementTimerEventKind.Created,
    actionId: action.actionId,
    policyDecisionId: action.policyDecisionId,
    evidenceReferences: [evidenceReference],
    scheduledAt: observedAt,
    effectiveAt: null,
    rollbackToken: action.rollbackToken,
    recoveredAfterRestart: false,
    unavailableReason: EnforcementUnavailableReason.AdapterUnavailable,
  };
}

function enforcementIntent() {
  return EnforcementIntentSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    intentId: 'intent-1',
    source: EnforcementIntentSource.LocalPolicyEvaluator,
    actor: null,
    device: {
      deviceId: 'child-device-1',
      childProfileId: 'child-1',
      label: 'Child Windows PC',
      platform: ParentPlatform.Windows,
    },
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
    capability: capabilityStatus(),
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
