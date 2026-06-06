import type {
  StatusBackendPayloadCustodyDataClass,
  StatusBackendPayloadCustodyEntry,
  StatusBackendPayloadCustodyEntryCandidate,
  StatusBackendPayloadCustodyState,
} from './status-backend-payload-custody.js';

export function statusBackendPayloadCustodyEntryIsSafe(
  entry: StatusBackendPayloadCustodyEntryCandidate,
  requiredDataClasses: ReadonlyArray<StatusBackendPayloadCustodyDataClass>
): boolean {
  return (
    !statusBackendPayloadCustodyHasClaimUpgrade(entry) &&
    requiredValuesArePresent(entry.disclosedDataClasses, requiredDataClasses) &&
    statusBackendPayloadCustodyRefsArePresent(entry) &&
    statusBackendPayloadCustodyStatesAreCoherent(entry)
  );
}

export function statusBackendPayloadCustodyCoversRequiredStates(
  entries: readonly StatusBackendPayloadCustodyEntry[]
): boolean {
  const states = new Set(entries.map((entry) => entry.custodyState));
  return [
    'custody-boundary-recorded',
    'retention-manual-required',
    'delete-request-recorded',
    'deletion-manual-required',
    'audit-export-ready',
    'backend-unavailable',
  ].every((state) => states.has(state as StatusBackendPayloadCustodyState));
}

function statusBackendPayloadCustodyHasClaimUpgrade(entry: StatusBackendPayloadCustodyEntryCandidate): boolean {
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

function statusBackendPayloadCustodyRefsArePresent(entry: StatusBackendPayloadCustodyEntryCandidate): boolean {
  return (
    entry.consentRefs.length > 0 &&
    entry.targetRefs.length > 0 &&
    entry.queueRefs.length > 0 &&
    entry.auditRefs.length > 0 &&
    entry.redactionRefs.length > 0 &&
    entry.custodyRefs.length > 0
  );
}

function statusBackendPayloadCustodyStatesAreCoherent(entry: StatusBackendPayloadCustodyEntryCandidate): boolean {
  return (
    statusBackendPayloadCustodyBoundaryIsCoherent(entry) &&
    statusBackendPayloadCustodyRetentionIsCoherent(entry) &&
    statusBackendPayloadCustodyDeleteIsCoherent(entry) &&
    statusBackendPayloadCustodyExportIsCoherent(entry) &&
    statusBackendPayloadCustodyBackendUnavailableIsCoherent(entry)
  );
}

function statusBackendPayloadCustodyBoundaryIsCoherent(entry: StatusBackendPayloadCustodyEntryCandidate): boolean {
  return (
    entry.parentConsentState === 'parent-approved' &&
    entry.payloadState === 'redacted-status-refs-only' &&
    entry.executionClaimState === 'status-backend-payload-custody-boundary-only'
  );
}

function statusBackendPayloadCustodyRetentionIsCoherent(entry: StatusBackendPayloadCustodyEntryCandidate): boolean {
  if (!['custody-boundary-recorded', 'retention-manual-required'].includes(entry.custodyState)) {
    return true;
  }

  return entry.storageState === 'manual-required' && entry.retentionRefs.length > 0;
}

function statusBackendPayloadCustodyDeleteIsCoherent(entry: StatusBackendPayloadCustodyEntryCandidate): boolean {
  if (!['delete-request-recorded', 'deletion-manual-required'].includes(entry.custodyState)) {
    return true;
  }

  return entry.deleteState === 'manual-required' && entry.deleteRefs.length > 0;
}

function statusBackendPayloadCustodyExportIsCoherent(entry: StatusBackendPayloadCustodyEntryCandidate): boolean {
  return (
    entry.custodyState !== 'audit-export-ready' ||
    (entry.auditExportState === 'support-safe-export-ready' &&
      entry.auditRefs.length > 0 &&
      entry.manualProofRequirements.length > 0)
  );
}

function statusBackendPayloadCustodyBackendUnavailableIsCoherent(
  entry: StatusBackendPayloadCustodyEntryCandidate
): boolean {
  return (
    entry.custodyState !== 'backend-unavailable' ||
    (entry.storageState === 'not-retained' &&
      entry.auditExportState === 'manual-required' &&
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
