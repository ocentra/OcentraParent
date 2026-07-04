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
pub struct ParentStorageUiState(u8);

impl ParentStorageUiState {
    pub const ProviderNotConfigured: Self = Self(0);
    pub const ProviderAuthExpired: Self = Self(1);
    pub const ProviderPermissionMissing: Self = Self(2);
    pub const ProviderRevoked: Self = Self(3);
    pub const ProviderQuotaExceeded: Self = Self(4);
    pub const ProviderUnavailable: Self = Self(5);
    pub const LocalStoreUnavailable: Self = Self(6);
    pub const KeyUnavailable: Self = Self(7);
    pub const KeyRevoked: Self = Self(8);
    pub const WrongHousehold: Self = Self(9);
    pub const WrongDevice: Self = Self(10);
    pub const SchemaUnsupported: Self = Self(11);
    pub const BundleCorrupt: Self = Self(12);
    pub const TombstoneConflict: Self = Self(13);
    pub const ManualRequired: Self = Self(14);
    pub const OfflineQueued: Self = Self(15);
    pub const SyncDisabled: Self = Self(16);
    pub const RemoteDisabled: Self = Self(17);
    pub const OcentraHostedStorageNotUsed: Self = Self(18);
    pub const Ready: Self = Self(19);

    pub fn as_str(&self) -> &'static str {
        UI_STATE_VALUES[self.0 as usize]
    }

    fn parse(value: &str) -> Option<Self> {
        UI_STATE_VALUES
            .iter()
            .position(|candidate| *candidate == value)
            .map(|index| Self(index as u8))
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
