use super::protocol_lookup;
use crate::{constants, BrowserChannel};

impl BrowserChannel {
    pub fn from_protocol_str(value: &str) -> Option<Self> {
        protocol_lookup(
            value,
            [
                (constants::browser::CHANNEL_STABLE, Self::Stable),
                (constants::browser::CHANNEL_BETA, Self::Beta),
                (constants::browser::CHANNEL_DEV, Self::Dev),
                (constants::browser::CHANNEL_CANARY, Self::Canary),
                (constants::browser::CHANNEL_UNKNOWN, Self::Unknown),
            ],
        )
    }
}
