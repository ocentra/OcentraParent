use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

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

macro_rules! parent_owned_sync_text_identifier {
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

macro_rules! parent_owned_sync_string_enum {
    ($name:ident { $($(#[$variant_meta:meta])* $variant:ident),+ $(,)? }) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "kebab-case")]
        pub enum $name {
            $($(#[$variant_meta])* $variant,)+
        }
    };
}

parent_owned_sync_string_enum!(ParentPlatform {
    Windows,
    Linux,
    Macos,
    Android,
    Ios,
});

parent_owned_sync_string_enum!(ParentActorRole {
    Parent,
    Guardian,
    System,
});

parent_owned_sync_string_enum!(ParentEvidenceReferenceKind {
    JournalEvent,
    QueryStoreSummary,
    AuditTrail,
});

parent_owned_sync_string_enum!(ParentOwnedSyncExportDataClass {
    EncryptedJournalSegment,
    SqliteQueryRow,
    ParentRule,
    ApprovalDecision,
    DeviceRegistryEntry,
    NotificationHistory,
    AuditEvent,
    GeneratedSummary,
});

parent_owned_sync_string_enum!(ParentOwnedSyncExportFormat {
    EncryptedMachineReadable,
    EncryptedSupportBundle,
    HumanReadableParentReport,
});

parent_owned_sync_string_enum!(ParentOwnedSyncExportDestinationOwnership {
    ChildLocal,
    ParentDeviceLocal,
    ParentOwnedExternalStorage,
    OcentraHostedNonActivityMetadata,
});

parent_owned_sync_string_enum!(ParentOwnedSyncExportEncryptionState {
    EncryptedAtRest,
    HumanReadableParentAuthorized,
    NotApplicable,
});

parent_owned_sync_string_enum!(ParentOwnedSyncProviderMode {
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

parent_owned_sync_string_enum!(ParentOwnedSyncProviderStatus {
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

parent_owned_sync_string_enum!(ParentOwnedSyncManifestIntegrityState {
    Verified,
    Mismatch,
    Corrupt,
    NotApplicable,
});

parent_owned_sync_string_enum!(ParentOwnedSyncState {
    Synced,
    Stale,
    Missing,
    Conflict,
    OfflineRetryPending,
    PartialOutage,
    ManualRequired,
    NotStarted,
});

parent_owned_sync_string_enum!(ParentOwnedSyncTombstonePropagationState {
    NotRequested,
    Pending,
    Propagated,
    Blocked,
    ManualRequired,
});

parent_owned_sync_string_enum!(ParentOwnedSyncDisconnectVisibilityState {
    NotDisconnected,
    DisconnectVisible,
    ManualRequired,
});

parent_owned_sync_string_enum!(ParentOwnedSyncDeleteVisibilityState {
    NotRequested,
    DeleteVisible,
    DeleteConfirmed,
    DeleteFailed,
    ManualRequired,
});

parent_owned_sync_string_enum!(ParentOwnedSyncExportNonClaim {
    NoTransferRuntime,
    #[serde(rename = "no-connector-oauth")]
    NoConnectorOAuth,
    NoUploadRuntime,
    NoDeleteRuntime,
    NoDefaultOcentraCustody,
    NoRawChildEvidenceUploadByDefault,
});

parent_owned_sync_text_identifier!(ParentContractSchemaVersion);
parent_owned_sync_text_identifier!(FamilyId);
parent_owned_sync_text_identifier!(ChildProfileId);
parent_owned_sync_text_identifier!(ParentDeviceId);
parent_owned_sync_text_identifier!(ParentDeviceLabel);
parent_owned_sync_text_identifier!(ParentActorId);
parent_owned_sync_text_identifier!(ParentPolicyVersion);
parent_owned_sync_text_identifier!(ParentEvidenceReferenceId);
parent_owned_sync_text_identifier!(ParentActionReferenceId);
parent_owned_sync_text_identifier!(ParentTimestamp);
parent_owned_sync_text_identifier!(ParentOwnedSyncManifestId);
parent_owned_sync_text_identifier!(ParentOwnedSyncItemId);
parent_owned_sync_text_identifier!(ParentOwnedSyncVersionLabel);
parent_owned_sync_text_identifier!(ParentOwnedSyncPolicyRef);
parent_owned_sync_text_identifier!(ParentOwnedSyncProviderId);
parent_owned_sync_text_identifier!(ParentOwnedSyncProviderRef);
parent_owned_sync_text_identifier!(ParentOwnedSyncStatusRef);
parent_owned_sync_text_identifier!(ParentOwnedSyncCursorRef);
parent_owned_sync_text_identifier!(ParentOwnedSyncBatchRef);
parent_owned_sync_text_identifier!(ParentOwnedSyncConflictRef);
parent_owned_sync_text_identifier!(ParentOwnedSyncChecksumRef);
parent_owned_sync_text_identifier!(ParentOwnedSyncSignatureRef);
parent_owned_sync_text_identifier!(ParentOwnedSyncTombstoneRef);
parent_owned_sync_text_identifier!(ParentOwnedSyncDeleteRequestRef);

impl ParentOwnedSyncProviderMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::GoogleDriveAppdata => PROVIDER_MODE_GOOGLE_DRIVE_APPDATA,
            Self::GoogleDrivePickerFile => PROVIDER_MODE_GOOGLE_DRIVE_PICKER_FILE,
            Self::OnedriveApproot => PROVIDER_MODE_ONEDRIVE_APPROOT,
            Self::OnedriveParentSelectedFolder => PROVIDER_MODE_ONEDRIVE_PARENT_SELECTED_FOLDER,
            Self::IcloudDriveAppContainer => PROVIDER_MODE_ICLOUD_DRIVE_APP_CONTAINER,
            Self::IcloudDriveParentSelectedLocation => {
                PROVIDER_MODE_ICLOUD_DRIVE_PARENT_SELECTED_LOCATION
            }
            Self::DropboxAppFolder => PROVIDER_MODE_DROPBOX_APP_FOLDER,
            Self::DropboxParentSelectedFolder => PROVIDER_MODE_DROPBOX_PARENT_SELECTED_FOLDER,
            Self::NasFolder => PROVIDER_MODE_NAS_FOLDER,
            Self::LocalFolder => PROVIDER_MODE_LOCAL_FOLDER,
            Self::Disabled => PROVIDER_MODE_DISABLED,
        }
    }
}

impl ParentOwnedSyncProviderStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Ready => PROVIDER_STATUS_READY,
            Self::ManualRequired => PROVIDER_STATUS_MANUAL_REQUIRED,
            Self::Revoked => PROVIDER_STATUS_REVOKED,
            Self::WrongAccount => PROVIDER_STATUS_WRONG_ACCOUNT,
            Self::FolderUnavailable => PROVIDER_STATUS_FOLDER_UNAVAILABLE,
            Self::PartialUpload => PROVIDER_STATUS_PARTIAL_UPLOAD,
            Self::Disconnected => PROVIDER_STATUS_DISCONNECTED,
            Self::Disabled => PROVIDER_STATUS_DISABLED,
            Self::NotConfigured => PROVIDER_STATUS_NOT_CONFIGURED,
        }
    }
}

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
    [
        KNOWN_GAP_NO_PROVIDER_OAUTH_RUNTIME,
        KNOWN_GAP_NO_PROVIDER_UPLOAD_DELETE_RUNTIME,
        KNOWN_GAP_PARENT_SYNC_REMAINS_SEPARATE,
        KNOWN_GAP_MANIFEST_INTEGRITY_ONLY_CONTRACT_EVIDENCE,
        KNOWN_GAP_TOMBSTONE_PROPAGATION_MODELED_SEPARATELY,
        KNOWN_GAP_OCENTRA_NOT_DEFAULT_EVIDENCE_STORE,
    ]
}

pub fn required_parent_owned_sync_export_non_claims() -> Vec<ParentOwnedSyncExportNonClaim> {
    vec![
        ParentOwnedSyncExportNonClaim::NoTransferRuntime,
        ParentOwnedSyncExportNonClaim::NoConnectorOAuth,
        ParentOwnedSyncExportNonClaim::NoUploadRuntime,
        ParentOwnedSyncExportNonClaim::NoDeleteRuntime,
        ParentOwnedSyncExportNonClaim::NoDefaultOcentraCustody,
        ParentOwnedSyncExportNonClaim::NoRawChildEvidenceUploadByDefault,
    ]
}

pub fn sample_parent_owned_sync_export_contract_proof() -> ParentOwnedSyncExportContractProof {
    let timestamp = parent_timestamp("2026-06-28T18:40:00.000Z".to_string());

    ParentOwnedSyncExportContractProof {
        schema_version: PARENT_OWNED_SYNC_EXPORT_SCHEMA_VERSION.to_string(),
        contract_version: contract_version("v0.6".to_string()),
        manifest: ParentOwnedSyncExportManifest {
            schema_version: PARENT_OWNED_SYNC_EXPORT_SCHEMA_VERSION.to_string(),
            manifest_id: manifest_id("parent-owned-sync-manifest-proof-1".to_string()),
            family: FamilyReference {
                family_id: family_id("family-parent-owned-sync-proof-1".to_string()),
            },
            device: ParentDeviceReference {
                device_id: parent_device_id("windows-parent-owned-sync-proof-1".to_string()),
                child_profile_id: Some(child_profile_id(
                    "child-parent-owned-sync-proof-1".to_string(),
                )),
                label: parent_device_label("Windows parent-owned sync proof device".to_string()),
                platform: ParentPlatform::Windows,
            },
            parent_action: ParentActionReference {
                action_reference_id: parent_action_id(
                    "parent-action-parent-owned-sync-proof-1".to_string(),
                ),
                actor: ParentActorReference {
                    actor_id: parent_actor_id("parent-owned-sync-proof-actor-1".to_string()),
                    role: ParentActorRole::Parent,
                },
                policy_version: parent_policy_version(
                    "parent-owned-sync-proof-policy-v1".to_string(),
                ),
                created_at: timestamp.clone(),
            },
            product_version: version_label("0.1.1".to_string()),
            manifest_version: version_label("parent-owned-sync.manifest.v1".to_string()),
            generated_at: timestamp.clone(),
            items: sample_manifest_items(),
        },
        provider_statuses: sample_provider_statuses(&timestamp),
        sync_states: sample_sync_states(),
        tombstones: sample_tombstones(&timestamp),
        non_claims: required_parent_owned_sync_export_non_claims(),
        transfer_runtime_claimed: false,
        connector_o_auth_claimed: false,
        upload_runtime_claimed: false,
        delete_runtime_claimed: false,
        ocentra_hosted_child_evidence_stored: false,
        updated_at: timestamp,
    }
}

fn sample_manifest_items() -> Vec<ParentOwnedSyncExportManifestItem> {
    vec![
        manifest_item(
            "journal",
            ParentOwnedSyncExportDataClass::EncryptedJournalSegment,
            ParentOwnedSyncExportFormat::EncryptedMachineReadable,
        ),
        manifest_item(
            "query",
            ParentOwnedSyncExportDataClass::SqliteQueryRow,
            ParentOwnedSyncExportFormat::EncryptedMachineReadable,
        ),
        manifest_item(
            "rule",
            ParentOwnedSyncExportDataClass::ParentRule,
            ParentOwnedSyncExportFormat::EncryptedSupportBundle,
        ),
        manifest_item(
            "approval",
            ParentOwnedSyncExportDataClass::ApprovalDecision,
            ParentOwnedSyncExportFormat::EncryptedSupportBundle,
        ),
        manifest_item(
            "registry",
            ParentOwnedSyncExportDataClass::DeviceRegistryEntry,
            ParentOwnedSyncExportFormat::EncryptedMachineReadable,
        ),
        manifest_item(
            "notifications",
            ParentOwnedSyncExportDataClass::NotificationHistory,
            ParentOwnedSyncExportFormat::EncryptedSupportBundle,
        ),
        manifest_item(
            "audit",
            ParentOwnedSyncExportDataClass::AuditEvent,
            ParentOwnedSyncExportFormat::EncryptedMachineReadable,
        ),
        manifest_item(
            "summary",
            ParentOwnedSyncExportDataClass::GeneratedSummary,
            ParentOwnedSyncExportFormat::HumanReadableParentReport,
        ),
    ]
}

fn manifest_item(
    suffix: impl AsRef<str>,
    data_class: ParentOwnedSyncExportDataClass,
    export_format: ParentOwnedSyncExportFormat,
) -> ParentOwnedSyncExportManifestItem {
    let suffix = suffix.as_ref();
    let human_readable = export_format == ParentOwnedSyncExportFormat::HumanReadableParentReport;

    ParentOwnedSyncExportManifestItem {
        item_id: item_id(format!("manifest-item-{suffix}")),
        data_class,
        export_format,
        destination_ownership:
            ParentOwnedSyncExportDestinationOwnership::ParentOwnedExternalStorage,
        schema_version_label: version_label(format!("{suffix}.v1")),
        encryption: ParentOwnedSyncExportEncryptionMetadata {
            encryption_state: if human_readable {
                ParentOwnedSyncExportEncryptionState::HumanReadableParentAuthorized
            } else {
                ParentOwnedSyncExportEncryptionState::EncryptedAtRest
            },
            encrypted_before_upload: !human_readable,
            key_owner: ParentOwnedSyncExportDestinationOwnership::ParentOwnedExternalStorage,
            proof_requirement_ref: policy_ref(format!("encryption-proof-{suffix}")),
        },
        parent_action_required: true,
        raw_child_evidence_uploaded_by_default: false,
        ocentra_hosted_family_data_stored: false,
        claim_safe: true,
    }
}

fn sample_provider_statuses(timestamp: &ParentTimestamp) -> Vec<ParentOwnedSyncProviderStatusRow> {
    let mut rows = sample_provider_statuses_cloud(timestamp);
    rows.extend(sample_provider_statuses_icloud(timestamp));
    rows.extend(sample_provider_statuses_storage(timestamp));
    rows
}

fn sample_provider_statuses_cloud(
    timestamp: &ParentTimestamp,
) -> Vec<ParentOwnedSyncProviderStatusRow> {
    [
        ParentOwnedSyncProviderStatusInput {
            provider_mode: ParentOwnedSyncProviderMode::GoogleDriveAppdata,
            provider_status: ParentOwnedSyncProviderStatus::Ready,
            destination_ownership:
                ParentOwnedSyncExportDestinationOwnership::ParentOwnedExternalStorage,
            account_ref: Some("account-google-drive-appdata"),
            folder_ref: Some("folder-google-drive-appdata"),
            revocation_ref: None,
            disconnect_visibility_state: ParentOwnedSyncDisconnectVisibilityState::NotDisconnected,
            delete_visibility_state: ParentOwnedSyncDeleteVisibilityState::NotRequested,
            timestamp,
        },
        ParentOwnedSyncProviderStatusInput {
            provider_mode: ParentOwnedSyncProviderMode::GoogleDrivePickerFile,
            provider_status: ParentOwnedSyncProviderStatus::ManualRequired,
            destination_ownership:
                ParentOwnedSyncExportDestinationOwnership::ParentOwnedExternalStorage,
            account_ref: Some("account-google-drive-picker"),
            folder_ref: Some("folder-google-drive-picker"),
            revocation_ref: None,
            disconnect_visibility_state: ParentOwnedSyncDisconnectVisibilityState::ManualRequired,
            delete_visibility_state: ParentOwnedSyncDeleteVisibilityState::ManualRequired,
            timestamp,
        },
        ParentOwnedSyncProviderStatusInput {
            provider_mode: ParentOwnedSyncProviderMode::OnedriveApproot,
            provider_status: ParentOwnedSyncProviderStatus::Revoked,
            destination_ownership:
                ParentOwnedSyncExportDestinationOwnership::ParentOwnedExternalStorage,
            account_ref: Some("account-onedrive-approot"),
            folder_ref: Some("folder-onedrive-approot"),
            revocation_ref: Some("revoked-onedrive-approot"),
            disconnect_visibility_state: ParentOwnedSyncDisconnectVisibilityState::NotDisconnected,
            delete_visibility_state: ParentOwnedSyncDeleteVisibilityState::NotRequested,
            timestamp,
        },
        ParentOwnedSyncProviderStatusInput {
            provider_mode: ParentOwnedSyncProviderMode::OnedriveParentSelectedFolder,
            provider_status: ParentOwnedSyncProviderStatus::WrongAccount,
            destination_ownership:
                ParentOwnedSyncExportDestinationOwnership::ParentOwnedExternalStorage,
            account_ref: Some("account-onedrive-selected"),
            folder_ref: Some("folder-onedrive-selected"),
            revocation_ref: None,
            disconnect_visibility_state: ParentOwnedSyncDisconnectVisibilityState::NotDisconnected,
            delete_visibility_state: ParentOwnedSyncDeleteVisibilityState::DeleteVisible,
            timestamp,
        },
    ]
    .iter()
    .map(provider_status_row)
    .collect()
}

fn sample_provider_statuses_icloud(
    timestamp: &ParentTimestamp,
) -> Vec<ParentOwnedSyncProviderStatusRow> {
    [
        ParentOwnedSyncProviderStatusInput {
            provider_mode: ParentOwnedSyncProviderMode::IcloudDriveAppContainer,
            provider_status: ParentOwnedSyncProviderStatus::FolderUnavailable,
            destination_ownership:
                ParentOwnedSyncExportDestinationOwnership::ParentOwnedExternalStorage,
            account_ref: Some("account-icloud-container"),
            folder_ref: Some("folder-icloud-container"),
            revocation_ref: None,
            disconnect_visibility_state: ParentOwnedSyncDisconnectVisibilityState::NotDisconnected,
            delete_visibility_state: ParentOwnedSyncDeleteVisibilityState::DeleteFailed,
            timestamp,
        },
        ParentOwnedSyncProviderStatusInput {
            provider_mode: ParentOwnedSyncProviderMode::IcloudDriveParentSelectedLocation,
            provider_status: ParentOwnedSyncProviderStatus::Disconnected,
            destination_ownership:
                ParentOwnedSyncExportDestinationOwnership::ParentOwnedExternalStorage,
            account_ref: Some("account-icloud-location"),
            folder_ref: Some("folder-icloud-location"),
            revocation_ref: None,
            disconnect_visibility_state:
                ParentOwnedSyncDisconnectVisibilityState::DisconnectVisible,
            delete_visibility_state: ParentOwnedSyncDeleteVisibilityState::NotRequested,
            timestamp,
        },
        ParentOwnedSyncProviderStatusInput {
            provider_mode: ParentOwnedSyncProviderMode::DropboxAppFolder,
            provider_status: ParentOwnedSyncProviderStatus::PartialUpload,
            destination_ownership:
                ParentOwnedSyncExportDestinationOwnership::ParentOwnedExternalStorage,
            account_ref: Some("account-dropbox-app"),
            folder_ref: Some("folder-dropbox-app"),
            revocation_ref: None,
            disconnect_visibility_state: ParentOwnedSyncDisconnectVisibilityState::NotDisconnected,
            delete_visibility_state: ParentOwnedSyncDeleteVisibilityState::DeleteConfirmed,
            timestamp,
        },
    ]
    .iter()
    .map(provider_status_row)
    .collect()
}

fn sample_provider_statuses_storage(
    timestamp: &ParentTimestamp,
) -> Vec<ParentOwnedSyncProviderStatusRow> {
    [
        ParentOwnedSyncProviderStatusInput {
            provider_mode: ParentOwnedSyncProviderMode::DropboxParentSelectedFolder,
            provider_status: ParentOwnedSyncProviderStatus::Ready,
            destination_ownership:
                ParentOwnedSyncExportDestinationOwnership::ParentOwnedExternalStorage,
            account_ref: Some("account-dropbox-selected"),
            folder_ref: Some("folder-dropbox-selected"),
            revocation_ref: None,
            disconnect_visibility_state: ParentOwnedSyncDisconnectVisibilityState::NotDisconnected,
            delete_visibility_state: ParentOwnedSyncDeleteVisibilityState::NotRequested,
            timestamp,
        },
        ParentOwnedSyncProviderStatusInput {
            provider_mode: ParentOwnedSyncProviderMode::NasFolder,
            provider_status: ParentOwnedSyncProviderStatus::Ready,
            destination_ownership:
                ParentOwnedSyncExportDestinationOwnership::ParentOwnedExternalStorage,
            account_ref: Some("account-nas-folder"),
            folder_ref: Some("folder-nas-folder"),
            revocation_ref: None,
            disconnect_visibility_state: ParentOwnedSyncDisconnectVisibilityState::NotDisconnected,
            delete_visibility_state: ParentOwnedSyncDeleteVisibilityState::NotRequested,
            timestamp,
        },
        ParentOwnedSyncProviderStatusInput {
            provider_mode: ParentOwnedSyncProviderMode::LocalFolder,
            provider_status: ParentOwnedSyncProviderStatus::Ready,
            destination_ownership: ParentOwnedSyncExportDestinationOwnership::ParentDeviceLocal,
            account_ref: Some("account-local-folder"),
            folder_ref: Some("folder-local-folder"),
            revocation_ref: None,
            disconnect_visibility_state: ParentOwnedSyncDisconnectVisibilityState::NotDisconnected,
            delete_visibility_state: ParentOwnedSyncDeleteVisibilityState::NotRequested,
            timestamp,
        },
        ParentOwnedSyncProviderStatusInput {
            provider_mode: ParentOwnedSyncProviderMode::Disabled,
            provider_status: ParentOwnedSyncProviderStatus::Disabled,
            destination_ownership: ParentOwnedSyncExportDestinationOwnership::ParentDeviceLocal,
            account_ref: None,
            folder_ref: None,
            revocation_ref: None,
            disconnect_visibility_state: ParentOwnedSyncDisconnectVisibilityState::NotDisconnected,
            delete_visibility_state: ParentOwnedSyncDeleteVisibilityState::NotRequested,
            timestamp,
        },
    ]
    .iter()
    .map(provider_status_row)
    .collect()
}

struct ParentOwnedSyncProviderStatusInput<'a> {
    provider_mode: ParentOwnedSyncProviderMode,
    provider_status: ParentOwnedSyncProviderStatus,
    destination_ownership: ParentOwnedSyncExportDestinationOwnership,
    account_ref: Option<&'a str>,
    folder_ref: Option<&'a str>,
    revocation_ref: Option<&'a str>,
    disconnect_visibility_state: ParentOwnedSyncDisconnectVisibilityState,
    delete_visibility_state: ParentOwnedSyncDeleteVisibilityState,
    timestamp: &'a ParentTimestamp,
}

fn provider_status_row(
    input: &ParentOwnedSyncProviderStatusInput<'_>,
) -> ParentOwnedSyncProviderStatusRow {
    ParentOwnedSyncProviderStatusRow {
        provider_id: provider_id(format!("provider-{}", input.provider_mode.as_str())),
        provider_mode: input.provider_mode,
        provider_status: input.provider_status,
        destination_ownership: input.destination_ownership,
        account_ref: input.account_ref.map(provider_ref),
        folder_ref: input.folder_ref.map(provider_ref),
        status_ref: status_ref(format!(
            "provider-status-{}-{}",
            input.provider_mode.as_str(),
            input.provider_status.as_str()
        )),
        revocation_ref: input.revocation_ref.map(provider_ref),
        disconnect_visibility_state: input.disconnect_visibility_state,
        delete_visibility_state: input.delete_visibility_state,
        last_checked_at: input.timestamp.clone(),
        oauth_runtime_claimed: false,
        upload_runtime_claimed: false,
        delete_runtime_claimed: false,
        ocentra_hosted_family_data_stored: false,
        claim_safe: true,
    }
}

fn sample_sync_states() -> Vec<ParentOwnedSyncStateRow> {
    let mut rows = sample_sync_state_rows_primary();
    rows.extend(sample_sync_state_rows_secondary());
    rows
}

fn sample_sync_state_rows_primary() -> Vec<ParentOwnedSyncStateRow> {
    [
        ParentOwnedSyncStateRowInput {
            sync_state: ParentOwnedSyncState::Synced,
            provider_status_ref_value: "provider-status-google-drive-appdata-ready",
            cursor_ref_value: Some("cursor-synced"),
            batch_ref_value: Some("batch-synced"),
            manifest_integrity_state: ParentOwnedSyncManifestIntegrityState::Verified,
            checksum_ref_value: Some("checksum-synced"),
            signature_ref_value: Some("signature-synced"),
            last_successful_sync_at: Some("2026-06-28T18:40:00.000Z"),
            conflict_ref_value: None,
            retry_queue_ref_value: None,
            parent_action_required: false,
        },
        ParentOwnedSyncStateRowInput {
            sync_state: ParentOwnedSyncState::Stale,
            provider_status_ref_value: "provider-status-dropbox-parent-selected-folder-ready",
            cursor_ref_value: Some("cursor-stale"),
            batch_ref_value: Some("batch-stale"),
            manifest_integrity_state: ParentOwnedSyncManifestIntegrityState::Verified,
            checksum_ref_value: Some("checksum-stale"),
            signature_ref_value: Some("signature-stale"),
            last_successful_sync_at: Some("2026-06-28T18:20:00.000Z"),
            conflict_ref_value: None,
            retry_queue_ref_value: None,
            parent_action_required: false,
        },
        ParentOwnedSyncStateRowInput {
            sync_state: ParentOwnedSyncState::Missing,
            provider_status_ref_value: "provider-status-local-folder-ready",
            cursor_ref_value: None,
            batch_ref_value: None,
            manifest_integrity_state: ParentOwnedSyncManifestIntegrityState::Mismatch,
            checksum_ref_value: None,
            signature_ref_value: None,
            last_successful_sync_at: None,
            conflict_ref_value: None,
            retry_queue_ref_value: Some("retry-missing-manifest"),
            parent_action_required: true,
        },
        ParentOwnedSyncStateRowInput {
            sync_state: ParentOwnedSyncState::Conflict,
            provider_status_ref_value:
                "provider-status-onedrive-parent-selected-folder-wrong-account",
            cursor_ref_value: Some("cursor-conflict"),
            batch_ref_value: Some("batch-conflict"),
            manifest_integrity_state: ParentOwnedSyncManifestIntegrityState::Verified,
            checksum_ref_value: Some("checksum-conflict"),
            signature_ref_value: Some("signature-conflict"),
            last_successful_sync_at: None,
            conflict_ref_value: Some("conflict-parent-owned-sync-1"),
            retry_queue_ref_value: Some("retry-conflict"),
            parent_action_required: true,
        },
    ]
    .iter()
    .map(sync_state_row)
    .collect()
}

fn sample_sync_state_rows_secondary() -> Vec<ParentOwnedSyncStateRow> {
    [
        ParentOwnedSyncStateRowInput {
            sync_state: ParentOwnedSyncState::OfflineRetryPending,
            provider_status_ref_value: "provider-status-nas-folder-ready",
            cursor_ref_value: Some("cursor-offline-retry"),
            batch_ref_value: Some("batch-offline-retry"),
            manifest_integrity_state: ParentOwnedSyncManifestIntegrityState::Verified,
            checksum_ref_value: Some("checksum-offline-retry"),
            signature_ref_value: Some("signature-offline-retry"),
            last_successful_sync_at: Some("2026-06-28T18:10:00.000Z"),
            conflict_ref_value: None,
            retry_queue_ref_value: Some("retry-offline"),
            parent_action_required: false,
        },
        ParentOwnedSyncStateRowInput {
            sync_state: ParentOwnedSyncState::PartialOutage,
            provider_status_ref_value: "provider-status-dropbox-app-folder-partial-upload",
            cursor_ref_value: Some("cursor-partial-outage"),
            batch_ref_value: Some("batch-partial-outage"),
            manifest_integrity_state: ParentOwnedSyncManifestIntegrityState::Verified,
            checksum_ref_value: Some("checksum-partial-outage"),
            signature_ref_value: Some("signature-partial-outage"),
            last_successful_sync_at: Some("2026-06-28T18:00:00.000Z"),
            conflict_ref_value: None,
            retry_queue_ref_value: Some("retry-partial-outage"),
            parent_action_required: false,
        },
        ParentOwnedSyncStateRowInput {
            sync_state: ParentOwnedSyncState::ManualRequired,
            provider_status_ref_value: "provider-status-google-drive-picker-file-manual-required",
            cursor_ref_value: Some("cursor-manual"),
            batch_ref_value: Some("batch-manual"),
            manifest_integrity_state: ParentOwnedSyncManifestIntegrityState::Corrupt,
            checksum_ref_value: Some("checksum-manual"),
            signature_ref_value: Some("signature-manual"),
            last_successful_sync_at: None,
            conflict_ref_value: None,
            retry_queue_ref_value: Some("retry-manual-review"),
            parent_action_required: true,
        },
        ParentOwnedSyncStateRowInput {
            sync_state: ParentOwnedSyncState::NotStarted,
            provider_status_ref_value: "provider-status-disabled-disabled",
            cursor_ref_value: None,
            batch_ref_value: None,
            manifest_integrity_state: ParentOwnedSyncManifestIntegrityState::NotApplicable,
            checksum_ref_value: None,
            signature_ref_value: None,
            last_successful_sync_at: None,
            conflict_ref_value: None,
            retry_queue_ref_value: None,
            parent_action_required: false,
        },
    ]
    .iter()
    .map(sync_state_row)
    .collect()
}

struct ParentOwnedSyncStateRowInput<'a> {
    sync_state: ParentOwnedSyncState,
    provider_status_ref_value: &'a str,
    cursor_ref_value: Option<&'a str>,
    batch_ref_value: Option<&'a str>,
    manifest_integrity_state: ParentOwnedSyncManifestIntegrityState,
    checksum_ref_value: Option<&'a str>,
    signature_ref_value: Option<&'a str>,
    last_successful_sync_at: Option<&'a str>,
    conflict_ref_value: Option<&'a str>,
    retry_queue_ref_value: Option<&'a str>,
    parent_action_required: bool,
}

fn sync_state_row(input: &ParentOwnedSyncStateRowInput<'_>) -> ParentOwnedSyncStateRow {
    ParentOwnedSyncStateRow {
        sync_state: input.sync_state,
        provider_status_ref: status_ref(input.provider_status_ref_value),
        cursor_ref: input.cursor_ref_value.map(cursor_ref),
        batch_ref: input.batch_ref_value.map(batch_ref),
        manifest_integrity_state: input.manifest_integrity_state,
        manifest_checksum_ref: input.checksum_ref_value.map(checksum_ref),
        manifest_signature_ref: input.signature_ref_value.map(signature_ref),
        last_successful_sync_at: input.last_successful_sync_at.map(parent_timestamp),
        conflict_ref: input.conflict_ref_value.map(conflict_ref),
        retry_queue_ref: input.retry_queue_ref_value.map(policy_ref),
        parent_action_required: input.parent_action_required,
        claim_safe: true,
    }
}

fn sample_tombstones(timestamp: &ParentTimestamp) -> Vec<ParentOwnedSyncTombstoneRow> {
    vec![
        tombstone_row(
            "tombstone-none".to_string(),
            ParentOwnedSyncExportDataClass::ParentRule,
            ParentOwnedSyncTombstonePropagationState::NotRequested,
            None,
            "provider-status-google-drive-appdata-ready".to_string(),
            None,
            None,
        ),
        tombstone_row(
            "tombstone-pending".to_string(),
            ParentOwnedSyncExportDataClass::AuditEvent,
            ParentOwnedSyncTombstonePropagationState::Pending,
            Some("delete-request-audit".to_string()),
            "provider-status-dropbox-app-folder-partial-upload".to_string(),
            None,
            None,
        ),
        tombstone_row(
            "tombstone-propagated".to_string(),
            ParentOwnedSyncExportDataClass::GeneratedSummary,
            ParentOwnedSyncTombstonePropagationState::Propagated,
            Some("delete-request-summary".to_string()),
            "provider-status-onedrive-parent-selected-folder-wrong-account".to_string(),
            Some(timestamp.as_str()),
            None,
        ),
        tombstone_row(
            "tombstone-blocked".to_string(),
            ParentOwnedSyncExportDataClass::NotificationHistory,
            ParentOwnedSyncTombstonePropagationState::Blocked,
            Some("delete-request-notification".to_string()),
            "provider-status-icloud-drive-app-container-folder-unavailable".to_string(),
            None,
            Some("blocked-folder-unavailable".to_string()),
        ),
        tombstone_row(
            "tombstone-manual".to_string(),
            ParentOwnedSyncExportDataClass::DeviceRegistryEntry,
            ParentOwnedSyncTombstonePropagationState::ManualRequired,
            Some("delete-request-device-registry".to_string()),
            "provider-status-google-drive-picker-file-manual-required".to_string(),
            None,
            Some("manual-delete-confirmation-required".to_string()),
        ),
    ]
}

fn tombstone_row(
    tombstone_ref_value: String,
    data_class: ParentOwnedSyncExportDataClass,
    propagation_state: ParentOwnedSyncTombstonePropagationState,
    delete_request_ref_value: Option<String>,
    provider_status_ref_value: String,
    last_propagated_at: Option<&str>,
    blocked_reason_ref_value: Option<String>,
) -> ParentOwnedSyncTombstoneRow {
    ParentOwnedSyncTombstoneRow {
        tombstone_ref: tombstone_ref(tombstone_ref_value),
        data_class,
        propagation_state,
        delete_request_ref: delete_request_ref_value.map(delete_request_ref),
        provider_status_ref: status_ref(provider_status_ref_value),
        last_propagated_at: last_propagated_at.map(parent_timestamp),
        blocked_reason_ref: blocked_reason_ref_value.map(policy_ref),
        claim_safe: true,
    }
}

fn contract_version(value: impl Into<String>) -> ParentContractSchemaVersion {
    crate::schema_option_or_unreachable(
        ParentContractSchemaVersion::parse(value),
        CONTRACT_VERSION_EXPECT_MESSAGE,
    )
}

fn family_id(value: impl Into<String>) -> FamilyId {
    crate::schema_option_or_unreachable(FamilyId::parse(value), FAMILY_ID_EXPECT_MESSAGE)
}

fn child_profile_id(value: impl Into<String>) -> ChildProfileId {
    crate::schema_option_or_unreachable(
        ChildProfileId::parse(value),
        CHILD_PROFILE_ID_EXPECT_MESSAGE,
    )
}

fn parent_device_id(value: impl Into<String>) -> ParentDeviceId {
    crate::schema_option_or_unreachable(
        ParentDeviceId::parse(value),
        PARENT_DEVICE_ID_EXPECT_MESSAGE,
    )
}

fn parent_device_label(value: impl Into<String>) -> ParentDeviceLabel {
    crate::schema_option_or_unreachable(
        ParentDeviceLabel::parse(value),
        PARENT_DEVICE_LABEL_EXPECT_MESSAGE,
    )
}

fn parent_actor_id(value: impl Into<String>) -> ParentActorId {
    crate::schema_option_or_unreachable(ParentActorId::parse(value), PARENT_ACTOR_ID_EXPECT_MESSAGE)
}

fn parent_policy_version(value: impl Into<String>) -> ParentPolicyVersion {
    crate::schema_option_or_unreachable(
        ParentPolicyVersion::parse(value),
        PARENT_POLICY_VERSION_EXPECT_MESSAGE,
    )
}

fn parent_action_id(value: impl Into<String>) -> ParentActionReferenceId {
    crate::schema_option_or_unreachable(
        ParentActionReferenceId::parse(value),
        PARENT_ACTION_ID_EXPECT_MESSAGE,
    )
}

fn parent_timestamp(value: impl Into<String>) -> ParentTimestamp {
    crate::schema_option_or_unreachable(
        ParentTimestamp::parse(value),
        PARENT_TIMESTAMP_EXPECT_MESSAGE,
    )
}

fn manifest_id(value: impl Into<String>) -> ParentOwnedSyncManifestId {
    crate::schema_option_or_unreachable(
        ParentOwnedSyncManifestId::parse(value),
        MANIFEST_ID_EXPECT_MESSAGE,
    )
}

fn item_id(value: impl Into<String>) -> ParentOwnedSyncItemId {
    crate::schema_option_or_unreachable(ParentOwnedSyncItemId::parse(value), ITEM_ID_EXPECT_MESSAGE)
}

fn version_label(value: impl Into<String>) -> ParentOwnedSyncVersionLabel {
    crate::schema_option_or_unreachable(
        ParentOwnedSyncVersionLabel::parse(value),
        VERSION_LABEL_EXPECT_MESSAGE,
    )
}

fn policy_ref(value: impl Into<String>) -> ParentOwnedSyncPolicyRef {
    crate::schema_option_or_unreachable(
        ParentOwnedSyncPolicyRef::parse(value),
        POLICY_REF_EXPECT_MESSAGE,
    )
}

fn provider_id(value: impl Into<String>) -> ParentOwnedSyncProviderId {
    crate::schema_option_or_unreachable(
        ParentOwnedSyncProviderId::parse(value),
        PROVIDER_ID_EXPECT_MESSAGE,
    )
}

fn provider_ref(value: impl Into<String>) -> ParentOwnedSyncProviderRef {
    crate::schema_option_or_unreachable(
        ParentOwnedSyncProviderRef::parse(value),
        PROVIDER_REF_EXPECT_MESSAGE,
    )
}

fn status_ref(value: impl Into<String>) -> ParentOwnedSyncStatusRef {
    crate::schema_option_or_unreachable(
        ParentOwnedSyncStatusRef::parse(value),
        STATUS_REF_EXPECT_MESSAGE,
    )
}

fn cursor_ref(value: impl Into<String>) -> ParentOwnedSyncCursorRef {
    crate::schema_option_or_unreachable(
        ParentOwnedSyncCursorRef::parse(value),
        CURSOR_REF_EXPECT_MESSAGE,
    )
}

fn batch_ref(value: impl Into<String>) -> ParentOwnedSyncBatchRef {
    crate::schema_option_or_unreachable(
        ParentOwnedSyncBatchRef::parse(value),
        BATCH_REF_EXPECT_MESSAGE,
    )
}

fn conflict_ref(value: impl Into<String>) -> ParentOwnedSyncConflictRef {
    crate::schema_option_or_unreachable(
        ParentOwnedSyncConflictRef::parse(value),
        CONFLICT_REF_EXPECT_MESSAGE,
    )
}

fn checksum_ref(value: impl Into<String>) -> ParentOwnedSyncChecksumRef {
    crate::schema_option_or_unreachable(
        ParentOwnedSyncChecksumRef::parse(value),
        CHECKSUM_REF_EXPECT_MESSAGE,
    )
}

fn signature_ref(value: impl Into<String>) -> ParentOwnedSyncSignatureRef {
    crate::schema_option_or_unreachable(
        ParentOwnedSyncSignatureRef::parse(value),
        SIGNATURE_REF_EXPECT_MESSAGE,
    )
}

fn tombstone_ref(value: impl Into<String>) -> ParentOwnedSyncTombstoneRef {
    crate::schema_option_or_unreachable(
        ParentOwnedSyncTombstoneRef::parse(value),
        TOMBSTONE_REF_EXPECT_MESSAGE,
    )
}

fn delete_request_ref(value: impl Into<String>) -> ParentOwnedSyncDeleteRequestRef {
    crate::schema_option_or_unreachable(
        ParentOwnedSyncDeleteRequestRef::parse(value),
        DELETE_REQUEST_REF_EXPECT_MESSAGE,
    )
}
