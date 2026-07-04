use super::protocol_lookup;
use crate::{constants, BrowserActiveTabState};

impl BrowserActiveTabState {
    pub fn from_protocol_str(value: &str) -> Option<Self> {
        protocol_lookup(
            value,
            [
                (
                    constants::browser::ACTIVE_STATE_KNOWN_ACTIVE,
                    Self::KnownActive,
                ),
                (
                    constants::browser::ACTIVE_STATE_KNOWN_INACTIVE,
                    Self::KnownInactive,
                ),
                (constants::browser::ACTIVE_STATE_UNKNOWN, Self::Unknown),
            ],
        )
    }
}
