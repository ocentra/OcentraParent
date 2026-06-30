/* generated from crates/schema/src/parent_owned_sync_export.rs */

import type {
  GeneratedParentOwnedSyncExportContractProof,
  GeneratedParentOwnedSyncExportManifestItem,
  GeneratedParentOwnedSyncExportNonClaim,
  GeneratedParentOwnedSyncProviderStatusRow,
  GeneratedParentOwnedSyncStateRow,
  GeneratedParentOwnedSyncTombstoneRow,
} from './parent-owned-sync-export-contracts';

const requiredDataClasses = [
  'encrypted-journal-segment',
  'sqlite-query-row',
  'parent-rule',
  'approval-decision',
  'device-registry-entry',
  'notification-history',
  'audit-event',
  'generated-summary',
] as const;

const requiredNonClaims = [
  'no-transfer-runtime',
  'no-connector-oauth',
  'no-upload-runtime',
  'no-delete-runtime',
  'no-default-ocentra-custody',
  'no-raw-child-evidence-upload-by-default',
] as const;

export function syncExportManifestItemIsHonestGenerated(item: GeneratedParentOwnedSyncExportManifestItem): boolean {
  if (
    !item.parentActionRequired ||
    item.rawChildEvidenceUploadedByDefault ||
    item.ocentraHostedFamilyDataStored ||
    !item.claimSafe ||
    item.destinationOwnership === 'ocentra-hosted-non-activity-metadata'
  ) {
    return false;
  }

  if (item.exportFormat === 'human-readable-parent-report') {
    return (
      item.encryption.encryptionState === 'human-readable-parent-authorized' &&
      !item.encryption.encryptedBeforeUpload
    );
  }

  return item.encryption.encryptionState === 'encrypted-at-rest' && item.encryption.encryptedBeforeUpload;
}

export function syncExportProviderStatusRowIsHonestGenerated(row: GeneratedParentOwnedSyncProviderStatusRow): boolean {
  if (
    row.destinationOwnership === 'ocentra-hosted-non-activity-metadata' ||
    row.oauthRuntimeClaimed ||
    row.uploadRuntimeClaimed ||
    row.deleteRuntimeClaimed ||
    row.ocentraHostedFamilyDataStored ||
    !row.claimSafe
  ) {
    return false;
  }

  if (row.providerStatus === 'ready') {
    return row.accountRef !== null && row.folderRef !== null;
  }
  if (row.providerStatus === 'revoked') {
    return row.revocationRef !== null;
  }
  if (row.providerStatus === 'disabled' || row.providerStatus === 'not-configured') {
    return row.accountRef === null && row.folderRef === null && row.revocationRef === null;
  }
  if (row.providerStatus === 'disconnected') {
    return row.disconnectVisibilityState === 'disconnect-visible';
  }
  if (row.providerStatus === 'manual-required') {
    return row.disconnectVisibilityState === 'manual-required' || row.deleteVisibilityState === 'manual-required';
  }
  return true;
}

export function syncExportSyncStateRowIsHonestGenerated(row: GeneratedParentOwnedSyncStateRow): boolean {
  if (!row.claimSafe) {
    return false;
  }

  if (row.syncState === 'synced' || row.syncState === 'stale') {
    return (
      row.cursorRef !== null &&
      row.batchRef !== null &&
      row.manifestChecksumRef !== null &&
      row.manifestSignatureRef !== null &&
      row.lastSuccessfulSyncAt !== null &&
      row.manifestIntegrityState !== 'corrupt' &&
      row.manifestIntegrityState !== 'not-applicable'
    );
  }
  if (row.syncState === 'conflict') {
    return row.conflictRef !== null && row.retryQueueRef !== null;
  }
  if (
    row.syncState === 'missing' ||
    row.syncState === 'offline-retry-pending' ||
    row.syncState === 'partial-outage' ||
    row.syncState === 'manual-required'
  ) {
    return row.retryQueueRef !== null;
  }
  return (
    row.cursorRef === null &&
    row.batchRef === null &&
    row.manifestChecksumRef === null &&
    row.manifestSignatureRef === null &&
    row.lastSuccessfulSyncAt === null &&
    row.conflictRef === null &&
    row.retryQueueRef === null &&
    row.manifestIntegrityState === 'not-applicable'
  );
}

export function syncExportTombstoneRowIsHonestGenerated(row: GeneratedParentOwnedSyncTombstoneRow): boolean {
  if (!row.claimSafe) {
    return false;
  }

  if (row.propagationState === 'not-requested') {
    return row.deleteRequestRef === null && row.lastPropagatedAt === null && row.blockedReasonRef === null;
  }
  if (row.propagationState === 'pending') {
    return row.deleteRequestRef !== null;
  }
  if (row.propagationState === 'propagated') {
    return row.deleteRequestRef !== null && row.lastPropagatedAt !== null;
  }
  return row.deleteRequestRef !== null && row.blockedReasonRef !== null;
}

export function syncExportContractProofIsHonestGenerated(proof: GeneratedParentOwnedSyncExportContractProof): boolean {
  return (
    hasRequiredNonClaimsGenerated(proof.nonClaims) &&
    proof.manifest.items.every(syncExportManifestItemIsHonestGenerated) &&
    proof.providerStatuses.every(syncExportProviderStatusRowIsHonestGenerated) &&
    proof.syncStates.every(syncExportSyncStateRowIsHonestGenerated) &&
    proof.tombstones.every(syncExportTombstoneRowIsHonestGenerated) &&
    !proof.transferRuntimeClaimed &&
    !proof.connectorOAuthClaimed &&
    !proof.uploadRuntimeClaimed &&
    !proof.deleteRuntimeClaimed &&
    !proof.ocentraHostedChildEvidenceStored
  );
}

export function syncExportCoversRequiredDataClassesGenerated(
  items: ReadonlyArray<GeneratedParentOwnedSyncExportManifestItem>
): boolean {
  const covered = new Set(items.map((item) => item.dataClass));
  return requiredDataClasses.every((dataClass) => covered.has(dataClass));
}

function hasRequiredNonClaimsGenerated(nonClaims: readonly GeneratedParentOwnedSyncExportNonClaim[]): boolean {
  const claims = new Set(nonClaims);
  return claims.size === nonClaims.length && requiredNonClaims.every((claim) => claims.has(claim));
}
