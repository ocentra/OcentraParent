import { describe, expect, it } from 'vitest';
import {
  V08NotificationProviderStatusBoundaryEntrySchema,
  V08NotificationProviderStatusBoundaryReadModel,
  V08NotificationProviderStatusBoundaryReadModelSchema,
} from '../src/v0-8-notification-provider-status-boundary';

describe('V0.8 notification provider status boundary', () => {
  registerProviderStatusCoverageTest();
  registerReadinessCoverageTest();
  registerNoDeliveryClaimTest();
  registerRejectionTest();
});

function registerProviderStatusCoverageTest() {
  it('covers queued delivered failed unavailable and manual-required as provider status contract states', () => {
    const readModel = V08NotificationProviderStatusBoundaryReadModelSchema.parse(
      V08NotificationProviderStatusBoundaryReadModel
    );

    expect(readModel.readModelId).toBe('v0-8-notification-provider-status-boundary');
    expect(readModel.entries).toHaveLength(5);
    expect(countBy(readModel.entries.map((entry) => entry.providerStatus))).toEqual({
      queued: 1,
      delivered: 1,
      failed: 1,
      unavailable: 1,
      'manual-required': 1,
    });
    expect(entryFor('notification-provider-delivered-receipt-required').statusProofState).toBe(
      'delivery-receipt-required'
    );
    expect(entryFor('notification-provider-delivered-receipt-required').deliveryClaimState).toBe('receipt-required');
  });
}

function registerReadinessCoverageTest() {
  it('represents quiet-hours and escalation readiness without provider delivery implementation', () => {
    const readModel = V08NotificationProviderStatusBoundaryReadModelSchema.parse(
      V08NotificationProviderStatusBoundaryReadModel
    );

    expect(countBy(readModel.entries.map((entry) => entry.quietHoursReadiness))).toEqual({
      ready: 2,
      'defer-noncritical': 1,
      unavailable: 1,
      'manual-required': 1,
    });
    expect(countBy(readModel.entries.map((entry) => entry.escalationReadiness))).toEqual({
      ready: 1,
      'waiting-window': 1,
      'manual-required': 2,
      unavailable: 1,
    });
    for (const entry of readModel.entries) {
      expect(entry.readinessRefs.length).toBeGreaterThan(0);
      expect(entry.preferenceRefs).toContain('notification-parent-preferences-ref');
    }
  });
}

function registerNoDeliveryClaimTest() {
  it('keeps provider delivery child-evidence storage and sensitive payload claims false', () => {
    for (const entry of V08NotificationProviderStatusBoundaryReadModel.entries) {
      expect(entry.providerDeliveryImplemented).toBe(false);
      expect(entry.providerDeliveryObserved).toBe(false);
      expect(entry.deliveredNotificationClaimed).toBe(false);
      expect(entry.sensitiveProviderPayloadClaimed).toBe(false);
      expect(entry.providerStoresChildEvidenceClaimed).toBe(false);
      expect(entry.auditRefs).toContain('notification-provider-status-audit-ref');
      expect(entry.minimalPayloadBoundary.length).toBeGreaterThan(0);
    }
  });
}

function registerRejectionTest() {
  it('rejects delivery implementation observed delivery sensitive payload storage and invalid delivered rows', () => {
    const queued = entryFor('notification-provider-queued-contract');
    const delivered = entryFor('notification-provider-delivered-receipt-required');

    for (const invalidEntry of [
      { ...queued, statusEntryId: 'invalid-provider-implemented', providerDeliveryImplemented: true },
      { ...queued, statusEntryId: 'invalid-provider-observed', providerDeliveryObserved: true },
      { ...queued, statusEntryId: 'invalid-delivery-claimed', deliveredNotificationClaimed: true },
      { ...queued, statusEntryId: 'invalid-sensitive-payload', sensitiveProviderPayloadClaimed: true },
      { ...queued, statusEntryId: 'invalid-child-evidence-storage', providerStoresChildEvidenceClaimed: true },
      { ...delivered, statusEntryId: 'invalid-delivered-without-receipt', providerReceiptRefs: [] },
      { ...delivered, statusEntryId: 'invalid-delivered-observed', providerDeliveryObserved: true },
    ]) {
      expect(() => V08NotificationProviderStatusBoundaryEntrySchema.parse(invalidEntry)).toThrow();
    }
  });
}

function entryFor(statusEntryId: string) {
  const entry = V08NotificationProviderStatusBoundaryReadModel.entries.find(
    (candidate) => candidate.statusEntryId === statusEntryId
  );
  if (entry === undefined) {
    throw new Error(`Missing V0.8 notification provider status entry: ${statusEntryId}`);
  }
  return entry;
}

function countBy(values: readonly string[]) {
  return values.reduce<Record<string, number>>((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}
