use super::protocol_lookup;
use crate::{constants, BrowserQueryVisibilityLabel};

impl BrowserQueryVisibilityLabel {
    pub fn from_protocol_str(value: &str) -> Option<Self> {
        protocol_lookup(
            value,
            [
                (
                    constants::browser::QUERY_VISIBILITY_LIVE_LOCAL,
                    Self::LiveLocal,
                ),
                (constants::browser::QUERY_VISIBILITY_LIVE_LAN, Self::LiveLan),
                (
                    constants::browser::QUERY_VISIBILITY_PARENT_CACHE,
                    Self::ParentCache,
                ),
                (
                    constants::browser::QUERY_VISIBILITY_PARENT_OWNED_EXPORT,
                    Self::ParentOwnedExport,
                ),
                (
                    constants::browser::QUERY_VISIBILITY_UNAVAILABLE,
                    Self::Unavailable,
                ),
            ],
        )
    }
}
