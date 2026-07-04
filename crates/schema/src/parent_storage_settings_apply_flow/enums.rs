use serde::{Deserialize, Serialize};

use super::constants::*;

macro_rules! parent_storage_string_enum_as_str {
    ($name:ident { $($variant:ident => $value:expr),+ $(,)? }) => {
        impl $name {
            pub fn as_str(&self) -> &'static str {
                const VALUES: &[&str] = &[$($value),+];
                VALUES[*self as usize]
            }
        }
    };
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ParentStorageModeLabel {
    #[serde(rename = "local-only")]
    LocalOnly,
    #[serde(rename = "local-plus-encrypted-backup")]
    LocalPlusEncryptedBackup,
    #[serde(rename = "local-plus-encrypted-provider-sync")]
    LocalPlusEncryptedProviderSync,
    #[serde(rename = "provider-disconnected")]
    ProviderDisconnected,
    #[serde(rename = "provider-error")]
    ProviderError,
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "disabled")]
    Disabled,
}

parent_storage_string_enum_as_str!(ParentStorageModeLabel {
    LocalOnly => PARENT_STORAGE_MODE_LABEL_LOCAL_ONLY,
    LocalPlusEncryptedBackup => PARENT_STORAGE_MODE_LABEL_LOCAL_PLUS_ENCRYPTED_BACKUP,
    LocalPlusEncryptedProviderSync => PARENT_STORAGE_MODE_LABEL_LOCAL_PLUS_ENCRYPTED_PROVIDER_SYNC,
    ProviderDisconnected => PARENT_STORAGE_MODE_LABEL_PROVIDER_DISCONNECTED,
    ProviderError => PARENT_STORAGE_MODE_LABEL_PROVIDER_ERROR,
    ManualRequired => PARENT_STORAGE_MODE_LABEL_MANUAL_REQUIRED,
    Disabled => PARENT_STORAGE_MODE_LABEL_DISABLED,
});

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ParentStorageUiState {
    #[serde(rename = "providerNotConfigured")]
    ProviderNotConfigured,
    #[serde(rename = "providerAuthExpired")]
    ProviderAuthExpired,
    #[serde(rename = "providerPermissionMissing")]
    ProviderPermissionMissing,
    #[serde(rename = "providerRevoked")]
    ProviderRevoked,
    #[serde(rename = "providerQuotaExceeded")]
    ProviderQuotaExceeded,
    #[serde(rename = "providerUnavailable")]
    ProviderUnavailable,
    #[serde(rename = "localStoreUnavailable")]
    LocalStoreUnavailable,
    #[serde(rename = "keyUnavailable")]
    KeyUnavailable,
    #[serde(rename = "keyRevoked")]
    KeyRevoked,
    #[serde(rename = "wrongHousehold")]
    WrongHousehold,
    #[serde(rename = "wrongDevice")]
    WrongDevice,
    #[serde(rename = "schemaUnsupported")]
    SchemaUnsupported,
    #[serde(rename = "bundleCorrupt")]
    BundleCorrupt,
    #[serde(rename = "tombstoneConflict")]
    TombstoneConflict,
    #[serde(rename = "manualRequired")]
    ManualRequired,
    #[serde(rename = "offlineQueued")]
    OfflineQueued,
    #[serde(rename = "syncDisabled")]
    SyncDisabled,
    #[serde(rename = "remoteDisabled")]
    RemoteDisabled,
    #[serde(rename = "ocentraHostedStorageNotUsed")]
    OcentraHostedStorageNotUsed,
    #[serde(rename = "ready")]
    Ready,
}

parent_storage_string_enum_as_str!(ParentStorageUiState {
    ProviderNotConfigured => PARENT_STORAGE_UI_STATE_PROVIDER_NOT_CONFIGURED,
    ProviderAuthExpired => PARENT_STORAGE_UI_STATE_PROVIDER_AUTH_EXPIRED,
    ProviderPermissionMissing => PARENT_STORAGE_UI_STATE_PROVIDER_PERMISSION_MISSING,
    ProviderRevoked => PARENT_STORAGE_UI_STATE_PROVIDER_REVOKED,
    ProviderQuotaExceeded => PARENT_STORAGE_UI_STATE_PROVIDER_QUOTA_EXCEEDED,
    ProviderUnavailable => PARENT_STORAGE_UI_STATE_PROVIDER_UNAVAILABLE,
    LocalStoreUnavailable => PARENT_STORAGE_UI_STATE_LOCAL_STORE_UNAVAILABLE,
    KeyUnavailable => PARENT_STORAGE_UI_STATE_KEY_UNAVAILABLE,
    KeyRevoked => PARENT_STORAGE_UI_STATE_KEY_REVOKED,
    WrongHousehold => PARENT_STORAGE_UI_STATE_WRONG_HOUSEHOLD,
    WrongDevice => PARENT_STORAGE_UI_STATE_WRONG_DEVICE,
    SchemaUnsupported => PARENT_STORAGE_UI_STATE_SCHEMA_UNSUPPORTED,
    BundleCorrupt => PARENT_STORAGE_UI_STATE_BUNDLE_CORRUPT,
    TombstoneConflict => PARENT_STORAGE_UI_STATE_TOMBSTONE_CONFLICT,
    ManualRequired => PARENT_STORAGE_UI_STATE_MANUAL_REQUIRED,
    OfflineQueued => PARENT_STORAGE_UI_STATE_OFFLINE_QUEUED,
    SyncDisabled => PARENT_STORAGE_UI_STATE_SYNC_DISABLED,
    RemoteDisabled => PARENT_STORAGE_UI_STATE_REMOTE_DISABLED,
    OcentraHostedStorageNotUsed => PARENT_STORAGE_UI_STATE_OCENTRA_HOSTED_STORAGE_NOT_USED,
    Ready => PARENT_STORAGE_UI_STATE_READY,
});

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ParentStorageEncryptionStatus {
    #[serde(rename = "encrypted-before-upload")]
    EncryptedBeforeUpload,
    #[serde(rename = "human-readable-parent-authorized")]
    HumanReadableParentAuthorized,
    #[serde(rename = "not-applicable")]
    NotApplicable,
    #[serde(rename = "manual-required")]
    ManualRequired,
}

parent_storage_string_enum_as_str!(ParentStorageEncryptionStatus {
    EncryptedBeforeUpload => PARENT_STORAGE_ENCRYPTION_STATUS_ENCRYPTED_BEFORE_UPLOAD,
    HumanReadableParentAuthorized => PARENT_STORAGE_ENCRYPTION_STATUS_HUMAN_READABLE_PARENT_AUTHORIZED,
    NotApplicable => PARENT_STORAGE_ENCRYPTION_STATUS_NOT_APPLICABLE,
    ManualRequired => PARENT_STORAGE_ENCRYPTION_STATUS_MANUAL_REQUIRED,
});

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ParentStorageKeyStatus {
    #[serde(rename = "keyAvailable")]
    KeyAvailable,
    #[serde(rename = "keyUnavailable")]
    KeyUnavailable,
    #[serde(rename = "keyRevoked")]
    KeyRevoked,
    #[serde(rename = "manualRequired")]
    ManualRequired,
}

parent_storage_string_enum_as_str!(ParentStorageKeyStatus {
    KeyAvailable => PARENT_STORAGE_KEY_STATUS_KEY_AVAILABLE,
    KeyUnavailable => PARENT_STORAGE_KEY_STATUS_KEY_UNAVAILABLE,
    KeyRevoked => PARENT_STORAGE_KEY_STATUS_KEY_REVOKED,
    ManualRequired => PARENT_STORAGE_KEY_STATUS_MANUAL_REQUIRED,
});

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ParentStoragePreviewState {
    #[serde(rename = "importPreviewPassed")]
    ImportPreviewPassed,
    #[serde(rename = "partialRestore")]
    PartialRestore,
    #[serde(rename = "wrongHousehold")]
    WrongHousehold,
    #[serde(rename = "wrongKey")]
    WrongKey,
    #[serde(rename = "schemaUnsupported")]
    SchemaUnsupported,
    #[serde(rename = "bundleCorrupt")]
    BundleCorrupt,
    #[serde(rename = "tombstoneConflict")]
    TombstoneConflict,
    #[serde(rename = "manualRequired")]
    ManualRequired,
}

parent_storage_string_enum_as_str!(ParentStoragePreviewState {
    ImportPreviewPassed => PARENT_STORAGE_PREVIEW_STATE_IMPORT_PREVIEW_PASSED,
    PartialRestore => PARENT_STORAGE_PREVIEW_STATE_PARTIAL_RESTORE,
    WrongHousehold => PARENT_STORAGE_PREVIEW_STATE_WRONG_HOUSEHOLD,
    WrongKey => PARENT_STORAGE_PREVIEW_STATE_WRONG_KEY,
    SchemaUnsupported => PARENT_STORAGE_PREVIEW_STATE_SCHEMA_UNSUPPORTED,
    BundleCorrupt => PARENT_STORAGE_PREVIEW_STATE_BUNDLE_CORRUPT,
    TombstoneConflict => PARENT_STORAGE_PREVIEW_STATE_TOMBSTONE_CONFLICT,
    ManualRequired => PARENT_STORAGE_PREVIEW_STATE_MANUAL_REQUIRED,
});

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ParentStorageApplyState {
    #[serde(rename = "notStarted")]
    NotStarted,
    #[serde(rename = "applyRequiresConfirmation")]
    ApplyRequiresConfirmation,
    #[serde(rename = "applyPending")]
    ApplyPending,
    #[serde(rename = "applied")]
    Applied,
    #[serde(rename = "partial")]
    Partial,
    #[serde(rename = "rollbackManualRequired")]
    RollbackManualRequired,
    #[serde(rename = "blockedManualRequired")]
    BlockedManualRequired,
}

parent_storage_string_enum_as_str!(ParentStorageApplyState {
    NotStarted => PARENT_STORAGE_APPLY_STATE_NOT_STARTED,
    ApplyRequiresConfirmation => PARENT_STORAGE_APPLY_STATE_APPLY_REQUIRES_CONFIRMATION,
    ApplyPending => PARENT_STORAGE_APPLY_STATE_APPLY_PENDING,
    Applied => PARENT_STORAGE_APPLY_STATE_APPLIED,
    Partial => PARENT_STORAGE_APPLY_STATE_PARTIAL,
    RollbackManualRequired => PARENT_STORAGE_APPLY_STATE_ROLLBACK_MANUAL_REQUIRED,
    BlockedManualRequired => PARENT_STORAGE_APPLY_STATE_BLOCKED_MANUAL_REQUIRED,
});

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ParentStorageDeleteActionKind {
    #[serde(rename = "delete-local-child-evidence")]
    DeleteLocalChildEvidence,
    #[serde(rename = "delete-parent-portal-cache")]
    DeleteParentPortalCache,
    #[serde(rename = "delete-generated-report")]
    DeleteGeneratedReport,
    #[serde(rename = "delete-provider-backup-copy")]
    DeleteProviderBackupCopy,
    #[serde(rename = "delete-support-bundle")]
    DeleteSupportBundle,
    #[serde(rename = "delete-ocentra-metadata")]
    DeleteOcentraMetadata,
}

parent_storage_string_enum_as_str!(ParentStorageDeleteActionKind {
    DeleteLocalChildEvidence => PARENT_STORAGE_DELETE_ACTION_KIND_LOCAL_CHILD_EVIDENCE,
    DeleteParentPortalCache => PARENT_STORAGE_DELETE_ACTION_KIND_PARENT_PORTAL_CACHE,
    DeleteGeneratedReport => PARENT_STORAGE_DELETE_ACTION_KIND_GENERATED_REPORT,
    DeleteProviderBackupCopy => PARENT_STORAGE_DELETE_ACTION_KIND_PROVIDER_BACKUP_COPY,
    DeleteSupportBundle => PARENT_STORAGE_DELETE_ACTION_KIND_SUPPORT_BUNDLE,
    DeleteOcentraMetadata => PARENT_STORAGE_DELETE_ACTION_KIND_OCENTRA_METADATA,
});

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ParentStorageCopyKey {
    #[serde(rename = "custody-boundary")]
    CustodyBoundary,
    #[serde(rename = "metadata-leakage")]
    MetadataLeakage,
    #[serde(rename = "sensitive-encrypted-before-upload")]
    SensitiveEncryptedBeforeUpload,
    #[serde(rename = "lost-key-may-be-unrecoverable")]
    LostKeyMayBeUnrecoverable,
    #[serde(rename = "disconnect-does-not-delete")]
    DisconnectDoesNotDelete,
    #[serde(rename = "tombstones-may-be-required")]
    TombstonesMayBeRequired,
    #[serde(rename = "backup-queued")]
    BackupQueued,
    #[serde(rename = "provider-upload-pending")]
    ProviderUploadPending,
    #[serde(rename = "provider-upload-failed")]
    ProviderUploadFailed,
    #[serde(rename = "provider-upload-confirmed")]
    ProviderUploadConfirmed,
    #[serde(rename = "import-preview-passed")]
    ImportPreviewPassed,
    #[serde(rename = "apply-requires-confirmation")]
    ApplyRequiresConfirmation,
    #[serde(rename = "deleted-locally-provider-delete-pending")]
    DeletedLocallyProviderDeletePending,
    #[serde(rename = "provider-disconnected-existing-files-may-remain")]
    ProviderDisconnectedExistingFilesMayRemain,
    #[serde(rename = "manual-proof-required")]
    ManualProofRequired,
}

parent_storage_string_enum_as_str!(ParentStorageCopyKey {
    CustodyBoundary => PARENT_STORAGE_COPY_KEY_CUSTODY_BOUNDARY,
    MetadataLeakage => PARENT_STORAGE_COPY_KEY_METADATA_LEAKAGE,
    SensitiveEncryptedBeforeUpload => PARENT_STORAGE_COPY_KEY_SENSITIVE_ENCRYPTED_BEFORE_UPLOAD,
    LostKeyMayBeUnrecoverable => PARENT_STORAGE_COPY_KEY_LOST_KEY_MAY_BE_UNRECOVERABLE,
    DisconnectDoesNotDelete => PARENT_STORAGE_COPY_KEY_DISCONNECT_DOES_NOT_DELETE,
    TombstonesMayBeRequired => PARENT_STORAGE_COPY_KEY_TOMBSTONES_MAY_BE_REQUIRED,
    BackupQueued => PARENT_STORAGE_COPY_KEY_BACKUP_QUEUED,
    ProviderUploadPending => PARENT_STORAGE_COPY_KEY_PROVIDER_UPLOAD_PENDING,
    ProviderUploadFailed => PARENT_STORAGE_COPY_KEY_PROVIDER_UPLOAD_FAILED,
    ProviderUploadConfirmed => PARENT_STORAGE_COPY_KEY_PROVIDER_UPLOAD_CONFIRMED,
    ImportPreviewPassed => PARENT_STORAGE_COPY_KEY_IMPORT_PREVIEW_PASSED,
    ApplyRequiresConfirmation => PARENT_STORAGE_COPY_KEY_APPLY_REQUIRES_CONFIRMATION,
    DeletedLocallyProviderDeletePending => PARENT_STORAGE_COPY_KEY_DELETED_LOCALLY_PROVIDER_DELETE_PENDING,
    ProviderDisconnectedExistingFilesMayRemain => PARENT_STORAGE_COPY_KEY_PROVIDER_DISCONNECTED_EXISTING_FILES_MAY_REMAIN,
    ManualProofRequired => PARENT_STORAGE_COPY_KEY_MANUAL_PROOF_REQUIRED,
});

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ParentStorageNoClaim {
    #[serde(rename = "no-portal-implementation-ready")]
    NoPortalImplementationReady,
    #[serde(rename = "no-provider-runtime-ready")]
    NoProviderRuntimeReady,
    #[serde(rename = "no-auto-apply")]
    NoAutoApply,
    #[serde(rename = "no-disconnect-deletes-provider-data")]
    NoDisconnectDeletesProviderData,
    #[serde(rename = "no-delete-disconnect-collapse")]
    NoDeleteDisconnectCollapse,
    #[serde(rename = "no-ts-business-owner")]
    NoTsBusinessOwner,
    #[serde(rename = "no-lan-ownership")]
    NoLanOwnership,
}

parent_storage_string_enum_as_str!(ParentStorageNoClaim {
    NoPortalImplementationReady => PARENT_STORAGE_NO_CLAIM_PORTAL_IMPLEMENTATION_READY,
    NoProviderRuntimeReady => PARENT_STORAGE_NO_CLAIM_PROVIDER_RUNTIME_READY,
    NoAutoApply => PARENT_STORAGE_NO_CLAIM_AUTO_APPLY,
    NoDisconnectDeletesProviderData => PARENT_STORAGE_NO_CLAIM_DISCONNECT_DELETES_PROVIDER_DATA,
    NoDeleteDisconnectCollapse => PARENT_STORAGE_NO_CLAIM_DELETE_DISCONNECT_COLLAPSE,
    NoTsBusinessOwner => PARENT_STORAGE_NO_CLAIM_TS_BUSINESS_OWNER,
    NoLanOwnership => PARENT_STORAGE_NO_CLAIM_LAN_OWNERSHIP,
});
