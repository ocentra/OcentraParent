import { describe, expect, it } from 'vitest';
import {
  V08IntegrityAlertStatusBridgeEntrySchema,
  V08IntegrityAlertStatusBridgeReadModel,
  V08IntegrityAlertStatusBridgeReadModelSchema,
} from '../src/v0-8-integrity-alert-status-bridge';

describe('V0.8 integrity alert status bridge', () => {
  registerBridgeSummaryTest();
  registerBridgeRefsTest();
  registerBridgeRejectionTest();
});

function registerBridgeSummaryTest() {
  it('covers permission stale stopped removed and tamper statuses without delivery or hardening claims', () => {
    const readModel = V08IntegrityAlertStatusBridgeReadModelSchema.parse(V08IntegrityAlertStatusBridgeReadModel);

    expect(readModel.readModelId).toBe('v0-8-integrity-alert-status-bridge');
    expect(readModel.entries).toHaveLength(4);
    expect(countBy(readModel.entries.map((entry) => entry.integrityAlertState))).toEqual({
      'permission-loss': 1,
      'stale-heartbeat': 1,
      'stopped-or-removed': 1,
      'tamper-manual-required': 1,
    });
    expect(countBy(readModel.entries.map((entry) => entry.parentVisibleStatus))).toEqual({
      'permission-action-required': 1,
      'agent-heartbeat-stale': 1,
      'agent-stopped-or-removed': 1,
      'tamper-review-required': 1,
    });
    expect(readModel.entries.every((entry) => entry.deliveryState === 'not-delivered-provider-not-configured')).toBe(
      true
    );
    expect(readModel.entries.every((entry) => !entry.providerDeliveryClaimed)).toBe(true);
    expect(readModel.entries.every((entry) => !entry.broadBlockingClaimed)).toBe(true);
    expect(readModel.entries.every((entry) => !entry.tamperResistanceClaimed)).toBe(true);
    expect(readModel.entries.every((entry) => !entry.mobileEnforcementClaimed)).toBe(true);
    expect(readModel.entries.every((entry) => !entry.stealthPersistenceClaimed)).toBe(true);
    expect(readModel.entries.every((entry) => !entry.privilegeEscalationClaimed)).toBe(true);
  });
}

function registerBridgeRefsTest() {
  it('links notification intent status audit integrity and drill-in references for each parent-visible state', () => {
    const permission = entryFor('permission-loss-alert-status');
    const stale = entryFor('stale-heartbeat-alert-status');
    const stopped = entryFor('stopped-or-removed-alert-status');
    const tamper = entryFor('tamper-manual-alert-status');

    expect(permission.notificationIntentRefs).toContain('notification-intent-permission-loss-ref');
    expect(stale.integrityRefs).toContain('integrity-heartbeat-ref');
    expect(stopped.manualProofRequirements).toContain('uninstall detection artifact');
    expect(tamper.auditState).toBe('manual-required');
    expect(tamper.manualProofRequirements).toContain('security review before hardening');

    for (const entry of V08IntegrityAlertStatusBridgeReadModel.entries) {
      expect(entry.auditRefs.length).toBeGreaterThan(0);
      expect(entry.notificationStatusRefs).toContain('notification-status-provider-not-configured-ref');
      expect(entry.drillInRefs.length).toBeGreaterThan(0);
    }
  });
}

function registerBridgeRejectionTest() {
  it('rejects provider delivery anti-tamper and missing audit reference upgrades', () => {
    const permission = entryFor('permission-loss-alert-status');

    for (const invalidEntry of [
      { ...permission, bridgeEntryId: 'invalid-provider-delivery', providerDeliveryClaimed: true },
      { ...permission, bridgeEntryId: 'invalid-broad-blocking', broadBlockingClaimed: true },
      { ...permission, bridgeEntryId: 'invalid-tamper-resistance', tamperResistanceClaimed: true },
      { ...permission, bridgeEntryId: 'invalid-mobile', mobileEnforcementClaimed: true },
      { ...permission, bridgeEntryId: 'invalid-stealth', stealthPersistenceClaimed: true },
      { ...permission, bridgeEntryId: 'invalid-privilege', privilegeEscalationClaimed: true },
      { ...permission, bridgeEntryId: 'invalid-audit-refs', auditRefs: [] },
    ]) {
      expect(() => V08IntegrityAlertStatusBridgeEntrySchema.parse(invalidEntry)).toThrow();
    }
  });
}

function entryFor(bridgeEntryId: string) {
  const entry = V08IntegrityAlertStatusBridgeReadModel.entries.find(
    (candidate) => candidate.bridgeEntryId === bridgeEntryId
  );
  if (entry === undefined) {
    throw new Error(`Missing V0.8 integrity alert status bridge entry: ${bridgeEntryId}`);
  }
  return entry;
}

function countBy(values: readonly string[]) {
  return values.reduce<Record<string, number>>((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}
