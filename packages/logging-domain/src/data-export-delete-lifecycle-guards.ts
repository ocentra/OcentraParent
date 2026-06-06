import type {
  DataExportDeleteLifecycleDataClass,
  DataExportDeleteLifecycleEntry,
  DataExportDeleteLifecycleEntryCandidate,
  DataExportDeleteLifecycleState,
} from './data-export-delete-lifecycle.js';

export function dataExportDeleteLifecycleEntryIsSafe(
  entry: DataExportDeleteLifecycleEntryCandidate,
  requiredDataClasses: ReadonlyArray<DataExportDeleteLifecycleDataClass>
): boolean {
  return (
    !dataExportDeleteLifecycleHasClaimUpgrade(entry) &&
    requiredValuesArePresent(entry.disclosedDataClasses, requiredDataClasses) &&
    dataExportDeleteLifecycleRefsArePresent(entry) &&
    dataExportDeleteLifecycleStatesAreCoherent(entry)
  );
}

export function dataExportDeleteLifecycleCoversRequiredStates(
  entries: readonly DataExportDeleteLifecycleEntry[]
): boolean {
  const states = new Set(entries.map((entry) => entry.lifecycleState));
  return ['requested', 'authorized', 'queued', 'running', 'succeeded', 'failed', 'manual-required'].every((state) =>
    states.has(state as DataExportDeleteLifecycleState)
  );
}

function dataExportDeleteLifecycleHasClaimUpgrade(entry: DataExportDeleteLifecycleEntryCandidate): boolean {
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
    entry.realBackendUploadExecuted,
    entry.publicRuntimeExecuted,
    entry.providerExecutionOccurred,
    entry.productionSlaClaimed,
    entry.remoteSupportSessionExecuted,
    entry.childActivityCustodyClaimed,
    entry.ocentraHostedFamilyDataDefault,
  ].some(Boolean);
}

function dataExportDeleteLifecycleRefsArePresent(entry: DataExportDeleteLifecycleEntryCandidate): boolean {
  return (
    entry.requestRefs.length > 0 &&
    entry.authorizationRefs.length > 0 &&
    entry.queueRefs.length > 0 &&
    entry.auditRefs.length > 0 &&
    entry.custodyRefs.length > 0
  );
}

function dataExportDeleteLifecycleStatesAreCoherent(entry: DataExportDeleteLifecycleEntryCandidate): boolean {
  return (
    entry.parentInitiationState === 'parent-initiated' &&
    entry.parentAuthorizationState === 'parent-authorized' &&
    entry.payloadState === 'redacted-runtime-status-only' &&
    entry.custodyState === 'parent-owned-local-output-only' &&
    dataExportDeleteLifecycleManualRowsAreCoherent(entry) &&
    dataExportDeleteLifecycleTerminalRowsAreCoherent(entry)
  );
}

function dataExportDeleteLifecycleManualRowsAreCoherent(entry: DataExportDeleteLifecycleEntryCandidate): boolean {
  return entry.lifecycleState !== 'manual-required' || entry.manualProofRequirements.length > 0;
}

function dataExportDeleteLifecycleTerminalRowsAreCoherent(entry: DataExportDeleteLifecycleEntryCandidate): boolean {
  if (!['succeeded', 'failed'].includes(entry.lifecycleState)) {
    return true;
  }
  return entry.runtimeRefs.length > 0 && entry.auditRefs.length > 0;
}

function requiredValuesArePresent<T extends string>(
  actualValues: ReadonlyArray<T>,
  requiredValues: ReadonlyArray<T>
): boolean {
  const actual = new Set(actualValues);
  return actual.size === actualValues.length && requiredValues.every((value) => actual.has(value));
}
