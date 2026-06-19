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
  EnforcementUnavailableReason,
  type EnforcementUnavailableStatus,
  EnforcementUnavailableStatusSchema,
} from '../../src/enforcement';
import { ParentContractSchemaVersion, ParentEvidenceReferenceKind, ParentPlatform } from '@ocentra-parent/schema-domain/family-reference-primitives';

const observedAt = '2026-05-24T21:20:00.000Z';
const evidenceReference = {
  evidenceReferenceId: 'evidence-enforcement-audit-boundary-1',
  kind: ParentEvidenceReferenceKind.ActivityEvent,
  observedAt,
};
type EnforcementAuditEventKindValue = (typeof EnforcementAuditEventKind)[keyof typeof EnforcementAuditEventKind];

describe('parent enforcement audit boundary contracts', () => {
  it('requires unavailable audit events to mirror result capability and unavailable status', () => {
    const capability = unavailableCapability();
    const unavailableStatus = unavailableStatusFor(capability);
    const action = enforcementAction(capability);
    const result = unavailableResult(action, capability, unavailableStatus);
    const audit = EnforcementAuditEventSchema.parse(enforcementAudit(action, result, capability, unavailableStatus));

    expect(audit.auditEventKind).toBe(EnforcementAuditEventKind.Unavailable);
    expect(audit.capability).toEqual(result.capability);
    expect(audit.unavailableStatus).toEqual(result.unavailableStatus);
    expect(audit.unavailableStatus?.unavailableReason).toBe(EnforcementUnavailableReason.AdapterUnavailable);
  });

  it('rejects unavailable results that are audited as successful enforcement', () => {
    const capability = unavailableCapability();
    const unavailableStatus = unavailableStatusFor(capability);
    const action = enforcementAction(capability);
    const result = unavailableResult(action, capability, unavailableStatus);

    expectAuditRejected(
      enforcementAudit(action, result, capability, unavailableStatus, {
        auditEventKind: EnforcementAuditEventKind.Succeeded,
      })
    );
  });

  it('rejects unavailable audit events when the result did not carry unavailable state', () => {
    const capability = supportedCapability();
    const action = enforcementAction(capability);
    const result = enforcedResult(action, capability);

    expectAuditRejected(
      enforcementAudit(action, result, capability, null, {
        auditEventKind: EnforcementAuditEventKind.Unavailable,
      })
    );
  });

  it('rejects audit capability state that diverges from the carried result capability', () => {
    const capability = unavailableCapability();
    const unavailableStatus = unavailableStatusFor(capability);
    const action = enforcementAction(capability);
    const result = unavailableResult(action, capability, unavailableStatus);
    const mismatchedCapability = supportedCapability();

    expectAuditRejected(enforcementAudit(action, result, mismatchedCapability, unavailableStatus));
  });

  it('rejects unavailable audit status that diverges from the result unavailable status', () => {
    const capability = unavailableCapability();
    const unavailableStatus = unavailableStatusFor(capability);
    const action = enforcementAction(capability);
    const result = unavailableResult(action, capability, unavailableStatus);
    const mismatchedUnavailableStatus = EnforcementUnavailableStatusSchema.parse({
      ...unavailableStatus,
      retryable: false,
    });

    expectAuditRejected(enforcementAudit(action, result, capability, mismatchedUnavailableStatus));
  });
});

function supportedCapability() {
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

function unavailableCapability() {
  return EnforcementCapabilityStatusSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    platform: ParentPlatform.Windows,
    adapterKind: EnforcementAdapterKind.ProcessControl,
    capabilityState: EnforcementCapabilityState.Degraded,
    permissionState: 'allowed',
    dependencyState: 'installed',
    supportedActions: [EnforcementMode.TerminateProcess],
    degradedReason: EnforcementUnavailableReason.AdapterUnavailable,
    lastCheckedAt: observedAt,
  });
}

function unavailableStatusFor(capability: EnforcementCapabilityStatus) {
  return EnforcementUnavailableStatusSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    capability,
    unavailableReason: EnforcementUnavailableReason.AdapterUnavailable,
    retryable: true,
    checkedAt: observedAt,
  });
}

function enforcementAction(capability: EnforcementCapabilityStatus) {
  return EnforcementActionSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    actionId: 'action-audit-boundary-1',
    intentId: 'intent-audit-boundary-1',
    policyDecisionId: 'decision-audit-boundary-1',
    policyAction: 'block',
    adapterKind: EnforcementAdapterKind.ProcessControl,
    platform: ParentPlatform.Windows,
    target: {
      targetId: 'target-audit-boundary-1',
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
    expiresAt: '2026-05-24T21:35:00.000Z',
    rollbackToken: 'rollback-audit-boundary-1',
  });
}

function unavailableResult(
  action: ReturnType<typeof enforcementAction>,
  capability: EnforcementCapabilityStatus,
  unavailableStatus: EnforcementUnavailableStatus
) {
  return EnforcementResultSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    resultId: 'result-audit-boundary-1',
    actionId: action.actionId,
    status: EnforcementResultStatus.Unavailable,
    adapterResultCode: EnforcementAdapterResultCode.AdapterUnavailable,
    startedAt: observedAt,
    completedAt: observedAt,
    rollbackToken: action.rollbackToken,
    rollbackState: EnforcementRollbackState.Unavailable,
    unavailableReason: EnforcementUnavailableReason.AdapterUnavailable,
    unavailableStatus,
    failedReason: null,
    nextCheckAt: action.expiresAt,
    capability,
  });
}

function enforcedResult(action: ReturnType<typeof enforcementAction>, capability: EnforcementCapabilityStatus) {
  return EnforcementResultSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    resultId: 'result-audit-boundary-success-1',
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
    capability,
  });
}

function enforcementAudit(
  action: ReturnType<typeof enforcementAction>,
  result: ReturnType<typeof unavailableResult> | ReturnType<typeof enforcedResult>,
  capability: EnforcementCapabilityStatus,
  unavailableStatus: EnforcementUnavailableStatus | null,
  overrides: Partial<{ auditEventKind: EnforcementAuditEventKindValue }> = {}
) {
  return {
    schemaVersion: ParentContractSchemaVersion.V0_6,
    auditEventId: 'audit-audit-boundary-1',
    auditEventKind: overrides.auditEventKind ?? EnforcementAuditEventKind.Unavailable,
    action,
    result,
    capability,
    unavailableStatus,
    policyVersion: 'policy-version-audit-boundary-1',
    evidenceReferences: [evidenceReference],
    actor: null,
    parentOverride: null,
    journalSequence: 'journal-sequence-audit-boundary-1',
    observedAt,
  };
}

function expectAuditRejected(audit: ReturnType<typeof enforcementAudit>) {
  expect(() => EnforcementAuditEventSchema.parse(audit)).toThrow();
}
