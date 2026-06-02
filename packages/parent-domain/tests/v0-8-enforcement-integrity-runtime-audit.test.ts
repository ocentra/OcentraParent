import { describe, expect, it } from 'vitest';
import {
  V08EnforcementIntegrityRuntimeAuditEntrySchema,
  V08EnforcementIntegrityRuntimeAuditReadModel,
  V08EnforcementIntegrityRuntimeAuditReadModelSchema,
} from '../src/v0-8-enforcement-integrity-runtime-audit';

describe('V0.8 enforcement integrity runtime audit', () => {
  it('covers supported results rejections recovery integrity states and honest non-claims', () => {
    const readModel = V08EnforcementIntegrityRuntimeAuditReadModelSchema.parse(
      V08EnforcementIntegrityRuntimeAuditReadModel
    );

    expect(readModel.readModelId).toBe('v0-8-enforcement-integrity-runtime-audit');
    expect(readModel.entries).toHaveLength(14);
    expect(countBy(readModel.entries.map((entry) => entry.result))).toEqual({
      succeeded: 1,
      expired: 1,
      'rolled-back': 1,
      superseded: 1,
      'no-op': 1,
      failed: 2,
      'observe-only': 1,
      'manual-required': 2,
      unavailable: 3,
      unsupported: 1,
    });
    expect(countBy(readModel.entries.map((entry) => entry.execution))).toEqual({
      'executed-supported-boundary': 4,
      'dry-run-no-adapter-execution': 1,
      'rejected-before-adapter': 2,
      'observe-only-no-execution': 1,
      'manual-required-no-execution': 2,
      'unavailable-no-execution': 2,
      'recovery-needed-no-execution': 1,
      'unsupported-no-execution': 1,
    });
    expect(countBy(readModel.entries.map((entry) => entry.integrityState))).toEqual({
      running: 8,
      'not-applicable': 2,
      'permission-missing': 1,
      'adapter-unavailable': 1,
      'stale-heartbeat': 1,
      'tamper-signal-manual-required': 1,
    });
    expect(readModel.entries.every((entry) => !entry.broadInstalledAppBlockingClaimed)).toBe(true);
    expect(readModel.entries.every((entry) => !entry.hostNetworkDomainBlockingClaimed)).toBe(true);
    expect(readModel.entries.every((entry) => !entry.exactActiveTabEnforcementClaimed)).toBe(true);
    expect(readModel.entries.every((entry) => !entry.notificationDeliveryClaimed)).toBe(true);
    expect(readModel.entries.every((entry) => !entry.tamperHardeningClaimed)).toBe(true);
    expect(readModel.entries.every((entry) => !entry.mobilePrivilegeClaimed)).toBe(true);
    expect(readModel.entries.every((entry) => !entry.stealthPersistenceClaimed)).toBe(true);
    expect(readModel.entries.every((entry) => !entry.privilegeEscalationClaimed)).toBe(true);
  });

  it('links supported execution to policy evidence timer rollback child and audit refs', () => {
    const succeeded = entryFor('app-time-limit-action-succeeded');
    const expired = entryFor('app-time-limit-action-expired');
    const rolledBack = entryFor('app-time-limit-action-rolled-back');
    const override = entryFor('parent-override-superseded-action');

    expect(succeeded).toMatchObject({
      result: 'succeeded',
      execution: 'executed-supported-boundary',
      intentState: 'validated',
      timerState: 'active-timer-backed',
      rollbackState: 'rollback-token-backed',
      childState: 'reason-ref-backed',
      auditState: 'audit-backed',
    });
    expect(succeeded.policyDecisionRefs).toContain('policy-decision-ref');
    expect(succeeded.evidenceRefs).toContain('owned-process-identity-ref');
    expect(succeeded.adapterOutcomeRefs).toContain('adapter-outcome-ref');
    expect(succeeded.childStatusRefs).toContain('child-status-ref');
    expect(expired.timerState).toBe('expired-backed');
    expect(rolledBack.rollbackState).toBe('rollback-completed');
    expect(override.parentIntentRefs).toContain('parent-override-intent-ref');
  });

  it('proves dry-run observe-only stale wrong-device manual unavailable and unsupported paths do not execute adapters', () => {
    expect(entryFor('dry-run-preview-no-op')).toMatchObject({
      result: 'no-op',
      execution: 'dry-run-no-adapter-execution',
      intentState: 'observe-only',
    });
    expect(entryFor('network-domain-observe-only')).toMatchObject({
      surface: 'network-domain-observe-only',
      result: 'observe-only',
      execution: 'observe-only-no-execution',
    });
    expect(entryFor('stale-policy-decision-rejected')).toMatchObject({
      result: 'failed',
      execution: 'rejected-before-adapter',
      intentState: 'rejected-stale',
    });
    expect(entryFor('wrong-device-intent-rejected')).toMatchObject({
      result: 'failed',
      execution: 'rejected-before-adapter',
      intentState: 'rejected-wrong-device',
    });
    expect(entryFor('host-network-domain-filter-manual-required')).toMatchObject({
      result: 'manual-required',
      execution: 'manual-required-no-execution',
      surface: 'host-network-domain-filter',
    });
    expect(entryFor('adapter-unavailable-recovery-needed')).toMatchObject({
      result: 'unavailable',
      execution: 'recovery-needed-no-execution',
      timerState: 'recovery-needed',
    });
    expect(entryFor('mobile-child-control-unsupported')).toMatchObject({
      result: 'unsupported',
      execution: 'unsupported-no-execution',
      platform: 'ios',
    });
  });

  it('keeps permission heartbeat and tamper states explicit without anti-tamper claims', () => {
    const permissionLoss = entryFor('permission-loss-unavailable');
    const staleHeartbeat = entryFor('stale-integrity-heartbeat');
    const tamper = entryFor('tamper-uninstall-detection-manual-required');

    expect(permissionLoss).toMatchObject({
      result: 'unavailable',
      integrityState: 'permission-missing',
      auditState: 'audit-backed',
    });
    expect(staleHeartbeat).toMatchObject({
      result: 'unavailable',
      integrityState: 'stale-heartbeat',
      surface: 'integrity-heartbeat',
    });
    expect(tamper).toMatchObject({
      result: 'manual-required',
      integrityState: 'tamper-signal-manual-required',
      auditState: 'manual-required',
      tamperHardeningClaimed: false,
      stealthPersistenceClaimed: false,
      privilegeEscalationClaimed: false,
    });
    expect(tamper.manualProofRequirements).toContain('security review before hardening');
  });

  it('rejects claim upgrades and inconsistent result execution pairings', () => {
    const supported = entryFor('app-time-limit-action-succeeded');
    const dryRun = entryFor('dry-run-preview-no-op');

    for (const invalidEntry of [
      { ...supported, auditEntryId: 'invalid-broad-app-claim', broadInstalledAppBlockingClaimed: true },
      { ...supported, auditEntryId: 'invalid-host-domain-claim', hostNetworkDomainBlockingClaimed: true },
      { ...supported, auditEntryId: 'invalid-notification-claim', notificationDeliveryClaimed: true },
      { ...supported, auditEntryId: 'invalid-tamper-claim', tamperHardeningClaimed: true },
      { ...supported, auditEntryId: 'invalid-stealth-claim', stealthPersistenceClaimed: true },
      { ...supported, auditEntryId: 'invalid-privilege-claim', privilegeEscalationClaimed: true },
      { ...dryRun, auditEntryId: 'invalid-dry-run-execution', result: 'succeeded' },
      { ...supported, auditEntryId: 'invalid-supported-missing-audit', auditRefs: [] },
    ]) {
      expect(() => V08EnforcementIntegrityRuntimeAuditEntrySchema.parse(invalidEntry)).toThrow();
    }
  });
});

function entryFor(auditEntryId: string) {
  const entry = V08EnforcementIntegrityRuntimeAuditReadModel.entries.find(
    (candidate) => candidate.auditEntryId === auditEntryId
  );
  if (entry === undefined) {
    throw new Error(`Missing V0.8 enforcement integrity runtime audit entry: ${auditEntryId}`);
  }
  return entry;
}

function countBy(values: readonly string[]) {
  return values.reduce<Record<string, number>>((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}
