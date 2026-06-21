import { describe, expect, it } from 'vitest';

import {
  ProviderSecretExecutionReadinessEntrySchema,
  ProviderSecretExecutionReadinessReadModelSchema,
  ProviderSecretExecutionRequiredDataClasses,
} from '@ocentra-parent/schema-domain/provider-secret-execution-readiness';
import { ProviderSecretExecutionReadinessReadModel } from '@ocentra-parent/schema-domain/provider-secret-execution-readiness-read-model';

describe('provider secret execution readiness logging contract', () => {
  it('covers execution boundary secret store rotation revocation operator manual and audit states', assertCoverage);
  it('keeps every row support-safe and linked to custody preflight operator audit refs', assertSafeRows);
  it('keeps provider secret execution readiness manual before execution claims', assertManualStates);
  it(
    'rejects provider secrets tokens payload custody execution account billing remote support and SLA claims',
    assertInvalidRows
  );
});

function assertCoverage() {
  const readModel = ProviderSecretExecutionReadinessReadModelSchema.parse(ProviderSecretExecutionReadinessReadModel);

  expect(readModel.readModelId).toBe('provider-secret-execution-readiness-proof');
  expect(readModel.entries).toHaveLength(7);
  expect(countBy(readModel.entries.map((entry) => entry.readinessStatus))).toEqual({
    'execution-boundary-recorded': 1,
    'backend-secret-store-preflight-required': 1,
    'rotation-preflight-required': 1,
    'revocation-preflight-required': 1,
    'operator-approval-required': 1,
    'execution-manual-required': 1,
    'audit-export-ready': 1,
  });
}

function assertSafeRows() {
  for (const entry of ProviderSecretExecutionReadinessReadModel.entries) {
    expect(entry.payloadState).toBe('support-safe-status-refs-only');
    expect(entry.disclosedDataClasses).toEqual([...ProviderSecretExecutionRequiredDataClasses]);
    expect(entry.allowedDestinations).toEqual(['manual-security-runbook', 'support-safe-audit-export']);
    expect(entry.custodyStatusRefs).toEqual(['production-support-provider-secret-custody-status-proof-ref']);
    expect(entry.backendSecretStoreRefs).toEqual(['backend-secret-store-preflight-ref']);
    expect(entry.rotationRefs).toEqual(['provider-secret-rotation-preflight-ref']);
    expect(entry.revocationRefs).toEqual(['provider-secret-revocation-preflight-ref']);
    expect(entry.operatorApprovalRefs).toEqual(['provider-secret-operator-approval-ref']);
    expect(entry.auditRefs).toEqual(['provider-secret-execution-audit-ref']);
    expect(entry.containsProviderSecrets).toBe(false);
    expect(entry.containsPaymentProviderTokens).toBe(false);
    expect(entry.containsRawChildActivity).toBe(false);
    expect(entry.containsRawSupportBundlePayloads).toBe(false);
    expect(entry.containsAccountLookupResults).toBe(false);
    expect(entry.containsBillingProviderContactRecords).toBe(false);
    expect(entry.containsRemoteSupportTranscripts).toBe(false);
    expect(entry.backendSecretStoreExecuted).toBe(false);
    expect(entry.providerSecretRotationExecuted).toBe(false);
    expect(entry.providerSecretRevocationExecuted).toBe(false);
    expect(entry.providerSecretExecutionDelivered).toBe(false);
    expect(entry.supportBackendUploadExecuted).toBe(false);
    expect(entry.accountLookupExecuted).toBe(false);
    expect(entry.billingProviderContactExecuted).toBe(false);
    expect(entry.remoteSupportSessionExecuted).toBe(false);
    expect(entry.productionSlaClaimed).toBe(false);
    expect(entry.ocentraHostedFamilyDataDefault).toBe(false);
  }
}

function assertManualStates() {
  expect(entryFor('provider-secret-execution-boundary-recorded').executionState).toBe('not-implemented');
  expect(entryFor('provider-secret-backend-store-preflight-required').backendSecretStoreState).toBe('manual-required');
  expect(entryFor('provider-secret-rotation-preflight-required').rotationState).toBe('manual-required');
  expect(entryFor('provider-secret-revocation-preflight-required').revocationState).toBe('manual-required');
  expect(entryFor('provider-secret-operator-approval-required').operatorApprovalState).toBe('manual-required');
  expect(entryFor('provider-secret-execution-manual-required').executionState).toBe('manual-required');
  expect(entryFor('provider-secret-execution-audit-export-ready').allowedDestinations).toEqual([
    'manual-security-runbook',
    'support-safe-audit-export',
  ]);
}

function assertInvalidRows() {
  const boundary = entryFor('provider-secret-execution-boundary-recorded');
  const manual = entryFor('provider-secret-execution-manual-required');
  const audit = entryFor('provider-secret-execution-audit-export-ready');

  for (const invalidEntry of [
    { ...boundary, statusId: 'invalid-provider-secret', containsProviderSecrets: true },
    { ...boundary, statusId: 'invalid-provider-token', containsPaymentProviderTokens: true },
    { ...boundary, statusId: 'invalid-child-activity', containsRawChildActivity: true },
    { ...boundary, statusId: 'invalid-support-payload', containsRawSupportBundlePayloads: true },
    { ...boundary, statusId: 'invalid-account-result', containsAccountLookupResults: true },
    { ...boundary, statusId: 'invalid-billing-record', containsBillingProviderContactRecords: true },
    { ...boundary, statusId: 'invalid-remote-transcript', containsRemoteSupportTranscripts: true },
    { ...boundary, statusId: 'invalid-backend-store-executed', backendSecretStoreExecuted: true },
    { ...boundary, statusId: 'invalid-rotation-executed', providerSecretRotationExecuted: true },
    { ...boundary, statusId: 'invalid-revocation-executed', providerSecretRevocationExecuted: true },
    { ...boundary, statusId: 'invalid-execution-delivered', providerSecretExecutionDelivered: true },
    { ...boundary, statusId: 'invalid-support-upload', supportBackendUploadExecuted: true },
    { ...boundary, statusId: 'invalid-account-lookup', accountLookupExecuted: true },
    { ...boundary, statusId: 'invalid-billing-contact', billingProviderContactExecuted: true },
    { ...boundary, statusId: 'invalid-remote-support', remoteSupportSessionExecuted: true },
    { ...boundary, statusId: 'invalid-sla', productionSlaClaimed: true },
    { ...boundary, statusId: 'invalid-hosted-family-data', ocentraHostedFamilyDataDefault: true },
    { ...boundary, statusId: 'invalid-no-custody-ref', custodyStatusRefs: [] },
    { ...boundary, statusId: 'invalid-duplicate-data-class', disclosedDataClasses: ['manual-proof-ref'] },
    { ...boundary, statusId: 'invalid-boundary-manual', executionState: 'manual-required' },
    { ...manual, statusId: 'invalid-manual-readiness-only', executionState: 'readiness-only' },
    { ...audit, statusId: 'invalid-audit-no-destination', allowedDestinations: ['manual-security-runbook'] },
  ]) {
    expect(() => ProviderSecretExecutionReadinessEntrySchema.parse(invalidEntry)).toThrow();
  }
}

function entryFor(statusId: string) {
  const entry = ProviderSecretExecutionReadinessReadModel.entries.find((candidate) => candidate.statusId === statusId);
  if (entry === undefined) {
    throw new Error(`Missing provider secret execution readiness entry: ${statusId}`);
  }
  return entry;
}

function countBy(values: readonly string[]) {
  return values.reduce<Record<string, number>>((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}
