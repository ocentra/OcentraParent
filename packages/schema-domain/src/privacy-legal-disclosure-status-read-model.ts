import {
  type PrivacyLegalDisclosureDestination,
  PrivacyLegalDisclosureEntrySchema,
  type PrivacyLegalDisclosureEntry,
  PrivacyLegalDisclosureReadModelSchema,
  PrivacyLegalDisclosureRequiredDataClasses,
  type PrivacyLegalDisclosureState,
} from './privacy-legal-disclosure-status.js';

type PrivacyLegalDisclosureEntryInput = {
  disclosureId: string;
  disclosureState: PrivacyLegalDisclosureState;
  allowedDestinations: readonly PrivacyLegalDisclosureDestination[];
  disclosureStatusRefs: readonly string[];
  failureRefs: readonly string[];
  manualProofRequirements: readonly string[];
};

const generatedAt = '2026-06-06T09:03:00.000Z';

export const PrivacyLegalDisclosureReadModel = PrivacyLegalDisclosureReadModelSchema.parse({
  schemaVersion: 1,
  readModelId: 'production-support-privacy-legal-disclosure-status-proof',
  generatedAt,
  sourceContractRefs: [
    'production-distribution-support-feature-doc',
    'release-installer-privacy-legal-disclosure-expectation',
    'data-custody-privacy-legal-disclosure-boundary',
    'production-support-publication-execution-status-proof',
    'production-release-public-docs-freshness-proof',
  ],
  entries: [
    privacyLegalDisclosureEntry({
      disclosureId: 'privacy-legal-disclosure-requested',
      disclosureState: 'disclosure-requested',
      allowedDestinations: ['support-safe-disclosure-status-boundary'],
      disclosureStatusRefs: ['privacy-legal-disclosure-request-status-ref'],
      failureRefs: [],
      manualProofRequirements: [],
    }),
    privacyLegalDisclosureEntry({
      disclosureId: 'privacy-legal-parent-authorized',
      disclosureState: 'parent-authorized',
      allowedDestinations: ['support-safe-disclosure-status-boundary'],
      disclosureStatusRefs: ['privacy-legal-parent-authorization-status-ref'],
      failureRefs: [],
      manualProofRequirements: [],
    }),
    privacyLegalDisclosureEntry({
      disclosureId: 'privacy-legal-review-queued',
      disclosureState: 'legal-review-queued',
      allowedDestinations: ['manual-legal-review'],
      disclosureStatusRefs: ['privacy-legal-review-queue-status-ref'],
      failureRefs: [],
      manualProofRequirements: ['legal review queue runner proof required before disclosure execution claim'],
    }),
    privacyLegalDisclosureEntry({
      disclosureId: 'privacy-legal-review-running',
      disclosureState: 'legal-review-running',
      allowedDestinations: ['manual-legal-review'],
      disclosureStatusRefs: ['privacy-legal-review-running-status-ref'],
      failureRefs: [],
      manualProofRequirements: ['legal reviewer workflow proof required before legal execution claim'],
    }),
    privacyLegalDisclosureEntry({
      disclosureId: 'privacy-legal-parent-notification-ready',
      disclosureState: 'parent-notification-ready',
      allowedDestinations: ['support-safe-disclosure-status-boundary'],
      disclosureStatusRefs: ['privacy-legal-parent-notification-status-ref'],
      failureRefs: [],
      manualProofRequirements: [],
    }),
    privacyLegalDisclosureEntry({
      disclosureId: 'privacy-legal-publication-ready',
      disclosureState: 'publication-ready',
      allowedDestinations: ['support-safe-disclosure-status-boundary'],
      disclosureStatusRefs: ['privacy-legal-publication-ready-status-ref'],
      failureRefs: [],
      manualProofRequirements: [],
    }),
    privacyLegalDisclosureEntry({
      disclosureId: 'privacy-legal-disclosure-failed',
      disclosureState: 'disclosure-failed',
      allowedDestinations: ['support-safe-disclosure-status-boundary'],
      disclosureStatusRefs: ['privacy-legal-disclosure-failure-status-ref'],
      failureRefs: ['privacy-legal-disclosure-failure-audit-ref'],
      manualProofRequirements: ['manual remediation proof required before failed disclosure can be retried'],
    }),
    privacyLegalDisclosureEntry({
      disclosureId: 'privacy-legal-manual-required',
      disclosureState: 'manual-required',
      allowedDestinations: ['manual-legal-review'],
      disclosureStatusRefs: ['privacy-legal-manual-required-status-ref'],
      failureRefs: [],
      manualProofRequirements: [
        'published privacy/legal disclosure runner and support runbook proof required before production execution claim',
      ],
    }),
  ],
});

function privacyLegalDisclosureEntry(input: PrivacyLegalDisclosureEntryInput): PrivacyLegalDisclosureEntry {
  return PrivacyLegalDisclosureEntrySchema.parse({
    schemaVersion: 1,
    parentAuthorizationState: 'parent-authorized',
    payloadState: 'support-safe-disclosure-status-only',
    custodyState: 'no-child-activity-custody',
    disclosedDataClasses: [...PrivacyLegalDisclosureRequiredDataClasses],
    parentConsentRefs: ['parent-support-disclosure-consent-ref'],
    privacyPolicyRefs: ['public-privacy-policy-source-contract-ref'],
    legalReviewRefs: ['manual-legal-review-runbook-ref'],
    publicationRefs: ['production-support-publication-execution-status-proof-ref'],
    supportRunbookRefs: ['support-runbook-publication-status-ref'],
    auditRefs: ['privacy-legal-disclosure-audit-ref'],
    containsTokens: false,
    containsRawChildActivity: false,
    containsRawUrls: false,
    containsScreenshots: false,
    containsJournals: false,
    containsSqliteSnapshots: false,
    containsPrivatePaths: false,
    containsCommandLines: false,
    containsKeystrokes: false,
    containsClipboardData: false,
    containsMessageContents: false,
    containsProviderSecrets: false,
    containsRemoteSupportTranscripts: false,
    legalDisclosureExecuted: false,
    publicRuntimeExecuted: false,
    supportBackendUploadExecuted: false,
    accountLookupExecuted: false,
    billingProviderContactExecuted: false,
    remoteSupportSessionExecuted: false,
    productionSlaClaimed: false,
    childActivityCustodyClaimed: false,
    lastCheckedAt: generatedAt,
    ...input,
  });
}
