import { describe, expect, it } from 'vitest';

import {
  StatusBackendRedactionManifestEntrySchema,
  StatusBackendRedactionManifestReadModelSchema,
  StatusBackendRedactionManifestRequiredDataClasses,
} from '@ocentra-parent/schema-domain/status-backend-redaction-manifest';
import { StatusBackendRedactionManifestReadModel } from '@ocentra-parent/schema-domain/status-backend-redaction-manifest-read-model';

describe('status backend redaction manifest logging contract', () => {
  it('covers manifest ready manual review and backend unavailable states', assertCoverage);
  it('requires parent consent target queue audit and redaction manifest refs', assertSafeBoundary);
  it('keeps redaction review and manifest execution manual', assertManifestBoundary);
  it('rejects sensitive payloads and execution claims', assertInvalidRows);
});

function assertCoverage() {
  const readModel = StatusBackendRedactionManifestReadModelSchema.parse(StatusBackendRedactionManifestReadModel);

  expect(readModel.readModelId).toBe('production-support-status-backend-redaction-manifest-proof');
  expect(readModel.entries).toHaveLength(6);
  expect(countBy(readModel.entries.map((entry) => entry.manifestState))).toEqual({
    'redaction-manifest-ready': 1,
    'redaction-manifest-manual-required': 1,
    'redaction-review-queued': 1,
    'redaction-review-running': 1,
    'redaction-review-failed': 1,
    'backend-unavailable': 1,
  });
}

function assertSafeBoundary() {
  for (const entry of StatusBackendRedactionManifestReadModel.entries) {
    expect(entry.parentConsentState).toBe('parent-approved');
    expect(entry.executionClaimState).toBe('status-backend-redaction-manifest-boundary-only');
    expect(entry.payloadState).toBe('redacted-status-refs-only');
    expect(entry.disclosedDataClasses).toEqual([...StatusBackendRedactionManifestRequiredDataClasses]);
    expect(entry.consentRefs).toEqual(['parent-status-backend-redaction-manifest-consent-ref']);
    expect(entry.targetRefs).toEqual(['production-support-status-backend-runtime-execution-proof-ref']);
    expect(entry.queueRefs).toEqual(['production-support-status-backend-execution-queue-proof-ref']);
    expect(entry.auditRefs).toEqual(['production-support-status-backend-queue-audit-persistence-proof-ref']);
    expect(entry.redactionManifestRefs).toEqual([
      'support-bundle-redaction-proof-ref',
      'status-backend-redaction-manifest-ref',
    ]);
    expect(entry.containsTokens).toBe(false);
    expect(entry.containsRawChildActivity).toBe(false);
    expect(entry.containsRawSupportBundles).toBe(false);
    expect(entry.containsProviderSecrets).toBe(false);
    expect(entry.containsAccountLookupResults).toBe(false);
    expect(entry.containsBillingContactRecords).toBe(false);
    expect(entry.containsBackendUploadPayloads).toBe(false);
    expect(entry.containsStatusBackendPayloads).toBe(false);
    expect(entry.containsPublicRuntimePayloads).toBe(false);
    expect(entry.containsRemoteSupportTranscripts).toBe(false);
    expect(entry.realStatusBackendExecution).toBe(false);
    expect(entry.statusBackendPayloadCustodyClaimed).toBe(false);
    expect(entry.durableStatusBackendPayloadStorage).toBe(false);
    expect(entry.statusBackendPayloadDeletionExecuted).toBe(false);
    expect(entry.retryWorkerExecution).toBe(false);
    expect(entry.auditPersistenceExecuted).toBe(false);
    expect(entry.publicRuntimeExecution).toBe(false);
    expect(entry.supportBackendUploadExecution).toBe(false);
    expect(entry.providerExecution).toBe(false);
    expect(entry.accountLookupExecuted).toBe(false);
    expect(entry.billingProviderContactExecuted).toBe(false);
    expect(entry.remoteSupportSessionExecuted).toBe(false);
    expect(entry.productionSlaClaimed).toBe(false);
    expect(entry.ocentraHostedFamilyDataDefault).toBe(false);
  }
}

function assertManifestBoundary() {
  const ready = entryFor('status-backend-redaction-manifest-ready');
  const manual = entryFor('status-backend-redaction-manifest-manual-required');
  const queued = entryFor('status-backend-redaction-review-queued');
  const running = entryFor('status-backend-redaction-review-running');
  const failed = entryFor('status-backend-redaction-review-failed');
  const unavailable = entryFor('status-backend-redaction-backend-unavailable');

  expect(ready.redactionManifestState).toBe('support-safe-manifest-ready');
  expect(ready.redactionReviewState).toBe('reviewed');
  expect(ready.redactionSummaryRefs).toEqual(['status-backend-redaction-summary-reviewed-ref']);
  expect(manual.redactionManifestState).toBe('manual-required');
  expect(manual.redactionReviewState).toBe('manual-required');
  expect(queued.redactionReviewState).toBe('queued');
  expect(running.redactionReviewState).toBe('running');
  expect(failed.redactionReviewState).toBe('failed');
  expect(failed.failureRefs).toEqual(['status-backend-redaction-review-failure-ref']);
  expect(unavailable.redactionManifestState).toBe('manual-required');
  expect(unavailable.failureRefs).toEqual(['status-backend-redaction-backend-unavailable-ref']);
}

function assertInvalidRows() {
  const ready = entryFor('status-backend-redaction-manifest-ready');
  const manual = entryFor('status-backend-redaction-manifest-manual-required');
  const queued = entryFor('status-backend-redaction-review-queued');
  const unavailable = entryFor('status-backend-redaction-backend-unavailable');

  for (const invalidEntry of [
    { ...ready, manifestId: 'invalid-token', containsTokens: true },
    { ...ready, manifestId: 'invalid-child-activity', containsRawChildActivity: true },
    { ...ready, manifestId: 'invalid-raw-support-bundle', containsRawSupportBundles: true },
    { ...ready, manifestId: 'invalid-provider-secret', containsProviderSecrets: true },
    { ...ready, manifestId: 'invalid-account-lookup-result', containsAccountLookupResults: true },
    { ...ready, manifestId: 'invalid-billing-contact-record', containsBillingContactRecords: true },
    { ...ready, manifestId: 'invalid-backend-upload-payload', containsBackendUploadPayloads: true },
    { ...ready, manifestId: 'invalid-status-backend-payload', containsStatusBackendPayloads: true },
    { ...ready, manifestId: 'invalid-public-runtime-payload', containsPublicRuntimePayloads: true },
    { ...ready, manifestId: 'invalid-remote-transcript', containsRemoteSupportTranscripts: true },
    { ...ready, manifestId: 'invalid-status-backend-execution', realStatusBackendExecution: true },
    { ...ready, manifestId: 'invalid-payload-custody', statusBackendPayloadCustodyClaimed: true },
    { ...ready, manifestId: 'invalid-durable-storage', durableStatusBackendPayloadStorage: true },
    { ...ready, manifestId: 'invalid-payload-deletion', statusBackendPayloadDeletionExecuted: true },
    { ...ready, manifestId: 'invalid-retry-worker', retryWorkerExecution: true },
    { ...ready, manifestId: 'invalid-audit-persistence', auditPersistenceExecuted: true },
    { ...ready, manifestId: 'invalid-public-runtime-execution', publicRuntimeExecution: true },
    { ...ready, manifestId: 'invalid-support-upload-execution', supportBackendUploadExecution: true },
    { ...ready, manifestId: 'invalid-provider-execution', providerExecution: true },
    { ...ready, manifestId: 'invalid-account-lookup', accountLookupExecuted: true },
    { ...ready, manifestId: 'invalid-billing-contact', billingProviderContactExecuted: true },
    { ...ready, manifestId: 'invalid-remote-support', remoteSupportSessionExecuted: true },
    { ...ready, manifestId: 'invalid-production-sla', productionSlaClaimed: true },
    { ...ready, manifestId: 'invalid-hosted-family-data', ocentraHostedFamilyDataDefault: true },
    { ...ready, manifestId: 'invalid-no-consent', parentConsentState: 'required' },
    { ...ready, manifestId: 'invalid-no-target-ref', targetRefs: [] },
    { ...ready, manifestId: 'invalid-duplicate-data-class', disclosedDataClasses: ['failure-ref'] },
    { ...ready, manifestId: 'invalid-ready-unreviewed', redactionReviewState: 'running' },
    { ...manual, manifestId: 'invalid-manual-ready', redactionManifestState: 'support-safe-manifest-ready' },
    { ...queued, manifestId: 'invalid-queued-reviewed', redactionReviewState: 'reviewed' },
    { ...queued, manifestId: 'invalid-queued-no-failure-ref', failureRefs: [] },
    { ...unavailable, manifestId: 'invalid-unavailable-reviewed', redactionReviewState: 'reviewed' },
  ]) {
    expect(() => StatusBackendRedactionManifestEntrySchema.parse(invalidEntry)).toThrow();
  }
}

function entryFor(manifestId: string) {
  const entry = StatusBackendRedactionManifestReadModel.entries.find(
    (candidate) => candidate.manifestId === manifestId
  );
  if (entry === undefined) {
    throw new Error(`Missing status backend redaction manifest entry: ${manifestId}`);
  }
  return entry;
}

function countBy(values: readonly string[]) {
  return values.reduce<Record<string, number>>((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}
