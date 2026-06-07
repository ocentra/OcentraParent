import type {
  StatusBackendDeleteExecutorDataClass,
  StatusBackendDeleteExecutorEntry,
  StatusBackendDeleteExecutorEntryCandidate,
  StatusBackendDeleteExecutorState,
} from './status-backend-delete-executor.js';

export function statusBackendDeleteExecutorEntryIsSafe(
  entry: StatusBackendDeleteExecutorEntryCandidate,
  requiredDataClasses: ReadonlyArray<StatusBackendDeleteExecutorDataClass>
): boolean {
  return (
    !statusBackendDeleteExecutorHasClaimUpgrade(entry) &&
    requiredValuesArePresent(entry.disclosedDataClasses, requiredDataClasses) &&
    statusBackendDeleteExecutorRefsArePresent(entry) &&
    statusBackendDeleteExecutorStatesAreCoherent(entry)
  );
}

export function statusBackendDeleteExecutorCoversRequiredStates(
  entries: readonly StatusBackendDeleteExecutorEntry[]
): boolean {
  const states = new Set(entries.map((entry) => entry.deleteExecutorState));
  return [
    'delete-request-recorded',
    'delete-executor-authorized',
    'delete-executor-queued',
    'delete-executor-running',
    'deletion-manual-required',
    'delete-executor-failed',
    'audit-export-ready',
    'backend-unavailable',
  ].every((state) => states.has(state as StatusBackendDeleteExecutorState));
}

function statusBackendDeleteExecutorHasClaimUpgrade(entry: StatusBackendDeleteExecutorEntryCandidate): boolean {
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
    entry.statusBackendDeleteExecutorExecuted,
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

function statusBackendDeleteExecutorRefsArePresent(entry: StatusBackendDeleteExecutorEntryCandidate): boolean {
  return (
    entry.consentRefs.length > 0 &&
    entry.targetRefs.length > 0 &&
    entry.queueRefs.length > 0 &&
    entry.auditRefs.length > 0 &&
    entry.redactionRefs.length > 0 &&
    entry.custodyRefs.length > 0 &&
    entry.deleteRefs.length > 0 &&
    entry.executorRefs.length > 0
  );
}

function statusBackendDeleteExecutorStatesAreCoherent(entry: StatusBackendDeleteExecutorEntryCandidate): boolean {
  return (
    statusBackendDeleteExecutorBoundaryIsCoherent(entry) &&
    statusBackendDeleteExecutorQueueIsCoherent(entry) &&
    statusBackendDeleteExecutorManualDeletionIsCoherent(entry) &&
    statusBackendDeleteExecutorFailureIsCoherent(entry) &&
    statusBackendDeleteExecutorExportIsCoherent(entry) &&
    statusBackendDeleteExecutorUnavailableIsCoherent(entry)
  );
}

function statusBackendDeleteExecutorBoundaryIsCoherent(entry: StatusBackendDeleteExecutorEntryCandidate): boolean {
  return (
    entry.parentConsentState === 'parent-approved' &&
    entry.payloadState === 'redacted-delete-status-refs-only' &&
    entry.executionClaimState === 'status-backend-delete-executor-boundary-only'
  );
}

function statusBackendDeleteExecutorQueueIsCoherent(entry: StatusBackendDeleteExecutorEntryCandidate): boolean {
  if (!['delete-executor-queued', 'delete-executor-running'].includes(entry.deleteExecutorState)) {
    return true;
  }

  return entry.executorExecutionState === 'manual-required' && entry.queueRefs.length > 0;
}

function statusBackendDeleteExecutorManualDeletionIsCoherent(
  entry: StatusBackendDeleteExecutorEntryCandidate
): boolean {
  if (entry.deleteExecutorState !== 'deletion-manual-required') {
    return true;
  }

  return (
    entry.executorExecutionState === 'manual-required' &&
    entry.payloadDeletionState === 'manual-required' &&
    entry.deleteRefs.length > 0 &&
    entry.manualProofRequirements.length > 0
  );
}

function statusBackendDeleteExecutorFailureIsCoherent(entry: StatusBackendDeleteExecutorEntryCandidate): boolean {
  return (
    entry.deleteExecutorState !== 'delete-executor-failed' ||
    (entry.executorExecutionState === 'manual-required' &&
      entry.failureRefs.length > 0 &&
      entry.manualProofRequirements.length > 0)
  );
}

function statusBackendDeleteExecutorExportIsCoherent(entry: StatusBackendDeleteExecutorEntryCandidate): boolean {
  return (
    entry.deleteExecutorState !== 'audit-export-ready' ||
    (entry.auditExportState === 'support-safe-export-ready' &&
      entry.auditRefs.length > 0 &&
      entry.manualProofRequirements.length > 0)
  );
}

function statusBackendDeleteExecutorUnavailableIsCoherent(entry: StatusBackendDeleteExecutorEntryCandidate): boolean {
  return (
    entry.deleteExecutorState !== 'backend-unavailable' ||
    (entry.executorExecutionState === 'not-executed' &&
      entry.payloadDeletionState === 'not-requested' &&
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
