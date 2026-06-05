import type {
  SupportBackendUploadDataClass,
  SupportBackendUploadStatusEntry,
  SupportBackendUploadStatusEntryCandidate,
  SupportBackendUploadStatusState,
} from './support-backend-upload-status.js';

export function supportBackendUploadStatusEntryIsSafe(
  entry: SupportBackendUploadStatusEntryCandidate,
  requiredDataClasses: ReadonlyArray<SupportBackendUploadDataClass>
): boolean {
  return (
    !supportBackendUploadStatusHasClaimUpgrade(entry) &&
    requiredValuesArePresent(entry.disclosedDataClasses, requiredDataClasses) &&
    supportBackendUploadStatusRefsArePresent(entry) &&
    supportBackendUploadStatusStatesAreCoherent(entry)
  );
}

export function supportBackendUploadStatusCoversRequiredStates(
  entries: readonly SupportBackendUploadStatusEntry[]
): boolean {
  const states = new Set(entries.map((entry) => entry.uploadStatus));
  return [
    'upload-queued',
    'upload-running',
    'upload-succeeded',
    'upload-failed',
    'upload-manual-required',
    'backend-unavailable',
    'provider-unavailable',
  ].every((state) => states.has(state as SupportBackendUploadStatusState));
}

function supportBackendUploadStatusHasClaimUpgrade(entry: SupportBackendUploadStatusEntryCandidate): boolean {
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
    entry.realSupportBackendUploadExecuted,
    entry.accountLookupExecuted,
    entry.billingProviderExecuted,
    entry.ocentraHostedFamilyDataDefault,
  ].some(Boolean);
}

function supportBackendUploadStatusRefsArePresent(entry: SupportBackendUploadStatusEntryCandidate): boolean {
  return (
    entry.consentRefs.length > 0 &&
    entry.redactionRefs.length > 0 &&
    entry.auditRefs.length > 0 &&
    entry.backendRefs.length > 0 &&
    entry.providerRefs.length > 0
  );
}

function supportBackendUploadStatusStatesAreCoherent(entry: SupportBackendUploadStatusEntryCandidate): boolean {
  return (
    supportBackendUploadConsentStateIsCoherent(entry) &&
    supportBackendUploadAvailableStateIsCoherent(entry) &&
    supportBackendUploadFailedStateIsCoherent(entry) &&
    supportBackendUploadManualStateIsCoherent(entry) &&
    supportBackendUploadBackendUnavailableStateIsCoherent(entry) &&
    supportBackendUploadProviderUnavailableStateIsCoherent(entry)
  );
}

function supportBackendUploadConsentStateIsCoherent(entry: SupportBackendUploadStatusEntryCandidate): boolean {
  return (
    entry.parentInitiationState === 'parent-initiated' &&
    entry.parentConsentState === 'parent-approved' &&
    entry.executionClaimState === 'status-boundary-only' &&
    entry.payloadState === 'redacted-status-and-audit-refs-only' &&
    entry.custodyState === 'no-ocentra-hosted-family-data'
  );
}

function supportBackendUploadAvailableStateIsCoherent(entry: SupportBackendUploadStatusEntryCandidate): boolean {
  if (!['upload-queued', 'upload-running', 'upload-succeeded'].includes(entry.uploadStatus)) {
    return true;
  }

  return (
    entry.backendAvailabilityState === 'available' &&
    entry.providerAvailabilityState === 'available' &&
    entry.retryState === 'not-needed' &&
    entry.abandonState === 'not-requested' &&
    entry.allowedDestinations.includes('support-safe-upload-status-boundary')
  );
}

function supportBackendUploadFailedStateIsCoherent(entry: SupportBackendUploadStatusEntryCandidate): boolean {
  return (
    entry.uploadStatus !== 'upload-failed' ||
    (entry.backendAvailabilityState === 'available' &&
      entry.providerAvailabilityState === 'available' &&
      entry.retryState === 'retry-exhausted' &&
      entry.abandonState === 'abandoned' &&
      entry.retryRefs.length > 0 &&
      entry.abandonRefs.length > 0 &&
      entry.failureRefs.length > 0)
  );
}

function supportBackendUploadManualStateIsCoherent(entry: SupportBackendUploadStatusEntryCandidate): boolean {
  return (
    entry.uploadStatus !== 'upload-manual-required' ||
    (entry.backendAvailabilityState === 'manual-required' &&
      entry.providerAvailabilityState === 'manual-required' &&
      entry.retryState === 'manual-required' &&
      entry.abandonState === 'not-applicable' &&
      entry.manualProofRequirements.length > 0 &&
      entry.allowedDestinations.includes('manual-support-backend'))
  );
}

function supportBackendUploadBackendUnavailableStateIsCoherent(
  entry: SupportBackendUploadStatusEntryCandidate
): boolean {
  return (
    entry.uploadStatus !== 'backend-unavailable' ||
    (entry.backendAvailabilityState === 'unavailable' &&
      entry.providerAvailabilityState === 'available' &&
      entry.retryState === 'retry-queued' &&
      entry.abandonState === 'not-requested' &&
      entry.retryRefs.length > 0 &&
      entry.failureRefs.length > 0)
  );
}

function supportBackendUploadProviderUnavailableStateIsCoherent(
  entry: SupportBackendUploadStatusEntryCandidate
): boolean {
  return (
    entry.uploadStatus !== 'provider-unavailable' ||
    (entry.backendAvailabilityState === 'available' &&
      entry.providerAvailabilityState === 'unavailable' &&
      entry.retryState === 'retry-queued' &&
      entry.abandonState === 'not-requested' &&
      entry.retryRefs.length > 0 &&
      entry.failureRefs.length > 0)
  );
}

function requiredValuesArePresent<T extends string>(
  actualValues: ReadonlyArray<T>,
  requiredValues: ReadonlyArray<T>
): boolean {
  const actual = new Set(actualValues);
  return actual.size === actualValues.length && requiredValues.every((value) => actual.has(value));
}
