/* generated from crates/schema/src/parent_storage_settings_apply_flow.rs */

import { Schema, brandedNonEmptyStringSchema } from './effect';

import type {
  GeneratedParentOwnedSyncDeleteVisibilityState,
  GeneratedParentOwnedSyncDisconnectVisibilityState,
  GeneratedParentOwnedSyncExportDataClass,
  GeneratedParentOwnedSyncProviderMode,
  GeneratedParentOwnedSyncProviderStatus,
  GeneratedParentOwnedSyncState,
} from './generated-parent-owned-sync-export-contracts';

export const ParentStorageSettingsApplyFlowContractRuntime = {
  SchemaVersion: 'parent-storage-settings-apply-flow-proof',
} as const;

export type GeneratedParentContractSchemaVersion = 'v0.6';
export const GeneratedParentStorageSettingsRowIdSchema = brandedNonEmptyStringSchema('ParentStorageSettingsRowId');
export const GeneratedParentStoragePreviewIdSchema = brandedNonEmptyStringSchema('ParentStoragePreviewId');
export const GeneratedParentStorageHouseholdRefSchema = brandedNonEmptyStringSchema('ParentStorageHouseholdRef');
export const GeneratedParentStorageApplyIntentDigestSchema = Schema.String.pipe(
  Schema.filter(
    (value) => /^[0-9a-f]{64}$/.test(value) || 'Expected a 64-character lowercase hexadecimal apply intent digest'
  ),
  Schema.brand('ParentStorageApplyIntentDigest')
);
export const GeneratedParentStorageApplyIdSchema = brandedNonEmptyStringSchema('ParentStorageApplyId');
export const GeneratedParentStorageActionIdSchema = brandedNonEmptyStringSchema('ParentStorageActionId');
export const GeneratedParentStorageTimestampSchema = brandedNonEmptyStringSchema('ParentTimestamp');

export type GeneratedParentStorageSettingsRowId = typeof GeneratedParentStorageSettingsRowIdSchema.Type;
export type GeneratedParentStoragePreviewId = typeof GeneratedParentStoragePreviewIdSchema.Type;
export type GeneratedParentStorageHouseholdRef = typeof GeneratedParentStorageHouseholdRefSchema.Type;
export type GeneratedParentStorageApplyIntentDigest = typeof GeneratedParentStorageApplyIntentDigestSchema.Type;
export type GeneratedParentStorageApplyId = typeof GeneratedParentStorageApplyIdSchema.Type;
export type GeneratedParentStorageActionId = typeof GeneratedParentStorageActionIdSchema.Type;
export type GeneratedParentStorageTimestamp = typeof GeneratedParentStorageTimestampSchema.Type;
export type GeneratedParentStorageModeLabel =
  | 'local-only'
  | 'local-plus-encrypted-backup'
  | 'local-plus-encrypted-provider-sync'
  | 'provider-disconnected'
  | 'provider-error'
  | 'manual-required'
  | 'disabled';
export type GeneratedParentStorageUiState =
  | 'providerNotConfigured'
  | 'providerAuthExpired'
  | 'providerPermissionMissing'
  | 'providerRevoked'
  | 'providerQuotaExceeded'
  | 'providerUnavailable'
  | 'localStoreUnavailable'
  | 'keyUnavailable'
  | 'keyRevoked'
  | 'wrongHousehold'
  | 'wrongDevice'
  | 'schemaUnsupported'
  | 'bundleCorrupt'
  | 'tombstoneConflict'
  | 'manualRequired'
  | 'offlineQueued'
  | 'syncDisabled'
  | 'remoteDisabled'
  | 'ocentraHostedStorageNotUsed'
  | 'ready';
export type GeneratedParentStorageEncryptionStatus =
  'encrypted-before-upload' | 'human-readable-parent-authorized' | 'not-applicable' | 'manual-required';
export type GeneratedParentStorageKeyStatus = 'keyAvailable' | 'keyUnavailable' | 'keyRevoked' | 'manualRequired';
export type GeneratedParentStoragePreviewState =
  | 'importPreviewPassed'
  | 'partialRestore'
  | 'wrongHousehold'
  | 'wrongKey'
  | 'schemaUnsupported'
  | 'bundleCorrupt'
  | 'tombstoneConflict'
  | 'manualRequired';
export type GeneratedParentStorageApplyState =
  | 'notStarted'
  | 'applyRequiresConfirmation'
  | 'applyPending'
  | 'applied'
  | 'partial'
  | 'rollbackManualRequired'
  | 'blockedManualRequired';
export type GeneratedParentStorageDeleteActionKind =
  | 'delete-local-child-evidence'
  | 'delete-parent-portal-cache'
  | 'delete-generated-report'
  | 'delete-provider-backup-copy'
  | 'delete-support-bundle'
  | 'delete-ocentra-metadata';
export type GeneratedParentStorageCopyKey =
  | 'custody-boundary'
  | 'metadata-leakage'
  | 'sensitive-encrypted-before-upload'
  | 'lost-key-may-be-unrecoverable'
  | 'disconnect-does-not-delete'
  | 'tombstones-may-be-required'
  | 'backup-queued'
  | 'provider-upload-pending'
  | 'provider-upload-failed'
  | 'provider-upload-confirmed'
  | 'import-preview-passed'
  | 'apply-requires-confirmation'
  | 'deleted-locally-provider-delete-pending'
  | 'provider-disconnected-existing-files-may-remain'
  | 'manual-proof-required';
export type GeneratedParentStorageNoClaim =
  | 'no-portal-implementation-ready'
  | 'no-provider-runtime-ready'
  | 'no-auto-apply'
  | 'no-disconnect-deletes-provider-data'
  | 'no-delete-disconnect-collapse'
  | 'no-ts-business-owner'
  | 'no-lan-ownership';

export const GeneratedParentStorageModeLabels = [
  'local-only',
  'local-plus-encrypted-backup',
  'local-plus-encrypted-provider-sync',
  'provider-disconnected',
  'provider-error',
  'manual-required',
  'disabled',
] as const satisfies readonly GeneratedParentStorageModeLabel[];
export const GeneratedParentStorageUiStates = [
  'providerNotConfigured',
  'providerAuthExpired',
  'providerPermissionMissing',
  'providerRevoked',
  'providerQuotaExceeded',
  'providerUnavailable',
  'localStoreUnavailable',
  'keyUnavailable',
  'keyRevoked',
  'wrongHousehold',
  'wrongDevice',
  'schemaUnsupported',
  'bundleCorrupt',
  'tombstoneConflict',
  'manualRequired',
  'offlineQueued',
  'syncDisabled',
  'remoteDisabled',
  'ocentraHostedStorageNotUsed',
  'ready',
] as const satisfies readonly GeneratedParentStorageUiState[];
export const GeneratedParentStorageEncryptionStatuses = [
  'encrypted-before-upload',
  'human-readable-parent-authorized',
  'not-applicable',
  'manual-required',
] as const satisfies readonly GeneratedParentStorageEncryptionStatus[];
export const GeneratedParentStorageKeyStatuses = [
  'keyAvailable',
  'keyUnavailable',
  'keyRevoked',
  'manualRequired',
] as const satisfies readonly GeneratedParentStorageKeyStatus[];
export const GeneratedParentStoragePreviewStates = [
  'importPreviewPassed',
  'partialRestore',
  'wrongHousehold',
  'wrongKey',
  'schemaUnsupported',
  'bundleCorrupt',
  'tombstoneConflict',
  'manualRequired',
] as const satisfies readonly GeneratedParentStoragePreviewState[];
export const GeneratedParentStorageApplyStates = [
  'notStarted',
  'applyRequiresConfirmation',
  'applyPending',
  'applied',
  'partial',
  'rollbackManualRequired',
  'blockedManualRequired',
] as const satisfies readonly GeneratedParentStorageApplyState[];
export const GeneratedParentStorageDeleteActionKinds = [
  'delete-local-child-evidence',
  'delete-parent-portal-cache',
  'delete-generated-report',
  'delete-provider-backup-copy',
  'delete-support-bundle',
  'delete-ocentra-metadata',
] as const satisfies readonly GeneratedParentStorageDeleteActionKind[];
export const GeneratedParentStorageCopyKeys = [
  'custody-boundary',
  'metadata-leakage',
  'sensitive-encrypted-before-upload',
  'lost-key-may-be-unrecoverable',
  'disconnect-does-not-delete',
  'tombstones-may-be-required',
  'backup-queued',
  'provider-upload-pending',
  'provider-upload-failed',
  'provider-upload-confirmed',
  'import-preview-passed',
  'apply-requires-confirmation',
  'deleted-locally-provider-delete-pending',
  'provider-disconnected-existing-files-may-remain',
  'manual-proof-required',
] as const satisfies readonly GeneratedParentStorageCopyKey[];
export const GeneratedParentStorageNoClaims = [
  'no-portal-implementation-ready',
  'no-provider-runtime-ready',
  'no-auto-apply',
  'no-disconnect-deletes-provider-data',
  'no-delete-disconnect-collapse',
  'no-ts-business-owner',
  'no-lan-ownership',
] as const satisfies readonly GeneratedParentStorageNoClaim[];

export interface GeneratedParentStorageModeCard {
  readonly rowId: GeneratedParentStorageSettingsRowId;
  readonly currentModeLabel: GeneratedParentStorageModeLabel;
  readonly uiState: GeneratedParentStorageUiState;
  readonly providerMode: GeneratedParentOwnedSyncProviderMode;
  readonly providerStatus: GeneratedParentOwnedSyncProviderStatus;
  readonly syncState: GeneratedParentOwnedSyncState;
  readonly encryptionStatus: GeneratedParentStorageEncryptionStatus;
  readonly keyStatus: GeneratedParentStorageKeyStatus;
  readonly manualRequiredVisible: boolean;
  readonly disconnectVisible: boolean;
  readonly deleteVisible: boolean;
  readonly restorePreviewAvailable: boolean;
  readonly applyBackAvailable: boolean;
  readonly lastSuccessAt: GeneratedParentStorageTimestamp | null;
  readonly lastFailureAt: GeneratedParentStorageTimestamp | null;
  readonly summary: string;
}

export interface GeneratedParentStorageRestorePreview {
  readonly previewId: GeneratedParentStoragePreviewId;
  readonly householdRef: GeneratedParentStorageHouseholdRef;
  readonly previewState: GeneratedParentStoragePreviewState;
  readonly createdAt: GeneratedParentStorageTimestamp;
  readonly productVersion: string;
  readonly schemaVersion: string;
  readonly householdMatch: boolean;
  readonly deviceMatch: boolean;
  readonly dataClasses: readonly GeneratedParentOwnedSyncExportDataClass[];
  readonly conflicts: readonly string[];
  readonly rejectedSections: readonly GeneratedParentOwnedSyncExportDataClass[];
  readonly partialRestore: boolean;
  readonly confirmationRequired: boolean;
  readonly localTruthAuthoritative: boolean;
  readonly tombstonesPreserved: boolean;
  readonly manualRequiredNote: string | null;
}

export interface GeneratedParentStorageApplyDecision {
  readonly applyId: GeneratedParentStorageApplyId;
  readonly applyIntentDigest: GeneratedParentStorageApplyIntentDigest;
  readonly applyState: GeneratedParentStorageApplyState;
  readonly confirmationRequired: boolean;
  readonly willChange: readonly GeneratedParentOwnedSyncExportDataClass[];
  readonly willNotChange: readonly GeneratedParentOwnedSyncExportDataClass[];
  readonly preservedTombstones: readonly GeneratedParentOwnedSyncExportDataClass[];
  readonly manualReviewRequired: readonly string[];
  readonly rollbackAvailable: boolean;
  readonly manualRequiredNote: string | null;
}

export interface GeneratedParentStorageDeleteActionRow {
  readonly actionId: GeneratedParentStorageActionId;
  readonly actionKind: GeneratedParentStorageDeleteActionKind;
  readonly state: GeneratedParentOwnedSyncDeleteVisibilityState;
  readonly separateFromDisconnect: boolean;
  readonly proofRequired: boolean;
  readonly notes: string;
}

export interface GeneratedParentStorageDisconnectRow {
  readonly actionId: GeneratedParentStorageActionId;
  readonly state: GeneratedParentOwnedSyncDisconnectVisibilityState;
  readonly existingFilesMayRemain: boolean;
  readonly providerDeleteRequestedSeparately: boolean;
  readonly notes: string;
}

export interface GeneratedParentStorageClaimSafeCopyRow {
  readonly copyKey: GeneratedParentStorageCopyKey;
  readonly statement: string;
  readonly forbiddenWithoutState: boolean;
  readonly notes: string;
}

export interface GeneratedParentStorageSettingsApplyFlowContractProof {
  readonly schemaVersion: typeof ParentStorageSettingsApplyFlowContractRuntime.SchemaVersion;
  readonly contractVersion: GeneratedParentContractSchemaVersion;
  readonly modeCard: GeneratedParentStorageModeCard;
  readonly restorePreview: GeneratedParentStorageRestorePreview;
  readonly applyDecision: GeneratedParentStorageApplyDecision;
  readonly deleteActions: readonly GeneratedParentStorageDeleteActionRow[];
  readonly disconnectAction: GeneratedParentStorageDisconnectRow;
  readonly claimSafeCopy: readonly GeneratedParentStorageClaimSafeCopyRow[];
  readonly noClaims: readonly GeneratedParentStorageNoClaim[];
  readonly updatedAt: GeneratedParentStorageTimestamp;
}

export const GeneratedParentStorageKnownGaps = [
  'Final portal rendering remains owned by portal-ux-household-surfaces-plan.',
  'Desktop host wiring remains owned by parent-client-runtime-distribution-plan.',
  'Provider SDK runtime remains unclaimed for this packet.',
  'Automatic provider delete or apply execution remains unclaimed for this packet.',
] as const;

export const GeneratedParentStorageSettingsApplyFlowContractProof = {
  schemaVersion: 'parent-storage-settings-apply-flow-proof',
  contractVersion: 'v0.6',
  modeCard: {
    rowId: 'parent-storage-settings-row-proof-1',
    currentModeLabel: 'manual-required',
    uiState: 'manualRequired',
    providerMode: 'google-drive-picker-file',
    providerStatus: 'manual-required',
    syncState: 'manual-required',
    encryptionStatus: 'encrypted-before-upload',
    keyStatus: 'manualRequired',
    manualRequiredVisible: true,
    disconnectVisible: false,
    deleteVisible: true,
    restorePreviewAvailable: true,
    applyBackAvailable: false,
    lastSuccessAt: '2026-06-28T19:10:00.000Z',
    lastFailureAt: '2026-06-28T19:12:00.000Z',
    summary: 'Manual proof required before provider-backed apply or delete proceeds.',
  },
  restorePreview: {
    previewId: 'restore-preview-proof-1',
    householdRef: 'household-proof-1',
    previewState: 'partialRestore',
    createdAt: '2026-06-28T19:14:00.000Z',
    productVersion: '2026.06.28',
    schemaVersion: 'export-import-backup-recovery-proof',
    householdMatch: true,
    deviceMatch: true,
    dataClasses: ['encrypted-journal-segment', 'generated-summary', 'notification-history'],
    conflicts: ['notification-history tombstone preserved'],
    rejectedSections: ['notification-history'],
    partialRestore: true,
    confirmationRequired: true,
    localTruthAuthoritative: true,
    tombstonesPreserved: true,
    manualRequiredNote: 'Manual review is required before any blocked section can be reconsidered.',
  },
  applyDecision: {
    applyId: 'apply-decision-proof-1',
    applyIntentDigest: 'd21a05fd04a3e1f8a18b8d4131683513b898d8642d38a65a55ce9d6cc30799f2',
    applyState: 'blockedManualRequired',
    confirmationRequired: true,
    willChange: ['encrypted-journal-segment', 'generated-summary'],
    willNotChange: ['notification-history'],
    preservedTombstones: ['notification-history'],
    manualReviewRequired: ['notification-history tombstone conflict'],
    rollbackAvailable: false,
    manualRequiredNote: 'Manual review is required before any blocked section can be reconsidered.',
  },
  deleteActions: [
    {
      actionId: 'delete-local-evidence',
      actionKind: 'delete-local-child-evidence',
      state: 'delete-visible',
      separateFromDisconnect: true,
      proofRequired: true,
      notes: 'Local delete remains separate from disconnect and provider delete.',
    },
    {
      actionId: 'delete-parent-cache',
      actionKind: 'delete-parent-portal-cache',
      state: 'delete-visible',
      separateFromDisconnect: true,
      proofRequired: true,
      notes: 'Parent cache delete is separate from evidence delete.',
    },
    {
      actionId: 'delete-generated-report',
      actionKind: 'delete-generated-report',
      state: 'delete-visible',
      separateFromDisconnect: true,
      proofRequired: true,
      notes: 'Generated report delete does not imply source evidence delete.',
    },
    {
      actionId: 'delete-provider-copy',
      actionKind: 'delete-provider-backup-copy',
      state: 'manual-required',
      separateFromDisconnect: true,
      proofRequired: true,
      notes: 'Provider delete remains manual-required until provider runtime proof exists.',
    },
    {
      actionId: 'delete-support-bundle',
      actionKind: 'delete-support-bundle',
      state: 'delete-visible',
      separateFromDisconnect: true,
      proofRequired: true,
      notes: 'Support bundle delete is separate from provider or local evidence delete.',
    },
    {
      actionId: 'delete-ocentra-metadata',
      actionKind: 'delete-ocentra-metadata',
      state: 'delete-visible',
      separateFromDisconnect: true,
      proofRequired: true,
      notes: 'Control-plane metadata delete remains a distinct parent action.',
    },
  ],
  disconnectAction: {
    actionId: 'disconnect-provider-proof-1',
    state: 'disconnect-visible',
    existingFilesMayRemain: true,
    providerDeleteRequestedSeparately: true,
    notes: 'Disconnect stops future sync only; existing provider files may remain.',
  },
  claimSafeCopy: [
    {
      copyKey: 'custody-boundary',
      statement: 'Ocentra does not store child activity data by default.',
      forbiddenWithoutState: true,
      notes: 'Claim-safe copy only; no success-looking shorthand without proof state.',
    },
    {
      copyKey: 'metadata-leakage',
      statement:
        'Your selected storage provider may see encrypted file metadata such as file name, size, and modified time.',
      forbiddenWithoutState: true,
      notes: 'Claim-safe copy only; no success-looking shorthand without proof state.',
    },
    {
      copyKey: 'sensitive-encrypted-before-upload',
      statement: 'Sensitive data is encrypted before upload.',
      forbiddenWithoutState: true,
      notes: 'Claim-safe copy only; no success-looking shorthand without proof state.',
    },
    {
      copyKey: 'lost-key-may-be-unrecoverable',
      statement:
        'If you lose your recovery key or device keys, Ocentra may not be able to recover encrypted child activity data.',
      forbiddenWithoutState: true,
      notes: 'Claim-safe copy only; no success-looking shorthand without proof state.',
    },
    {
      copyKey: 'disconnect-does-not-delete',
      statement:
        'Disconnecting a provider stops future sync but does not automatically delete files already written there unless you request deletion and proof succeeds.',
      forbiddenWithoutState: true,
      notes: 'Claim-safe copy only; no success-looking shorthand without proof state.',
    },
    {
      copyKey: 'tombstones-may-be-required',
      statement: 'Deleting local data may require tombstones so old backups do not restore deleted evidence.',
      forbiddenWithoutState: true,
      notes: 'Claim-safe copy only; no success-looking shorthand without proof state.',
    },
    {
      copyKey: 'backup-queued',
      statement: 'Backup queued.',
      forbiddenWithoutState: true,
      notes: 'Claim-safe copy only; no success-looking shorthand without proof state.',
    },
    {
      copyKey: 'provider-upload-pending',
      statement: 'Provider upload pending.',
      forbiddenWithoutState: true,
      notes: 'Claim-safe copy only; no success-looking shorthand without proof state.',
    },
    {
      copyKey: 'provider-upload-failed',
      statement: 'Provider upload failed.',
      forbiddenWithoutState: true,
      notes: 'Claim-safe copy only; no success-looking shorthand without proof state.',
    },
    {
      copyKey: 'provider-upload-confirmed',
      statement: 'Provider upload confirmed.',
      forbiddenWithoutState: true,
      notes: 'Claim-safe copy only; no success-looking shorthand without proof state.',
    },
    {
      copyKey: 'import-preview-passed',
      statement: 'Import preview passed.',
      forbiddenWithoutState: true,
      notes: 'Claim-safe copy only; no success-looking shorthand without proof state.',
    },
    {
      copyKey: 'apply-requires-confirmation',
      statement: 'Apply requires confirmation.',
      forbiddenWithoutState: true,
      notes: 'Claim-safe copy only; no success-looking shorthand without proof state.',
    },
    {
      copyKey: 'deleted-locally-provider-delete-pending',
      statement: 'Deleted locally; provider delete pending.',
      forbiddenWithoutState: true,
      notes: 'Claim-safe copy only; no success-looking shorthand without proof state.',
    },
    {
      copyKey: 'provider-disconnected-existing-files-may-remain',
      statement: 'Provider disconnected; existing files may remain.',
      forbiddenWithoutState: true,
      notes: 'Claim-safe copy only; no success-looking shorthand without proof state.',
    },
    {
      copyKey: 'manual-proof-required',
      statement: 'Manual proof required.',
      forbiddenWithoutState: true,
      notes: 'Claim-safe copy only; no success-looking shorthand without proof state.',
    },
  ],
  noClaims: [
    'no-portal-implementation-ready',
    'no-provider-runtime-ready',
    'no-auto-apply',
    'no-disconnect-deletes-provider-data',
    'no-delete-disconnect-collapse',
    'no-ts-business-owner',
    'no-lan-ownership',
  ],
  updatedAt: '2026-06-28T19:16:00.000Z',
} as const;
