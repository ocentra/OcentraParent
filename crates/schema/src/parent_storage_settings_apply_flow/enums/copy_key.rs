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
pub struct ParentStorageCopyKey(u8);

impl ParentStorageCopyKey {
    pub const CustodyBoundary: Self = Self(0);
    pub const MetadataLeakage: Self = Self(1);
    pub const SensitiveEncryptedBeforeUpload: Self = Self(2);
    pub const LostKeyMayBeUnrecoverable: Self = Self(3);
    pub const DisconnectDoesNotDelete: Self = Self(4);
    pub const TombstonesMayBeRequired: Self = Self(5);
    pub const BackupQueued: Self = Self(6);
    pub const ProviderUploadPending: Self = Self(7);
    pub const ProviderUploadFailed: Self = Self(8);
    pub const ProviderUploadConfirmed: Self = Self(9);
    pub const ImportPreviewPassed: Self = Self(10);
    pub const ApplyRequiresConfirmation: Self = Self(11);
    pub const DeletedLocallyProviderDeletePending: Self = Self(12);
    pub const ProviderDisconnectedExistingFilesMayRemain: Self = Self(13);
    pub const ManualProofRequired: Self = Self(14);

    pub fn as_str(&self) -> &'static str {
        COPY_KEY_VALUES[self.0 as usize]
    }

    fn parse(value: &str) -> Option<Self> {
        COPY_KEY_VALUES
            .iter()
            .position(|candidate| *candidate == value)
            .map(|index| Self(index as u8))
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
