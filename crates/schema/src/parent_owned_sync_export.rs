use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

mod identifiers;
mod manifest;
mod providers;
#[macro_use]
mod macros;
mod sample;
mod sync_states;
mod tombstones;

pub const PARENT_OWNED_SYNC_EXPORT_SCHEMA_VERSION: &str = "parent-owned-sync-export-manifest-proof";
const PROVIDER_MODE_GOOGLE_DRIVE_APPDATA: &str = "google-drive-appdata";
const PROVIDER_MODE_GOOGLE_DRIVE_PICKER_FILE: &str = "google-drive-picker-file";
const PROVIDER_MODE_ONEDRIVE_APPROOT: &str = "onedrive-approot";
const PROVIDER_MODE_ONEDRIVE_PARENT_SELECTED_FOLDER: &str = "onedrive-parent-selected-folder";
const PROVIDER_MODE_ICLOUD_DRIVE_APP_CONTAINER: &str = "icloud-drive-app-container";
const PROVIDER_MODE_ICLOUD_DRIVE_PARENT_SELECTED_LOCATION: &str =
    "icloud-drive-parent-selected-location";
const PROVIDER_MODE_DROPBOX_APP_FOLDER: &str = "dropbox-app-folder";
const PROVIDER_MODE_DROPBOX_PARENT_SELECTED_FOLDER: &str = "dropbox-parent-selected-folder";
const PROVIDER_MODE_NAS_FOLDER: &str = "nas-folder";
const PROVIDER_MODE_LOCAL_FOLDER: &str = "local-folder";
const PROVIDER_MODE_DISABLED: &str = "disabled";
const PROVIDER_STATUS_READY: &str = "ready";
const PROVIDER_STATUS_MANUAL_REQUIRED: &str = "manual-required";
const PROVIDER_STATUS_REVOKED: &str = "revoked";
const PROVIDER_STATUS_WRONG_ACCOUNT: &str = "wrong-account";
const PROVIDER_STATUS_FOLDER_UNAVAILABLE: &str = "folder-unavailable";
const PROVIDER_STATUS_PARTIAL_UPLOAD: &str = "partial-upload";
const PROVIDER_STATUS_DISCONNECTED: &str = "disconnected";
const PROVIDER_STATUS_DISABLED: &str = "disabled";
const PROVIDER_STATUS_NOT_CONFIGURED: &str = "not-configured";
const CONTRACT_VERSION_EXPECT_MESSAGE: &str = "contract version";
const FAMILY_ID_EXPECT_MESSAGE: &str = "family id";
const CHILD_PROFILE_ID_EXPECT_MESSAGE: &str = "child profile id";
const PARENT_DEVICE_ID_EXPECT_MESSAGE: &str = "device id";
const PARENT_DEVICE_LABEL_EXPECT_MESSAGE: &str = "device label";
const PARENT_ACTOR_ID_EXPECT_MESSAGE: &str = "actor id";
const PARENT_POLICY_VERSION_EXPECT_MESSAGE: &str = "policy version";
const PARENT_ACTION_ID_EXPECT_MESSAGE: &str = "parent action id";
const PARENT_TIMESTAMP_EXPECT_MESSAGE: &str = "parent timestamp";
const MANIFEST_ID_EXPECT_MESSAGE: &str = "manifest id";
const ITEM_ID_EXPECT_MESSAGE: &str = "item id";
const VERSION_LABEL_EXPECT_MESSAGE: &str = "version label";
const POLICY_REF_EXPECT_MESSAGE: &str = "policy ref";
const PROVIDER_ID_EXPECT_MESSAGE: &str = "provider id";
const PROVIDER_REF_EXPECT_MESSAGE: &str = "provider ref";
const STATUS_REF_EXPECT_MESSAGE: &str = "status ref";
const CURSOR_REF_EXPECT_MESSAGE: &str = "cursor ref";
const BATCH_REF_EXPECT_MESSAGE: &str = "batch ref";
const CONFLICT_REF_EXPECT_MESSAGE: &str = "conflict ref";
const CHECKSUM_REF_EXPECT_MESSAGE: &str = "checksum ref";
const SIGNATURE_REF_EXPECT_MESSAGE: &str = "signature ref";
const TOMBSTONE_REF_EXPECT_MESSAGE: &str = "tombstone ref";
const DELETE_REQUEST_REF_EXPECT_MESSAGE: &str = "delete request ref";
const KNOWN_GAP_NO_PROVIDER_OAUTH_RUNTIME: &str = "No provider OAuth runtime, token refresh, or revocation handling is implemented by this contract proof.";
const KNOWN_GAP_NO_PROVIDER_UPLOAD_DELETE_RUNTIME: &str = "No provider upload or delete runtime is implemented; status rows stay claim-safe instead of implying transfer execution.";
const KNOWN_GAP_PARENT_SYNC_REMAINS_SEPARATE: &str = "Parent-owned cloud sync remains separate from local export/delete runtime and does not imply restore/apply-back execution.";
const KNOWN_GAP_MANIFEST_INTEGRITY_ONLY_CONTRACT_EVIDENCE: &str = "Manifest integrity is explicit, but checksum and signature refs are contract evidence only until runtime verification exists.";
const KNOWN_GAP_TOMBSTONE_PROPAGATION_MODELED_SEPARATELY: &str = "Tombstone propagation is modeled separately from sync success so blocked or manual-required delete visibility stays explicit.";
const KNOWN_GAP_OCENTRA_NOT_DEFAULT_EVIDENCE_STORE: &str = "Ocentra-hosted cloud metadata is not the default evidence store and no raw child evidence upload is claimed by default.";

parent_owned_sync_string_enums!(ParentPlatform {
    Windows,
    Linux,
    Macos,
    Android,
    Ios,
});

parent_owned_sync_string_enums!(ParentActorRole {
    Parent,
    Guardian,
    System,
});

parent_owned_sync_string_enums!(ParentEvidenceReferenceKind {
    JournalEvent,
    QueryStoreSummary,
    AuditTrail,
});

parent_owned_sync_string_enums!(ParentOwnedSyncExportDataClass {
    EncryptedJournalSegment,
    SqliteQueryRow,
    ParentRule,
    ApprovalDecision,
    DeviceRegistryEntry,
    NotificationHistory,
    AuditEvent,
    GeneratedSummary,
});

parent_owned_sync_string_enums!(ParentOwnedSyncExportFormat {
    EncryptedMachineReadable,
    EncryptedSupportBundle,
    HumanReadableParentReport,
});

parent_owned_sync_string_enums!(ParentOwnedSyncExportDestinationOwnership {
    ChildLocal,
    ParentDeviceLocal,
    ParentOwnedExternalStorage,
    OcentraHostedNonActivityMetadata,
});

parent_owned_sync_string_enums!(ParentOwnedSyncExportEncryptionState {
    EncryptedAtRest,
    HumanReadableParentAuthorized,
    NotApplicable,
});

parent_owned_sync_string_enums!(ParentOwnedSyncProviderMode {
    GoogleDriveAppdata,
    GoogleDrivePickerFile,
    OnedriveApproot,
    OnedriveParentSelectedFolder,
    IcloudDriveAppContainer,
    IcloudDriveParentSelectedLocation,
    DropboxAppFolder,
    DropboxParentSelectedFolder,
    NasFolder,
    LocalFolder,
    Disabled,
});

parent_owned_sync_string_enums!(ParentOwnedSyncProviderStatus {
    Ready,
    ManualRequired,
    Revoked,
    WrongAccount,
    FolderUnavailable,
    PartialUpload,
    Disconnected,
    Disabled,
    NotConfigured,
});

parent_owned_sync_string_enums!(ParentOwnedSyncManifestIntegrityState {
    Verified,
    Mismatch,
    Corrupt,
    NotApplicable,
});

parent_owned_sync_string_enums!(ParentOwnedSyncState {
    Synced,
    Stale,
    Missing,
    Conflict,
    OfflineRetryPending,
    PartialOutage,
    ManualRequired,
    NotStarted,
});

parent_owned_sync_string_enums!(ParentOwnedSyncTombstonePropagationState {
    NotRequested,
    Pending,
    Propagated,
    Blocked,
    ManualRequired,
});

parent_owned_sync_string_enums!(ParentOwnedSyncDisconnectVisibilityState {
    NotDisconnected,
    DisconnectVisible,
    ManualRequired,
});

parent_owned_sync_string_enums!(ParentOwnedSyncDeleteVisibilityState {
    NotRequested,
    DeleteVisible,
    DeleteConfirmed,
    DeleteFailed,
    ManualRequired,
});

parent_owned_sync_string_enums!(ParentOwnedSyncExportNonClaim {
    NoTransferRuntime,
    #[serde(rename = "no-connector-oauth")]
    NoConnectorOAuth,
    NoUploadRuntime,
    NoDeleteRuntime,
    NoDefaultOcentraCustody,
    NoRawChildEvidenceUploadByDefault,
});

parent_owned_sync_text_identifiers!(
    ParentContractSchemaVersion,
    FamilyId,
    ChildProfileId,
    ParentDeviceId,
    ParentDeviceLabel,
    ParentActorId,
    ParentPolicyVersion,
    ParentEvidenceReferenceId,
    ParentActionReferenceId,
    ParentTimestamp,
    ParentOwnedSyncManifestId,
    ParentOwnedSyncItemId,
    ParentOwnedSyncVersionLabel,
    ParentOwnedSyncPolicyRef,
    ParentOwnedSyncProviderId,
    ParentOwnedSyncProviderRef,
    ParentOwnedSyncStatusRef,
    ParentOwnedSyncCursorRef,
    ParentOwnedSyncBatchRef,
    ParentOwnedSyncConflictRef,
    ParentOwnedSyncChecksumRef,
    ParentOwnedSyncSignatureRef,
    ParentOwnedSyncTombstoneRef,
    ParentOwnedSyncDeleteRequestRef,
);

parent_owned_sync_string_enum_as_str_values!(
    ParentOwnedSyncProviderMode {
        variants: [
            GoogleDriveAppdata,
            GoogleDrivePickerFile,
            OnedriveApproot,
            OnedriveParentSelectedFolder,
            IcloudDriveAppContainer,
            IcloudDriveParentSelectedLocation,
            DropboxAppFolder,
            DropboxParentSelectedFolder,
            NasFolder,
            LocalFolder,
            Disabled,
        ],
        values: [
            PROVIDER_MODE_GOOGLE_DRIVE_APPDATA,
            PROVIDER_MODE_GOOGLE_DRIVE_PICKER_FILE,
            PROVIDER_MODE_ONEDRIVE_APPROOT,
            PROVIDER_MODE_ONEDRIVE_PARENT_SELECTED_FOLDER,
            PROVIDER_MODE_ICLOUD_DRIVE_APP_CONTAINER,
            PROVIDER_MODE_ICLOUD_DRIVE_PARENT_SELECTED_LOCATION,
            PROVIDER_MODE_DROPBOX_APP_FOLDER,
            PROVIDER_MODE_DROPBOX_PARENT_SELECTED_FOLDER,
            PROVIDER_MODE_NAS_FOLDER,
            PROVIDER_MODE_LOCAL_FOLDER,
            PROVIDER_MODE_DISABLED,
        ],
    },
    ParentOwnedSyncProviderStatus {
        variants: [
            Ready,
            ManualRequired,
            Revoked,
            WrongAccount,
            FolderUnavailable,
            PartialUpload,
            Disconnected,
            Disabled,
            NotConfigured,
        ],
        values: [
            PROVIDER_STATUS_READY,
            PROVIDER_STATUS_MANUAL_REQUIRED,
            PROVIDER_STATUS_REVOKED,
            PROVIDER_STATUS_WRONG_ACCOUNT,
            PROVIDER_STATUS_FOLDER_UNAVAILABLE,
            PROVIDER_STATUS_PARTIAL_UPLOAD,
            PROVIDER_STATUS_DISCONNECTED,
            PROVIDER_STATUS_DISABLED,
            PROVIDER_STATUS_NOT_CONFIGURED,
        ],
    },
);

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentActorReference {
    pub actor_id: ParentActorId,
    pub role: ParentActorRole,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FamilyReference {
    pub family_id: FamilyId,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentDeviceReference {
    pub device_id: ParentDeviceId,
    pub child_profile_id: Option<ChildProfileId>,
    pub label: ParentDeviceLabel,
    pub platform: ParentPlatform,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentEvidenceReference {
    pub evidence_reference_id: ParentEvidenceReferenceId,
    pub kind: ParentEvidenceReferenceKind,
    pub observed_at: ParentTimestamp,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentActionReference {
    pub action_reference_id: ParentActionReferenceId,
    pub actor: ParentActorReference,
    pub policy_version: ParentPolicyVersion,
    pub created_at: ParentTimestamp,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentOwnedSyncExportEncryptionMetadata {
    pub encryption_state: ParentOwnedSyncExportEncryptionState,
    pub encrypted_before_upload: bool,
    pub key_owner: ParentOwnedSyncExportDestinationOwnership,
    pub proof_requirement_ref: ParentOwnedSyncPolicyRef,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentOwnedSyncExportManifestItem {
    pub item_id: ParentOwnedSyncItemId,
    pub data_class: ParentOwnedSyncExportDataClass,
    pub export_format: ParentOwnedSyncExportFormat,
    pub destination_ownership: ParentOwnedSyncExportDestinationOwnership,
    pub schema_version_label: ParentOwnedSyncVersionLabel,
    pub encryption: ParentOwnedSyncExportEncryptionMetadata,
    pub parent_action_required: bool,
    pub raw_child_evidence_uploaded_by_default: bool,
    pub ocentra_hosted_family_data_stored: bool,
    pub claim_safe: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentOwnedSyncExportManifest {
    pub schema_version: String,
    pub manifest_id: ParentOwnedSyncManifestId,
    pub family: FamilyReference,
    pub device: ParentDeviceReference,
    pub parent_action: ParentActionReference,
    pub product_version: ParentOwnedSyncVersionLabel,
    pub manifest_version: ParentOwnedSyncVersionLabel,
    pub generated_at: ParentTimestamp,
    pub items: Vec<ParentOwnedSyncExportManifestItem>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentOwnedSyncProviderStatusRow {
    pub provider_id: ParentOwnedSyncProviderId,
    pub provider_mode: ParentOwnedSyncProviderMode,
    pub provider_status: ParentOwnedSyncProviderStatus,
    pub destination_ownership: ParentOwnedSyncExportDestinationOwnership,
    pub account_ref: Option<ParentOwnedSyncProviderRef>,
    pub folder_ref: Option<ParentOwnedSyncProviderRef>,
    pub status_ref: ParentOwnedSyncStatusRef,
    pub revocation_ref: Option<ParentOwnedSyncProviderRef>,
    pub disconnect_visibility_state: ParentOwnedSyncDisconnectVisibilityState,
    pub delete_visibility_state: ParentOwnedSyncDeleteVisibilityState,
    pub last_checked_at: ParentTimestamp,
    pub oauth_runtime_claimed: bool,
    pub upload_runtime_claimed: bool,
    pub delete_runtime_claimed: bool,
    pub ocentra_hosted_family_data_stored: bool,
    pub claim_safe: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentOwnedSyncStateRow {
    pub sync_state: ParentOwnedSyncState,
    pub provider_status_ref: ParentOwnedSyncStatusRef,
    pub cursor_ref: Option<ParentOwnedSyncCursorRef>,
    pub batch_ref: Option<ParentOwnedSyncBatchRef>,
    pub manifest_integrity_state: ParentOwnedSyncManifestIntegrityState,
    pub manifest_checksum_ref: Option<ParentOwnedSyncChecksumRef>,
    pub manifest_signature_ref: Option<ParentOwnedSyncSignatureRef>,
    pub last_successful_sync_at: Option<ParentTimestamp>,
    pub conflict_ref: Option<ParentOwnedSyncConflictRef>,
    pub retry_queue_ref: Option<ParentOwnedSyncPolicyRef>,
    pub parent_action_required: bool,
    pub claim_safe: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentOwnedSyncTombstoneRow {
    pub tombstone_ref: ParentOwnedSyncTombstoneRef,
    pub data_class: ParentOwnedSyncExportDataClass,
    pub propagation_state: ParentOwnedSyncTombstonePropagationState,
    pub delete_request_ref: Option<ParentOwnedSyncDeleteRequestRef>,
    pub provider_status_ref: ParentOwnedSyncStatusRef,
    pub last_propagated_at: Option<ParentTimestamp>,
    pub blocked_reason_ref: Option<ParentOwnedSyncPolicyRef>,
    pub claim_safe: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentOwnedSyncExportContractProof {
    pub schema_version: String,
    pub contract_version: ParentContractSchemaVersion,
    pub manifest: ParentOwnedSyncExportManifest,
    pub provider_statuses: Vec<ParentOwnedSyncProviderStatusRow>,
    pub sync_states: Vec<ParentOwnedSyncStateRow>,
    pub tombstones: Vec<ParentOwnedSyncTombstoneRow>,
    pub non_claims: Vec<ParentOwnedSyncExportNonClaim>,
    pub transfer_runtime_claimed: bool,
    pub connector_o_auth_claimed: bool,
    pub upload_runtime_claimed: bool,
    pub delete_runtime_claimed: bool,
    pub ocentra_hosted_child_evidence_stored: bool,
    pub updated_at: ParentTimestamp,
}

pub fn parent_owned_sync_export_known_gaps() -> [&'static str; 6] {
    sample::parent_owned_sync_export_known_gaps()
}

pub fn required_parent_owned_sync_export_non_claims() -> Vec<ParentOwnedSyncExportNonClaim> {
    sample::required_parent_owned_sync_export_non_claims()
}

pub fn sample_parent_owned_sync_export_contract_proof() -> ParentOwnedSyncExportContractProof {
    sample::sample_parent_owned_sync_export_contract_proof()
}
