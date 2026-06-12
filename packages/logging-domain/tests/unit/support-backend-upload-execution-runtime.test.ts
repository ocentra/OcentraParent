import { describe, expect, it } from 'vitest';

import {
  SupportBackendUploadExecutionRuntimeEntrySchema,
  SupportBackendUploadExecutionRuntimeReadModelSchema,
  SupportBackendUploadExecutionRuntimeRequiredDataClasses,
} from '../../src/support-backend-upload-execution-runtime';
import { SupportBackendUploadExecutionRuntimeReadModel } from '../../src/support-backend-upload-execution-runtime-read-model';

describe('support backend upload execution runtime logging contract', () => {
  it('covers request preflight manual unavailable retry and abandon runtime states', assertCoverage);
  it('requires parent consent redaction audit status refs and support-safe runtime payloads', assertSafeBoundary);
  it('keeps backend upload execution manual or unavailable without production claims', assertRuntimeBoundary);
  it('rejects sensitive custody execution account billing remote support and SLA claims', assertInvalidRows);
});

function assertCoverage() {
  const readModel = SupportBackendUploadExecutionRuntimeReadModelSchema.parse(
    SupportBackendUploadExecutionRuntimeReadModel
  );

  expect(readModel.readModelId).toBe('support-backend-upload-execution-runtime-proof');
  expect(readModel.entries).toHaveLength(7);
  expect(countBy(readModel.entries.map((entry) => entry.runtimeState))).toEqual({
    'execution-request-recorded': 1,
    'redaction-preflight-ready': 1,
    'dispatch-manual-required': 1,
    'backend-unavailable': 1,
    'provider-unavailable': 1,
    'retry-scheduled': 1,
    'operator-abandoned': 1,
  });
}

function assertSafeBoundary() {
  for (const entry of SupportBackendUploadExecutionRuntimeReadModel.entries) {
    expect(entry.parentInitiationState).toBe('parent-initiated');
    expect(entry.parentConsentState).toBe('parent-approved');
    expect(entry.executionClaimState).toBe('runtime-boundary-only');
    expect(entry.payloadState).toBe('redacted-runtime-refs-only');
    expect(entry.custodyState).toBe('no-ocentra-hosted-family-data');
    expect(entry.disclosedDataClasses).toEqual([...SupportBackendUploadExecutionRuntimeRequiredDataClasses]);
    expect(entry.consentRefs).toEqual(['parent-support-upload-consent-artifact-ref']);
    expect(entry.redactionRefs).toEqual([
      'support-bundle-redaction-proof-ref',
      'support-upload-redaction-preflight-ref',
    ]);
    expect(entry.auditRefs).toEqual([
      'support-upload-runtime-audit-event-ref',
      'support-upload-custody-boundary-audit-ref',
    ]);
    expect(entry.statusRefs).toEqual(['production-support-backend-upload-status-proof-ref']);
    expect(entry.runtimeRefs).toEqual(['support-upload-runtime-boundary-ref']);
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
    expect(entry.billingProviderContactExecuted).toBe(false);
    expect(entry.remoteSupportSessionExecuted).toBe(false);
    expect(entry.productionSlaClaimed).toBe(false);
    expect(entry.ocentraHostedFamilyDataDefault).toBe(false);
  }
}

function assertRuntimeBoundary() {
  const request = entryFor('support-upload-execution-request-recorded');
  const preflight = entryFor('support-upload-redaction-preflight-ready');
  const manual = entryFor('support-upload-dispatch-manual-required');
  const backendUnavailable = entryFor('support-upload-execution-backend-unavailable');
  const providerUnavailable = entryFor('support-upload-execution-provider-unavailable');
  const retry = entryFor('support-upload-execution-retry-scheduled');
  const abandoned = entryFor('support-upload-execution-operator-abandoned');

  for (const entry of [request, preflight]) {
    expect(entry.backendAvailabilityState).toBe('available');
    expect(entry.providerAvailabilityState).toBe('available');
    expect(entry.retryState).toBe('not-needed');
    expect(entry.abandonState).toBe('not-requested');
    expect(entry.manualProofRequirements).toEqual([]);
  }

  expect(manual.backendAvailabilityState).toBe('manual-required');
  expect(manual.providerAvailabilityState).toBe('manual-required');
  expect(manual.retryState).toBe('manual-required');
  expect(manual.manualProofRequirements).toEqual([
    'support backend upload adapter implementation before execution can be claimed',
    'operator runbook and retention/delete proof before production upload can be claimed',
  ]);

  expect(backendUnavailable.retryRefs).toEqual(['support-upload-runtime-backend-retry-schedule-ref']);
  expect(providerUnavailable.retryRefs).toEqual(['support-upload-runtime-provider-retry-schedule-ref']);
  expect(retry.retryState).toBe('retry-scheduled');
  expect(retry.abandonState).toBe('not-requested');
  expect(abandoned.retryState).toBe('retry-exhausted');
  expect(abandoned.abandonState).toBe('abandoned');
  expect(abandoned.abandonRefs).toEqual(['support-upload-runtime-operator-abandon-ref', 'parent-abandon-decision-ref']);
}

function assertInvalidRows() {
  const request = entryFor('support-upload-execution-request-recorded');
  const manual = entryFor('support-upload-dispatch-manual-required');
  const retry = entryFor('support-upload-execution-retry-scheduled');
  const abandoned = entryFor('support-upload-execution-operator-abandoned');

  for (const invalidEntry of [
    { ...request, runtimeId: 'invalid-token', containsTokens: true },
    { ...request, runtimeId: 'invalid-child-activity', containsRawChildActivity: true },
    { ...request, runtimeId: 'invalid-raw-url', containsRawUrls: true },
    { ...request, runtimeId: 'invalid-screenshot', containsScreenshots: true },
    { ...request, runtimeId: 'invalid-journal', containsJournals: true },
    { ...request, runtimeId: 'invalid-sqlite', containsSqliteSnapshots: true },
    { ...request, runtimeId: 'invalid-private-path', containsPrivatePaths: true },
    { ...request, runtimeId: 'invalid-command-line', containsCommandLines: true },
    { ...request, runtimeId: 'invalid-keystroke', containsKeystrokes: true },
    { ...request, runtimeId: 'invalid-clipboard', containsClipboardData: true },
    { ...request, runtimeId: 'invalid-message-content', containsMessageContents: true },
    { ...request, runtimeId: 'invalid-provider-secret', containsProviderSecrets: true },
    { ...request, runtimeId: 'invalid-remote-transcript', containsRemoteSupportTranscripts: true },
    { ...request, runtimeId: 'invalid-real-backend-execution', realSupportBackendUploadExecuted: true },
    { ...request, runtimeId: 'invalid-account-lookup', accountLookupExecuted: true },
    { ...request, runtimeId: 'invalid-billing-contact', billingProviderContactExecuted: true },
    { ...request, runtimeId: 'invalid-remote-support-session', remoteSupportSessionExecuted: true },
    { ...request, runtimeId: 'invalid-production-sla', productionSlaClaimed: true },
    { ...request, runtimeId: 'invalid-hosted-family-data', ocentraHostedFamilyDataDefault: true },
    { ...request, runtimeId: 'invalid-no-consent', parentConsentState: 'required' },
    { ...request, runtimeId: 'invalid-no-status-ref', statusRefs: [] },
    { ...request, runtimeId: 'invalid-no-runtime-ref', runtimeRefs: [] },
    { ...request, runtimeId: 'invalid-duplicate-data-class', disclosedDataClasses: ['runtime-request-status'] },
    { ...manual, runtimeId: 'invalid-manual-no-proof', manualProofRequirements: [] },
    { ...retry, runtimeId: 'invalid-retry-not-scheduled', retryState: 'not-needed' },
    { ...abandoned, runtimeId: 'invalid-abandoned-no-ref', abandonRefs: [] },
  ]) {
    expect(() => SupportBackendUploadExecutionRuntimeEntrySchema.parse(invalidEntry)).toThrow();
  }
}

function entryFor(runtimeId: string) {
  const entry = SupportBackendUploadExecutionRuntimeReadModel.entries.find(
    (candidate) => candidate.runtimeId === runtimeId
  );
  if (entry === undefined) {
    throw new Error(`Missing support backend upload execution runtime entry: ${runtimeId}`);
  }
  return entry;
}

function countBy(values: readonly string[]) {
  return values.reduce<Record<string, number>>((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}
