use serde::{Deserialize, Serialize};

use super::super::constants::{
    PARENT_STORAGE_MODE_LABEL_DISABLED, PARENT_STORAGE_MODE_LABEL_LOCAL_ONLY,
    PARENT_STORAGE_MODE_LABEL_LOCAL_PLUS_ENCRYPTED_BACKUP,
    PARENT_STORAGE_MODE_LABEL_LOCAL_PLUS_ENCRYPTED_PROVIDER_SYNC,
    PARENT_STORAGE_MODE_LABEL_MANUAL_REQUIRED, PARENT_STORAGE_MODE_LABEL_PROVIDER_DISCONNECTED,
    PARENT_STORAGE_MODE_LABEL_PROVIDER_ERROR,
};

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
        const VALUES: &[&str] = &[
            PARENT_STORAGE_MODE_LABEL_LOCAL_ONLY,
            PARENT_STORAGE_MODE_LABEL_LOCAL_PLUS_ENCRYPTED_BACKUP,
            PARENT_STORAGE_MODE_LABEL_LOCAL_PLUS_ENCRYPTED_PROVIDER_SYNC,
            PARENT_STORAGE_MODE_LABEL_PROVIDER_DISCONNECTED,
            PARENT_STORAGE_MODE_LABEL_PROVIDER_ERROR,
            PARENT_STORAGE_MODE_LABEL_MANUAL_REQUIRED,
            PARENT_STORAGE_MODE_LABEL_DISABLED,
        ];
        VALUES[*self as usize]
    }
}
