use super::protocol_lookup;
use crate::{constants, BrowserManagementTier};

impl BrowserManagementTier {
    pub fn from_protocol_str(value: &str) -> Option<Self> {
        protocol_lookup(
            value,
            [
                (constants::browser::MANAGEMENT_TIER_MANAGED, Self::Managed),
                (
                    constants::browser::MANAGEMENT_TIER_OWNED_SHELL,
                    Self::OwnedShell,
                ),
                (
                    constants::browser::MANAGEMENT_TIER_MANAGED_PROFILE_EXTENSION,
                    Self::ManagedProfileExtension,
                ),
                (
                    constants::browser::MANAGEMENT_TIER_UNMANAGED,
                    Self::Unmanaged,
                ),
                (
                    constants::browser::MANAGEMENT_TIER_UNSUPPORTED,
                    Self::Unsupported,
                ),
                (
                    constants::browser::MANAGEMENT_TIER_MANUAL_REQUIRED,
                    Self::ManualRequired,
                ),
                (constants::browser::MANAGEMENT_TIER_UNKNOWN, Self::Unknown),
            ],
        )
    }
}
