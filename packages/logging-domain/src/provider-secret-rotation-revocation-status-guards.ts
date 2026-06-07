import {
  ProviderSecretRotationRevocationRequiredDataClasses,
  type ProviderSecretRotationRevocationDataClass,
  type ProviderSecretRotationRevocationStatusEntryCandidate,
} from './provider-secret-rotation-revocation-status.js';

export function providerSecretRotationRevocationStatusEntryIsSafe(
  entry: ProviderSecretRotationRevocationStatusEntryCandidate
): boolean {
  return (
    !providerSecretRotationRevocationStatusHasClaimUpgrade(entry) &&
    providerSecretRotationRevocationRequiredValuesArePresent(entry.disclosedDataClasses) &&
    providerSecretRotationRevocationRefsArePresent(entry) &&
    providerSecretRotationRevocationStatesAreCoherent(entry)
  );
}

function providerSecretRotationRevocationStatusHasClaimUpgrade(
  entry: ProviderSecretRotationRevocationStatusEntryCandidate
): boolean {
  return [
    entry.containsProviderSecrets,
    entry.containsPaymentProviderTokens,
    entry.containsRawChildActivity,
    entry.containsRawSupportBundlePayloads,
    entry.containsAccountLookupResults,
    entry.containsBillingProviderContactRecords,
    entry.containsRemoteSupportTranscripts,
    entry.backendSecretStoreExecuted,
    entry.rotationExecuted,
    entry.revocationExecuted,
    entry.providerSecretDelivered,
    entry.supportBackendUploadExecuted,
    entry.accountLookupExecuted,
    entry.billingProviderContactExecuted,
    entry.remoteSupportSessionExecuted,
    entry.productionSlaClaimed,
    entry.ocentraHostedFamilyDataDefault,
  ].some(Boolean);
}

function providerSecretRotationRevocationRefsArePresent(
  entry: ProviderSecretRotationRevocationStatusEntryCandidate
): boolean {
  return (
    entry.custodyStatusRefs.length > 0 &&
    entry.executionReadinessRefs.length > 0 &&
    entry.backendSecretStoreRefs.length > 0 &&
    entry.auditRefs.length > 0 &&
    entry.manualProofRequirements.length > 0
  );
}

function providerSecretRotationRevocationStatesAreCoherent(
  entry: ProviderSecretRotationRevocationStatusEntryCandidate
): boolean {
  return (
    entry.payloadState === 'support-safe-status-refs-only' &&
    entry.allowedDestinations.includes('support-safe-status-boundary') &&
    providerSecretRotationStateIsCoherent(entry) &&
    providerSecretRevocationStateIsCoherent(entry) &&
    providerSecretOperatorApprovalStateIsCoherent(entry)
  );
}

function providerSecretRotationStateIsCoherent(entry: ProviderSecretRotationRevocationStatusEntryCandidate): boolean {
  if (
    entry.rotationRevocationStatus.startsWith('rotation-') ||
    entry.rotationRevocationStatus === 'audit-export-ready'
  ) {
    return entry.rotationState !== 'not-applicable' && entry.rotationRefs.length > 0;
  }

  return entry.rotationState === 'not-applicable';
}

function providerSecretRevocationStateIsCoherent(entry: ProviderSecretRotationRevocationStatusEntryCandidate): boolean {
  if (entry.rotationRevocationStatus.startsWith('revocation-')) {
    return entry.revocationState !== 'not-applicable' && entry.revocationRefs.length > 0;
  }

  return entry.revocationState === 'not-applicable' || entry.rotationRevocationStatus === 'audit-export-ready';
}

function providerSecretOperatorApprovalStateIsCoherent(
  entry: ProviderSecretRotationRevocationStatusEntryCandidate
): boolean {
  if (entry.rotationRevocationStatus.endsWith('manual-required')) {
    return entry.operatorApprovalState === 'manual-required' && entry.operatorApprovalRefs.length > 0;
  }

  return true;
}

function providerSecretRotationRevocationRequiredValuesArePresent(
  actualValues: ReadonlyArray<ProviderSecretRotationRevocationDataClass>
): boolean {
  const actual = new Set(actualValues);
  return (
    actual.size === actualValues.length &&
    ProviderSecretRotationRevocationRequiredDataClasses.every((value) => actual.has(value))
  );
}
