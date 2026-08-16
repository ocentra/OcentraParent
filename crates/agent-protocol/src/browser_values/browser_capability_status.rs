use super::protocol_lookup;
use crate::{constants, BrowserCapabilityStatus};

impl BrowserCapabilityStatus {
    pub fn from_protocol_str(value: &str) -> Option<Self> {
        protocol_lookup(
            value,
            [
                (
                    constants::browser::CAPABILITY_STATUS_AVAILABLE,
                    Self::Available,
                ),
                (
                    constants::browser::CAPABILITY_STATUS_TAB_LIST_ONLY,
                    Self::TabListOnly,
                ),
                (
                    constants::browser::CAPABILITY_STATUS_UNSUPPORTED_BROWSER,
                    Self::UnsupportedBrowser,
                ),
                (
                    constants::browser::CAPABILITY_STATUS_UNMANAGED_BROWSER,
                    Self::UnmanagedBrowser,
                ),
                (
                    constants::browser::CAPABILITY_STATUS_MANAGED_PROFILE_MISSING,
                    Self::ManagedProfileMissing,
                ),
                (
                    constants::browser::CAPABILITY_STATUS_BRIDGE_MISSING,
                    Self::BridgeMissing,
                ),
                (
                    constants::browser::CAPABILITY_STATUS_PERMISSION_LIMITED,
                    Self::PermissionLimited,
                ),
                (constants::browser::CAPABILITY_STATUS_STALE, Self::Stale),
                (
                    constants::browser::CAPABILITY_STATUS_ADAPTER_ERROR,
                    Self::AdapterError,
                ),
                (
                    constants::browser::CAPABILITY_STATUS_DISABLED_BY_PARENT,
                    Self::DisabledByParent,
                ),
            ],
        )
    }
}
