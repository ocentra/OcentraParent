import { describe, expect, it } from 'vitest';
import {
  V3NotificationRuleProviderRetryContractEntrySchema,
  V3NotificationRuleProviderRetryContractReadModel,
  V3NotificationRuleProviderRetryContractReadModelSchema,
} from '@ocentra-parent/schema-domain/notification-v3-provider-retry';

describe('V3 notification rule provider retry contract', () => {
  registerContractCoverageTest();
  registerContractBoundaryTest();
  registerContractRejectionTest();
});

function registerContractCoverageTest() {
  it('covers notification rule reason provider channel delivery retry and preference states', () => {
    const readModel = V3NotificationRuleProviderRetryContractReadModelSchema.parse(
      V3NotificationRuleProviderRetryContractReadModel
    );

    expect(readModel.readModelId).toBe('v3-notification-rule-provider-retry-contract');
    expect(readModel.entries).toHaveLength(6);
    expect(countBy(readModel.entries.map((entry) => entry.reasonCode))).toEqual({
      'policy-violation': 1,
      'parent-request': 1,
      'suspicious-unknown': 1,
      'device-offline': 1,
      'sync-failure': 1,
      'provider-failure': 1,
    });
    expect(countBy(readModel.entries.map((entry) => entry.providerChannel))).toEqual({
      push: 1,
      email: 1,
      sms: 1,
      whatsapp: 1,
      'in-app': 2,
    });
    expect(countBy(readModel.entries.map((entry) => entry.deliveryResultState))).toEqual({
      queued: 1,
      'receipt-required': 1,
      'retryable-failure': 1,
      'permanent-failure': 1,
      'manual-required': 1,
      'not-sent': 1,
    });
    expect(countBy(readModel.entries.map((entry) => entry.retryPolicyState))).toEqual({
      'no-retry': 2,
      'exponential-backoff': 1,
      'manual-review': 1,
      'quiet-hours-deferred': 1,
      'provider-disabled': 1,
    });
  });
}

function registerContractBoundaryTest() {
  it('keeps provider retry contract refs auditable without delivery execution or raw evidence payload claims', () => {
    for (const entry of V3NotificationRuleProviderRetryContractReadModel.entries) {
      expect(entry.notificationRuleRef.length).toBeGreaterThan(0);
      expect(entry.notificationIntentRef.length).toBeGreaterThan(0);
      expect(entry.deliveryAttemptRef.length).toBeGreaterThan(0);
      expect(entry.deliveryResultRef.length).toBeGreaterThan(0);
      expect(entry.retryPolicyRef.length).toBeGreaterThan(0);
      expect(entry.quietHoursPolicyRef.length).toBeGreaterThan(0);
      expect(entry.escalationPolicyRef.length).toBeGreaterThan(0);
      expect(entry.parentPreferenceRef.length).toBeGreaterThan(0);
      expect(entry.auditRefs.length).toBeGreaterThan(0);
      expect(entry.evidenceRefs.length).toBeGreaterThan(0);
      expect(entry.providerAdapterImplemented).toBe(false);
      expect(entry.deliveryAttemptExecuted).toBe(false);
      expect(entry.providerReceiptObserved).toBe(false);
      expect(entry.rawEvidenceInProviderPayload).toBe(false);
      expect(entry.providerStoresChildEvidenceClaimed).toBe(false);
    }

    const retryable = entryFor('notification-rule-suspicious-unknown-email-retryable-failure');
    expect(retryable.deliveryResultState).toBe('retryable-failure');
    expect(retryable.retryPolicyState).toBe('exponential-backoff');
    const quietHours = entryFor('notification-rule-sync-failure-whatsapp-quiet-hours-deferred');
    expect(quietHours.deliveryAttemptState).toBe('suppressed-quiet-hours');
    expect(quietHours.parentPreferenceState).toBe('quiet-hours-active');
  });
}

function registerContractRejectionTest() {
  it('rejects provider runtime claims and incoherent retry or quiet-hours states', () => {
    const queued = entryFor('notification-rule-policy-violation-push-queued');
    const retryable = entryFor('notification-rule-suspicious-unknown-email-retryable-failure');
    const quietHours = entryFor('notification-rule-sync-failure-whatsapp-quiet-hours-deferred');
    const receiptRequired = entryFor('notification-rule-parent-request-in-app-receipt-required');

    for (const invalidEntry of [
      { ...queued, contractEntryId: 'invalid-provider-adapter-implemented', providerAdapterImplemented: true },
      { ...queued, contractEntryId: 'invalid-delivery-attempt-executed', deliveryAttemptExecuted: true },
      { ...queued, contractEntryId: 'invalid-provider-receipt-observed', providerReceiptObserved: true },
      { ...queued, contractEntryId: 'invalid-raw-evidence-payload', rawEvidenceInProviderPayload: true },
      { ...queued, contractEntryId: 'invalid-provider-child-storage', providerStoresChildEvidenceClaimed: true },
      { ...retryable, contractEntryId: 'invalid-retryable-with-no-retry', retryPolicyState: 'no-retry' },
      { ...quietHours, contractEntryId: 'invalid-quiet-hours-without-suppression', deliveryAttemptState: 'queued' },
      { ...receiptRequired, contractEntryId: 'invalid-receipt-required-without-ref', providerReceiptRefs: [] },
    ]) {
      expect(() => V3NotificationRuleProviderRetryContractEntrySchema.parse(invalidEntry)).toThrow();
    }
  });
}

function entryFor(contractEntryId: string) {
  const entry = V3NotificationRuleProviderRetryContractReadModel.entries.find(
    (candidate) => candidate.contractEntryId === contractEntryId
  );
  if (entry === undefined) {
    throw new Error(`Missing V3 notification rule provider retry contract entry: ${contractEntryId}`);
  }
  return entry;
}

function countBy(values: readonly string[]) {
  return values.reduce<Record<string, number>>((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}
