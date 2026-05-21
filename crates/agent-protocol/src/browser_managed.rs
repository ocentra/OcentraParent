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
    pub process_id: Option<u32>,
    pub bridge_kind: Option<BrowserBridgeKind>,
    pub bridge_endpoint_ref: Option<String>,
    pub managed_state: BrowserManagedState,
    pub capability_status: BrowserCapabilityStatus,
    pub degraded_reason: Option<String>,
    pub started_at: Option<String>,
    pub custody_label: BrowserCustodyLabel,
    pub query_visibility: BrowserQueryVisibilityLabel,
}
