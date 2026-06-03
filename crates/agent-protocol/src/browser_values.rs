use crate::{
    constants, BrowserActiveProofSource, BrowserActiveTabCapability, BrowserActiveTabState,
    BrowserCapabilityStatus, BrowserChannel, BrowserCustodyLabel, BrowserExactUrlCapability,
    BrowserFamily, BrowserInventoryInstallState, BrowserManagementTier,
    BrowserQueryVisibilityLabel, BrowserUnmanagedDetectionConfidence,
    BrowserUnmanagedDetectionReason, BrowserUnmanagedProcessKind,
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

impl BrowserActiveProofSource {
    pub fn from_protocol_str(value: &str) -> Option<Self> {
        match value {
            constants::browser::ACTIVE_PROOF_SOURCE_TARGET_LIST_ONLY => Some(Self::TargetListOnly),
            constants::browser::ACTIVE_PROOF_SOURCE_CDP_FOCUS_ACTIVATION => {
                Some(Self::CdpFocusActivation)
            }
            constants::browser::ACTIVE_PROOF_SOURCE_MANAGED_EXTENSION_EVENT => {
                Some(Self::ManagedExtensionEvent)
            }
            constants::browser::ACTIVE_PROOF_SOURCE_FOREGROUND_CORRELATION => {
                Some(Self::ForegroundCorrelation)
            }
            constants::browser::ACTIVE_PROOF_SOURCE_OWNED_SHELL_EVENT => {
                Some(Self::OwnedShellEvent)
            }
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

impl BrowserUnmanagedDetectionConfidence {
    pub fn from_protocol_str(value: &str) -> Option<Self> {
        match value {
            constants::browser::UNMANAGED_DETECTION_CONFIDENCE_HIGH => Some(Self::High),
            constants::browser::UNMANAGED_DETECTION_CONFIDENCE_MEDIUM => Some(Self::Medium),
            constants::browser::UNMANAGED_DETECTION_CONFIDENCE_LOW => Some(Self::Low),
            _ => None,
        }
    }
}

impl BrowserUnmanagedProcessKind {
    pub fn from_protocol_str(value: &str) -> Option<Self> {
        match value {
            constants::browser::UNMANAGED_PROCESS_KIND_SUPPORTED_BROWSER => {
                Some(Self::SupportedBrowser)
            }
            constants::browser::UNMANAGED_PROCESS_KIND_UNSUPPORTED_BROWSER => {
                Some(Self::UnsupportedBrowser)
            }
            constants::browser::UNMANAGED_PROCESS_KIND_PORTABLE_BROWSER => {
                Some(Self::PortableBrowser)
            }
            constants::browser::UNMANAGED_PROCESS_KIND_TOR_PRIVACY_BROWSER => {
                Some(Self::TorPrivacyBrowser)
            }
            constants::browser::UNMANAGED_PROCESS_KIND_PACKAGED_BROWSER => {
                Some(Self::PackagedBrowser)
            }
            constants::browser::UNMANAGED_PROCESS_KIND_EMBEDDED_BROWSER_LIKE => {
                Some(Self::EmbeddedBrowserLike)
            }
            constants::browser::UNMANAGED_PROCESS_KIND_UNKNOWN_BROWSER_LIKE => {
                Some(Self::UnknownBrowserLike)
            }
            constants::browser::UNMANAGED_PROCESS_KIND_POSSIBLE_SOCIAL_BYPASS => {
                Some(Self::PossibleSocialBypass)
            }
            constants::browser::UNMANAGED_PROCESS_KIND_POSSIBLE_BROWSER_GAME_BYPASS => {
                Some(Self::PossibleBrowserGameBypass)
            }
            constants::browser::UNMANAGED_PROCESS_KIND_POSSIBLE_CLOUD_GAMING_BYPASS => {
                Some(Self::PossibleCloudGamingBypass)
            }
            _ => None,
        }
    }
}

impl BrowserUnmanagedDetectionReason {
    pub fn from_protocol_str(value: &str) -> Option<Self> {
        match value {
            constants::browser::UNMANAGED_DETECTION_REASON_SUPPORTED_BROWSER_OUTSIDE_MANAGED_SESSION => {
                Some(Self::SupportedBrowserOutsideManagedSession)
            }
            constants::browser::UNMANAGED_DETECTION_REASON_UNSUPPORTED_BROWSER_PROCESS => {
                Some(Self::UnsupportedBrowserProcess)
            }
            constants::browser::UNMANAGED_DETECTION_REASON_PORTABLE_BROWSER_PROCESS => {
                Some(Self::PortableBrowserProcess)
            }
            constants::browser::UNMANAGED_DETECTION_REASON_TOR_PRIVACY_BROWSER_PROCESS => {
                Some(Self::TorPrivacyBrowserProcess)
            }
            constants::browser::UNMANAGED_DETECTION_REASON_PACKAGED_BROWSER_PROCESS => {
                Some(Self::PackagedBrowserProcess)
            }
            constants::browser::UNMANAGED_DETECTION_REASON_BROWSER_LIKE_PROCESS => {
                Some(Self::BrowserLikeProcess)
            }
            constants::browser::UNMANAGED_DETECTION_REASON_POSSIBLE_SOCIAL_BYPASS => {
                Some(Self::PossibleSocialBypass)
            }
            constants::browser::UNMANAGED_DETECTION_REASON_POSSIBLE_BROWSER_GAME_BYPASS => {
                Some(Self::PossibleBrowserGameBypass)
            }
            constants::browser::UNMANAGED_DETECTION_REASON_POSSIBLE_CLOUD_GAMING_BYPASS => {
                Some(Self::PossibleCloudGamingBypass)
            }
            _ => None,
        }
    }
}

impl BrowserInventoryInstallState {
    pub fn from_protocol_str(value: &str) -> Option<Self> {
        match value {
            constants::browser::INVENTORY_INSTALL_STATE_INSTALLED => Some(Self::Installed),
            constants::browser::INVENTORY_INSTALL_STATE_NOT_INSTALLED => Some(Self::NotInstalled),
            constants::browser::INVENTORY_INSTALL_STATE_CANDIDATE_RUNNING => {
                Some(Self::CandidateRunning)
            }
            constants::browser::INVENTORY_INSTALL_STATE_PACKAGED => Some(Self::Packaged),
            constants::browser::INVENTORY_INSTALL_STATE_PORTABLE => Some(Self::Portable),
            constants::browser::INVENTORY_INSTALL_STATE_UNKNOWN => Some(Self::Unknown),
            _ => None,
        }
    }
}

impl BrowserManagementTier {
    pub fn from_protocol_str(value: &str) -> Option<Self> {
        match value {
            constants::browser::MANAGEMENT_TIER_MANAGED => Some(Self::Managed),
            constants::browser::MANAGEMENT_TIER_OWNED_SHELL => Some(Self::OwnedShell),
            constants::browser::MANAGEMENT_TIER_MANAGED_PROFILE_EXTENSION => {
                Some(Self::ManagedProfileExtension)
            }
            constants::browser::MANAGEMENT_TIER_UNMANAGED => Some(Self::Unmanaged),
            constants::browser::MANAGEMENT_TIER_UNSUPPORTED => Some(Self::Unsupported),
            constants::browser::MANAGEMENT_TIER_MANUAL_REQUIRED => Some(Self::ManualRequired),
            constants::browser::MANAGEMENT_TIER_UNKNOWN => Some(Self::Unknown),
            _ => None,
        }
    }
}

impl BrowserExactUrlCapability {
    pub fn from_protocol_str(value: &str) -> Option<Self> {
        match value {
            constants::browser::EXACT_URL_CAPABILITY_MANAGED_EXACT_URL_AVAILABLE => {
                Some(Self::ManagedExactUrlAvailable)
            }
            constants::browser::EXACT_URL_CAPABILITY_MANAGED_TARGET_LIST_ONLY => {
                Some(Self::ManagedTargetListOnly)
            }
            constants::browser::EXACT_URL_CAPABILITY_MANUAL_REQUIRED => Some(Self::ManualRequired),
            constants::browser::EXACT_URL_CAPABILITY_NOT_CLAIMED => Some(Self::NotClaimed),
            constants::browser::EXACT_URL_CAPABILITY_UNSUPPORTED => Some(Self::Unsupported),
            constants::browser::EXACT_URL_CAPABILITY_UNAVAILABLE => Some(Self::Unavailable),
            _ => None,
        }
    }
}

impl BrowserActiveTabCapability {
    pub fn from_protocol_str(value: &str) -> Option<Self> {
        match value {
            constants::browser::ACTIVE_TAB_CAPABILITY_KNOWN_ACTIVE_SUPPORTED => {
                Some(Self::KnownActiveSupported)
            }
            constants::browser::ACTIVE_TAB_CAPABILITY_TARGET_LIST_ONLY => {
                Some(Self::TargetListOnly)
            }
            constants::browser::ACTIVE_TAB_CAPABILITY_MANUAL_REQUIRED => Some(Self::ManualRequired),
            constants::browser::ACTIVE_TAB_CAPABILITY_NOT_CLAIMED => Some(Self::NotClaimed),
            constants::browser::ACTIVE_TAB_CAPABILITY_UNSUPPORTED => Some(Self::Unsupported),
            constants::browser::ACTIVE_TAB_CAPABILITY_UNAVAILABLE => Some(Self::Unavailable),
            _ => None,
        }
    }
}
