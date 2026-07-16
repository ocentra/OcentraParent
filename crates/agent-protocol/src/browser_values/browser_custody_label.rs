use super::protocol_lookup;
use crate::{constants, BrowserCustodyLabel};

impl BrowserCustodyLabel {
    pub fn from_protocol_str(value: &str) -> Option<Self> {
        protocol_lookup(
            value,
            [
                (
                    constants::browser::CUSTODY_CHILD_DEVICE_LOCAL,
                    Self::ChildDeviceLocal,
                ),
                (
                    constants::browser::CUSTODY_LOCAL_NETWORK_CHILD_AGENT,
                    Self::LocalNetworkChildAgent,
                ),
                (constants::browser::CUSTODY_PARENT_CACHE, Self::ParentCache),
                (
                    constants::browser::CUSTODY_PARENT_OWNED_EXPORT,
                    Self::ParentOwnedExport,
                ),
                (constants::browser::CUSTODY_UNAVAILABLE, Self::Unavailable),
            ],
        )
    }
}
