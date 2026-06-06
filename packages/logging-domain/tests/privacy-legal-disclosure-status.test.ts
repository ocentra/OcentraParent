import { describe, expect, it } from 'vitest';

import {
  PrivacyLegalDisclosureEntrySchema,
  PrivacyLegalDisclosureReadModelSchema,
  PrivacyLegalDisclosureRequiredDataClasses,
} from '../src/privacy-legal-disclosure-status';
import { PrivacyLegalDisclosureReadModel } from '../src/privacy-legal-disclosure-status-read-model';

describe('privacy legal disclosure status logging contract', () => {
  it('covers privacy legal disclosure lifecycle states', () => {
    const readModel = PrivacyLegalDisclosureReadModelSchema.parse(PrivacyLegalDisclosureReadModel);

    expect(readModel.readModelId).toBe('production-support-privacy-legal-disclosure-status-proof');
    expect(readModel.entries.map((entry) => entry.disclosureState)).toEqual([
      'disclosure-requested',
      'parent-authorized',
      'legal-review-queued',
      'legal-review-running',
      'parent-notification-ready',
      'publication-ready',
      'disclosure-failed',
      'manual-required',
    ]);
  });

  it('requires parent authorization legal review publication audit and support-safe payload refs', () => {
    for (const entry of PrivacyLegalDisclosureReadModel.entries) {
      expectSupportSafeDisclosureEntry(entry);
    }
  });

  it('rejects sensitive data and production execution overclaims', () => {
    const ready = entryFor('privacy-legal-publication-ready');

    for (const invalidEntry of invalidDisclosureEntries(ready)) {
      expect(() => PrivacyLegalDisclosureEntrySchema.parse(invalidEntry)).toThrow();
    }
  });
});

function expectSupportSafeDisclosureEntry(entry: (typeof PrivacyLegalDisclosureReadModel.entries)[number]) {
  expect(entry.parentAuthorizationState).toBe('parent-authorized');
  expect(entry.payloadState).toBe('support-safe-disclosure-status-only');
  expect(entry.custodyState).toBe('no-child-activity-custody');
  expect(entry.disclosedDataClasses).toEqual([...PrivacyLegalDisclosureRequiredDataClasses]);
  expect(entry.parentConsentRefs).toEqual(['parent-support-disclosure-consent-ref']);
  expect(entry.privacyPolicyRefs).toHaveLength(1);
  expect(entry.legalReviewRefs).toHaveLength(1);
  expect(entry.publicationRefs).toHaveLength(1);
  expect(entry.auditRefs).toHaveLength(1);
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
  expect(entry.legalDisclosureExecuted).toBe(false);
  expect(entry.publicRuntimeExecuted).toBe(false);
  expect(entry.supportBackendUploadExecuted).toBe(false);
  expect(entry.accountLookupExecuted).toBe(false);
  expect(entry.billingProviderContactExecuted).toBe(false);
  expect(entry.remoteSupportSessionExecuted).toBe(false);
  expect(entry.productionSlaClaimed).toBe(false);
  expect(entry.childActivityCustodyClaimed).toBe(false);
}

function invalidDisclosureEntries(ready: ReturnType<typeof entryFor>) {
  return [
    { ...ready, disclosureId: 'invalid-token', containsTokens: true },
    { ...ready, disclosureId: 'invalid-child-activity', containsRawChildActivity: true },
    { ...ready, disclosureId: 'invalid-raw-url', containsRawUrls: true },
    { ...ready, disclosureId: 'invalid-screenshot', containsScreenshots: true },
    { ...ready, disclosureId: 'invalid-journal', containsJournals: true },
    { ...ready, disclosureId: 'invalid-sqlite', containsSqliteSnapshots: true },
    { ...ready, disclosureId: 'invalid-private-path', containsPrivatePaths: true },
    { ...ready, disclosureId: 'invalid-command-line', containsCommandLines: true },
    { ...ready, disclosureId: 'invalid-keystroke', containsKeystrokes: true },
    { ...ready, disclosureId: 'invalid-clipboard', containsClipboardData: true },
    { ...ready, disclosureId: 'invalid-message-content', containsMessageContents: true },
    { ...ready, disclosureId: 'invalid-provider-secret', containsProviderSecrets: true },
    { ...ready, disclosureId: 'invalid-transcript', containsRemoteSupportTranscripts: true },
    { ...ready, disclosureId: 'invalid-legal-execution', legalDisclosureExecuted: true },
    { ...ready, disclosureId: 'invalid-public-runtime', publicRuntimeExecuted: true },
    { ...ready, disclosureId: 'invalid-support-upload', supportBackendUploadExecuted: true },
    { ...ready, disclosureId: 'invalid-account-lookup', accountLookupExecuted: true },
    { ...ready, disclosureId: 'invalid-billing-provider', billingProviderContactExecuted: true },
    { ...ready, disclosureId: 'invalid-remote-support', remoteSupportSessionExecuted: true },
    { ...ready, disclosureId: 'invalid-sla', productionSlaClaimed: true },
    { ...ready, disclosureId: 'invalid-custody', childActivityCustodyClaimed: true },
    { ...ready, disclosureId: 'invalid-no-legal-ref', legalReviewRefs: [] },
    { ...ready, disclosureId: 'invalid-duplicate-class', disclosedDataClasses: ['manual-proof-ref'] },
  ];
}

function entryFor(disclosureId: string) {
  const entry = PrivacyLegalDisclosureReadModel.entries.find((candidate) => candidate.disclosureId === disclosureId);
  if (entry === undefined) {
    throw new Error(`Missing privacy/legal disclosure entry: ${disclosureId}`);
  }
  return entry;
}
