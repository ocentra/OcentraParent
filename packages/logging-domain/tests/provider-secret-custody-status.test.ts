import { describe, expect, it } from 'vitest';

import {
  ProviderSecretCustodyRequiredDataClasses,
  ProviderSecretCustodyStatusEntrySchema,
  ProviderSecretCustodyStatusReadModelSchema,
} from '../src/provider-secret-custody-status';
import { ProviderSecretCustodyStatusReadModel } from '../src/provider-secret-custody-status-read-model';

describe('provider secret custody status logging contract', () => {
  it('covers provider secret custody absence backend store rotation revocation and audit states', assertCoverage);
  it(
    'keeps every row support-safe and linked to legal provider billing redaction audit and custody refs',
    assertSafeRows
  );
  it(
    'keeps provider secret custody backend store rotation and revocation manual before execution claims',
    assertManualStates
  );
  it(
    'rejects provider secrets tokens payload custody execution account billing remote support and SLA claims',
    assertInvalidRows
  );
});

function assertCoverage() {
  const readModel = ProviderSecretCustodyStatusReadModelSchema.parse(ProviderSecretCustodyStatusReadModel);

  expect(readModel.readModelId).toBe('production-support-provider-secret-custody-status-proof');
  expect(readModel.entries).toHaveLength(6);
  expect(countBy(readModel.entries.map((entry) => entry.custodyStatus))).toEqual({
    'custody-boundary-recorded': 1,
    'provider-secret-absent': 1,
    'backend-secret-store-manual-required': 1,
    'rotation-manual-required': 1,
    'revocation-manual-required': 1,
    'audit-export-ready': 1,
  });
}

function assertSafeRows() {
  for (const entry of ProviderSecretCustodyStatusReadModel.entries) {
    expect(entry.payloadState).toBe('support-safe-status-refs-only');
    expect(entry.custodyBoundaryState).toBe('no-provider-secret-custody');
    expect(entry.disclosedDataClasses).toEqual([...ProviderSecretCustodyRequiredDataClasses]);
    expect(entry.allowedDestinations).toEqual(['support-safe-status-boundary', 'manual-security-runbook']);
    expect(entry.legalProviderRefs).toEqual(['production-support-legal-provider-readiness-proof-ref']);
    expect(entry.billingSupportRefs).toEqual(['billing-support-admin-status-proof-ref']);
    expect(entry.redactionRefs).toEqual(['support-bundle-redaction-proof-ref']);
    expect(entry.auditRefs).toEqual(['provider-secret-custody-audit-status-ref']);
    expect(entry.custodyRefs).toEqual(['data-custody-provider-secret-non-custody-boundary-ref']);
    expect(entry.containsProviderSecrets).toBe(false);
    expect(entry.containsPaymentProviderTokens).toBe(false);
    expect(entry.containsRawChildActivity).toBe(false);
    expect(entry.containsRawSupportBundlePayloads).toBe(false);
    expect(entry.containsAccountLookupResults).toBe(false);
    expect(entry.containsBillingProviderContactRecords).toBe(false);
    expect(entry.containsRemoteSupportTranscripts).toBe(false);
    expect(entry.providerSecretCustodyExecuted).toBe(false);
    expect(entry.backendSecretStoreImplemented).toBe(false);
    expect(entry.rotationExecuted).toBe(false);
    expect(entry.revocationExecuted).toBe(false);
    expect(entry.supportBackendUploadExecuted).toBe(false);
    expect(entry.accountLookupExecuted).toBe(false);
    expect(entry.billingProviderContactExecuted).toBe(false);
    expect(entry.remoteSupportSessionExecuted).toBe(false);
    expect(entry.productionSlaClaimed).toBe(false);
    expect(entry.ocentraHostedFamilyDataDefault).toBe(false);
  }
}

function assertManualStates() {
  expect(entryFor('provider-secret-absent-from-support-status').providerSecretCustodyState).toBe('not-implemented');
  expect(entryFor('provider-secret-absent-from-support-status').backendSecretStoreState).toBe('not-applicable');
  expect(entryFor('provider-secret-backend-store-manual-required').backendSecretStoreState).toBe('manual-required');
  expect(entryFor('provider-secret-rotation-manual-required').rotationRefs).toEqual([
    'provider-secret-rotation-runbook-ref',
  ]);
  expect(entryFor('provider-secret-revocation-manual-required').revocationRefs).toEqual([
    'provider-secret-revocation-runbook-ref',
  ]);
  expect(entryFor('provider-secret-custody-audit-export-ready').rotationRefs).toEqual([
    'provider-secret-rotation-audit-ref',
  ]);
  expect(entryFor('provider-secret-custody-audit-export-ready').revocationRefs).toEqual([
    'provider-secret-revocation-audit-ref',
  ]);
}

function assertInvalidRows() {
  const boundary = entryFor('provider-secret-custody-boundary-recorded');
  const rotation = entryFor('provider-secret-rotation-manual-required');
  const revocation = entryFor('provider-secret-revocation-manual-required');

  for (const invalidEntry of [
    { ...boundary, statusId: 'invalid-provider-secret', containsProviderSecrets: true },
    { ...boundary, statusId: 'invalid-provider-token', containsPaymentProviderTokens: true },
    { ...boundary, statusId: 'invalid-child-activity', containsRawChildActivity: true },
    { ...boundary, statusId: 'invalid-support-payload', containsRawSupportBundlePayloads: true },
    { ...boundary, statusId: 'invalid-account-result', containsAccountLookupResults: true },
    { ...boundary, statusId: 'invalid-billing-record', containsBillingProviderContactRecords: true },
    { ...boundary, statusId: 'invalid-remote-transcript', containsRemoteSupportTranscripts: true },
    { ...boundary, statusId: 'invalid-custody-executed', providerSecretCustodyExecuted: true },
    { ...boundary, statusId: 'invalid-backend-store', backendSecretStoreImplemented: true },
    { ...rotation, statusId: 'invalid-rotation-executed', rotationExecuted: true },
    { ...revocation, statusId: 'invalid-revocation-executed', revocationExecuted: true },
    { ...boundary, statusId: 'invalid-support-upload', supportBackendUploadExecuted: true },
    { ...boundary, statusId: 'invalid-account-lookup', accountLookupExecuted: true },
    { ...boundary, statusId: 'invalid-billing-contact', billingProviderContactExecuted: true },
    { ...boundary, statusId: 'invalid-remote-support', remoteSupportSessionExecuted: true },
    { ...boundary, statusId: 'invalid-sla', productionSlaClaimed: true },
    { ...boundary, statusId: 'invalid-hosted-family-data', ocentraHostedFamilyDataDefault: true },
    { ...boundary, statusId: 'invalid-no-legal-ref', legalProviderRefs: [] },
    { ...boundary, statusId: 'invalid-duplicate-data-class', disclosedDataClasses: ['manual-proof-ref'] },
    { ...rotation, statusId: 'invalid-rotation-no-ref', rotationRefs: [] },
    { ...revocation, statusId: 'invalid-revocation-no-ref', revocationRefs: [] },
  ]) {
    expect(() => ProviderSecretCustodyStatusEntrySchema.parse(invalidEntry)).toThrow();
  }
}

function entryFor(statusId: string) {
  const entry = ProviderSecretCustodyStatusReadModel.entries.find((candidate) => candidate.statusId === statusId);
  if (entry === undefined) {
    throw new Error(`Missing provider secret custody status entry: ${statusId}`);
  }
  return entry;
}

function countBy(values: readonly string[]) {
  return values.reduce<Record<string, number>>((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}
