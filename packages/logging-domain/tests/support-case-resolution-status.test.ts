import { describe, expect, it } from 'vitest';

import {
  SupportCaseResolutionRequiredDataClasses,
  SupportCaseResolutionStatusEntrySchema,
  SupportCaseResolutionStatusReadModelSchema,
} from '../src/support-case-resolution-status';
import { SupportCaseResolutionStatusReadModel } from '../src/support-case-resolution-status-read-model';

describe('support case resolution status logging contract', () => {
  it('covers opened triage update escalation response closure and SLA states', assertCoverage);
  it('requires parent consent redaction audit publication refs and support-safe payloads', assertSafeBoundary);
  it(
    'proves manual escalation response closure and SLA behavior without support execution claims',
    assertStatusBehavior
  );
  it('rejects sensitive custody claims missing refs duplicate data classes and incoherent states', assertInvalidRows);
});

function assertCoverage() {
  const readModel = SupportCaseResolutionStatusReadModelSchema.parse(SupportCaseResolutionStatusReadModel);

  expect(readModel.readModelId).toBe('production-support-case-resolution-status-proof');
  expect(readModel.entries).toHaveLength(7);
  expect(countBy(readModel.entries.map((entry) => entry.caseStatus))).toEqual({
    'case-opened': 1,
    'triage-ready': 1,
    'parent-update-ready': 1,
    'escalation-manual-required': 1,
    'response-manual-required': 1,
    'closure-ready': 1,
    'sla-manual-required': 1,
  });
}

function assertSafeBoundary() {
  for (const entry of SupportCaseResolutionStatusReadModel.entries) {
    expect(entry.parentInitiationState).toBe('parent-initiated');
    expect(entry.parentConsentState).toBe('parent-approved');
    expect(entry.casePayloadState).toBe('support-safe-status-and-refs-only');
    expect(entry.custodyState).toBe('no-ocentra-hosted-family-data');
    expect(entry.disclosedDataClasses).toEqual([...SupportCaseResolutionRequiredDataClasses]);
    expect(entry.parentConsentRefs).toEqual(['parent-support-case-consent-artifact-ref']);
    expect(entry.incidentRefs).toEqual([
      'support-incident-workflow-proof-ref',
      'support-incident-status-publication-ref',
    ]);
    expect(entry.redactionRefs).toEqual(['support-bundle-redaction-proof-ref', 'support-safe-case-summary-ref']);
    expect(entry.auditRefs).toEqual(['support-case-status-audit-ref', 'support-case-resolution-audit-ref']);
    expect(entry.publicationRefs).toEqual(['production-support-publication-workflow-proof-ref']);
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

function assertStatusBehavior() {
  const opened = entryFor('support-case-opened');
  const triage = entryFor('support-case-triage-ready');
  const update = entryFor('support-case-parent-update-ready');
  const escalation = entryFor('support-case-escalation-manual-required');
  const response = entryFor('support-case-response-manual-required');
  const closure = entryFor('support-case-closure-ready');
  const sla = entryFor('support-case-sla-manual-required');

  for (const entry of [opened, triage, update]) {
    expect(entry.operatorResponseState).toBe('manual-required');
    expect(entry.escalationState).toBe('not-requested');
    expect(entry.allowedDestinations).toEqual(['support-safe-case-status-boundary']);
    expect(entry.responseRefs.length).toBe(1);
  }

  expect(escalation.escalationState).toBe('manual-required');
  expect(escalation.escalationRefs).toEqual([
    'support-case-escalation-runbook-ref',
    'support-case-provider-contact-manual-ref',
  ]);
  expect(escalation.manualProofRequirements).toEqual([
    'support escalation operator workflow before escalation execution can be claimed',
  ]);

  expect(response.responseRefs).toEqual(['support-case-operator-response-runbook-ref']);
  expect(response.manualProofRequirements).toEqual([
    'support operator response workflow before parent response execution can be claimed',
  ]);

  expect(closure.closureRefs).toEqual(['support-case-closure-audit-ref', 'support-case-parent-closeout-ref']);
  expect(closure.allowedDestinations).toEqual(['support-safe-case-status-boundary']);

  expect(sla.slaState).toBe('manual-required');
  expect(sla.slaRefs).toEqual(['support-case-sla-policy-manual-ref', 'support-case-sla-publication-manual-ref']);
  expect(sla.manualProofRequirements).toEqual([
    'published production support SLA before support timing commitments can be claimed',
  ]);
}

function assertInvalidRows() {
  const opened = entryFor('support-case-opened');
  const escalation = entryFor('support-case-escalation-manual-required');
  const response = entryFor('support-case-response-manual-required');
  const closure = entryFor('support-case-closure-ready');
  const sla = entryFor('support-case-sla-manual-required');

  for (const invalidEntry of [
    { ...opened, caseId: 'invalid-token', containsTokens: true },
    { ...opened, caseId: 'invalid-child-activity', containsRawChildActivity: true },
    { ...opened, caseId: 'invalid-raw-url', containsRawUrls: true },
    { ...opened, caseId: 'invalid-screenshot', containsScreenshots: true },
    { ...opened, caseId: 'invalid-journal', containsJournals: true },
    { ...opened, caseId: 'invalid-sqlite', containsSqliteSnapshots: true },
    { ...opened, caseId: 'invalid-private-path', containsPrivatePaths: true },
    { ...opened, caseId: 'invalid-command-line', containsCommandLines: true },
    { ...opened, caseId: 'invalid-keystroke', containsKeystrokes: true },
    { ...opened, caseId: 'invalid-clipboard', containsClipboardData: true },
    { ...opened, caseId: 'invalid-message-content', containsMessageContents: true },
    { ...opened, caseId: 'invalid-provider-secret', containsProviderSecrets: true },
    { ...opened, caseId: 'invalid-remote-transcript', containsRemoteSupportTranscripts: true },
    { ...opened, caseId: 'invalid-real-backend-execution', realSupportBackendUploadExecuted: true },
    { ...opened, caseId: 'invalid-account-lookup', accountLookupExecuted: true },
    { ...opened, caseId: 'invalid-billing-contact', billingProviderContactExecuted: true },
    { ...opened, caseId: 'invalid-remote-support', remoteSupportSessionExecuted: true },
    { ...opened, caseId: 'invalid-sla', productionSlaClaimed: true },
    { ...opened, caseId: 'invalid-hosted-family-data', ocentraHostedFamilyDataDefault: true },
    { ...opened, caseId: 'invalid-no-consent', parentConsentState: 'required' },
    { ...opened, caseId: 'invalid-no-consent-ref', parentConsentRefs: [] },
    { ...opened, caseId: 'invalid-no-incident-ref', incidentRefs: [] },
    { ...opened, caseId: 'invalid-no-redaction-ref', redactionRefs: [] },
    { ...opened, caseId: 'invalid-no-audit-ref', auditRefs: [] },
    { ...opened, caseId: 'invalid-no-publication-ref', publicationRefs: [] },
    { ...opened, caseId: 'invalid-duplicate-data-class', disclosedDataClasses: ['case-status'] },
    { ...escalation, caseId: 'invalid-escalation-no-ref', escalationRefs: [] },
    { ...escalation, caseId: 'invalid-escalation-not-manual', escalationState: 'not-requested' },
    { ...response, caseId: 'invalid-response-no-ref', responseRefs: [] },
    { ...response, caseId: 'invalid-response-no-proof', manualProofRequirements: [] },
    { ...closure, caseId: 'invalid-closure-no-ref', closureRefs: [] },
    { ...sla, caseId: 'invalid-sla-not-manual', slaState: 'not-claimed' },
    { ...sla, caseId: 'invalid-sla-no-ref', slaRefs: [] },
  ]) {
    expect(() => SupportCaseResolutionStatusEntrySchema.parse(invalidEntry)).toThrow();
  }
}

function entryFor(caseId: string) {
  const entry = SupportCaseResolutionStatusReadModel.entries.find((candidate) => candidate.caseId === caseId);
  if (entry === undefined) {
    throw new Error(`Missing support case resolution status entry: ${caseId}`);
  }
  return entry;
}

function countBy(values: readonly string[]) {
  return values.reduce<Record<string, number>>((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}
