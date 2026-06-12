import { describe, expect, it } from 'vitest';

import {
  SupportBackendUploadRequiredDataClasses,
  SupportBackendUploadStatusEntrySchema,
  SupportBackendUploadStatusReadModelSchema,
} from '../../src/support-backend-upload-status';
import { SupportBackendUploadStatusReadModel } from '../../src/support-backend-upload-status-read-model';

describe('support backend upload status logging contract', () => {
  it(
    'covers queued running succeeded failed manual backend-unavailable and provider-unavailable states',
    assertCoverage
  );
  it('requires parent consent redaction audit refs support-safe payloads and custody boundaries', assertSafeBoundary);
  it('proves retry abandon manual backend and provider status behavior without execution claims', assertStatusBehavior);
  it('rejects sensitive custody claims missing refs duplicate data classes and incoherent states', assertInvalidRows);
});

function assertCoverage() {
  const readModel = SupportBackendUploadStatusReadModelSchema.parse(SupportBackendUploadStatusReadModel);

  expect(readModel.readModelId).toBe('support-backend-upload-status-proof');
  expect(readModel.entries).toHaveLength(7);
  expect(countBy(readModel.entries.map((entry) => entry.uploadStatus))).toEqual({
    'upload-queued': 1,
    'upload-running': 1,
    'upload-succeeded': 1,
    'upload-failed': 1,
    'upload-manual-required': 1,
    'backend-unavailable': 1,
    'provider-unavailable': 1,
  });
}

function assertSafeBoundary() {
  for (const entry of SupportBackendUploadStatusReadModel.entries) {
    expect(entry.parentInitiationState).toBe('parent-initiated');
    expect(entry.parentConsentState).toBe('parent-approved');
    expect(entry.executionClaimState).toBe('status-boundary-only');
    expect(entry.payloadState).toBe('redacted-status-and-audit-refs-only');
    expect(entry.custodyState).toBe('no-ocentra-hosted-family-data');
    expect(entry.disclosedDataClasses).toEqual([...SupportBackendUploadRequiredDataClasses]);
    expect(entry.consentRefs).toEqual(['parent-support-upload-consent-artifact-ref']);
    expect(entry.redactionRefs).toEqual(['support-bundle-redaction-proof-ref', 'support-safe-upload-summary-ref']);
    expect(entry.auditRefs).toEqual([
      'support-upload-status-audit-event-ref',
      'support-upload-custody-boundary-audit-ref',
    ]);
    expect(entry.containsTokens).toBe(false);
    expect(entry.containsRawChildActivity).toBe(false);
    expect(entry.containsRawUrls).toBe(false);
    expect(entry.containsScreenshots).toBe(false);
    expect(entry.containsJournals).toBe(false);
    expect(entry.containsSqliteSnapshots).toBe(false);
    expect(entry.containsPrivatePaths).toBe(false);
    expect(entry.containsCommandLines).toBe(false);
    expect(entry.containsKeystrokes).toBe(false);
    expect(entry.containsClipboardData).toBe(false);
    expect(entry.containsMessageContents).toBe(false);
    expect(entry.containsProviderSecrets).toBe(false);
    expect(entry.containsRemoteSupportTranscripts).toBe(false);
    expect(entry.realSupportBackendUploadExecuted).toBe(false);
    expect(entry.accountLookupExecuted).toBe(false);
    expect(entry.billingProviderExecuted).toBe(false);
    expect(entry.ocentraHostedFamilyDataDefault).toBe(false);
  }
}

function assertStatusBehavior() {
  const queued = entryFor('support-upload-status-queued');
  const running = entryFor('support-upload-status-running');
  const succeeded = entryFor('support-upload-status-succeeded');
  const failed = entryFor('support-upload-status-failed-abandoned');
  const manual = entryFor('support-upload-status-manual-required');
  const backendUnavailable = entryFor('support-upload-status-backend-unavailable');
  const providerUnavailable = entryFor('support-upload-status-provider-unavailable');

  for (const entry of [queued, running, succeeded]) {
    expect(entry.backendAvailabilityState).toBe('available');
    expect(entry.providerAvailabilityState).toBe('available');
    expect(entry.retryState).toBe('not-needed');
    expect(entry.abandonState).toBe('not-requested');
    expect(entry.allowedDestinations).toEqual(['support-safe-upload-status-boundary']);
  }

  expect(failed.retryState).toBe('retry-exhausted');
  expect(failed.abandonState).toBe('abandoned');
  expect(failed.retryRefs).toEqual(['support-upload-retry-policy-ref', 'support-upload-retry-exhausted-audit-ref']);
  expect(failed.abandonRefs).toEqual(['parent-abandon-decision-ref', 'support-upload-abandon-audit-ref']);
  expect(failed.failureRefs).toEqual(['support-upload-failure-status-ref']);

  expect(manual.backendAvailabilityState).toBe('manual-required');
  expect(manual.providerAvailabilityState).toBe('manual-required');
  expect(manual.retryState).toBe('manual-required');
  expect(manual.manualProofRequirements).toEqual([
    'support backend upload implementation and operator runbook before upload can be claimed',
  ]);

  expect(backendUnavailable.backendAvailabilityState).toBe('unavailable');
  expect(backendUnavailable.retryState).toBe('retry-queued');
  expect(backendUnavailable.retryRefs).toEqual(['support-upload-backend-retry-queue-ref']);
  expect(providerUnavailable.providerAvailabilityState).toBe('unavailable');
  expect(providerUnavailable.retryState).toBe('retry-queued');
  expect(providerUnavailable.retryRefs).toEqual(['support-upload-provider-retry-queue-ref']);
}

function assertInvalidRows() {
  const queued = entryFor('support-upload-status-queued');
  const failed = entryFor('support-upload-status-failed-abandoned');
  const manual = entryFor('support-upload-status-manual-required');
  const backendUnavailable = entryFor('support-upload-status-backend-unavailable');
  const providerUnavailable = entryFor('support-upload-status-provider-unavailable');

  for (const invalidEntry of [
    { ...queued, uploadId: 'invalid-token', containsTokens: true },
    { ...queued, uploadId: 'invalid-child-activity', containsRawChildActivity: true },
    { ...queued, uploadId: 'invalid-raw-url', containsRawUrls: true },
    { ...queued, uploadId: 'invalid-screenshot', containsScreenshots: true },
    { ...queued, uploadId: 'invalid-journal', containsJournals: true },
    { ...queued, uploadId: 'invalid-sqlite', containsSqliteSnapshots: true },
    { ...queued, uploadId: 'invalid-private-path', containsPrivatePaths: true },
    { ...queued, uploadId: 'invalid-command-line', containsCommandLines: true },
    { ...queued, uploadId: 'invalid-keystroke', containsKeystrokes: true },
    { ...queued, uploadId: 'invalid-clipboard', containsClipboardData: true },
    { ...queued, uploadId: 'invalid-message-content', containsMessageContents: true },
    { ...queued, uploadId: 'invalid-provider-secret', containsProviderSecrets: true },
    { ...queued, uploadId: 'invalid-remote-transcript', containsRemoteSupportTranscripts: true },
    { ...queued, uploadId: 'invalid-real-backend-execution', realSupportBackendUploadExecuted: true },
    { ...queued, uploadId: 'invalid-account-lookup', accountLookupExecuted: true },
    { ...queued, uploadId: 'invalid-billing-provider', billingProviderExecuted: true },
    { ...queued, uploadId: 'invalid-hosted-family-data', ocentraHostedFamilyDataDefault: true },
    { ...queued, uploadId: 'invalid-no-consent', parentConsentState: 'required' },
    { ...queued, uploadId: 'invalid-no-consent-ref', consentRefs: [] },
    { ...queued, uploadId: 'invalid-no-redaction-ref', redactionRefs: [] },
    { ...queued, uploadId: 'invalid-no-audit-ref', auditRefs: [] },
    { ...queued, uploadId: 'invalid-duplicate-data-class', disclosedDataClasses: ['upload-status'] },
    { ...failed, uploadId: 'invalid-failed-no-retry-ref', retryRefs: [] },
    { ...failed, uploadId: 'invalid-failed-not-abandoned', abandonState: 'not-requested' },
    { ...failed, uploadId: 'invalid-failed-no-abandon-ref', abandonRefs: [] },
    { ...manual, uploadId: 'invalid-manual-no-proof', manualProofRequirements: [] },
    { ...backendUnavailable, uploadId: 'invalid-backend-unavailable-no-retry', retryState: 'not-needed' },
    {
      ...providerUnavailable,
      uploadId: 'invalid-provider-unavailable-available',
      providerAvailabilityState: 'available',
    },
  ]) {
    expect(() => SupportBackendUploadStatusEntrySchema.parse(invalidEntry)).toThrow();
  }
}

function entryFor(uploadId: string) {
  const entry = SupportBackendUploadStatusReadModel.entries.find((candidate) => candidate.uploadId === uploadId);
  if (entry === undefined) {
    throw new Error(`Missing support backend upload status entry: ${uploadId}`);
  }
  return entry;
}

function countBy(values: readonly string[]) {
  return values.reduce<Record<string, number>>((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}
