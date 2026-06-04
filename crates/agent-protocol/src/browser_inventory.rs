use serde::{Deserialize, Serialize};

use crate::{
    constants, BrowserCapabilityStatus, BrowserChannel, BrowserCustodyLabel, BrowserFamily,
    BrowserQueryVisibilityLabel,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserInventoryInstallState {
    #[serde(rename = "installed")]
    Installed,
    #[serde(rename = "not-installed")]
    NotInstalled,
    #[serde(rename = "candidate-running")]
    CandidateRunning,
    #[serde(rename = "packaged")]
    Packaged,
    #[serde(rename = "portable")]
    Portable,
    #[serde(rename = "unknown")]
    Unknown,
}

impl BrowserInventoryInstallState {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::Installed => constants::browser::INVENTORY_INSTALL_STATE_INSTALLED,
            Self::NotInstalled => constants::browser::INVENTORY_INSTALL_STATE_NOT_INSTALLED,
            Self::CandidateRunning => constants::browser::INVENTORY_INSTALL_STATE_CANDIDATE_RUNNING,
            Self::Packaged => constants::browser::INVENTORY_INSTALL_STATE_PACKAGED,
            Self::Portable => constants::browser::INVENTORY_INSTALL_STATE_PORTABLE,
            Self::Unknown => constants::browser::INVENTORY_INSTALL_STATE_UNKNOWN,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserInventoryRunningState {
    #[serde(rename = "not-running")]
    NotRunning,
    #[serde(rename = "running-managed")]
    RunningManaged,
    #[serde(rename = "running-unmanaged")]
    RunningUnmanaged,
    #[serde(rename = "running-unknown")]
    RunningUnknown,
    #[serde(rename = "unknown")]
    Unknown,
}

impl BrowserInventoryRunningState {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::NotRunning => constants::browser::INVENTORY_RUNNING_STATE_NOT_RUNNING,
            Self::RunningManaged => constants::browser::INVENTORY_RUNNING_STATE_RUNNING_MANAGED,
            Self::RunningUnmanaged => constants::browser::INVENTORY_RUNNING_STATE_RUNNING_UNMANAGED,
            Self::RunningUnknown => constants::browser::INVENTORY_RUNNING_STATE_RUNNING_UNKNOWN,
            Self::Unknown => constants::browser::INVENTORY_RUNNING_STATE_UNKNOWN,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserManagementTier {
    #[serde(rename = "managed")]
    Managed,
    #[serde(rename = "owned-shell")]
    OwnedShell,
    #[serde(rename = "managed-profile-extension")]
    ManagedProfileExtension,
    #[serde(rename = "unmanaged")]
    Unmanaged,
    #[serde(rename = "unsupported")]
    Unsupported,
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "unknown")]
    Unknown,
}

impl BrowserManagementTier {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::Managed => constants::browser::MANAGEMENT_TIER_MANAGED,
            Self::OwnedShell => constants::browser::MANAGEMENT_TIER_OWNED_SHELL,
            Self::ManagedProfileExtension => {
                constants::browser::MANAGEMENT_TIER_MANAGED_PROFILE_EXTENSION
            }
            Self::Unmanaged => constants::browser::MANAGEMENT_TIER_UNMANAGED,
            Self::Unsupported => constants::browser::MANAGEMENT_TIER_UNSUPPORTED,
            Self::ManualRequired => constants::browser::MANAGEMENT_TIER_MANUAL_REQUIRED,
            Self::Unknown => constants::browser::MANAGEMENT_TIER_UNKNOWN,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserSupportTier {
    #[serde(rename = "managed-url-tab")]
    ManagedUrlTab,
    #[serde(rename = "managed-target-list")]
    ManagedTargetList,
    #[serde(rename = "candidate")]
    Candidate,
    #[serde(rename = "unmanaged-process-only")]
    UnmanagedProcessOnly,
    #[serde(rename = "unsupported")]
    Unsupported,
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "unknown")]
    Unknown,
}

impl BrowserSupportTier {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::ManagedUrlTab => constants::browser::SUPPORT_TIER_MANAGED_URL_TAB,
            Self::ManagedTargetList => constants::browser::SUPPORT_TIER_MANAGED_TARGET_LIST,
            Self::Candidate => constants::browser::SUPPORT_TIER_CANDIDATE,
            Self::UnmanagedProcessOnly => constants::browser::SUPPORT_TIER_UNMANAGED_PROCESS_ONLY,
            Self::Unsupported => constants::browser::SUPPORT_TIER_UNSUPPORTED,
            Self::ManualRequired => constants::browser::SUPPORT_TIER_MANUAL_REQUIRED,
            Self::Unknown => constants::browser::SUPPORT_TIER_UNKNOWN,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserExactUrlCapability {
    #[serde(rename = "managed-exact-url-available")]
    ManagedExactUrlAvailable,
    #[serde(rename = "managed-target-list-only")]
    ManagedTargetListOnly,
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "not-claimed")]
    NotClaimed,
    #[serde(rename = "unsupported")]
    Unsupported,
    #[serde(rename = "unavailable")]
    Unavailable,
}

impl BrowserExactUrlCapability {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::ManagedExactUrlAvailable => {
                constants::browser::EXACT_URL_CAPABILITY_MANAGED_EXACT_URL_AVAILABLE
            }
            Self::ManagedTargetListOnly => {
                constants::browser::EXACT_URL_CAPABILITY_MANAGED_TARGET_LIST_ONLY
            }
            Self::ManualRequired => constants::browser::EXACT_URL_CAPABILITY_MANUAL_REQUIRED,
            Self::NotClaimed => constants::browser::EXACT_URL_CAPABILITY_NOT_CLAIMED,
            Self::Unsupported => constants::browser::EXACT_URL_CAPABILITY_UNSUPPORTED,
            Self::Unavailable => constants::browser::EXACT_URL_CAPABILITY_UNAVAILABLE,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserActiveTabCapability {
    #[serde(rename = "known-active-supported")]
    KnownActiveSupported,
    #[serde(rename = "target-list-only")]
    TargetListOnly,
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "not-claimed")]
    NotClaimed,
    #[serde(rename = "unsupported")]
    Unsupported,
    #[serde(rename = "unavailable")]
    Unavailable,
}

impl BrowserActiveTabCapability {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::KnownActiveSupported => {
                constants::browser::ACTIVE_TAB_CAPABILITY_KNOWN_ACTIVE_SUPPORTED
            }
            Self::TargetListOnly => constants::browser::ACTIVE_TAB_CAPABILITY_TARGET_LIST_ONLY,
            Self::ManualRequired => constants::browser::ACTIVE_TAB_CAPABILITY_MANUAL_REQUIRED,
            Self::NotClaimed => constants::browser::ACTIVE_TAB_CAPABILITY_NOT_CLAIMED,
            Self::Unsupported => constants::browser::ACTIVE_TAB_CAPABILITY_UNSUPPORTED,
            Self::Unavailable => constants::browser::ACTIVE_TAB_CAPABILITY_UNAVAILABLE,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserManagedProfileState {
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "missing")]
    Missing,
    #[serde(rename = "repair-required")]
    RepairRequired,
    #[serde(rename = "not-applicable")]
    NotApplicable,
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "unavailable")]
    Unavailable,
}

impl BrowserManagedProfileState {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::Ready => constants::browser::MANAGED_PROFILE_STATE_READY,
            Self::Missing => constants::browser::MANAGED_PROFILE_STATE_MISSING,
            Self::RepairRequired => constants::browser::MANAGED_PROFILE_STATE_REPAIR_REQUIRED,
            Self::NotApplicable => constants::browser::MANAGED_PROFILE_STATE_NOT_APPLICABLE,
            Self::ManualRequired => constants::browser::MANAGED_PROFILE_STATE_MANUAL_REQUIRED,
            Self::Unavailable => constants::browser::MANAGED_PROFILE_STATE_UNAVAILABLE,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserUnmanagedFallbackCapability {
    #[serde(rename = "report-only")]
    ReportOnly,
    #[serde(rename = "warn-child")]
    WarnChild,
    #[serde(rename = "terminate-process")]
    TerminateProcess,
    #[serde(rename = "relaunch-managed")]
    RelaunchManaged,
    #[serde(rename = "os-block-manual-required")]
    OsBlockManualRequired,
    #[serde(rename = "unsupported")]
    Unsupported,
    #[serde(rename = "unavailable")]
    Unavailable,
}

impl BrowserUnmanagedFallbackCapability {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::ReportOnly => constants::browser::UNMANAGED_FALLBACK_REPORT_ONLY,
            Self::WarnChild => constants::browser::UNMANAGED_FALLBACK_WARN_CHILD,
            Self::TerminateProcess => constants::browser::UNMANAGED_FALLBACK_TERMINATE_PROCESS,
            Self::RelaunchManaged => constants::browser::UNMANAGED_FALLBACK_RELAUNCH_MANAGED,
            Self::OsBlockManualRequired => {
                constants::browser::UNMANAGED_FALLBACK_OS_BLOCK_MANUAL_REQUIRED
            }
            Self::Unsupported => constants::browser::UNMANAGED_FALLBACK_UNSUPPORTED,
            Self::Unavailable => constants::browser::UNMANAGED_FALLBACK_UNAVAILABLE,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserInventoryRow {
    pub schema_version: u16,
    pub inventory_row_id: String,
    pub scanned_at: String,
    pub device_id: String,
    pub browser_family: BrowserFamily,
    pub browser_channel: BrowserChannel,
    pub product_name: String,
    pub browser_version: Option<String>,
    pub install_state: BrowserInventoryInstallState,
    pub running_state: BrowserInventoryRunningState,
    pub management_tier: BrowserManagementTier,
    pub support_tier: BrowserSupportTier,
    pub exact_url_capability: BrowserExactUrlCapability,
    pub active_tab_capability: BrowserActiveTabCapability,
    pub managed_profile_state: BrowserManagedProfileState,
    pub unmanaged_fallback_capability: BrowserUnmanagedFallbackCapability,
    pub executable_path_ref: Option<String>,
    pub publisher_signature_ref: Option<String>,
    pub file_hash_ref: Option<String>,
    pub profile_id: Option<String>,
    pub process_id: Option<u32>,
    pub capability_status: BrowserCapabilityStatus,
    pub reason_code: String,
    pub custody_label: BrowserCustodyLabel,
    pub query_visibility: BrowserQueryVisibilityLabel,
}

impl BrowserInventoryRow {
    pub fn claim_boundary_is_honest(&self) -> bool {
        let exact_url_managed = matches!(
            self.exact_url_capability,
            BrowserExactUrlCapability::ManagedExactUrlAvailable
                | BrowserExactUrlCapability::ManagedTargetListOnly
        );
        let managed_boundary = matches!(
            self.management_tier,
            BrowserManagementTier::Managed
                | BrowserManagementTier::OwnedShell
                | BrowserManagementTier::ManagedProfileExtension
        );
        if exact_url_managed && !managed_boundary {
            return false;
        }
        if self.active_tab_capability == BrowserActiveTabCapability::KnownActiveSupported
            && self.exact_url_capability != BrowserExactUrlCapability::ManagedExactUrlAvailable
        {
            return false;
        }
        if self.management_tier == BrowserManagementTier::Unmanaged
            && self.exact_url_capability != BrowserExactUrlCapability::NotClaimed
        {
            return false;
        }
        if self.management_tier == BrowserManagementTier::Unsupported
            && self.support_tier != BrowserSupportTier::Unsupported
        {
            return false;
        }
        true
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserInventoryReadModel {
    pub schema_version: u16,
    pub generated_at: String,
    pub limit: u64,
    pub returned: u64,
    pub latest_observed_at: Option<String>,
    pub capability_status: Option<BrowserCapabilityStatus>,
    pub custody_label: BrowserCustodyLabel,
    pub query_visibility: BrowserQueryVisibilityLabel,
    pub rows: Vec<BrowserInventoryRow>,
}
