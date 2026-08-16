use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::super::constants::{
    PARENT_STORAGE_COPY_KEY_APPLY_REQUIRES_CONFIRMATION, PARENT_STORAGE_COPY_KEY_BACKUP_QUEUED,
    PARENT_STORAGE_COPY_KEY_CUSTODY_BOUNDARY,
    PARENT_STORAGE_COPY_KEY_DELETED_LOCALLY_PROVIDER_DELETE_PENDING,
    PARENT_STORAGE_COPY_KEY_DISCONNECT_DOES_NOT_DELETE,
    PARENT_STORAGE_COPY_KEY_IMPORT_PREVIEW_PASSED,
    PARENT_STORAGE_COPY_KEY_LOST_KEY_MAY_BE_UNRECOVERABLE,
    PARENT_STORAGE_COPY_KEY_MANUAL_PROOF_REQUIRED, PARENT_STORAGE_COPY_KEY_METADATA_LEAKAGE,
    PARENT_STORAGE_COPY_KEY_PROVIDER_DISCONNECTED_EXISTING_FILES_MAY_REMAIN,
    PARENT_STORAGE_COPY_KEY_PROVIDER_UPLOAD_CONFIRMED,
    PARENT_STORAGE_COPY_KEY_PROVIDER_UPLOAD_FAILED,
    PARENT_STORAGE_COPY_KEY_PROVIDER_UPLOAD_PENDING,
    PARENT_STORAGE_COPY_KEY_SENSITIVE_ENCRYPTED_BEFORE_UPLOAD,
    PARENT_STORAGE_COPY_KEY_TOMBSTONES_MAY_BE_REQUIRED,
};

const COPY_KEY_VALUES: [&str; 15] = [
    PARENT_STORAGE_COPY_KEY_CUSTODY_BOUNDARY,
    PARENT_STORAGE_COPY_KEY_METADATA_LEAKAGE,
    PARENT_STORAGE_COPY_KEY_SENSITIVE_ENCRYPTED_BEFORE_UPLOAD,
    PARENT_STORAGE_COPY_KEY_LOST_KEY_MAY_BE_UNRECOVERABLE,
    PARENT_STORAGE_COPY_KEY_DISCONNECT_DOES_NOT_DELETE,
    PARENT_STORAGE_COPY_KEY_TOMBSTONES_MAY_BE_REQUIRED,
    PARENT_STORAGE_COPY_KEY_BACKUP_QUEUED,
    PARENT_STORAGE_COPY_KEY_PROVIDER_UPLOAD_PENDING,
    PARENT_STORAGE_COPY_KEY_PROVIDER_UPLOAD_FAILED,
    PARENT_STORAGE_COPY_KEY_PROVIDER_UPLOAD_CONFIRMED,
    PARENT_STORAGE_COPY_KEY_IMPORT_PREVIEW_PASSED,
    PARENT_STORAGE_COPY_KEY_APPLY_REQUIRES_CONFIRMATION,
    PARENT_STORAGE_COPY_KEY_DELETED_LOCALLY_PROVIDER_DELETE_PENDING,
    PARENT_STORAGE_COPY_KEY_PROVIDER_DISCONNECTED_EXISTING_FILES_MAY_REMAIN,
    PARENT_STORAGE_COPY_KEY_MANUAL_PROOF_REQUIRED,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(u8)]
pub enum ParentStorageCopyKey {
    CustodyBoundary,
    MetadataLeakage,
    SensitiveEncryptedBeforeUpload,
    LostKeyMayBeUnrecoverable,
    DisconnectDoesNotDelete,
    TombstonesMayBeRequired,
    BackupQueued,
    ProviderUploadPending,
    ProviderUploadFailed,
    ProviderUploadConfirmed,
    ImportPreviewPassed,
    ApplyRequiresConfirmation,
    DeletedLocallyProviderDeletePending,
    ProviderDisconnectedExistingFilesMayRemain,
    ManualProofRequired,
}

impl ParentStorageCopyKey {
    pub fn as_str(&self) -> &'static str {
        COPY_KEY_VALUES[*self as usize]
    }

    fn parse(value: &str) -> Option<Self> {
        let variants = [
            Self::CustodyBoundary,
            Self::MetadataLeakage,
            Self::SensitiveEncryptedBeforeUpload,
            Self::LostKeyMayBeUnrecoverable,
            Self::DisconnectDoesNotDelete,
            Self::TombstonesMayBeRequired,
            Self::BackupQueued,
            Self::ProviderUploadPending,
            Self::ProviderUploadFailed,
            Self::ProviderUploadConfirmed,
            Self::ImportPreviewPassed,
            Self::ApplyRequiresConfirmation,
            Self::DeletedLocallyProviderDeletePending,
            Self::ProviderDisconnectedExistingFilesMayRemain,
            Self::ManualProofRequired,
        ];

        COPY_KEY_VALUES
            .iter()
            .position(|candidate| *candidate == value)
            .map(|index| variants[index])
    }
}

impl Serialize for ParentStorageCopyKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for ParentStorageCopyKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::parse(&value).ok_or_else(|| D::Error::custom("invalid parent storage copy key"))
    }
}
