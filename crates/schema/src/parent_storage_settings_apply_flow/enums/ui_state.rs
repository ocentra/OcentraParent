use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::super::constants::{
    PARENT_STORAGE_UI_STATE_BUNDLE_CORRUPT, PARENT_STORAGE_UI_STATE_KEY_REVOKED,
    PARENT_STORAGE_UI_STATE_KEY_UNAVAILABLE, PARENT_STORAGE_UI_STATE_LOCAL_STORE_UNAVAILABLE,
    PARENT_STORAGE_UI_STATE_MANUAL_REQUIRED,
    PARENT_STORAGE_UI_STATE_OCENTRA_HOSTED_STORAGE_NOT_USED,
    PARENT_STORAGE_UI_STATE_OFFLINE_QUEUED, PARENT_STORAGE_UI_STATE_PROVIDER_AUTH_EXPIRED,
    PARENT_STORAGE_UI_STATE_PROVIDER_NOT_CONFIGURED,
    PARENT_STORAGE_UI_STATE_PROVIDER_PERMISSION_MISSING,
    PARENT_STORAGE_UI_STATE_PROVIDER_QUOTA_EXCEEDED, PARENT_STORAGE_UI_STATE_PROVIDER_REVOKED,
    PARENT_STORAGE_UI_STATE_PROVIDER_UNAVAILABLE, PARENT_STORAGE_UI_STATE_READY,
    PARENT_STORAGE_UI_STATE_REMOTE_DISABLED, PARENT_STORAGE_UI_STATE_SCHEMA_UNSUPPORTED,
    PARENT_STORAGE_UI_STATE_SYNC_DISABLED, PARENT_STORAGE_UI_STATE_TOMBSTONE_CONFLICT,
    PARENT_STORAGE_UI_STATE_WRONG_DEVICE, PARENT_STORAGE_UI_STATE_WRONG_HOUSEHOLD,
};

const UI_STATE_VALUES: [&str; 20] = [
    PARENT_STORAGE_UI_STATE_PROVIDER_NOT_CONFIGURED,
    PARENT_STORAGE_UI_STATE_PROVIDER_AUTH_EXPIRED,
    PARENT_STORAGE_UI_STATE_PROVIDER_PERMISSION_MISSING,
    PARENT_STORAGE_UI_STATE_PROVIDER_REVOKED,
    PARENT_STORAGE_UI_STATE_PROVIDER_QUOTA_EXCEEDED,
    PARENT_STORAGE_UI_STATE_PROVIDER_UNAVAILABLE,
    PARENT_STORAGE_UI_STATE_LOCAL_STORE_UNAVAILABLE,
    PARENT_STORAGE_UI_STATE_KEY_UNAVAILABLE,
    PARENT_STORAGE_UI_STATE_KEY_REVOKED,
    PARENT_STORAGE_UI_STATE_WRONG_HOUSEHOLD,
    PARENT_STORAGE_UI_STATE_WRONG_DEVICE,
    PARENT_STORAGE_UI_STATE_SCHEMA_UNSUPPORTED,
    PARENT_STORAGE_UI_STATE_BUNDLE_CORRUPT,
    PARENT_STORAGE_UI_STATE_TOMBSTONE_CONFLICT,
    PARENT_STORAGE_UI_STATE_MANUAL_REQUIRED,
    PARENT_STORAGE_UI_STATE_OFFLINE_QUEUED,
    PARENT_STORAGE_UI_STATE_SYNC_DISABLED,
    PARENT_STORAGE_UI_STATE_REMOTE_DISABLED,
    PARENT_STORAGE_UI_STATE_OCENTRA_HOSTED_STORAGE_NOT_USED,
    PARENT_STORAGE_UI_STATE_READY,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(u8)]
pub enum ParentStorageUiState {
    ProviderNotConfigured,
    ProviderAuthExpired,
    ProviderPermissionMissing,
    ProviderRevoked,
    ProviderQuotaExceeded,
    ProviderUnavailable,
    LocalStoreUnavailable,
    KeyUnavailable,
    KeyRevoked,
    WrongHousehold,
    WrongDevice,
    SchemaUnsupported,
    BundleCorrupt,
    TombstoneConflict,
    ManualRequired,
    OfflineQueued,
    SyncDisabled,
    RemoteDisabled,
    OcentraHostedStorageNotUsed,
    Ready,
}

impl ParentStorageUiState {
    pub fn as_str(&self) -> &'static str {
        UI_STATE_VALUES[*self as usize]
    }

    fn parse(value: &str) -> Option<Self> {
        let variants = [
            Self::ProviderNotConfigured,
            Self::ProviderAuthExpired,
            Self::ProviderPermissionMissing,
            Self::ProviderRevoked,
            Self::ProviderQuotaExceeded,
            Self::ProviderUnavailable,
            Self::LocalStoreUnavailable,
            Self::KeyUnavailable,
            Self::KeyRevoked,
            Self::WrongHousehold,
            Self::WrongDevice,
            Self::SchemaUnsupported,
            Self::BundleCorrupt,
            Self::TombstoneConflict,
            Self::ManualRequired,
            Self::OfflineQueued,
            Self::SyncDisabled,
            Self::RemoteDisabled,
            Self::OcentraHostedStorageNotUsed,
            Self::Ready,
        ];

        UI_STATE_VALUES
            .iter()
            .position(|candidate| *candidate == value)
            .map(|index| variants[index])
    }
}

impl Serialize for ParentStorageUiState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for ParentStorageUiState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::parse(&value).ok_or_else(|| D::Error::custom("invalid parent storage ui state"))
    }
}
