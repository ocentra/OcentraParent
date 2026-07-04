use serde::{Deserialize, Serialize};

use super::super::constants::{
    PARENT_STORAGE_KEY_STATUS_KEY_AVAILABLE, PARENT_STORAGE_KEY_STATUS_KEY_REVOKED,
    PARENT_STORAGE_KEY_STATUS_KEY_UNAVAILABLE, PARENT_STORAGE_KEY_STATUS_MANUAL_REQUIRED,
};

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
        const VALUES: &[&str] = &[
            PARENT_STORAGE_KEY_STATUS_KEY_AVAILABLE,
            PARENT_STORAGE_KEY_STATUS_KEY_UNAVAILABLE,
            PARENT_STORAGE_KEY_STATUS_KEY_REVOKED,
            PARENT_STORAGE_KEY_STATUS_MANUAL_REQUIRED,
        ];
        VALUES[*self as usize]
    }
}
