use serde::{Deserialize, Serialize};

use crate::{
    constants, BrowserCapabilityStatus, BrowserChannel, BrowserCustodyLabel, BrowserFamily,
    BrowserQueryVisibilityLabel,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
    const PROTOCOL_STRINGS: [&'static str; 6] = [
        constants::browser::INVENTORY_INSTALL_STATE_INSTALLED,
        constants::browser::INVENTORY_INSTALL_STATE_NOT_INSTALLED,
        constants::browser::INVENTORY_INSTALL_STATE_CANDIDATE_RUNNING,
        constants::browser::INVENTORY_INSTALL_STATE_PACKAGED,
        constants::browser::INVENTORY_INSTALL_STATE_PORTABLE,
        constants::browser::INVENTORY_INSTALL_STATE_UNKNOWN,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
    const PROTOCOL_STRINGS: [&'static str; 5] = [
        constants::browser::INVENTORY_RUNNING_STATE_NOT_RUNNING,
        constants::browser::INVENTORY_RUNNING_STATE_RUNNING_MANAGED,
        constants::browser::INVENTORY_RUNNING_STATE_RUNNING_UNMANAGED,
        constants::browser::INVENTORY_RUNNING_STATE_RUNNING_UNKNOWN,
        constants::browser::INVENTORY_RUNNING_STATE_UNKNOWN,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
    const PROTOCOL_STRINGS: [&'static str; 7] = [
        constants::browser::MANAGEMENT_TIER_MANAGED,
        constants::browser::MANAGEMENT_TIER_OWNED_SHELL,
        constants::browser::MANAGEMENT_TIER_MANAGED_PROFILE_EXTENSION,
        constants::browser::MANAGEMENT_TIER_UNMANAGED,
        constants::browser::MANAGEMENT_TIER_UNSUPPORTED,
        constants::browser::MANAGEMENT_TIER_MANUAL_REQUIRED,
        constants::browser::MANAGEMENT_TIER_UNKNOWN,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
    const PROTOCOL_STRINGS: [&'static str; 7] = [
        constants::browser::SUPPORT_TIER_MANAGED_URL_TAB,
        constants::browser::SUPPORT_TIER_MANAGED_TARGET_LIST,
        constants::browser::SUPPORT_TIER_CANDIDATE,
        constants::browser::SUPPORT_TIER_UNMANAGED_PROCESS_ONLY,
        constants::browser::SUPPORT_TIER_UNSUPPORTED,
        constants::browser::SUPPORT_TIER_MANUAL_REQUIRED,
        constants::browser::SUPPORT_TIER_UNKNOWN,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
    const PROTOCOL_STRINGS: [&'static str; 6] = [
        constants::browser::EXACT_URL_CAPABILITY_MANAGED_EXACT_URL_AVAILABLE,
        constants::browser::EXACT_URL_CAPABILITY_MANAGED_TARGET_LIST_ONLY,
        constants::browser::EXACT_URL_CAPABILITY_MANUAL_REQUIRED,
        constants::browser::EXACT_URL_CAPABILITY_NOT_CLAIMED,
        constants::browser::EXACT_URL_CAPABILITY_UNSUPPORTED,
        constants::browser::EXACT_URL_CAPABILITY_UNAVAILABLE,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
    const PROTOCOL_STRINGS: [&'static str; 6] = [
        constants::browser::ACTIVE_TAB_CAPABILITY_KNOWN_ACTIVE_SUPPORTED,
        constants::browser::ACTIVE_TAB_CAPABILITY_TARGET_LIST_ONLY,
        constants::browser::ACTIVE_TAB_CAPABILITY_MANUAL_REQUIRED,
        constants::browser::ACTIVE_TAB_CAPABILITY_NOT_CLAIMED,
        constants::browser::ACTIVE_TAB_CAPABILITY_UNSUPPORTED,
        constants::browser::ACTIVE_TAB_CAPABILITY_UNAVAILABLE,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
    const PROTOCOL_STRINGS: [&'static str; 6] = [
        constants::browser::MANAGED_PROFILE_STATE_READY,
        constants::browser::MANAGED_PROFILE_STATE_MISSING,
        constants::browser::MANAGED_PROFILE_STATE_REPAIR_REQUIRED,
        constants::browser::MANAGED_PROFILE_STATE_NOT_APPLICABLE,
        constants::browser::MANAGED_PROFILE_STATE_MANUAL_REQUIRED,
        constants::browser::MANAGED_PROFILE_STATE_UNAVAILABLE,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
    const PROTOCOL_STRINGS: [&'static str; 7] = [
        constants::browser::UNMANAGED_FALLBACK_REPORT_ONLY,
        constants::browser::UNMANAGED_FALLBACK_WARN_CHILD,
        constants::browser::UNMANAGED_FALLBACK_TERMINATE_PROCESS,
        constants::browser::UNMANAGED_FALLBACK_RELAUNCH_MANAGED,
        constants::browser::UNMANAGED_FALLBACK_OS_BLOCK_MANUAL_REQUIRED,
        constants::browser::UNMANAGED_FALLBACK_UNSUPPORTED,
        constants::browser::UNMANAGED_FALLBACK_UNAVAILABLE,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
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
