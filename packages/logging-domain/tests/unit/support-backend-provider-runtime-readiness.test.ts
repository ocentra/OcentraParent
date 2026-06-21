import { describe, expect, it } from 'vitest';

import {
  SupportBackendProviderRuntimeReadinessEntrySchema,
  SupportBackendProviderRuntimeReadinessReadModelSchema,
  SupportBackendProviderRuntimeReadinessRequiredDataClasses,
} from '@ocentra-parent/schema-domain/support-backend-provider-runtime-readiness';
import { SupportBackendProviderRuntimeReadinessReadModel } from '@ocentra-parent/schema-domain/support-backend-provider-runtime-readiness-read-model';

describe('support backend provider runtime readiness logging contract', () => {
  it('covers upload provider billing account legal remote support SLA and audit states', assertCoverage);
  it('keeps every row support-safe and linked to existing production support proof refs', assertSafeRows);
  it('keeps upload and provider execution readiness manual before execution claims', assertManualBoundaries);
  it(
    'rejects provider secrets support payload custody execution provider legal remote support and SLA claims',
    assertInvalidRows
  );
});

function assertCoverage() {
  const readModel = SupportBackendProviderRuntimeReadinessReadModelSchema.parse(
    SupportBackendProviderRuntimeReadinessReadModel
  );

  expect(readModel.readModelId).toBe('production-support-backend-provider-runtime-readiness-proof');
  expect(readModel.entries).toHaveLength(8);
  expect(countBy(readModel.entries.map((entry) => entry.readinessState))).toEqual({
    'upload-runtime-linked': 1,
    'provider-secret-preflight-linked': 1,
    'billing-provider-manual-required': 1,
    'account-lookup-manual-required': 1,
    'legal-disclosure-manual-required': 1,
    'remote-support-manual-required': 1,
    'sla-manual-required': 1,
    'audit-export-ready': 1,
  });
}

function assertSafeRows() {
  for (const entry of SupportBackendProviderRuntimeReadinessReadModel.entries) {
    expect(entry.payloadState).toBe('support-safe-status-refs-only');
    expect(entry.custodyState).toBe('no-ocentra-hosted-family-data');
    expect(entry.disclosedDataClasses).toEqual([...SupportBackendProviderRuntimeReadinessRequiredDataClasses]);
    expect(entry.uploadRuntimeRefs).toEqual(['production-support-backend-upload-execution-runtime-proof-ref']);
    expect(entry.custodyAuditRefs).toEqual(['production-support-backend-upload-custody-audit-proof-ref']);
    expect(entry.providerSecretRefs).toEqual(['provider-secret-execution-readiness-proof-ref']);
    expect(entry.accountBillingRefs).toEqual(['production-support-account-sla-status-proof-ref']);
    expect(entry.privacyLegalRefs).toEqual(['production-support-privacy-legal-disclosure-status-proof-ref']);
    expect(entry.caseStatusRefs).toEqual(['production-support-case-resolution-status-proof-ref']);
    expect(entry.auditRefs).toEqual(['support-backend-provider-runtime-readiness-audit-ref']);
    expect(entry.containsProviderSecrets).toBe(false);
    expect(entry.containsPaymentProviderTokens).toBe(false);
    expect(entry.containsRawChildActivity).toBe(false);
    expect(entry.containsRawSupportBundlePayloads).toBe(false);
    expect(entry.containsAccountLookupResults).toBe(false);
    expect(entry.containsBillingProviderContactRecords).toBe(false);
    expect(entry.containsRemoteSupportTranscripts).toBe(false);
    expect(entry.supportBackendUploadExecuted).toBe(false);
    expect(entry.providerSecretDelivered).toBe(false);
    expect(entry.accountLookupExecuted).toBe(false);
    expect(entry.billingProviderContactExecuted).toBe(false);
    expect(entry.legalDisclosureExecuted).toBe(false);
    expect(entry.remoteSupportSessionExecuted).toBe(false);
    expect(entry.productionSlaClaimed).toBe(false);
    expect(entry.ocentraHostedFamilyDataDefault).toBe(false);
  }
}

function assertManualBoundaries() {
  expect(entryFor('support-backend-provider-upload-runtime-linked').uploadRuntimeState).toBe('readiness-only');
  expect(entryFor('support-backend-provider-secret-preflight-linked').providerSecretState).toBe('manual-required');
  expect(entryFor('support-backend-billing-provider-manual-required').billingProviderState).toBe('manual-required');
  expect(entryFor('support-backend-account-lookup-manual-required').accountLookupState).toBe('manual-required');
  expect(entryFor('support-backend-legal-disclosure-manual-required').legalDisclosureState).toBe('manual-required');
  expect(entryFor('support-backend-remote-support-manual-required').remoteSupportState).toBe('manual-required');
  expect(entryFor('support-backend-sla-manual-required').productionSlaState).toBe('manual-required');
  expect(entryFor('support-backend-provider-audit-export-ready').manualProofRequirements).toEqual([
    'support-safe provider runtime audit export review before runtime/provider execution can be claimed',
  ]);
}

function assertInvalidRows() {
  const upload = entryFor('support-backend-provider-upload-runtime-linked');
  const provider = entryFor('support-backend-provider-secret-preflight-linked');
  const audit = entryFor('support-backend-provider-audit-export-ready');

  for (const invalidEntry of [
    { ...upload, statusId: 'invalid-provider-secret', containsProviderSecrets: true },
    { ...upload, statusId: 'invalid-payment-token', containsPaymentProviderTokens: true },
    { ...upload, statusId: 'invalid-child-activity', containsRawChildActivity: true },
    { ...upload, statusId: 'invalid-support-payload', containsRawSupportBundlePayloads: true },
    { ...upload, statusId: 'invalid-account-result', containsAccountLookupResults: true },
    { ...upload, statusId: 'invalid-billing-record', containsBillingProviderContactRecords: true },
    { ...upload, statusId: 'invalid-remote-transcript', containsRemoteSupportTranscripts: true },
    { ...upload, statusId: 'invalid-support-upload-executed', supportBackendUploadExecuted: true },
    { ...upload, statusId: 'invalid-provider-secret-delivered', providerSecretDelivered: true },
    { ...upload, statusId: 'invalid-account-lookup-executed', accountLookupExecuted: true },
    { ...upload, statusId: 'invalid-billing-provider-executed', billingProviderContactExecuted: true },
    { ...upload, statusId: 'invalid-legal-executed', legalDisclosureExecuted: true },
    { ...upload, statusId: 'invalid-remote-support-executed', remoteSupportSessionExecuted: true },
    { ...upload, statusId: 'invalid-sla', productionSlaClaimed: true },
    { ...upload, statusId: 'invalid-hosted-family-data', ocentraHostedFamilyDataDefault: true },
    { ...upload, statusId: 'invalid-upload-not-implemented', uploadRuntimeState: 'not-implemented' },
    { ...provider, statusId: 'invalid-provider-readiness-only', providerSecretState: 'readiness-only' },
    { ...audit, statusId: 'invalid-no-audit-ref', auditRefs: [] },
    { ...audit, statusId: 'invalid-duplicate-data-class', disclosedDataClasses: ['manual-proof-ref'] },
  ]) {
    expect(() => SupportBackendProviderRuntimeReadinessEntrySchema.parse(invalidEntry)).toThrow();
  }
}

function entryFor(statusId: string) {
  const entry = SupportBackendProviderRuntimeReadinessReadModel.entries.find(
    (candidate) => candidate.statusId === statusId
  );
  if (entry === undefined) {
    throw new Error(`Missing support backend provider runtime readiness entry: ${statusId}`);
  }
  return entry;
}

function countBy(values: readonly string[]) {
  return values.reduce<Record<string, number>>((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}
