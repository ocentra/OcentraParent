import { describe, expect, it } from 'vitest';
import {
  NotificationAuditHistoryEntrySchema,
  NotificationAuditHistoryReadModel,
  NotificationAuditHistoryReadModelSchema,
} from '../src/notification-audit-history';

describe('notification audit history logging contract', () => {
  registerProviderStatusCoverageTest();
  registerRetryLifecycleCoverageTest();
  registerRedactionAndCustodyTest();
  registerReceiptManualQuietHoursEscalationRefsTest();
  registerRejectionTest();
});

function registerProviderStatusCoverageTest() {
  it('covers provider status logs without claiming provider delivery runtime', () => {
    const readModel = NotificationAuditHistoryReadModelSchema.parse(NotificationAuditHistoryReadModel);

    expect(readModel.readModelId).toBe('notification-audit-history-contract-proof');
    expect(readModel.entries).toHaveLength(6);
    expect(countBy(readModel.entries.map((entry) => entry.providerStatus))).toEqual({
      queued: 1,
      delivered: 1,
      failed: 2,
      unavailable: 1,
      'manual-required': 1,
    });
    expect(entryFor('notification-audit-provider-queued').providerStatus).toBe('queued');
    expect(entryFor('notification-audit-delivered-receipt-required').providerStatus).toBe('delivered');
    expect(entryFor('notification-audit-provider-unavailable').providerStatus).toBe('unavailable');
  });
}

function registerRetryLifecycleCoverageTest() {
  it('covers retry lifecycle states as log history contract rows', () => {
    const readModel = NotificationAuditHistoryReadModelSchema.parse(NotificationAuditHistoryReadModel);

    expect(countBy(readModel.entries.map((entry) => entry.retryLifecycleState))).toEqual({
      'not-scheduled': 1,
      'receipt-required-contract': 1,
      'retry-scheduled-contract': 1,
      'manual-review-required': 1,
      'provider-unavailable': 1,
      'quiet-hours-deferred-contract': 1,
    });
    expect(entryFor('notification-audit-failed-retry-scheduled').retryRefs).toEqual([
      'retry-policy-exponential-backoff-ref',
    ]);
    expect(entryFor('notification-audit-manual-quiet-hours-deferred').retryRefs).toEqual([
      'quiet-hours-deferred-retry-ref',
    ]);
  });
}

function registerRedactionAndCustodyTest() {
  it('keeps payload fields redaction-safe and child data out of Ocentra custody', () => {
    for (const entry of NotificationAuditHistoryReadModel.entries) {
      expect(entry.payloadRedactionState).toBe('minimal-operational-fields-only');
      expect(entry.childDataCustodyState).toBe('no-ocentra-hosted-child-data');
      expect(entry.redactionSafePayloadFields).toEqual([
        'alert-id-ref',
        'family-scope-ref',
        'device-scope-ref',
        'severity',
        'reason-code',
        'provider-channel',
        'provider-status',
        'retry-lifecycle-state',
        'parent-action-link-ref',
        'audit-entry-ref',
      ]);
      expect(entry.rawChildDataIncluded).toBe(false);
      expect(entry.rawEvidencePayloadIncluded).toBe(false);
      expect(entry.sensitiveProviderPayloadIncluded).toBe(false);
      expect(entry.ocentraHostedChildDataStored).toBe(false);
      expect(entry.providerStoresChildEvidenceClaimed).toBe(false);
      expect(entry.evidenceRefs).toEqual(['authenticated-evidence-drill-in-ref']);
    }
  });
}

function registerReceiptManualQuietHoursEscalationRefsTest() {
  it('requires receipt manual-required quiet-hours and escalation refs for the matching history rows', () => {
    expect(entryFor('notification-audit-delivered-receipt-required').receiptRefs).toEqual([
      'provider-receipt-required-ref',
    ]);
    expect(entryFor('notification-audit-delivered-receipt-required').manualProofRequirements).toEqual([
      'provider receipt artifact before delivered history can be claimed',
    ]);
    expect(entryFor('notification-audit-failed-manual-review').manualRequiredRefs).toEqual([
      'manual-review-required-ref',
    ]);
    expect(entryFor('notification-audit-manual-quiet-hours-deferred').quietHoursRefs).toEqual([
      'quiet-hours-defer-noncritical-ref',
    ]);
    expect(entryFor('notification-audit-manual-quiet-hours-deferred').escalationRefs).toEqual([
      'escalation-waiting-window-ref',
    ]);
  });
}

function registerRejectionTest() {
  it('rejects provider runtime claims sensitive payloads custody claims and incoherent refs', () => {
    const queued = entryFor('notification-audit-provider-queued');
    const delivered = entryFor('notification-audit-delivered-receipt-required');
    const retryScheduled = entryFor('notification-audit-failed-retry-scheduled');
    const manualRequired = entryFor('notification-audit-manual-quiet-hours-deferred');

    for (const invalidEntry of [
      { ...queued, auditEntryId: 'invalid-provider-adapter', providerAdapterImplemented: true },
      { ...queued, auditEntryId: 'invalid-send-executed', sendAttemptExecuted: true },
      { ...queued, auditEntryId: 'invalid-retry-observed', retryExecutionObserved: true },
      { ...queued, auditEntryId: 'invalid-webhook-receipt', webhookReceiptIngested: true },
      { ...queued, auditEntryId: 'invalid-credential-present', providerCredentialPresent: true },
      { ...queued, auditEntryId: 'invalid-history-ui', notificationHistoryUiClaimed: true },
      { ...queued, auditEntryId: 'invalid-raw-child-data', rawChildDataIncluded: true },
      { ...queued, auditEntryId: 'invalid-sensitive-payload', sensitiveProviderPayloadIncluded: true },
      { ...queued, auditEntryId: 'invalid-ocentra-custody', ocentraHostedChildDataStored: true },
      { ...delivered, auditEntryId: 'invalid-delivered-no-receipt', receiptRefs: [] },
      { ...retryScheduled, auditEntryId: 'invalid-retry-no-ref', retryRefs: [] },
      { ...manualRequired, auditEntryId: 'invalid-manual-no-ref', manualRequiredRefs: [] },
      {
        ...queued,
        auditEntryId: 'invalid-duplicate-payload-field',
        redactionSafePayloadFields: ['alert-id-ref', 'alert-id-ref'],
      },
    ]) {
      expect(() => NotificationAuditHistoryEntrySchema.parse(invalidEntry)).toThrow();
    }
  });
}

function entryFor(auditEntryId: string) {
  const entry = NotificationAuditHistoryReadModel.entries.find((candidate) => candidate.auditEntryId === auditEntryId);
  if (entry === undefined) {
    throw new Error(`Missing notification audit history entry: ${auditEntryId}`);
  }
  return entry;
}

function countBy(values: readonly string[]) {
  return values.reduce<Record<string, number>>((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}
