/* generated from crates/schema/src/parent_owned_sync_export.rs */

import { brandedNonEmptyStringSchema } from './effect';

export const ParentOwnedSyncExportContractRuntime = {
  SchemaVersion: 'parent-owned-sync-export-manifest-proof',
} as const;

export type GeneratedParentContractSchemaVersion = 'v0.6';
export const GeneratedFamilyIdSchema = brandedNonEmptyStringSchema('FamilyId');
export const GeneratedChildProfileIdSchema = brandedNonEmptyStringSchema('ChildProfileId');
export const GeneratedParentDeviceIdSchema = brandedNonEmptyStringSchema('ParentDeviceId');
export const GeneratedParentDeviceLabelSchema = brandedNonEmptyStringSchema('ParentDeviceLabel');
export const GeneratedParentActorIdSchema = brandedNonEmptyStringSchema('ParentActorId');
export const GeneratedParentPolicyVersionSchema = brandedNonEmptyStringSchema('ParentPolicyVersion');
export const GeneratedParentEvidenceReferenceIdSchema = brandedNonEmptyStringSchema('ParentEvidenceReferenceId');
export const GeneratedParentActionReferenceIdSchema = brandedNonEmptyStringSchema('ParentActionReferenceId');
export const GeneratedParentTimestampSchema = brandedNonEmptyStringSchema('ParentTimestamp');
export const GeneratedParentOwnedSyncManifestIdSchema = brandedNonEmptyStringSchema('ParentOwnedSyncManifestId');
export const GeneratedParentOwnedSyncItemIdSchema = brandedNonEmptyStringSchema('ParentOwnedSyncItemId');
export const GeneratedParentOwnedSyncVersionLabelSchema = brandedNonEmptyStringSchema('ParentOwnedSyncVersionLabel');
export const GeneratedParentOwnedSyncPolicyRefSchema = brandedNonEmptyStringSchema('ParentOwnedSyncPolicyRef');
export const GeneratedParentOwnedSyncProviderIdSchema = brandedNonEmptyStringSchema('ParentOwnedSyncProviderId');
export const GeneratedParentOwnedSyncProviderRefSchema = brandedNonEmptyStringSchema('ParentOwnedSyncProviderRef');
export const GeneratedParentOwnedSyncStatusRefSchema = brandedNonEmptyStringSchema('ParentOwnedSyncStatusRef');
export const GeneratedParentOwnedSyncCursorRefSchema = brandedNonEmptyStringSchema('ParentOwnedSyncCursorRef');
export const GeneratedParentOwnedSyncBatchRefSchema = brandedNonEmptyStringSchema('ParentOwnedSyncBatchRef');
export const GeneratedParentOwnedSyncConflictRefSchema = brandedNonEmptyStringSchema('ParentOwnedSyncConflictRef');
export const GeneratedParentOwnedSyncChecksumRefSchema = brandedNonEmptyStringSchema('ParentOwnedSyncChecksumRef');
export const GeneratedParentOwnedSyncSignatureRefSchema = brandedNonEmptyStringSchema('ParentOwnedSyncSignatureRef');
export const GeneratedParentOwnedSyncTombstoneRefSchema = brandedNonEmptyStringSchema('ParentOwnedSyncTombstoneRef');
export const GeneratedParentOwnedSyncDeleteRequestRefSchema = brandedNonEmptyStringSchema(
  'ParentOwnedSyncDeleteRequestRef'
);

export type GeneratedFamilyId = typeof GeneratedFamilyIdSchema.Type;
export type GeneratedChildProfileId = typeof GeneratedChildProfileIdSchema.Type;
export type GeneratedParentDeviceId = typeof GeneratedParentDeviceIdSchema.Type;
export type GeneratedParentDeviceLabel = typeof GeneratedParentDeviceLabelSchema.Type;
export type GeneratedParentActorId = typeof GeneratedParentActorIdSchema.Type;
export type GeneratedParentPolicyVersion = typeof GeneratedParentPolicyVersionSchema.Type;
export type GeneratedParentEvidenceReferenceId = typeof GeneratedParentEvidenceReferenceIdSchema.Type;
export type GeneratedParentActionReferenceId = typeof GeneratedParentActionReferenceIdSchema.Type;
export type GeneratedParentTimestamp = typeof GeneratedParentTimestampSchema.Type;
export type GeneratedParentOwnedSyncManifestId = typeof GeneratedParentOwnedSyncManifestIdSchema.Type;
export type GeneratedParentOwnedSyncItemId = typeof GeneratedParentOwnedSyncItemIdSchema.Type;
export type GeneratedParentOwnedSyncVersionLabel = typeof GeneratedParentOwnedSyncVersionLabelSchema.Type;
export type GeneratedParentOwnedSyncPolicyRef = typeof GeneratedParentOwnedSyncPolicyRefSchema.Type;
export type GeneratedParentOwnedSyncProviderId = typeof GeneratedParentOwnedSyncProviderIdSchema.Type;
export type GeneratedParentOwnedSyncProviderRef = typeof GeneratedParentOwnedSyncProviderRefSchema.Type;
export type GeneratedParentOwnedSyncStatusRef = typeof GeneratedParentOwnedSyncStatusRefSchema.Type;
export type GeneratedParentOwnedSyncCursorRef = typeof GeneratedParentOwnedSyncCursorRefSchema.Type;
export type GeneratedParentOwnedSyncBatchRef = typeof GeneratedParentOwnedSyncBatchRefSchema.Type;
export type GeneratedParentOwnedSyncConflictRef = typeof GeneratedParentOwnedSyncConflictRefSchema.Type;
export type GeneratedParentOwnedSyncChecksumRef = typeof GeneratedParentOwnedSyncChecksumRefSchema.Type;
export type GeneratedParentOwnedSyncSignatureRef = typeof GeneratedParentOwnedSyncSignatureRefSchema.Type;
export type GeneratedParentOwnedSyncTombstoneRef = typeof GeneratedParentOwnedSyncTombstoneRefSchema.Type;
export type GeneratedParentOwnedSyncDeleteRequestRef = typeof GeneratedParentOwnedSyncDeleteRequestRefSchema.Type;

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

export interface GeneratedParentActorReference {
  readonly actorId: GeneratedParentActorId;
  readonly role: GeneratedParentActorRole;
}

export interface GeneratedFamilyReference {
  readonly familyId: GeneratedFamilyId;
}

export interface GeneratedParentDeviceReference {
  readonly deviceId: GeneratedParentDeviceId;
  readonly childProfileId: GeneratedChildProfileId | null;
  readonly label: GeneratedParentDeviceLabel;
  readonly platform: GeneratedParentPlatform;
}

export interface GeneratedParentEvidenceReference {
  readonly evidenceReferenceId: GeneratedParentEvidenceReferenceId;
  readonly kind: GeneratedParentEvidenceReferenceKind;
  readonly observedAt: GeneratedParentTimestamp;
}

export interface GeneratedParentActionReference {
  readonly actionReferenceId: GeneratedParentActionReferenceId;
  readonly actor: GeneratedParentActorReference;
  readonly policyVersion: GeneratedParentPolicyVersion;
  readonly createdAt: GeneratedParentTimestamp;
}

export interface GeneratedParentOwnedSyncExportEncryptionMetadata {
  readonly encryptionState: GeneratedParentOwnedSyncExportEncryptionState;
  readonly encryptedBeforeUpload: boolean;
  readonly keyOwner: GeneratedParentOwnedSyncExportDestinationOwnership;
  readonly proofRequirementRef: GeneratedParentOwnedSyncPolicyRef;
}

export interface GeneratedParentOwnedSyncExportManifestItem {
  readonly itemId: GeneratedParentOwnedSyncItemId;
  readonly dataClass: GeneratedParentOwnedSyncExportDataClass;
  readonly exportFormat: GeneratedParentOwnedSyncExportFormat;
  readonly destinationOwnership: GeneratedParentOwnedSyncExportDestinationOwnership;
  readonly schemaVersionLabel: GeneratedParentOwnedSyncVersionLabel;
  readonly encryption: GeneratedParentOwnedSyncExportEncryptionMetadata;
  readonly parentActionRequired: boolean;
  readonly rawChildEvidenceUploadedByDefault: boolean;
  readonly ocentraHostedFamilyDataStored: boolean;
  readonly claimSafe: boolean;
}

export interface GeneratedParentOwnedSyncExportManifest {
  readonly schemaVersion: typeof ParentOwnedSyncExportContractRuntime.SchemaVersion;
  readonly manifestId: GeneratedParentOwnedSyncManifestId;
  readonly family: GeneratedFamilyReference;
  readonly device: GeneratedParentDeviceReference;
  readonly parentAction: GeneratedParentActionReference;
  readonly productVersion: GeneratedParentOwnedSyncVersionLabel;
  readonly manifestVersion: GeneratedParentOwnedSyncVersionLabel;
  readonly generatedAt: GeneratedParentTimestamp;
  readonly items: readonly GeneratedParentOwnedSyncExportManifestItem[];
}

export interface GeneratedParentOwnedSyncProviderStatusRow {
  readonly providerId: GeneratedParentOwnedSyncProviderId;
  readonly providerMode: GeneratedParentOwnedSyncProviderMode;
  readonly providerStatus: GeneratedParentOwnedSyncProviderStatus;
  readonly destinationOwnership: GeneratedParentOwnedSyncExportDestinationOwnership;
  readonly accountRef: GeneratedParentOwnedSyncProviderRef | null;
  readonly folderRef: GeneratedParentOwnedSyncProviderRef | null;
  readonly statusRef: GeneratedParentOwnedSyncStatusRef;
  readonly revocationRef: GeneratedParentOwnedSyncProviderRef | null;
  readonly disconnectVisibilityState: GeneratedParentOwnedSyncDisconnectVisibilityState;
  readonly deleteVisibilityState: GeneratedParentOwnedSyncDeleteVisibilityState;
  readonly lastCheckedAt: GeneratedParentTimestamp;
  readonly oauthRuntimeClaimed: boolean;
  readonly uploadRuntimeClaimed: boolean;
  readonly deleteRuntimeClaimed: boolean;
  readonly ocentraHostedFamilyDataStored: boolean;
  readonly claimSafe: boolean;
}

export interface GeneratedParentOwnedSyncStateRow {
  readonly syncState: GeneratedParentOwnedSyncState;
  readonly providerStatusRef: GeneratedParentOwnedSyncStatusRef;
  readonly cursorRef: GeneratedParentOwnedSyncCursorRef | null;
  readonly batchRef: GeneratedParentOwnedSyncBatchRef | null;
  readonly manifestIntegrityState: GeneratedParentOwnedSyncManifestIntegrityState;
  readonly manifestChecksumRef: GeneratedParentOwnedSyncChecksumRef | null;
  readonly manifestSignatureRef: GeneratedParentOwnedSyncSignatureRef | null;
  readonly lastSuccessfulSyncAt: GeneratedParentTimestamp | null;
  readonly conflictRef: GeneratedParentOwnedSyncConflictRef | null;
  readonly retryQueueRef: GeneratedParentOwnedSyncPolicyRef | null;
  readonly parentActionRequired: boolean;
  readonly claimSafe: boolean;
}

export interface GeneratedParentOwnedSyncTombstoneRow {
  readonly tombstoneRef: GeneratedParentOwnedSyncTombstoneRef;
  readonly dataClass: GeneratedParentOwnedSyncExportDataClass;
  readonly propagationState: GeneratedParentOwnedSyncTombstonePropagationState;
  readonly deleteRequestRef: GeneratedParentOwnedSyncDeleteRequestRef | null;
  readonly providerStatusRef: GeneratedParentOwnedSyncStatusRef;
  readonly lastPropagatedAt: GeneratedParentTimestamp | null;
  readonly blockedReasonRef: GeneratedParentOwnedSyncPolicyRef | null;
  readonly claimSafe: boolean;
}

export interface GeneratedParentOwnedSyncExportContractProof {
  readonly schemaVersion: typeof ParentOwnedSyncExportContractRuntime.SchemaVersion;
  readonly contractVersion: GeneratedParentContractSchemaVersion;
  readonly manifest: GeneratedParentOwnedSyncExportManifest;
  readonly providerStatuses: readonly GeneratedParentOwnedSyncProviderStatusRow[];
  readonly syncStates: readonly GeneratedParentOwnedSyncStateRow[];
  readonly tombstones: readonly GeneratedParentOwnedSyncTombstoneRow[];
  readonly nonClaims: readonly GeneratedParentOwnedSyncExportNonClaim[];
  readonly transferRuntimeClaimed: boolean;
  readonly connectorOAuthClaimed: boolean;
  readonly uploadRuntimeClaimed: boolean;
  readonly deleteRuntimeClaimed: boolean;
  readonly ocentraHostedChildEvidenceStored: boolean;
  readonly updatedAt: GeneratedParentTimestamp;
}

export const GeneratedParentOwnedSyncExportKnownGaps = [
  'No provider OAuth runtime, token refresh, or revocation handling is implemented by this contract proof.',
  'No provider upload or delete runtime is implemented; status rows stay claim-safe instead of implying transfer execution.',
  'Parent-owned cloud sync remains separate from local export/delete runtime and does not imply restore/apply-back execution.',
  'Manifest integrity is explicit, but checksum and signature refs are contract evidence only until runtime verification exists.',
  'Tombstone propagation is modeled separately from sync success so blocked or manual-required delete visibility stays explicit.',
  'Ocentra-hosted cloud metadata is not the default evidence store and no raw child evidence upload is claimed by default.',
] as const;

export const GeneratedParentOwnedSyncExportContractProof = {
  schemaVersion: 'parent-owned-sync-export-manifest-proof',
  contractVersion: 'v0.6',
  manifest: {
    schemaVersion: 'parent-owned-sync-export-manifest-proof',
    manifestId: 'parent-owned-sync-manifest-proof-1',
    family: {
      familyId: 'family-parent-owned-sync-proof-1',
    },
    device: {
      deviceId: 'windows-parent-owned-sync-proof-1',
      childProfileId: 'child-parent-owned-sync-proof-1',
      label: 'Windows parent-owned sync proof device',
      platform: 'windows',
    },
    parentAction: {
      actionReferenceId: 'parent-action-parent-owned-sync-proof-1',
      actor: {
        actorId: 'parent-owned-sync-proof-actor-1',
        role: 'parent',
      },
      policyVersion: 'parent-owned-sync-proof-policy-v1',
      createdAt: '2026-06-28T18:40:00.000Z',
    },
    productVersion: '0.1.1',
    manifestVersion: 'parent-owned-sync.manifest.v1',
    generatedAt: '2026-06-28T18:40:00.000Z',
    items: [
      {
        itemId: 'manifest-item-journal',
        dataClass: 'encrypted-journal-segment',
        exportFormat: 'encrypted-machine-readable',
        destinationOwnership: 'parent-owned-external-storage',
        schemaVersionLabel: 'journal.v1',
        encryption: {
          encryptionState: 'encrypted-at-rest',
          encryptedBeforeUpload: true,
          keyOwner: 'parent-owned-external-storage',
          proofRequirementRef: 'encryption-proof-journal',
        },
        parentActionRequired: true,
        rawChildEvidenceUploadedByDefault: false,
        ocentraHostedFamilyDataStored: false,
        claimSafe: true,
      },
      {
        itemId: 'manifest-item-query',
        dataClass: 'sqlite-query-row',
        exportFormat: 'encrypted-machine-readable',
        destinationOwnership: 'parent-owned-external-storage',
        schemaVersionLabel: 'query.v1',
        encryption: {
          encryptionState: 'encrypted-at-rest',
          encryptedBeforeUpload: true,
          keyOwner: 'parent-owned-external-storage',
          proofRequirementRef: 'encryption-proof-query',
        },
        parentActionRequired: true,
        rawChildEvidenceUploadedByDefault: false,
        ocentraHostedFamilyDataStored: false,
        claimSafe: true,
      },
      {
        itemId: 'manifest-item-rule',
        dataClass: 'parent-rule',
        exportFormat: 'encrypted-support-bundle',
        destinationOwnership: 'parent-owned-external-storage',
        schemaVersionLabel: 'rule.v1',
        encryption: {
          encryptionState: 'encrypted-at-rest',
          encryptedBeforeUpload: true,
          keyOwner: 'parent-owned-external-storage',
          proofRequirementRef: 'encryption-proof-rule',
        },
        parentActionRequired: true,
        rawChildEvidenceUploadedByDefault: false,
        ocentraHostedFamilyDataStored: false,
        claimSafe: true,
      },
      {
        itemId: 'manifest-item-approval',
        dataClass: 'approval-decision',
        exportFormat: 'encrypted-support-bundle',
        destinationOwnership: 'parent-owned-external-storage',
        schemaVersionLabel: 'approval.v1',
        encryption: {
          encryptionState: 'encrypted-at-rest',
          encryptedBeforeUpload: true,
          keyOwner: 'parent-owned-external-storage',
          proofRequirementRef: 'encryption-proof-approval',
        },
        parentActionRequired: true,
        rawChildEvidenceUploadedByDefault: false,
        ocentraHostedFamilyDataStored: false,
        claimSafe: true,
      },
      {
        itemId: 'manifest-item-registry',
        dataClass: 'device-registry-entry',
        exportFormat: 'encrypted-machine-readable',
        destinationOwnership: 'parent-owned-external-storage',
        schemaVersionLabel: 'registry.v1',
        encryption: {
          encryptionState: 'encrypted-at-rest',
          encryptedBeforeUpload: true,
          keyOwner: 'parent-owned-external-storage',
          proofRequirementRef: 'encryption-proof-registry',
        },
        parentActionRequired: true,
        rawChildEvidenceUploadedByDefault: false,
        ocentraHostedFamilyDataStored: false,
        claimSafe: true,
      },
      {
        itemId: 'manifest-item-notifications',
        dataClass: 'notification-history',
        exportFormat: 'encrypted-support-bundle',
        destinationOwnership: 'parent-owned-external-storage',
        schemaVersionLabel: 'notifications.v1',
        encryption: {
          encryptionState: 'encrypted-at-rest',
          encryptedBeforeUpload: true,
          keyOwner: 'parent-owned-external-storage',
          proofRequirementRef: 'encryption-proof-notifications',
        },
        parentActionRequired: true,
        rawChildEvidenceUploadedByDefault: false,
        ocentraHostedFamilyDataStored: false,
        claimSafe: true,
      },
      {
        itemId: 'manifest-item-audit',
        dataClass: 'audit-event',
        exportFormat: 'encrypted-machine-readable',
        destinationOwnership: 'parent-owned-external-storage',
        schemaVersionLabel: 'audit.v1',
        encryption: {
          encryptionState: 'encrypted-at-rest',
          encryptedBeforeUpload: true,
          keyOwner: 'parent-owned-external-storage',
          proofRequirementRef: 'encryption-proof-audit',
        },
        parentActionRequired: true,
        rawChildEvidenceUploadedByDefault: false,
        ocentraHostedFamilyDataStored: false,
        claimSafe: true,
      },
      {
        itemId: 'manifest-item-summary',
        dataClass: 'generated-summary',
        exportFormat: 'human-readable-parent-report',
        destinationOwnership: 'parent-owned-external-storage',
        schemaVersionLabel: 'summary.v1',
        encryption: {
          encryptionState: 'human-readable-parent-authorized',
          encryptedBeforeUpload: false,
          keyOwner: 'parent-owned-external-storage',
          proofRequirementRef: 'encryption-proof-summary',
        },
        parentActionRequired: true,
        rawChildEvidenceUploadedByDefault: false,
        ocentraHostedFamilyDataStored: false,
        claimSafe: true,
      },
    ],
  },
  providerStatuses: [
    {
      providerId: 'provider-google-drive-appdata',
      providerMode: 'google-drive-appdata',
      providerStatus: 'ready',
      destinationOwnership: 'parent-owned-external-storage',
      accountRef: 'account-google-drive-appdata',
      folderRef: 'folder-google-drive-appdata',
      statusRef: 'provider-status-google-drive-appdata-ready',
      revocationRef: null,
      disconnectVisibilityState: 'not-disconnected',
      deleteVisibilityState: 'not-requested',
      lastCheckedAt: '2026-06-28T18:40:00.000Z',
      oauthRuntimeClaimed: false,
      uploadRuntimeClaimed: false,
      deleteRuntimeClaimed: false,
      ocentraHostedFamilyDataStored: false,
      claimSafe: true,
    },
    {
      providerId: 'provider-google-drive-picker-file',
      providerMode: 'google-drive-picker-file',
      providerStatus: 'manual-required',
      destinationOwnership: 'parent-owned-external-storage',
      accountRef: 'account-google-drive-picker',
      folderRef: 'folder-google-drive-picker',
      statusRef: 'provider-status-google-drive-picker-file-manual-required',
      revocationRef: null,
      disconnectVisibilityState: 'manual-required',
      deleteVisibilityState: 'manual-required',
      lastCheckedAt: '2026-06-28T18:40:00.000Z',
      oauthRuntimeClaimed: false,
      uploadRuntimeClaimed: false,
      deleteRuntimeClaimed: false,
      ocentraHostedFamilyDataStored: false,
      claimSafe: true,
    },
    {
      providerId: 'provider-onedrive-approot',
      providerMode: 'onedrive-approot',
      providerStatus: 'revoked',
      destinationOwnership: 'parent-owned-external-storage',
      accountRef: 'account-onedrive-approot',
      folderRef: 'folder-onedrive-approot',
      statusRef: 'provider-status-onedrive-approot-revoked',
      revocationRef: 'revoked-onedrive-approot',
      disconnectVisibilityState: 'not-disconnected',
      deleteVisibilityState: 'not-requested',
      lastCheckedAt: '2026-06-28T18:40:00.000Z',
      oauthRuntimeClaimed: false,
      uploadRuntimeClaimed: false,
      deleteRuntimeClaimed: false,
      ocentraHostedFamilyDataStored: false,
      claimSafe: true,
    },
    {
      providerId: 'provider-onedrive-parent-selected-folder',
      providerMode: 'onedrive-parent-selected-folder',
      providerStatus: 'wrong-account',
      destinationOwnership: 'parent-owned-external-storage',
      accountRef: 'account-onedrive-selected',
      folderRef: 'folder-onedrive-selected',
      statusRef: 'provider-status-onedrive-parent-selected-folder-wrong-account',
      revocationRef: null,
      disconnectVisibilityState: 'not-disconnected',
      deleteVisibilityState: 'delete-visible',
      lastCheckedAt: '2026-06-28T18:40:00.000Z',
      oauthRuntimeClaimed: false,
      uploadRuntimeClaimed: false,
      deleteRuntimeClaimed: false,
      ocentraHostedFamilyDataStored: false,
      claimSafe: true,
    },
    {
      providerId: 'provider-icloud-drive-app-container',
      providerMode: 'icloud-drive-app-container',
      providerStatus: 'folder-unavailable',
      destinationOwnership: 'parent-owned-external-storage',
      accountRef: 'account-icloud-container',
      folderRef: 'folder-icloud-container',
      statusRef: 'provider-status-icloud-drive-app-container-folder-unavailable',
      revocationRef: null,
      disconnectVisibilityState: 'not-disconnected',
      deleteVisibilityState: 'delete-failed',
      lastCheckedAt: '2026-06-28T18:40:00.000Z',
      oauthRuntimeClaimed: false,
      uploadRuntimeClaimed: false,
      deleteRuntimeClaimed: false,
      ocentraHostedFamilyDataStored: false,
      claimSafe: true,
    },
    {
      providerId: 'provider-icloud-drive-parent-selected-location',
      providerMode: 'icloud-drive-parent-selected-location',
      providerStatus: 'disconnected',
      destinationOwnership: 'parent-owned-external-storage',
      accountRef: 'account-icloud-location',
      folderRef: 'folder-icloud-location',
      statusRef: 'provider-status-icloud-drive-parent-selected-location-disconnected',
      revocationRef: null,
      disconnectVisibilityState: 'disconnect-visible',
      deleteVisibilityState: 'not-requested',
      lastCheckedAt: '2026-06-28T18:40:00.000Z',
      oauthRuntimeClaimed: false,
      uploadRuntimeClaimed: false,
      deleteRuntimeClaimed: false,
      ocentraHostedFamilyDataStored: false,
      claimSafe: true,
    },
    {
      providerId: 'provider-dropbox-app-folder',
      providerMode: 'dropbox-app-folder',
      providerStatus: 'partial-upload',
      destinationOwnership: 'parent-owned-external-storage',
      accountRef: 'account-dropbox-app',
      folderRef: 'folder-dropbox-app',
      statusRef: 'provider-status-dropbox-app-folder-partial-upload',
      revocationRef: null,
      disconnectVisibilityState: 'not-disconnected',
      deleteVisibilityState: 'delete-confirmed',
      lastCheckedAt: '2026-06-28T18:40:00.000Z',
      oauthRuntimeClaimed: false,
      uploadRuntimeClaimed: false,
      deleteRuntimeClaimed: false,
      ocentraHostedFamilyDataStored: false,
      claimSafe: true,
    },
    {
      providerId: 'provider-dropbox-parent-selected-folder',
      providerMode: 'dropbox-parent-selected-folder',
      providerStatus: 'ready',
      destinationOwnership: 'parent-owned-external-storage',
      accountRef: 'account-dropbox-selected',
      folderRef: 'folder-dropbox-selected',
      statusRef: 'provider-status-dropbox-parent-selected-folder-ready',
      revocationRef: null,
      disconnectVisibilityState: 'not-disconnected',
      deleteVisibilityState: 'not-requested',
      lastCheckedAt: '2026-06-28T18:40:00.000Z',
      oauthRuntimeClaimed: false,
      uploadRuntimeClaimed: false,
      deleteRuntimeClaimed: false,
      ocentraHostedFamilyDataStored: false,
      claimSafe: true,
    },
    {
      providerId: 'provider-nas-folder',
      providerMode: 'nas-folder',
      providerStatus: 'ready',
      destinationOwnership: 'parent-owned-external-storage',
      accountRef: 'account-nas-folder',
      folderRef: 'folder-nas-folder',
      statusRef: 'provider-status-nas-folder-ready',
      revocationRef: null,
      disconnectVisibilityState: 'not-disconnected',
      deleteVisibilityState: 'not-requested',
      lastCheckedAt: '2026-06-28T18:40:00.000Z',
      oauthRuntimeClaimed: false,
      uploadRuntimeClaimed: false,
      deleteRuntimeClaimed: false,
      ocentraHostedFamilyDataStored: false,
      claimSafe: true,
    },
    {
      providerId: 'provider-local-folder',
      providerMode: 'local-folder',
      providerStatus: 'ready',
      destinationOwnership: 'parent-device-local',
      accountRef: 'account-local-folder',
      folderRef: 'folder-local-folder',
      statusRef: 'provider-status-local-folder-ready',
      revocationRef: null,
      disconnectVisibilityState: 'not-disconnected',
      deleteVisibilityState: 'not-requested',
      lastCheckedAt: '2026-06-28T18:40:00.000Z',
      oauthRuntimeClaimed: false,
      uploadRuntimeClaimed: false,
      deleteRuntimeClaimed: false,
      ocentraHostedFamilyDataStored: false,
      claimSafe: true,
    },
    {
      providerId: 'provider-disabled',
      providerMode: 'disabled',
      providerStatus: 'disabled',
      destinationOwnership: 'parent-device-local',
      accountRef: null,
      folderRef: null,
      statusRef: 'provider-status-disabled-disabled',
      revocationRef: null,
      disconnectVisibilityState: 'not-disconnected',
      deleteVisibilityState: 'not-requested',
      lastCheckedAt: '2026-06-28T18:40:00.000Z',
      oauthRuntimeClaimed: false,
      uploadRuntimeClaimed: false,
      deleteRuntimeClaimed: false,
      ocentraHostedFamilyDataStored: false,
      claimSafe: true,
    },
  ],
  syncStates: [
    {
      syncState: 'synced',
      providerStatusRef: 'provider-status-google-drive-appdata-ready',
      cursorRef: 'cursor-synced',
      batchRef: 'batch-synced',
      manifestIntegrityState: 'verified',
      manifestChecksumRef: 'checksum-synced',
      manifestSignatureRef: 'signature-synced',
      lastSuccessfulSyncAt: '2026-06-28T18:40:00.000Z',
      conflictRef: null,
      retryQueueRef: null,
      parentActionRequired: false,
      claimSafe: true,
    },
    {
      syncState: 'stale',
      providerStatusRef: 'provider-status-dropbox-parent-selected-folder-ready',
      cursorRef: 'cursor-stale',
      batchRef: 'batch-stale',
      manifestIntegrityState: 'verified',
      manifestChecksumRef: 'checksum-stale',
      manifestSignatureRef: 'signature-stale',
      lastSuccessfulSyncAt: '2026-06-28T18:20:00.000Z',
      conflictRef: null,
      retryQueueRef: null,
      parentActionRequired: false,
      claimSafe: true,
    },
    {
      syncState: 'missing',
      providerStatusRef: 'provider-status-local-folder-ready',
      cursorRef: null,
      batchRef: null,
      manifestIntegrityState: 'mismatch',
      manifestChecksumRef: null,
      manifestSignatureRef: null,
      lastSuccessfulSyncAt: null,
      conflictRef: null,
      retryQueueRef: 'retry-missing-manifest',
      parentActionRequired: true,
      claimSafe: true,
    },
    {
      syncState: 'conflict',
      providerStatusRef: 'provider-status-onedrive-parent-selected-folder-wrong-account',
      cursorRef: 'cursor-conflict',
      batchRef: 'batch-conflict',
      manifestIntegrityState: 'verified',
      manifestChecksumRef: 'checksum-conflict',
      manifestSignatureRef: 'signature-conflict',
      lastSuccessfulSyncAt: null,
      conflictRef: 'conflict-parent-owned-sync-1',
      retryQueueRef: 'retry-conflict',
      parentActionRequired: true,
      claimSafe: true,
    },
    {
      syncState: 'offline-retry-pending',
      providerStatusRef: 'provider-status-nas-folder-ready',
      cursorRef: 'cursor-offline-retry',
      batchRef: 'batch-offline-retry',
      manifestIntegrityState: 'verified',
      manifestChecksumRef: 'checksum-offline-retry',
      manifestSignatureRef: 'signature-offline-retry',
      lastSuccessfulSyncAt: '2026-06-28T18:10:00.000Z',
      conflictRef: null,
      retryQueueRef: 'retry-offline',
      parentActionRequired: false,
      claimSafe: true,
    },
    {
      syncState: 'partial-outage',
      providerStatusRef: 'provider-status-dropbox-app-folder-partial-upload',
      cursorRef: 'cursor-partial-outage',
      batchRef: 'batch-partial-outage',
      manifestIntegrityState: 'verified',
      manifestChecksumRef: 'checksum-partial-outage',
      manifestSignatureRef: 'signature-partial-outage',
      lastSuccessfulSyncAt: '2026-06-28T18:00:00.000Z',
      conflictRef: null,
      retryQueueRef: 'retry-partial-outage',
      parentActionRequired: false,
      claimSafe: true,
    },
    {
      syncState: 'manual-required',
      providerStatusRef: 'provider-status-google-drive-picker-file-manual-required',
      cursorRef: 'cursor-manual',
      batchRef: 'batch-manual',
      manifestIntegrityState: 'corrupt',
      manifestChecksumRef: 'checksum-manual',
      manifestSignatureRef: 'signature-manual',
      lastSuccessfulSyncAt: null,
      conflictRef: null,
      retryQueueRef: 'retry-manual-review',
      parentActionRequired: true,
      claimSafe: true,
    },
    {
      syncState: 'not-started',
      providerStatusRef: 'provider-status-disabled-disabled',
      cursorRef: null,
      batchRef: null,
      manifestIntegrityState: 'not-applicable',
      manifestChecksumRef: null,
      manifestSignatureRef: null,
      lastSuccessfulSyncAt: null,
      conflictRef: null,
      retryQueueRef: null,
      parentActionRequired: false,
      claimSafe: true,
    },
  ],
  tombstones: [
    {
      tombstoneRef: 'tombstone-none',
      dataClass: 'parent-rule',
      propagationState: 'not-requested',
      deleteRequestRef: null,
      providerStatusRef: 'provider-status-google-drive-appdata-ready',
      lastPropagatedAt: null,
      blockedReasonRef: null,
      claimSafe: true,
    },
    {
      tombstoneRef: 'tombstone-pending',
      dataClass: 'audit-event',
      propagationState: 'pending',
      deleteRequestRef: 'delete-request-audit',
      providerStatusRef: 'provider-status-dropbox-app-folder-partial-upload',
      lastPropagatedAt: null,
      blockedReasonRef: null,
      claimSafe: true,
    },
    {
      tombstoneRef: 'tombstone-propagated',
      dataClass: 'generated-summary',
      propagationState: 'propagated',
      deleteRequestRef: 'delete-request-summary',
      providerStatusRef: 'provider-status-onedrive-parent-selected-folder-wrong-account',
      lastPropagatedAt: '2026-06-28T18:40:00.000Z',
      blockedReasonRef: null,
      claimSafe: true,
    },
    {
      tombstoneRef: 'tombstone-blocked',
      dataClass: 'notification-history',
      propagationState: 'blocked',
      deleteRequestRef: 'delete-request-notification',
      providerStatusRef: 'provider-status-icloud-drive-app-container-folder-unavailable',
      lastPropagatedAt: null,
      blockedReasonRef: 'blocked-folder-unavailable',
      claimSafe: true,
    },
    {
      tombstoneRef: 'tombstone-manual',
      dataClass: 'device-registry-entry',
      propagationState: 'manual-required',
      deleteRequestRef: 'delete-request-device-registry',
      providerStatusRef: 'provider-status-google-drive-picker-file-manual-required',
      lastPropagatedAt: null,
      blockedReasonRef: 'manual-delete-confirmation-required',
      claimSafe: true,
    },
  ],
  nonClaims: [
    'no-transfer-runtime',
    'no-connector-oauth',
    'no-upload-runtime',
    'no-delete-runtime',
    'no-default-ocentra-custody',
    'no-raw-child-evidence-upload-by-default',
  ],
  transferRuntimeClaimed: false,
  connectorOAuthClaimed: false,
  uploadRuntimeClaimed: false,
  deleteRuntimeClaimed: false,
  ocentraHostedChildEvidenceStored: false,
  updatedAt: '2026-06-28T18:40:00.000Z',
} as const;
