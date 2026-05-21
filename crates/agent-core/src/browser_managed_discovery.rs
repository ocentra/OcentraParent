use std::path::{Path, PathBuf};

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserManagedInstallCandidate {
    pub executable_path: PathBuf,
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

pub fn installed_managed_browser_candidates(
    candidate_paths: &[PathBuf],
) -> Vec<BrowserManagedInstallCandidate> {
    candidate_paths
        .iter()
        .filter_map(|path| installed_managed_browser_candidate(path))
        .collect()
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

fn installed_managed_browser_candidate(path: &Path) -> Option<BrowserManagedInstallCandidate> {
    if !path.is_file() {
        return None;
    }
    let identity = managed_browser_executable_identity(path);
    if !identity.supports_managed_cdp {
        return None;
    }

    Some(BrowserManagedInstallCandidate {
        executable_path: path.to_path_buf(),
        browser_family: identity.browser_family,
        browser_channel: identity.browser_channel,
    })
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
    normalized_component_names(path)
        .last()
        .cloned()
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
    path.to_string_lossy()
        .split(['/', '\\'])
        .filter(|component| !component.is_empty())
        .map(str::to_ascii_lowercase)
        .collect()
}
