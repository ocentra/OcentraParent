use std::path::{Path, PathBuf};

use ocentra_parent_agent_protocol::browser::{
    BrowserCapabilityStatus, BrowserChannel, BrowserFamily,
};
use ocentra_parent_agent_protocol::browser_inventory::BrowserActiveTabCapability;
use ocentra_parent_agent_protocol::browser_inventory::BrowserExactUrlCapability;
use ocentra_parent_agent_protocol::browser_inventory::BrowserInventoryInstallState;
use ocentra_parent_agent_protocol::browser_inventory::BrowserInventoryRunningState;
use ocentra_parent_agent_protocol::browser_inventory::BrowserManagedProfileState;
use ocentra_parent_agent_protocol::browser_inventory::BrowserManagementTier;
use ocentra_parent_agent_protocol::browser_inventory::BrowserSupportTier;
use ocentra_parent_agent_protocol::browser_inventory::BrowserUnmanagedFallbackCapability;
use ocentra_parent_agent_protocol::constants;

use crate::process_capture::ProcessObservation;

#[path = "browser_windows_inventory/identity.rs"]
mod identity;
#[path = "browser_windows_inventory/observation.rs"]
mod observation;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BrowserWindowsSupportKind {
    ManagedChromium,
    ManualChromium,
    Unsupported,
    Unknown,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserWindowsExecutableIdentity {
    pub browser_family: BrowserFamily,
    pub browser_channel: BrowserChannel,
    pub product_name: &'static str,
    pub supports_managed_cdp: bool,
    support_kind: BrowserWindowsSupportKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserWindowsInventoryObservation {
    pub executable_path: Option<PathBuf>,
    pub process_id: Option<u32>,
    pub product_name: String,
    pub browser_family: BrowserFamily,
    pub browser_channel: BrowserChannel,
    pub install_state: BrowserInventoryInstallState,
    pub running_state: BrowserInventoryRunningState,
    pub management_tier: BrowserManagementTier,
    pub support_tier: BrowserSupportTier,
    pub exact_url_capability: BrowserExactUrlCapability,
    pub active_tab_capability: BrowserActiveTabCapability,
    pub managed_profile_state: BrowserManagedProfileState,
    pub unmanaged_fallback_capability: BrowserUnmanagedFallbackCapability,
    pub capability_status: BrowserCapabilityStatus,
    pub reason_code: &'static str,
}

pub fn windows_browser_inventory_observations(
    candidate_paths: &[PathBuf],
    process_observations: &[ProcessObservation],
    managed_process_id: Option<u32>,
) -> Vec<BrowserWindowsInventoryObservation> {
    observation::windows_browser_inventory_observations(
        candidate_paths,
        process_observations,
        managed_process_id,
    )
}

pub fn windows_browser_executable_identity(path: &Path) -> BrowserWindowsExecutableIdentity {
    identity::windows_browser_executable_identity(path)
}

pub(super) fn process_identity_path(process: &ProcessObservation) -> &Path {
    process
        .executable_path
        .as_deref()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new(&process.name))
}

pub(super) fn install_state_from_path(path: &Path) -> BrowserInventoryInstallState {
    let components = normalized_component_names(path);
    if components
        .iter()
        .any(|name| name == constants::browser::PATH_SEGMENT_WINDOWS_APPS_NORMALIZED)
    {
        return BrowserInventoryInstallState::Packaged;
    }
    if components
        .iter()
        .any(|name| name.contains(constants::browser::PATH_SEGMENT_PORTABLE_NORMALIZED))
    {
        return BrowserInventoryInstallState::Portable;
    }
    BrowserInventoryInstallState::Installed
}

pub(super) fn executable_name_normalized(path: &Path) -> String {
    normalized_component_names(path)
        .last()
        .cloned()
        .unwrap_or_default()
}

pub(super) fn normalized_component_names(path: &Path) -> Vec<String> {
    path.to_string_lossy()
        .split(['/', '\\'])
        .filter(|component| !component.is_empty())
        .map(str::to_ascii_lowercase)
        .collect()
}
