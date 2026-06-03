use serde::{Deserialize, Serialize};

use crate::{
    constants, BrowserCapabilityStatus, BrowserChannel, BrowserCustodyLabel, BrowserFamily,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserManagedState {
    #[serde(rename = "not-installed")]
    NotInstalled,
    #[serde(rename = "installed-unsupported")]
    InstalledUnsupported,
    #[serde(rename = "installed-supported")]
    InstalledSupported,
    #[serde(rename = "managed-profile-ready")]
    ManagedProfileReady,
    #[serde(rename = "launch-pending")]
    LaunchPending,
    #[serde(rename = "running-managed")]
    RunningManaged,
    #[serde(rename = "bridge-connected")]
    BridgeConnected,
    #[serde(rename = "bridge-disconnected")]
    BridgeDisconnected,
    #[serde(rename = "permission-required")]
    PermissionRequired,
    #[serde(rename = "stopped")]
    Stopped,
    #[serde(rename = "error")]
    Error,
}

impl BrowserManagedState {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::NotInstalled => constants::browser::MANAGED_STATE_NOT_INSTALLED,
            Self::InstalledUnsupported => constants::browser::MANAGED_STATE_INSTALLED_UNSUPPORTED,
            Self::InstalledSupported => constants::browser::MANAGED_STATE_INSTALLED_SUPPORTED,
            Self::ManagedProfileReady => constants::browser::MANAGED_STATE_MANAGED_PROFILE_READY,
            Self::LaunchPending => constants::browser::MANAGED_STATE_LAUNCH_PENDING,
            Self::RunningManaged => constants::browser::MANAGED_STATE_RUNNING_MANAGED,
            Self::BridgeConnected => constants::browser::MANAGED_STATE_BRIDGE_CONNECTED,
            Self::BridgeDisconnected => constants::browser::MANAGED_STATE_BRIDGE_DISCONNECTED,
            Self::PermissionRequired => constants::browser::MANAGED_STATE_PERMISSION_REQUIRED,
            Self::Stopped => constants::browser::MANAGED_STATE_STOPPED,
            Self::Error => constants::browser::MANAGED_STATE_ERROR,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserBridgeKind {
    #[serde(rename = "chromium-devtools-protocol")]
    ChromiumDevtoolsProtocol,
}

impl BrowserBridgeKind {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::ChromiumDevtoolsProtocol => {
                constants::browser::BRIDGE_KIND_CHROMIUM_DEVTOOLS_PROTOCOL
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserQueryVisibilityLabel {
    #[serde(rename = "live-local")]
    LiveLocal,
    #[serde(rename = "live-lan")]
    LiveLan,
    #[serde(rename = "parent-cache")]
    ParentCache,
    #[serde(rename = "parent-owned-export")]
    ParentOwnedExport,
    #[serde(rename = "unavailable")]
    Unavailable,
}

impl BrowserQueryVisibilityLabel {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::LiveLocal => constants::browser::QUERY_VISIBILITY_LIVE_LOCAL,
            Self::LiveLan => constants::browser::QUERY_VISIBILITY_LIVE_LAN,
            Self::ParentCache => constants::browser::QUERY_VISIBILITY_PARENT_CACHE,
            Self::ParentOwnedExport => constants::browser::QUERY_VISIBILITY_PARENT_OWNED_EXPORT,
            Self::Unavailable => constants::browser::QUERY_VISIBILITY_UNAVAILABLE,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserManagedProfileLifecycleState {
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "missing")]
    Missing,
    #[serde(rename = "repair-required")]
    RepairRequired,
    #[serde(rename = "deleted")]
    Deleted,
    #[serde(rename = "unsafe-default-profile")]
    UnsafeDefaultProfile,
    #[serde(rename = "unowned-profile")]
    UnownedProfile,
    #[serde(rename = "unavailable")]
    Unavailable,
}

impl BrowserManagedProfileLifecycleState {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::Ready => constants::browser::PROFILE_STORE_LIFECYCLE_READY,
            Self::Missing => constants::browser::PROFILE_STORE_LIFECYCLE_MISSING,
            Self::RepairRequired => constants::browser::PROFILE_STORE_LIFECYCLE_REPAIR_REQUIRED,
            Self::Deleted => constants::browser::PROFILE_STORE_LIFECYCLE_DELETED,
            Self::UnsafeDefaultProfile => {
                constants::browser::PROFILE_STORE_LIFECYCLE_UNSAFE_DEFAULT_PROFILE
            }
            Self::UnownedProfile => constants::browser::PROFILE_STORE_LIFECYCLE_UNOWNED_PROFILE,
            Self::Unavailable => constants::browser::PROFILE_STORE_LIFECYCLE_UNAVAILABLE,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserUnmanagedDetectionConfidence {
    #[serde(rename = "high")]
    High,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "low")]
    Low,
}

impl BrowserUnmanagedDetectionConfidence {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::High => constants::browser::UNMANAGED_DETECTION_CONFIDENCE_HIGH,
            Self::Medium => constants::browser::UNMANAGED_DETECTION_CONFIDENCE_MEDIUM,
            Self::Low => constants::browser::UNMANAGED_DETECTION_CONFIDENCE_LOW,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserUnmanagedProcessKind {
    #[serde(rename = "supported-browser")]
    SupportedBrowser,
    #[serde(rename = "unsupported-browser")]
    UnsupportedBrowser,
    #[serde(rename = "portable-browser")]
    PortableBrowser,
    #[serde(rename = "tor-privacy-browser")]
    TorPrivacyBrowser,
    #[serde(rename = "packaged-browser")]
    PackagedBrowser,
    #[serde(rename = "embedded-browser-like")]
    EmbeddedBrowserLike,
    #[serde(rename = "unknown-browser-like")]
    UnknownBrowserLike,
    #[serde(rename = "possible-social-bypass")]
    PossibleSocialBypass,
    #[serde(rename = "possible-browser-game-bypass")]
    PossibleBrowserGameBypass,
    #[serde(rename = "possible-cloud-gaming-bypass")]
    PossibleCloudGamingBypass,
}

impl BrowserUnmanagedProcessKind {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::SupportedBrowser => constants::browser::UNMANAGED_PROCESS_KIND_SUPPORTED_BROWSER,
            Self::UnsupportedBrowser => {
                constants::browser::UNMANAGED_PROCESS_KIND_UNSUPPORTED_BROWSER
            }
            Self::PortableBrowser => constants::browser::UNMANAGED_PROCESS_KIND_PORTABLE_BROWSER,
            Self::TorPrivacyBrowser => {
                constants::browser::UNMANAGED_PROCESS_KIND_TOR_PRIVACY_BROWSER
            }
            Self::PackagedBrowser => constants::browser::UNMANAGED_PROCESS_KIND_PACKAGED_BROWSER,
            Self::EmbeddedBrowserLike => {
                constants::browser::UNMANAGED_PROCESS_KIND_EMBEDDED_BROWSER_LIKE
            }
            Self::UnknownBrowserLike => {
                constants::browser::UNMANAGED_PROCESS_KIND_UNKNOWN_BROWSER_LIKE
            }
            Self::PossibleSocialBypass => {
                constants::browser::UNMANAGED_PROCESS_KIND_POSSIBLE_SOCIAL_BYPASS
            }
            Self::PossibleBrowserGameBypass => {
                constants::browser::UNMANAGED_PROCESS_KIND_POSSIBLE_BROWSER_GAME_BYPASS
            }
            Self::PossibleCloudGamingBypass => {
                constants::browser::UNMANAGED_PROCESS_KIND_POSSIBLE_CLOUD_GAMING_BYPASS
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserUnmanagedDetectionReason {
    #[serde(rename = "supported-browser-outside-managed-session")]
    SupportedBrowserOutsideManagedSession,
    #[serde(rename = "unsupported-browser-process")]
    UnsupportedBrowserProcess,
    #[serde(rename = "portable-browser-process")]
    PortableBrowserProcess,
    #[serde(rename = "tor-privacy-browser-process")]
    TorPrivacyBrowserProcess,
    #[serde(rename = "packaged-browser-process")]
    PackagedBrowserProcess,
    #[serde(rename = "browser-like-process")]
    BrowserLikeProcess,
    #[serde(rename = "possible-social-bypass")]
    PossibleSocialBypass,
    #[serde(rename = "possible-browser-game-bypass")]
    PossibleBrowserGameBypass,
    #[serde(rename = "possible-cloud-gaming-bypass")]
    PossibleCloudGamingBypass,
}

impl BrowserUnmanagedDetectionReason {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::SupportedBrowserOutsideManagedSession => {
                constants::browser::UNMANAGED_DETECTION_REASON_SUPPORTED_BROWSER_OUTSIDE_MANAGED_SESSION
            }
            Self::UnsupportedBrowserProcess => {
                constants::browser::UNMANAGED_DETECTION_REASON_UNSUPPORTED_BROWSER_PROCESS
            }
            Self::PortableBrowserProcess => {
                constants::browser::UNMANAGED_DETECTION_REASON_PORTABLE_BROWSER_PROCESS
            }
            Self::TorPrivacyBrowserProcess => {
                constants::browser::UNMANAGED_DETECTION_REASON_TOR_PRIVACY_BROWSER_PROCESS
            }
            Self::PackagedBrowserProcess => {
                constants::browser::UNMANAGED_DETECTION_REASON_PACKAGED_BROWSER_PROCESS
            }
            Self::BrowserLikeProcess => {
                constants::browser::UNMANAGED_DETECTION_REASON_BROWSER_LIKE_PROCESS
            }
            Self::PossibleSocialBypass => {
                constants::browser::UNMANAGED_DETECTION_REASON_POSSIBLE_SOCIAL_BYPASS
            }
            Self::PossibleBrowserGameBypass => {
                constants::browser::UNMANAGED_DETECTION_REASON_POSSIBLE_BROWSER_GAME_BYPASS
            }
            Self::PossibleCloudGamingBypass => {
                constants::browser::UNMANAGED_DETECTION_REASON_POSSIBLE_CLOUD_GAMING_BYPASS
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserManagedProfileStoreEntry {
    pub schema_version: u16,
    pub profile_id: String,
    pub profile_path_ref: String,
    pub profile_root_ref: String,
    pub profile_scope_id: String,
    pub device_id: String,
    pub browser_family: BrowserFamily,
    pub browser_channel: BrowserChannel,
    pub lifecycle_state: BrowserManagedProfileLifecycleState,
    pub custody_label: BrowserCustodyLabel,
    pub policy_revision: String,
    pub created_at: String,
    pub updated_at: String,
    pub missing_since: Option<String>,
    pub repaired_at: Option<String>,
    pub deleted_at: Option<String>,
    pub repair_reason: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserManagedSessionStatus {
    pub schema_version: u16,
    pub checked_at: String,
    pub managed_browser_session_id: Option<String>,
    pub browser_family: Option<BrowserFamily>,
    pub browser_channel: Option<BrowserChannel>,
    pub browser_version: Option<String>,
    pub profile_id: Option<String>,
    pub profile_path_ref: Option<String>,
    pub profile_root_ref: Option<String>,
    pub profile_scope_id: Option<String>,
    pub profile_lifecycle_state: Option<BrowserManagedProfileLifecycleState>,
    pub policy_revision: Option<String>,
    pub process_id: Option<u32>,
    pub bridge_kind: Option<BrowserBridgeKind>,
    pub bridge_endpoint_ref: Option<String>,
    pub unmanaged_process_name: Option<String>,
    pub unmanaged_executable_path_ref: Option<String>,
    pub unmanaged_signature_ref: Option<String>,
    pub unmanaged_process_hash_ref: Option<String>,
    pub unmanaged_process_kind: Option<BrowserUnmanagedProcessKind>,
    pub unmanaged_detection_confidence: Option<BrowserUnmanagedDetectionConfidence>,
    pub unmanaged_detection_reason: Option<BrowserUnmanagedDetectionReason>,
    pub managed_state: BrowserManagedState,
    pub capability_status: BrowserCapabilityStatus,
    pub degraded_reason: Option<String>,
    pub started_at: Option<String>,
    pub custody_label: BrowserCustodyLabel,
    pub query_visibility: BrowserQueryVisibilityLabel,
}
