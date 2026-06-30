use super::parent_owned_sync_export::{
    parent_owned_sync_export_known_gaps, sample_parent_owned_sync_export_contract_proof,
    PARENT_OWNED_SYNC_EXPORT_SCHEMA_VERSION,
};

const PARENT_OWNED_SYNC_EXPORT_PROOF_JSON_EXPECT_MESSAGE: &str =
    "parent owned sync export proof json";
const PARENT_OWNED_SYNC_EXPORT_TYPESCRIPT_LINE_BREAK: &str = "\n";

pub fn parent_owned_sync_export_contracts_typescript() -> String {
    let proof_json = crate::schema_result_or_unreachable(
        serde_json::to_string_pretty(&sample_parent_owned_sync_export_contract_proof()),
        PARENT_OWNED_SYNC_EXPORT_PROOF_JSON_EXPECT_MESSAGE,
    );
    let known_gaps = parent_owned_sync_export_known_gaps()
        .iter()
        .map(|gap| format!("  {:?},", gap))
        .collect::<Vec<_>>()
        .join(PARENT_OWNED_SYNC_EXPORT_TYPESCRIPT_LINE_BREAK);

    format!(
        r#"/* generated from crates/schema/src/parent_owned_sync_export.rs */

export const ParentOwnedSyncExportContractRuntime = {{
  SchemaVersion: '{schema_version}',
}} as const;

export type GeneratedParentContractSchemaVersion = 'v0.6';
export type GeneratedFamilyId = string;
export type GeneratedChildProfileId = string;
export type GeneratedParentDeviceId = string;
export type GeneratedParentDeviceLabel = string;
export type GeneratedParentActorId = string;
export type GeneratedParentPolicyVersion = string;
export type GeneratedParentEvidenceReferenceId = string;
export type GeneratedParentActionReferenceId = string;
export type GeneratedParentTimestamp = string;
export type GeneratedParentOwnedSyncManifestId = string;
export type GeneratedParentOwnedSyncItemId = string;
export type GeneratedParentOwnedSyncVersionLabel = string;
export type GeneratedParentOwnedSyncPolicyRef = string;
export type GeneratedParentOwnedSyncProviderId = string;
export type GeneratedParentOwnedSyncProviderRef = string;
export type GeneratedParentOwnedSyncStatusRef = string;
export type GeneratedParentOwnedSyncCursorRef = string;
export type GeneratedParentOwnedSyncBatchRef = string;
export type GeneratedParentOwnedSyncConflictRef = string;
export type GeneratedParentOwnedSyncChecksumRef = string;
export type GeneratedParentOwnedSyncSignatureRef = string;
export type GeneratedParentOwnedSyncTombstoneRef = string;
export type GeneratedParentOwnedSyncDeleteRequestRef = string;

export type GeneratedParentPlatform = 'windows' | 'linux' | 'macos' | 'android' | 'ios';
export type GeneratedParentActorRole = 'parent' | 'guardian' | 'system';
export type GeneratedParentEvidenceReferenceKind = 'journal-event' | 'query-store-summary' | 'audit-trail';
export type GeneratedParentOwnedSyncExportDataClass =
  | 'encrypted-journal-segment'
  | 'sqlite-query-row'
  | 'parent-rule'
  | 'approval-decision'
  | 'device-registry-entry'
  | 'notification-history'
  | 'audit-event'
  | 'generated-summary';
export type GeneratedParentOwnedSyncExportFormat =
  | 'encrypted-machine-readable'
  | 'encrypted-support-bundle'
  | 'human-readable-parent-report';
export type GeneratedParentOwnedSyncExportDestinationOwnership =
  | 'child-local'
  | 'parent-device-local'
  | 'parent-owned-external-storage'
  | 'ocentra-hosted-non-activity-metadata';
export type GeneratedParentOwnedSyncExportEncryptionState =
  | 'encrypted-at-rest'
  | 'human-readable-parent-authorized'
  | 'not-applicable';
export type GeneratedParentOwnedSyncProviderMode =
  | 'google-drive-appdata'
  | 'google-drive-picker-file'
  | 'onedrive-approot'
  | 'onedrive-parent-selected-folder'
  | 'icloud-drive-app-container'
  | 'icloud-drive-parent-selected-location'
  | 'dropbox-app-folder'
  | 'dropbox-parent-selected-folder'
  | 'nas-folder'
  | 'local-folder'
  | 'disabled';
export type GeneratedParentOwnedSyncProviderStatus =
  | 'ready'
  | 'manual-required'
  | 'revoked'
  | 'wrong-account'
  | 'folder-unavailable'
  | 'partial-upload'
  | 'disconnected'
  | 'disabled'
  | 'not-configured';
export type GeneratedParentOwnedSyncManifestIntegrityState = 'verified' | 'mismatch' | 'corrupt' | 'not-applicable';
export type GeneratedParentOwnedSyncState =
  | 'synced'
  | 'stale'
  | 'missing'
  | 'conflict'
  | 'offline-retry-pending'
  | 'partial-outage'
  | 'manual-required'
  | 'not-started';
export type GeneratedParentOwnedSyncTombstonePropagationState =
  | 'not-requested'
  | 'pending'
  | 'propagated'
  | 'blocked'
  | 'manual-required';
export type GeneratedParentOwnedSyncDisconnectVisibilityState =
  | 'not-disconnected'
  | 'disconnect-visible'
  | 'manual-required';
export type GeneratedParentOwnedSyncDeleteVisibilityState =
  | 'not-requested'
  | 'delete-visible'
  | 'delete-confirmed'
  | 'delete-failed'
  | 'manual-required';
export type GeneratedParentOwnedSyncExportNonClaim =
  | 'no-transfer-runtime'
  | 'no-connector-oauth'
  | 'no-upload-runtime'
  | 'no-delete-runtime'
  | 'no-default-ocentra-custody'
  | 'no-raw-child-evidence-upload-by-default';

export const GeneratedParentOwnedSyncExportDataClasses = [
  'encrypted-journal-segment',
  'sqlite-query-row',
  'parent-rule',
  'approval-decision',
  'device-registry-entry',
  'notification-history',
  'audit-event',
  'generated-summary',
] as const satisfies readonly GeneratedParentOwnedSyncExportDataClass[];
export const GeneratedParentOwnedSyncExportFormats = [
  'encrypted-machine-readable',
  'encrypted-support-bundle',
  'human-readable-parent-report',
] as const satisfies readonly GeneratedParentOwnedSyncExportFormat[];
export const GeneratedParentOwnedSyncExportDestinationOwnerships = [
  'child-local',
  'parent-device-local',
  'parent-owned-external-storage',
  'ocentra-hosted-non-activity-metadata',
] as const satisfies readonly GeneratedParentOwnedSyncExportDestinationOwnership[];
export const GeneratedParentOwnedSyncExportEncryptionStates = [
  'encrypted-at-rest',
  'human-readable-parent-authorized',
  'not-applicable',
] as const satisfies readonly GeneratedParentOwnedSyncExportEncryptionState[];
export const GeneratedParentOwnedSyncProviderModes = [
  'google-drive-appdata',
  'google-drive-picker-file',
  'onedrive-approot',
  'onedrive-parent-selected-folder',
  'icloud-drive-app-container',
  'icloud-drive-parent-selected-location',
  'dropbox-app-folder',
  'dropbox-parent-selected-folder',
  'nas-folder',
  'local-folder',
  'disabled',
] as const satisfies readonly GeneratedParentOwnedSyncProviderMode[];
export const GeneratedParentOwnedSyncProviderStatuses = [
  'ready',
  'manual-required',
  'revoked',
  'wrong-account',
  'folder-unavailable',
  'partial-upload',
  'disconnected',
  'disabled',
  'not-configured',
] as const satisfies readonly GeneratedParentOwnedSyncProviderStatus[];
export const GeneratedParentOwnedSyncManifestIntegrityStates = [
  'verified',
  'mismatch',
  'corrupt',
  'not-applicable',
] as const satisfies readonly GeneratedParentOwnedSyncManifestIntegrityState[];
export const GeneratedParentOwnedSyncStates = [
  'synced',
  'stale',
  'missing',
  'conflict',
  'offline-retry-pending',
  'partial-outage',
  'manual-required',
  'not-started',
] as const satisfies readonly GeneratedParentOwnedSyncState[];
export const GeneratedParentOwnedSyncTombstonePropagationStates = [
  'not-requested',
  'pending',
  'propagated',
  'blocked',
  'manual-required',
] as const satisfies readonly GeneratedParentOwnedSyncTombstonePropagationState[];
export const GeneratedParentOwnedSyncDisconnectVisibilityStates = [
  'not-disconnected',
  'disconnect-visible',
  'manual-required',
] as const satisfies readonly GeneratedParentOwnedSyncDisconnectVisibilityState[];
export const GeneratedParentOwnedSyncDeleteVisibilityStates = [
  'not-requested',
  'delete-visible',
  'delete-confirmed',
  'delete-failed',
  'manual-required',
] as const satisfies readonly GeneratedParentOwnedSyncDeleteVisibilityState[];
export const GeneratedParentOwnedSyncExportNonClaims = [
  'no-transfer-runtime',
  'no-connector-oauth',
  'no-upload-runtime',
  'no-delete-runtime',
  'no-default-ocentra-custody',
  'no-raw-child-evidence-upload-by-default',
] as const satisfies readonly GeneratedParentOwnedSyncExportNonClaim[];

export interface GeneratedParentActorReference {{
  actorId: GeneratedParentActorId;
  role: GeneratedParentActorRole;
}}

export interface GeneratedFamilyReference {{
  familyId: GeneratedFamilyId;
}}

export interface GeneratedParentDeviceReference {{
  deviceId: GeneratedParentDeviceId;
  childProfileId: GeneratedChildProfileId | null;
  label: GeneratedParentDeviceLabel;
  platform: GeneratedParentPlatform;
}}

export interface GeneratedParentEvidenceReference {{
  evidenceReferenceId: GeneratedParentEvidenceReferenceId;
  kind: GeneratedParentEvidenceReferenceKind;
  observedAt: GeneratedParentTimestamp;
}}

export interface GeneratedParentActionReference {{
  actionReferenceId: GeneratedParentActionReferenceId;
  actor: GeneratedParentActorReference;
  policyVersion: GeneratedParentPolicyVersion;
  createdAt: GeneratedParentTimestamp;
}}

export interface GeneratedParentOwnedSyncExportEncryptionMetadata {{
  encryptionState: GeneratedParentOwnedSyncExportEncryptionState;
  encryptedBeforeUpload: boolean;
  keyOwner: GeneratedParentOwnedSyncExportDestinationOwnership;
  proofRequirementRef: GeneratedParentOwnedSyncPolicyRef;
}}

export interface GeneratedParentOwnedSyncExportManifestItem {{
  itemId: GeneratedParentOwnedSyncItemId;
  dataClass: GeneratedParentOwnedSyncExportDataClass;
  exportFormat: GeneratedParentOwnedSyncExportFormat;
  destinationOwnership: GeneratedParentOwnedSyncExportDestinationOwnership;
  schemaVersionLabel: GeneratedParentOwnedSyncVersionLabel;
  encryption: GeneratedParentOwnedSyncExportEncryptionMetadata;
  parentActionRequired: boolean;
  rawChildEvidenceUploadedByDefault: boolean;
  ocentraHostedFamilyDataStored: boolean;
  claimSafe: boolean;
}}

export interface GeneratedParentOwnedSyncExportManifest {{
  schemaVersion: typeof ParentOwnedSyncExportContractRuntime.SchemaVersion;
  manifestId: GeneratedParentOwnedSyncManifestId;
  family: GeneratedFamilyReference;
  device: GeneratedParentDeviceReference;
  parentAction: GeneratedParentActionReference;
  productVersion: GeneratedParentOwnedSyncVersionLabel;
  manifestVersion: GeneratedParentOwnedSyncVersionLabel;
  generatedAt: GeneratedParentTimestamp;
  items: readonly GeneratedParentOwnedSyncExportManifestItem[];
}}

export interface GeneratedParentOwnedSyncProviderStatusRow {{
  providerId: GeneratedParentOwnedSyncProviderId;
  providerMode: GeneratedParentOwnedSyncProviderMode;
  providerStatus: GeneratedParentOwnedSyncProviderStatus;
  destinationOwnership: GeneratedParentOwnedSyncExportDestinationOwnership;
  accountRef: GeneratedParentOwnedSyncProviderRef | null;
  folderRef: GeneratedParentOwnedSyncProviderRef | null;
  statusRef: GeneratedParentOwnedSyncStatusRef;
  revocationRef: GeneratedParentOwnedSyncProviderRef | null;
  disconnectVisibilityState: GeneratedParentOwnedSyncDisconnectVisibilityState;
  deleteVisibilityState: GeneratedParentOwnedSyncDeleteVisibilityState;
  lastCheckedAt: GeneratedParentTimestamp;
  oauthRuntimeClaimed: boolean;
  uploadRuntimeClaimed: boolean;
  deleteRuntimeClaimed: boolean;
  ocentraHostedFamilyDataStored: boolean;
  claimSafe: boolean;
}}

export interface GeneratedParentOwnedSyncStateRow {{
  syncState: GeneratedParentOwnedSyncState;
  providerStatusRef: GeneratedParentOwnedSyncStatusRef;
  cursorRef: GeneratedParentOwnedSyncCursorRef | null;
  batchRef: GeneratedParentOwnedSyncBatchRef | null;
  manifestIntegrityState: GeneratedParentOwnedSyncManifestIntegrityState;
  manifestChecksumRef: GeneratedParentOwnedSyncChecksumRef | null;
  manifestSignatureRef: GeneratedParentOwnedSyncSignatureRef | null;
  lastSuccessfulSyncAt: GeneratedParentTimestamp | null;
  conflictRef: GeneratedParentOwnedSyncConflictRef | null;
  retryQueueRef: GeneratedParentOwnedSyncPolicyRef | null;
  parentActionRequired: boolean;
  claimSafe: boolean;
}}

export interface GeneratedParentOwnedSyncTombstoneRow {{
  tombstoneRef: GeneratedParentOwnedSyncTombstoneRef;
  dataClass: GeneratedParentOwnedSyncExportDataClass;
  propagationState: GeneratedParentOwnedSyncTombstonePropagationState;
  deleteRequestRef: GeneratedParentOwnedSyncDeleteRequestRef | null;
  providerStatusRef: GeneratedParentOwnedSyncStatusRef;
  lastPropagatedAt: GeneratedParentTimestamp | null;
  blockedReasonRef: GeneratedParentOwnedSyncPolicyRef | null;
  claimSafe: boolean;
}}

export interface GeneratedParentOwnedSyncExportContractProof {{
  schemaVersion: typeof ParentOwnedSyncExportContractRuntime.SchemaVersion;
  contractVersion: GeneratedParentContractSchemaVersion;
  manifest: GeneratedParentOwnedSyncExportManifest;
  providerStatuses: readonly GeneratedParentOwnedSyncProviderStatusRow[];
  syncStates: readonly GeneratedParentOwnedSyncStateRow[];
  tombstones: readonly GeneratedParentOwnedSyncTombstoneRow[];
  nonClaims: readonly GeneratedParentOwnedSyncExportNonClaim[];
  transferRuntimeClaimed: boolean;
  connectorOAuthClaimed: boolean;
  uploadRuntimeClaimed: boolean;
  deleteRuntimeClaimed: boolean;
  ocentraHostedChildEvidenceStored: boolean;
  updatedAt: GeneratedParentTimestamp;
}}

export const GeneratedParentOwnedSyncExportKnownGaps = [
{known_gaps}
] as const;

export const GeneratedParentOwnedSyncExportContractProof = {proof_json} as const satisfies GeneratedParentOwnedSyncExportContractProof;
"#,
        schema_version = PARENT_OWNED_SYNC_EXPORT_SCHEMA_VERSION,
        proof_json = proof_json,
        known_gaps = known_gaps,
    )
}

pub fn parent_owned_sync_export_contract_rules_typescript() -> String {
    r#"/* generated from crates/schema/src/parent_owned_sync_export.rs */

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
"#
    .to_owned()
}
