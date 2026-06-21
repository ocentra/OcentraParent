import { describe, expect, it } from 'vitest';

import {
  StatusBackendPayloadCustodyEntrySchema,
  StatusBackendPayloadCustodyReadModelSchema,
  StatusBackendPayloadCustodyRequiredDataClasses,
} from '@ocentra-parent/schema-domain/status-backend-payload-custody';
import { StatusBackendPayloadCustodyReadModel } from '@ocentra-parent/schema-domain/status-backend-payload-custody-read-model';

describe('status backend payload custody logging contract', () => {
  it('covers custody retention delete audit export and backend unavailable states', assertCoverage);
  it('requires parent consent redaction queue audit target and custody refs', assertSafeBoundary);
  it('keeps status backend payload retention deletion and backend availability manual', assertCustodyBoundary);
  it('rejects sensitive payload custody and execution claims', assertInvalidRows);
});

function assertCoverage() {
  const readModel = StatusBackendPayloadCustodyReadModelSchema.parse(StatusBackendPayloadCustodyReadModel);

  expect(readModel.readModelId).toBe('production-support-status-backend-payload-custody-proof');
  expect(readModel.entries).toHaveLength(6);
  expect(countBy(readModel.entries.map((entry) => entry.custodyState))).toEqual({
    'custody-boundary-recorded': 1,
    'retention-manual-required': 1,
    'delete-request-recorded': 1,
    'deletion-manual-required': 1,
    'audit-export-ready': 1,
    'backend-unavailable': 1,
  });
}

function assertSafeBoundary() {
  for (const entry of StatusBackendPayloadCustodyReadModel.entries) {
    expect(entry.parentConsentState).toBe('parent-approved');
    expect(entry.executionClaimState).toBe('status-backend-payload-custody-boundary-only');
    expect(entry.payloadState).toBe('redacted-status-refs-only');
    expect(entry.disclosedDataClasses).toEqual([...StatusBackendPayloadCustodyRequiredDataClasses]);
    expect(entry.consentRefs).toEqual(['parent-status-backend-payload-consent-ref']);
    expect(entry.targetRefs).toEqual(['production-support-status-backend-public-runtime-followthrough-proof-ref']);
    expect(entry.queueRefs).toEqual(['production-support-status-backend-execution-queue-proof-ref']);
    expect(entry.auditRefs).toEqual(['production-support-status-backend-execution-queue-audit-ref']);
    expect(entry.redactionRefs).toEqual([
      'support-bundle-redaction-proof-ref',
      'status-backend-payload-redaction-summary-ref',
    ]);
    expect(entry.custodyRefs).toEqual(['data-custody-status-backend-payload-boundary-ref']);
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

function assertCustodyBoundary() {
  const custody = entryFor('status-backend-payload-custody-boundary-recorded');
  const retention = entryFor('status-backend-payload-retention-manual-required');
  const deleteRequest = entryFor('status-backend-payload-delete-request-recorded');
  const deletion = entryFor('status-backend-payload-deletion-manual-required');
  const auditExport = entryFor('status-backend-payload-audit-export-ready');
  const unavailable = entryFor('status-backend-payload-backend-unavailable');

  expect(custody.storageState).toBe('manual-required');
  expect(custody.retentionRefs).toEqual(['status-backend-payload-retention-manual-proof-ref']);
  expect(retention.manualProofRequirements).toEqual([
    'published retention runbook before durable status payload storage can be claimed',
  ]);
  expect(deleteRequest.deleteState).toBe('manual-required');
  expect(deleteRequest.deleteRefs).toEqual(['parent-status-backend-payload-delete-request-ref']);
  expect(deletion.deleteRefs).toEqual(['status-backend-payload-delete-manual-proof-ref']);
  expect(auditExport.auditExportState).toBe('support-safe-export-ready');
  expect(auditExport.retentionRefs).toEqual(['status-backend-payload-retention-audit-ref']);
  expect(auditExport.deleteRefs).toEqual(['status-backend-payload-delete-audit-ref']);
  expect(unavailable.storageState).toBe('not-retained');
}

function assertInvalidRows() {
  const custody = entryFor('status-backend-payload-custody-boundary-recorded');
  const deletion = entryFor('status-backend-payload-deletion-manual-required');
  const auditExport = entryFor('status-backend-payload-audit-export-ready');
  const unavailable = entryFor('status-backend-payload-backend-unavailable');

  for (const invalidEntry of [
    { ...custody, custodyId: 'invalid-token', containsTokens: true },
    { ...custody, custodyId: 'invalid-child-activity', containsRawChildActivity: true },
    { ...custody, custodyId: 'invalid-raw-support-bundle', containsRawSupportBundles: true },
    { ...custody, custodyId: 'invalid-provider-secret', containsProviderSecrets: true },
    { ...custody, custodyId: 'invalid-account-lookup-result', containsAccountLookupResults: true },
    { ...custody, custodyId: 'invalid-billing-contact-record', containsBillingContactRecords: true },
    { ...custody, custodyId: 'invalid-backend-upload-payload', containsBackendUploadPayloads: true },
    { ...custody, custodyId: 'invalid-status-backend-payload', containsStatusBackendPayloads: true },
    { ...custody, custodyId: 'invalid-public-runtime-payload', containsPublicRuntimePayloads: true },
    { ...custody, custodyId: 'invalid-remote-transcript', containsRemoteSupportTranscripts: true },
    { ...custody, custodyId: 'invalid-status-backend-execution', realStatusBackendExecution: true },
    { ...custody, custodyId: 'invalid-durable-storage', durableStatusBackendPayloadStorage: true },
    { ...custody, custodyId: 'invalid-payload-deletion', statusBackendPayloadDeletionExecuted: true },
    { ...custody, custodyId: 'invalid-retry-worker', retryWorkerExecution: true },
    { ...custody, custodyId: 'invalid-audit-persistence', auditPersistenceExecuted: true },
    { ...custody, custodyId: 'invalid-public-runtime-execution', publicRuntimeExecution: true },
    { ...custody, custodyId: 'invalid-support-upload-execution', supportBackendUploadExecution: true },
    { ...custody, custodyId: 'invalid-provider-execution', providerExecution: true },
    { ...custody, custodyId: 'invalid-account-lookup', accountLookupExecuted: true },
    { ...custody, custodyId: 'invalid-billing-contact', billingProviderContactExecuted: true },
    { ...custody, custodyId: 'invalid-remote-support', remoteSupportSessionExecuted: true },
    { ...custody, custodyId: 'invalid-production-sla', productionSlaClaimed: true },
    { ...custody, custodyId: 'invalid-hosted-family-data', ocentraHostedFamilyDataDefault: true },
    { ...custody, custodyId: 'invalid-no-consent', parentConsentState: 'required' },
    { ...custody, custodyId: 'invalid-no-custody-ref', custodyRefs: [] },
    { ...custody, custodyId: 'invalid-duplicate-data-class', disclosedDataClasses: ['manual-runbook-ref'] },
    { ...custody, custodyId: 'invalid-retention-not-manual', storageState: 'not-retained' },
    { ...deletion, custodyId: 'invalid-delete-no-ref', deleteRefs: [] },
    { ...auditExport, custodyId: 'invalid-export-manual', auditExportState: 'manual-required' },
    { ...unavailable, custodyId: 'invalid-unavailable-retention', storageState: 'manual-required' },
  ]) {
    expect(() => StatusBackendPayloadCustodyEntrySchema.parse(invalidEntry)).toThrow();
  }
}

function entryFor(custodyId: string) {
  const entry = StatusBackendPayloadCustodyReadModel.entries.find((candidate) => candidate.custodyId === custodyId);
  if (entry === undefined) {
    throw new Error(`Missing status backend payload custody entry: ${custodyId}`);
  }
  return entry;
}

function countBy(values: readonly string[]) {
  return values.reduce<Record<string, number>>((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}
