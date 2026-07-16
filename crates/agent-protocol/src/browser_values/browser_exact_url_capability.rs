use super::protocol_lookup;
use crate::{constants, BrowserExactUrlCapability};

impl BrowserExactUrlCapability {
    pub fn from_protocol_str(value: &str) -> Option<Self> {
        protocol_lookup(
            value,
            [
                (
                    constants::browser::EXACT_URL_CAPABILITY_MANAGED_EXACT_URL_AVAILABLE,
                    Self::ManagedExactUrlAvailable,
                ),
                (
                    constants::browser::EXACT_URL_CAPABILITY_MANAGED_TARGET_LIST_ONLY,
                    Self::ManagedTargetListOnly,
                ),
                (
                    constants::browser::EXACT_URL_CAPABILITY_MANUAL_REQUIRED,
                    Self::ManualRequired,
                ),
                (
                    constants::browser::EXACT_URL_CAPABILITY_NOT_CLAIMED,
                    Self::NotClaimed,
                ),
                (
                    constants::browser::EXACT_URL_CAPABILITY_UNSUPPORTED,
                    Self::Unsupported,
                ),
                (
                    constants::browser::EXACT_URL_CAPABILITY_UNAVAILABLE,
                    Self::Unavailable,
                ),
            ],
        )
    }
}
