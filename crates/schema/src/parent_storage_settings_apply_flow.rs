use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::parent_owned_sync_export::{
    ParentOwnedSyncDeleteVisibilityState, ParentOwnedSyncDisconnectVisibilityState,
    ParentOwnedSyncExportDataClass, ParentOwnedSyncProviderMode, ParentOwnedSyncProviderStatus,
    ParentOwnedSyncState,
};

pub const PARENT_STORAGE_SETTINGS_APPLY_FLOW_SCHEMA_VERSION: &str =
    "parent-storage-settings-apply-flow-proof";
const PARENT_STORAGE_CONTRACT_VERSION_VALUE: &str = "v0.6";
const PARENT_STORAGE_ROW_ID_VALUE: &str = "parent-storage-settings-row-proof-1";
const PARENT_STORAGE_LAST_SUCCESS_AT_VALUE: &str = "2026-06-28T19:10:00.000Z";
const PARENT_STORAGE_LAST_FAILURE_AT_VALUE: &str = "2026-06-28T19:12:00.000Z";
const PARENT_STORAGE_RESTORE_PREVIEW_ID_VALUE: &str = "restore-preview-proof-1";
const PARENT_STORAGE_RESTORE_PREVIEW_CREATED_AT_VALUE: &str = "2026-06-28T19:14:00.000Z";
const PARENT_STORAGE_PRODUCT_VERSION_VALUE: &str = "2026.06.28";
const PARENT_STORAGE_EXPORT_SCHEMA_VERSION_VALUE: &str = "export-import-backup-recovery-proof";
const PARENT_STORAGE_APPLY_DECISION_ID_VALUE: &str = "apply-decision-proof-1";
const PARENT_STORAGE_DELETE_LOCAL_EVIDENCE_ACTION_ID: &str = "delete-local-evidence";
const PARENT_STORAGE_DELETE_PARENT_CACHE_ACTION_ID: &str = "delete-parent-cache";
const PARENT_STORAGE_DELETE_GENERATED_REPORT_ACTION_ID: &str = "delete-generated-report";
const PARENT_STORAGE_DELETE_PROVIDER_COPY_ACTION_ID: &str = "delete-provider-copy";
const PARENT_STORAGE_DELETE_SUPPORT_BUNDLE_ACTION_ID: &str = "delete-support-bundle";
const PARENT_STORAGE_DELETE_OCENTRA_METADATA_ACTION_ID: &str = "delete-ocentra-metadata";
const PARENT_STORAGE_DISCONNECT_ACTION_ID: &str = "disconnect-provider-proof-1";
const PARENT_STORAGE_UPDATED_AT_VALUE: &str = "2026-06-28T19:16:00.000Z";
const PARENT_STORAGE_CONFLICT_NOTIFICATION_HISTORY_TOMBSTONE_PRESERVED: &str =
    "notification-history tombstone preserved";
const PARENT_STORAGE_CONFLICT_NOTIFICATION_HISTORY_TOMBSTONE_CONFLICT: &str =
    "notification-history tombstone conflict";
const PARENT_STORAGE_SUMMARY_MANUAL_PROOF_REQUIRED: &str =
    "Manual proof required before provider-backed apply or delete proceeds.";
const PARENT_STORAGE_MANUAL_REVIEW_REQUIRED_NOTE: &str =
    "Manual review is required before any blocked section can be reconsidered.";
const PARENT_STORAGE_DELETE_LOCAL_EVIDENCE_NOTE: &str =
    "Local delete remains separate from disconnect and provider delete.";
const PARENT_STORAGE_DELETE_PARENT_CACHE_NOTE: &str =
    "Parent cache delete is separate from evidence delete.";
const PARENT_STORAGE_DELETE_GENERATED_REPORT_NOTE: &str =
    "Generated report delete does not imply source evidence delete.";
const PARENT_STORAGE_DELETE_PROVIDER_COPY_NOTE: &str =
    "Provider delete remains manual-required until provider runtime proof exists.";
const PARENT_STORAGE_DELETE_SUPPORT_BUNDLE_NOTE: &str =
    "Support bundle delete is separate from provider or local evidence delete.";
const PARENT_STORAGE_DELETE_OCENTRA_METADATA_NOTE: &str =
    "Control-plane metadata delete remains a distinct parent action.";
const PARENT_STORAGE_DISCONNECT_NOTE: &str =
    "Disconnect stops future sync only; existing provider files may remain.";
const PARENT_STORAGE_CLAIM_SAFE_COPY_NOTE: &str =
    "Claim-safe copy only; no success-looking shorthand without proof state.";
const PARENT_STORAGE_COPY_CUSTODY_BOUNDARY_STATEMENT: &str =
    "Ocentra does not store child activity data by default.";
const PARENT_STORAGE_COPY_METADATA_LEAKAGE_STATEMENT: &str = "Your selected storage provider may see encrypted file metadata such as file name, size, and modified time.";
const PARENT_STORAGE_COPY_SENSITIVE_ENCRYPTED_BEFORE_UPLOAD_STATEMENT: &str =
    "Sensitive data is encrypted before upload.";
const PARENT_STORAGE_COPY_LOST_KEY_MAY_BE_UNRECOVERABLE_STATEMENT: &str = "If you lose your recovery key or device keys, Ocentra may not be able to recover encrypted child activity data.";
const PARENT_STORAGE_COPY_DISCONNECT_DOES_NOT_DELETE_STATEMENT: &str = "Disconnecting a provider stops future sync but does not automatically delete files already written there unless you request deletion and proof succeeds.";
const PARENT_STORAGE_COPY_TOMBSTONES_MAY_BE_REQUIRED_STATEMENT: &str =
    "Deleting local data may require tombstones so old backups do not restore deleted evidence.";
const PARENT_STORAGE_COPY_BACKUP_QUEUED_STATEMENT: &str = "Backup queued.";
const PARENT_STORAGE_COPY_PROVIDER_UPLOAD_PENDING_STATEMENT: &str = "Provider upload pending.";
const PARENT_STORAGE_COPY_PROVIDER_UPLOAD_FAILED_STATEMENT: &str = "Provider upload failed.";
const PARENT_STORAGE_COPY_PROVIDER_UPLOAD_CONFIRMED_STATEMENT: &str = "Provider upload confirmed.";
const PARENT_STORAGE_COPY_IMPORT_PREVIEW_PASSED_STATEMENT: &str = "Import preview passed.";
const PARENT_STORAGE_COPY_APPLY_REQUIRES_CONFIRMATION_STATEMENT: &str =
    "Apply requires confirmation.";
const PARENT_STORAGE_COPY_DELETED_LOCALLY_PROVIDER_DELETE_PENDING_STATEMENT: &str =
    "Deleted locally; provider delete pending.";
const PARENT_STORAGE_COPY_PROVIDER_DISCONNECTED_EXISTING_FILES_MAY_REMAIN_STATEMENT: &str =
    "Provider disconnected; existing files may remain.";
const PARENT_STORAGE_COPY_MANUAL_PROOF_REQUIRED_STATEMENT: &str = "Manual proof required.";
const PARENT_STORAGE_KNOWN_GAP_FINAL_PORTAL_RENDERING: &str =
    "Final portal rendering remains owned by portal-ux-household-surfaces-plan.";
const PARENT_STORAGE_KNOWN_GAP_DESKTOP_HOST_WIRING: &str =
    "Desktop host wiring remains owned by parent-client-runtime-distribution-plan.";
const PARENT_STORAGE_KNOWN_GAP_PROVIDER_SDK_RUNTIME: &str =
    "Provider SDK runtime remains unclaimed for this packet.";
const PARENT_STORAGE_KNOWN_GAP_AUTOMATIC_PROVIDER_DELETE_OR_APPLY: &str =
    "Automatic provider delete or apply execution remains unclaimed for this packet.";
const PARENT_STORAGE_EXPECT_CONTRACT_VERSION: &str = "contract version";
const PARENT_STORAGE_EXPECT_ROW_ID: &str = "row id";
const PARENT_STORAGE_EXPECT_PREVIEW_ID: &str = "preview id";
const PARENT_STORAGE_EXPECT_APPLY_ID: &str = "apply id";
const PARENT_STORAGE_EXPECT_ACTION_ID: &str = "action id";
const PARENT_STORAGE_EXPECT_TIMESTAMP: &str = "timestamp";

const PARENT_STORAGE_MODE_LABEL_LOCAL_ONLY: &str = "local-only";
const PARENT_STORAGE_MODE_LABEL_LOCAL_PLUS_ENCRYPTED_BACKUP: &str = "local-plus-encrypted-backup";
const PARENT_STORAGE_MODE_LABEL_LOCAL_PLUS_ENCRYPTED_PROVIDER_SYNC: &str =
    "local-plus-encrypted-provider-sync";
const PARENT_STORAGE_MODE_LABEL_PROVIDER_DISCONNECTED: &str = "provider-disconnected";
const PARENT_STORAGE_MODE_LABEL_PROVIDER_ERROR: &str = "provider-error";
const PARENT_STORAGE_MODE_LABEL_MANUAL_REQUIRED: &str = "manual-required";
const PARENT_STORAGE_MODE_LABEL_DISABLED: &str = "disabled";

const PARENT_STORAGE_UI_STATE_PROVIDER_NOT_CONFIGURED: &str = "providerNotConfigured";
const PARENT_STORAGE_UI_STATE_PROVIDER_AUTH_EXPIRED: &str = "providerAuthExpired";
const PARENT_STORAGE_UI_STATE_PROVIDER_PERMISSION_MISSING: &str = "providerPermissionMissing";
const PARENT_STORAGE_UI_STATE_PROVIDER_REVOKED: &str = "providerRevoked";
const PARENT_STORAGE_UI_STATE_PROVIDER_QUOTA_EXCEEDED: &str = "providerQuotaExceeded";
const PARENT_STORAGE_UI_STATE_PROVIDER_UNAVAILABLE: &str = "providerUnavailable";
const PARENT_STORAGE_UI_STATE_LOCAL_STORE_UNAVAILABLE: &str = "localStoreUnavailable";
const PARENT_STORAGE_UI_STATE_KEY_UNAVAILABLE: &str = "keyUnavailable";
const PARENT_STORAGE_UI_STATE_KEY_REVOKED: &str = "keyRevoked";
const PARENT_STORAGE_UI_STATE_WRONG_HOUSEHOLD: &str = "wrongHousehold";
const PARENT_STORAGE_UI_STATE_WRONG_DEVICE: &str = "wrongDevice";
const PARENT_STORAGE_UI_STATE_SCHEMA_UNSUPPORTED: &str = "schemaUnsupported";
const PARENT_STORAGE_UI_STATE_BUNDLE_CORRUPT: &str = "bundleCorrupt";
const PARENT_STORAGE_UI_STATE_TOMBSTONE_CONFLICT: &str = "tombstoneConflict";
const PARENT_STORAGE_UI_STATE_MANUAL_REQUIRED: &str = "manualRequired";
const PARENT_STORAGE_UI_STATE_OFFLINE_QUEUED: &str = "offlineQueued";
const PARENT_STORAGE_UI_STATE_SYNC_DISABLED: &str = "syncDisabled";
const PARENT_STORAGE_UI_STATE_REMOTE_DISABLED: &str = "remoteDisabled";
const PARENT_STORAGE_UI_STATE_OCENTRA_HOSTED_STORAGE_NOT_USED: &str = "ocentraHostedStorageNotUsed";
const PARENT_STORAGE_UI_STATE_READY: &str = "ready";

const PARENT_STORAGE_ENCRYPTION_STATUS_ENCRYPTED_BEFORE_UPLOAD: &str = "encrypted-before-upload";
const PARENT_STORAGE_ENCRYPTION_STATUS_HUMAN_READABLE_PARENT_AUTHORIZED: &str =
    "human-readable-parent-authorized";
const PARENT_STORAGE_ENCRYPTION_STATUS_NOT_APPLICABLE: &str = "not-applicable";
const PARENT_STORAGE_ENCRYPTION_STATUS_MANUAL_REQUIRED: &str = "manual-required";

const PARENT_STORAGE_KEY_STATUS_KEY_AVAILABLE: &str = "keyAvailable";
const PARENT_STORAGE_KEY_STATUS_KEY_UNAVAILABLE: &str = "keyUnavailable";
const PARENT_STORAGE_KEY_STATUS_KEY_REVOKED: &str = "keyRevoked";
const PARENT_STORAGE_KEY_STATUS_MANUAL_REQUIRED: &str = "manualRequired";

const PARENT_STORAGE_PREVIEW_STATE_IMPORT_PREVIEW_PASSED: &str = "importPreviewPassed";
const PARENT_STORAGE_PREVIEW_STATE_PARTIAL_RESTORE: &str = "partialRestore";
const PARENT_STORAGE_PREVIEW_STATE_WRONG_HOUSEHOLD: &str = "wrongHousehold";
const PARENT_STORAGE_PREVIEW_STATE_WRONG_KEY: &str = "wrongKey";
const PARENT_STORAGE_PREVIEW_STATE_SCHEMA_UNSUPPORTED: &str = "schemaUnsupported";
const PARENT_STORAGE_PREVIEW_STATE_BUNDLE_CORRUPT: &str = "bundleCorrupt";
const PARENT_STORAGE_PREVIEW_STATE_TOMBSTONE_CONFLICT: &str = "tombstoneConflict";
const PARENT_STORAGE_PREVIEW_STATE_MANUAL_REQUIRED: &str = "manualRequired";

const PARENT_STORAGE_APPLY_STATE_NOT_STARTED: &str = "notStarted";
const PARENT_STORAGE_APPLY_STATE_APPLY_REQUIRES_CONFIRMATION: &str = "applyRequiresConfirmation";
const PARENT_STORAGE_APPLY_STATE_APPLY_PENDING: &str = "applyPending";
const PARENT_STORAGE_APPLY_STATE_APPLIED: &str = "applied";
const PARENT_STORAGE_APPLY_STATE_PARTIAL: &str = "partial";
const PARENT_STORAGE_APPLY_STATE_ROLLBACK_MANUAL_REQUIRED: &str = "rollbackManualRequired";
const PARENT_STORAGE_APPLY_STATE_BLOCKED_MANUAL_REQUIRED: &str = "blockedManualRequired";

const PARENT_STORAGE_DELETE_ACTION_KIND_LOCAL_CHILD_EVIDENCE: &str = "delete-local-child-evidence";
const PARENT_STORAGE_DELETE_ACTION_KIND_PARENT_PORTAL_CACHE: &str = "delete-parent-portal-cache";
const PARENT_STORAGE_DELETE_ACTION_KIND_GENERATED_REPORT: &str = "delete-generated-report";
const PARENT_STORAGE_DELETE_ACTION_KIND_PROVIDER_BACKUP_COPY: &str = "delete-provider-backup-copy";
const PARENT_STORAGE_DELETE_ACTION_KIND_SUPPORT_BUNDLE: &str = "delete-support-bundle";
const PARENT_STORAGE_DELETE_ACTION_KIND_OCENTRA_METADATA: &str = "delete-ocentra-metadata";

const PARENT_STORAGE_COPY_KEY_CUSTODY_BOUNDARY: &str = "custody-boundary";
const PARENT_STORAGE_COPY_KEY_METADATA_LEAKAGE: &str = "metadata-leakage";
const PARENT_STORAGE_COPY_KEY_SENSITIVE_ENCRYPTED_BEFORE_UPLOAD: &str =
    "sensitive-encrypted-before-upload";
const PARENT_STORAGE_COPY_KEY_LOST_KEY_MAY_BE_UNRECOVERABLE: &str = "lost-key-may-be-unrecoverable";
const PARENT_STORAGE_COPY_KEY_DISCONNECT_DOES_NOT_DELETE: &str = "disconnect-does-not-delete";
const PARENT_STORAGE_COPY_KEY_TOMBSTONES_MAY_BE_REQUIRED: &str = "tombstones-may-be-required";
const PARENT_STORAGE_COPY_KEY_BACKUP_QUEUED: &str = "backup-queued";
const PARENT_STORAGE_COPY_KEY_PROVIDER_UPLOAD_PENDING: &str = "provider-upload-pending";
const PARENT_STORAGE_COPY_KEY_PROVIDER_UPLOAD_FAILED: &str = "provider-upload-failed";
const PARENT_STORAGE_COPY_KEY_PROVIDER_UPLOAD_CONFIRMED: &str = "provider-upload-confirmed";
const PARENT_STORAGE_COPY_KEY_IMPORT_PREVIEW_PASSED: &str = "import-preview-passed";
const PARENT_STORAGE_COPY_KEY_APPLY_REQUIRES_CONFIRMATION: &str = "apply-requires-confirmation";
const PARENT_STORAGE_COPY_KEY_DELETED_LOCALLY_PROVIDER_DELETE_PENDING: &str =
    "deleted-locally-provider-delete-pending";
const PARENT_STORAGE_COPY_KEY_PROVIDER_DISCONNECTED_EXISTING_FILES_MAY_REMAIN: &str =
    "provider-disconnected-existing-files-may-remain";
const PARENT_STORAGE_COPY_KEY_MANUAL_PROOF_REQUIRED: &str = "manual-proof-required";

const PARENT_STORAGE_NO_CLAIM_PORTAL_IMPLEMENTATION_READY: &str = "no-portal-implementation-ready";
const PARENT_STORAGE_NO_CLAIM_PROVIDER_RUNTIME_READY: &str = "no-provider-runtime-ready";
const PARENT_STORAGE_NO_CLAIM_AUTO_APPLY: &str = "no-auto-apply";
const PARENT_STORAGE_NO_CLAIM_DISCONNECT_DELETES_PROVIDER_DATA: &str =
    "no-disconnect-deletes-provider-data";
const PARENT_STORAGE_NO_CLAIM_DELETE_DISCONNECT_COLLAPSE: &str = "no-delete-disconnect-collapse";
const PARENT_STORAGE_NO_CLAIM_TS_BUSINESS_OWNER: &str = "no-ts-business-owner";
const PARENT_STORAGE_NO_CLAIM_LAN_OWNERSHIP: &str = "no-lan-ownership";

macro_rules! parent_storage_text_identifier {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            pub fn parse(value: impl Into<String>) -> Option<Self> {
                let value = value.into();
                if value.trim().is_empty() {
                    None
                } else {
                    Some(Self(value))
                }
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl Display for $name {
            fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(self.as_str())
            }
        }
    };
}

parent_storage_text_identifier!(ParentStorageContractVersion);
parent_storage_text_identifier!(ParentStorageSettingsRowId);
parent_storage_text_identifier!(ParentStoragePreviewId);
parent_storage_text_identifier!(ParentStorageApplyId);
parent_storage_text_identifier!(ParentStorageActionId);
parent_storage_text_identifier!(ParentStorageTimestamp);

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

impl ParentStorageModeLabel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::LocalOnly => PARENT_STORAGE_MODE_LABEL_LOCAL_ONLY,
            Self::LocalPlusEncryptedBackup => PARENT_STORAGE_MODE_LABEL_LOCAL_PLUS_ENCRYPTED_BACKUP,
            Self::LocalPlusEncryptedProviderSync => {
                PARENT_STORAGE_MODE_LABEL_LOCAL_PLUS_ENCRYPTED_PROVIDER_SYNC
            }
            Self::ProviderDisconnected => PARENT_STORAGE_MODE_LABEL_PROVIDER_DISCONNECTED,
            Self::ProviderError => PARENT_STORAGE_MODE_LABEL_PROVIDER_ERROR,
            Self::ManualRequired => PARENT_STORAGE_MODE_LABEL_MANUAL_REQUIRED,
            Self::Disabled => PARENT_STORAGE_MODE_LABEL_DISABLED,
        }
    }
}

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

impl ParentStorageUiState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ProviderNotConfigured => PARENT_STORAGE_UI_STATE_PROVIDER_NOT_CONFIGURED,
            Self::ProviderAuthExpired => PARENT_STORAGE_UI_STATE_PROVIDER_AUTH_EXPIRED,
            Self::ProviderPermissionMissing => PARENT_STORAGE_UI_STATE_PROVIDER_PERMISSION_MISSING,
            Self::ProviderRevoked => PARENT_STORAGE_UI_STATE_PROVIDER_REVOKED,
            Self::ProviderQuotaExceeded => PARENT_STORAGE_UI_STATE_PROVIDER_QUOTA_EXCEEDED,
            Self::ProviderUnavailable => PARENT_STORAGE_UI_STATE_PROVIDER_UNAVAILABLE,
            Self::LocalStoreUnavailable => PARENT_STORAGE_UI_STATE_LOCAL_STORE_UNAVAILABLE,
            Self::KeyUnavailable => PARENT_STORAGE_UI_STATE_KEY_UNAVAILABLE,
            Self::KeyRevoked => PARENT_STORAGE_UI_STATE_KEY_REVOKED,
            Self::WrongHousehold => PARENT_STORAGE_UI_STATE_WRONG_HOUSEHOLD,
            Self::WrongDevice => PARENT_STORAGE_UI_STATE_WRONG_DEVICE,
            Self::SchemaUnsupported => PARENT_STORAGE_UI_STATE_SCHEMA_UNSUPPORTED,
            Self::BundleCorrupt => PARENT_STORAGE_UI_STATE_BUNDLE_CORRUPT,
            Self::TombstoneConflict => PARENT_STORAGE_UI_STATE_TOMBSTONE_CONFLICT,
            Self::ManualRequired => PARENT_STORAGE_UI_STATE_MANUAL_REQUIRED,
            Self::OfflineQueued => PARENT_STORAGE_UI_STATE_OFFLINE_QUEUED,
            Self::SyncDisabled => PARENT_STORAGE_UI_STATE_SYNC_DISABLED,
            Self::RemoteDisabled => PARENT_STORAGE_UI_STATE_REMOTE_DISABLED,
            Self::OcentraHostedStorageNotUsed => {
                PARENT_STORAGE_UI_STATE_OCENTRA_HOSTED_STORAGE_NOT_USED
            }
            Self::Ready => PARENT_STORAGE_UI_STATE_READY,
        }
    }
}

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

impl ParentStorageEncryptionStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::EncryptedBeforeUpload => PARENT_STORAGE_ENCRYPTION_STATUS_ENCRYPTED_BEFORE_UPLOAD,
            Self::HumanReadableParentAuthorized => {
                PARENT_STORAGE_ENCRYPTION_STATUS_HUMAN_READABLE_PARENT_AUTHORIZED
            }
            Self::NotApplicable => PARENT_STORAGE_ENCRYPTION_STATUS_NOT_APPLICABLE,
            Self::ManualRequired => PARENT_STORAGE_ENCRYPTION_STATUS_MANUAL_REQUIRED,
        }
    }
}

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

impl ParentStorageKeyStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::KeyAvailable => PARENT_STORAGE_KEY_STATUS_KEY_AVAILABLE,
            Self::KeyUnavailable => PARENT_STORAGE_KEY_STATUS_KEY_UNAVAILABLE,
            Self::KeyRevoked => PARENT_STORAGE_KEY_STATUS_KEY_REVOKED,
            Self::ManualRequired => PARENT_STORAGE_KEY_STATUS_MANUAL_REQUIRED,
        }
    }
}

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

impl ParentStoragePreviewState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ImportPreviewPassed => PARENT_STORAGE_PREVIEW_STATE_IMPORT_PREVIEW_PASSED,
            Self::PartialRestore => PARENT_STORAGE_PREVIEW_STATE_PARTIAL_RESTORE,
            Self::WrongHousehold => PARENT_STORAGE_PREVIEW_STATE_WRONG_HOUSEHOLD,
            Self::WrongKey => PARENT_STORAGE_PREVIEW_STATE_WRONG_KEY,
            Self::SchemaUnsupported => PARENT_STORAGE_PREVIEW_STATE_SCHEMA_UNSUPPORTED,
            Self::BundleCorrupt => PARENT_STORAGE_PREVIEW_STATE_BUNDLE_CORRUPT,
            Self::TombstoneConflict => PARENT_STORAGE_PREVIEW_STATE_TOMBSTONE_CONFLICT,
            Self::ManualRequired => PARENT_STORAGE_PREVIEW_STATE_MANUAL_REQUIRED,
        }
    }
}

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

impl ParentStorageApplyState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NotStarted => PARENT_STORAGE_APPLY_STATE_NOT_STARTED,
            Self::ApplyRequiresConfirmation => {
                PARENT_STORAGE_APPLY_STATE_APPLY_REQUIRES_CONFIRMATION
            }
            Self::ApplyPending => PARENT_STORAGE_APPLY_STATE_APPLY_PENDING,
            Self::Applied => PARENT_STORAGE_APPLY_STATE_APPLIED,
            Self::Partial => PARENT_STORAGE_APPLY_STATE_PARTIAL,
            Self::RollbackManualRequired => PARENT_STORAGE_APPLY_STATE_ROLLBACK_MANUAL_REQUIRED,
            Self::BlockedManualRequired => PARENT_STORAGE_APPLY_STATE_BLOCKED_MANUAL_REQUIRED,
        }
    }
}

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

impl ParentStorageDeleteActionKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DeleteLocalChildEvidence => {
                PARENT_STORAGE_DELETE_ACTION_KIND_LOCAL_CHILD_EVIDENCE
            }
            Self::DeleteParentPortalCache => PARENT_STORAGE_DELETE_ACTION_KIND_PARENT_PORTAL_CACHE,
            Self::DeleteGeneratedReport => PARENT_STORAGE_DELETE_ACTION_KIND_GENERATED_REPORT,
            Self::DeleteProviderBackupCopy => {
                PARENT_STORAGE_DELETE_ACTION_KIND_PROVIDER_BACKUP_COPY
            }
            Self::DeleteSupportBundle => PARENT_STORAGE_DELETE_ACTION_KIND_SUPPORT_BUNDLE,
            Self::DeleteOcentraMetadata => PARENT_STORAGE_DELETE_ACTION_KIND_OCENTRA_METADATA,
        }
    }
}

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

impl ParentStorageCopyKey {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CustodyBoundary => PARENT_STORAGE_COPY_KEY_CUSTODY_BOUNDARY,
            Self::MetadataLeakage => PARENT_STORAGE_COPY_KEY_METADATA_LEAKAGE,
            Self::SensitiveEncryptedBeforeUpload => {
                PARENT_STORAGE_COPY_KEY_SENSITIVE_ENCRYPTED_BEFORE_UPLOAD
            }
            Self::LostKeyMayBeUnrecoverable => {
                PARENT_STORAGE_COPY_KEY_LOST_KEY_MAY_BE_UNRECOVERABLE
            }
            Self::DisconnectDoesNotDelete => PARENT_STORAGE_COPY_KEY_DISCONNECT_DOES_NOT_DELETE,
            Self::TombstonesMayBeRequired => PARENT_STORAGE_COPY_KEY_TOMBSTONES_MAY_BE_REQUIRED,
            Self::BackupQueued => PARENT_STORAGE_COPY_KEY_BACKUP_QUEUED,
            Self::ProviderUploadPending => PARENT_STORAGE_COPY_KEY_PROVIDER_UPLOAD_PENDING,
            Self::ProviderUploadFailed => PARENT_STORAGE_COPY_KEY_PROVIDER_UPLOAD_FAILED,
            Self::ProviderUploadConfirmed => PARENT_STORAGE_COPY_KEY_PROVIDER_UPLOAD_CONFIRMED,
            Self::ImportPreviewPassed => PARENT_STORAGE_COPY_KEY_IMPORT_PREVIEW_PASSED,
            Self::ApplyRequiresConfirmation => PARENT_STORAGE_COPY_KEY_APPLY_REQUIRES_CONFIRMATION,
            Self::DeletedLocallyProviderDeletePending => {
                PARENT_STORAGE_COPY_KEY_DELETED_LOCALLY_PROVIDER_DELETE_PENDING
            }
            Self::ProviderDisconnectedExistingFilesMayRemain => {
                PARENT_STORAGE_COPY_KEY_PROVIDER_DISCONNECTED_EXISTING_FILES_MAY_REMAIN
            }
            Self::ManualProofRequired => PARENT_STORAGE_COPY_KEY_MANUAL_PROOF_REQUIRED,
        }
    }
}

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

impl ParentStorageNoClaim {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NoPortalImplementationReady => {
                PARENT_STORAGE_NO_CLAIM_PORTAL_IMPLEMENTATION_READY
            }
            Self::NoProviderRuntimeReady => PARENT_STORAGE_NO_CLAIM_PROVIDER_RUNTIME_READY,
            Self::NoAutoApply => PARENT_STORAGE_NO_CLAIM_AUTO_APPLY,
            Self::NoDisconnectDeletesProviderData => {
                PARENT_STORAGE_NO_CLAIM_DISCONNECT_DELETES_PROVIDER_DATA
            }
            Self::NoDeleteDisconnectCollapse => PARENT_STORAGE_NO_CLAIM_DELETE_DISCONNECT_COLLAPSE,
            Self::NoTsBusinessOwner => PARENT_STORAGE_NO_CLAIM_TS_BUSINESS_OWNER,
            Self::NoLanOwnership => PARENT_STORAGE_NO_CLAIM_LAN_OWNERSHIP,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentStorageModeCard {
    pub row_id: ParentStorageSettingsRowId,
    pub current_mode_label: ParentStorageModeLabel,
    pub ui_state: ParentStorageUiState,
    pub provider_mode: ParentOwnedSyncProviderMode,
    pub provider_status: ParentOwnedSyncProviderStatus,
    pub sync_state: ParentOwnedSyncState,
    pub encryption_status: ParentStorageEncryptionStatus,
    pub key_status: ParentStorageKeyStatus,
    pub manual_required_visible: bool,
    pub disconnect_visible: bool,
    pub delete_visible: bool,
    pub restore_preview_available: bool,
    pub apply_back_available: bool,
    pub last_success_at: Option<ParentStorageTimestamp>,
    pub last_failure_at: Option<ParentStorageTimestamp>,
    pub summary: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentStorageRestorePreview {
    pub preview_id: ParentStoragePreviewId,
    pub preview_state: ParentStoragePreviewState,
    pub created_at: ParentStorageTimestamp,
    pub product_version: String,
    pub schema_version: String,
    pub household_match: bool,
    pub device_match: bool,
    pub data_classes: Vec<ParentOwnedSyncExportDataClass>,
    pub conflicts: Vec<String>,
    pub rejected_sections: Vec<ParentOwnedSyncExportDataClass>,
    pub partial_restore: bool,
    pub confirmation_required: bool,
    pub local_truth_authoritative: bool,
    pub tombstones_preserved: bool,
    pub manual_required_note: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentStorageApplyDecision {
    pub apply_id: ParentStorageApplyId,
    pub apply_state: ParentStorageApplyState,
    pub confirmation_required: bool,
    pub will_change: Vec<ParentOwnedSyncExportDataClass>,
    pub will_not_change: Vec<ParentOwnedSyncExportDataClass>,
    pub preserved_tombstones: Vec<ParentOwnedSyncExportDataClass>,
    pub manual_review_required: Vec<String>,
    pub rollback_available: bool,
    pub manual_required_note: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentStorageDeleteActionRow {
    pub action_id: ParentStorageActionId,
    pub action_kind: ParentStorageDeleteActionKind,
    pub state: ParentOwnedSyncDeleteVisibilityState,
    pub separate_from_disconnect: bool,
    pub proof_required: bool,
    pub notes: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentStorageDisconnectRow {
    pub action_id: ParentStorageActionId,
    pub state: ParentOwnedSyncDisconnectVisibilityState,
    pub existing_files_may_remain: bool,
    pub provider_delete_requested_separately: bool,
    pub notes: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentStorageClaimSafeCopyRow {
    pub copy_key: ParentStorageCopyKey,
    pub statement: String,
    pub forbidden_without_state: bool,
    pub notes: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentStorageSettingsApplyFlowContractProof {
    pub schema_version: String,
    pub contract_version: ParentStorageContractVersion,
    pub mode_card: ParentStorageModeCard,
    pub restore_preview: ParentStorageRestorePreview,
    pub apply_decision: ParentStorageApplyDecision,
    pub delete_actions: Vec<ParentStorageDeleteActionRow>,
    pub disconnect_action: ParentStorageDisconnectRow,
    pub claim_safe_copy: Vec<ParentStorageClaimSafeCopyRow>,
    pub no_claims: Vec<ParentStorageNoClaim>,
    pub updated_at: ParentStorageTimestamp,
}

pub fn required_parent_storage_mode_labels() -> Vec<ParentStorageModeLabel> {
    vec![
        ParentStorageModeLabel::LocalOnly,
        ParentStorageModeLabel::LocalPlusEncryptedBackup,
        ParentStorageModeLabel::LocalPlusEncryptedProviderSync,
        ParentStorageModeLabel::ProviderDisconnected,
        ParentStorageModeLabel::ProviderError,
        ParentStorageModeLabel::ManualRequired,
        ParentStorageModeLabel::Disabled,
    ]
}

pub fn required_parent_storage_delete_action_kinds() -> Vec<ParentStorageDeleteActionKind> {
    vec![
        ParentStorageDeleteActionKind::DeleteLocalChildEvidence,
        ParentStorageDeleteActionKind::DeleteParentPortalCache,
        ParentStorageDeleteActionKind::DeleteGeneratedReport,
        ParentStorageDeleteActionKind::DeleteProviderBackupCopy,
        ParentStorageDeleteActionKind::DeleteSupportBundle,
        ParentStorageDeleteActionKind::DeleteOcentraMetadata,
    ]
}

pub fn required_parent_storage_copy_keys() -> Vec<ParentStorageCopyKey> {
    vec![
        ParentStorageCopyKey::CustodyBoundary,
        ParentStorageCopyKey::MetadataLeakage,
        ParentStorageCopyKey::SensitiveEncryptedBeforeUpload,
        ParentStorageCopyKey::LostKeyMayBeUnrecoverable,
        ParentStorageCopyKey::DisconnectDoesNotDelete,
        ParentStorageCopyKey::TombstonesMayBeRequired,
        ParentStorageCopyKey::BackupQueued,
        ParentStorageCopyKey::ProviderUploadPending,
        ParentStorageCopyKey::ProviderUploadFailed,
        ParentStorageCopyKey::ProviderUploadConfirmed,
        ParentStorageCopyKey::ImportPreviewPassed,
        ParentStorageCopyKey::ApplyRequiresConfirmation,
        ParentStorageCopyKey::DeletedLocallyProviderDeletePending,
        ParentStorageCopyKey::ProviderDisconnectedExistingFilesMayRemain,
        ParentStorageCopyKey::ManualProofRequired,
    ]
}

pub fn required_parent_storage_no_claims() -> Vec<ParentStorageNoClaim> {
    vec![
        ParentStorageNoClaim::NoPortalImplementationReady,
        ParentStorageNoClaim::NoProviderRuntimeReady,
        ParentStorageNoClaim::NoAutoApply,
        ParentStorageNoClaim::NoDisconnectDeletesProviderData,
        ParentStorageNoClaim::NoDeleteDisconnectCollapse,
        ParentStorageNoClaim::NoTsBusinessOwner,
        ParentStorageNoClaim::NoLanOwnership,
    ]
}

pub fn parent_storage_settings_apply_flow_known_gaps() -> [&'static str; 4] {
    [
        PARENT_STORAGE_KNOWN_GAP_FINAL_PORTAL_RENDERING,
        PARENT_STORAGE_KNOWN_GAP_DESKTOP_HOST_WIRING,
        PARENT_STORAGE_KNOWN_GAP_PROVIDER_SDK_RUNTIME,
        PARENT_STORAGE_KNOWN_GAP_AUTOMATIC_PROVIDER_DELETE_OR_APPLY,
    ]
}

pub fn sample_parent_storage_settings_apply_flow_contract_proof(
) -> ParentStorageSettingsApplyFlowContractProof {
    ParentStorageSettingsApplyFlowContractProof {
        schema_version: owned_text(PARENT_STORAGE_SETTINGS_APPLY_FLOW_SCHEMA_VERSION),
        contract_version: contract_version(PARENT_STORAGE_CONTRACT_VERSION_VALUE),
        mode_card: ParentStorageModeCard {
            row_id: row_id(PARENT_STORAGE_ROW_ID_VALUE),
            current_mode_label: ParentStorageModeLabel::ManualRequired,
            ui_state: ParentStorageUiState::ManualRequired,
            provider_mode: ParentOwnedSyncProviderMode::GoogleDrivePickerFile,
            provider_status: ParentOwnedSyncProviderStatus::ManualRequired,
            sync_state: ParentOwnedSyncState::ManualRequired,
            encryption_status: ParentStorageEncryptionStatus::EncryptedBeforeUpload,
            key_status: ParentStorageKeyStatus::ManualRequired,
            manual_required_visible: true,
            disconnect_visible: false,
            delete_visible: true,
            restore_preview_available: true,
            apply_back_available: false,
            last_success_at: Some(timestamp(PARENT_STORAGE_LAST_SUCCESS_AT_VALUE)),
            last_failure_at: Some(timestamp(PARENT_STORAGE_LAST_FAILURE_AT_VALUE)),
            summary: owned_text(PARENT_STORAGE_SUMMARY_MANUAL_PROOF_REQUIRED),
        },
        restore_preview: ParentStorageRestorePreview {
            preview_id: preview_id(PARENT_STORAGE_RESTORE_PREVIEW_ID_VALUE),
            preview_state: ParentStoragePreviewState::PartialRestore,
            created_at: timestamp(PARENT_STORAGE_RESTORE_PREVIEW_CREATED_AT_VALUE),
            product_version: owned_text(PARENT_STORAGE_PRODUCT_VERSION_VALUE),
            schema_version: owned_text(PARENT_STORAGE_EXPORT_SCHEMA_VERSION_VALUE),
            household_match: true,
            device_match: true,
            data_classes: vec![
                ParentOwnedSyncExportDataClass::EncryptedJournalSegment,
                ParentOwnedSyncExportDataClass::GeneratedSummary,
                ParentOwnedSyncExportDataClass::NotificationHistory,
            ],
            conflicts: vec![owned_text(
                PARENT_STORAGE_CONFLICT_NOTIFICATION_HISTORY_TOMBSTONE_PRESERVED,
            )],
            rejected_sections: vec![ParentOwnedSyncExportDataClass::NotificationHistory],
            partial_restore: true,
            confirmation_required: true,
            local_truth_authoritative: true,
            tombstones_preserved: true,
            manual_required_note: Some(owned_text(PARENT_STORAGE_MANUAL_REVIEW_REQUIRED_NOTE)),
        },
        apply_decision: ParentStorageApplyDecision {
            apply_id: apply_id(PARENT_STORAGE_APPLY_DECISION_ID_VALUE),
            apply_state: ParentStorageApplyState::ApplyRequiresConfirmation,
            confirmation_required: true,
            will_change: vec![
                ParentOwnedSyncExportDataClass::EncryptedJournalSegment,
                ParentOwnedSyncExportDataClass::GeneratedSummary,
            ],
            will_not_change: vec![ParentOwnedSyncExportDataClass::NotificationHistory],
            preserved_tombstones: vec![ParentOwnedSyncExportDataClass::NotificationHistory],
            manual_review_required: vec![owned_text(
                PARENT_STORAGE_CONFLICT_NOTIFICATION_HISTORY_TOMBSTONE_CONFLICT,
            )],
            rollback_available: false,
            manual_required_note: None,
        },
        delete_actions: vec![
            delete_action(
                PARENT_STORAGE_DELETE_LOCAL_EVIDENCE_ACTION_ID,
                ParentStorageDeleteActionKind::DeleteLocalChildEvidence,
                ParentOwnedSyncDeleteVisibilityState::DeleteVisible,
                PARENT_STORAGE_DELETE_LOCAL_EVIDENCE_NOTE,
            ),
            delete_action(
                PARENT_STORAGE_DELETE_PARENT_CACHE_ACTION_ID,
                ParentStorageDeleteActionKind::DeleteParentPortalCache,
                ParentOwnedSyncDeleteVisibilityState::DeleteVisible,
                PARENT_STORAGE_DELETE_PARENT_CACHE_NOTE,
            ),
            delete_action(
                PARENT_STORAGE_DELETE_GENERATED_REPORT_ACTION_ID,
                ParentStorageDeleteActionKind::DeleteGeneratedReport,
                ParentOwnedSyncDeleteVisibilityState::DeleteVisible,
                PARENT_STORAGE_DELETE_GENERATED_REPORT_NOTE,
            ),
            delete_action(
                PARENT_STORAGE_DELETE_PROVIDER_COPY_ACTION_ID,
                ParentStorageDeleteActionKind::DeleteProviderBackupCopy,
                ParentOwnedSyncDeleteVisibilityState::ManualRequired,
                PARENT_STORAGE_DELETE_PROVIDER_COPY_NOTE,
            ),
            delete_action(
                PARENT_STORAGE_DELETE_SUPPORT_BUNDLE_ACTION_ID,
                ParentStorageDeleteActionKind::DeleteSupportBundle,
                ParentOwnedSyncDeleteVisibilityState::DeleteVisible,
                PARENT_STORAGE_DELETE_SUPPORT_BUNDLE_NOTE,
            ),
            delete_action(
                PARENT_STORAGE_DELETE_OCENTRA_METADATA_ACTION_ID,
                ParentStorageDeleteActionKind::DeleteOcentraMetadata,
                ParentOwnedSyncDeleteVisibilityState::DeleteVisible,
                PARENT_STORAGE_DELETE_OCENTRA_METADATA_NOTE,
            ),
        ],
        disconnect_action: ParentStorageDisconnectRow {
            action_id: action_id(PARENT_STORAGE_DISCONNECT_ACTION_ID),
            state: ParentOwnedSyncDisconnectVisibilityState::DisconnectVisible,
            existing_files_may_remain: true,
            provider_delete_requested_separately: true,
            notes: owned_text(PARENT_STORAGE_DISCONNECT_NOTE),
        },
        claim_safe_copy: sample_claim_safe_copy(),
        no_claims: required_parent_storage_no_claims(),
        updated_at: timestamp(PARENT_STORAGE_UPDATED_AT_VALUE),
    }
}

fn sample_claim_safe_copy() -> Vec<ParentStorageClaimSafeCopyRow> {
    vec![
        copy_row(
            ParentStorageCopyKey::CustodyBoundary,
            PARENT_STORAGE_COPY_CUSTODY_BOUNDARY_STATEMENT,
        ),
        copy_row(
            ParentStorageCopyKey::MetadataLeakage,
            PARENT_STORAGE_COPY_METADATA_LEAKAGE_STATEMENT,
        ),
        copy_row(
            ParentStorageCopyKey::SensitiveEncryptedBeforeUpload,
            PARENT_STORAGE_COPY_SENSITIVE_ENCRYPTED_BEFORE_UPLOAD_STATEMENT,
        ),
        copy_row(
            ParentStorageCopyKey::LostKeyMayBeUnrecoverable,
            PARENT_STORAGE_COPY_LOST_KEY_MAY_BE_UNRECOVERABLE_STATEMENT,
        ),
        copy_row(
            ParentStorageCopyKey::DisconnectDoesNotDelete,
            PARENT_STORAGE_COPY_DISCONNECT_DOES_NOT_DELETE_STATEMENT,
        ),
        copy_row(
            ParentStorageCopyKey::TombstonesMayBeRequired,
            PARENT_STORAGE_COPY_TOMBSTONES_MAY_BE_REQUIRED_STATEMENT,
        ),
        copy_row(
            ParentStorageCopyKey::BackupQueued,
            PARENT_STORAGE_COPY_BACKUP_QUEUED_STATEMENT,
        ),
        copy_row(
            ParentStorageCopyKey::ProviderUploadPending,
            PARENT_STORAGE_COPY_PROVIDER_UPLOAD_PENDING_STATEMENT,
        ),
        copy_row(
            ParentStorageCopyKey::ProviderUploadFailed,
            PARENT_STORAGE_COPY_PROVIDER_UPLOAD_FAILED_STATEMENT,
        ),
        copy_row(
            ParentStorageCopyKey::ProviderUploadConfirmed,
            PARENT_STORAGE_COPY_PROVIDER_UPLOAD_CONFIRMED_STATEMENT,
        ),
        copy_row(
            ParentStorageCopyKey::ImportPreviewPassed,
            PARENT_STORAGE_COPY_IMPORT_PREVIEW_PASSED_STATEMENT,
        ),
        copy_row(
            ParentStorageCopyKey::ApplyRequiresConfirmation,
            PARENT_STORAGE_COPY_APPLY_REQUIRES_CONFIRMATION_STATEMENT,
        ),
        copy_row(
            ParentStorageCopyKey::DeletedLocallyProviderDeletePending,
            PARENT_STORAGE_COPY_DELETED_LOCALLY_PROVIDER_DELETE_PENDING_STATEMENT,
        ),
        copy_row(
            ParentStorageCopyKey::ProviderDisconnectedExistingFilesMayRemain,
            PARENT_STORAGE_COPY_PROVIDER_DISCONNECTED_EXISTING_FILES_MAY_REMAIN_STATEMENT,
        ),
        copy_row(
            ParentStorageCopyKey::ManualProofRequired,
            PARENT_STORAGE_COPY_MANUAL_PROOF_REQUIRED_STATEMENT,
        ),
    ]
}

fn delete_action(
    value: &str,
    action_kind: ParentStorageDeleteActionKind,
    state: ParentOwnedSyncDeleteVisibilityState,
    notes: &str,
) -> ParentStorageDeleteActionRow {
    ParentStorageDeleteActionRow {
        action_id: action_id(value),
        action_kind,
        state,
        separate_from_disconnect: true,
        proof_required: true,
        notes: owned_text(notes),
    }
}

fn copy_row(copy_key: ParentStorageCopyKey, statement: &str) -> ParentStorageClaimSafeCopyRow {
    ParentStorageClaimSafeCopyRow {
        copy_key,
        statement: owned_text(statement),
        forbidden_without_state: true,
        notes: owned_text(PARENT_STORAGE_CLAIM_SAFE_COPY_NOTE),
    }
}

fn owned_text(value: &str) -> String {
    value.to_owned()
}

fn contract_version(value: &str) -> ParentStorageContractVersion {
    crate::schema_option_or_unreachable(
        ParentStorageContractVersion::parse(value),
        PARENT_STORAGE_EXPECT_CONTRACT_VERSION,
    )
}

fn row_id(value: &str) -> ParentStorageSettingsRowId {
    crate::schema_option_or_unreachable(
        ParentStorageSettingsRowId::parse(value),
        PARENT_STORAGE_EXPECT_ROW_ID,
    )
}

fn preview_id(value: &str) -> ParentStoragePreviewId {
    crate::schema_option_or_unreachable(
        ParentStoragePreviewId::parse(value),
        PARENT_STORAGE_EXPECT_PREVIEW_ID,
    )
}

fn apply_id(value: &str) -> ParentStorageApplyId {
    crate::schema_option_or_unreachable(
        ParentStorageApplyId::parse(value),
        PARENT_STORAGE_EXPECT_APPLY_ID,
    )
}

fn action_id(value: &str) -> ParentStorageActionId {
    crate::schema_option_or_unreachable(
        ParentStorageActionId::parse(value),
        PARENT_STORAGE_EXPECT_ACTION_ID,
    )
}

fn timestamp(value: &str) -> ParentStorageTimestamp {
    crate::schema_option_or_unreachable(
        ParentStorageTimestamp::parse(value),
        PARENT_STORAGE_EXPECT_TIMESTAMP,
    )
}
