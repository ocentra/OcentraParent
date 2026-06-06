import type {
  PrivacyLegalDisclosureDataClass,
  PrivacyLegalDisclosureEntry,
  PrivacyLegalDisclosureEntryCandidate,
  PrivacyLegalDisclosureState,
} from './privacy-legal-disclosure-status.js';

export function privacyLegalDisclosureEntryIsSafe(
  entry: PrivacyLegalDisclosureEntryCandidate,
  requiredDataClasses: ReadonlyArray<PrivacyLegalDisclosureDataClass>
): boolean {
  return (
    !privacyLegalDisclosureHasClaimUpgrade(entry) &&
    requiredValuesArePresent(entry.disclosedDataClasses, requiredDataClasses) &&
    privacyLegalDisclosureRefsArePresent(entry) &&
    privacyLegalDisclosureStatesAreCoherent(entry)
  );
}

export function privacyLegalDisclosureCoversRequiredStates(entries: readonly PrivacyLegalDisclosureEntry[]): boolean {
  const states = new Set(entries.map((entry) => entry.disclosureState));
  return [
    'disclosure-requested',
    'parent-authorized',
    'legal-review-queued',
    'legal-review-running',
    'parent-notification-ready',
    'publication-ready',
    'disclosure-failed',
    'manual-required',
  ].every((state) => states.has(state as PrivacyLegalDisclosureState));
}

function privacyLegalDisclosureHasClaimUpgrade(entry: PrivacyLegalDisclosureEntryCandidate): boolean {
  return [
    entry.containsTokens,
    entry.containsRawChildActivity,
    entry.containsRawUrls,
    entry.containsScreenshots,
    entry.containsJournals,
    entry.containsSqliteSnapshots,
    entry.containsPrivatePaths,
    entry.containsCommandLines,
    entry.containsKeystrokes,
    entry.containsClipboardData,
    entry.containsMessageContents,
    entry.containsProviderSecrets,
    entry.containsRemoteSupportTranscripts,
    entry.legalDisclosureExecuted,
    entry.publicRuntimeExecuted,
    entry.supportBackendUploadExecuted,
    entry.accountLookupExecuted,
    entry.billingProviderContactExecuted,
    entry.remoteSupportSessionExecuted,
    entry.productionSlaClaimed,
    entry.childActivityCustodyClaimed,
  ].some(Boolean);
}

function privacyLegalDisclosureRefsArePresent(entry: PrivacyLegalDisclosureEntryCandidate): boolean {
  return (
    entry.parentConsentRefs.length > 0 &&
    entry.privacyPolicyRefs.length > 0 &&
    entry.legalReviewRefs.length > 0 &&
    entry.auditRefs.length > 0 &&
    entry.publicationRefs.length > 0
  );
}

function privacyLegalDisclosureStatesAreCoherent(entry: PrivacyLegalDisclosureEntryCandidate): boolean {
  return (
    entry.parentAuthorizationState === 'parent-authorized' &&
    entry.payloadState === 'support-safe-disclosure-status-only' &&
    entry.custodyState === 'no-child-activity-custody' &&
    privacyLegalDisclosureManualRowsAreCoherent(entry) &&
    privacyLegalDisclosureFailureRowsAreCoherent(entry)
  );
}

function privacyLegalDisclosureManualRowsAreCoherent(entry: PrivacyLegalDisclosureEntryCandidate): boolean {
  return entry.disclosureState !== 'manual-required' || entry.manualProofRequirements.length > 0;
}

function privacyLegalDisclosureFailureRowsAreCoherent(entry: PrivacyLegalDisclosureEntryCandidate): boolean {
  return entry.disclosureState !== 'disclosure-failed' || entry.failureRefs.length > 0;
}

function requiredValuesArePresent<T extends string>(
  actualValues: ReadonlyArray<T>,
  requiredValues: ReadonlyArray<T>
): boolean {
  const actual = new Set(actualValues);
  return actual.size === actualValues.length && requiredValues.every((value) => actual.has(value));
}
