import { describe, expect, it } from 'vitest';

import {
  ProviderSecretRotationRevocationRequiredDataClasses,
  ProviderSecretRotationRevocationStatusEntrySchema,
  ProviderSecretRotationRevocationStatusReadModelSchema,
} from '../src/provider-secret-rotation-revocation-status';
import { ProviderSecretRotationRevocationStatusReadModel } from '../src/provider-secret-rotation-revocation-status-read-model';

describe('provider secret rotation revocation status logging contract', () => {
  it('covers rotation revocation preflight manual and audit states', assertCoverage);
  it('keeps every row support-safe and linked to custody readiness backend approval and audit refs', assertSafeRows);
  it('keeps rotation and revocation preflight or manual before execution claims', assertManualStates);
  it('rejects secrets tokens payload execution account billing remote support and SLA claims', assertInvalidRows);
});

function assertCoverage() {
  const readModel = ProviderSecretRotationRevocationStatusReadModelSchema.parse(
    ProviderSecretRotationRevocationStatusReadModel
  );

  expect(readModel.readModelId).toBe('production-support-provider-secret-rotation-revocation-status-proof');
  expect(readModel.entries).toHaveLength(7);
  expect(countBy(readModel.entries.map((entry) => entry.rotationRevocationStatus))).toEqual({
    'rotation-requested': 1,
    'rotation-preflight-ready': 1,
    'rotation-manual-required': 1,
    'revocation-requested': 1,
    'revocation-preflight-ready': 1,
    'revocation-manual-required': 1,
    'audit-export-ready': 1,
  });
}

function assertSafeRows() {
  for (const entry of ProviderSecretRotationRevocationStatusReadModel.entries) {
    expect(entry.payloadState).toBe('support-safe-status-refs-only');
    expect(entry.disclosedDataClasses).toEqual([...ProviderSecretRotationRevocationRequiredDataClasses]);
    expect(entry.allowedDestinations).toEqual(['support-safe-status-boundary', 'manual-security-runbook']);
    expect(entry.custodyStatusRefs).toEqual(['production-support-provider-secret-custody-status-proof-ref']);
    expect(entry.executionReadinessRefs).toEqual(['provider-secret-execution-readiness-proof-ref']);
    expect(entry.backendSecretStoreRefs).toEqual(['provider-secret-backend-secret-store-preflight-ref']);
    expect(entry.auditRefs).toEqual(['provider-secret-rotation-revocation-audit-status-ref']);
    expect(entry.containsProviderSecrets).toBe(false);
    expect(entry.containsPaymentProviderTokens).toBe(false);
    expect(entry.containsRawChildActivity).toBe(false);
    expect(entry.containsRawSupportBundlePayloads).toBe(false);
    expect(entry.containsAccountLookupResults).toBe(false);
    expect(entry.containsBillingProviderContactRecords).toBe(false);
    expect(entry.containsRemoteSupportTranscripts).toBe(false);
    expect(entry.backendSecretStoreExecuted).toBe(false);
    expect(entry.rotationExecuted).toBe(false);
    expect(entry.revocationExecuted).toBe(false);
    expect(entry.providerSecretDelivered).toBe(false);
    expect(entry.supportBackendUploadExecuted).toBe(false);
    expect(entry.accountLookupExecuted).toBe(false);
    expect(entry.billingProviderContactExecuted).toBe(false);
    expect(entry.remoteSupportSessionExecuted).toBe(false);
    expect(entry.productionSlaClaimed).toBe(false);
    expect(entry.ocentraHostedFamilyDataDefault).toBe(false);
  }
}

function assertManualStates() {
  expect(entryFor('provider-secret-rotation-preflight-ready').rotationState).toBe('preflight-ready');
  expect(entryFor('provider-secret-rotation-manual-required').rotationState).toBe('manual-required');
  expect(entryFor('provider-secret-revocation-preflight-ready').revocationState).toBe('preflight-ready');
  expect(entryFor('provider-secret-revocation-manual-required').revocationState).toBe('manual-required');
  expect(entryFor('provider-secret-rotation-request-recorded').operatorApprovalRefs).toEqual([
    'provider-secret-rotation-operator-approval-ref',
  ]);
  expect(entryFor('provider-secret-revocation-request-recorded').operatorApprovalRefs).toEqual([
    'provider-secret-revocation-operator-approval-ref',
  ]);
}

function assertInvalidRows() {
  const rotation = entryFor('provider-secret-rotation-manual-required');
  const revocation = entryFor('provider-secret-revocation-manual-required');

  for (const invalidEntry of [
    { ...rotation, statusId: 'invalid-provider-secret', containsProviderSecrets: true },
    { ...rotation, statusId: 'invalid-provider-token', containsPaymentProviderTokens: true },
    { ...rotation, statusId: 'invalid-child-activity', containsRawChildActivity: true },
    { ...rotation, statusId: 'invalid-support-payload', containsRawSupportBundlePayloads: true },
    { ...rotation, statusId: 'invalid-account-result', containsAccountLookupResults: true },
    { ...rotation, statusId: 'invalid-billing-record', containsBillingProviderContactRecords: true },
    { ...rotation, statusId: 'invalid-remote-transcript', containsRemoteSupportTranscripts: true },
    { ...rotation, statusId: 'invalid-backend-store-executed', backendSecretStoreExecuted: true },
    { ...rotation, statusId: 'invalid-rotation-executed', rotationExecuted: true },
    { ...revocation, statusId: 'invalid-revocation-executed', revocationExecuted: true },
    { ...rotation, statusId: 'invalid-provider-delivered', providerSecretDelivered: true },
    { ...rotation, statusId: 'invalid-support-upload', supportBackendUploadExecuted: true },
    { ...rotation, statusId: 'invalid-account-lookup', accountLookupExecuted: true },
    { ...rotation, statusId: 'invalid-billing-contact', billingProviderContactExecuted: true },
    { ...rotation, statusId: 'invalid-remote-support', remoteSupportSessionExecuted: true },
    { ...rotation, statusId: 'invalid-sla', productionSlaClaimed: true },
    { ...rotation, statusId: 'invalid-hosted-family-data', ocentraHostedFamilyDataDefault: true },
    { ...rotation, statusId: 'invalid-no-custody-ref', custodyStatusRefs: [] },
    { ...rotation, statusId: 'invalid-duplicate-data-class', disclosedDataClasses: ['manual-proof-ref'] },
    { ...rotation, statusId: 'invalid-rotation-no-ref', rotationRefs: [] },
    { ...revocation, statusId: 'invalid-revocation-no-ref', revocationRefs: [] },
    { ...rotation, statusId: 'invalid-no-approval-ref', operatorApprovalRefs: [] },
  ]) {
    expect(() => ProviderSecretRotationRevocationStatusEntrySchema.parse(invalidEntry)).toThrow();
  }
}

function entryFor(statusId: string) {
  const entry = ProviderSecretRotationRevocationStatusReadModel.entries.find(
    (candidate) => candidate.statusId === statusId
  );
  if (entry === undefined) {
    throw new Error(`Missing provider secret rotation/revocation status entry: ${statusId}`);
  }
  return entry;
}

function countBy(values: readonly string[]) {
  return values.reduce<Record<string, number>>((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}
