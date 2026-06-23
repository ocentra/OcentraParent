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
    let mut observations = candidate_paths
        .iter()
        .filter_map(|path| windows_browser_inventory_path_observation(path))
        .collect::<Vec<_>>();
    for process in process_observations {
        let Some(process_observation) =
            windows_browser_inventory_process_observation(process, managed_process_id)
        else {
            continue;
        };
        if let Some(process_path) = process
            .executable_path
            .as_deref()
            .filter(|path| !path.as_os_str().is_empty())
        {
            if let Some(candidate_observation) = observations.iter_mut().find(|observation| {
                observation
                    .executable_path
                    .as_deref()
                    .is_some_and(|candidate_path| {
                        normalized_component_names(candidate_path)
                            == normalized_component_names(process_path)
                    })
            }) {
                if candidate_observation.process_id.is_none() {
                    candidate_observation.process_id = process_observation.process_id;
                }
                candidate_observation.running_state = process_observation.running_state.clone();
                candidate_observation.management_tier = process_observation.management_tier.clone();
                candidate_observation.support_tier = process_observation.support_tier.clone();
                candidate_observation.exact_url_capability =
                    process_observation.exact_url_capability.clone();
                candidate_observation.active_tab_capability =
                    process_observation.active_tab_capability.clone();
                candidate_observation.managed_profile_state =
                    process_observation.managed_profile_state.clone();
                candidate_observation.unmanaged_fallback_capability =
                    process_observation.unmanaged_fallback_capability.clone();
                candidate_observation.capability_status =
                    process_observation.capability_status.clone();
                candidate_observation.reason_code = process_observation.reason_code;
                continue;
            }
        }
        observations.push(process_observation);
    }
    observations.sort_by(|left, right| {
        left.product_name
            .cmp(&right.product_name)
            .then_with(|| {
                left.browser_channel
                    .as_protocol_str()
                    .cmp(right.browser_channel.as_protocol_str())
            })
            .then_with(|| left.process_id.cmp(&right.process_id))
    });
    observations
}

pub fn windows_browser_executable_identity(path: &Path) -> BrowserWindowsExecutableIdentity {
    let executable_name = executable_name_normalized(path);
    let components = normalized_component_names(path);
    match executable_name.as_str() {
        constants::browser::EXECUTABLE_MSEDGE_WINDOWS
        | constants::browser::EXECUTABLE_MSEDGE_LINUX
        | constants::browser::EXECUTABLE_MICROSOFT_EDGE_LINUX => BrowserWindowsExecutableIdentity {
            browser_family: BrowserFamily::Edge,
            browser_channel: browser_channel_from_components(&components),
            product_name: constants::browser::PRODUCT_NAME_MICROSOFT_EDGE,
            supports_managed_cdp: true,
            support_kind: BrowserWindowsSupportKind::ManagedChromium,
        },
        constants::browser::EXECUTABLE_CHROME_WINDOWS
        | constants::browser::EXECUTABLE_CHROME_LINUX
        | constants::browser::EXECUTABLE_GOOGLE_CHROME_LINUX => chrome_identity(&components),
        constants::browser::EXECUTABLE_BRAVE_WINDOWS => BrowserWindowsExecutableIdentity {
            browser_family: BrowserFamily::Brave,
            browser_channel: BrowserChannel::Stable,
            product_name: constants::browser::PRODUCT_NAME_BRAVE_BROWSER,
            supports_managed_cdp: false,
            support_kind: BrowserWindowsSupportKind::ManualChromium,
        },
        constants::browser::EXECUTABLE_VIVALDI_WINDOWS => BrowserWindowsExecutableIdentity {
            browser_family: BrowserFamily::UnknownChromium,
            browser_channel: BrowserChannel::Stable,
            product_name: constants::browser::PRODUCT_NAME_VIVALDI_BROWSER,
            supports_managed_cdp: false,
            support_kind: BrowserWindowsSupportKind::ManualChromium,
        },
        constants::browser::EXECUTABLE_OPERA_WINDOWS
        | constants::browser::EXECUTABLE_OPERA_GX_WINDOWS => opera_identity(&components),
        constants::browser::EXECUTABLE_CHROMIUM_WINDOWS => BrowserWindowsExecutableIdentity {
            browser_family: BrowserFamily::UnknownChromium,
            browser_channel: BrowserChannel::Unknown,
            product_name: constants::browser::PRODUCT_NAME_CHROMIUM,
            supports_managed_cdp: false,
            support_kind: BrowserWindowsSupportKind::ManualChromium,
        },
        constants::browser::EXECUTABLE_FIREFOX_WINDOWS => firefox_identity(&components),
        constants::browser::EXECUTABLE_TOR_WINDOWS => unsupported_identity(
            BrowserFamily::Unknown,
            constants::browser::PRODUCT_NAME_TOR_BROWSER,
        ),
        constants::browser::EXECUTABLE_DUCKDUCKGO_WINDOWS => unsupported_identity(
            BrowserFamily::Unknown,
            constants::browser::PRODUCT_NAME_DUCKDUCKGO_BROWSER,
        ),
        constants::browser::EXECUTABLE_ARC_WINDOWS => unsupported_identity(
            BrowserFamily::UnknownChromium,
            constants::browser::PRODUCT_NAME_ARC_BROWSER,
        ),
        _ => BrowserWindowsExecutableIdentity {
            browser_family: BrowserFamily::Unknown,
            browser_channel: BrowserChannel::Unknown,
            product_name: constants::browser::FAMILY_UNKNOWN,
            supports_managed_cdp: false,
            support_kind: BrowserWindowsSupportKind::Unknown,
        },
    }
}

fn windows_browser_inventory_path_observation(
    path: &Path,
) -> Option<BrowserWindowsInventoryObservation> {
    if !path.is_file() {
        return None;
    }
    let identity = windows_browser_executable_identity(path);
    match identity.support_kind {
        BrowserWindowsSupportKind::ManagedChromium => {
            Some(managed_chromium_path_observation(path, identity))
        }
        BrowserWindowsSupportKind::ManualChromium => {
            Some(manual_chromium_path_observation(path, identity))
        }
        BrowserWindowsSupportKind::Unsupported => {
            Some(unsupported_path_observation(path, identity))
        }
        BrowserWindowsSupportKind::Unknown => None,
    }
}

fn windows_browser_inventory_process_observation(
    process: &ProcessObservation,
    managed_process_id: Option<u32>,
) -> Option<BrowserWindowsInventoryObservation> {
    if managed_process_id == Some(process.pid) {
        return None;
    }
    let identity = windows_browser_executable_identity(process_identity_path(process));
    match identity.support_kind {
        BrowserWindowsSupportKind::ManagedChromium | BrowserWindowsSupportKind::ManualChromium => {
            Some(unmanaged_process_observation(process, identity))
        }
        BrowserWindowsSupportKind::Unsupported => {
            Some(unsupported_process_observation(process, identity))
        }
        BrowserWindowsSupportKind::Unknown => None,
    }
}

fn process_identity_path(process: &ProcessObservation) -> &Path {
    process
        .executable_path
        .as_deref()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new(&process.name))
}

fn managed_chromium_path_observation(
    path: &Path,
    identity: BrowserWindowsExecutableIdentity,
) -> BrowserWindowsInventoryObservation {
    BrowserWindowsInventoryObservation {
        executable_path: Some(path.to_path_buf()),
        process_id: None,
        product_name: identity.product_name.to_string(),
        browser_family: identity.browser_family,
        browser_channel: identity.browser_channel,
        install_state: install_state_from_path(path),
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

fn manual_chromium_path_observation(
    path: &Path,
    identity: BrowserWindowsExecutableIdentity,
) -> BrowserWindowsInventoryObservation {
    BrowserWindowsInventoryObservation {
        executable_path: Some(path.to_path_buf()),
        process_id: None,
        product_name: identity.product_name.to_string(),
        browser_family: identity.browser_family,
        browser_channel: identity.browser_channel,
        install_state: install_state_from_path(path),
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

fn unsupported_path_observation(
    path: &Path,
    identity: BrowserWindowsExecutableIdentity,
) -> BrowserWindowsInventoryObservation {
    BrowserWindowsInventoryObservation {
        executable_path: Some(path.to_path_buf()),
        process_id: None,
        product_name: identity.product_name.to_string(),
        browser_family: identity.browser_family,
        browser_channel: identity.browser_channel,
        install_state: install_state_from_path(path),
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
    identity: BrowserWindowsExecutableIdentity,
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
    identity: BrowserWindowsExecutableIdentity,
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

fn install_state_from_path(path: &Path) -> BrowserInventoryInstallState {
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

fn chrome_identity(components: &[String]) -> BrowserWindowsExecutableIdentity {
    if components
        .iter()
        .any(|name| name == constants::browser::PATH_SEGMENT_CHROME_FOR_TESTING_NORMALIZED)
    {
        return BrowserWindowsExecutableIdentity {
            browser_family: BrowserFamily::Chrome,
            browser_channel: BrowserChannel::Stable,
            product_name: constants::browser::PRODUCT_NAME_CHROME_FOR_TESTING,
            supports_managed_cdp: true,
            support_kind: BrowserWindowsSupportKind::ManagedChromium,
        };
    }
    if components
        .iter()
        .any(|name| name == constants::browser::PATH_SEGMENT_CHROMIUM_NORMALIZED)
    {
        return BrowserWindowsExecutableIdentity {
            browser_family: BrowserFamily::UnknownChromium,
            browser_channel: BrowserChannel::Unknown,
            product_name: constants::browser::PRODUCT_NAME_CHROMIUM,
            supports_managed_cdp: false,
            support_kind: BrowserWindowsSupportKind::ManualChromium,
        };
    }
    BrowserWindowsExecutableIdentity {
        browser_family: BrowserFamily::Chrome,
        browser_channel: browser_channel_from_components(components),
        product_name: constants::browser::PRODUCT_NAME_GOOGLE_CHROME,
        supports_managed_cdp: true,
        support_kind: BrowserWindowsSupportKind::ManagedChromium,
    }
}

fn opera_identity(components: &[String]) -> BrowserWindowsExecutableIdentity {
    let product_name = if components.iter().any(|name| {
        name == constants::browser::PATH_SEGMENT_OPERA_GX_STABLE
            .to_ascii_lowercase()
            .as_str()
    }) {
        constants::browser::PRODUCT_NAME_OPERA_GX_BROWSER
    } else {
        constants::browser::PRODUCT_NAME_OPERA_BROWSER
    };
    BrowserWindowsExecutableIdentity {
        browser_family: BrowserFamily::Opera,
        browser_channel: BrowserChannel::Stable,
        product_name,
        supports_managed_cdp: false,
        support_kind: BrowserWindowsSupportKind::ManualChromium,
    }
}

fn firefox_identity(components: &[String]) -> BrowserWindowsExecutableIdentity {
    if components
        .iter()
        .any(|name| name == constants::browser::PATH_SEGMENT_TOR_BROWSER_NORMALIZED)
    {
        return unsupported_identity(
            BrowserFamily::Unknown,
            constants::browser::PRODUCT_NAME_TOR_BROWSER,
        );
    }
    let product_name = if components
        .iter()
        .any(|name| name == constants::browser::PATH_SEGMENT_FIREFOX_DEVELOPER_NORMALIZED)
    {
        constants::browser::PRODUCT_NAME_FIREFOX_DEVELOPER_EDITION
    } else {
        constants::browser::PRODUCT_NAME_MOZILLA_FIREFOX
    };
    unsupported_identity(BrowserFamily::Firefox, product_name)
}

fn unsupported_identity(
    browser_family: BrowserFamily,
    product_name: &'static str,
) -> BrowserWindowsExecutableIdentity {
    BrowserWindowsExecutableIdentity {
        browser_family,
        browser_channel: BrowserChannel::Unknown,
        product_name,
        supports_managed_cdp: false,
        support_kind: BrowserWindowsSupportKind::Unsupported,
    }
}

fn browser_channel_from_components(components: &[String]) -> BrowserChannel {
    if components
        .iter()
        .any(|name| name == constants::browser::PATH_SEGMENT_EDGE_BETA)
        || components
            .iter()
            .any(|name| name == constants::browser::PATH_SEGMENT_CHROME_BETA)
    {
        return BrowserChannel::Beta;
    }
    if components
        .iter()
        .any(|name| name == constants::browser::PATH_SEGMENT_EDGE_DEV)
        || components
            .iter()
            .any(|name| name == constants::browser::PATH_SEGMENT_CHROME_DEV)
    {
        return BrowserChannel::Dev;
    }
    if components
        .iter()
        .any(|name| name == constants::browser::PATH_SEGMENT_EDGE_SXS)
        || components
            .iter()
            .any(|name| name == constants::browser::PATH_SEGMENT_CHROME_SXS)
    {
        return BrowserChannel::Canary;
    }
    BrowserChannel::Stable
}

fn executable_name_normalized(path: &Path) -> String {
    normalized_component_names(path)
        .last()
        .cloned()
        .unwrap_or_default()
}

fn normalized_component_names(path: &Path) -> Vec<String> {
    path.to_string_lossy()
        .split(['/', '\\'])
        .filter(|component| !component.is_empty())
        .map(str::to_ascii_lowercase)
        .collect()
}
