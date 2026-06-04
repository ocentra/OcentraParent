import { describe, expect, it } from 'vitest';

import {
  SupportIncidentWorkflowEntrySchema,
  SupportIncidentWorkflowReadModelSchema,
  SupportIncidentWorkflowRequiredDataClasses,
} from '../src/support-incident-workflow';
import { SupportIncidentWorkflowReadModel } from '../src/support-incident-workflow-read-model';

describe('support incident workflow logging contract', () => {
  it(
    'covers parent consent privacy legal redaction upload billing and account workflow states',
    assertWorkflowCoverage
  );
  it('requires privacy legal disclosure redaction audit refs and local custody boundaries', assertDisclosureBoundaries);
  it('keeps backend upload billing escalation and account lookup manual-required only', assertManualBoundaries);
  it('rejects sensitive data custody claim upgrades missing refs and incoherent disclosure states', assertInvalidRows);
});

function assertWorkflowCoverage() {
  const readModel = SupportIncidentWorkflowReadModelSchema.parse(SupportIncidentWorkflowReadModel);

  expect(readModel.readModelId).toBe('support-incident-workflow-proof');
  expect(readModel.entries).toHaveLength(6);
  expect(countBy(readModel.entries.map((entry) => entry.workflowState))).toEqual({
    'parent-consent-gate': 1,
    'privacy-legal-disclosure-required': 1,
    'redaction-audit-review': 1,
    'backend-upload-manual-required': 1,
    'billing-escalation-manual-required': 1,
    'account-lookup-manual-required': 1,
  });
  expect(entryFor('support-workflow-parent-consent-gate').parentConsentState).toBe('required');
  expect(entryFor('support-workflow-privacy-legal-disclosure').parentConsentState).toBe('parent-approved');
}

function assertDisclosureBoundaries() {
  for (const entry of SupportIncidentWorkflowReadModel.entries) {
    expect(entry.disclosedDataClasses).toEqual([...SupportIncidentWorkflowRequiredDataClasses]);
    expect(entry.custodyState).toBe('no-ocentra-child-activity-custody');
    expect(entry.consentRefs).toEqual(['parent-support-consent-artifact-ref']);
    expect(entry.privacyLegalRefs).toEqual(['privacy-disclosure-version-ref', 'legal-disclosure-version-ref']);
    expect(entry.redactionRefs).toEqual(['support-bundle-redaction-proof-ref', 'support-safe-summary-ref']);
    expect(entry.auditRefs).toEqual(['support-incident-audit-event-ref', 'custody-boundary-audit-ref']);
  }

  const disclosure = entryFor('support-workflow-privacy-legal-disclosure');
  expect(disclosure.privacyDisclosureState).toBe('disclosed-before-export');
  expect(disclosure.legalDisclosureState).toBe('disclosed-before-export');
  expect(disclosure.allowedDestinations).toEqual(['parent-local-export', 'support-safe-redaction-summary']);
}

function assertManualBoundaries() {
  const upload = entryFor('support-workflow-backend-upload-manual-required');
  const billing = entryFor('support-workflow-billing-escalation-manual-required');
  const account = entryFor('support-workflow-account-lookup-manual-required');

  expect(upload.backendUploadState).toBe('manual-required');
  expect(upload.backendUploadExecuted).toBe(false);
  expect(upload.allowedDestinations).toEqual(['manual-support-backend']);
  expect(upload.manualProofRequirements).toEqual([
    'production support backend upload implementation before upload can be claimed',
  ]);

  expect(billing.billingEscalationState).toBe('manual-required');
  expect(billing.billingProviderContacted).toBe(false);
  expect(billing.billingRefs).toEqual(['billing-status-manual-escalation-ref']);
  expect(billing.allowedDestinations).toEqual(['manual-billing-provider']);

  expect(account.accountLookupState).toBe('manual-required');
  expect(account.accountLookupExecuted).toBe(false);
  expect(account.accountRefs).toEqual(['account-status-manual-lookup-ref']);
  expect(account.allowedDestinations).toEqual(['manual-account-lookup']);
}

function assertInvalidRows() {
  const disclosure = entryFor('support-workflow-privacy-legal-disclosure');
  const billing = entryFor('support-workflow-billing-escalation-manual-required');
  const account = entryFor('support-workflow-account-lookup-manual-required');
  const upload = entryFor('support-workflow-backend-upload-manual-required');

  for (const invalidEntry of [
    { ...disclosure, incidentId: 'invalid-token', containsTokens: true },
    { ...disclosure, incidentId: 'invalid-child-activity', containsChildActivity: true },
    { ...disclosure, incidentId: 'invalid-raw-url', containsRawUrls: true },
    { ...disclosure, incidentId: 'invalid-screenshot', containsScreenshots: true },
    { ...disclosure, incidentId: 'invalid-journal', containsJournals: true },
    { ...disclosure, incidentId: 'invalid-sqlite', containsSqliteSnapshots: true },
    { ...disclosure, incidentId: 'invalid-private-path', containsPrivatePaths: true },
    { ...disclosure, incidentId: 'invalid-command-line', containsCommandLines: true },
    { ...disclosure, incidentId: 'invalid-keystroke', containsKeystrokes: true },
    { ...disclosure, incidentId: 'invalid-clipboard', containsClipboardData: true },
    { ...disclosure, incidentId: 'invalid-message-content', containsMessageContents: true },
    { ...disclosure, incidentId: 'invalid-provider-secret', providerSecretPresent: true },
    { ...disclosure, incidentId: 'invalid-backend-upload', backendUploadExecuted: true },
    { ...disclosure, incidentId: 'invalid-billing-contact', billingProviderContacted: true },
    { ...disclosure, incidentId: 'invalid-account-lookup', accountLookupExecuted: true },
    { ...disclosure, incidentId: 'invalid-remote-support', remoteSupportSessionStarted: true },
    { ...disclosure, incidentId: 'invalid-sla', productionSlaClaimed: true },
    { ...disclosure, incidentId: 'invalid-hosted-custody', ocentraHostedChildActivityCustody: true },
    { ...disclosure, incidentId: 'invalid-no-privacy', privacyDisclosureState: 'not-shown' },
    { ...disclosure, incidentId: 'invalid-no-legal', legalDisclosureState: 'not-shown' },
    { ...disclosure, incidentId: 'invalid-no-consent-ref', consentRefs: [] },
    { ...disclosure, incidentId: 'invalid-no-privacy-ref', privacyLegalRefs: [] },
    { ...disclosure, incidentId: 'invalid-no-redaction-ref', redactionRefs: [] },
    { ...disclosure, incidentId: 'invalid-no-audit-ref', auditRefs: [] },
    { ...disclosure, incidentId: 'invalid-duplicate-data-class', disclosedDataClasses: ['incident-status'] },
    { ...upload, incidentId: 'invalid-upload-not-manual', backendUploadState: 'not-applicable' },
    { ...billing, incidentId: 'invalid-billing-no-ref', billingRefs: [] },
    { ...account, incidentId: 'invalid-account-no-ref', accountRefs: [] },
  ]) {
    expect(() => SupportIncidentWorkflowEntrySchema.parse(invalidEntry)).toThrow();
  }
}

function entryFor(incidentId: string) {
  const entry = SupportIncidentWorkflowReadModel.entries.find((candidate) => candidate.incidentId === incidentId);
  if (entry === undefined) {
    throw new Error(`Missing support incident workflow entry: ${incidentId}`);
  }
  return entry;
}

function countBy(values: readonly string[]) {
  return values.reduce<Record<string, number>>((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}
