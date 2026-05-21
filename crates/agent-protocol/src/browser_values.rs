use crate::{
    constants, BrowserActiveTabState, BrowserCapabilityStatus, BrowserChannel, BrowserCustodyLabel,
    BrowserFamily, BrowserQueryVisibilityLabel,
};

impl BrowserFamily {
    pub fn from_protocol_str(value: &str) -> Option<Self> {
        match value {
            constants::browser::FAMILY_EDGE => Some(Self::Edge),
            constants::browser::FAMILY_CHROME => Some(Self::Chrome),
            constants::browser::FAMILY_BRAVE => Some(Self::Brave),
            constants::browser::FAMILY_FIREFOX => Some(Self::Firefox),
            constants::browser::FAMILY_OPERA => Some(Self::Opera),
            constants::browser::FAMILY_UNKNOWN_CHROMIUM => Some(Self::UnknownChromium),
            constants::browser::FAMILY_UNKNOWN => Some(Self::Unknown),
            _ => None,
        }
    }
}

impl BrowserChannel {
    pub fn from_protocol_str(value: &str) -> Option<Self> {
        match value {
            constants::browser::CHANNEL_STABLE => Some(Self::Stable),
            constants::browser::CHANNEL_BETA => Some(Self::Beta),
            constants::browser::CHANNEL_DEV => Some(Self::Dev),
            constants::browser::CHANNEL_CANARY => Some(Self::Canary),
            constants::browser::CHANNEL_UNKNOWN => Some(Self::Unknown),
            _ => None,
        }
    }
}

impl BrowserActiveTabState {
    pub fn from_protocol_str(value: &str) -> Option<Self> {
        match value {
            constants::browser::ACTIVE_STATE_KNOWN_ACTIVE => Some(Self::KnownActive),
            constants::browser::ACTIVE_STATE_KNOWN_INACTIVE => Some(Self::KnownInactive),
            constants::browser::ACTIVE_STATE_UNKNOWN => Some(Self::Unknown),
            _ => None,
        }
    }
}

impl BrowserCapabilityStatus {
    pub fn from_protocol_str(value: &str) -> Option<Self> {
        match value {
            constants::browser::CAPABILITY_STATUS_AVAILABLE => Some(Self::Available),
            constants::browser::CAPABILITY_STATUS_TAB_LIST_ONLY => Some(Self::TabListOnly),
            constants::browser::CAPABILITY_STATUS_UNSUPPORTED_BROWSER => {
                Some(Self::UnsupportedBrowser)
            }
            constants::browser::CAPABILITY_STATUS_UNMANAGED_BROWSER => Some(Self::UnmanagedBrowser),
            constants::browser::CAPABILITY_STATUS_MANAGED_PROFILE_MISSING => {
                Some(Self::ManagedProfileMissing)
            }
            constants::browser::CAPABILITY_STATUS_BRIDGE_MISSING => Some(Self::BridgeMissing),
            constants::browser::CAPABILITY_STATUS_PERMISSION_LIMITED => {
                Some(Self::PermissionLimited)
            }
            constants::browser::CAPABILITY_STATUS_STALE => Some(Self::Stale),
            constants::browser::CAPABILITY_STATUS_ADAPTER_ERROR => Some(Self::AdapterError),
            constants::browser::CAPABILITY_STATUS_DISABLED_BY_PARENT => {
                Some(Self::DisabledByParent)
            }
            _ => None,
        }
    }
}

impl BrowserCustodyLabel {
    pub fn from_protocol_str(value: &str) -> Option<Self> {
        match value {
            constants::browser::CUSTODY_CHILD_DEVICE_LOCAL => Some(Self::ChildDeviceLocal),
            constants::browser::CUSTODY_LOCAL_NETWORK_CHILD_AGENT => {
                Some(Self::LocalNetworkChildAgent)
            }
            constants::browser::CUSTODY_PARENT_CACHE => Some(Self::ParentCache),
            constants::browser::CUSTODY_PARENT_OWNED_EXPORT => Some(Self::ParentOwnedExport),
            constants::browser::CUSTODY_UNAVAILABLE => Some(Self::Unavailable),
            _ => None,
        }
    }
}

impl BrowserQueryVisibilityLabel {
    pub fn from_protocol_str(value: &str) -> Option<Self> {
        match value {
            constants::browser::QUERY_VISIBILITY_LIVE_LOCAL => Some(Self::LiveLocal),
            constants::browser::QUERY_VISIBILITY_LIVE_LAN => Some(Self::LiveLan),
            constants::browser::QUERY_VISIBILITY_PARENT_CACHE => Some(Self::ParentCache),
            constants::browser::QUERY_VISIBILITY_PARENT_OWNED_EXPORT => {
                Some(Self::ParentOwnedExport)
            }
            constants::browser::QUERY_VISIBILITY_UNAVAILABLE => Some(Self::Unavailable),
            _ => None,
        }
    }
}
