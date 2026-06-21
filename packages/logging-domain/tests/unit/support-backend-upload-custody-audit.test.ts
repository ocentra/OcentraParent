import { describe, expect, it } from 'vitest';

import {
  SupportBackendUploadCustodyAuditEntrySchema,
  SupportBackendUploadCustodyAuditReadModelSchema,
  SupportBackendUploadCustodyAuditRequiredDataClasses,
} from '@ocentra-parent/schema-domain/support-backend-upload-custody-audit';
import { SupportBackendUploadCustodyAuditReadModel } from '@ocentra-parent/schema-domain/support-backend-upload-custody-audit-read-model';

describe('support backend upload custody audit logging contract', () => {
  it('covers custody retention delete and support-safe audit export states', assertCoverage);
  it('requires parent consent redaction audit status runtime and custody refs', assertSafeBoundary);
  it('keeps retention and deletion manual before backend custody claims', assertCustodyBoundary);
  it(
    'rejects sensitive custody execution retention deletion account billing remote support and SLA claims',
    assertInvalidRows
  );
});

function assertCoverage() {
  const readModel = SupportBackendUploadCustodyAuditReadModelSchema.parse(SupportBackendUploadCustodyAuditReadModel);

  expect(readModel.readModelId).toBe('production-support-backend-upload-custody-audit-proof');
  expect(readModel.entries).toHaveLength(5);
  expect(countBy(readModel.entries.map((entry) => entry.auditState))).toEqual({
    'custody-boundary-recorded': 1,
    'retention-manual-required': 1,
    'delete-request-recorded': 1,
    'deletion-manual-required': 1,
    'audit-export-ready': 1,
  });
}

function assertSafeBoundary() {
  for (const entry of SupportBackendUploadCustodyAuditReadModel.entries) {
    expect(entry.parentInitiationState).toBe('parent-initiated');
    expect(entry.parentConsentState).toBe('parent-approved');
    expect(entry.executionClaimState).toBe('custody-audit-boundary-only');
    expect(entry.payloadState).toBe('redacted-audit-refs-only');
    expect(entry.custodyState).toBe('parent-owned-export-only');
    expect(entry.disclosedDataClasses).toEqual([...SupportBackendUploadCustodyAuditRequiredDataClasses]);
    expect(entry.consentRefs).toEqual(['parent-support-upload-consent-artifact-ref']);
    expect(entry.redactionRefs).toEqual([
      'support-bundle-redaction-proof-ref',
      'support-upload-redaction-preflight-ref',
    ]);
    expect(entry.auditRefs).toEqual(['support-upload-custody-audit-event-ref']);
    expect(entry.statusRefs).toEqual(['production-support-backend-upload-status-proof-ref']);
    expect(entry.runtimeRefs).toEqual(['production-support-backend-upload-execution-runtime-proof-ref']);
    expect(entry.custodyRefs).toEqual(['data-custody-support-upload-boundary-ref']);
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
    expect(entry.supportBackendRetainedPayload).toBe(false);
    expect(entry.supportBackendDeletedPayload).toBe(false);
    expect(entry.ocentraHostedFamilyDataDefault).toBe(false);
    expect(entry.accountLookupExecuted).toBe(false);
    expect(entry.billingProviderContactExecuted).toBe(false);
    expect(entry.remoteSupportSessionExecuted).toBe(false);
    expect(entry.productionSlaClaimed).toBe(false);
  }
}

function assertCustodyBoundary() {
  const custody = entryFor('support-upload-custody-boundary-recorded');
  const retention = entryFor('support-upload-retention-manual-required');
  const deleteRequest = entryFor('support-upload-delete-request-recorded');
  const deletion = entryFor('support-upload-deletion-manual-required');
  const auditExport = entryFor('support-upload-custody-audit-export-ready');

  expect(custody.retentionState).toBe('manual-required');
  expect(custody.retentionRefs).toEqual(['support-upload-retention-manual-proof-ref']);
  expect(retention.manualProofRequirements).toEqual([
    'published retention runbook before support backend retention can be claimed',
  ]);
  expect(deleteRequest.deleteState).toBe('manual-required');
  expect(deleteRequest.deleteRefs).toEqual(['parent-support-upload-delete-request-ref']);
  expect(deletion.deleteRefs).toEqual(['support-upload-delete-manual-proof-ref']);
  expect(auditExport.auditExportState).toBe('support-safe-export-ready');
  expect(auditExport.retentionRefs).toEqual(['support-upload-retention-audit-ref']);
  expect(auditExport.deleteRefs).toEqual(['support-upload-delete-audit-ref']);
}

function assertInvalidRows() {
  const custody = entryFor('support-upload-custody-boundary-recorded');
  const deletion = entryFor('support-upload-deletion-manual-required');
  const auditExport = entryFor('support-upload-custody-audit-export-ready');

  for (const invalidEntry of [
    { ...custody, auditId: 'invalid-token', containsTokens: true },
    { ...custody, auditId: 'invalid-child-activity', containsRawChildActivity: true },
    { ...custody, auditId: 'invalid-raw-url', containsRawUrls: true },
    { ...custody, auditId: 'invalid-screenshot', containsScreenshots: true },
    { ...custody, auditId: 'invalid-journal', containsJournals: true },
    { ...custody, auditId: 'invalid-sqlite', containsSqliteSnapshots: true },
    { ...custody, auditId: 'invalid-private-path', containsPrivatePaths: true },
    { ...custody, auditId: 'invalid-command-line', containsCommandLines: true },
    { ...custody, auditId: 'invalid-keystroke', containsKeystrokes: true },
    { ...custody, auditId: 'invalid-clipboard', containsClipboardData: true },
    { ...custody, auditId: 'invalid-message-content', containsMessageContents: true },
    { ...custody, auditId: 'invalid-provider-secret', containsProviderSecrets: true },
    { ...custody, auditId: 'invalid-remote-transcript', containsRemoteSupportTranscripts: true },
    { ...custody, auditId: 'invalid-backend-execution', realSupportBackendUploadExecuted: true },
    { ...custody, auditId: 'invalid-retained-payload', supportBackendRetainedPayload: true },
    { ...custody, auditId: 'invalid-deleted-payload', supportBackendDeletedPayload: true },
    { ...custody, auditId: 'invalid-hosted-family-data', ocentraHostedFamilyDataDefault: true },
    { ...custody, auditId: 'invalid-account-lookup', accountLookupExecuted: true },
    { ...custody, auditId: 'invalid-billing-contact', billingProviderContactExecuted: true },
    { ...custody, auditId: 'invalid-remote-support', remoteSupportSessionExecuted: true },
    { ...custody, auditId: 'invalid-production-sla', productionSlaClaimed: true },
    { ...custody, auditId: 'invalid-no-consent', parentConsentState: 'required' },
    { ...custody, auditId: 'invalid-no-custody-ref', custodyRefs: [] },
    { ...custody, auditId: 'invalid-duplicate-data-class', disclosedDataClasses: ['audit-export-ref'] },
    { ...custody, auditId: 'invalid-retention-not-manual', retentionState: 'not-applicable' },
    { ...deletion, auditId: 'invalid-delete-no-ref', deleteRefs: [] },
    { ...auditExport, auditId: 'invalid-export-manual', auditExportState: 'manual-required' },
  ]) {
    expect(() => SupportBackendUploadCustodyAuditEntrySchema.parse(invalidEntry)).toThrow();
  }
}

function entryFor(auditId: string) {
  const entry = SupportBackendUploadCustodyAuditReadModel.entries.find((candidate) => candidate.auditId === auditId);
  if (entry === undefined) {
    throw new Error(`Missing support backend upload custody audit entry: ${auditId}`);
  }
  return entry;
}

function countBy(values: readonly string[]) {
  return values.reduce<Record<string, number>>((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}
