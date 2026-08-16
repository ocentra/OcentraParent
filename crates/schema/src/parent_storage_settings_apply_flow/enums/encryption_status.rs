use serde::{Deserialize, Serialize};

use super::super::constants::{
    PARENT_STORAGE_ENCRYPTION_STATUS_ENCRYPTED_BEFORE_UPLOAD,
    PARENT_STORAGE_ENCRYPTION_STATUS_HUMAN_READABLE_PARENT_AUTHORIZED,
    PARENT_STORAGE_ENCRYPTION_STATUS_MANUAL_REQUIRED,
    PARENT_STORAGE_ENCRYPTION_STATUS_NOT_APPLICABLE,
};

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
        const VALUES: &[&str] = &[
            PARENT_STORAGE_ENCRYPTION_STATUS_ENCRYPTED_BEFORE_UPLOAD,
            PARENT_STORAGE_ENCRYPTION_STATUS_HUMAN_READABLE_PARENT_AUTHORIZED,
            PARENT_STORAGE_ENCRYPTION_STATUS_NOT_APPLICABLE,
            PARENT_STORAGE_ENCRYPTION_STATUS_MANUAL_REQUIRED,
        ];
        VALUES[*self as usize]
    }
}
