use std::path::Path;

use ocentra_parent_agent_protocol::browser::BrowserCapabilityStatus;
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

use super::{
    BrowserWindowsExecutableIdentity, BrowserWindowsInventoryObservation, BrowserWindowsSupportKind,
};

pub(super) fn windows_browser_inventory_process_observation(
    process: &ProcessObservation,
    managed_process_id: Option<u32>,
) -> Option<BrowserWindowsInventoryObservation> {
    if managed_process_id == Some(process.pid) {
        return None;
    }
    let identity =
        super::windows_browser_executable_identity(super::process_identity_path(process));
    match identity.support_kind {
        BrowserWindowsSupportKind::ManagedChromium | BrowserWindowsSupportKind::ManualChromium => {
            Some(unmanaged_process_observation(process, &identity))
        }
        BrowserWindowsSupportKind::Unsupported => {
            Some(unsupported_process_observation(process, &identity))
        }
        BrowserWindowsSupportKind::Unknown => None,
    }
}

pub(super) fn managed_chromium_path_observation(
    path: &Path,
    identity: &BrowserWindowsExecutableIdentity,
) -> BrowserWindowsInventoryObservation {
    BrowserWindowsInventoryObservation {
        executable_path: Some(path.to_path_buf()),
        process_id: None,
        product_name: identity.product_name.to_string(),
        browser_family: identity.browser_family,
        browser_channel: identity.browser_channel,
        install_state: super::install_state_from_path(path),
        running_state: BrowserInventoryRunningState::NotRunning,
        management_tier: BrowserManagementTier::Managed,
        support_tier: BrowserSupportTier::Candidate,
        exact_url_capability: BrowserExactUrlCapability::Unavailable,
        active_tab_capability: BrowserActiveTabCapability::Unavailable,
        managed_profile_state: BrowserManagedProfileState::Missing,
        unmanaged_fallback_capability: BrowserUnmanagedFallbackCapability::OsBlockManualRequired,
        capability_status: BrowserCapabilityStatus::ManagedProfileMissing,
        reason_code: constants::browser::INVENTORY_REASON_WINDOWS_MANAGED_PROFILE_REQUIRED,
    }
}

pub(super) fn manual_chromium_path_observation(
    path: &Path,
    identity: &BrowserWindowsExecutableIdentity,
) -> BrowserWindowsInventoryObservation {
    BrowserWindowsInventoryObservation {
        executable_path: Some(path.to_path_buf()),
        process_id: None,
        product_name: identity.product_name.to_string(),
        browser_family: identity.browser_family,
        browser_channel: identity.browser_channel,
        install_state: super::install_state_from_path(path),
        running_state: BrowserInventoryRunningState::NotRunning,
        management_tier: BrowserManagementTier::ManualRequired,
        support_tier: BrowserSupportTier::Candidate,
        exact_url_capability: BrowserExactUrlCapability::ManualRequired,
        active_tab_capability: BrowserActiveTabCapability::ManualRequired,
        managed_profile_state: BrowserManagedProfileState::ManualRequired,
        unmanaged_fallback_capability: BrowserUnmanagedFallbackCapability::ReportOnly,
        capability_status: BrowserCapabilityStatus::PermissionLimited,
        reason_code: constants::browser::INVENTORY_REASON_WINDOWS_CHROMIUM_FORK_MANUAL_REQUIRED,
    }
}

pub(super) fn unsupported_path_observation(
    path: &Path,
    identity: &BrowserWindowsExecutableIdentity,
) -> BrowserWindowsInventoryObservation {
    BrowserWindowsInventoryObservation {
        executable_path: Some(path.to_path_buf()),
        process_id: None,
        product_name: identity.product_name.to_string(),
        browser_family: identity.browser_family,
        browser_channel: identity.browser_channel,
        install_state: super::install_state_from_path(path),
        running_state: BrowserInventoryRunningState::NotRunning,
        management_tier: BrowserManagementTier::Unsupported,
        support_tier: BrowserSupportTier::Unsupported,
        exact_url_capability: BrowserExactUrlCapability::Unsupported,
        active_tab_capability: BrowserActiveTabCapability::Unsupported,
        managed_profile_state: BrowserManagedProfileState::NotApplicable,
        unmanaged_fallback_capability: BrowserUnmanagedFallbackCapability::Unsupported,
        capability_status: BrowserCapabilityStatus::UnsupportedBrowser,
        reason_code: constants::browser::INVENTORY_REASON_WINDOWS_UNSUPPORTED_LATER_ADAPTER,
    }
}

fn unmanaged_process_observation(
    process: &ProcessObservation,
    identity: &BrowserWindowsExecutableIdentity,
) -> BrowserWindowsInventoryObservation {
    BrowserWindowsInventoryObservation {
        executable_path: process
            .executable_path
            .as_deref()
            .filter(|path| !path.as_os_str().is_empty())
            .map(Path::to_path_buf),
        process_id: Some(process.pid),
        product_name: identity.product_name.to_string(),
        browser_family: identity.browser_family,
        browser_channel: identity.browser_channel,
        install_state: BrowserInventoryInstallState::CandidateRunning,
        running_state: BrowserInventoryRunningState::RunningUnmanaged,
        management_tier: BrowserManagementTier::Unmanaged,
        support_tier: BrowserSupportTier::UnmanagedProcessOnly,
        exact_url_capability: BrowserExactUrlCapability::NotClaimed,
        active_tab_capability: BrowserActiveTabCapability::NotClaimed,
        managed_profile_state: BrowserManagedProfileState::NotApplicable,
        unmanaged_fallback_capability: BrowserUnmanagedFallbackCapability::ReportOnly,
        capability_status: BrowserCapabilityStatus::UnmanagedBrowser,
        reason_code: constants::browser::INVENTORY_REASON_UNMANAGED_BROWSER_PROCESS_ONLY,
    }
}

fn unsupported_process_observation(
    process: &ProcessObservation,
    identity: &BrowserWindowsExecutableIdentity,
) -> BrowserWindowsInventoryObservation {
    BrowserWindowsInventoryObservation {
        executable_path: process
            .executable_path
            .as_deref()
            .filter(|path| !path.as_os_str().is_empty())
            .map(Path::to_path_buf),
        process_id: Some(process.pid),
        product_name: identity.product_name.to_string(),
        browser_family: identity.browser_family,
        browser_channel: identity.browser_channel,
        install_state: BrowserInventoryInstallState::CandidateRunning,
        running_state: BrowserInventoryRunningState::RunningUnknown,
        management_tier: BrowserManagementTier::Unsupported,
        support_tier: BrowserSupportTier::Unsupported,
        exact_url_capability: BrowserExactUrlCapability::Unsupported,
        active_tab_capability: BrowserActiveTabCapability::Unsupported,
        managed_profile_state: BrowserManagedProfileState::NotApplicable,
        unmanaged_fallback_capability: BrowserUnmanagedFallbackCapability::Unsupported,
        capability_status: BrowserCapabilityStatus::UnsupportedBrowser,
        reason_code: constants::browser::INVENTORY_REASON_WINDOWS_BROWSER_PROCESS_UNSUPPORTED,
    }
}
