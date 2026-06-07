import { describe, expect, it } from 'vitest';

import {
  StatusBackendDeleteExecutorEntrySchema,
  StatusBackendDeleteExecutorReadModelSchema,
  StatusBackendDeleteExecutorRequiredDataClasses,
} from '../src/status-backend-delete-executor';
import { StatusBackendDeleteExecutorReadModel } from '../src/status-backend-delete-executor-read-model';

describe('status backend delete executor logging contract', () => {
  it('covers delete request authorization queue running manual failure audit and unavailable states', assertCoverage);
  it('requires parent consent redaction queue audit target custody delete and executor refs', assertSafeBoundary);
  it('keeps delete executor and payload deletion execution manual or not executed', assertDeleteExecutorBoundary);
  it('rejects sensitive payloads and execution claims', assertInvalidRows);
});

function assertCoverage() {
  const readModel = StatusBackendDeleteExecutorReadModelSchema.parse(StatusBackendDeleteExecutorReadModel);

  expect(readModel.readModelId).toBe('production-support-status-backend-delete-executor-proof');
  expect(readModel.entries).toHaveLength(8);
  expect(countBy(readModel.entries.map((entry) => entry.deleteExecutorState))).toEqual({
    'delete-request-recorded': 1,
    'delete-executor-authorized': 1,
    'delete-executor-queued': 1,
    'delete-executor-running': 1,
    'deletion-manual-required': 1,
    'delete-executor-failed': 1,
    'audit-export-ready': 1,
    'backend-unavailable': 1,
  });
}

function assertSafeBoundary() {
  for (const entry of StatusBackendDeleteExecutorReadModel.entries) {
    expect(entry.parentConsentState).toBe('parent-approved');
    expect(entry.executionClaimState).toBe('status-backend-delete-executor-boundary-only');
    expect(entry.payloadState).toBe('redacted-delete-status-refs-only');
    expect(entry.disclosedDataClasses).toEqual([...StatusBackendDeleteExecutorRequiredDataClasses]);
    expect(entry.consentRefs).toEqual(['parent-status-backend-delete-consent-ref']);
    expect(entry.targetRefs).toEqual(['production-support-status-backend-public-runtime-followthrough-proof-ref']);
    expect(entry.queueRefs).toEqual(['production-support-status-backend-execution-queue-proof-ref']);
    expect(entry.auditRefs).toEqual(['production-support-status-backend-queue-audit-persistence-proof-ref']);
    expect(entry.redactionRefs).toEqual([
      'support-bundle-redaction-proof-ref',
      'status-backend-delete-executor-redaction-summary-ref',
    ]);
    expect(entry.custodyRefs).toEqual(['data-custody-status-backend-payload-boundary-ref']);
    expect(entry.deleteRefs.length).toBeGreaterThan(0);
    expect(entry.executorRefs.length).toBeGreaterThan(0);
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
    expect(entry.durableStatusBackendPayloadStorage).toBe(false);
    expect(entry.statusBackendDeleteExecutorExecuted).toBe(false);
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

function assertDeleteExecutorBoundary() {
  const request = entryFor('status-backend-delete-request-recorded');
  const authorized = entryFor('status-backend-delete-executor-authorized');
  const queued = entryFor('status-backend-delete-executor-queued');
  const running = entryFor('status-backend-delete-executor-running');
  const manual = entryFor('status-backend-deletion-manual-required');
  const failed = entryFor('status-backend-delete-executor-failed');
  const audit = entryFor('status-backend-delete-executor-audit-export-ready');
  const unavailable = entryFor('status-backend-delete-executor-backend-unavailable');

  expect(request.deleteRefs).toEqual(['parent-status-backend-delete-request-ref']);
  expect(authorized.manualProofRequirements).toEqual([
    'parent authorization proof before delete executor dispatch can be claimed',
  ]);
  expect(queued.executorExecutionState).toBe('manual-required');
  expect(queued.executorRefs).toEqual(['status-backend-delete-executor-queue-ref']);
  expect(running.manualProofRequirements).toEqual([
    'delete executor runtime smoke before running execution can be claimed',
  ]);
  expect(manual.payloadDeletionState).toBe('manual-required');
  expect(manual.deleteRefs).toEqual(['status-backend-payload-delete-manual-proof-ref']);
  expect(failed.failureRefs).toEqual(['status-backend-delete-executor-failure-ref']);
  expect(audit.auditExportState).toBe('support-safe-export-ready');
  expect(unavailable.executorExecutionState).toBe('not-executed');
  expect(unavailable.payloadDeletionState).toBe('not-requested');
}

function assertInvalidRows() {
  const request = entryFor('status-backend-delete-request-recorded');
  const queued = entryFor('status-backend-delete-executor-queued');
  const manual = entryFor('status-backend-deletion-manual-required');
  const failed = entryFor('status-backend-delete-executor-failed');
  const audit = entryFor('status-backend-delete-executor-audit-export-ready');
  const unavailable = entryFor('status-backend-delete-executor-backend-unavailable');

  for (const invalidEntry of [
    { ...request, executorId: 'invalid-token', containsTokens: true },
    { ...request, executorId: 'invalid-child-activity', containsRawChildActivity: true },
    { ...request, executorId: 'invalid-raw-support-bundle', containsRawSupportBundles: true },
    { ...request, executorId: 'invalid-provider-secret', containsProviderSecrets: true },
    { ...request, executorId: 'invalid-account-lookup-result', containsAccountLookupResults: true },
    { ...request, executorId: 'invalid-billing-contact-record', containsBillingContactRecords: true },
    { ...request, executorId: 'invalid-backend-upload-payload', containsBackendUploadPayloads: true },
    { ...request, executorId: 'invalid-status-backend-payload', containsStatusBackendPayloads: true },
    { ...request, executorId: 'invalid-public-runtime-payload', containsPublicRuntimePayloads: true },
    { ...request, executorId: 'invalid-remote-transcript', containsRemoteSupportTranscripts: true },
    { ...request, executorId: 'invalid-status-backend-execution', realStatusBackendExecution: true },
    { ...request, executorId: 'invalid-durable-storage', durableStatusBackendPayloadStorage: true },
    { ...request, executorId: 'invalid-delete-executor-executed', statusBackendDeleteExecutorExecuted: true },
    { ...request, executorId: 'invalid-payload-deletion', statusBackendPayloadDeletionExecuted: true },
    { ...request, executorId: 'invalid-retry-worker', retryWorkerExecution: true },
    { ...request, executorId: 'invalid-audit-persistence', auditPersistenceExecuted: true },
    { ...request, executorId: 'invalid-public-runtime-execution', publicRuntimeExecution: true },
    { ...request, executorId: 'invalid-support-upload-execution', supportBackendUploadExecution: true },
    { ...request, executorId: 'invalid-provider-execution', providerExecution: true },
    { ...request, executorId: 'invalid-account-lookup', accountLookupExecuted: true },
    { ...request, executorId: 'invalid-billing-contact', billingProviderContactExecuted: true },
    { ...request, executorId: 'invalid-remote-support', remoteSupportSessionExecuted: true },
    { ...request, executorId: 'invalid-production-sla', productionSlaClaimed: true },
    { ...request, executorId: 'invalid-hosted-family-data', ocentraHostedFamilyDataDefault: true },
    { ...request, executorId: 'invalid-no-consent', parentConsentState: 'required' },
    { ...request, executorId: 'invalid-no-delete-ref', deleteRefs: [] },
    { ...request, executorId: 'invalid-no-executor-ref', executorRefs: [] },
    { ...request, executorId: 'invalid-duplicate-data-class', disclosedDataClasses: ['manual-runbook-ref'] },
    { ...queued, executorId: 'invalid-queued-not-manual', executorExecutionState: 'not-executed' },
    { ...manual, executorId: 'invalid-manual-no-proof', manualProofRequirements: [] },
    { ...failed, executorId: 'invalid-failure-no-ref', failureRefs: [] },
    { ...audit, executorId: 'invalid-export-manual', auditExportState: 'manual-required' },
    { ...unavailable, executorId: 'invalid-unavailable-executed', executorExecutionState: 'manual-required' },
  ]) {
    expect(() => StatusBackendDeleteExecutorEntrySchema.parse(invalidEntry)).toThrow();
  }
}

function entryFor(executorId: string) {
  const entry = StatusBackendDeleteExecutorReadModel.entries.find((candidate) => candidate.executorId === executorId);
  if (entry === undefined) {
    throw new Error(`Missing status backend delete executor entry: ${executorId}`);
  }
  return entry;
}

function countBy(values: readonly string[]) {
  return values.reduce<Record<string, number>>((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}
