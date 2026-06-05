import type {
  SupportBackendUploadCustodyAuditDataClass,
  SupportBackendUploadCustodyAuditEntry,
  SupportBackendUploadCustodyAuditEntryCandidate,
  SupportBackendUploadCustodyAuditState,
} from './support-backend-upload-custody-audit.js';

export function supportBackendUploadCustodyAuditEntryIsSafe(
  entry: SupportBackendUploadCustodyAuditEntryCandidate,
  requiredDataClasses: ReadonlyArray<SupportBackendUploadCustodyAuditDataClass>
): boolean {
  return (
    !supportBackendUploadCustodyAuditHasClaimUpgrade(entry) &&
    requiredValuesArePresent(entry.disclosedDataClasses, requiredDataClasses) &&
    supportBackendUploadCustodyAuditRefsArePresent(entry) &&
    supportBackendUploadCustodyAuditStatesAreCoherent(entry)
  );
}

export function supportBackendUploadCustodyAuditCoversRequiredStates(
  entries: readonly SupportBackendUploadCustodyAuditEntry[]
): boolean {
  const states = new Set(entries.map((entry) => entry.auditState));
  return [
    'custody-boundary-recorded',
    'retention-manual-required',
    'delete-request-recorded',
    'deletion-manual-required',
    'audit-export-ready',
  ].every((state) => states.has(state as SupportBackendUploadCustodyAuditState));
}

function supportBackendUploadCustodyAuditHasClaimUpgrade(
  entry: SupportBackendUploadCustodyAuditEntryCandidate
): boolean {
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
    entry.supportBackendRetainedPayload,
    entry.supportBackendDeletedPayload,
    entry.ocentraHostedFamilyDataDefault,
    entry.accountLookupExecuted,
    entry.billingProviderContactExecuted,
    entry.remoteSupportSessionExecuted,
    entry.productionSlaClaimed,
  ].some(Boolean);
}

function supportBackendUploadCustodyAuditRefsArePresent(
  entry: SupportBackendUploadCustodyAuditEntryCandidate
): boolean {
  return (
    entry.consentRefs.length > 0 &&
    entry.redactionRefs.length > 0 &&
    entry.auditRefs.length > 0 &&
    entry.statusRefs.length > 0 &&
    entry.runtimeRefs.length > 0 &&
    entry.custodyRefs.length > 0
  );
}

function supportBackendUploadCustodyAuditStatesAreCoherent(
  entry: SupportBackendUploadCustodyAuditEntryCandidate
): boolean {
  return (
    supportBackendUploadCustodyAuditBoundaryIsCoherent(entry) &&
    supportBackendUploadCustodyAuditRetentionIsCoherent(entry) &&
    supportBackendUploadCustodyAuditDeleteIsCoherent(entry) &&
    supportBackendUploadCustodyAuditExportIsCoherent(entry)
  );
}

function supportBackendUploadCustodyAuditBoundaryIsCoherent(
  entry: SupportBackendUploadCustodyAuditEntryCandidate
): boolean {
  return (
    entry.parentInitiationState === 'parent-initiated' &&
    entry.parentConsentState === 'parent-approved' &&
    entry.payloadState === 'redacted-audit-refs-only' &&
    entry.custodyState === 'parent-owned-export-only' &&
    entry.executionClaimState === 'custody-audit-boundary-only'
  );
}

function supportBackendUploadCustodyAuditRetentionIsCoherent(
  entry: SupportBackendUploadCustodyAuditEntryCandidate
): boolean {
  if (!['custody-boundary-recorded', 'retention-manual-required'].includes(entry.auditState)) {
    return true;
  }

  return entry.retentionState === 'manual-required' && entry.manualProofRequirements.length > 0;
}

function supportBackendUploadCustodyAuditDeleteIsCoherent(
  entry: SupportBackendUploadCustodyAuditEntryCandidate
): boolean {
  if (!['delete-request-recorded', 'deletion-manual-required'].includes(entry.auditState)) {
    return true;
  }

  return entry.deleteState === 'manual-required' && entry.deleteRefs.length > 0;
}

function supportBackendUploadCustodyAuditExportIsCoherent(
  entry: SupportBackendUploadCustodyAuditEntryCandidate
): boolean {
  return (
    entry.auditState !== 'audit-export-ready' ||
    (entry.auditExportState === 'support-safe-export-ready' &&
      entry.auditRefs.length > 0 &&
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
