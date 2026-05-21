use std::path::Path;

use ocentra_parent_agent_protocol::{constants, BrowserChannel, BrowserFamily};

use crate::process_capture::ProcessObservation;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserManagedExecutableIdentity {
    pub browser_family: BrowserFamily,
    pub browser_channel: BrowserChannel,
    pub supports_managed_cdp: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserUnmanagedProcessObservation {
    pub process_id: u32,
    pub process_name: String,
    pub browser_family: BrowserFamily,
    pub browser_channel: BrowserChannel,
}

pub fn managed_browser_executable_identity(path: &Path) -> BrowserManagedExecutableIdentity {
    let executable_name = executable_name_normalized(path);
    let browser_channel = browser_channel_from_path(path);
    match executable_name.as_str() {
        constants::browser::EXECUTABLE_MSEDGE_WINDOWS
        | constants::browser::EXECUTABLE_MSEDGE_LINUX
        | constants::browser::EXECUTABLE_MICROSOFT_EDGE_LINUX => BrowserManagedExecutableIdentity {
            browser_family: BrowserFamily::Edge,
            browser_channel,
            supports_managed_cdp: true,
        },
        constants::browser::EXECUTABLE_CHROME_WINDOWS
        | constants::browser::EXECUTABLE_CHROME_LINUX
        | constants::browser::EXECUTABLE_GOOGLE_CHROME_LINUX => BrowserManagedExecutableIdentity {
            browser_family: BrowserFamily::Chrome,
            browser_channel,
            supports_managed_cdp: true,
        },
        _ => BrowserManagedExecutableIdentity {
            browser_family: BrowserFamily::Unknown,
            browser_channel: BrowserChannel::Unknown,
            supports_managed_cdp: false,
        },
    }
}

pub fn unmanaged_browser_processes(
    observations: &[ProcessObservation],
    managed_process_id: Option<u32>,
) -> Vec<BrowserUnmanagedProcessObservation> {
    observations
        .iter()
        .filter_map(|observation| unmanaged_browser_process(observation, managed_process_id))
        .collect()
}

fn unmanaged_browser_process(
    observation: &ProcessObservation,
    managed_process_id: Option<u32>,
) -> Option<BrowserUnmanagedProcessObservation> {
    if managed_process_id == Some(observation.pid) {
        return None;
    }
    let identity = managed_browser_executable_identity(Path::new(&observation.name));
    if !identity.supports_managed_cdp {
        return None;
    }

    Some(BrowserUnmanagedProcessObservation {
        process_id: observation.pid,
        process_name: observation.name.clone(),
        browser_family: identity.browser_family,
        browser_channel: identity.browser_channel,
    })
}

fn executable_name_normalized(path: &Path) -> String {
    path.file_name()
        .map(|name| name.to_string_lossy().to_ascii_lowercase())
        .unwrap_or_default()
}

fn browser_channel_from_path(path: &Path) -> BrowserChannel {
    let components = normalized_component_names(path);
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

fn normalized_component_names(path: &Path) -> Vec<String> {
    path.components()
        .map(|component| component.as_os_str().to_string_lossy().to_ascii_lowercase())
        .collect()
}
