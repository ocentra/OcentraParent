use serde::{Deserialize, Serialize};

use crate::constants;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserFamily {
    #[serde(rename = "edge")]
    Edge,
    #[serde(rename = "chrome")]
    Chrome,
    #[serde(rename = "brave")]
    Brave,
    #[serde(rename = "firefox")]
    Firefox,
    #[serde(rename = "opera")]
    Opera,
    #[serde(rename = "unknown-chromium")]
    UnknownChromium,
    #[serde(rename = "unknown")]
    Unknown,
}

impl BrowserFamily {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::Edge => constants::browser::FAMILY_EDGE,
            Self::Chrome => constants::browser::FAMILY_CHROME,
            Self::Brave => constants::browser::FAMILY_BRAVE,
            Self::Firefox => constants::browser::FAMILY_FIREFOX,
            Self::Opera => constants::browser::FAMILY_OPERA,
            Self::UnknownChromium => constants::browser::FAMILY_UNKNOWN_CHROMIUM,
            Self::Unknown => constants::browser::FAMILY_UNKNOWN,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserChannel {
    #[serde(rename = "stable")]
    Stable,
    #[serde(rename = "beta")]
    Beta,
    #[serde(rename = "dev")]
    Dev,
    #[serde(rename = "canary")]
    Canary,
    #[serde(rename = "unknown")]
    Unknown,
}

impl BrowserChannel {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::Stable => constants::browser::CHANNEL_STABLE,
            Self::Beta => constants::browser::CHANNEL_BETA,
            Self::Dev => constants::browser::CHANNEL_DEV,
            Self::Canary => constants::browser::CHANNEL_CANARY,
            Self::Unknown => constants::browser::CHANNEL_UNKNOWN,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserActiveTabState {
    #[serde(rename = "known-active")]
    KnownActive,
    #[serde(rename = "known-inactive")]
    KnownInactive,
    #[serde(rename = "unknown")]
    Unknown,
}

impl BrowserActiveTabState {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::KnownActive => constants::browser::ACTIVE_STATE_KNOWN_ACTIVE,
            Self::KnownInactive => constants::browser::ACTIVE_STATE_KNOWN_INACTIVE,
            Self::Unknown => constants::browser::ACTIVE_STATE_UNKNOWN,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserActiveProofSource {
    #[serde(rename = "target-list-only")]
    TargetListOnly,
    #[serde(rename = "cdp-focus-activation")]
    CdpFocusActivation,
    #[serde(rename = "managed-extension-event")]
    ManagedExtensionEvent,
    #[serde(rename = "foreground-correlation")]
    ForegroundCorrelation,
    #[serde(rename = "owned-shell-event")]
    OwnedShellEvent,
}

impl BrowserActiveProofSource {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::TargetListOnly => constants::browser::ACTIVE_PROOF_SOURCE_TARGET_LIST_ONLY,
            Self::CdpFocusActivation => {
                constants::browser::ACTIVE_PROOF_SOURCE_CDP_FOCUS_ACTIVATION
            }
            Self::ManagedExtensionEvent => {
                constants::browser::ACTIVE_PROOF_SOURCE_MANAGED_EXTENSION_EVENT
            }
            Self::ForegroundCorrelation => {
                constants::browser::ACTIVE_PROOF_SOURCE_FOREGROUND_CORRELATION
            }
            Self::OwnedShellEvent => constants::browser::ACTIVE_PROOF_SOURCE_OWNED_SHELL_EVENT,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserCapabilityStatus {
    #[serde(rename = "available")]
    Available,
    #[serde(rename = "tab-list-only")]
    TabListOnly,
    #[serde(rename = "unsupported-browser")]
    UnsupportedBrowser,
    #[serde(rename = "unmanaged-browser")]
    UnmanagedBrowser,
    #[serde(rename = "managed-profile-missing")]
    ManagedProfileMissing,
    #[serde(rename = "bridge-missing")]
    BridgeMissing,
    #[serde(rename = "permission-limited")]
    PermissionLimited,
    #[serde(rename = "stale")]
    Stale,
    #[serde(rename = "adapter-error")]
    AdapterError,
    #[serde(rename = "disabled-by-parent")]
    DisabledByParent,
}

impl BrowserCapabilityStatus {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::Available => constants::browser::CAPABILITY_STATUS_AVAILABLE,
            Self::TabListOnly => constants::browser::CAPABILITY_STATUS_TAB_LIST_ONLY,
            Self::UnsupportedBrowser => constants::browser::CAPABILITY_STATUS_UNSUPPORTED_BROWSER,
            Self::UnmanagedBrowser => constants::browser::CAPABILITY_STATUS_UNMANAGED_BROWSER,
            Self::ManagedProfileMissing => {
                constants::browser::CAPABILITY_STATUS_MANAGED_PROFILE_MISSING
            }
            Self::BridgeMissing => constants::browser::CAPABILITY_STATUS_BRIDGE_MISSING,
            Self::PermissionLimited => constants::browser::CAPABILITY_STATUS_PERMISSION_LIMITED,
            Self::Stale => constants::browser::CAPABILITY_STATUS_STALE,
            Self::AdapterError => constants::browser::CAPABILITY_STATUS_ADAPTER_ERROR,
            Self::DisabledByParent => constants::browser::CAPABILITY_STATUS_DISABLED_BY_PARENT,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserCustodyLabel {
    #[serde(rename = "child-device-local")]
    ChildDeviceLocal,
    #[serde(rename = "local-network-child-agent")]
    LocalNetworkChildAgent,
    #[serde(rename = "parent-cache")]
    ParentCache,
    #[serde(rename = "parent-owned-export")]
    ParentOwnedExport,
    #[serde(rename = "unavailable")]
    Unavailable,
}

impl BrowserCustodyLabel {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::ChildDeviceLocal => constants::browser::CUSTODY_CHILD_DEVICE_LOCAL,
            Self::LocalNetworkChildAgent => constants::browser::CUSTODY_LOCAL_NETWORK_CHILD_AGENT,
            Self::ParentCache => constants::browser::CUSTODY_PARENT_CACHE,
            Self::ParentOwnedExport => constants::browser::CUSTODY_PARENT_OWNED_EXPORT,
            Self::Unavailable => constants::browser::CUSTODY_UNAVAILABLE,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserEvidenceRecentSummary {
    pub schema_version: u16,
    pub returned: u64,
    pub latest_event_id: Option<String>,
    pub latest_observed_at: Option<String>,
    pub browser_evidence_id: Option<String>,
    pub source_id: Option<String>,
    pub adapter_id: Option<String>,
    pub managed_browser_session_id: Option<String>,
    pub browser_family: Option<String>,
    pub active_state: Option<String>,
    pub active_proof_source: Option<String>,
    pub url: Option<String>,
    pub origin: Option<String>,
    pub domain: Option<String>,
    pub title: Option<String>,
    pub capability_status: Option<String>,
    pub custody_label: Option<String>,
}
