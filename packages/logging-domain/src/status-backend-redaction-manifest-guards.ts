import type {
  StatusBackendRedactionManifestDataClass,
  StatusBackendRedactionManifestEntry,
  StatusBackendRedactionManifestEntryCandidate,
  StatusBackendRedactionManifestState,
} from './status-backend-redaction-manifest.js';

export function statusBackendRedactionManifestEntryIsSafe(
  entry: StatusBackendRedactionManifestEntryCandidate,
  requiredDataClasses: ReadonlyArray<StatusBackendRedactionManifestDataClass>
): boolean {
  return (
    !statusBackendRedactionManifestHasClaimUpgrade(entry) &&
    requiredValuesArePresent(entry.disclosedDataClasses, requiredDataClasses) &&
    statusBackendRedactionManifestRefsArePresent(entry) &&
    statusBackendRedactionManifestStatesAreCoherent(entry)
  );
}

export function statusBackendRedactionManifestCoversRequiredStates(
  entries: readonly StatusBackendRedactionManifestEntry[]
): boolean {
  const states = new Set(entries.map((entry) => entry.manifestState));
  return [
    'redaction-manifest-ready',
    'redaction-manifest-manual-required',
    'redaction-review-queued',
    'redaction-review-running',
    'redaction-review-failed',
    'backend-unavailable',
  ].every((state) => states.has(state as StatusBackendRedactionManifestState));
}

function statusBackendRedactionManifestHasClaimUpgrade(entry: StatusBackendRedactionManifestEntryCandidate): boolean {
  return [
    entry.containsTokens,
    entry.containsRawChildActivity,
    entry.containsRawSupportBundles,
    entry.containsProviderSecrets,
    entry.containsAccountLookupResults,
    entry.containsBillingContactRecords,
    entry.containsBackendUploadPayloads,
    entry.containsStatusBackendPayloads,
    entry.containsPublicRuntimePayloads,
    entry.containsRemoteSupportTranscripts,
    entry.realStatusBackendExecution,
    entry.statusBackendPayloadCustodyClaimed,
    entry.durableStatusBackendPayloadStorage,
    entry.statusBackendPayloadDeletionExecuted,
    entry.retryWorkerExecution,
    entry.auditPersistenceExecuted,
    entry.publicRuntimeExecution,
    entry.supportBackendUploadExecution,
    entry.providerExecution,
    entry.accountLookupExecuted,
    entry.billingProviderContactExecuted,
    entry.remoteSupportSessionExecuted,
    entry.productionSlaClaimed,
    entry.ocentraHostedFamilyDataDefault,
  ].some(Boolean);
}

function statusBackendRedactionManifestRefsArePresent(entry: StatusBackendRedactionManifestEntryCandidate): boolean {
  return (
    entry.consentRefs.length > 0 &&
    entry.targetRefs.length > 0 &&
    entry.queueRefs.length > 0 &&
    entry.auditRefs.length > 0 &&
    entry.redactionManifestRefs.length > 0 &&
    entry.manualProofRequirements.length > 0
  );
}

function statusBackendRedactionManifestStatesAreCoherent(entry: StatusBackendRedactionManifestEntryCandidate): boolean {
  return (
    statusBackendRedactionManifestBoundaryIsCoherent(entry) &&
    statusBackendRedactionManifestReadyIsCoherent(entry) &&
    statusBackendRedactionManifestManualIsCoherent(entry) &&
    statusBackendRedactionManifestReviewIsCoherent(entry) &&
    statusBackendRedactionManifestBackendUnavailableIsCoherent(entry)
  );
}

function statusBackendRedactionManifestBoundaryIsCoherent(
  entry: StatusBackendRedactionManifestEntryCandidate
): boolean {
  return (
    entry.parentConsentState === 'parent-approved' &&
    entry.payloadState === 'redacted-status-refs-only' &&
    entry.executionClaimState === 'status-backend-redaction-manifest-boundary-only'
  );
}

function statusBackendRedactionManifestReadyIsCoherent(entry: StatusBackendRedactionManifestEntryCandidate): boolean {
  return (
    entry.manifestState !== 'redaction-manifest-ready' ||
    (entry.redactionManifestState === 'support-safe-manifest-ready' &&
      entry.redactionReviewState === 'reviewed' &&
      entry.redactionSummaryRefs.length > 0)
  );
}

function statusBackendRedactionManifestManualIsCoherent(entry: StatusBackendRedactionManifestEntryCandidate): boolean {
  return (
    entry.manifestState !== 'redaction-manifest-manual-required' ||
    (entry.redactionManifestState === 'manual-required' && entry.manualProofRequirements.length > 0)
  );
}

function statusBackendRedactionManifestReviewIsCoherent(entry: StatusBackendRedactionManifestEntryCandidate): boolean {
  if (
    !['redaction-review-queued', 'redaction-review-running', 'redaction-review-failed'].includes(entry.manifestState)
  ) {
    return true;
  }

  return entry.redactionReviewState !== 'reviewed' && entry.failureRefs.length > 0;
}

function statusBackendRedactionManifestBackendUnavailableIsCoherent(
  entry: StatusBackendRedactionManifestEntryCandidate
): boolean {
  return (
    entry.manifestState !== 'backend-unavailable' ||
    (entry.redactionManifestState === 'manual-required' &&
      entry.redactionReviewState === 'manual-required' &&
      entry.manualProofRequirements.length > 0)
  );
}

function requiredValuesArePresent<T extends string>(
  actualValues: ReadonlyArray<T>,
  requiredValues: ReadonlyArray<T>
): boolean {
  const actual = new Set(actualValues);
  return actual.size === actualValues.length && requiredValues.every((value) => actual.has(value));
}
