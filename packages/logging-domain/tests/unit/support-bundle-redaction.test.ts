import { describe, expect, it } from 'vitest';
import {
  SupportBundleRedactionEntrySchema,
  SupportBundleRedactionReadModelSchema,
  SupportBundleRequiredDataClasses,
  SupportBundleRequiredDiagnosticReferenceKinds,
  SupportBundleRequiredPayloadFields,
} from '@ocentra-parent/schema-domain/support-bundle-redaction';
import { SupportBundleRedactionReadModel } from '@ocentra-parent/schema-domain/support-bundle-redaction-read-model';

describe('support bundle redaction logging contract', () => {
  it(
    'covers support consent ready review upload status backend billing and account states',
    assertIncidentStatusCoverage
  );
  it('discloses only support-safe release diagnostic billing and account metadata classes', assertSafeDataClasses);
  it('keeps forbidden child activity private data and provider claims out of support bundles', assertForbiddenData);
  it('requires manual proof refs for billing escalation account lookup and support upload', assertManualBoundaries);
  it('rejects claim upgrades sensitive data missing refs duplicate fields and incoherent states', assertInvalidRows);
});

function assertIncidentStatusCoverage() {
  const readModel = SupportBundleRedactionReadModelSchema.parse(SupportBundleRedactionReadModel);

  expect(readModel.readModelId).toBe('support-bundle-redaction-proof');
  expect(readModel.entries).toHaveLength(8);
  expect(countBy(readModel.entries.map((entry) => entry.incidentStatus))).toEqual({
    'parent-consent-required': 1,
    'support-bundle-ready': 1,
    'manual-review-required': 1,
    'backend-upload-manual-required': 1,
    'status-backend-redaction-ready': 1,
    'status-backend-redaction-manual-required': 1,
    'billing-escalation-manual-required': 1,
    'account-lookup-manual-required': 1,
  });
  expect(entryFor('support-incident-parent-consent-required').parentConsentState).toBe('required');
  expect(entryFor('support-incident-bundle-ready').parentConsentState).toBe('parent-approved');
  expect(entryFor('support-incident-status-backend-redaction-ready').statusBackendRefs).toEqual([
    'status-backend-execution-queue-ref',
    'status-backend-queue-audit-persistence-ref',
    'status-backend-redaction-manifest-ref',
  ]);
}

function assertSafeDataClasses() {
  for (const entry of SupportBundleRedactionReadModel.entries) {
    expect(entry.payloadRedactionState).toBe('support-safe-metadata-only');
    expect(entry.childActivityCustodyState).toBe('no-child-activity-custody');
    expect(entry.disclosedDataClasses).toEqual([...SupportBundleRequiredDataClasses]);
    expect(entry.redactionSafePayloadFields).toEqual([...SupportBundleRequiredPayloadFields]);
    expect(entry.diagnosticReferenceKinds).toEqual([...SupportBundleRequiredDiagnosticReferenceKinds]);
    expect(entry.diagnosticRefs).toEqual([
      'support-safe-proof-json-ref',
      'package-preview-workflow-ref',
      'support-redaction-summary-ref',
      'status-backend-redaction-manifest-ref',
      'manual-support-runbook-ref',
      'production-support-status-row-ref',
    ]);
  }
}

function assertForbiddenData() {
  for (const entry of SupportBundleRedactionReadModel.entries) {
    expect(entry.containsTokens).toBe(false);
    expect(entry.containsChildActivity).toBe(false);
    expect(entry.containsRawUrls).toBe(false);
    expect(entry.containsScreenshots).toBe(false);
    expect(entry.containsJournals).toBe(false);
    expect(entry.containsSqliteSnapshots).toBe(false);
    expect(entry.containsPrivatePaths).toBe(false);
    expect(entry.containsCommandLines).toBe(false);
    expect(entry.containsKeystrokes).toBe(false);
    expect(entry.containsClipboardData).toBe(false);
    expect(entry.containsMessageContents).toBe(false);
    expect(entry.containsStatusBackendPayload).toBe(false);
    expect(entry.publicRuntimePayloadIncluded).toBe(false);
    expect(entry.providerSecretPresent).toBe(false);
    expect(entry.backendUploadExecuted).toBe(false);
    expect(entry.statusBackendExecutionClaimed).toBe(false);
    expect(entry.billingProviderContacted).toBe(false);
    expect(entry.accountLookupExecuted).toBe(false);
    expect(entry.remoteSupportSessionStarted).toBe(false);
    expect(entry.productionSlaClaimed).toBe(false);
  }
}

function assertManualBoundaries() {
  const statusBackend = entryFor('support-incident-status-backend-redaction-manual-required');

  expect(entryFor('support-incident-backend-upload-manual-required').backendUploadState).toBe('manual-required');
  expect(entryFor('support-incident-backend-upload-manual-required').manualProofRequirements).toEqual([
    'production support backend upload implementation before upload can be claimed',
  ]);
  expect(statusBackend.statusBackendRefs).toEqual(['status-backend-redaction-runbook-manual-required-ref']);
  expect(statusBackend.manualProofRequirements).toEqual([
    'manual status backend redaction review before any status backend payload storage can be claimed',
  ]);
  expect(entryFor('support-incident-billing-escalation-manual-required').billingRefs).toEqual([
    'billing-status-manual-escalation-ref',
  ]);
  expect(entryFor('support-incident-billing-escalation-manual-required').manualProofRequirements).toEqual([
    'billing backend and support escalation runbook before provider contact can be claimed',
  ]);
  expect(entryFor('support-incident-account-lookup-manual-required').accountRefs).toEqual([
    'account-status-manual-lookup-ref',
  ]);
  expect(entryFor('support-incident-account-lookup-manual-required').manualProofRequirements).toEqual([
    'account lookup backend and access audit before account lookup can be claimed',
  ]);
}

function assertInvalidRows() {
  const ready = entryFor('support-incident-bundle-ready');
  const billing = entryFor('support-incident-billing-escalation-manual-required');
  const account = entryFor('support-incident-account-lookup-manual-required');
  const upload = entryFor('support-incident-backend-upload-manual-required');

  for (const invalidEntry of [
    { ...ready, incidentId: 'invalid-token', containsTokens: true },
    { ...ready, incidentId: 'invalid-child-activity', containsChildActivity: true },
    { ...ready, incidentId: 'invalid-raw-url', containsRawUrls: true },
    { ...ready, incidentId: 'invalid-screenshot', containsScreenshots: true },
    { ...ready, incidentId: 'invalid-journal', containsJournals: true },
    { ...ready, incidentId: 'invalid-sqlite', containsSqliteSnapshots: true },
    { ...ready, incidentId: 'invalid-private-path', containsPrivatePaths: true },
    { ...ready, incidentId: 'invalid-command-line', containsCommandLines: true },
    { ...ready, incidentId: 'invalid-keystroke', containsKeystrokes: true },
    { ...ready, incidentId: 'invalid-clipboard', containsClipboardData: true },
    { ...ready, incidentId: 'invalid-message-content', containsMessageContents: true },
    { ...ready, incidentId: 'invalid-status-backend-payload', containsStatusBackendPayload: true },
    { ...ready, incidentId: 'invalid-public-runtime-payload', publicRuntimePayloadIncluded: true },
    { ...ready, incidentId: 'invalid-provider-secret', providerSecretPresent: true },
    { ...ready, incidentId: 'invalid-backend-upload', backendUploadExecuted: true },
    { ...ready, incidentId: 'invalid-status-backend-execution', statusBackendExecutionClaimed: true },
    { ...ready, incidentId: 'invalid-billing-provider-contact', billingProviderContacted: true },
    { ...ready, incidentId: 'invalid-account-lookup', accountLookupExecuted: true },
    { ...ready, incidentId: 'invalid-remote-support', remoteSupportSessionStarted: true },
    { ...ready, incidentId: 'invalid-sla', productionSlaClaimed: true },
    { ...ready, incidentId: 'invalid-ready-no-consent', parentConsentState: 'required' },
    {
      ...ready,
      incidentId: 'invalid-status-backend-redaction-ready-no-ref',
      incidentStatus: 'status-backend-redaction-ready',
      statusBackendRefs: [],
    },
    {
      ...ready,
      incidentId: 'invalid-status-backend-redaction-manual-no-proof',
      incidentStatus: 'status-backend-redaction-manual-required',
      manualProofRequirements: [],
    },
    { ...billing, incidentId: 'invalid-billing-no-ref', billingRefs: [] },
    { ...account, incidentId: 'invalid-account-no-ref', accountRefs: [] },
    { ...upload, incidentId: 'invalid-upload-no-proof', manualProofRequirements: [] },
    {
      ...ready,
      incidentId: 'invalid-duplicate-field',
      redactionSafePayloadFields: ['incident-id-ref', 'incident-id-ref'],
    },
    {
      ...ready,
      incidentId: 'invalid-missing-data-class',
      disclosedDataClasses: ['release-version'],
    },
    {
      ...ready,
      incidentId: 'invalid-missing-diagnostic-kind',
      diagnosticReferenceKinds: ['proof-json-ref'],
    },
  ]) {
    expect(() => SupportBundleRedactionEntrySchema.parse(invalidEntry)).toThrow();
  }
}

function entryFor(incidentId: string) {
  const entry = SupportBundleRedactionReadModel.entries.find((candidate) => candidate.incidentId === incidentId);
  if (entry === undefined) {
    throw new Error(`Missing support bundle redaction entry: ${incidentId}`);
  }
  return entry;
}

function countBy(values: readonly string[]) {
  return values.reduce<Record<string, number>>((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}
