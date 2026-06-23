import type {
  SupportBackendUploadExecutionRuntimeDataClass,
  SupportBackendUploadExecutionRuntimeEntry,
  SupportBackendUploadExecutionRuntimeEntryCandidate,
  SupportBackendUploadExecutionRuntimeState,
} from './support-backend-upload-execution-runtime.js';
import { supportProofHasAnyClaimUpgrade, supportProofRequiredValuesArePresent } from './support-proof-contract.js';

export function supportBackendUploadExecutionRuntimeEntryIsSafe(
  entry: SupportBackendUploadExecutionRuntimeEntryCandidate,
  requiredDataClasses: ReadonlyArray<SupportBackendUploadExecutionRuntimeDataClass>
): boolean {
  return (
    !supportBackendUploadExecutionRuntimeHasClaimUpgrade(entry) &&
    supportProofRequiredValuesArePresent(entry.disclosedDataClasses, requiredDataClasses) &&
    supportBackendUploadExecutionRuntimeRefsArePresent(entry) &&
    supportBackendUploadExecutionRuntimeStatesAreCoherent(entry)
  );
}

export function supportBackendUploadExecutionRuntimeCoversRequiredStates(
  entries: readonly SupportBackendUploadExecutionRuntimeEntry[]
): boolean {
  const states = new Set(entries.map((entry) => entry.runtimeState));
  return [
    'execution-request-recorded',
    'redaction-preflight-ready',
    'dispatch-manual-required',
    'backend-unavailable',
    'provider-unavailable',
    'retry-scheduled',
    'operator-abandoned',
  ].every((state) => states.has(state as SupportBackendUploadExecutionRuntimeState));
}

function supportBackendUploadExecutionRuntimeHasClaimUpgrade(
  entry: SupportBackendUploadExecutionRuntimeEntryCandidate
): boolean {
  return supportProofHasAnyClaimUpgrade([
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
    entry.billingProviderContactExecuted,
    entry.remoteSupportSessionExecuted,
    entry.productionSlaClaimed,
    entry.ocentraHostedFamilyDataDefault,
  ]);
}

function supportBackendUploadExecutionRuntimeRefsArePresent(
  entry: SupportBackendUploadExecutionRuntimeEntryCandidate
): boolean {
  return (
    entry.consentRefs.length > 0 &&
    entry.redactionRefs.length > 0 &&
    entry.auditRefs.length > 0 &&
    entry.statusRefs.length > 0 &&
    entry.runtimeRefs.length > 0
  );
}

function supportBackendUploadExecutionRuntimeStatesAreCoherent(
  entry: SupportBackendUploadExecutionRuntimeEntryCandidate
): boolean {
  return (
    supportBackendUploadExecutionRuntimeBoundaryIsCoherent(entry) &&
    supportBackendUploadExecutionRuntimeRequestIsCoherent(entry) &&
    supportBackendUploadExecutionRuntimeManualIsCoherent(entry) &&
    supportBackendUploadExecutionRuntimeUnavailableIsCoherent(entry) &&
    supportBackendUploadExecutionRuntimeRetryIsCoherent(entry) &&
    supportBackendUploadExecutionRuntimeAbandonIsCoherent(entry)
  );
}

function supportBackendUploadExecutionRuntimeBoundaryIsCoherent(
  entry: SupportBackendUploadExecutionRuntimeEntryCandidate
): boolean {
  return (
    entry.parentInitiationState === 'parent-initiated' &&
    entry.parentConsentState === 'parent-approved' &&
    entry.executionClaimState === 'runtime-boundary-only' &&
    entry.payloadState === 'redacted-runtime-refs-only' &&
    entry.custodyState === 'no-ocentra-hosted-family-data'
  );
}

function supportBackendUploadExecutionRuntimeRequestIsCoherent(
  entry: SupportBackendUploadExecutionRuntimeEntryCandidate
): boolean {
  if (!['execution-request-recorded', 'redaction-preflight-ready'].includes(entry.runtimeState)) {
    return true;
  }

  return (
    entry.backendAvailabilityState === 'available' &&
    entry.providerAvailabilityState === 'available' &&
    entry.retryState === 'not-needed' &&
    entry.abandonState === 'not-requested' &&
    entry.manualProofRequirements.length === 0
  );
}

function supportBackendUploadExecutionRuntimeManualIsCoherent(
  entry: SupportBackendUploadExecutionRuntimeEntryCandidate
): boolean {
  return (
    entry.runtimeState !== 'dispatch-manual-required' ||
    (entry.backendAvailabilityState === 'manual-required' &&
      entry.providerAvailabilityState === 'manual-required' &&
      entry.retryState === 'manual-required' &&
      entry.abandonState === 'not-applicable' &&
      entry.manualProofRequirements.length > 0)
  );
}

function supportBackendUploadExecutionRuntimeUnavailableIsCoherent(
  entry: SupportBackendUploadExecutionRuntimeEntryCandidate
): boolean {
  if (!['backend-unavailable', 'provider-unavailable'].includes(entry.runtimeState)) {
    return true;
  }

  return entry.failureRefs.length > 0 && entry.retryRefs.length > 0 && entry.retryState === 'retry-scheduled';
}

function supportBackendUploadExecutionRuntimeRetryIsCoherent(
  entry: SupportBackendUploadExecutionRuntimeEntryCandidate
): boolean {
  return (
    entry.runtimeState !== 'retry-scheduled' ||
    (entry.retryState === 'retry-scheduled' && entry.retryRefs.length > 0 && entry.abandonState === 'not-requested')
  );
}

function supportBackendUploadExecutionRuntimeAbandonIsCoherent(
  entry: SupportBackendUploadExecutionRuntimeEntryCandidate
): boolean {
  return (
    entry.runtimeState !== 'operator-abandoned' ||
    (entry.retryState === 'retry-exhausted' && entry.abandonState === 'abandoned' && entry.abandonRefs.length > 0)
  );
}
