use std::path::{Path, PathBuf};

use ocentra_parent_agent_protocol::browser::{
    BrowserCapabilityStatus, BrowserChannel, BrowserFamily,
};
use ocentra_parent_agent_protocol::browser_inventory::{
    BrowserActiveTabCapability, BrowserExactUrlCapability, BrowserInventoryInstallState,
    BrowserInventoryRunningState, BrowserManagedProfileState, BrowserManagementTier,
    BrowserSupportTier, BrowserUnmanagedFallbackCapability,
};
use ocentra_parent_agent_protocol::constants;

use crate::{
    browser_windows_inventory::BrowserWindowsInventoryObservation,
    process_capture::ProcessObservation,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserPlatformInventoryObservation {
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

impl From<&BrowserWindowsInventoryObservation> for BrowserPlatformInventoryObservation {
    fn from(observation: &BrowserWindowsInventoryObservation) -> Self {
        Self {
            executable_path: observation.executable_path.clone(),
            process_id: observation.process_id,
            product_name: observation.product_name.clone(),
            browser_family: observation.browser_family,
            browser_channel: observation.browser_channel,
            install_state: observation.install_state,
            running_state: observation.running_state,
            management_tier: observation.management_tier,
            support_tier: observation.support_tier,
            exact_url_capability: observation.exact_url_capability,
            active_tab_capability: observation.active_tab_capability,
            managed_profile_state: observation.managed_profile_state,
            unmanaged_fallback_capability: observation.unmanaged_fallback_capability,
            capability_status: observation.capability_status,
            reason_code: observation.reason_code,
        }
    }
}

pub fn browser_platform_inventory_observations(
    candidate_paths: &[PathBuf],
    process_observations: &[ProcessObservation],
    managed_process_id: Option<u32>,
) -> Vec<BrowserPlatformInventoryObservation> {
    if cfg!(windows) {
        return crate::browser_windows_inventory::windows_browser_inventory_observations(
            candidate_paths,
            process_observations,
            managed_process_id,
        )
        .iter()
        .map(BrowserPlatformInventoryObservation::from)
        .collect();
    }

    process_observations
        .iter()
        .filter(|process| managed_process_id != Some(process.pid))
        .filter_map(manual_platform_process_observation)
        .collect()
}

fn manual_platform_process_observation(
    process: &ProcessObservation,
) -> Option<BrowserPlatformInventoryObservation> {
    let identity = platform_browser_identity(process)?;
    Some(BrowserPlatformInventoryObservation {
        executable_path: process
            .executable_path
            .as_deref()
            .filter(|path| !path.as_os_str().is_empty())
            .map(Path::to_path_buf),
        process_id: Some(process.pid),
        product_name: identity.product_name.to_string(),
        browser_family: identity.browser_family,
        browser_channel: BrowserChannel::Unknown,
        install_state: BrowserInventoryInstallState::CandidateRunning,
        running_state: BrowserInventoryRunningState::RunningUnknown,
        management_tier: BrowserManagementTier::ManualRequired,
        support_tier: BrowserSupportTier::ManualRequired,
        exact_url_capability: BrowserExactUrlCapability::ManualRequired,
        active_tab_capability: BrowserActiveTabCapability::ManualRequired,
        managed_profile_state: BrowserManagedProfileState::ManualRequired,
        unmanaged_fallback_capability: BrowserUnmanagedFallbackCapability::ReportOnly,
        capability_status: BrowserCapabilityStatus::PermissionLimited,
        reason_code: constants::browser::INVENTORY_REASON_CROSS_PLATFORM_MANUAL_REQUIRED,
    })
}

struct PlatformBrowserIdentity {
    browser_family: BrowserFamily,
    product_name: &'static str,
}

fn platform_browser_identity(process: &ProcessObservation) -> Option<PlatformBrowserIdentity> {
    let name = process
        .executable_path
        .as_deref()
        .and_then(Path::file_name)
        .map(|name| name.to_string_lossy().to_ascii_lowercase())
        .unwrap_or_else(|| process.name.to_ascii_lowercase());
    let name = name.strip_suffix(".exe").unwrap_or(&name);

    let identity = match name {
        "chrome" | "google-chrome" => PlatformBrowserIdentity {
            browser_family: BrowserFamily::Chrome,
            product_name: constants::browser::PRODUCT_NAME_GOOGLE_CHROME,
        },
        "chromium" => PlatformBrowserIdentity {
            browser_family: BrowserFamily::UnknownChromium,
            product_name: constants::browser::PRODUCT_NAME_CHROMIUM,
        },
        "msedge" | "microsoft-edge" => PlatformBrowserIdentity {
            browser_family: BrowserFamily::Edge,
            product_name: constants::browser::PRODUCT_NAME_MICROSOFT_EDGE,
        },
        "firefox" => PlatformBrowserIdentity {
            browser_family: BrowserFamily::Firefox,
            product_name: constants::browser::PRODUCT_NAME_MOZILLA_FIREFOX,
        },
        "brave" | "brave-browser" => PlatformBrowserIdentity {
            browser_family: BrowserFamily::Brave,
            product_name: constants::browser::PRODUCT_NAME_BRAVE_BROWSER,
        },
        "opera" | "opera_sandbox" => PlatformBrowserIdentity {
            browser_family: BrowserFamily::Opera,
            product_name: constants::browser::PRODUCT_NAME_OPERA_BROWSER,
        },
        "safari" => PlatformBrowserIdentity {
            browser_family: BrowserFamily::Unknown,
            product_name: constants::browser::PRODUCT_NAME_SAFARI_BROWSER,
        },
        _ => return None,
    };
    Some(identity)
}
