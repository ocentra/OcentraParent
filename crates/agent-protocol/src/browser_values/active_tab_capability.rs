use super::protocol_lookup;
use crate::{constants, BrowserActiveTabCapability};

impl BrowserActiveTabCapability {
    pub fn from_protocol_str(value: &str) -> Option<Self> {
        protocol_lookup(
            value,
            [
                (
                    constants::browser::ACTIVE_TAB_CAPABILITY_KNOWN_ACTIVE_SUPPORTED,
                    Self::KnownActiveSupported,
                ),
                (
                    constants::browser::ACTIVE_TAB_CAPABILITY_TARGET_LIST_ONLY,
                    Self::TargetListOnly,
                ),
                (
                    constants::browser::ACTIVE_TAB_CAPABILITY_MANUAL_REQUIRED,
                    Self::ManualRequired,
                ),
                (
                    constants::browser::ACTIVE_TAB_CAPABILITY_NOT_CLAIMED,
                    Self::NotClaimed,
                ),
                (
                    constants::browser::ACTIVE_TAB_CAPABILITY_UNSUPPORTED,
                    Self::Unsupported,
                ),
                (
                    constants::browser::ACTIVE_TAB_CAPABILITY_UNAVAILABLE,
                    Self::Unavailable,
                ),
            ],
        )
    }
}
